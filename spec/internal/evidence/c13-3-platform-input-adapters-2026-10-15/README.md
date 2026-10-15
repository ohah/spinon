# C13.3 Android/iOS platform input adapter 실행 근거 · 2026-10-15

이 변경은 고정 400×240 CSS px 장면에만 연결하는 opt-in `c13-3-scroll-gesture-fixture-test` adapter다. Android `MotionEvent`와 iOS `UIPanGestureRecognizer`의 단일-pointer 입력을 Rust bounded queue/session에 전달하고 immutable scroll snapshot을 그린다. 제품 CSS 지원 또는 C13.3 전체 완료를 뜻하지 않는다.

## Android API 37 ARM64 AVD

- 대상: `spinon_api37_1_compat16k`, API 37, ARM64, `emulator-5554`, 1080×2400 px, density 2.625, ANGLE/Vulkan/SwiftShader. 실기기를 연결하지 않았다.
- 최신 ADB swipe 뒤 inner offset `80.5 CSS px`, revision 44, marker가 화면에서 이동했다. 다섯 번 연속 swipe 뒤 inner `106`, outer `186`, root `91.314 CSS px`, revision 173까지 이어졌다. 5 physical-pixel negative-control swipe는 Android native slop 21px보다 작아 pan을 열지 않고 offset/revision 0을 유지했다.
- queue worker를 1000ms 늦추는 별도 제어에서 UI callback은 queue enqueue 뒤 반환했고 worker 완료를 기다리지 않았다. activity pause 시 queued cancel 뒤 같은 host에서 새 gesture도 받았다.
- 두 번째 pointer 검사는 앱이 직접 만든 synthetic `MotionEvent`다. 실제 ADB multi-touch 결과로 취급하지 않는다.
- [상세 판정·로그·화면·기본/opt-in artifact 검사](../c13-3-android-input-adapter-2026-10-15/README.md)

## iPhone 17 Pro iOS 26.2 Simulator

- 대상: iPhone 17 Pro Simulator `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`, iOS 26.2. 입력은 `idb`가 보낸 Simulator touch다. 물리 iPhone은 사용하지 않았다.
- 기본 iOS Simulator build 성공, Mach-O SHA-256 `e68eff4bb60dc39df25a76586fa05369db760180ca602b1f3d6235a629c7b7a7`. 기본 artifact의 C13.3 FFI symbol은 0개이고 adapter callback/scene payload 없이 disabled route 문자열만 포함된다. 분리 checkout에는 V8 소스가 없어 고정 revision `7b50b62cb18f28617959e8452e2cd18195b38bcf`의 clean V8 checkout을 읽기 전용 빌드 입력으로 지정했다. opt-in build도 성공했고 Mach-O SHA-256은 `7b864459ec530171a0bf00610c0420e39e394274539cdb8002c1603fbb35b14d`; FFI symbol 4개와 fixed-scene payload가 있다. [default build log](./ios-default-build.log) · [default artifact 검사](./ios-default-artifact-check.txt) · [opt-in build log](./ios-optin-build.log) · [opt-in artifact 검사](./ios-optin-artifact-check.txt).
- `UIPanGestureRecognizer`의 `.began`을 native slop 경계로 사용하며 UIKit 임계값은 수치로 노출되지 않는다. 4pt 이하 움직임은 gesture callback을 만들지 않는 control이었다. 실제 위 swipe는 inner offset `70 CSS px`, revision 14와 red marker 이동을 만들었다. 최신 opt-in build의 연속 swipe는 inner `106`, outer `186`, root `238 CSS px`, revision 106까지 이어졌다. [ready](./ios-iphone17pro-ready.png) · [swipe 후](./ios-iphone17pro-after-swipe.png) · [nested chaining 화면](./ios-iphone17pro-chained-scroll.png) · [root max 화면](./ios-iphone17pro-root-chain.png) · [below-system-slop](./ios-iphone17pro-below-native-slop.png) · [nested chaining trace](./ios-c13-3-touch-summary.log).
- HOME을 보내는 중 native recognizer failure가 active gesture를 cancel하고 복귀 뒤 새 gesture를 받는 로그가 있다. 이 관찰은 `didEnterBackgroundNotification` observer가 직접 호출됐다는 증거로 확대하지 않는다. [touch summary log](./ios-c13-3-touch-summary.log) · [resume 화면](./ios-iphone17pro-after-resume.png).
- 기본 앱의 fixture route disabled 화면: [feature-off screenshot](./ios-iphone17pro-feature-off.png)

## 현재 동작 경계

- root max 화면은 renderer clip escape가 아니었다. `chained-scroll`과 `root-chain`을 RGB pixel 비교한 결과 277,200개 변경 픽셀의 bbox는 `[150,810) × [1098,1518)`이고, 400×240pt Metal canvas `[3,1203) × [951,1671)` 안이다. 캔버스 바깥 변경은 0픽셀이다. root content가 scrollport를 채우고 nested box가 사라지는 정상 화면 변화다. 플랫폼별 표본·해시는 [pixel 경계 대조](./screen-bounds-check.txt)에 기록했다.

- 두 플랫폼 모두 single-pointer direct touch, native slop과 고정 fixture의 `auto/auto` chaining만 연결했다. computed-style `overscroll-behavior`/`touch-action` projection은 renderer scroll policy에 연결되지 않았다.
- Android second-pointer 동작은 synthetic probe뿐이고 iOS 실제 multi-touch는 미검증이다. 실제 tap/click activation 결합, wheel/trackpad, local overscroll glow/bounce, runtime layout/style invalidation, S04.10 displayed-frame hit target과 S05.2 public events는 미완료다.
- `ScrollStateRevision`은 offset 변화가 게시될 때만 증가한다. Android AVD와 iOS Simulator 결과를 real device, hardware GPU, optical scanout 또는 CSS/WPT conformance로 해석하지 않는다.
- fixture FFI, JNI/ObjC wrapper, route callback과 payload는 opt-in build에서만 활성화한다. C13.3 전체 및 상위 C13은 미완료다.

## 변경 검토

- [platform adapter 구현 실패 경로 20개](../c13-3-platform-input-adapters-implementation-review-20-2026-10-15.md)
- [PR 변경 통합 검토 30개](../c13-3-platform-input-adapters-pr-review-30-2026-10-15.md)
