# Publishing and Release Readiness

<<<<<<< HEAD
This document describes how `zenoh-flat-jni` is built, verified, and published
so that `zenoh-java` and `zenoh-kotlin` can depend on it as an ordinary Maven
artifact.

It describes the pipeline as it exists in this repository. Where something is
not yet implemented or not yet exercised, it is listed under
[Known gaps](#known-gaps) rather than described as if it worked.
=======
This document defines how `zenoh-flat-jni` must be built, verified, and
published so that `zenoh-java` and `zenoh-kotlin` can depend on it as an
ordinary Maven artifact.

It also records the known problems in the current publishing implementation.
Those problems must be fixed before the first production release.
>>>>>>> origin/main

## Release relationship

`zenoh-flat-jni` owns the generated Kotlin/JNI boundary and the native
libraries. `zenoh-java` and `zenoh-kotlin` are downstream wrappers and must not
build or package their own copies of the JNI library.

The release order is:

```text
<<<<<<< HEAD
pinned prebindgen, zenoh-flat, and Zenoh revisions
=======
fixed prebindgen, zenoh-flat, and Zenoh revisions
>>>>>>> origin/main
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

<<<<<<< HEAD
Maven Central releases are immutable. A downstream release must therefore never
depend on an unpublished `zenoh-flat-jni` version or assume that an already
published JNI artifact can be replaced later.

## Artifacts

### Desktop JVM

=======
Maven Central releases are immutable. A downstream release must therefore
never depend on an unpublished `zenoh-flat-jni` version or assume that an
already published JNI artifact can be replaced later.

## Intended artifacts

### Desktop JVM

The primary publication should be:

>>>>>>> origin/main
```text
org.eclipse.zenoh:zenoh-flat-jni:<version>
```

<<<<<<< HEAD
A universal JVM JAR: the Kotlin/JVM classes plus one native library per
supported desktop target. `NativeLibrary.kt` resolves them from this layout,
which the release build reproduces exactly:
=======
It must be a universal JVM JAR containing the Kotlin/JVM classes and the native
library for every supported desktop target. `NativeLibrary.kt` currently
expects the multi-platform resources in the following layout:
>>>>>>> origin/main

```text
x86_64-unknown-linux-gnu/x86_64-unknown-linux-gnu.zip
aarch64-unknown-linux-gnu/aarch64-unknown-linux-gnu.zip
x86_64-apple-darwin/x86_64-apple-darwin.zip
aarch64-apple-darwin/aarch64-apple-darwin.zip
x86_64-pc-windows-msvc/x86_64-pc-windows-msvc.zip
aarch64-pc-windows-msvc/aarch64-pc-windows-msvc.zip
```

<<<<<<< HEAD
Each ZIP holds exactly the release native library for that target:
=======
Each ZIP must contain exactly the release native library for that target:
>>>>>>> origin/main

- Linux: `libzenoh_flat_jni.so`
- macOS: `libzenoh_flat_jni.dylib`
- Windows: `zenoh_flat_jni.dll`

<<<<<<< HEAD
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
=======
If a target is not built and tested, it must not be advertised as supported.

### Android

The desktop JAR is not sufficient for Android. Android releases of
`zenoh-java` and `zenoh-kotlin` require an AAR with Android NDK libraries in
the standard layout:
>>>>>>> origin/main

```text
jni/armeabi-v7a/libzenoh_flat_jni.so
jni/arm64-v8a/libzenoh_flat_jni.so
jni/x86/libzenoh_flat_jni.so
jni/x86_64/libzenoh_flat_jni.so
```

<<<<<<< HEAD
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
=======
The recommended coordinates are:

```text
org.eclipse.zenoh:zenoh-flat-jni-android:<version>
```

Alternatively, the project may become a variant-aware Kotlin Multiplatform
publication with JVM and Android variants. Whichever design is chosen,
downstream Android publications must resolve an Android AAR, not the desktop
JAR.

## Required Maven Central contents

Each publication must include:

- The primary JAR or AAR.
- A sources JAR.
- A Javadoc/documentation JAR.
- A complete POM with name, description, project URL, license, developers, and
  SCM information.
- A valid PGP signature for every required artifact and POM.
- Repository checksums generated during publication.

The tag, `Cargo.toml` version, Gradle version, Maven version, and GitHub release
version must agree.

## Current release blockers

The following issues are present in the repository at the time this document
was written. A successful CI build on `main` does not resolve these publishing
issues.

### 1. Incorrect Gradle property syntax in the publishing workflow

`build.gradle.kts` reads `remotePublication` and `release` with
`project.findProperty`, which requires Gradle project properties:

```text
-PremotePublication=true -Prelease=true
```

The current tag workflow passes:

```text
-DremotePublication=true -Drelease=true
```

As a result, remote publication and release mode are not enabled. The workflow
can select the debug library and the local repository instead of Maven
Central.

**Required fix:** use `-P` properties or replace the configuration with a
single explicit, validated publication mode.

### 2. Obsolete Sonatype endpoint and credentials

The build still targets:

```text
https://s01.oss.sonatype.org/service/local/staging/deploy/maven2/
```

The legacy OSSRH service has been retired. The project must use the Central
Publisher Portal or the Portal OSSRH Staging API and authenticate with Central
Portal user tokens.

**Required fix:** migrate the repository URL, Gradle publishing plugin, task
names, secrets, and close/release procedure together. Prefer the same Central
Portal mechanism already used by the downstream Zenoh repositories.

### 3. Incomplete Gradle wrapper

`gradle/wrapper/gradle-wrapper.jar` is not committed. Consequently,
`./gradlew` cannot start in a clean checkout.

**Required fix:** regenerate and commit a complete Gradle wrapper at a pinned
version. CI must use that wrapper and verify its checksum.

### 4. Standalone checkout cannot build

`Cargo.toml` contains sibling path dependencies on `../zenoh-flat` and
`../prebindgen/prebindgen`, while the publishing workflow checks out only this
repository.

**Required fix:** either:

1. use released crates or exact Git revisions, or
2. check out the required sibling repositories at exact commits in every
   build job.

The publication must not depend on a developer's PREBINDGEN workspace layout.

### 5. Release inputs are not reproducibly pinned

The Rust dependencies currently follow mutable `main` branches. A release
rebuilt later could therefore use different sources.

**Required fix:** pin releases to crates.io versions, Git tags, or full commit
revisions; commit `Cargo.lock`; and build with `cargo build --release --locked`.
Record the selected `prebindgen`, `zenoh-flat`, and Zenoh revisions in the
release output.

### 6. Only the publishing runner's native library is packaged

The normal `jar` task chooses a native library from `target/debug` or
`target/release` based on the current operating system. Because the publishing
job runs on Ubuntu, it cannot create the promised multi-platform JAR.

There is a separate `packageJar` task, but the Maven publication uses
`from(components["java"])`; therefore `packageJar` is not the primary
published artifact.

**Required fix:** build native libraries in an operating-system/target matrix,
upload them as CI artifacts, and assemble one universal JAR in a dedicated
publication job. The primary `MavenPublication` must use that exact assembled
artifact.

### 7. No Android publication

The current project applies only the Kotlin/JVM plugin and produces no Android
AAR or Android NDK matrix.

**Required fix:** add an Android library publication and cross-compile the four
supported Android ABIs. Add APK/AAR inspection and an emulator smoke test.

### 8. Missing Javadoc artifact

The current Maven publication registers a sources JAR but no Javadoc JAR.
Maven Central validation requires a Javadoc/documentation artifact.

**Required fix:** generate documentation with Dokka, or provide another valid
documentation JAR, and attach it to every applicable publication.

### 9. Publication is not tested as an external dependency

The workspace tests use local path dependencies and downstream composite
builds. They do not prove that the Maven metadata, transitive dependencies,
resource layout, or native extraction work.

**Required fix:** publish to an isolated temporary Maven repository and run
clean consumer builds with composite substitution disabled.

### 10. GitHub release creation is not gated by Central availability

The current workflow creates a GitHub release immediately after its Gradle
command. It does not establish that Central accepted, released, and made the
artifact resolvable.

**Required fix:** create/finalize the GitHub release only after Central
validation succeeds. Before releasing downstream projects, poll Maven Central
until the exact coordinates can be resolved from a clean environment.

## Required CI workflow

The release workflow should use separate jobs with immutable artifacts between
them.

### 1. Validate release inputs

- Require an explicit version.
- Verify that the tag and all project version files agree.
- Verify that the version does not already exist on Maven Central.
- Verify that the source commit is clean and reachable from the release tag.
- Record all Rust dependency revisions.

### 2. Generate and verify sources

- Run the binding generator from the pinned inputs.
- Fail if committed generated Rust, Kotlin, or `REPORT.md` files change.
- Run formatting, Clippy, Rust tests, and Kotlin tests.

### 3. Build desktop native libraries

Build the declared desktop target matrix in release mode with the lockfile.
Package one target ZIP per matrix entry and upload it with a SHA-256 manifest.

Native tests should run on every available native runner. Cross-compiled
targets that cannot execute in CI still require format, architecture, exported
symbol, and archive-layout checks.

### 4. Build Android native libraries

Build the four Android ABI libraries with the pinned NDK. Package them into the
AAR's `jni/<abi>/` layout and upload a SHA-256 manifest.

### 5. Assemble publications

One job must download the native build outputs and create the final JVM JAR and
Android AAR. It must also create sources, documentation, POM, signatures, and
checksums.

The job must inspect the archives and fail if a required target is missing, a
debug artifact is present, or an unexpected duplicate native library exists.

### 6. Test the assembled artifacts

Publish the candidate artifacts to an isolated file-based Maven repository.
Test small external consumer projects and the real `zenoh-java` and
`zenoh-kotlin` builds against that repository.

The downstream test checkouts must:

- disable `includeBuild("../zenoh-flat-jni")`;
- use the candidate Maven version;
- have no source or path dependency on this checkout; and
- run on Linux, macOS, Windows, and an Android emulator as applicable.

At minimum, the smoke test must load the library from the published artifact,
open and close a Zenoh session, exercise serialization, and perform a small
pub/sub or query round trip.

### 7. Upload to Central staging

Upload the signed release to a user-managed Central deployment. Wait for
Central validation before allowing release. A failed deployment must be
dropped.

### 8. Release and verify

After approval:

1. release the validated Central deployment;
2. wait until the exact coordinates resolve from Maven Central;
3. download the artifacts again and compare their hashes with the verified CI
   artifacts;
4. rerun a minimal external-consumer smoke test; and
5. create/finalize the GitHub release.

## Dry-run and verification procedure

Gradle's `--dry-run` only shows task selection. It does not generate or validate
publishable artifacts. Use it only as an additional task-wiring check.

### Local publication dry-run

Add a file-based Maven repository named `dryRun`, located under
`build/dry-run-repository`, and publish the candidate there:

```bash
./gradlew clean check \
  publishMavenPublicationToDryRunRepository \
  -Prelease=true
```

Use the corresponding publication task for the Android artifact.

Do not use a developer's normal `mavenLocal()` as the main release test: it can
hide dependencies left by previous builds and makes it difficult to prove
which repository supplied an artifact.

Inspect the repository and archives:

```bash
find build/dry-run-repository -type f -print
jar tf path/to/zenoh-flat-jni-VERSION.jar
unzip -l path/to/zenoh-flat-jni-android-VERSION.aar
gpg --verify path/to/artifact.asc path/to/artifact
```

Also inspect the generated POM and Gradle module metadata and compare every
native binary with the matrix SHA-256 manifest.

### Isolated consumer test

Use an empty Gradle cache or `--refresh-dependencies`, disable downstream
composite builds, and resolve the candidate solely from the dry-run
repository. Use Gradle `dependencyInsight` to prove which repository and
version were selected.

Publishing and consuming from the same multi-repository workspace is not a
valid substitute for this test.

### Central dry-run

The closest end-to-end dry run is a user-managed Central staging deployment:

1. upload the signed candidate;
2. wait for Central validation;
3. inspect the deployment;
4. run consumer tests against it if the chosen publication mechanism permits;
5. drop it instead of releasing it.

Snapshots are useful for repeated integration testing, but snapshot success
does not replace release validation.
>>>>>>> origin/main

## Downstream release requirements

Before releasing `zenoh-java` or `zenoh-kotlin`:

<<<<<<< HEAD
- The selected `zenoh-flat-jni` version must already resolve from Maven Central.
- The dependency version must live in one release-controlled property or version
  catalog, not be duplicated as a string.
- Local composite substitution (`includeBuild("../zenoh-flat-jni")`) must be
  optional and disabled in release CI.
- `zenoh-java`'s obsolete workflow that builds the removed `zenoh-jni/Cargo.toml`
  must be deleted.
- JVM publications depend on the desktop JVM artifact; Android publications
  depend on the Android artifact.
=======
- The selected `zenoh-flat-jni` version must already resolve from Maven
  Central.
- The dependency version must be maintained in one release-controlled
  property or version catalog, not duplicated as a string.
- Local composite substitution must be optional and disabled in release CI.
- `zenoh-java`'s obsolete workflow that builds the removed
  `zenoh-jni/Cargo.toml` must be deleted.
- JVM publications must depend on the desktop JVM artifact.
- Android publications must depend on the Android artifact/variant.
>>>>>>> origin/main
- Published downstream POM and Gradle metadata must contain the intended
  `zenoh-flat-jni` dependency.
- Downstream tests must pass using only released Maven artifacts.

<<<<<<< HEAD
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
=======
## Release checklist

The release owner must confirm all items below:

- [ ] Complete, pinned Gradle wrapper is committed.
- [ ] Current Central Portal endpoint and token credentials are configured.
- [ ] Remote/release Gradle properties are passed correctly.
- [ ] Tag, Cargo, Gradle, Maven, and GitHub versions agree.
- [ ] Rust dependencies are immutable and `Cargo.lock` is honored.
- [ ] Generated source tree is reproducible and clean.
- [ ] Rust, Kotlin, correspondence, and concurrency tests pass.
- [ ] All declared desktop targets are present in the universal JAR.
- [ ] All declared Android ABIs are present in the AAR.
- [ ] Sources and Javadoc artifacts are present.
- [ ] POM, signatures, and checksums pass inspection.
- [ ] External consumer tests pass from an isolated Maven repository.
- [ ] `zenoh-java` passes with composite substitution disabled.
- [ ] `zenoh-kotlin` passes with composite substitution disabled.
- [ ] Central staging validation passes.
- [ ] Released coordinates resolve from a clean Maven Central consumer.
- [ ] Downloaded Central artifact hashes match the verified CI artifacts.
- [ ] GitHub release is created only after Maven Central verification.
>>>>>>> origin/main
