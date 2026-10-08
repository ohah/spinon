plugins {
    id("com.android.application")
}

val repoRoot = rootProject.projectDir.resolve("../..")
val spinonNdkVersion = repoRoot.resolve("tools/android-ndk-version.txt").readText().trim()
val spinonS03DomGcFixture = providers.gradleProperty("spinonS03DomGcFixture").orElse("0")
val spinonS03DomGcFixtureValue = spinonS03DomGcFixture.get()
require(spinonS03DomGcFixtureValue == "0" || spinonS03DomGcFixtureValue == "1") {
    "spinonS03DomGcFixture는 0 또는 1이어야 합니다."
}
val prepareSpinonBootstrap by tasks.registering(Exec::class) {
    workingDir = repoRoot
    environment("SPINON_ENABLE_S03_DOM_GC_FIXTURE", spinonS03DomGcFixture.get())
    commandLine("bash", "tools/build-android.sh")
}

tasks.named("preBuild").configure {
    dependsOn(prepareSpinonBootstrap)
}

tasks.configureEach {
    if (name.startsWith("merge") &&
        (name.endsWith("Assets") || name.endsWith("JniLibFolders") || name.endsWith("NativeLibs"))
    ) {
        dependsOn(prepareSpinonBootstrap)
    }
}

android {
    namespace = "dev.spinon.bootstrap"
    compileSdk {
        version = release(37) {
            minorApiLevel = 2
        }
    }
    buildToolsVersion = "35.0.0"
    ndkVersion = spinonNdkVersion

    defaultConfig {
        applicationId = "dev.spinon.bootstrap"
        minSdk = 29
        targetSdk = 36
        versionCode = 1
        versionName = "0.1.0-bootstrap"
        buildConfigField(
            "boolean",
            "SPINON_S03_DOM_GC_FIXTURE",
            (spinonS03DomGcFixtureValue == "1").toString(),
        )
    }

    buildFeatures {
        buildConfig = true
    }

    sourceSets["main"].apply {
        assets.srcDir(repoRoot.resolve("build/spinon/bootstrap"))
        jniLibs.srcDir(repoRoot.resolve("build/spinon/android/jniLibs"))
    }
}
