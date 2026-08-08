pluginManagement {
    repositories {
        gradlePluginPortal()
        mavenCentral()
    }
}

rootProject.name = "zenoh-flat-jni"

println("Zenoh Flat JNI version: ${file("version.txt").readText().trim()}")
