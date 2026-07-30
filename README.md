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

### Prerequisites

- Rust 1.70+ with JNI support
- Gradle 7.0+
- JDK 11+
- Android SDK (for Android builds)

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

# Adjust paths in Cargo.toml (temporarily):
# zenoh-flat = { version = "1.9.0", path = "../../PREBINDGEN/zenoh-flat", ... }
# prebindgen = { path = "../../PREBINDGEN/prebindgen/prebindgen" }

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
./gradlew -Pandroid=true build
```

## Architecture

The generated bindings are created via `prebindgen` in the build process:

1. **Rust source** (`zenoh-flat`) marked with `#[prebindgen]` annotations
2. **Proc-macro** captures annotated items to JSONL format
3. **`prebindgen::lang::JniGen`** reads JSONL and generates:
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
