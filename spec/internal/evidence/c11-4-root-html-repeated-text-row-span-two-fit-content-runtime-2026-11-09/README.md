# 반복 텍스트 row-span의 두 `fit-content()` 행

## RuntimeSession 및 Rust 검증

- Chrome 157 관측 3,840/3,840 scene을 parsed Root HTML `RuntimeSession` matrix에서 대조했다. 같은 실행에서 min-content-floor control과 three-track atomic rejection/same-session recovery도 포함했다(총 3 tests, 0 failures, 171.42s).
- 기존 single-fit row-span 회귀: authored positions 1–3의 1,920 scene, positions 4–6의 4,320 scene, 기존 960-scene matrix가 통과했다. 연관된 rejection/recovery 검사도 집중 실행에서 각각 4, 3, 3 tests 통과했다.
- Layout profile은 15개 두-position pair와 고정 tail order를 모두 통과했다. fractional remainder 배분과 auto track 없는 used-prefix를 포함한 `blink_` distribution/alignment 5개도 통과했다.
- `spinon-layout`, `spinon-style-to-layout`, `spinon-runtime`의 feature-enabled Clippy가 `-D warnings -A clippy::too_many_arguments`로 통과했다. 예외는 무관한 기존 fixture helper 2개에만 적용했다. `cargo fmt --all -- --check`와 `git diff --check`도 통과했다.
- Chrome 157 `chrome157-min-content-floor-control.json`은 두 `fit-content(0.5px)` 행의 min-content floor가 cap을 넘는 대조다. 연결 screenshot은 관찰용이며 RuntimeSession이 row trace, item frame, paint box, text descriptor를 별도로 검사한다.
- [구현/runtime 실패 관점 검토 20개](implementation-runtime-review-20.md), PR #1311 전체 변경의 [별도 diff 검토 30개](pr-diff-review-30.md), Android 보충 실행의 [증거 검토 20개](android/api37213-swangle-existing-avd-readonly-2026-11-10/android-evidence-review-20.md)를 남겼다.

## iOS Simulator

- 임시 iPhone 17 Pro / iOS 26.2 Simulator에서 기본 Root HTML 경로를 실행했다. `layout=ready boxes=12`, CSS root `402×220`, Metal renderer, 네 text payload와 `SPINON_C0410_DRAW ... presented`를 기록했다.
- [Screenshot](ios/screenshot.png)은 OS 화면 visual smoke다. Metal drawable readback이나 pixel parity를 증명하지 않는다. [Runtime marker](ios/runtime-markers.log)를 보존했다.
- 임시 Simulator를 종료하고 삭제했다. 기존 iPhone 17 Pro/Pro Max 프로필은 그대로 보존했다. 현재 iOS 26.2 inventory에는 `S03` 프로필이 없다.

## Android Emulator

- API 37.2 `google_apis_ps16k` 임시 AVD는 5분 39초 offline 뒤 종료·삭제했다. Emulator를 37.2.13으로 올린 뒤에도 새 API 37.2 AVD가 5분 08초 offline이었다. console은 응답했지만 ADB는 앱 실행 전 상태에서 회복되지 않았다. [당시 기록](android/api37213-emulator-offline.md)을 보존했다.
- Emulator 37.2.12의 API 37.0 `google_apis` AVD도 앱 설치 전에 실패했다. macOS는 Hypervisor.framework를 보고했으나 Emulator는 `hvf is not enabled`, 반복된 `mprotect failed: Permission denied`, QEMU thread hang 후 status 139를 기록했다. 상세는 [API 37.0 QEMU 기록](android/api370-emulator-hang.md)에 있다.
- 후속 실행은 기존 API 37.2 AVD의 userdata/cache를 격리 임시 경로로 보내고 기존 프로필을 read-only로 열었다. `-gpu swangle`의 Android 17/API 37 ARM64 환경에서 기본 Root HTML 앱 경로의 Chrome sentinel 세 개가 모두 `layout=ready`와 `SPINON_C0410_DRAW status=0 presented`에 도달했다. early `[1,2]`/`start`/`span2Six-forward`, adjacent `[3,4]`/`stretch`/`span2Span3Six-reverse`, separated `[1,6]`/`stretch`/`span2Span3Six-forward` 결과는 [Android 실행 근거](android/api37213-swangle-existing-avd-readonly-2026-11-10/README.md)에 있다.
- 세 Android screenshot은 화면 visual smoke다. PixelCopy, surface readback, pixel parity, 실기기 또는 hardware GPU 증거가 아니다. 앞선 AVD boot 실패 기록은 성공한 후속 실행과 분리해 보존했다.
- 이전 PackageManager parse failure는 별개의 미귀속 실패다. 그때 재사용한 AVD app data는 보존했고 원본 APK restore는 `Success`를 반환했다.

## 범위와 정리

- 이 결과는 bounded 구현 후보다. 일반 CSS Grid 지원, WPT 적합성, hardware-device parity, C11.4/C11 완료를 뜻하지 않는다. 공식 완료 집계는 `78/214`다.
- 이번 실행에 쓴 임시 iOS Simulator와 Android Emulator를 종료했다. Android 임시 userdata/cache와 생성한 APK 출력 폴더를 제거했다. 재사용 가능한 AVD 프로필/snapshot, 기존 iOS 프로필, 공유 SDK와 빌드 캐시는 보존했다. 이 작업에서 띄운 Agent Browser 세션은 남아 있지 않다.
