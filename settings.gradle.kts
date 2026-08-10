pluginManagement {
    repositories {
        gradlePluginPortal()
        mavenCentral()
        google()   // the Android Gradle Plugin
    }
}

rootProject.name = "zenoh-flat-jni"

println("Zenoh Flat JNI version: ${file("version.txt").readText().trim()}")
