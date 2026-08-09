//
// An external consumer of the zenoh-flat-jni Maven artifact — deliberately not
// part of the main Gradle build and with no path or composite dependency on it.
// It proves that a release candidate resolves, that its POM metadata is usable,
// and that the native library extracts and loads out of the published JAR.
//
// Run against a candidate:
//   gradle run -PcandidateRepository=<url> -PcandidateVersion=<version>
//
// Against Maven Central, pass the Central URL as candidateRepository.
//
plugins {
    kotlin("jvm") version "1.9.0"
    application
}

val candidateRepository: String by project
val candidateVersion: String by project

repositories {
    // The content filters make the resolution source unambiguous: the artifact
    // can only come from the candidate repository, never from a cached Central
    // copy of an earlier release with the same coordinates.
    maven {
        name = "candidate"
        url = uri(candidateRepository)
        content { includeGroup("org.eclipse.zenoh") }
    }
    mavenCentral {
        content { excludeGroup("org.eclipse.zenoh") }
    }
}

dependencies {
    implementation("org.eclipse.zenoh:zenoh-flat-jni:$candidateVersion")
}

kotlin {
    jvmToolchain(11)
}

application {
    mainClass.set("smoke.SmokeTestKt")
}
