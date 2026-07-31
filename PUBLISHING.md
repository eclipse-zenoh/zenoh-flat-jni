# Publishing and Release Readiness

This document describes how `zenoh-flat-jni` is built, verified, and published
so that `zenoh-java` and `zenoh-kotlin` can depend on it as an ordinary Maven
artifact.

It describes the pipeline as it exists in this repository. Where something is
not yet implemented or not yet exercised, it is listed under
[Known gaps](#known-gaps) rather than described as if it worked.

## Release relationship

`zenoh-flat-jni` owns the generated Kotlin/JNI boundary and the native
libraries. `zenoh-java` and `zenoh-kotlin` are downstream wrappers and must not
build or package their own copies of the JNI library.

The release order is:

```text
pinned prebindgen, zenoh-flat, and Zenoh revisions
                         |
                         v
             publish zenoh-flat-jni
                         |
                         v
        verify it resolves from Maven Central
                    /           \
                   v             v
          release zenoh-java  release zenoh-kotlin
```

Maven Central releases are immutable. A downstream release must therefore never
depend on an unpublished `zenoh-flat-jni` version or assume that an already
published JNI artifact can be replaced later.

## Artifacts

### Desktop JVM

```text
org.eclipse.zenoh:zenoh-flat-jni:<version>
```

A universal JVM JAR: the Kotlin/JVM classes plus one native library per
supported desktop target. `NativeLibrary.kt` resolves them from this layout,
which the release build reproduces exactly:

```text
x86_64-unknown-linux-gnu/x86_64-unknown-linux-gnu.zip
aarch64-unknown-linux-gnu/aarch64-unknown-linux-gnu.zip
x86_64-apple-darwin/x86_64-apple-darwin.zip
aarch64-apple-darwin/aarch64-apple-darwin.zip
x86_64-pc-windows-msvc/x86_64-pc-windows-msvc.zip
aarch64-pc-windows-msvc/aarch64-pc-windows-msvc.zip
```

Each ZIP holds exactly the release native library for that target:

- Linux: `libzenoh_flat_jni.so`
- macOS: `libzenoh_flat_jni.dylib`
- Windows: `zenoh_flat_jni.dll`

The `desktopTargets` map in `build.gradle.kts` is the single declaration of this
set: it drives the release build matrix and `verifyDesktopArtifact`. A target
that is not built is not advertised.

A developer build produces a different JAR — the host library alone, at the JAR
root (`NativeLibrary`'s second loading strategy). The two layouts are mutually
exclusive by construction: when `jni-libs/` is present, the host library is not
bundled and no Cargo build is wired into `jar` at all.

### Android

```text
org.eclipse.zenoh:zenoh-flat-jni-android:<version>
```

An AAR with the Android NDK libraries in the standard layout:

```text
jni/armeabi-v7a/libzenoh_flat_jni.so
jni/arm64-v8a/libzenoh_flat_jni.so
jni/x86/libzenoh_flat_jni.so
jni/x86_64/libzenoh_flat_jni.so
```

The AAR is assembled by a plain `Zip` task, not by the Android Gradle Plugin.
This module has no Android resources, no manifest entries beyond `minSdk`, and
no Android-only code, so its AAR is `classes.jar` plus `jni/<abi>/` — applying
AGP would mean installing an Android SDK in CI to zip four `.so` files. The
`androidAar` task synthesises the mandatory `AndroidManifest.xml` and empty
`R.txt`, and `androidClassesJar` deliberately packs `output.classesDirs` rather
than the desktop `jar`, so the AAR does not carry the desktop native ZIPs.

Switch to `com.android.library` if this module ever grows real Android
resources or a non-trivial manifest.

Downstream Android publications must resolve this AAR, not the desktop JAR.

### Maven Central contents

Both publications carry:

- The primary JAR or AAR.
- A sources JAR (`sourcesJar`: Kotlin sources, the Rust `src/`, `build.rs`,
  `Cargo.toml`).
- A Dokka-generated Javadoc JAR (`javadocJar`).
- A POM with name, description, URL, license, developers, and SCM information.
  The Android POM is built from `artifact()` entries, so its single runtime
  dependency on `kotlin-stdlib` is written explicitly.
- A PGP signature per artifact and POM (signing is required only when
  `-PremotePublication=true`).
- Checksums, generated during publication.

The tag, `version.txt`, `Cargo.toml`, and the Maven version must agree;
`version.txt` is the single source of truth and the `validate` job enforces the
rest. `gradle.properties` no longer carries a second copy.

## Reproducibility

Every archive task sets `isPreserveFileTimestamps = false` and
`isReproducibleFileOrder = true`. Two clean builds of the same inputs therefore
produce a byte-identical JAR, which is what makes the post-release hash
comparison against Maven Central meaningful.

Release inputs are pinned in [`release-inputs.env`](release-inputs.env):

| Input | Why it is pinned |
| --- | --- |
| `ZENOH_FLAT_REF` | sibling path dependency; also pins the Zenoh revision it selects |
| `PREBINDGEN_REF` | the generator — a different one means different bindings |
| `CARGO_NDK_VERSION` | `cargo install --locked` pins the lockfile of the *selected* release, not which release is selected |
| `ANDROID_NDK_VERSION` | validated together with the cargo-ndk pin |

`Cargo.lock` is committed and every release build runs `--locked`. CI checks the
sibling repositories out at these revisions rather than rewriting `Cargo.toml`
to Git dependencies: rewriting the manifest invalidates the lockfile, which is
the whole point of committing it. Regenerate and commit `Cargo.lock` whenever a
pinned revision moves, or the release build fails.

## Release pipeline

[`.github/workflows/publish.yml`](.github/workflows/publish.yml) runs on a `v*`
tag or via `workflow_dispatch`. Every gate runs before the irreversible step.

### 1. `validate` — release inputs

Reads the pinned inputs, checks that `version.txt`, the `[package]` version in
`Cargo.toml`, and the tag agree, and refuses to proceed if the version already
exists on Maven Central.

### 2. `generated-sources`

Rebuilds the bindings from the pinned inputs with `cargo build --release
--locked` and fails if `src/generated_bindings.rs` or `kotlin/generated` differ
from what is committed — a release must not ship generated sources nobody
reviewed.

### 3. `desktop-natives`

Builds the six declared targets with `--locked`, packages each as
`<target>/<target>.zip`, and uploads it with its SHA-256 in the job summary.

`aarch64-unknown-linux-gnu` is cross-compiled with plain Cargo and
`gcc-aarch64-linux-gnu` rather than `cross`, because the sibling path
dependencies live outside the project directory `cross` mounts into its
container.

### 4. `android-natives`

Builds the four ABIs with the pinned cargo-ndk and NDK. `cargo ndk -o` writes
exactly the AAR's `jni/<abi>/` layout, so no repackaging step is needed.

### 5–6. `consumer-test` — assemble and consume

Downloads the native artifacts, assembles both publications, and publishes them
to an isolated file-based repository under `build/dry-run-repository`.

`verifyDesktopArtifact` and `verifyAndroidArtifact` run as publication
dependencies and fail the build on:

- a missing target ZIP or ABI library;
- a target ZIP whose contents are not exactly the one expected library;
- a stray native library at the JAR root, which would shadow the per-target
  resources on whichever platform happened to match it;
- a missing `AndroidManifest.xml`, `classes.jar`, or `R.txt`.

[`ci/consumer-smoke-test`](ci/consumer-smoke-test) then resolves the candidate
from that repository as an external Gradle project — no path dependency, no
composite build — and runs it. Its repository declarations use content filters
(`includeGroup` on the candidate repository, `excludeGroup` on Maven Central),
so resolution *cannot* silently fall back to a previously released artifact with
the same coordinates: if it builds, the candidate is what it resolved.

The smoke test creates a key expression, round-trips it across JNI, and closes
the handle. Key expressions need no network, ports, or discovery, so a CI runner
cannot make it flaky, and the exercise still covers native-library extraction
from the JAR resources, a JNI call in both directions, and a handle's full
create/use/close cycle.

This job runs on `ubuntu-latest`, `ubuntu-24.04-arm`, `macos-13`,
`macos-latest`, and `windows-latest` — five of the six advertised desktop
targets. The Ubuntu run also records the candidate checksums as an artifact for
the post-release comparison.

`mavenLocal()` is deliberately not used for this: it can serve leftovers from
earlier builds, which makes it impossible to prove which repository supplied an
artifact.

### 7. `stage` — Central staging

Uploads and **closes** a staging deployment through the Central Publisher
Portal's OSSRH Staging API:

```text
nexusUrl               https://ossrh-staging-api.central.sonatype.com/service/local/
snapshotRepositoryUrl  https://central.sonatype.com/repository/maven-snapshots/
```

Central runs its own validation — signatures, checksums, POM completeness —
and nothing is public. A deployment that fails, or a dry run, is simply
dropped. The legacy `s01.oss.sonatype.org` endpoint is retired and its OSSRH
credentials no longer work.

### 8. `release-staging` and `verify-central`

`release-staging` releases the closed deployment. It is guarded by the
`maven-central` GitHub environment — protect that environment with required
reviewers, because everything before it is reversible and this step is not.

`verify-central` then, on each of the five runner platforms:

1. polls Maven Central until the coordinates resolve (up to an hour);
2. downloads the released JAR and compares its SHA-256 with the candidate CI
   verified;
3. reruns the consumer smoke test, resolving from Maven Central.

### 9. `github-release`

Created last, from the verified release, recording the zenoh-flat and prebindgen
revisions the artifacts were built from.

## Dry run

Gradle's `--dry-run` only shows task selection; it does not generate or validate
publishable artifacts. Use it only as a task-wiring check.

### Central staging dry run

`workflow_dispatch` with `stage_only=true` (the default) runs the entire
pipeline through Central staging validation and stops. Inspect the deployment in
the Central Portal, then drop it instead of releasing it.

Snapshots (`-PSNAPSHOT`) are useful for repeated integration testing, but
snapshot success does not replace release validation.

### Local dry run

Populate `jni-libs/` — and optionally `android-libs/` — then publish to the same
isolated repository the CI consumer test uses:

```bash
./gradlew publishAllPublicationsToDryRunRepository
find build/dry-run-repository -type f -print

cd ci/consumer-smoke-test
gradle run -PcandidateRepository="file://$PWD/../../build/dry-run-repository" \
           -PcandidateVersion="$(cat ../../version.txt)"
```

Inspect the archives and signatures directly:

```bash
unzip -l build/libs/zenoh-flat-jni-*.jar
unzip -l build/distributions/zenoh-flat-jni-android-*.aar
gpg --verify path/to/artifact.asc path/to/artifact
```

`-PremotePublication=true` fails fast unless `jni-libs/` or `android-libs/` is
present, so a remote publication cannot silently ship the publishing runner's
own host library.

## Required secrets

| Secret | Use |
| --- | --- |
| `CENTRAL_SONATYPE_TOKEN_USERNAME` | Central Portal user token |
| `CENTRAL_SONATYPE_TOKEN_PASSWORD` | Central Portal user token |
| `ORG_GPG_SUBKEY_ID` | signing |
| `ORG_GPG_PRIVATE_KEY` | signing |
| `ORG_GPG_PASSPHRASE` | signing |

## Downstream release requirements

Before releasing `zenoh-java` or `zenoh-kotlin`:

- The selected `zenoh-flat-jni` version must already resolve from Maven Central.
- The dependency version must live in one release-controlled property or version
  catalog, not be duplicated as a string.
- Local composite substitution (`includeBuild("../zenoh-flat-jni")`) must be
  optional and disabled in release CI.
- `zenoh-java`'s obsolete workflow that builds the removed `zenoh-jni/Cargo.toml`
  must be deleted.
- JVM publications depend on the desktop JVM artifact; Android publications
  depend on the Android artifact.
- Published downstream POM and Gradle metadata must contain the intended
  `zenoh-flat-jni` dependency.
- Downstream tests must pass using only released Maven artifacts.

## Known gaps

The pipeline is implemented but parts of it have not been executed, and two
items from the original plan are not built at all.

- **The Android artifact has no runtime test.** The AAR's contents are verified
  by archive inspection; nothing loads it. This needs an Android consumer app
  and an emulator job. Until then the Android publication is unproven at
  runtime, and the `CARGO_NDK_VERSION` pin is a current-stable choice rather
  than a validated one.
- **`aarch64-pc-windows-msvc` has no runner**, so it is covered by archive
  inspection only.
- **The cross-build matrix and the Central upload have never run.** Both need
  repository secrets and CI runners. Run the `stage_only` dry run before
  attempting a production release.
- **Downstream release suites have not been run against a Maven artifact** with
  composite substitution disabled; those changes belong in `zenoh-java` and
  `zenoh-kotlin`.

## Release checklist

- [ ] `release-inputs.env` points at the intended revisions and `Cargo.lock` is
      regenerated for them.
- [ ] `version.txt`, `Cargo.toml`, and the tag agree.
- [ ] Central Portal token and GPG secrets are configured.
- [ ] The `maven-central` environment requires a reviewer.
- [ ] A `stage_only` dry run passed Central validation and was dropped.
- [ ] Generated sources are clean; Rust and Kotlin tests pass.
- [ ] All declared desktop targets are in the JAR and all ABIs in the AAR.
- [ ] Sources, Javadoc, POM, signatures, and checksums pass inspection.
- [ ] The external consumer test passed on every runner platform.
- [ ] Central staging validation passed.
- [ ] The released coordinates resolve and the served hash matches the verified
      candidate.
- [ ] The GitHub release was created only after that verification.
