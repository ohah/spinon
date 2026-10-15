# C13.3 computed scroll policy 연결 · 2026-10-15

## 범위와 사전 판정 기준

- 기준 구현은 Chrome for Testing 155.0.8059.40의 Agent Browser touch-emulation 비교다. 고정 viewport는 400×240 CSS px, DPR 1이며 fixture·관찰 JSON·화면은 [사전 비교](../c13-3-scroll-gesture-precomparison-2026-10-15/README.md)와 연결한다.
- 이 변경은 `RuntimeBlockFlexScrollGestureV1`에서 computed `overscroll-behavior-x/y`를 scroll container의 축별 input policy로, computed `touch-action`을 hit target부터 가장 가까운 scroll owner까지의 immutable snapshot chain으로 연결한다.
- 입력 계약과 제한 profile은 저장소의 `plan/c13-3-scroll-gestures.md`를 따른다. Chrome의 혼합축 diagonal 결과가 축 독립 계약과 다르면 계획 계약을 pass oracle로 유지한다.
- 앱 재현 장면은 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 동일한 400×240 CSS nested-scroll fixture다. 앱 장면의 computed style은 기본 `auto/auto`이며, non-default `contain`/`pan-x` 정책 전달은 opt-in Rust fixture tests에서 검증한다. 실기기, terminal-browser, native wheel/trackpad는 근거에 포함하지 않는다.

사전 판정 기준은 다음과 같다.

1. `overscroll-behavior-x/y` computed 값이 같은 owner의 immutable `RuntimeScrollContainer` policy로 전달되고, `overflow:auto/scroll`이면서 해당 축 range가 있을 때만 직접 소비한다. `overflow:hidden`은 C13.2처럼 직접 소비하지 않는다.
2. gesture 시작 시 hit target에서 가장 가까운 scroll owner까지 computed `touch-action`을 교집합한다. 지원 값은 `auto`, `none`, `pan-x`, `pan-y`; 지원 밖 값은 fail-closed한다.
3. inner X `contain`은 남은 X delta를 막고, Y `auto`는 ancestor로 전달한다. inner-content `pan-x`는 수직 pan을 막고 수평 pan을 허용한다.
4. computed default `auto/auto`는 기존 C13.3 app adapter 동작과 같고, 기본 artifact에는 C13.3 fixture symbol·payload가 들어가지 않는다.
5. focused test, workspace test, Clippy, rustfmt, diff check를 통과하고 두 모바일 simulator의 실제 app 입력 trace와 캡처를 저장한다.

## 구현

- C13.3 fixture feature 아래에 별도 runtime cascade/profile dispatch를 추가해 제한된 computed style 결과가 renderer까지 손실 없이 도달하게 했다.
- `overflow:auto/scroll`과 실제 nonzero range를 확인한 뒤 축별 `overscroll-behavior`를 router policy에 기록한다. root viewport owner는 fixture root computed style을 사용한다.
- 각 render box metadata에 hit target→nearest scroll owner의 `touch-action` chain을 immutable `Arc`로 저장한다. 시작점 조회는 clip-aware hit target을 사용하고, 하나라도 지원하지 않는 값이면 `Unsupported`를 돌려 session이 pan을 거부하게 한다.
- Android fixture ready marker와 touch listener를 generated `BuildConfig.SPINON_C133_GESTURE_FIXTURE_TEST`에도 묶었다. 첫 artifact 점검에서 기본 APK의 `classes2.dex`에 ready marker가 남는 것을 발견해 두 경로에 compile-time guard를 추가했다. 수정 후 기본 APK의 marker·FFI symbol 검사는 0건이다.

## Rust 검증

| 명령 | 결과 |
| --- | --- |
| `cargo test --locked -p spinon-style-to-render` | 70 passed, 0 failed, 1 ignored |
| `cargo test --locked -p spinon-style-to-render --features c13-3-gesture-fixture-test c13_3_scroll_policy -- --nocapture` | 3 passed |
| `cargo test --locked -p spinon-runtime --features c13-3-gesture-fixture-test` | 141 passed, 0 failed, 3 ignored; integration 3/3과 lifecycle 16/16 통과 |
| `cargo test --locked -p spinon-ffi --features c13-3-gesture-fixture-test` | 43 passed, 0 failed, 2 ignored |
| `cargo test --locked --workspace` | 861 passed, 0 failed, 4 ignored |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | 통과 |
| `cargo fmt --all -- --check` | 통과 |
| `git diff --check` | 통과 |

