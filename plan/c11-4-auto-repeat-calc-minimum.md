# C11.4 · auto-repeat 고정 `calc()` 최소 track 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** Chrome 156 oracle와 CSS Grid 2025 CRD 독립 기대값을 여섯 고정 scene으로 기록했다. Stylo computed-value 정규화 뒤 기존 `LengthPx` 경로가 calc scene과 fixed numeric control에 동일하게 작동하는지 style·Android/iOS 시뮬레이터에서 확인했다. 이 제한 runtime 검증은 C11.4 전체 완료 판정이 아니다.

## 결손과 목표

초기 가설은 [`auto_repeat_track_has_fixed_minimum`](../crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs)이 CSS Grid `<fixed-size>`인 `minmax(calc(절대 길이), 1fr)`를 거부한다는 것이었다. 실제 cascade fixture로 `calc(20px + 20px)`, 곱셈·나눗셈, `min()`·`max()`·`clamp()` 등 절대 길이 식 10개를 확인한 결과 Stylo의 computed `LengthPercentage::unpack()`이 모두 `LengthPx(40)`으로 정규화했다. 기존 게이트는 이 값을 이미 허용한다. 초기 거부 가설과 “Stylo computed `Calc`” 전제는 재현되지 않았다.

수정 목표는 계산 경로를 추가하지 않고 canonical pure-absolute `calc()`가 existing `LengthPx` route에서 고정 numeric control과 동등한지 cascade 및 앱 runtime에서 확인하는 것이다. 비율이 포함된 식은 여전히 별도 scope이며 fail-closed 경계를 유지한다. CSS Grid 2025 CRD [§7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes)와 [§7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat)은 fixed-size repeat과 definite 크기 기반 repeat count를 정의한다.

## 구현 범위

- 기존 opt-in `RuntimeBlockFlexGridTrackSizingV1`와 현재 computed `LengthPx` auto-repeat 경로만 대상으로 한다. 고정 입력 `repeat(auto-fill | auto-fit, minmax(calc(20px + 20px), 1fr))`이 cascade 뒤 `LengthPx(40)`이 되고 기존 fixed-size allowlist에서 control과 같은 typed track으로 변환되는지 검증한다. 새 evaluator, parser, 일반 CSS 허용 경로는 추가하지 않는다.
- 범위는 이 fixture에서 관찰한 `calc(20px + 20px)` 한 식과 fixed `minmax(40px, 1fr)` 대조군이다. 다른 연산자·범위·차원 조합의 지원을 이 결과로 추론하지 않는다. percentage calc는 남아 있는 computed `Calc`가 기존 gate에서 거부되는 fail-closed control로 둔다.
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

모든 scene에서 root와 item frame, explicit/used track list, repeat count, `auto-fit` collapse 및 C11.4 stage trace를 scene ID별 비교한다. definite 184px에 최소 40px짜리 track 네 개가 들어가고 남은 24px을 네 `1fr` 최대값이 나눠 갖는다. `auto-fit`은 placement 뒤 비어 있는 세 track을 접으며, gap이 0이므로 occupied track은 184px이다. Android/iOS 화면은 장면의 표시 기록이며 이 slice는 PixelCopy 또는 Metal pixel equality를 검사하지 않는다. 실제 기대 배열과 Chrome 관찰이 다르면 구현 전에 계획/fixture를 고친다.

## 구현·검증 관문

