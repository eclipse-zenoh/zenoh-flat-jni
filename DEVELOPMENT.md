# Local Development

This document describes how to set up zenoh-flat-jni for local development.

## Prerequisites

- Rust — pinned by `rust-toolchain.toml` (currently 1.97.1); [rustup](https://rustup.rs/) installs that exact toolchain automatically, and CI and releases use the same one
- JDK 11+ (Gradle comes from the committed wrapper — use `./gradlew`)

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

### Where the dependencies come from

Nothing needs to sit next to this repository. The generator is a published
crate, and the zenoh side is declared the way every zenoh binding declares it:

```toml
# [dependencies] and [build-dependencies]
zenoh-flat = { version = "1.9.0", git = "https://github.com/eclipse-zenoh/zenoh-flat.git", branch = "main", features = ["unstable"] }
prebindgen-jni = "0.5"          # build-dependency, from crates.io
prebindgen-registry = "0.5"     # build-dependency, from crates.io
prebindgen-jni-runtime = "0.5"  # runtime, from crates.io
```

### Building against a local checkout

Do not edit this repository's manifest for that — a modified `Cargo.toml`
invalidates the committed `Cargo.lock`, which release builds consume with
`--locked`. Override the source from outside instead, e.g. in a
workspace-level `.cargo/config.toml` that sits above every repository you have
checked out:

```toml
[patch."https://github.com/eclipse-zenoh/zenoh-flat.git"]
zenoh-flat = { path = "zenoh-flat" }

[patch.crates-io]
prebindgen-jni = { path = "prebindgen/prebindgen-jni" }
prebindgen-registry = { path = "prebindgen/prebindgen-registry" }
prebindgen-jni-runtime = { path = "prebindgen/prebindgen-jni-runtime" }
```

### Published Versions (CI/Release)

Nothing has to sit next to this repository any more. `prebindgen-*` comes from
crates.io as a version constraint, and `zenoh`, `zenoh-ext` and `zenoh-flat` are
Git dependencies on `branch = "main"` — the shape every zenoh binding uses. A
bare clone therefore builds.

Which commit of each of those a build actually resolves is fixed by the
committed `Cargo.lock`, which the shared lockfile-sync bot keeps aligned with
Zenoh's own. Release builds run `--locked`, so a lockfile that no longer matches
the manifest fails the build rather than resolving something else.

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
`android-libs/` (see [PUBLISHING.md](PUBLISHING.md)). Use the cargo-ndk and NDK
versions pinned in
[`publish-android.yml`](.github/workflows/publish-android.yml) — a release built
with anything else is not the artifact CI verified.

```bash
cargo install cargo-ndk --locked --version 4.1.2   # CARGO_NDK_VERSION
rustup target add armv7-linux-androideabi aarch64-linux-android i686-linux-android x86_64-linux-android

# ANDROID_NDK_HOME must point at the pinned NDK (r26)
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

The Maven Central release pipeline, the artifact layouts it produces, its
verification gates, the required secrets, and the known gaps are documented in
[PUBLISHING.md](PUBLISHING.md).

To rehearse a publication locally without touching Central, populate `jni-libs/`
(and optionally `android-libs/`) and publish to the same isolated file-based
repository the CI consumer test uses:

```bash
./gradlew publishAllPublicationsToDryRunRepository
find build/dry-run-repository -type f

cd ci/consumer-smoke-test
gradle run -PcandidateRepository="file://$PWD/../../build/dry-run-repository" \
           -PcandidateVersion="$(cat ../../version.txt)"
```

Do not use `mavenLocal()` for this: it can serve leftovers from earlier builds,
which makes it impossible to prove which repository supplied an artifact.

## Documentation

- [Zenoh](https://zenoh.io/)
- [prebindgen](https://github.com/milyin/prebindgen)
- [zenoh-flat](https://github.com/milyin/prebindgen/tree/main/zenoh-flat)
- [zenoh-java](https://github.com/eclipse-zenoh/zenoh-java)
