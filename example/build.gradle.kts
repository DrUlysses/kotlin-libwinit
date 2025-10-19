import gobley.gradle.rust.dsl.*

plugins {
    alias(libs.plugins.kotlin.multiplatform)
}

kotlin {
    hostNativeTarget()

    linuxX64 {
        binaries.executable {
            entryPoint = "main"
        }
    }

    sourceSets {
        commonMain.dependencies {
            implementation(project(":kotlin-libwinit"))
        }
    }

    sourceSets.all {
        languageSettings.optIn("kotlin.contracts.ExperimentalContracts")
        languageSettings.optIn("kotlinx.cinterop.UnsafeNumber")
        languageSettings.optIn("kotlinx.cinterop.ExperimentalForeignApi")
    }
}
