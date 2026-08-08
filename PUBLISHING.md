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
produce a byte-identical JAR, so a hash comparison against what Maven Central
serves is meaningful.

What a release resolves from is fixed by the same mechanisms every zenoh binding
uses, not by a file of its own:

| Input | How it is fixed |
| --- | --- |
| `zenoh`, `zenoh-ext`, `zenoh-flat` | `version` + `git` + `branch` in `Cargo.toml`; the release bump re-points `branch` at `release/X.Y.Z` |
| every transitive crate | `Cargo.lock`, committed and kept byte-aligned with Zenoh's by the shared lockfile-sync bot |
| the Rust compiler | `rust-toolchain.toml` |
| `cargo-ndk`, the NDK | pinned in `publish-android.yml`; `cargo install --locked` pins the dependencies of the *selected* release, not which release is selected |

Every release build runs `--locked`, so a lockfile that does not match the
manifest fails the build instead of silently resolving something else.

## Release pipeline

Releases are driven by
[`.github/workflows/release.yml`](.github/workflows/release.yml), the same shape
zenoh-java and zenoh-kotlin use. It is started manually
(`workflow_dispatch`) and runs four jobs.

### 1. `tag` — branch, bump, tag

`eclipse-zenoh/ci/create-release-branch` creates the release branch, then
[`ci/scripts/bump-and-tag.bash`](ci/scripts/bump-and-tag.bash) writes the
version into `version.txt` and `Cargo.toml`, re-points every `zenoh.*`
dependency at the Zenoh release branch, refreshes `Cargo.lock`, commits and
tags.

With `live-run: false` the release is not skipped but *redirected*: the branch
becomes `release/dry-run/<version>` and the version comes from `git describe`.
That is the dry run.

### 2. `publish-jvm` — the desktop JVM artifact

[`publish-jvm.yml`](.github/workflows/publish-jvm.yml), called as a reusable
workflow:

- **`builds`** cross-compiles the six declared targets with `--locked` and
  packages each as `<target>/<target>.zip` with its SHA-256 in the job summary.
  The toolchain is not named in the workflow — `rust-toolchain.toml` supplies
  it, so the release uses the compiler CI uses. On the host target it also fails
  if `src/generated_bindings.rs`, `kotlin/generated` or `kotlin/REPORT.md`
  differ from what is committed: a release must not ship generated sources
  nobody reviewed.

  `aarch64-unknown-linux-gnu` is cross-compiled with plain Cargo and
  `gcc-aarch64-linux-gnu` rather than `cross`, which is simpler than teaching
  `cross`'s container about the build.

- **`consumer-test`** assembles the publication, publishes it to an isolated
  file repository under `build/dry-run-repository`, and runs
  [`ci/consumer-smoke-test`](ci/consumer-smoke-test) against it as an external
  Gradle project — no path dependency, no composite build. Its repository
  declarations use content filters (`includeGroup` on the candidate repository,
  `excludeGroup` on Maven Central), so resolution cannot silently fall back to a
  previously released artifact with the same coordinates: if it builds, the
  candidate is what it resolved. `mavenLocal()` is deliberately not used, since
  it can serve leftovers from earlier builds.

  The smoke test creates a key expression, round-trips it across JNI and closes
  the handle. Key expressions need no network, ports or discovery, so a runner
  cannot make it flaky, and it still covers native-library extraction from the
  JAR, a JNI call in both directions, and a handle's create/use/close cycle.

  It runs on Linux, macOS and Windows.

- **`publish_jvm_package`** checks that `version.txt` and `Cargo.toml` agree,
  then publishes. `verifyDesktopArtifact` runs as a publication dependency and
  fails the build on a missing target ZIP, a ZIP whose contents are not exactly
  the one expected library, or a stray native at the JAR root that would shadow
  the per-target resources.

### 3. `publish-android` — the Android artifact

