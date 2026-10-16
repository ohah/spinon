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
val spinonC04UaCascadeFixtureProbe =
    providers.gradleProperty("spinonC04UaCascadeFixtureProbe").orElse("0")
val spinonC04UaCascadeFixtureProbeValue = spinonC04UaCascadeFixtureProbe.get()
require(spinonC04UaCascadeFixtureProbeValue == "0" ||
    spinonC04UaCascadeFixtureProbeValue == "1") {
    "spinonC04UaCascadeFixtureProbe는 0 또는 1이어야 합니다."
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
val spinonC124WorkerStackResidencyTest =
    providers.environmentVariable("SPINON_ENABLE_C124_WORKER_STACK_RESIDENCY_TEST").orElse("0")
val spinonC124WorkerStackResidencyTestValue = spinonC124WorkerStackResidencyTest.get()
require(spinonC124WorkerStackResidencyTestValue == "0" ||
    spinonC124WorkerStackResidencyTestValue == "1") {
    "SPINON_ENABLE_C124_WORKER_STACK_RESIDENCY_TEST는 0 또는 1이어야 합니다."
}
require(spinonC124WorkerStackResidencyTestValue != "1" || spinonC04RuntimeGpuValue == "1") {
    "C12.4 worker stack 검증은 C04.10 runtime GPU fixture와 함께 켜야 합니다."
}
val canonicalC131OverflowClipFixtureTest =
    providers.environmentVariable("SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST").orNull
val legacyC131OverflowClipFixtureTest =
    providers.environmentVariable("SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST").orNull
require(canonicalC131OverflowClipFixtureTest == null || legacyC131OverflowClipFixtureTest == null ||
    canonicalC131OverflowClipFixtureTest == legacyC131OverflowClipFixtureTest) {
    "C13.1 fixture flag 이름에 서로 다른 값이 설정됐습니다."
}
val spinonC131OverflowClipFixtureTestValue =
    canonicalC131OverflowClipFixtureTest ?: legacyC131OverflowClipFixtureTest ?: "0"
require(spinonC131OverflowClipFixtureTestValue == "0" ||
    spinonC131OverflowClipFixtureTestValue == "1") {
    "SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST는 0 또는 1이어야 합니다."
}
require(spinonC131OverflowClipFixtureTestValue != "1" || spinonC04RuntimeGpuValue == "1") {
    "C13.1 overflow clip fixture는 C04.10 runtime GPU fixture와 함께 켜야 합니다."
}
val spinonC132ScrollFixtureTest =
    providers.environmentVariable("SPINON_ENABLE_C13_2_SCROLL_FIXTURE_TEST").orElse("0")
val spinonC132ScrollFixtureTestValue = spinonC132ScrollFixtureTest.get()
require(spinonC132ScrollFixtureTestValue == "0" || spinonC132ScrollFixtureTestValue == "1") {
    "SPINON_ENABLE_C13_2_SCROLL_FIXTURE_TEST는 0 또는 1이어야 합니다."
}
require(spinonC132ScrollFixtureTestValue != "1" || spinonC04RuntimeGpuValue == "1") {
    "C13.2 scroll fixture는 C04.10 runtime GPU fixture와 함께 켜야 합니다."
}
val spinonC133GestureFixtureTest =
    providers.environmentVariable("SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST").orElse("0")
val spinonC133GestureFixtureTestValue = spinonC133GestureFixtureTest.get()
require(spinonC133GestureFixtureTestValue == "0" || spinonC133GestureFixtureTestValue == "1") {
    "SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST는 0 또는 1이어야 합니다."
}
require(spinonC133GestureFixtureTestValue != "1" || spinonC04RuntimeGpuValue == "1") {
    "C13.3 gesture fixture는 C04.10 runtime GPU fixture와 함께 켜야 합니다."
}
val spinonC141AppFixtureTest =
    providers.environmentVariable("SPINON_ENABLE_C14_1_APP_FIXTURE_TEST").orElse("0")
val spinonC141AppFixtureTestValue = spinonC141AppFixtureTest.get()
require(spinonC141AppFixtureTestValue == "0" || spinonC141AppFixtureTestValue == "1") {
    "SPINON_ENABLE_C14_1_APP_FIXTURE_TEST는 0 또는 1이어야 합니다."
}
val spinonC142AppFixtureTest =
    providers.environmentVariable("SPINON_ENABLE_C14_2_APP_FIXTURE_TEST").orElse("0")
val spinonC142AppFixtureTestValue = spinonC142AppFixtureTest.get()
require(spinonC142AppFixtureTestValue == "0" || spinonC142AppFixtureTestValue == "1") {
    "SPINON_ENABLE_C14_2_APP_FIXTURE_TEST는 0 또는 1이어야 합니다."
}
val spinonC143AppFixtureTest =
    providers.environmentVariable("SPINON_ENABLE_C14_3_APP_FIXTURE_TEST").orElse("0")
val spinonC143AppFixtureTestValue = spinonC143AppFixtureTest.get()
require(spinonC143AppFixtureTestValue == "0" || spinonC143AppFixtureTestValue == "1") {
    "SPINON_ENABLE_C14_3_APP_FIXTURE_TEST는 0 또는 1이어야 합니다."
}
val spinonC181ImagePaintFixtureTest =
    providers.environmentVariable("SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST").orElse("0")
val spinonC181ImagePaintFixtureTestValue = spinonC181ImagePaintFixtureTest.get()
require(spinonC181ImagePaintFixtureTestValue == "0" || spinonC181ImagePaintFixtureTestValue == "1") {
    "SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST는 0 또는 1이어야 합니다."
}
require(spinonC181ImagePaintFixtureTestValue != "1" || spinonC04RuntimeGpuValue == "1") {
    "C18.1 image paint fixture는 C04.10 runtime GPU fixture와 함께 켜야 합니다."
}
val prepareSpinonBootstrap by tasks.registering(Exec::class) {
    workingDir = repoRoot
    environment("SPINON_ENABLE_S03_DOM_GC_FIXTURE", spinonS03DomGcFixture.get())
    environment("SPINON_ENABLE_C04_RUNTIME_GPU", spinonC04RuntimeGpu.get())
    environment(
        "SPINON_ENABLE_C04_UA_CASCADE_FIXTURE_PROBE",
        spinonC04UaCascadeFixtureProbe.get(),
    )
    environment(
        "SPINON_ENABLE_C04_RUNTIME_GPU_FAILURE_FIXTURE",
        spinonC04RuntimeGpuFailureFixture.get(),
    )
    environment(
        "SPINON_ENABLE_C124_WORKER_STACK_RESIDENCY_TEST",
        spinonC124WorkerStackResidencyTestValue,
    )
    environment(
        "SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST",
        spinonC131OverflowClipFixtureTestValue,
    )
    environment("SPINON_ENABLE_C13_2_SCROLL_FIXTURE_TEST", spinonC132ScrollFixtureTestValue)
    environment("SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST", spinonC133GestureFixtureTestValue)
    environment("SPINON_ENABLE_C14_1_APP_FIXTURE_TEST", spinonC141AppFixtureTestValue)
    environment("SPINON_ENABLE_C14_2_APP_FIXTURE_TEST", spinonC142AppFixtureTestValue)
    environment("SPINON_ENABLE_C14_3_APP_FIXTURE_TEST", spinonC143AppFixtureTestValue)
    environment("SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST", spinonC181ImagePaintFixtureTestValue)
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
        buildConfigField(
            "boolean",
            "SPINON_C124_WORKER_STACK_RESIDENCY_TEST",
            (spinonC124WorkerStackResidencyTestValue == "1").toString(),
        )
        buildConfigField(
            "boolean",
            "SPINON_C131_OVERFLOW_CLIP_FIXTURE_TEST",
            (spinonC131OverflowClipFixtureTestValue == "1").toString(),
        )
        buildConfigField(
            "boolean",
            "SPINON_C132_SCROLL_FIXTURE_TEST",
            (spinonC132ScrollFixtureTestValue == "1").toString(),
        )
        buildConfigField(
            "boolean",
            "SPINON_C133_GESTURE_FIXTURE_TEST",
            (spinonC133GestureFixtureTestValue == "1").toString(),
        )
        buildConfigField(
            "boolean",
            "SPINON_C141_APP_FIXTURE_TEST",
            (spinonC141AppFixtureTestValue == "1").toString(),
        )
        buildConfigField(
            "boolean",
            "SPINON_C142_APP_FIXTURE_TEST",
            (spinonC142AppFixtureTestValue == "1").toString(),
        )
        buildConfigField(
            "boolean",
            "SPINON_C143_APP_FIXTURE_TEST",
            (spinonC143AppFixtureTestValue == "1").toString(),
        )
        buildConfigField(
            "boolean",
            "SPINON_IMAGE_PAINT_FIXTURE_TEST",
            (spinonC181ImagePaintFixtureTestValue == "1").toString(),
        )
    }

    buildFeatures {
        buildConfig = true
    }

    sourceSets["main"].apply {
        assets.srcDir(repoRoot.resolve("build/spinon/bootstrap"))
        jniLibs.srcDir(repoRoot.resolve("build/spinon/android/jniLibs"))
        if (spinonC133GestureFixtureTestValue == "1") {
            java.srcDir("src/c13_3_gesture_fixture_test/java")
        }
        if (spinonC181ImagePaintFixtureTestValue == "1") {
            java.srcDir("src/c18_1_image_paint_fixture_test/java")
        }
    }
}
