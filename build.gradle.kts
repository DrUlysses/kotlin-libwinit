allprojects {
    repositories {
        mavenCentral()
    }
}

plugins {
    alias(libs.plugins.kotlin.multiplatform) apply false
    alias(libs.plugins.kotlin.atomicfu) apply false
    alias(libs.plugins.gobley.uniffi) apply false
    alias(libs.plugins.gobley.cargo) apply false
    alias(libs.plugins.gobley.rust) apply false
}
