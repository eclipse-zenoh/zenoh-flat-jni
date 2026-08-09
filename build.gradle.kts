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

import java.util.zip.ZipFile
import java.util.zip.ZipInputStream

plugins {
    kotlin("jvm") version "1.9.0"
    id("org.jetbrains.dokka-javadoc") version "2.0.0"
    id("io.github.gradle-nexus.publish-plugin") version "2.0.0"
    `maven-publish`
    signing
}

// Keep in step with the `kotlin("jvm")` plugin version above — it is the stdlib
// version written into the hand-built Android POM.
val kotlinVersion = "1.9.0"

group = "org.eclipse.zenoh"

// version.txt is the single source of truth; the release workflow checks that
// the tag and Cargo.toml agree with it.
val baseVersion = file("version.txt").readText().trim()
version = if (project.hasProperty("SNAPSHOT")) "$baseVersion-SNAPSHOT" else baseVersion

repositories {
    mavenCentral()
}

// ============================================================================
// Build Configuration
// ============================================================================

// NOTE: these are Gradle *project* properties (-P…), not system properties (-D…).
val isRemotePublication = project.findProperty("remotePublication")?.toString()?.toBoolean() == true
val release = project.findProperty("release")?.toString()?.toBoolean() == true

enum class BuildMode {
    DEBUG, RELEASE
}

val buildMode = if (release) BuildMode.RELEASE else BuildMode.DEBUG

// The desktop targets advertised by `NativeLibrary.determineTarget()`. A release
// JAR must carry one `<target>/<target>.zip` resource per entry — that is the
// layout strategy 3 of the loader expects.
val desktopTargets = mapOf(
    "x86_64-unknown-linux-gnu" to "libzenoh_flat_jni.so",
    "aarch64-unknown-linux-gnu" to "libzenoh_flat_jni.so",
    "x86_64-apple-darwin" to "libzenoh_flat_jni.dylib",
    "aarch64-apple-darwin" to "libzenoh_flat_jni.dylib",
    "x86_64-pc-windows-msvc" to "zenoh_flat_jni.dll",
    "aarch64-pc-windows-msvc" to "zenoh_flat_jni.dll",
)

// The Android ABIs advertised for the AAR publication.
val androidAbis = listOf("armeabi-v7a", "arm64-v8a", "x86", "x86_64")

// The release workflow's cross-build matrix drops its results here; both
// directories are absent in a normal developer build.
//   jni-libs/<target>/<target>.zip          -> multi-platform desktop JAR
//   android-libs/<abi>/libzenoh_flat_jni.so -> Android AAR
val jniLibsDir = file("jni-libs")
val androidLibsDir = file("android-libs")
val isMultiPlatform = jniLibsDir.isDirectory
val isAndroidBuild = androidLibsDir.isDirectory

