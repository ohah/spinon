# C11.4 · definite auto-repeat percentage `calc()` 최소값 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** CSS Grid 2025 CRD 독립 기대값·Chrome 156 비교와 [계획 실패 관점 20개](./c11-4-auto-repeat-percentage-calc-minimum-plan-review-20.md)를 고정했다. definite axis에 한정한 `calc(length + percentage)` minimum slice를 구현하고 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각 6/6 scene을 확인했다. [구현/runtime 실패 관점 20개](../spec/internal/evidence/c11-4-auto-repeat-percentage-calc-minimum-runtime-2026-10-23/implementation-runtime-review-20.md)와 [PR diff 실패 관점 30개](../spec/internal/evidence/c11-4-auto-repeat-percentage-calc-minimum-runtime-2026-10-23/pr-diff-review-30.md)를 별도 기록했다. 구현 PR [#556](https://github.com/ohah/spinon/pull/556)이 열려 있으며 C11.4 전체 완료 판정은 아니다.

## 결손과 목표

직전 [auto-repeat pure-absolute `calc()` 최소값 slice](./c11-4-auto-repeat-calc-minimum.md)는 `calc(20px + 20px)`가 cascade에서 `LengthPx(40)`으로 정규화되는 경우만 앱 runtime에서 확인했다. 퍼센트가 남는 `calc()` 최소값은 기존 auto-repeat preflight에서 거부한다. CSS Grid 2025 CRD §7.2.1의 `<fixed-breadth>`는 `<length-percentage>`이며, definite container에서 columns의 퍼센트 기준은 inner inline size, rows의 기준은 inner block size다. §7.2.3.1의 auto-repeat은 fixed-size track을 요구하고, §7.2.3.2는 해당 axis의 definite size와 fixed minimum을 사용해 반복 수를 정한다.

