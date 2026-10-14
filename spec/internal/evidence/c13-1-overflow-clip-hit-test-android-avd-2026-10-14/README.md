# C13.1 Android AVD runtime hit-test · 2026-10-14

이 자료는 C13.1 opt-in fixture가 Android 앱 runtime에서 실제 게시할 snapshot에 hit query를 적용한 기록이다. 고정 Chrome `elementFromPoint()` 표본과 box query 결과의 일치를 확인한다. DOM listener/event dispatch, 일반 앱 입력 연결, 전체 hit-test matrix 또는 C13.1 완료를 선언하지 않는다.

## 환경과 실행

- 대상은 Android API 37 ARM64 AVD `spinon_api37_1_compat16k`, serial `emulator-5554`다. `ro.kernel.qemu=1`, `sys.boot_completed=1`이며 실기기를 연결하거나 선택하지 않았다. 속성은 [AVD properties](avd-properties.txt)에 있다.
- 최종 APK는 `SPINON_ENABLE_C04_RUNTIME_GPU=1` 및 `SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST=1`을 켜 `tools/build-android-app.sh`로 만들었다. 빌드 원본은 [android-build-final.log](android-build-final.log)이며 `BUILD SUCCESSFUL`이다. Gradle은 compile SDK 37.2와 AGP 8.13.2의 호환 권고를 출력했다.
- `adb -s emulator-5554 install -r platforms/android/app/build/outputs/apk/debug/app-debug.apk`로 설치하고 `am start -S -n dev.spinon.bootstrap/.MainActivity --ez spinon_c131_overflow_clip true`로 실행했다. run ID는 `b96d0f0b00f`, 앱 PID는 `6671`이다.
- Surface는 `790×263` physical px, density `2.625`, runtime viewport `300.9524×100.190475` CSS px다. renderer는 ANGLE/Vulkan/SwiftShader software renderer다.

## 결과

- 앱 초기 환경 갱신은 JS fixture 평가 전 빈 scene `layout=empty`, `boxes=0`으로 정상 통과한다. C13.1 hit query는 이 경로에서 건너뛰며 fixture eval 결과가 성공한 뒤에만 실행한다. 최초 통합 구현에서 환경 갱신에도 query를 실행해 빈 scene의 `visible-target` query가 실패하는 것을 재현했고, query를 fixture eval 경계로 옮겨 수정했다.
- 최종 `SPINON_C0410_EVAL` report는 C13.1 eval `status=0`, `layout=ready`, `boxes=13`이다. 앱이 게시할 runtime snapshot에 11개 Chrome oracle 좌표를 적용했고 `SPINON_C131_HIT_TEST` 로그가 모두 `status=matched`로 남았다. [비교 JSON](runtime-hit-test-comparison.json)은 Chrome observation의 표본 id·좌표·target frame index와 Android runtime 결과를 재검산해 hit query 11/11 일치를 기록한다. 전체 log는 [android-api37-arm64-hit-test-final.log](android-api37-arm64-hit-test-final.log)다.
- 같은 실행의 SurfaceView PixelCopy는 11개 RGBA 표본 모두 고정 Chrome screenshot과 exact 일치했다. log의 `SPINON_C131_PIXELCOPY_SUMMARY`는 `expected_samples=11`, `observed_samples=11`, `matched_samples=11`이다. 저장한 [PixelCopy PNG](android-api37-arm64-pixelcopy.png)는 790×263이며 SHA-256은 [artifact hashes](artifact-sha256.txt)에 있다.
- 긴 FFI report의 Android log truncation 때문에 JNI가 각 JSON hit sample을 별도 짧은 log row로 기록하도록 했다. [feature isolation](feature-isolation.txt)은 host static archive의 C13 심볼과 Chrome oracle payload가 feature-on artifact에 있고 default artifact에는 없는 것을 확인한다. 기본 app 경로에서는 C13.1 Cargo feature와 C define이 모두 꺼져 있어 이 logger/helper가 포함되지 않는다.

## 검증 범위

Android 앱의 C13.1 fixture eval이 runtime `RuntimeRenderSnapshot::hit_test_css_point()` query 결과를 Chrome 155 `document.elementFromPoint()`와 대조하고 다르면 fixture eval을 실패시킨다. 고정 11점만 확인했다. DOM 이벤트 전달, 사용자 터치, WPT, 다른 viewport·DPR, 하드웨어 GPU, scanout 후 compositor 표면, 실기기는 검증하지 않았다. iOS Simulator 통합과 전체 C13.1 oracle matrix도 남아 있다.

Rust workspace tests, feature-on `spinon-ffi` tests, workspace Clippy, C13 feature-on FFI Clippy, rustfmt, `git diff --check` 및 fixture module boundary test 5/5를 실행했다. 실행 artifact SHA-256은 [artifact hashes](artifact-sha256.txt)에 있다.
