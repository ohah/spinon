# C11.4 · max-content 축의 cyclic percentage와 auto-repeat 단계 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**선행 slice:** [cyclic percentage `minmax()`](./c11-4-cyclic-percent-minmax.md) · [definite percentage auto-repeat neighbor](./c11-4-auto-repeat-percentage-neighbors.md) · [signed percentage auto-repeat neighbor](./c11-4-auto-repeat-negative-percentage-neighbors.md)

**사전 비교:** [규범 기대·Agent Browser 관측·화면](../spec/internal/evidence/c11-4-cyclic-percentage-auto-repeat-precomparison-2026-10-25/README.md)

**상태 · 2026-10-25:** 이 계획의 fixed cyclic target/control은 제한 runtime slice로 구현해 구현 PR [#623](https://github.com/ohah/spinon/pull/623)에 병합했다(merge SHA `56daefe96e8143959bc99a89c7a8c80e7f1cc4ba`, source SHA `c5b8ef75804e39d36a37aa61b82b1f028115fefd`). Android API 37 16 KB AVD와 iPhone 17/iOS 26.2 Simulator에서 target/control frame, phase/count/basis/track trace와 PixelCopy/Metal sample이 모두 통과했다(AVD software renderer, iOS Metal drawable은 compositor 제출 전 표본). [구현·simulator·artifact 근거](../spec/internal/evidence/c11-4-cyclic-percentage-auto-repeat-runtime-2026-10-25/README.md) · [implementation/runtime review 20개](../spec/internal/evidence/c11-4-cyclic-percentage-auto-repeat-runtime-2026-10-25/implementation-runtime-review-20.md) · [PR diff review 20개](../spec/internal/evidence/c11-4-cyclic-percentage-auto-repeat-runtime-2026-10-25/pr-diff-review-20.md). 구현 범위는 exact two-scene opt-in fixture feature이며 일반 CSS admission, release APK/IPA fixture exclusion 또는 C11.4 전체 완료를 뜻하지 않는다. C11.4 checkbox와 공식 `72/213`은 미완료로 유지한다.

## 목적과 범위

계획 작성 당시 제한 profile에는 cyclic percentage track sizing과 definite-axis auto-repeat이 각각 별도 fixed-scene slice로 존재했지만, intrinsic max-content sizing에서 auto-repeat count를 구한 뒤 percentage track을 최종 layout하는 조합은 고정 runtime scene이 없었다. 병합된 implementation은 columns 축에서 다음 두 pass를 하나의 allowlisted scene으로 확인한다.

1. intrinsic `max-content` sizing에서 `max-content` width는 indefinite다. cyclic percentage breadth는 intrinsic sizing 계산에서 `auto`처럼 처리되고 auto-repeat은 한 번 확장된다.
2. 이 pass가 구한 `24px` content width로 grid를 최종 layout한다. 이때 width는 definite basis이고 `50%` breadth는 `12px`이다. Auto-repeat count를 이 pass에서 다시 계산하면 반복 수 2, used columns `[12px,12px]`가 된다.

한 definite `40px` control은 최종 definite pass의 auto-repeat count 기준을 따로 고정한다. 이 제한 후보는 `auto-fill`, 하나의 column axis, 한 track repeat pattern, 한 fixed-size item만 다룬다. 일반 author CSS admission과 C11.4 완료 상태를 넓히지 않는다.

## 기준과 독립 기대값

기준은 [CSS Grid Layout Module Level 1, 2025 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes), [§7.2.3.1–§7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat), [§11.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-grid-sizing), [§11.2·§11.5·§11.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)다. `max-content`가 indefinite size라는 용어 정의는 [CSS Sizing Level 3 §2](https://www.w3.org/TR/css-sizing-3/#definite)를 함께 참조한다. Chrome 관측은 별도 자료로 보존하며 규범 기대값을 만드는 데 사용하지 않는다.

`minmax(0px,50%)`은 `<fixed-size>` 문법의 `minmax(<fixed-breadth>,<track-breadth>)` 형태다. 0px minimum은 definite fixed breadth이고 50% maximum은 definite container size에서 풀리는 percentage breadth다. 이 auto-repeat pattern은 CSS grammar상 허용된다.

| Scene / sizing pass | Repeat count | Columns | Root content box | Item frame |
|---|---:|---|---|---|
| `cyclic-max-content-auto-fill`, intrinsic max-content pass | 1 | cyclic maximum은 intrinsic 계산 중 `auto`처럼 취급; resulting max-content width `24px` | pass가 정하는 inline size `24px` | `24×16px` |
| `cyclic-max-content-auto-fill`, final definite pass | 2 | `[12,12]px` | `24×16px` | `(0,0) 24×16px` |
| `definite-40px-auto-fill-control`, final pass | 2 | `[20,20]px` | `40×16px` | `(0,0) 24×16px` |

### 규범 계산

- CSS Sizing Level 3는 intrinsic keyword `max-content`를 indefinite size로 정의한다. 그러므로 첫 intrinsic pass에서 CSS Grid §7.2.3.2의 definite preferred/max size 및 definite minimum size branch는 선택되지 않고, `<auto-repeat>` track list를 한 번 반복한다.
- §7.2.1에 따라 cyclic percentage maximum은 intrinsic sizing 동안 `auto`로 취급되므로 single repeated track은 `minmax(0px,auto)`로 sizing된다. §11.2의 초기화에서 fixed `0px` minimum은 track base size를 0으로 두고, intrinsic `auto` maximum은 item contribution으로 정해질 growth limit을 갖는다. 유일한 item의 preferred width가 `24px`이고 available inline size에 의존하지 않으므로 그 item's max-content contribution도 `24px`이다. §11.5·§11.6의 max-content track sizing은 이 single-span contribution을 growth limit에 반영한 뒤 max-content constraint에서 유일한 track을 base size 0px부터 24px까지 maximize한다. Gutter와 다른 track이 없어 intrinsic grid content width는 정확히 `24px`이다.
- §11.1에 따라 indefinite grid는 min/max-content constraint로 sizing된 뒤 그 결과 크기로 실제 layout을 한다. 최종 pass의 `24px` size는 definite이므로 auto-fill은 definite preferred size branch를 따른다. `50%` maximum은 track당 `12px`; 두 번 반복하면 24px content box에 맞고 세 번이면 넘친다. 최종 used columns는 `[12px,12px]`다.
- definite control의 content width는 `40px`다. Repeat-count sizing maximum은 track당 `20px`이므로 두 번 반복하면 딱 맞고 세 번은 넘친다. Final columns는 `[20px,20px]`다.
- 모든 scene은 gap·margin·padding·border 0이다. Item의 minimum sizing function이 `0px`이므로 item automatic minimum이 track minimum을 바꾸지 않는다. Fixed item width 24px는 첫 repeat track보다 12px 또는 4px 넓을 수 있다.

위 독립 계산과 Chrome 156 관측을 구분한 machine-readable 자료, 원본 입력·화면·hash는 [사전 비교 evidence](../spec/internal/evidence/c11-4-cyclic-percentage-auto-repeat-precomparison-2026-10-25/README.md)에 있다. Chrome 156 CSSOM은 max-content target에서 `12px 12px`, control에서 `20px 20px`를 보였다. 이 final CSSOM은 intrinsic pass 최초 count를 보여주지 않으므로 그 단계 경계는 Spinon runtime trace에서 직접 확인한다.

## 구현 진입 관문과 결과: pass trace feasibility

계획 당시 trace는 일부 단계 관측에 재사용할 수 있었지만 요구된 pass provenance를 표현하지 않았다. `crates/spinon-layout/src/grid.rs`의 `LayoutGridTrackSizingPassTrace`는 `sequence`, `axis`, `stages`, `contributions`를 보유하고, `vendor/taffy/src/compute/grid/track_sizing.rs`는 축별 track-sizing 실행을 순서대로 기록했다. `crates/spinon-layout/src/taffy_style.rs`는 `TrackSizing` profile의 non-flow pass에서 이를 켰으며, `RuntimeGridTrackSizingDiagnostics`는 `c11-4-grid-track-sizing-fixture-test` feature에서 보존됐다. 구현은 이 opt-in 경계에 semantic phase, auto-repeat count, percentage basis를 실제 Taffy sizing callsite에서 capture하도록 연결했다. 상세 실행은 [runtime evidence](../spec/internal/evidence/c11-4-cyclic-percentage-auto-repeat-runtime-2026-10-25/README.md)에 있다.

계획 acceptance의 첫 관문은 target scene을 runtime style/layout 경로에 넣어 `LayoutOutput`과 완료 revision에 묶인 `RuntimeGridTrackSizingDiagnostics`에서 두 sizing phase를 구분하는 것이었다. 기존 field로 phase/count/basis를 증명할 수 없어 `c11-4-grid-track-sizing-fixture-test` 전용 hook을 actual Taffy sizing callsite에 추가했다. `intrinsic-max-content`/`final-definite` phase, 해당 pass의 auto-repeat count와 percentage basis가 runtime diagnostics 및 FFI report까지 보존되며, feature 없는 기본 artifact에는 이 trace ABI·payload가 들어가지 않는다. acceptance 결과는 [runtime evidence](../spec/internal/evidence/c11-4-cyclic-percentage-auto-repeat-runtime-2026-10-25/README.md)에 있다.

첫 pass의 count 1/basis 없음과 final pass의 count 2/basis 24px, 그리고 definite 40px control의 count 2/basis 40px를 재구성 값이 아닌 실제 trace에서 확인했다. sequence 순서를 phase label로 추정하지 않고 semantic phase를 직접 기록한다. 이 gate는 통과했다.

## 고정 입력과 acceptance

- Runtime scene `cyclic-max-content-auto-fill`: root `width:max-content; grid-template-columns:repeat(auto-fill,minmax(0px,50%)); grid-template-rows:auto`; runtime document에는 in-flow direct child 하나(`24×16px`)만 둔다. Root의 authored min/max constraints는 생략하고, exact validator는 computed `min-width`/`max-width`가 definite repeat-count branch를 선택하지 않는 초기값인지 검사한다.
- Runtime scene `definite-40px-auto-fill-control`: 같은 columns pattern과 item을 쓰고 root width만 definite `40px`로 고정한다. Control도 min/max width를 별도로 author하지 않으며 target과 같은 초기값만 허용한다.
- Browser 사전비교 HTML에는 각 grid 아래 화면 경계 표시용 `position:absolute` `.boundary` span이 하나 있다. 이것은 screenshot 관측 보조이며 Grid §6에 따라 out-of-flow라 sizing item이 아니다. Browser IDs `cyclic`/`definite`는 runtime IDs `cyclic-max-content-auto-fill`/`definite-40px-auto-fill-control`에 매핑된다. `.boundary`는 runtime HTML/scene, `nodeIds`, FFI payload 및 runtime child validator에 넣지 않으며 runtime fixture의 grid item은 오직 한 개다.
- Scene별로 document/style/environment revision, computed track AST, intrinsic-pass/final-pass identity, auto-repeat count, percentage basis, expanded used columns, row tracks, root/item frames와 Columns→Rows trace를 대조한다.
- 첫 구현 관문은 통과했다. Runtime API가 기존 trace만으로는 요구값을 증명할 수 없어 위의 fixture-test opt-in 계측을 추가했으며, 의미가 부여된 두 pass, count와 basis를 기록했다.
- Target에서 첫 pass count 1, max-content content width 24px, 최종 definite basis 24px, 최종 pass count 2 및 columns `[12,12]px`가 각각 관찰되어야 한다. Final-pass count를 보이지 않는 하나의 최종 CSSOM 값이나 scene count만으로 대체하지 않는다.
- Definite control은 count 2, columns `[20,20]px`, root 40×16px, item 24×16px여야 한다. Track list order는 정확히 일치하고 geometry는 1/64 CSS px 안에 있어야 한다.
- 계획 당시 acceptance는 기존 경로가 두 pass를 이미 일치시키면 fixture/assertion만 추가하고, 재계산 결함이 관찰될 때만 알고리즘을 바꾸는 것이었다. Target에서 intrinsic pass의 1-track 24px 결과가 final definite pass에 재사용되어야 했고, count가 다시 계산되지 않아 Taffy re-expansion을 exact fixture feature에 한정해 추가했다.

## 오류와 fail-closed 경계

- Fixture adapter는 정확한 root/child identity·순서·style·revision 및 authored/computed min/max width와 두 고정 scene만 허용한다. Target의 min/max width가 definite하지 않은지 검사해 한 번 반복 규칙의 전제를 고정한다. Runtime child tree에는 direct in-flow item 하나만 허용하며 browser-only `.boundary`를 runtime allowlist에 예외로 추가하지 않는다.

### FFI scene selector negative control

| Negative control | 정확한 변형 | Error owner와 기대 typed result |
|---|---|---|
| `unknown-scene` | `c11_4_grid_track_sizing_runtime_source`에 `cyclic-max-content-auto-fill-typo` 전달 | FFI dispatcher `Result<Cow<'static, str>, String>`가 `Err("알 수 없는 C11.4 scene ID: cyclic-max-content-auto-fill-typo")` 반환. Runtime layout은 호출되지 않고 scene/render 결과가 없다. |

### Runtime fixture input mutations

아래 네 입력 변형은 cyclic runtime fixture adapter 소유다. 각 adapter 호출은 `Err(RuntimeLayoutFailure { code: "unsupported_c114_cyclic_percentage_auto_repeat_fixture", node_id: Some(<offending node>), property: Some("<scene-id>: <reason>") })`로 끝나며 `RuntimeLayoutCompleted`, frame/trace snapshot, render scene 및 draw/present 결과를 내지 않는다.

| Negative control | 정확한 변형 | 기대 node 및 `property` 사유 |
|---|---|---|
| `extra-inflow-child` | target 아래 `1×1px` in-flow direct child `unexpected-item`을 추가 | root node; `exactly one in-flow child required` |
| `browser-marker-in-runtime` | 사전비교 HTML의 `.boundary` span을 runtime scene/node list에 포함 | marker node; `browser-only out-of-flow marker is not a runtime scene node` |
| `mutated-gap` | target에 `column-gap:1px`을 설정 | root node; 해당 scene ID와 `column-gap` |
| `mutated-repeat-mode` | target의 `auto-fill`을 `auto-fit`으로 변경 | root node; 해당 scene ID와 `grid-template-columns` |

### Layout 완료 뒤 trace/report controls

이 항목들은 정상 geometry layout 뒤 diagnostics/report 경계를 검사하므로 `RuntimeLayoutFailure`가 아니다.

| Negative control | 정확한 변형 | Error owner와 기대 typed result |
|---|---|---|
| `trace-missing` | 정상 completed layout의 diagnostics snapshot을 제거 | FFI `grid_track_sizing_trace_report`의 `Result<String, String>`가 `Err("C11.4 layout diagnostic snapshot이 없습니다")` 반환; 성공 report를 만들지 않는다. |
| `trace-revision-mismatch` | diagnostics source/style/environment revision을 completed runtime key와 불일치시킴 | 같은 FFI report owner가 `Err("C11.4 trace revision이 완료된 runtime key와 다릅니다")` 반환; stale report를 내지 않는다. |
| `trace-phase-ambiguous` | semantic phase가 없는 `sequence`/`axis` trace만 fixture acceptance에 전달 | C11.4 fixture trace gate가 추가할 test-only `CyclicAutoRepeatTraceGateError::MissingSemanticPhase { node_id }`로 실패한다. Layout completion이 존재할 수 있지만 fixture는 PASS, 성공 report 또는 draw/present evidence로 승격되지 않는다. |

- 계산 pass가 관찰되지 않았거나 pass별 count가 뒤섞인 trace는 성공으로 집계하지 않는다. Intrinsic pass와 final pass에서 count가 달라야 하는 target은 각각의 pass를 식별하지 못하면 acceptance fail이다.
- Nonfinite percentage, aggregate/f32 overflow, axis/basis misbinding, projection/calc identity mismatch는 기존 `LayoutError` 계열의 구체적인 node/property 오류로 끝내고 부분 `LayoutOutput`이나 stale scene을 반환하지 않는다.
- Auto-fit collapse, cyclic row axis, definite/indefinite percentage neighbors, percentage minimum, calc, multiple auto-repeat, named line/area, implicit placement, spans, nonzero gap, arbitrary CSS/HTML admission, WPT 전체 및 전체 C11.4는 이 slice에 포함하지 않는다.

## 후보 코드 소유 영역

- **FFI scene input 및 shared order:** 새 `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/cyclic_percentage_auto_repeat.rs`와 대응 JSON은 이 slice가 소유하고 `c04-runtime-gpu-fixture-test`로만 연다. Cyclic slice가 공용 `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs` dispatcher와 `trace_report.rs`의 cyclic wiring/phase-count-basis report를 먼저 통합하고 마친다. 그 뒤 orthogonal-row sibling이 shared base에 rebase한 다음 자기 isolated scene module과 dispatcher/report 추가를 순차 반영한다. 이 공용 파일들은 두 slice가 동시에 편집하지 않는다.
- **Style/layout projection:** 기존 typed auto-repeat과 cyclic percentage 경로를 재사용한다. 먼저 `crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs`, `crates/spinon-layout/src/taffy_style/grid.rs`, `crates/spinon-layout/src/grid_track_sizing_tests.rs`를 추적한다. Current code가 pass 사이 auto-repeat을 재계산하지 않는 경우에만 `vendor/taffy/src/compute/grid/track_sizing.rs` 또는 연결된 Taffy grid sizing source를 바꿀 근거를 낸다.
- **Runtime fixture adapter:** 새 `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_cyclic_percentage_auto_repeat.rs`는 exact input validator와 phase-tagged count/basis assertions를 소유한다. 기존 `c11_4_mixed` 및 orthogonal sibling의 scene module은 편집하지 않는다. 필요한 계측은 trace feasibility gate에서 필요성이 확인된 경우에만 opt-in `c11-4-grid-track-sizing-fixture-test`에 둔다.
- **공용 host integration:** Android `C0410RuntimeGpuDemo.java`와 iOS `C0410RuntimeGpuDemo.swift`의 cyclic scene 목록·expectation·capture를 cyclic slice가 먼저 통합하고 검증한다. 그 변경이 완료되면 orthogonal-row sibling이 shared base에 rebase하고 자기 scene 및 expectation을 두 adapter에 순차 추가한다. 같은 파일의 동시 편집은 금지한다. Slice별 capture/evidence 하위 폴더는 각각 분리한다.

## 검증 관문

1. 계획 변경 전 실패 관점이 서로 다른 20개 질문을 표로 답하고 독립 CSS Grid 계산과 browser observation을 구분한다.
2. CSS fixture의 SHA-256, `CSS.supports()` 문법 확인, Chrome raw CSSOM/DOMRect/UA, viewport, DPR과 screenshot을 보존한다. Browser는 oracle 관찰자이며 Spinon 결과가 아니다.
3. 첫 runtime gate에서 pass trace feasibility를 조사한다. 통과 후 focused Rust tests는 semantic intrinsic/final phase, repeat counts 1→2, percentage basis, used tracks, frames와 표의 8 owner별 fail-closed controls를 확인한다. 기존 definite auto-repeat와 cyclic minmax controls를 회귀 실행한다. Gate가 실패하면 후속 검증은 실행하지 않고 blocker를 기록한다.
4. Workspace test/Clippy/rustfmt 및 fixture-module boundary를 실행하고, default/product artifact negative control과 opt-in positive control에서 scene ID·fixture symbol·payload 경계를 확인한다. Archive 증거를 APK/IPA 전체 제외로 확대하지 않는다.
5. Android API 37 ARM64 16 KB AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 두 scene을 실행한다. 각 플랫폼별 root/item frames, final tracks, pass별 count trace, draw/present, capture/source/build/install hash를 따로 기록한다. AVD software rendering, iOS compositor 전 drawable 범위만 주장한다. Screenshot은 배치 기록이며 별도 PixelCopy/Metal sample gate가 없으면 pixel equality를 말하지 않는다.
6. Implementation/runtime에는 새 실패 관점 20개, 실제 PR diff에는 별도의 독립 실패 관점 20개를 적용한다. 수정된 revision에 영향받은 관점을 다시 확인한다. PR은 이 제한 scene, 근거와 남은 범위를 적고 현재 required checks/merge 상태를 새로 확인한다. C11.4 checkbox와 공식 `72/213`은 유지한다.

## 계획 실패 관점 20개 질문

1. `minmax(0px,50%)`가 `<auto-repeat>` `<fixed-size>` 문법을 만족하는가?
2. `max-content`를 definite preferred size로 잘못 분류하지 않고 intrinsic pass가 indefinite임을 확인하는가?
3. auto-repeat count 계산 시점과 CSS Grid §11.1 intrinsic/final pass 시점을 구별하는가?
4. Intrinsic pass의 `% → auto` 처리를 final percentage resolution으로 혼동하지 않는가?
5. 첫 pass에서 한 번만 반복되는 근거와 최종 pass에서 재확장되는 근거를 각각 규범 절에 연결하는가?
6. Max-content root width 24px을 Chrome 값 역산이 아니라 fixed item contribution에서 도출하는가?
7. Final percentage basis를 border-box/root frame이 아니라 24px content box로 고정하는가?
8. Final pass repeat maximum이 두 번 12px이고 세 번이면 overflow인 산술을 독립 확인하는가?
9. Definite 40px control에서 percentage maximum이 20px이고 count가 2인 이유를 별도로 계산하는가?
10. Fixed 0px track minimum 때문에 Grid item automatic minimum이 track base를 키우지 않는가?
11. Row `auto`의 16px 결과가 columns cyclic sizing을 바꾸지 않는가?
12. Zero gap, margin, border와 padding이 count 및 percentage basis에 반영됐는가?
13. In-flow child는 정확히 하나이며 임의의 implicit track/auto-placement를 추가하지 않는가?
14. HTML의 absolute marker는 out-of-flow 관측 보조일 뿐 repeat count/track sizing item이 아님을 보장하는가?
15. Chrome CSSOM의 final `[12,12]`만으로 intrinsic pass count 1을 증명했다고 오해하지 않는가?
16. Browser observation과 spec-derived expectation이 저장 파일과 README에서 분리되어 있는가?
17. Computed track list order, root/item frame과 두 pass의 provenance를 모두 acceptance에 포함하는가?
18. Unsupported scene/style, basis mismatch와 nonfinite/overflow에서 부분 결과나 stale trace를 거부하는가?
19. Current trace의 `sequence/axis/stages/contributions`와 missing phase/count/basis를 구분하고, opt-in 계측으로 증명할 수 없으면 첫 관문에서 중단하는가?
20. 이 fixed slice를 일반 CSS support, 전체 WPT 적합성 또는 C11.4 완료로 승격하지 않는가?

## 구현 전 사전 비교

[Agent Browser 사전 비교](../spec/internal/evidence/c11-4-cyclic-percentage-auto-repeat-precomparison-2026-10-25/README.md)는 규범 입력과 Chrome 156 observation을 보존한다. 이 단계에서는 계획과 evidence만 작성했으며 runtime implementation, `spec/STATUS.md`, parent plan, public roadmap은 변경하지 않는다.
