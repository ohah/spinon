# C11.4 · auto-repeat 고정 `calc()` 최소 track 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** Chrome 156 oracle와 CSS Grid 2025 CRD 독립 기대값을 여섯 고정 scene으로 기록했다. 계획 실패 관점 20개를 구현 전에 별도로 검토한다. 구현/runtime과 C11.4 완료 판정은 아직 없다.

## 결손과 목표

`RuntimeBlockFlexGridTrackSizingV1`에서 auto-repeat track 최소값은 [`auto_repeat_track_has_fixed_minimum`](../crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs)의 allowlist를 통과해야 한다. 현재 `LengthPx`와 `minmax(LengthPx, LengthPx | Fraction)`만 통과해 CSS Grid `<fixed-size>` 중 `minmax(calc(절대 길이), 1fr)`를 Stylo computed `Calc`로 받은 경우 cascade가 거부한다. Taffy 0.15.0과 Spinon의 typed math 경계는 이미 track minimum calc를 표현할 수 있으므로, 이 계획은 preflight allowlist만 제한적으로 넓힌다.

목표는 track 최소값 `calc()`를 텍스트로 다시 파싱하거나 숫자 상수로 바꾸지 않고, 검증 후 기존 typed `Calc`로 Taffy에 전달하는 것이다. CSS Grid 2025 CRD [§7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes)와 [§7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat)은 fixed-size repeat과 definite 크기 기반 repeat count를 정의한다.

## 구현 범위

- 기존 opt-in `RuntimeBlockFlexGridTrackSizingV1`만 대상으로 한다. 단일 track `repeat(auto-fill | auto-fit, minmax(calc(...), 1fr))`에서 computed calc가 순수 절대 길이이고 결과가 유한한 양수 CSS px일 때 허용한다.
- 계산식은 `LengthPx`와 `Number`의 typed leaf, 합·곱·부호·역수·`min()`·`max()`·`clamp()`를 dimension-safe 방식으로 평가해 preflight한다. 최종 결과는 길이여야 한다. 모든 중간 결과도 유한해야 한다. 퍼센트, 음수·0 결과, 비유한/범위 초과 결과, 잘못된 차원, 비지원 AST는 fail-closed다. 기존 typed `Calc` node 자체는 값 보존과 Taffy evaluation을 위해 유지한다.
- `auto-fill`/`auto-fit` columns와 rows를 각각 검사한다. 각 장면은 축 하나에 auto-repeat 하나만 쓰고, gap 0·fixed 184px content box로 다른 sizing 변수를 제거한다. 반복 최소값은 `calc(20px + 20px)`이고 최대값은 `1fr`이다.
- fixed `minmax(40px,1fr)` control이 calc scene과 같은 결과를 내야 한다. 한 item이 있는 `auto-fit`은 배치 뒤 세 빈 track을 collapse한다.
- 퍼센트가 든 calc, zero minimum, 두 개 이상 repeated track pattern, fixed maximum, intrinsic maximum, line-name/area/placement 결합, 양축 동시 repeat, WPT, 일반 CSS admission은 제외한다. 기존 `LengthPx` auto-repeat, typed implicit sizing, named-line 및 area 경계는 회귀 검증한다.

## 사전 비교 모델과 고정 입력