[`publish-android.yml`](.github/workflows/publish-android.yml) builds the four
ABIs with the pinned cargo-ndk and NDK — `cargo ndk -o` writes exactly the AAR's
`jni/<abi>/` layout, so nothing is repackaged — and publishes the AAR.
`verifyAndroidArtifact` checks the manifest, `classes.jar`, `R.txt` and all four
ABI libraries.

### 4. `publish-github`

`eclipse-zenoh/ci/publish-crates-github` creates the GitHub release, after both
Maven publications have succeeded.

## How a publication reaches Maven Central

Both publish workflows end in one Gradle command:

```bash
./gradlew publishMavenPublicationToSonatypeRepository \
          closeAndReleaseSonatypeStagingRepository \
          -PremotePublication=true -Prelease=true
```

`io.github.gradle-nexus.publish-plugin` uploads the signed artifacts to a
staging repository on the Central Publisher Portal's OSSRH Staging API, then
*closes* it — at which point Central validates signatures, checksums and POM
completeness — and only then *releases* it. Nothing is public until the release
step, and a failed validation leaves a staging repository that is simply
dropped.

```text
nexusUrl               https://ossrh-staging-api.central.sonatype.com/service/local/
snapshotRepositoryUrl  https://central.sonatype.com/repository/maven-snapshots/
```

The legacy `s01.oss.sonatype.org` endpoint is retired and its OSSRH credentials
no longer work; the credentials are Central Portal tokens.

For a dry run the workflows pass `-PSNAPSHOT` instead and omit
`closeAndReleaseSonatypeStagingRepository`: the version becomes
`<version>-SNAPSHOT` and goes to the snapshot repository, which is mutable and
never staged.

## Dry run

Gradle's `--dry-run` only shows task selection; it does not generate or validate
publishable artifacts. Use it only as a task-wiring check.

### Through the release workflow

Run `release.yml` with `live-run: false`. It branches to
`release/dry-run/<version>`, publishes `-SNAPSHOT` artifacts, and leaves the
released coordinates untouched. Snapshot success does not replace release
validation — snapshots skip Central's staging validation entirely.

Passing `maven_publish: false` runs everything except the upload.

### Locally

Populate `jni-libs/` — and optionally `android-libs/` — then publish to the same
isolated repository the CI consumer test uses:

```bash
cargo build --release --locked
mkdir -p jni-libs/x86_64-unknown-linux-gnu
(cd target/release && zip -j ../../jni-libs/x86_64-unknown-linux-gnu/x86_64-unknown-linux-gnu.zip libzenoh_flat_jni.so)

./gradlew publishMavenPublicationToDryRunRepository -Prelease=true
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

`-PremotePublication=true` is omitted above because it also switches on GPG
signing. It fails fast unless `jni-libs/` or `android-libs/` is present, so a
*remote* publication cannot silently ship the publishing runner's own host
library; the artifact layout itself is selected by `jni-libs/` being there, so a
local dry run without it still produces the multi-platform JAR.

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
  repository secrets and CI runners. Run a `live-run: false` dry run before
  attempting a production release.
- **Downstream release suites have not been run against a Maven artifact** with
  composite substitution disabled; those changes belong in `zenoh-java` and
  `zenoh-kotlin`.

## Release checklist

- [ ] `Cargo.lock` is committed and current for the manifest.
- [ ] `version.txt`, `Cargo.toml`, and the tag agree.
- [ ] Central Portal token and GPG secrets are configured.
- [ ] A `live-run: false` dry run completed and its snapshot resolved.
- [ ] Generated sources are clean; Rust and Kotlin tests pass.
- [ ] All declared desktop targets are in the JAR and all ABIs in the AAR.
- [ ] Sources, Javadoc, POM, signatures, and checksums pass inspection.
- [ ] The external consumer test passed on every runner platform.
- [ ] Central staging validation passed.
- [ ] The released coordinates resolve from Maven Central.
