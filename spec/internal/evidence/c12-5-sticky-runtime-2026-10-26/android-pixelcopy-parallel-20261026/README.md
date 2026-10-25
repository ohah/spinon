# C12.5 Android PixelCopy · 2026-10-26

## 판정

API 37 ARM64 AVD `emulator-5560`에서 C12.5 sticky runtime fixture가 통과했고, `SurfaceView` PixelCopy 색상 표본은 2/2 exact 일치했다. PixelCopy 캡처는 실제 앱 surface를 읽었으며 Chrome 기준값과 비교했다. 실기기나 hardware GPU 결과로 확대하지 않는다.

## 실행 및 artifact

- AVD: Android API 37 / Android 17, `sdk_gphone16k_arm64`, `arm64-v8a`; 1080×2400, density 420.
- Vulkan renderer: Goldfish GFXStream / llvmpipe (LLVM 21.1.4), software/virtualized AVD renderer.
- APK SHA-256은 요청된 `51233bf737d7f7f2d2c6e2a7f33e5e39c10ac95198f303c9f33cab220525cacc`와 일치했다. 설치된 `base.apk`를 에뮬레이터에서 다시 읽어 계산한 해시도 같았다. build log의 `c12_5_sticky_fixture_test=1`을 확인했다.
- native build output (unstripped) SHA-256은 요청된 `dca1cef5f08d4699152fc2bc96da5d67d0115fd676d250ef16222aea187a2815`와 같다. APK 안의 ARM64 library는 strip 이후 `7bf577984345066d2120208bcdfb1bee44b7fce18ea33a45fd3c20cbef8a4656`이다.
- source snapshot은 branch `feat/c12-5-sticky-20261026`, HEAD `d94f645843fca7ca431a3c51fc3519cced9109c0`와 dirty diff fingerprint `e24e8c016864733713418fbadb6318a54f3bafe9ca096bbe447f4590dbbb2a52`로 기록했다. 관련 source hashes와 build log 경로는 `source-revision.txt`에 있다. APK와 native build output hash가 기대값과 일치해 artifact/source mismatch는 발견하지 않았다.

## 결과

- runtime summary: `status=passed`, 13 boxes, `scroll_state_revision=5`, viewport 360×260 CSS px, scroll/clip/hit fixture 통과.
- PixelCopy: run `4bbd9533cfb2`; surface 945×683; `scale_x=2.625`, `scale_y=2.626923076923077` (delta 0.001923076923, 어댑터 허용 0.01 이내).
- `floor((css + 0.5) × scale)` 좌표 변환으로 CSS `(200,78)` → surface `(526,206)`, `#db3344` → RGBA `(219,51,68,255)` exact.
- CSS `(240,78)` → surface `(631,206)`, `#f5f5f5` → RGBA `(245,245,245,255)` exact.
- 어댑터의 log sample 외에 PixelCopy PNG를 로컬에서 다시 디코딩해 두 좌표의 RGBA를 독립 확인했다. 결과는 `pixel-coordinate-validation.txt`, `pixelcopy-surface.png`, `android-pixelcopy-current-run.log`에 있다.

## 기기 상태와 제한

`adb-state-before.txt`에서 전후 대상 목록을 기록했다. 물리 serial `R5KYB06SMPE`는 목록 관찰 외에 명령을 보내지 않았다. 테스트 전 foreground였던 Chrome first-run activity가 테스트 뒤 같은 activity/task로 복구됐음을 `foreground-restoration.txt`에 기록한 다음 지정 AVD를 종료했다. 종료 후 목록에는 `emulator-5560`가 없고 물리기기는 그대로 연결돼 있다.
