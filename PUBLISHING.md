# Publishing and Release Readiness

This document defines how `zenoh-flat-jni` must be built, verified, and
published so that `zenoh-java` and `zenoh-kotlin` can depend on it as an
ordinary Maven artifact.

It also records the known problems in the current publishing implementation.
Those problems must be fixed before the first production release.

## Release relationship

`zenoh-flat-jni` owns the generated Kotlin/JNI boundary and the native
libraries. `zenoh-java` and `zenoh-kotlin` are downstream wrappers and must not
build or package their own copies of the JNI library.

The release order is:

```text
fixed prebindgen, zenoh-flat, and Zenoh revisions
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

Maven Central releases are immutable. A downstream release must therefore
never depend on an unpublished `zenoh-flat-jni` version or assume that an
already published JNI artifact can be replaced later.

## Intended artifacts

### Desktop JVM

The primary publication should be:

```text
org.eclipse.zenoh:zenoh-flat-jni:<version>
```

It must be a universal JVM JAR containing the Kotlin/JVM classes and the native
library for every supported desktop target. `NativeLibrary.kt` currently
expects the multi-platform resources in the following layout:

```text
x86_64-unknown-linux-gnu/x86_64-unknown-linux-gnu.zip
aarch64-unknown-linux-gnu/aarch64-unknown-linux-gnu.zip
x86_64-apple-darwin/x86_64-apple-darwin.zip
aarch64-apple-darwin/aarch64-apple-darwin.zip
x86_64-pc-windows-msvc/x86_64-pc-windows-msvc.zip
aarch64-pc-windows-msvc/aarch64-pc-windows-msvc.zip
```

Each ZIP must contain exactly the release native library for that target:

- Linux: `libzenoh_flat_jni.so`
- macOS: `libzenoh_flat_jni.dylib`
- Windows: `zenoh_flat_jni.dll`

If a target is not built and tested, it must not be advertised as supported.

### Android

The desktop JAR is not sufficient for Android. Android releases of
`zenoh-java` and `zenoh-kotlin` require an AAR with Android NDK libraries in
the standard layout:

```text
jni/armeabi-v7a/libzenoh_flat_jni.so
jni/arm64-v8a/libzenoh_flat_jni.so
jni/x86/libzenoh_flat_jni.so
jni/x86_64/libzenoh_flat_jni.so
```

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

`Cargo.toml` contains sibling path dependencies on `../zenoh-flat` and on the
`../prebindgen/*` crates (`prebindgen-jni-runtime` at runtime,
`prebindgen-jni` + `prebindgen-registry` at build time), while the publishing
workflow checks out only this repository.

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

## Downstream release requirements

Before releasing `zenoh-java` or `zenoh-kotlin`:

- The selected `zenoh-flat-jni` version must already resolve from Maven
  Central.
- The dependency version must be maintained in one release-controlled
  property or version catalog, not duplicated as a string.
- Local composite substitution must be optional and disabled in release CI.
- `zenoh-java`'s obsolete workflow that builds the removed
  `zenoh-jni/Cargo.toml` must be deleted.
- JVM publications must depend on the desktop JVM artifact.
- Android publications must depend on the Android artifact/variant.
- Published downstream POM and Gradle metadata must contain the intended
  `zenoh-flat-jni` dependency.
- Downstream tests must pass using only released Maven artifacts.

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
