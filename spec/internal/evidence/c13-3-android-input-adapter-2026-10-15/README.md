# C13.3 Android 입력 어댑터 실행 근거

이 문서는 Android 입력 어댑터 구현 전 고정한 관찰·경계·판정 기준과 최종 AVD 실행 결과를 함께 기록한다. Android API 37 ARM64 AVD 전용 opt-in fixture 범위이며 실기기 또는 C13.3 전체 완료를 뜻하지 않는다. iOS 근거는 [플랫폼 공통 실행 기록](../c13-3-platform-input-adapters-2026-10-15/README.md)에 있다.

## 고정 기준

- Chrome oracle: `tests/fixtures/css/c13/c13-3-scroll-gesture.html`, Chrome `155.0.8059.40`, CSS viewport `400×240`, DPR 1.
- Chrome fixture SHA-256: `a4eff62477a8a5d01f58f8033472fd3dcd6e8bc7638723d3386f7060d5787c09`.
- Oracle 관찰 JSON: `tests/fixtures/css/references/c13-3-scroll-gesture-precomparison-v1.json`, SHA-256 `836f1c78c3a4b2df7d6a4ab9b56201d571398e01234977f416dcc5192493ec06`.
- Simulator 대상: `spinon_api37_1_compat16k`, API 37, `arm64-v8a`. ADB는 `emulator-5554`에서 `device`, SDK 37, ABI `arm64-v8a`를 반환했다. iPhone 17 Pro/iOS 26.2 Simulator는 별도 후속 범위다.
- 구현 기반: `origin/main`의 C13.3 queue/session 병합 SHA `46128dcc2dd314da461245d02cf7b0eab4e3277b`.

## Android slice 통과 기준

1. 앱의 실제 `MotionEvent` down/move/up/cancel을 통해 고정 400×240 CSS scene을 입력한다. native pixel과 CSS viewport point, `surfaceGeneration`, surface scale, `ViewConfiguration.getScaledTouchSlop()` 값과 pan 시작을 입력 trace에 남긴다.
2. UI callback은 제한된 queue에 입력만 추가하고 renderer/Rust worker의 처리가 끝날 때까지 기다리지 않는다. worker를 잠시 지연시키는 negative control에서 callback 반환이 worker 완료보다 먼저임을 trace의 동일 입력 sequence로 확인한다.
3. Begin에서 terminal slot을 예약하고 Move 포화·두 번째 pointer·surface lifecycle cancel을 정해진 순서로 소비한다. cancel 뒤 이미 게시된 offset을 유지하고 전체 contact가 끝난 뒤 fresh gesture를 받아들인다.
4. worker가 `RuntimeScrollGestureInputQueue`를 dequeue한 뒤 `RuntimeScrollGestureSession`에 연결하고, 변경된 immutable scroll snapshot을 presentation에 게시한다. 연속 cumulative Move가 입력 합계만큼 한 번 적용되고 `ScrollStateRevision`은 실제 offset 변경에서만 전진해야 한다.
5. Android AVD에서 scroll 전/후 화면과 실제 PixelCopy 또는 surface capture를 보존한다. marker 위치 변화와 owner offset·revision·입력 sequence를 같은 실행 기록에 연결한다.
6. below-slop tap control은 offset/revision을 바꾸지 않는다. 두 번째 contact는 active pan을 취소하고 이후 movement를 막는다. queue 처리 뒤 fresh touch는 같은 host에서 다시 스크롤한다. 표면 generation이 바뀐 이전 입력은 offset/presentation을 바꾸지 않는다.
7. C13.3 전용 Rust symbol, fixture 문자열과 payload는 default artifact에 없어야 하며 opt-in artifact에는 있어야 한다. feature가 꺼진 앱 route는 명시적 비활성 상태를 보여야 한다.

## 최종 AVD 실행 결과 · 2026-10-15

