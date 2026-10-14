# C13.1 overflow clip · Chrome/Android AVD runtime 비교 · 2026-10-14

이 자료는 fixture 전용 C13.1 제한 profile의 layout·paint 확인과 고정 point hit-test query 비교다. C13.1 전체 완료, DOM hit-test event 전달, WPT 적합성 또는 제품 CSS 지원을 선언하지 않는다.

## 입력과 환경

- fixture: [runtime-overflow-clip.html](../../../../tests/fixtures/css/c13/runtime-overflow-clip.html) 및 공유 JS [runtime-overflow-clip.js](../../../../tests/fixtures/css/c13/runtime-overflow-clip.js). browser와 V8 runtime이 같은 source를 쓴다.
- Chrome 기준: agent-browser `0.31.1`, Chrome for Testing `155.0.8059.40`, revision `cfaadc5a132d78e1828635aa8405a499f3e14864-refs/branch-heads/8059@{#936}`, HeadlessChrome UA, viewport `320×120` CSS px, DPR `1`. [Chrome observation JSON](chrome155-runtime-observation.json)과 [Chrome PNG](chrome155-runtime-overflow-clip.png)에 frame·sample·fixture hash를 보존했다.
- Android 대상: `spinon_api37_1_compat16k`, serial `emulator-5554`, Android API `37`, `arm64-v8a`, `sys.boot_completed=1`, `ro.kernel.qemu=1`. 연결 목록은 [adb-devices.txt](adb-devices.txt), 속성은 [avd-properties.txt](avd-properties.txt)에 있다. 실기기는 연결하거나 선택하지 않았다.
- renderer: Android Emulator OpenGL ES Translator → ANGLE/Vulkan → SwiftShader software renderer. SurfaceView `790×263` physical px, density `2.625`, runtime CSS viewport `300.9524×100.190475`, fixture root `300×100` CSS px.

## ADB 실패 원인과 사용한 복구

처음 `adb devices`가 비어 있던 것은 ADB daemon 연결 문제가 아니었다. `spinon_api37_2.ini`가 존재하지 않는 `/Users/yoonhb/.android/avd/spinon_api37_2.avd`를 가리키고, 해당 AVD의 `arm` architecture는 실행 중인 QEMU2에서 지원하지 않았다. 유효한 기존 API 37 ARM64 AVD `spinon_api37_1_compat16k`를 부팅해 `emulator-5554`로 붙였고 `sys.boot_completed=1`, APK 설치·앱 실행, runtime fixture와 PixelCopy까지 확인했다. 관련 초기 진단 원본은 [C14.1 AVD bootstrap 근거](../c14-1-android-avd-bootstrap-2026-10-14/README.md)다.

## 빌드와 실행

```sh
SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 \
SPINON_ENABLE_C04_RUNTIME_GPU=1 \
SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST=1 \
  mise exec -- bash tools/build-android-app.sh

adb -s emulator-5554 install -r platforms/android/app/build/outputs/apk/debug/app-debug.apk
adb -s emulator-5554 shell am start -S -n dev.spinon.bootstrap/.MainActivity \
  --ez spinon_c131_overflow_clip true
```

빌드 원본은 [android-build.log](android-build.log), 해당 APK와 PixelCopy PNG의 SHA-256은 [artifact-sha256.txt](artifact-sha256.txt)에 있다. Gradle은 compile SDK 37.2와 AGP 8.13.2 호환 권고를 출력했지만 `BUILD SUCCESSFUL`로 완료했다.

## 결과

- V8·Stylo·Taffy·WGPU fixture가 `status=0`, `layout=ready`, `boxes=13`으로 평가됐다. run `a101139308e`의 [Android 원본 log](android-api37-arm64.log)에서 C13 node frame 13개는 Chrome의 [고정 geometry](chrome155-runtime-observation.json)와 모두 일치했다. CSS px 최대 오차는 `0`이다.
- Android `SurfaceViewPixelCapture`가 PNG를 저장한 뒤 다시 읽고 11개 고정 CSS 좌표를 표본화했다. clip 안/밖, visible overflow, padding-box `overflow-clip-margin`, 중첩 x/y 축, positioned child escape 대조가 Chrome screenshot의 같은 좌표 RGBA와 11/11 exact였다. 표본별 observed/expected RGBA와 `status=matched`는 log에 보존했다.
- agent-browser로 같은 Chrome 155 fixture에서 `document.elementFromPoint()`를 11개 사전 고정 좌표에 호출했다. 당시 runtime의 scene.hit_test_css_point() 회귀 테스트는 visible overflow, clip 안/밖, clip margin 확장, nested x/y 축, positioned escape와 root/ancestor fallback의 원래 11개 반환 node를 일치시켰다. 이후 추가한 viewport 밖 null 네 점은 [후속 negative-control 실행](../c13-1-overflow-clip-hit-test-negative-controls-android-avd-2026-10-14/README.md)에 기록했다. 브라우저 결과는 [Chrome observation JSON](chrome155-runtime-observation.json)의 `hitTestSamples`에, runtime 비교는 `runtime_hit_test_matches_agent_browser_chrome_155_element_from_point_samples`에 있다.
- 후속 Android 앱 경로도 API 37 ARM64 AVD에서 같은 11 query를 게시 대상 runtime snapshot에 실행해 11/11 일치했다. JNI log row, PixelCopy와 build 근거는 [앱 runtime hit-test 실행](../c13-1-overflow-clip-hit-test-android-avd-2026-10-14/README.md)에 별도로 보존했다.
- [Android PixelCopy PNG](android-api37-arm64-pixelcopy.png)는 앱의 SurfaceView producer buffer다. scanout/compositor 뒤 실제 화면, GPU hardware 또는 실기기 결과로 확대하지 않는다.
- [feature isolation](feature-isolation.txt)은 rustc와 같은 LLVM 22 `llvm-nm`으로 default `spinon-ffi` static library에서 C13.1 FFI 심볼 2개가 없는지, feature artifact에는 있는지 확인했고, `strings`로 `c131-root` fixture payload의 부재/존재도 각각 확인했다. 이는 해당 정적 라이브러리만 검사한 것이며 최종 앱 artifact 크기나 제품 빌드 적합성 측정이 아니다.
- 변경된 8개 Rust package 전체 test target을 `c13-1-overflow-clip-fixture-test` feature on으로 실행해 통과했다([로그](changed-packages-cargo-test.log)). clip ancestry 예약 실패를 한 번 주입한 뒤 같은 reserve로 snapshot 빌드를 재시도하는 회귀 테스트도 포함한다. `cargo test --locked --workspace`, `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`, `git diff --check`, fixture module boundary test 5/5도 현재 변경에서 통과했다([workspace test](workspace-cargo-test.log), [Clippy](workspace-clippy.log)). 같은 앱 runtime worker의 오류 뒤 재요청 복구는 별도 관문으로 남아 있다.

## 남은 판정

전체 C13.1 oracle matrix, clip 바깥 draw 및 hit/event negative-control breadth, clip ancestry allocation failure 후 동일 runtime worker 복구, `clip`과 `hidden`/`flow-root` float 비교, unsupported radius/transform 재검증, iOS Simulator 통합, WPT 후보 분류는 남았다. browser query는 고정 좌표의 runtime box hit 결과이며 DOM event dispatch를 호출하거나 입증하지 않는다. snapshot builder 경계의 단일 예약 실패와 같은 reserve 재사용은 단위 테스트로 확인했다. overflow scrolling, scroll state/CSSOM, gesture, scrollbar, radius/transform, 공개 CSS 지원은 범위 밖이다.
