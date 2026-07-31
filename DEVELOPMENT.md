# Local Development

This document describes how to set up zenoh-flat-jni for local development.

## Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- JDK 11+ (for Gradle builds)
- Gradle 7.0+

## Standard Setup (Maven Central)

For normal usage, simply add the Maven dependency to your `build.gradle.kts`:

```kotlin
dependencies {
    implementation("org.eclipse.zenoh:zenoh-flat-jni:1.9.0")
}
```

## Local Development Setup

If you're contributing to zenoh-flat-jni or need to test local changes, follow these steps:

### Option 1: Direct Build (Recommended for Local Testing)

```bash
# Clone the repository
git clone https://github.com/eclipse-zenoh/zenoh-flat-jni.git
cd zenoh-flat-jni

# Build Rust library
cargo build --release

# Optional: Run tests
cargo test --all

# Optional: Build Kotlin JAR
./gradlew build
```

### Option 2: Local Maven Publication

If you need to test against zenoh-java or another project locally:

```bash
cd zenoh-flat-jni

# Build and publish to local Maven repository
./gradlew publishToMavenLocal

# Verify publication
ls ~/.m2/repository/org/eclipse/zenoh/zenoh-flat-jni/
```

Then in zenoh-java (or another consumer):

```bash
# The local version will be picked up automatically if versions match
cd zenoh-java
./gradlew build
```

### Option 3: Gradle Composite Build (For coordinated development)

This is the best option for simultaneous development of zenoh-flat-jni and zenoh-java.

**In zenoh-java's `settings.gradle.kts`, uncomment the composite build line:**

```kotlin
includeBuild("../zenoh-flat-jni")
```

Then in zenoh-java's `build.gradle.kts`, ensure the dependency version matches:

```kotlin
implementation("org.eclipse.zenoh:zenoh-flat-jni:1.9.0")
```

With this setup:
- Gradle will automatically build zenoh-flat-jni locally instead of fetching from Maven Central
- Changes to zenoh-flat-jni are immediately reflected in zenoh-java builds
- No manual publishing to Maven Local is needed

To use this setup:

```bash
# Create a workspace directory
mkdir -p ~/zenoh-workspace
cd ~/zenoh-workspace

# Clone both repositories
git clone https://github.com/eclipse-zenoh/zenoh-flat-jni.git
git clone https://github.com/eclipse-zenoh/zenoh-java.git

# Edit zenoh-java/settings.gradle.kts to uncomment includeBuild
cd zenoh-java
sed -i 's|// includeBuild|includeBuild|' settings.gradle.kts

# Build - Gradle will automatically build zenoh-flat-jni first
./gradlew build
```

## Dependency Management

### Path Dependencies (Internal Build Only)

The `Cargo.toml` in zenoh-flat-jni uses path dependencies for zenoh-flat and prebindgen:

```toml
zenoh-flat = { version = "1.9.0", path = "../zenoh-flat", features = ["unstable"] }
prebindgen = { path = "../prebindgen/prebindgen" }
```

To use these for local development:

```bash
# Ensure the PREBINDGEN workspace is checked out as a sibling
cd ~/workspace
git clone https://github.com/milyin/prebindgen.git
cd prebindgen/zenoh-flat
# zenoh-flat is inside the prebindgen workspace

# Or manually adjust Cargo.toml to point to your local zenoh-flat
```

### Published Versions (CI/Release)

CI and the release pipeline check both repositories out as siblings of this one,
at the revisions pinned in [`release-inputs.env`](release-inputs.env), so a bare
clone builds without a PREBINDGEN workspace. Keeping the manifests untouched is
what lets releases build `--locked` against the committed `Cargo.lock`.

`Cargo.lock` must be regenerated and committed whenever a pinned revision moves —
the release build fails otherwise. The path dependencies are temporary; they
become version constraints once zenoh-flat and prebindgen reach crates.io.

## Testing

```bash
# Run all Rust tests
cargo test --all

# Run with verbose output
cargo test --all --verbose

# Run specific test
cargo test test_name

# Run Kotlin tests (requires built Rust library)
./gradlew jvmTest

# Run both Rust and Kotlin tests
cargo test --all && ./gradlew jvmTest
```

## Linting & Formatting

```bash
# Format check
cargo fmt --check

# Format with fixes
cargo fmt

# Clippy lint
cargo clippy --all-targets --all-features -- -D warnings
```

## Building for Android