1. Stylo `TrackBreadth::Breadth`의 `LengthPercentage::unpack()` 결과를 고정 style fixture에서 확인한다. absolute calc가 computed `LengthPx`로 정규화되어 기존 allowlist를 통과하는지, percentage calc는 남은 `Calc`에서 거부되는지 검증한다.
2. 새 evaluator나 텍스트 재파싱을 추가하지 않는다. 기존 metadata·allocation fail-closed 경계와 `RuntimeBlockFlexGridTrackSizingV1` profile gate를 유지한다.
3. cascade 단위 검증은 columns/rows의 `auto-fill`·`auto-fit` 네 조합에서 absolute calc track이 `LengthPx(40)`으로 정규화되고 fixed `minmax(40px,1fr)` control과 같은 typed track이 되는지 확인한다. percentage calc는 `UnsupportedComputedGridValue`를 반환한다. named-line과 one/two-axis area auto-repeat은 focused regression에 포함한다.
4. 여섯 fixed scene을 app의 opt-in fixture route에 한 번씩 연결한다. 사용 중인 주 checkout에서 별도 feature branch로 진행하며 worktree는 만들지 않는다. Android API 37 ARM64 AVD `emulator-5558`과 iPhone 17 Pro/iOS 26.2 Simulator `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`에서 exact scene ID, root/item frame, axis track, repeat count, `auto-fit` collapse, trace 및 draw/present를 모두 확인한다.
5. 기본/product FFI static archive에서 fixture scene marker·전용 symbol이 빠지고 `c04-runtime-gpu-fixture-test` positive control에 포함되는지 대조한다. 최종 APK/IPA 제외를 확인하지 않으면 그 경계를 PR에 쓴다.
6. 관련 Rust 검증, fixture validator, `cargo fmt --all -- --check`, workspace all-target Clippy와 Android/iOS app build를 실행한다. 실행하지 않은 검사는 통과로 기록하지 않는다. 캐시가 정리된 상태에서 재생성되는 build target 크기와 사용 시간은 보고한다.
7. 계획 수정은 원래 계획 검토 표를 재사용하지 않은 별도 실패 관점 20개로 검토한다. runtime 동작은 계획 검토와 겹치지 않는 새 실패 관점 20개, 구현 PR diff는 별도의 30관점으로 검토한다. 결함을 찾으면 수정 후 영향을 받는 관점을 다시 판정한다.
8. 구현 PR 본문에 oracle·simulator scene, 화면과 hash, static archive 결과, 미검증 범위를 적고 가능한 Android/iOS screenshot을 이미지로 첨부한다. Agent Browser에서 PR 화면 이미지가 실제 표시되는지 확인하고 세션/process를 종료한다.
9. 구현 병합 뒤 요청된 Tailnet roadmap를 선택 파일 hash manifest로 갱신한다. HTTPS 상태·bytes/hash와 Agent Browser 렌더링을 확인한 뒤 Agent Browser를 종료한다. 계획/구현/Tailnet PR은 각각 current required-check와 merge state를 조회해 Rebase and merge한다.

## 계획 자체 적대 검토

[서로 다른 실패 관점 20개](./c11-4-auto-repeat-calc-minimum-plan-review-20.md). 이 계획 검토는 구현/runtime 검토와 PR diff 검토에 포함하지 않는다.

구현 중 관찰한 Stylo 계산식 정규화에 따른 계획 수정도 기존 검토를 재사용하지 않고 [수정 계획 실패 관점 20개](./c11-4-auto-repeat-calc-minimum-revision-review-20.md)로 검토한다.

계획 PR 변경사항은 별도의 서로 다른 실패 관점 30개로 검토하고 [plan-pr-diff-review-30.md](../spec/internal/evidence/c11-4-auto-repeat-calc-minimum-precomparison-2026-10-23/plan-pr-diff-review-30.md)에 기록한다. 구현/runtime 20개와 구현 PR diff 30개는 각각 새로 검토한다.

## 상태·계약 해석

이 제한 slice는 C11.4 또는 C11 전체 완료를 뜻하지 않는다. 미지원 calc 형태는 계속 fail-closed다. 공개 CSS admission과 WPT 적합성 주장을 추가하지 않는다. 내부 계약 numeric version `0.1.0`을 유지한다. 공식 집계는 계획 초안의 `72/213`에서 이후 상태 대장에 기록된 `74/207`로 갱신되어 있으며, 이 slice는 그 수를 바꾸지 않는다.