- 대상: `spinon_api37_1_compat16k`, Android 17/API 37, ARM64, serial `emulator-5554`, 1080×2400 physical px, density 2.625, ANGLE/Vulkan/SwiftShader. 연결된 물리 기기는 사용하지 않았다.
- Opt-in 빌드는 `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST=1 bun run build:android`로 성공했다. Android ARM64 merged `.so` SHA-256은 `2b875919092ea845866a06763df04e85ef60c854e960fd1f5c87af4cc54803cc`이며 `spinon_runtime_gpu_host_{new,eval,enqueue,process}_c13_3*` 4개 FFI symbol과 fixed-scene payload 문자열이 있다. [opt-in build log](./android-optin-build.log) · [artifact 검사](./android-optin-artifact-check.txt).
- 같은 source의 `SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST=0` 기본 빌드도 성공했다. merged `.so` SHA-256은 `65d2bda2e6d54b41c32324d1f62aac6256b996fe32fb8207df88b211fbcecb3f`이며 위 C13.3 FFI symbol 4개와 fixture payload가 없다. 기본 APK를 AVD에 설치해 route를 열면 명시적 비활성 화면과 `SPINON_C133_DISABLED`가 나타났다. [default build log](./android-default-build.log) · [artifact 검사](./android-default-artifact-check.txt) · [화면](./android-api37-avd-feature-off-final.png) · [로그](./android-api37-avd-feature-off-final.log).
- Opt-in AVD route는 `status=0`, CSS viewport `400×240`, `root:2, outer:4, inner:6`, queue capacity 64와 terminal capacity 8로 준비됐다. 실제 `adb shell input swipe`로 한 번 위로 스와이프한 뒤 inner offset은 `80.5 CSS px`, revision은 44였고 red marker가 안쪽 scrollport에서 이동했다. [single swipe screenshot](./android-api37-avd-after-swipe-final.png) · [입력/worker log](./android-api37-avd-final-swipe.log).
- 같은 inner 영역에서 실제 swipe 5회를 이어 보낸 실행의 마지막 offset은 `inner=106`, `outer=186`, `root=91.314 CSS px`, revision 173이었다. 이는 고정 `auto/auto` fixture의 안쪽→바깥쪽→root delta 전달만 증명한다. [chained screenshot](./android-api37-avd-chained-scroll-final.png) · [offset trace](./android-api37-avd-chained-scroll-final.log).
- negative control은 실제 AVD 화면에서 5 physical-pixel swipe였다. native `ViewConfiguration.scaledTouchSlop=21px`보다 작았고 모든 move가 `accepted=false`, terminal 뒤 offset/revision은 0이었다. [below-slop log](./android-api37-avd-below-slop-final.log) · [화면](./android-api37-avd-below-slop-final.png).
- Android 두 번째 pointer 행은 별도 synthetic `MotionEvent` probe로만 확인했다. `ACTION_POINTER_DOWN`에서 cancel을 예약하고 뒤의 move를 억제했다. 이는 ADB multi-touch 입력 증거가 아니다. activity-background cancel 이후 같은 host의 fresh swipe, worker 1000ms 지연 중 UI callback 비대기 결과는 앞선 실행 로그에 있다. [second-pointer](./android-api37-avd-second-pointer.log) · [lifecycle cancel](./android-api37-avd-lifecycle-cancel.log) · [worker delay](./android-api37-avd-worker-delay.log).

## 루트 최대 offset 화면 경계 대조 · 2026-10-15

- [연속 chaining 화면](./android-api37-avd-chained-scroll-final.png)과 [root max 화면](./android-api37-avd-root-max.png)을 RGB 픽셀 단위로 비교했다. 두 캡처 모두 1080×2400이며 yellow root border 픽셀 경계는 `[58,1022) × [1014,1560)`이다.
- 총 변경 119,449픽셀 중 118,862픽셀은 root border 안에 있고 border 픽셀 자체는 0개 변경됐다. 나머지 587픽셀은 시스템 status bar 시간 영역이다. 앱 제목과 하단 상태 문구 영역은 변경 0픽셀이다. root 배경과 scroll content 변화는 scrollport 안에 머물렀다.
- root max 상태는 log의 `root:0,238;outer:0,186;inner:0,106`, `ScrollStateRevision=252`로 연결된다. [root max 입력·offset log](./android-api37-avd-root-max.log) · [양 플랫폼 픽셀 경계 대조](../c13-3-platform-input-adapters-2026-10-15/screen-bounds-check.txt).

## 검증 한계

- 이 문서는 Android AVD만 다룬다. iOS Simulator 증거는 공통 README에 분리했으며, 실제 Android 기기, 하드웨어 GPU와 compositor scanout은 판정하지 않는다.
- C13.3 computed-style projection의 renderer policy 연결, iOS `UIPanGestureRecognizer`, wheel/trackpad, 공개 DOM event 및 S04.10 displayed-frame hit testing은 이 Android slice의 통과 범위가 아니다.
- UI 화면·PixelCopy 검증은 실제 AVD에서 성공한 결과만 추가한다. scripted JNI input은 native `MotionEvent` proof로 보고하지 않는다.
