# Zenoh Flat JNI

[![CI](https://github.com/eclipse-zenoh/zenoh-flat-jni/actions/workflows/ci.yml/badge.svg)](https://github.com/eclipse-zenoh/zenoh-flat-jni/actions/workflows/ci.yml)
[![Maven Central](https://img.shields.io/maven-central/v/org.eclipse.zenoh/zenoh-flat-jni)](https://central.sonatype.com/artifact/org.eclipse.zenoh/zenoh-flat-jni)
[![License](https://img.shields.io/badge/License-EPL%202.0%20or%20Apache%202.0-blue)](https://www.eclipse.org/legal/epl-2.0/)

Zenoh JNI bindings and Kotlin wrappers for the [Zenoh](https://zenoh.io) pub/sub, store/query and compute framework.

This project provides low-level JNI bindings (Rust) and high-level Kotlin wrappers generated from the `zenoh-flat` Rust crate using [`prebindgen`](https://github.com/milyin/prebindgen).

## Overview

The Zenoh Flat JNI library generates multi-language FFI bindings from a single annotated Rust source (`zenoh-flat`). It consists of:

- **Rust JNI layer** (`src/`) - Native bindings to Zenoh APIs
- **Kotlin wrappers** (`kotlin/`) - Type-safe Kotlin classes for JVM and Android
- **Generated Kotlin** (`generated-kotlin/`) - Auto-generated Kotlin sources from `prebindgen`

## Features

- 🦀 **Native Rust performance** - Direct JNI binding to Zenoh
- 🎯 **Type-safe API** - Kotlin data classes and enums for type safety
- 📱 **Multi-platform** - Support for JVM (Linux, macOS, Windows) and Android
- 🔒 **Memory-safe** - Automatic resource management via Kotlin classes
- 🚀 **Zero-copy** - Efficient data transfer between Rust and JVM

## Installation

### Maven Central

Add the dependency to your `build.gradle.kts`:

```kotlin
dependencies {
    implementation("org.eclipse.zenoh:zenoh-flat-jni:1.9.0")
}
```

Or in `pom.xml`:

```xml
<dependency>
    <groupId>org.eclipse.zenoh</groupId>
    <artifactId>zenoh-flat-jni</artifactId>
    <version>1.9.0</version>
</dependency>
```

## Usage

### Kotlin Example

```kotlin
import io.zenoh.jni.*

fun main() {
    // Initialize Zenoh
    val config = ZConfig()
    val session = config.openSession()
    
    // Put a key-value pair
    val keyExpr = ZKeyExpr.from("demo/example")
    session.put(keyExpr, "Hello, Zenoh!")
    
    // Close resources
    keyExpr.drop()
    session.drop()
}
```

## Development

Local setup is documented in [DEVELOPMENT.md](DEVELOPMENT.md); the Maven Central
release pipeline, artifact layouts, verification gates and dry-run procedure are
documented in [PUBLISHING.md](PUBLISHING.md).

### Prerequisites

- Rust — the version is pinned by `rust-toolchain.toml` (currently 1.97.1) and rustup installs it automatically
- JDK 11+ (Gradle comes from the committed wrapper)
- Android NDK r26 and `cargo-ndk` (for Android builds)

### Build from Source

```bash
# Clone the repository with dependencies
git clone https://github.com/eclipse-zenoh/zenoh-flat-jni.git
cd zenoh-flat-jni

# Build Rust library
cargo build --release

# Build Kotlin (optional, for local development)
./gradlew build
```

#### Local Development with Workspace

To use local versions of `zenoh-flat` and `prebindgen`:

```bash
# Ensure you have the PREBINDGEN workspace checked out locally
mkdir -p ~/zenoh-workspace
cd ~/zenoh-workspace

git clone https://github.com/milyin/prebindgen.git
git clone https://github.com/eclipse-zenoh/zenoh-flat-jni.git

cd zenoh-flat-jni

# To build against a local checkout instead of the published/Git sources,
# point Cargo at it without editing this repository's manifest — e.g. in a
# workspace-level .cargo/config.toml:
# [patch."https://github.com/eclipse-zenoh/zenoh-flat.git"]
# zenoh-flat = { path = "../zenoh-flat" }

cargo build --release
```

### Testing

```bash
# Run Rust tests
cargo test --all

# Run Kotlin tests
./gradlew jvmTest

# Lint
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
```

### Building for Android

```bash
./gradlew androidAar verifyAndroidArtifact
```

That cross-compiles all four ABIs and packages the AAR. One-time prerequisites,
as for any cross-compilation:

```bash
rustup target add armv7-linux-androideabi aarch64-linux-android \
                  i686-linux-android x86_64-linux-android
cargo install cargo-ndk --locked --version 4.1.2
export ANDROID_NDK_HOME=/path/to/android-ndk-r26
```

`verifyAndroidArtifact` fails unless the AAR carries a manifest, `classes.jar`,
`R.txt` and all four ABI libraries. The release runs the same
`buildAndroidLibs` task, so this is not a parallel developer-only path — see
[PUBLISHING.md](PUBLISHING.md).

The cross-compilation runs every time — Cargo decides what is actually stale, so
a Rust change is always picked up. (The release's publish job passes
`-PprebuiltAndroidLibs=true` to package libraries it downloaded as build
artifacts; that is the only path that skips it.)

## Architecture

The generated bindings are created via `prebindgen` in the build process:

1. **Rust source** (`zenoh-flat`) marked with `#[prebindgen]` annotations
2. **Proc-macro** captures annotated items to JSONL format
3. **`prebindgen_jni::JniGen`** reads JSONL and generates:
   - Rust JNI wrapper functions
   - Kotlin data classes and enums
4. **Gradle** packages Rust dylib + Kotlin sources into a JAR
5. **Maven Central** publishes the multi-platform JAR

## Integration with zenoh-java

`zenoh-flat-jni` is a standalone library that can be consumed by:

- **[zenoh-java](https://github.com/eclipse-zenoh/zenoh-java)** - Higher-level Zenoh API for JVM/Android
- **[zenoh-kotlin](https://github.com/eclipse-zenoh/zenoh-kotlin)** - Kotlin-first Zenoh API

Both projects depend on `zenoh-flat-jni` as a Maven artifact, enabling independent versioning and release cycles.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under either of

- Eclipse Public License 2.0 ([LICENSE](LICENSE) or http://www.eclipse.org/legal/epl-2.0)
- Apache License, Version 2.0 ([LICENSE](LICENSE) or https://www.apache.org/licenses/LICENSE-2.0)

at your option.

## Acknowledgments

This project is part of the [Eclipse Zenoh](https://zenoh.io) ecosystem and is maintained by the [ZettaScale](https://zettascale.tech) team.
