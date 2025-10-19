import gobley.gradle.rust.dsl.*
import org.gradle.api.publish.PublishingExtension
import org.gradle.api.publish.maven.MavenPublication

group = "dr.ulysses"
version = "1.0.0"

plugins {
    alias(libs.plugins.kotlin.multiplatform)
    alias(libs.plugins.kotlin.atomicfu)
    alias(libs.plugins.gobley.uniffi)
    alias(libs.plugins.gobley.cargo)
    alias(libs.plugins.gobley.rust)
    `maven-publish`
}

kotlin {
    hostNativeTarget()

    linuxX64 {
        compilations.getByName("main") {
            useRustUpLinker()
        }
    }

    sourceSets.all {
        languageSettings.optIn("kotlin.contracts.ExperimentalContracts")
        languageSettings.optIn("kotlinx.cinterop.UnsafeNumber")
        languageSettings.optIn("kotlinx.cinterop.ExperimentalForeignApi")
    }
}

afterEvaluate {
    configure<PublishingExtension> {
        repositories {
            mavenLocal()
        }
        
        publications.withType<MavenPublication> {
            pom {
                name.set("Kotlin LibWinit")
                description.set("Kotlin Multiplatform bindings for winit library")
                url.set("https://github.com/DrUlysses/kotlin-libwinit")
            }
        }
    }
}