kotlin {
    jvmToolchain(11)
    sourceSets {
        main {
            kotlin.srcDirs("kotlin", "generated-kotlin")
            // Empty/absent in a developer build, so this is a no-op there.
            resources.srcDir(jniLibsDir)
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
val hostDylibName = when {
    System.getProperty("os.name").lowercase().contains("win") -> "zenoh_flat_jni.dll"
    System.getProperty("os.name").lowercase().contains("mac") -> "libzenoh_flat_jni.dylib"
    else -> "libzenoh_flat_jni.so"
}

// A developer build bundles only the host library at the JAR root (loader
// strategy 2) and has to compile it first. A release build takes every native
// library from `jni-libs` as a resource, so it must NOT also bundle a host
// library — a stray extra copy is exactly what `verifyDesktopArtifact` rejects.
if (!isMultiPlatform) {
    tasks.named<Jar>("jar") {
        dependsOn("buildZenohFlatJni")
        from(jarTarget) {
            include(hostDylibName)
        }
    }
}

// ============================================================================
// Publication artifacts
// ============================================================================

// The release pipeline assembles the candidate in one job and compares its
// hash against what Maven Central ends up serving; that only means something
// if the archives are byte-reproducible.
tasks.withType<AbstractArchiveTask>().configureEach {
    isPreserveFileTimestamps = false
    isReproducibleFileOrder = true
}

val sourcesJar by tasks.registering(Jar::class) {
    archiveClassifier.set("sources")
    from("kotlin")
    from("generated-kotlin")
    from("src")
    from("build.rs")
    from("Cargo.toml")
}

// Maven Central requires a documentation artifact for every publication.
val javadocJar by tasks.registering(Jar::class) {
    dependsOn("dokkaGeneratePublicationJavadoc")
    archiveClassifier.set("javadoc")
    from(layout.buildDirectory.dir("dokka/javadoc"))
}

// ── Android AAR ─────────────────────────────────────────────────────────────
//
// ponytail: the AAR is assembled as a plain Zip rather than by the Android
// Gradle Plugin. This module has no Android resources, no manifest entries and
// no Android-only code — an AAR here is literally `classes.jar` plus
// `jni/<abi>/`, so applying AGP would mean an Android SDK install in CI to zip
// four .so files. Switch to `com.android.library` if this ever grows real
// Android resources or a non-trivial manifest.
val androidStubsDir = layout.buildDirectory.dir("android")

// AndroidManifest.xml and R.txt are mandatory in the AAR layout; this module
// declares no Android resources, so R.txt is empty.
val androidStubs by tasks.registering {
    outputs.dir(androidStubsDir)
    doLast {
        val dir = androidStubsDir.get().asFile.apply { mkdirs() }
        dir.resolve("AndroidManifest.xml").writeText(
            """
            <?xml version="1.0" encoding="utf-8"?>
            <manifest xmlns:android="http://schemas.android.com/apk/res/android"
                package="io.zenoh.jni">
                <uses-sdk android:minSdkVersion="21" />
            </manifest>
            """.trimIndent() + "\n"
        )
        dir.resolve("R.txt").writeText("")
    }
}

// The Android classes.jar must not carry the desktop native zips, hence
// `output.classesDirs` instead of the `jar` task.
val androidClassesJar by tasks.registering(Jar::class) {
    archiveClassifier.set("android-classes")
    from(sourceSets["main"].output.classesDirs)
}

// Cross-compile the Android ABIs, mirroring `buildZenohFlatJni` for the desktop
// library: one Gradle entry point, so the documented developer command and CI run
// the same code path rather than two copies of it.
//
// It runs cargo-ndk every time. Cargo is the incremental build system here, and
// it is the only one that knows whether the sources, Cargo.lock or the profile
// moved; skipping on the mere presence of four correctly-named files would
// happily package stale — or foreign — libraries.
//
// The single exception is `-PprebuiltAndroidLibs=true`, which the release's
// publish job passes: there `android-libs/` arrives as a downloaded build
// artifact and the runner has no NDK, so building is neither possible nor
// wanted.
val prebuiltAndroidLibs = project.findProperty("prebuiltAndroidLibs")?.toString()?.toBoolean() == true

val buildAndroidLibs by tasks.registering {
    description = "Cross-compile the Android ABIs into android-libs/ using cargo-ndk"
    onlyIf {
        if (prebuiltAndroidLibs) {
            logger.lifecycle("-PprebuiltAndroidLibs=true: using the android-libs/ already present")
        }
        !prebuiltAndroidLibs
    }
    doLast {
        val command = mutableListOf("cargo", "ndk", "-o", androidLibsDir.name)
        androidAbis.forEach { command += listOf("-t", it) }
        command += listOf("build", "--release", "--locked")

        val result = project.exec {
            commandLine(command)
            isIgnoreExitValue = true
        }
        if (result.exitValue != 0) {
            throw GradleException(
                "cargo ndk failed (exit ${result.exitValue}). " +
                    "Needs cargo-ndk installed, the four Android targets added, " +
                    "and ANDROID_NDK_HOME pointing at the NDK."
            )
        }
    }
}

val androidAar by tasks.registering(Zip::class) {
    dependsOn(buildAndroidLibs)
    archiveBaseName.set("zenoh-flat-jni-android")
    archiveVersion.set(project.version.toString())
    archiveExtension.set("aar")
    from(androidStubs)
    from(androidClassesJar) { rename { "classes.jar" } }
    from(androidLibsDir) { into("jni") }
}

// ============================================================================
// Artifact verification
// ============================================================================

private fun ZipFile.nestedEntryNames(name: String): List<String> =
    ZipInputStream(getInputStream(getEntry(name))).use { zip ->
        generateSequence { zip.nextEntry }.map { it.name }.toList()
    }

val verifyDesktopArtifact by tasks.registering {
    description = "Fail unless the JVM JAR carries exactly one native library per advertised desktop target"
    dependsOn(tasks.jar)
    doLast {
        val jarFile = tasks.jar.get().archiveFile.get().asFile
        val problems = mutableListOf<String>()
        ZipFile(jarFile).use { jar ->
            val names = jar.entries().toList().map { it.name }
            desktopTargets.forEach { (target, libName) ->
                val resource = "$target/$target.zip"
                when (val entry = jar.getEntry(resource)) {
                    null -> problems += "missing native resource `$resource`"
                    else -> {
                        val inner = jar.nestedEntryNames(entry.name)
                        if (inner != listOf(libName)) {
                            problems += "`$resource` contains $inner, expected exactly [$libName]"
                        }
                    }
                }
            }
            // A host library at the JAR root would shadow the per-target
            // resources on whichever platform happens to match it.
            names.filter { !it.contains('/') && (it.endsWith(".so") || it.endsWith(".dylib") || it.endsWith(".dll")) }
                .forEach { problems += "unexpected native library `$it` at the JAR root" }
        }
        if (problems.isNotEmpty()) {
            throw GradleException("$jarFile is not a valid multi-platform artifact:\n  " + problems.joinToString("\n  "))
        }
        logger.lifecycle("$jarFile carries all ${desktopTargets.size} desktop targets")
    }
}

val verifyAndroidArtifact by tasks.registering {
    description = "Fail unless the AAR carries a native library for every advertised Android ABI"
    dependsOn(androidAar)
    doLast {
        val aarFile = androidAar.get().archiveFile.get().asFile
        val problems = mutableListOf<String>()
        ZipFile(aarFile).use { aar ->
            listOf("AndroidManifest.xml", "classes.jar", "R.txt").forEach {
                if (aar.getEntry(it) == null) problems += "missing `$it`"
            }
            androidAbis.forEach { abi ->
                if (aar.getEntry("jni/$abi/libzenoh_flat_jni.so") == null) {
                    problems += "missing `jni/$abi/libzenoh_flat_jni.so`"
                }
            }
        }
        if (problems.isNotEmpty()) {
            throw GradleException("$aarFile is not a valid Android artifact:\n  " + problems.joinToString("\n  "))
        }
        logger.lifecycle("$aarFile carries all ${androidAbis.size} Android ABIs")
    }
}

// A remote publication that silently ships the publishing runner's own library
// is the failure mode this whole pipeline exists to prevent.
if (isRemotePublication && !isMultiPlatform && !isAndroidBuild) {
    throw GradleException(
        "-PremotePublication=true requires cross-built natives in `jni-libs/` (desktop) " +
            "or `android-libs/` (Android); see PUBLISHING.md."
    )
}

// ============================================================================
// Maven Publishing Configuration
// ============================================================================

fun MavenPom.describe(artifact: String) {
    name.set(if (artifact.endsWith("-android")) "Zenoh Flat JNI (Android)" else "Zenoh Flat JNI")
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

// The Central Publisher Portal's OSSRH Staging API. The retired
// s01.oss.sonatype.org endpoint is gone; credentials are Central Portal tokens.
nexusPublishing {
    repositories {
        sonatype {
            nexusUrl = uri("https://ossrh-staging-api.central.sonatype.com/service/local/")
            snapshotRepositoryUrl = uri("https://central.sonatype.com/repository/maven-snapshots/")
            username = System.getenv("CENTRAL_SONATYPE_TOKEN_USERNAME")
            password = System.getenv("CENTRAL_SONATYPE_TOKEN_PASSWORD")
        }
    }
}

publishing {
    repositories {
        // Local Maven repository for development.
        mavenLocal()
        // Isolated file-based repository for release dry-runs; consumer tests
        // resolve the candidate from here with composite builds disabled.
        maven {
            name = "dryRun"
            url = uri(layout.buildDirectory.dir("dry-run-repository"))
        }
        // `sonatype` is contributed by the nexus-publish plugin above.
    }

    publications {
        register<MavenPublication>("maven") {
            artifactId = "zenoh-flat-jni"

            from(components["java"])
            artifact(sourcesJar)
            artifact(javadocJar)

            pom { describe(artifactId) }
        }

        if (isAndroidBuild) {
            register<MavenPublication>("android") {
                artifactId = "zenoh-flat-jni-android"

                artifact(androidAar)
                artifact(sourcesJar)
                artifact(javadocJar)

                pom {
                    packaging = "aar"
                    describe(artifactId)
                    // `artifact()` publications carry no dependencies of their
                    // own; the Kotlin plugin's implicit stdlib is the only one.
                    withXml {
                        asNode().appendNode("dependencies").appendNode("dependency").apply {
                            appendNode("groupId", "org.jetbrains.kotlin")
                            appendNode("artifactId", "kotlin-stdlib")
                            appendNode("version", kotlinVersion)
                            appendNode("scope", "runtime")
                        }
                    }
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

// ============================================================================
// Task Dependencies
// ============================================================================

tasks.withType<PublishToMavenRepository>().configureEach {
    dependsOn(tasks.withType<Sign>())
    if (isMultiPlatform) dependsOn(verifyDesktopArtifact)
    if (isAndroidBuild) dependsOn(verifyAndroidArtifact)
}