The Android ABIs are cross-compiled with [cargo-ndk](https://github.com/bbqsrc/cargo-ndk)
straight into the AAR's `jni/<abi>/` layout; the AAR is then assembled from
`android-libs/` (see [Releasing](#releasing)).

```bash
cargo install cargo-ndk
rustup target add armv7-linux-androideabi aarch64-linux-android i686-linux-android x86_64-linux-android

# ANDROID_NDK_HOME must point at NDK r26
cargo ndk -o android-libs -t armeabi-v7a -t arm64-v8a -t x86 -t x86_64 build --release

./gradlew androidAar verifyAndroidArtifact
```

## CI/CD

When you push commits or create pull requests, GitHub Actions will automatically:
- Run format checks (`cargo fmt --check`)
- Run linter (`cargo clippy`)
- Run tests (`cargo test`)
- Build on Linux, macOS, and Windows

## Troubleshooting

### "Cannot find zenoh-flat" or "Cannot find prebindgen"

Make sure the path dependencies are correct in `Cargo.toml`. They should point to:
- `../zenoh-flat` (or adjust to your actual path)
- `../prebindgen/prebindgen`

### Gradle says "Cannot resolve org.eclipse.zenoh:zenoh-flat-jni"

This typically means:
1. The package hasn't been published to Maven Central yet
2. You're not using a local build (Option 2 or 3 above)
3. The version number doesn't match

Solution: Use `./gradlew publishToMavenLocal` first, then build.

### Tests fail with "Cannot load library"

This means the Rust library wasn't built or isn't found. Run:
```bash
cargo build --release
```

Then try tests again.

## Releasing

Maven Central releases are immutable, so the pipeline in
[`.github/workflows/publish.yml`](.github/workflows/publish.yml) puts every gate
*before* the irreversible step. Pushing a `v*` tag runs, in order:

1. **validate** — version.txt, `Cargo.toml` and the tag must agree, and the
   version must not already exist on Maven Central.
2. **generated-sources** — rebuilds the bindings from the pinned inputs and
   fails if the committed generated Rust/Kotlin differs.
3. **desktop-natives / android-natives** — `cargo build --release --locked` per
   target, packaged as `<target>/<target>.zip` and `jni/<abi>/`.
4. **consumer-test** — assembles the JAR and AAR, verifies their contents, then
   resolves and runs [`ci/consumer-smoke-test`](ci/consumer-smoke-test) against
   an isolated file-based repository on every runner platform.
5. **stage** — uploads and *closes* a Central staging deployment. Central
   validates it; nothing is public yet.
6. **release-staging** — releases the deployment. Guarded by the `maven-central`
   GitHub environment; protect it with required reviewers.
7. **verify-central** — waits for the coordinates to resolve, checks the served
   JAR hash against the one CI verified, and reruns the consumer smoke test.
8. **github-release** — only now.

Two artifacts are published: `org.eclipse.zenoh:zenoh-flat-jni` (universal
desktop JVM JAR, natives for all six targets) and
`org.eclipse.zenoh:zenoh-flat-jni-android` (AAR, four ABIs).

### Dry run

`workflow_dispatch` with `stage_only=true` (the default) runs everything up to
and including Central staging validation, then stops — drop the deployment from
the Central Portal afterwards.

To rehearse locally without touching Central, populate `jni-libs/` (and
optionally `android-libs/`) and publish to the file-based repository the
consumer test uses:

```bash
./gradlew publishAllPublicationsToDryRunRepository
find build/dry-run-repository -type f

cd ci/consumer-smoke-test
gradle run -PcandidateRepository="file://$PWD/../../build/dry-run-repository" \
           -PcandidateVersion="$(cat ../../version.txt)"
```

Do not use `mavenLocal()` for this: it can serve leftovers from earlier builds,
which makes it impossible to prove which repository supplied an artifact.

### Required secrets

`CENTRAL_SONATYPE_TOKEN_USERNAME` / `CENTRAL_SONATYPE_TOKEN_PASSWORD` (Central
Portal user tokens — the retired `s01.oss.sonatype.org` OSSRH credentials no
longer work), plus `ORG_GPG_SUBKEY_ID`, `ORG_GPG_PRIVATE_KEY` and
`ORG_GPG_PASSPHRASE`.

## Documentation

- [Zenoh](https://zenoh.io/)
- [prebindgen](https://github.com/milyin/prebindgen)
- [zenoh-flat](https://github.com/milyin/prebindgen/tree/main/zenoh-flat)
- [zenoh-java](https://github.com/eclipse-zenoh/zenoh-java)
