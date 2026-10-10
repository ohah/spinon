# C07.3 Taffy 차이와 적용 경계

## 재현 기준

- Chromium `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`.
- fixture: `tests/fixtures/css/c07/aspect-ratio.html`, 35개 node, viewport `400×1200` CSS px, DPR 1·2.
- Taffy `0.14.0`.
- 원본 Chrome reference: `tests/fixtures/css/references/c07-3-aspect-ratio-v1.json`.

## 발견한 차이와 수정

| 입력 | Chrome | 수정 전 Taffy | 처리 |
| --- | ---: | ---: | --- |
| leaf `width:160px; height:80px; aspect-ratio:16/9` | `160×80` | `160×90` | leaf의 두 축 크기가 definite이면 호출용 Taffy style에서 aspect ratio를 제거한다. 지정 크기는 유지한다. |
| `width:100px; min-height:80px; aspect-ratio:2` | `100×80` | `160×80` | min/max와 ratio 조합은 node와 충돌 property를 포함해 layout 전 거부한다. |
| `width:160px; max-height:60px; aspect-ratio:2` | `160×60` | `120×60` | 같은 fail-closed 경계. |
| `height:40px; min-width:100px; aspect-ratio:2` | `100×40` | `100×50` | 같은 fail-closed 경계. |
| `height:80px; max-width:120px; aspect-ratio:2` | `120×80` | `120×60` | 같은 fail-closed 경계. |
| Flex child `width:120px; min-height:70px; max-height:80px; aspect-ratio:2` | `120×80` | `140×80` | 같은 fail-closed 경계. |

Taffy 0.14.0의 leaf 계산은 style 크기와 known size를 조합한 뒤 height를 `max(clamped_height, width / ratio)`로 만든다. 이 마지막 단계가 CSS에서 두 지정 크기가 모두 definite인 leaf까지 키웠다. `crates/spinon-layout/src/calc_tree.rs`는 resolved style width와 height가 모두 존재할 때만 layout 호출용 복제 style의 ratio를 끈다. source computed style과 다른 노드의 ratio는 바꾸지 않는다. `crates/spinon-layout/src/tests.rs`는 이 exact case가 `160×80`을 유지하는지 독립적으로 검사한다.

다섯 min/max 사례는 처음 계획과 달리 positive 지원에서 제외했다. `crates/spinon-style-to-layout/src/projection/style_values.rs`가 typed min/max 값과 preferred ratio가 함께 있는 요소를 발견하면 `UnsupportedAspectRatioConstraint { node, property }`를 반환해 partial layout을 만들지 않는다. Rust negative test는 네 축과 Flex child 사례를 확인한다. Chrome reference에는 값이 남아 있어 이후 Taffy 동작 수정 또는 별도 adapter 구현 때 회귀 비교에 사용할 수 있다.

## 비교 단계 수정

Chrome `getComputedStyle()`의 width/height는 이 fixture에서 `auto` 대신 resolved px를 반환한다. Stylo computed snapshot과 해당 문자열을 직접 비교한 초안 assertion은 잘못된 비교였다. computed style 대조는 `computedStyleMap()`의 `aspect-ratio`·`box-sizing`만 사용한다. 실제 width/height/min/max/padding/border 결과는 node별 `getBoundingClientRect()`와 Spinon frame으로 비교한다. `getComputedStyle()` raw map은 조사 자료로만 reference에 보존한다.

positive layout test의 30개 지원 node 뒤에 있는 위치를 계속 비교하기 위해 다섯 unsupported CSS case는 test-only fixed-size spacer style을 사용한다. Spacer의 frame은 지원 성공으로 비교하지 않는다. 각 case 자체의 동작은 위의 negative runtime assertion으로 확인한다.

## 현재 실행 결과와 한계

- `mise exec -- bun run test:css-reference`: 75/75 통과. C07.3 테스트는 고정 Chrome 입력과 35 IDs, DPR 1·2, reference digest, 지원 geometry, 플랫폼 연결 경로를 확인한다.
- `mise exec -- cargo test --locked --workspace --all-features --quiet`: 411 통과, 2 ignored.
- `mise exec -- cargo check --locked --workspace --all-features`: 통과.
- `mise exec -- cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: 한 `question_mark` 지적을 고친 뒤 통과.
- `SPINON_ENABLE_C04_RUNTIME_GPU=1 mise exec -- bun run build:android`: 성공. API 37 ARM64 emulator에서 `status=0 layout=ready boxes=5`, WGPU `presented boxes=5`를 확인했다. 선택된 GL 경로의 ANGLE/SwiftShader는 software renderer다.
- `SPINON_ENABLE_C04_RUNTIME_GPU=1 mise exec -- bun run build:ios-sim`: 성공. iPhone 17 Pro / iOS 26.2 Simulator에서 `status=0 layout=ready boxes=5`, WGPU `presented boxes=5`를 확인했다.
- 두 simulator 화면에 16:9, 1:1, 3:4 배경 사각형이 표시된다. 화면·요약 로그는 아래 파일에 보존했다. 이 화면은 자식별 Chromium geometry 계측이 아니다.

| 플랫폼 | 화면 | 로그 |
| --- | --- | --- |
| Android API 37 emulator | [PNG](./c07-3-aspect-ratio-android-emulator-2026-10-10.png) | [Logcat](./c07-3-aspect-ratio-android-emulator-2026-10-10.log) |
| iPhone 17 Pro / iOS 26.2 Simulator | [PNG](./c07-3-aspect-ratio-ios-simulator-2026-10-10.png) | [OSLog](./c07-3-aspect-ratio-ios-simulator-2026-10-10.log) |

SHA-256: Android PNG `65966213690175600a9cc3b4c4b9279ec908fd765257eda24cdc776c298ecdd1`; Android log `3c7a82a57bb18d2dee4bd0131bba8766109cd4c80046d7760e5851db8ec05a9b`; iOS PNG `b2957a0304094aee39faf1f8fd2697c59969a8b33fdf103718b3b1139dea316e`; iOS log `513185039a8e30a9bb7b1af44f1986dc0865c37e3013053dcce84ee0429ff4de`.

### 현재 빌드 재실행 · 2026-10-10 12:32–12:33 KST

- `mise exec -- bun run test:css-reference`: 75/75 통과.
- `mise exec -- cargo test --locked --workspace --all-features --quiet`: 411 통과, 2 ignored.
- `mise exec -- cargo fmt --all -- --check`, `mise exec -- cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, `git diff --check`: 통과.
- 12:28 빌드된 Android APK를 API 37 emulator에 재설치하고 C07.3 launch extra로 실행했다. 최신 logcat에서 `layout=ready`, `boxes=5`, ANGLE/SwiftShader 선택과 WGPU `presented boxes=5`를 다시 확인했다.
- 12:29 빌드된 iOS app을 iPhone 17 Pro / iOS 26.2 Simulator에 재설치하고 `--spinon-c073-aspect-ratio`로 실행했다. 최신 OSLog에서 `layout=ready`, `boxes=5`, WGPU `presented boxes=5`를 다시 확인했다.
- 위 화면·로그 파일과 SHA-256은 이번 재실행 산출물이다. simulator 시각 확인은 CSS geometry별 독립 oracle이나 hardware GPU 성능 측정이 아니다.

실기기·hardware GPU 성능 및 모바일 자식별 frame oracle은 측정하지 않았다.
