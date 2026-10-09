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
val spinonC04RuntimeGpu = providers.gradleProperty("spinonC04RuntimeGpu").orElse("0")
val spinonC04RuntimeGpuValue = spinonC04RuntimeGpu.get()
require(spinonC04RuntimeGpuValue == "0" || spinonC04RuntimeGpuValue == "1") {
    "spinonC04RuntimeGpu는 0 또는 1이어야 합니다."
}
val spinonC04RuntimeGpuFailureFixture =
    providers.gradleProperty("spinonC04RuntimeGpuFailureFixture").orElse("0")
val spinonC04RuntimeGpuFailureFixtureValue = spinonC04RuntimeGpuFailureFixture.get()
require(spinonC04RuntimeGpuFailureFixtureValue == "0" ||
    spinonC04RuntimeGpuFailureFixtureValue == "1") {
    "spinonC04RuntimeGpuFailureFixture는 0 또는 1이어야 합니다."
}
require(spinonC04RuntimeGpuFailureFixtureValue != "1" || spinonC04RuntimeGpuValue == "1") {
    "draw 실패 fixture는 C04.10 runtime GPU fixture와 함께 켜야 합니다."
}
val prepareSpinonBootstrap by tasks.registering(Exec::class) {
    workingDir = repoRoot
    environment("SPINON_ENABLE_S03_DOM_GC_FIXTURE", spinonS03DomGcFixture.get())
    environment("SPINON_ENABLE_C04_RUNTIME_GPU", spinonC04RuntimeGpu.get())
    environment(
        "SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE",
        spinonC04RuntimeGpuFailureFixture.get(),
    )
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
        buildConfigField(
            "boolean",
            "SPINON_C04_RUNTIME_GPU",
            (spinonC04RuntimeGpuValue == "1").toString(),
        )
        buildConfigField(
            "boolean",
            "SPINON_C04_RUNTIME_GPU_FAILURE_FIXTURE",
            (spinonC04RuntimeGpuFailureFixtureValue == "1").toString(),
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
