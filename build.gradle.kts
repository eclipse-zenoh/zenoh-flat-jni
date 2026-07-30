//
// Copyright (c) 2023 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//

plugins {
    kotlin("jvm") version "1.9.0"
    `maven-publish`
    signing
}

group = "org.eclipse.zenoh"
version = rootProject.version

repositories {
    mavenCentral()
}

// ============================================================================
// Build Configuration
// ============================================================================

val isRemotePublication = project.findProperty("remotePublication")?.toString()?.toBoolean() == true
val release = project.findProperty("release")?.toString()?.toBoolean() == true

enum class BuildMode {
    DEBUG, RELEASE
}

val buildMode = if (release) BuildMode.RELEASE else BuildMode.DEBUG

kotlin {
    jvmToolchain(11)
    sourceSets {
        main {
            kotlin.srcDirs("kotlin", "generated-kotlin")
        }
    }
}

dependencies {
    // Self-verification: the correspondence tests compare this crate's pure-Kotlin
    // implementations against the native oracle it also ships (io.zenoh.jni.test).
    // kotlin("test") with useJUnitPlatform() selects the JUnit5 integration.
    testImplementation(kotlin("test"))
    testImplementation("org.junit.jupiter:junit-jupiter:5.10.0")
    testRuntimeOnly("org.junit.platform:junit-platform-launcher")
    // Guava TypeToken — the java.lang.reflect.Type serialization path.
    testImplementation("com.google.guava:guava:33.3.1-jre")
}

tasks.test {
    useJUnitPlatform()
    // The native lib must exist and be loadable. `NativeLibrary.ensureLoaded()`
    // tries `System.loadLibrary("zenoh_flat_jni")` first, so point
    // `java.library.path` at the freshly-built RELEASE dylib in target/release.
    //
    // The tests always use the *release* dylib, never debug: linking the debug
    // cdylib on Windows exceeds link.exe's 65535-object limit (LNK1189) because
    // the ~250-crate zenoh graph is linked object-by-object with no LTO. The
    // release profile (`lto = "fat"` + `codegen-units = 1`) collapses the graph
    // and links cleanly — this is the same dylib the `Build` CI job produces.
    dependsOn("buildZenohFlatJniRelease")
    systemProperty("java.library.path", file("target/release").absolutePath)
}

// ============================================================================
// Rust Build Configuration
// ============================================================================

tasks.register("buildZenohFlatJni") {
    description = "Build zenoh-flat-jni Rust library"
    doLast {
        val cargoCommand = mutableListOf("cargo", "build")
        if (buildMode == BuildMode.RELEASE) {
            cargoCommand.add("--release")
        }

        val result = project.exec {
            commandLine(*(cargoCommand.toTypedArray()))
            isIgnoreExitValue = true
        }

        if (result.exitValue != 0) {
            throw GradleException("Failed to build zenoh-flat-jni. Exit code: ${result.exitValue}")
        }
    }
}

// Release build used by the `test` task (see the LNK1189 note there); always
// `--release` regardless of the `-Prelease` property.
tasks.register("buildZenohFlatJniRelease") {
    description = "Build the release zenoh-flat-jni Rust library for the test suite"
    doLast {
        val result = project.exec {
            commandLine("cargo", "build", "--release")
            isIgnoreExitValue = true
        }
        if (result.exitValue != 0) {
            throw GradleException("Failed to build zenoh-flat-jni (release). Exit code: ${result.exitValue}")
        }
    }
}

// ============================================================================
// JAR Packaging
// ============================================================================

val jarTarget = if (buildMode == BuildMode.RELEASE) "target/release" else "target/debug"
val dylibExt = when {
    System.getProperty("os.name").lowercase().contains("mac") -> "dylib"
    System.getProperty("os.name").lowercase().contains("win") -> "dll"
    else -> "so"
}
val dylibName = when {
    System.getProperty("os.name").lowercase().contains("win") -> "zenoh_flat_jni.dll"
    System.getProperty("os.name").lowercase().contains("mac") -> "libzenoh_flat_jni.dylib"
    else -> "libzenoh_flat_jni.so"
}

tasks.named<Jar>("jar") {
    dependsOn("buildZenohFlatJni")
    from(jarTarget) {
        include(dylibName)
    }
}

tasks.register<Jar>("packageJar") {
    description = "Package zenoh-flat-jni library into JAR with native libraries and Kotlin sources"
    dependsOn("buildZenohFlatJni")
    
    archiveBaseName.set("zenoh-flat-jni")
    archiveVersion.set(project.version.toString())
    
    from(sourceSets["main"].output)
    
    // Include native library
    // Multi-platform JAR would include libs for all platforms
    // For now, include current platform's library
    val platformPath = "META-INF/lib"
    from(jarTarget) {
        include(dylibName)
    }
}

// ============================================================================
// Maven Publishing Configuration
// ============================================================================

publishing {
    repositories {
        if (isRemotePublication) {
            // Publish to Maven Central via OSSRH
            maven("https://s01.oss.sonatype.org/service/local/staging/deploy/maven2/") {
                name = "ossrh"
                credentials {
                    username = System.getenv("OSSRH_USERNAME") ?: ""
                    password = System.getenv("OSSRH_PASSWORD") ?: ""
                }
            }
        } else {
            // Local Maven repository for development
            mavenLocal()
        }
    }

    publications {
        register<MavenPublication>("maven") {
            groupId = "org.eclipse.zenoh"
            artifactId = "zenoh-flat-jni"
            version = project.version.toString()

            from(components["java"])

            // Include sources JAR
            val sourcesJar = tasks.register<Jar>("sourcesJar") {
                archiveBaseName.set("zenoh-flat-jni")
                archiveVersion.set(project.version.toString())
                archiveClassifier.set("sources")
                
                from("src")
                from("kotlin")
                from("build.rs")
                from("Cargo.toml")
            }
            artifact(sourcesJar)

            pom {
                name.set("Zenoh Flat JNI")
                description.set("Zenoh JNI bindings and Kotlin wrappers - generated from zenoh-flat via prebindgen")
                url.set("https://zenoh.io/")

                licenses {
                    license {
                        name.set("Eclipse Public License 2.0 OR Apache License 2.0")
                        url.set("http://www.eclipse.org/legal/epl-2.0")
                    }
                }

                developers {
                    developer {
                        id.set("ZettaScale")
                        name.set("ZettaScale Zenoh Team")
                        email.set("zenoh@zettascale.tech")
                    }
                }

                scm {
                    connection.set("scm:git:https://github.com/eclipse-zenoh/zenoh-flat-jni.git")
                    developerConnection.set("scm:git:https://github.com/eclipse-zenoh/zenoh-flat-jni.git")
                    url.set("https://github.com/eclipse-zenoh/zenoh-flat-jni")
                }
            }
        }
    }
}

// ============================================================================
// Signing Configuration
// ============================================================================

signing {
    isRequired = isRemotePublication
    if (isRemotePublication) {
        useInMemoryPgpKeys(
            System.getenv("ORG_GPG_SUBKEY_ID"),
            System.getenv("ORG_GPG_PRIVATE_KEY"),
            System.getenv("ORG_GPG_PASSPHRASE")
        )
    }
    sign(publishing.publications)
}

// Ensure signing happens after artifact generation
tasks.withType<Sign>().configureEach {
    dependsOn("packageJar")
}

// ============================================================================
// Task Dependencies
// ============================================================================

tasks.named("publishMavenPublicationTo" + if (isRemotePublication) "Ossrh" else "MavenLocal" + "Repository") {
    dependsOn(tasks.withType<Sign>())
}