[구현 전 비교 모델](../spec/0001-conformance.md#구현-전-비교-모델), [고정 CSS fixture](../tests/fixtures/css/c11/grid-auto-repeat-calc-minimum-c114.html), [Chrome 156 관찰과 독립 계산](../spec/internal/evidence/c11-4-auto-repeat-calc-minimum-precomparison-2026-10-23/README.md)을 기준으로 한다. Chrome은 Agent Browser 0.31.1 `HeadlessChrome/156.0.0.0`, viewport 1280×633 CSS px, DPR 1이다.

| Scene ID | 축·입력 | 독립 기대값: used tracks; item frame (root 원점 기준) |
| --- | --- | --- |
| `columns-calc-fill` | 184px content width, `auto-fill`, calc min, 4개 in-flow items | Columns `[46,46,46,46]px`; items `[[0,0,46,48],[46,0,46,48],[92,0,46,48],[138,0,46,48]]` |
| `columns-fixed-fill-control` | 동일 입력, `minmax(40px,1fr)` | Columns `[46,46,46,46]px`; 동일 item frame |
| `columns-calc-fit` | 동일 width, `auto-fit`, item 한 개 | 반복 전 4개, collapse 후 `[184,0,0,0]px`; item `[0,0,184,48]` |
| `rows-calc-fill` | 184px content height, `auto-fill`, calc min, 4개 in-flow items | Rows `[46,46,46,46]px`; items `[[0,0,48,46],[0,46,48,46],[0,92,48,46],[0,138,48,46]]` |
| `rows-fixed-fill-control` | 동일 입력, `minmax(40px,1fr)` | Rows `[46,46,46,46]px`; 동일 item frame |
| `rows-calc-fit` | 동일 height, `auto-fit`, item 한 개 | 반복 전 4개, collapse 후 `[184,0,0,0]px`; item `[0,0,48,184]` |

모든 scene에서 root와 item frame, explicit/used track list, repeat count, `auto-fit` collapse 및 C11.4 stage trace를 scene ID별 비교한다. definite 184px에 최소 40px짜리 track 네 개가 들어가고 남은 24px을 네 `1fr` 최대값이 나눠 갖는다. `auto-fit`은 placement 뒤 비어 있는 세 track을 접으며, gap이 0이므로 occupied track은 184px이다. 실제 기대 배열과 Chrome 관찰이 다르면 구현 전에 계획/fixture를 고친다.

## 구현·검증 관문

1. `track_size`가 pure absolute calc를 `ComputedCssMath` AST로 유지하는지, layout projection이 같은 node를 `LayoutCssMathValue`로 Taffy에 넘기는지 확인한다. text serialization 재파싱과 calc 값의 eager replacement는 허용하지 않는다.
2. cascade preflight에 dimension-preserving finite evaluator를 추가한다. Length×Length, unresolved percentage, scalar 최종값, 0/음수, 중간 overflow, `NaN`/infinity, 잘못된 AST를 거부한다. 기존 metadata·allocation fail-closed 경계와 profile gate를 유지한다.
3. cascade unit 검증은 각 허용 연산과 잘못된 차원·퍼센트·수치 경계를 분리한다. 퍼센트 calc 거부 시 partial `LayoutOutput`이 없고 같은 session의 corrected style revision이 성공하는 recovery control도 검사한다. 기존 `LengthPx`, explicit `Calc`, named-line, one/two-axis area auto-repeat을 focused regression에 포함한다.
4. 여섯 fixed scene을 app의 opt-in fixture route에 한 번씩 연결한다. 사용 중인 주 checkout에서 별도 feature branch로 진행하며 worktree는 만들지 않는다. Android API 37 ARM64 AVD `emulator-5558`과 iPhone 17 Pro/iOS 26.2 Simulator `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`에서 exact scene ID, root/item frame, axis track, repeat count, `auto-fit` collapse, trace 및 draw/present를 모두 확인한다.
5. 기본/product FFI static archive에서 fixture scene marker·전용 symbol이 빠지고 `c04-runtime-gpu-fixture-test` positive control에 포함되는지 대조한다. 최종 APK/IPA 제외를 확인하지 않으면 그 경계를 PR에 쓴다.
6. 관련 Rust 검증, fixture validator, `cargo fmt --all -- --check`, workspace all-target Clippy와 Android/iOS app build를 실행한다. 실행하지 않은 검사는 통과로 기록하지 않는다. 캐시가 정리된 상태에서 재생성되는 build target 크기와 사용 시간은 보고한다.
7. 구현/runtime 동작은 이 계획 표를 재사용하지 않은 새로운 실패 관점 20개로 검토한다. 구현 PR diff는 별도의 30관점으로 검토한다. 결함을 찾으면 수정 후 영향을 받는 관점을 다시 판정한다.
8. 구현 PR 본문에 oracle·simulator scene, 화면과 hash, static archive 결과, 미검증 범위를 적고 가능한 Android/iOS screenshot을 이미지로 첨부한다. Agent Browser에서 PR 화면 이미지가 실제 표시되는지 확인하고 세션/process를 종료한다.
9. 구현 병합 뒤 요청된 Tailnet roadmap를 선택 파일 hash manifest로 갱신한다. HTTPS 상태·bytes/hash와 Agent Browser 렌더링을 확인한 뒤 Agent Browser를 종료한다. 계획/구현/Tailnet PR은 각각 current required-check와 merge state를 조회해 Rebase and merge한다.

## 계획 자체 적대 검토

[서로 다른 실패 관점 20개](./c11-4-auto-repeat-calc-minimum-plan-review-20.md). 이 계획 검토는 구현/runtime 검토와 PR diff 검토에 포함하지 않는다.

계획 PR 변경사항은 별도의 서로 다른 실패 관점 30개로 검토하고 [plan-pr-diff-review-30.md](../spec/internal/evidence/c11-4-auto-repeat-calc-minimum-precomparison-2026-10-23/plan-pr-diff-review-30.md)에 기록한다. 구현/runtime 20개와 구현 PR diff 30개는 각각 새로 검토한다.

## 상태·계약 해석

이 제한 slice는 C11.4 또는 C11 전체 완료를 뜻하지 않는다. 미지원 calc 형태는 계속 fail-closed다. 공개 CSS admission과 WPT 적합성 주장을 추가하지 않고, 내부 계약 numeric version `0.1.0`과 공식 `72/213` 집계를 유지한다.