새 focused tests는 축별 contain 전달과 남은 Y delta의 root 전달, target→owner `pan-x` 교집합 및 수직 pan 거부, 중간 노드 교집합·nearest owner 경계·unsupported 값 fail-closed를 확인한다. focused 결과는 `focused-scroll-policy-test.log`, `runtime-feature-test.log`, `ffi-feature-test.log`, `workspace-test.log`에 있고 정적 검사는 `clippy.log`, `rustfmt.log`, `diff-check.log`에 있다.

## Simulator 동작

### Android API 37 ARM64 AVD

- `adb -s emulator-5554 shell input swipe 300 1220 300 1100 450`로 실제 simulator touch를 보냈다. 명시 serial을 사용했다.
- trace는 hit target `7`, inner offset `45.2061 CSS px`, `ScrollStateRevision(22)`를 기록했다. outer/root offset은 `0`이었다.
- swipe 뒤 빨간 marker가 파란 inner border 안에 남았다. 캡처: [준비 화면](./android-api37-avd-ready.png) · [swipe 뒤](./android-api37-avd-after-swipe.png). 전체 입력 trace는 `android-api37-avd-run.log`다.

### iPhone 17 Pro · iOS 26.2 Simulator

- `idb ui swipe --udid ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D --duration 0.45 95 450 95 370`으로 simulator touch를 보냈다.
- trace는 hit target `7`, inner offset `70 CSS px`, `ScrollStateRevision(7)`을 기록했다. outer/root offset은 `0`이었다.
- swipe 뒤 빨간 marker가 파란 inner border 안에 남았다. 캡처: [준비 화면](./ios-iphone17pro-ready.png) · [swipe 뒤](./ios-iphone17pro-after-swipe.png). 전체 입력 trace는 `ios-iphone17pro-run.log`다.

| 캡처·로그 | SHA-256 |
| --- | --- |
| Android ready | `f32e92cdb1a0a91cf9c81e72f404180c333b2d61ecff953b563d3595bc915e81` |
| Android after swipe | `d26d65276b90c61304c4d5d5343e7f5fa9f6b21ef2c9e90f6703234094dcdde9` |
| Android trace | `a1e85ccdfe05a545a80a3e91777064200845021cdb0ca4218051483cb626c18e` |
| iOS ready | `af84839ad6184d3c63417fa2bd1351c6f41c040d46011094e3c542969db18823` |
| iOS after swipe | `e11baa1a3ad2c83d463a97584ebe8f0066dce538e97789beebfbea0526ae2536` |
| iOS trace | `5f860bf5bce754066b93b929dd8e70c25256fc5fb046cc0e6151d76403dde6e1` |

두 simulator는 현재도 종료하지 않고 켜 둔 상태다. 실제 기기 결과가 아니며 compositor 제출 뒤 frame이나 hardware GPU 표시 동작을 증명하지 않는다.

## 기본·opt-in artifact 경계

[feature-boundary.md](./feature-boundary.md)에 기본/opt-in Android·iOS 산출물의 SHA-256과 marker/symbol 양성·음성 결과를 기록했다. 기본 Android APK와 기본 iOS executable에서 C13.3 fixture ABI·ready payload가 없고, opt-in artifact에는 양성 대조 문자열이 있다.

빌드 source revision, simulator ID와 전체 app/artifact·화면 hash 연결은 `source-revision.json`에 기록했다.

## 남은 검증 범위

- Android/iOS app scene은 여전히 default `auto/auto` style을 사용한다. non-default `contain`/`pan-x`는 Rust focused tests 결과이며 simulator의 CSS style matrix 결과가 아니다.
- `none` 대 `contain`의 local overscroll affordance, mobile `auto/contain/none` 전체 행렬, wheel/trackpad, tap activation 통합, style/policy invalidation, 실제 iOS multi-touch, root/body 특수 전파 규칙은 남아 있다.
- S04.10 displayed-frame input, S05.2 public DOM event, WPT 적합성, compositor/scanout, hardware GPU 및 C13.3 완료 판정은 범위 밖이다. C13.3 상태는 미완료로 유지한다.
- 저장소 전체 Rspress build는 기존 evidence의 누락된 screenshot import에서 실패했다. TailScale용 두 HTML은 현재 Markdown의 정적 미리보기 변환이며 전체 docs site build 성공으로 보고하지 않는다. [빌드 경계](./docs-build-note.md).

구현 실패 관점 20개는 [implementation-review-20.md](./implementation-review-20.md), 변경분 관점 30개는 [pr-delta-review-30.md](./pr-delta-review-30.md)에서 확인할 수 있다.
