import java.util.Properties

plugins {
    alias(libs.plugins.android.application)
    alias(libs.plugins.kotlin.compose)
    alias(libs.plugins.kotlin.serialization)
}

val keystorePropertiesFile = rootProject.file("keystore.properties")
val keystoreProperties = Properties().apply {
    if (keystorePropertiesFile.exists()) {
        load(keystorePropertiesFile.inputStream())
    }
}

android {
    namespace = "com.noctiro.wherebus"
    compileSdk {
        version = release(36) {
            minorApiLevel = 1
        }
    }

    defaultConfig {
        applicationId = "com.noctiro.wherebus"
        minSdk = 24
        targetSdk = 36
        versionCode = 1
        versionName = "1.0"

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        ndk {
            abiFilters += listOf(
                "arm64-v8a",
                "armeabi-v7a",
                "x86",
                "x86_64",
            )
        }
    }

    signingConfigs {
        if (keystorePropertiesFile.exists()) {
            create("release") {
                storeFile = file(keystoreProperties.getProperty("storeFile"))
                storePassword = keystoreProperties.getProperty("storePassword")
                keyAlias = keystoreProperties.getProperty("keyAlias")
                keyPassword = keystoreProperties.getProperty("keyPassword")
            }
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
            if (keystorePropertiesFile.exists()) {
                signingConfig = signingConfigs.getByName("release")
            }
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    buildFeatures {
        compose = true
        buildConfig = true
    }
    sourceSets {
        getByName("main").jniLibs.directories.add(layout.buildDirectory.dir("rustJniLibs").get().asFile.absolutePath)
    }
}

kotlin {
    jvmToolchain(17)
}

val rustTargetByAbi = mapOf(
    "arm64-v8a" to "aarch64-linux-android",
    "armeabi-v7a" to "armv7-linux-androideabi",
    "x86" to "i686-linux-android",
    "x86_64" to "x86_64-linux-android",
)

val ndkBinDir = providers.environmentVariable("ANDROID_NDK_HOME")
    .orElse(providers.environmentVariable("ANDROID_NDK_ROOT"))
    .map { File(it).resolve("toolchains/llvm/prebuilt/linux-x86_64/bin") }

val rustLibName = "libwherebus.so"
val rustOutputDir = layout.buildDirectory.dir("rustJniLibs")

val cargoBuildTasks = rustTargetByAbi.map { (abi, target) ->
    val taskName = "cargoBuild" + abi
        .split('-', '_')
        .joinToString("") { part ->
            part.replaceFirstChar { char -> char.uppercase() }
        }

    tasks.register<Exec>(taskName) {
        val linker = ndkBinDir.map { binDir ->
            when (target) {
                "aarch64-linux-android" -> File(binDir, "aarch64-linux-android24-clang")
                "armv7-linux-androideabi" -> File(binDir, "armv7a-linux-androideabi24-clang")
                "i686-linux-android" -> File(binDir, "i686-linux-android24-clang")
                else -> File(binDir, "x86_64-linux-android24-clang")
            }.absolutePath
        }

        inputs.dir(rootProject.file("../core/src"))
        inputs.file(rootProject.file("../core/Cargo.toml"))
        inputs.file(rootProject.file("../Cargo.lock"))
        outputs.file(rootProject.file("../target/$target/debug/$rustLibName"))

        workingDir = rootProject.file("..")
        val targetEnv = target.uppercase().replace('-', '_')
        val targetUnderscore = target.replace('-', '_')
        environment("CARGO_TARGET_${targetEnv}_LINKER", linker.get())
        environment("CC_$targetUnderscore", linker.get())
        environment("TARGET_CC", linker.get())
        environment("CC", linker.get())
        commandLine("cargo", "build", "-p", "wherebus", "--target", target)
    }
}

val buildRustAndroid by tasks.registering {
    group = "build"
    description = "Build Rust shared libraries for Android ABIs"

    dependsOn(cargoBuildTasks)

    doLast {
        rustTargetByAbi.forEach { (abi, target) ->
            val from = rootProject.file("../target/$target/debug/$rustLibName")
            val into = rustOutputDir.get().file("$abi/$rustLibName").asFile
            into.parentFile.mkdirs()
            from.copyTo(into, overwrite = true)
        }
    }
}

tasks.matching { task ->
    task.name == "mergeDebugNativeLibs" ||
        task.name == "mergeReleaseNativeLibs" ||
        task.name == "packageDebug" ||
        task.name == "packageRelease"
}.configureEach {
    dependsOn(buildRustAndroid)
}

dependencies {
    implementation(platform(libs.androidx.compose.bom))
    implementation(libs.androidx.activity.compose)
    implementation(libs.androidx.compose.material.icons.extended)
    implementation(libs.androidx.compose.material3)
    implementation(libs.androidx.compose.ui)
    implementation(libs.androidx.compose.ui.graphics)
    implementation(libs.androidx.compose.ui.tooling.preview)
    implementation(libs.androidx.core.ktx)
    implementation(libs.androidx.lifecycle.runtime.ktx)
    implementation(libs.androidx.lifecycle.runtime.compose)
    implementation(libs.androidx.lifecycle.viewmodel.compose)
    implementation(libs.androidx.lifecycle.viewmodel.ktx)
    implementation(libs.androidx.navigation.compose)
    implementation(libs.kotlinx.serialization.json)
    testImplementation(libs.junit)
    androidTestImplementation(platform(libs.androidx.compose.bom))
    androidTestImplementation(libs.androidx.compose.ui.test.junit4)
    androidTestImplementation(libs.androidx.espresso.core)
    androidTestImplementation(libs.androidx.junit)
    debugImplementation(libs.androidx.compose.ui.test.manifest)
    debugImplementation(libs.androidx.compose.ui.tooling)
}