목표는 `repeat(auto-fill | auto-fit, minmax(calc(20px + 10%), 1fr))`를 definite 112px axis의 기존 opt-in `RuntimeBlockFlexGridTrackSizingV1` fixture profile에서만 fixed `31.2px` control과 비교하는 것이다. 일반 CSS admission, 새 calc evaluator/parser, cyclic/indefinite percentage resolve 또는 계약 버전 변경은 추가하지 않는다. 참고 기준은 CSS Grid Layout Module Level 1 2025 CRD [§7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes)과 [§7.2.3.1–§7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#repeat-syntax), CSS Values and Units Level 4 [§5.6, §10](https://www.w3.org/TR/css-values/)다.

## 구현 범위와 fail-closed 경계

- CSS 입력은 `minmax(calc(20px + 10%), 1fr)` 하나와 numeric `minmax(31.2px, 1fr)` control이다. 지원하는 calc tree는 finite·nonnegative percentage와 finite·positive length 항으로 제한해 definite basis에서도 최소값이 0이 되지 않게 한다.
- columns에서는 definite 112px inner inline size, rows에서는 definite 112px inner block size를 쓴다. 각 scene에는 한 axis에 auto-repeat 하나만 둔다. gap·border·padding은 0이다.
- `auto-fill`은 세 반복 track과 세 item을 만든다. `auto-fit`은 세 반복 track 중 하나만 점유하고 배치 뒤 빈 두 track을 collapse한다. 반복 전 track count, 최종 used tracks, 모든 node frame과 C11.4 trace를 비교한다.
- 퍼센트 기준은 container size에 대한 것이며 viewport, device-pixel, border-box 또는 반대 axis에서 가져오지 않는다. percent와 absolute length가 함께 있는 `calc()`가 computed style에서 보존되면 기존 math binding을 사용한다. 원문 CSS 문자열 재파싱이나 별도 평가기는 추가하지 않는다.
- 두 축 동시 repeat, 한 axis의 여러 repeat, named line/area, nonzero gap, fixed-neighbor, implicit track, auto placement 변형, indefinite/cyclic root size, intrinsic root sizing, 음수·비유한 식, 일반 authored CSS runtime, WPT는 제외한다. 이 결과를 다른 percentage basis나 임의 `calc()` 문법으로 일반화하지 않는다.
- 기본/product FFI와 opt-in fixture-test archive를 비교하되 정적 archive 결과를 APK/IPA 전체에서 fixture 제외의 증거로 확대하지 않는다. PixelCopy/Metal 표면 픽셀 equality는 별도 명시 장면만 비교하며 iOS 표본은 compositor 제출 전 경계다.

## 사전 비교 모델과 고정 입력

[구현 전 비교 모델](../spec/0001-conformance.md#구현-전-비교-모델), [고정 CSS fixture](https://github.com/ohah/spinon/blob/main/tests/fixtures/css/c11/grid-auto-repeat-percentage-calc-minimum-c114.html), [Chrome 156 관찰과 독립 계산](../spec/internal/evidence/c11-4-auto-repeat-percentage-calc-minimum-precomparison-2026-10-23/README.md)을 기준으로 한다. Agent Browser 0.31.1의 `HeadlessChrome/156.0.0.0`, viewport 1280×633 CSS px, DPR 1을 사용했다.

| Scene ID | 축·입력 | 독립 기대값: auto-repeat count와 used tracks; item frame (root 원점 기준) |
| --- | --- | --- |
| `columns-percent-calc-fill` | definite 112px inline size, `auto-fill`, `calc(20px + 10%)`, item 3개 | 10% = 11.2px, min = 31.2px, count 3; `1fr` used tracks 각각 112/3px; items x=`0, 112/3, 224/3`, 폭 각각 112/3px, 높이 48px |
| `columns-percent-fixed-fill-control` | 동일, numeric `31.2px` 최소값 | count 3; calc scene과 같은 tracks/frame |
| `columns-percent-calc-fit` | 동일, `auto-fit`, item 1개 | pre-collapse count 3; used tracks `[112,0,0]px`; item `[0,0,112,48]` |
| `rows-percent-calc-fill` | definite 112px block size, `auto-fill`, item 3개 | 10% = 11.2px, min = 31.2px, count 3; `1fr` used tracks 각각 112/3px; items y=`0, 112/3, 224/3`, 높이 각각 112/3px, 폭 48px |
| `rows-percent-fixed-fill-control` | 동일, numeric `31.2px` 최소값 | count 3; calc scene과 같은 tracks/frame |
| `rows-percent-calc-fit` | 동일, `auto-fit`, item 1개 | pre-collapse count 3; used tracks `[112,0,0]px`; item `[0,0,48,112]` |

명세에서 도출한 exact fractional used value는 112/3px이며 Chrome 직렬화/float 결과에는 최대 1/64 CSS px 오차를 허용한다. fixed control이 calc scene과 같아야 하고, minimum 20px로 percent를 잘못 버리는 구현은 5개 track을 만들어 이 입력에서 판정 실패한다. Chrome 관찰은 simulator acceptance를 대체하지 않는다.

## 구현·검증 관문

1. existing Stylo computed `LengthPercentage`의 `Calc` node가 `20px + 10%`를 보존하는지 확인한다. 숫자 control은 `LengthPx(31.2)`가 된다. 공개 문법 재파싱이나 전용 calc evaluator를 만들지 않는다.
2. auto-repeat preflight의 fixed-minimum 판정이 이번 positive `Calc`만 통과하도록 좁혀진다. invalid/unsupported 계산 값, indefinite basis와 다른 track grammar는 기존 fail-closed 오류를 유지한다.
3. 여섯 scene에서 columns/rows별 112px definite basis, 반복 전 count 3, used tracks, `auto-fit` 빈 track collapse, node frame과 stage trace를 scene ID로 검사한다. `calc()`와 numeric control의 독립 결과가 일치해야 한다.
4. Android API 37 ARM64 AVD `emulator-5558` 및 iPhone 17 Pro/iOS 26.2 Simulator `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`에서 각 6/6 scene의 node frame, track list, repeat/collapse trace와 draw/present를 확인한다. 표면 캡처를 수행할 경우 scene별 mask/표본 위치를 사전에 고정하고, overflow/border stroke나 iOS compositor 이후 equality를 확인하지 않은 표본은 그렇게 표시한다.
5. 기본/product FFI archive에서 새 fixture scene marker/host symbol이 없고 opt-in fixture-test positive control에만 있는지 확인한다. APK/IPA bundle exclusion은 별도로 검증하지 않는 한 주장하지 않는다.
6. Rust focused checks, 관련 fixture validator, `cargo fmt --all -- --check`, workspace all-target Clippy 및 Android/iOS simulator builds를 수행한다. 실행하지 않은 검사는 통과로 보고하지 않는다. cache cleanup 후 재생성된 target 크기와 검증 범위를 남긴다.
7. 계획의 실패 관점 20개와 중복되지 않는 구현/runtime 실패 관점 20개, 구현 PR diff 실패 관점 30개를 별도로 작성한다. 결함을 고치면 영향을 받은 관점을 다시 판정한다.
8. 구현 PR에 Android/iOS simulator screen을 가능하면 삽입한다. GitHub PR 화면은 Agent Browser에서 이미지 로드를 확인하고 사용한 브라우저 세션과 프로세스를 종료한다.
9. 병합 후 Tailnet road map을 hash manifest로 갱신하고 local/HTTPS bytes·SHA-256 및 Agent Browser 화면을 검증한다. Tailnet server는 유지하고 확인 후 Agent Browser를 종료한다.

## 구현 결과

- cascade preflight는 `Sum`의 정확히 두 항이 유한 positive `LengthPx`와 유한 nonnegative `Percentage`인 경우만 보존한다. 임의 함수, 0px minimum, 음수 항, 추가 항은 계속 거부한다.
- layout preflight는 calc binding의 node/property가 현재 grid와 일치하고 해당 축의 percentage basis가 definite일 때만 통과시킨다. columns는 definite inline size, rows는 definite block size를 사용한다.
- 첫 simulator 실패에서 native host의 예상 frame 문자열이 정수 frame에 `.0`을 붙이는 차이를 확인했다. 두 번째 실패에서는 auto-fit 한 칸 item에 auto-fill용 마지막 track residual을 적용하던 오류를 확인했다. Android/Swift 기대 frame formatter와 auto-fit 분기를 수정한 뒤 양 플랫폼의 최종 여섯 장면이 통과했다.
- vendored Taffy의 `TrackSizingFunction::has_fixed_component()`가 `calc()` breadth를 fixed component로 세지 않아 auto-repeat이 한 번만 만들어지는 결함을 수정했다. `calc` feature에서 plain calc component를 인식하고 `spec/internal/evidence/c11-4-auto-repeat-percentage-calc-minimum-runtime-2026-10-23/taffy-calc-auto-repeat-test.log`에서 definite calc minimum의 세 번 반복을 확인했다.
- Android API 37 ARM64 AVD `emulator-5558`와 iPhone 17 Pro/iOS 26.2 Simulator `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`에서 각 6/6 scene을 확인했다. Node frames는 독립 112/3px 기대값에서 1/64 CSS px 이내, used tracks/collapse/trace와 draw/present가 통과했다. 각 플랫폼의 auto-fill calc/control tracks와 frame이 일치한다.
- 전체 scene catalog는 기존 absolute calc 여섯 장면을 유지해 12개다. 별도 실행 근거는 [runtime README](../spec/internal/evidence/c11-4-auto-repeat-percentage-calc-minimum-runtime-2026-10-23/README.md), [구현/runtime 실패 관점 20개](../spec/internal/evidence/c11-4-auto-repeat-percentage-calc-minimum-runtime-2026-10-23/implementation-runtime-review-20.md)와 simulator result JSON에 있다.
- `cargo fmt --all -- --check`, 관련 Rust/FFI tests, fixture-boundary Node test, workspace all-target Clippy `-D warnings`, Android/iOS simulator builds가 통과했다. default `c04-runtime-gpu` FFI archive에서는 신규 scene marker/module symbol이 없고 `c04-runtime-gpu-fixture-test` positive control에는 있다. 이는 static archive 경계이며 APK/IPA 전체 exclusion은 검증하지 않았다.
- 공식 완료 수는 최신 상태 대장의 `72/213` 그대로이며 C11.4 checkbox도 미완료다. simulator 화면 캡처는 제출 결과의 화면 근거이고 PixelCopy/Metal 표본별 픽셀 equality를 뜻하지 않는다.

## 제외 범위

임의 퍼센트/length-percentage 함수, 다른 calc 연산, zero/negative minimum, cyclic·indefinite basis, indefinite auto-repeat의 one-repeat 규칙, auto-repeat과 areas/named lines/implicit placement 결합, multiple repeats, nonzero gutter, CSS author API, WPT suite, Android 실기기/hardware GPU, iOS compositor 이후 픽셀과 전체 C11.4는 이 계획으로 완료하지 않는다. 공식 C11.4 checkbox, 최신 상태 대장의 `72/213` 완료 수 및 내부 계약 `0059` 버전 `0.1.0`은 유지한다.
