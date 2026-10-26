# C11.4 · overlapping span group 중간 track snapshot 검증 계획

**상위:** [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태 대장 C11.4](../spec/STATUS.md#L273)

**상태 · 2026-10-26:** 기존 고정 Chrome 155 overlap case에 test-only span-stage assertion을 구현했다. 이 branch에서 focused Rust tests, fmt, Clippy와 expected JSON 재생성이 통과했고, 구현/runtime 독립 review의 최종 유효 결과는 20/20 PASS다. 별도 실행 결과는 [implementation evidence](../spec/internal/evidence/c11-4-span-stage-oracle-implementation-2026-10-26/README.md)에 기록한다. 이 검증은 pure Rust trace test만 다루며 C11.4 CSS/runtime/app 동작을 추가하지 않는다. 상위 Stage B, C11.4 checkbox와 공식 `72/213`은 미완료로 유지한다.

## 확인한 결손

상위 계획 Stage B는 §11.3–§11.8의 initialization, contribution/span distribution, maximize/flex/stretch 단계를 각각 기록해 맞는지 확인하도록 요구한다([상위 Stage B](./c11-4-grid-track-sizing.md#L108)). 현재 [`grid_multispan_chrome_oracle_tests.rs`](../crates/spinon-layout/src/grid_multispan_chrome_oracle_tests.rs#L150)는 Chrome final columns, item frames, contribution query 및 batch order를 검사하지만, 각 `IntrinsicItemBatch` 뒤 track snapshot을 CSS 규범 기대값과 대조하지 않는다. Trace model은 batch stage마다 track index, base size, growth limit, gutter/collapse 및 internal `infinitely_growable` flag를 보존한다([trace model](../crates/spinon-layout/src/grid.rs#L159)).

이 slice는 CSS 규범에서 도출할 수 있는 track order, base size, growth limit 및 span-group sequence만 assertion한다. `infinitely_growable`은 trace API의 내부 상태이며 CSS Grid 규범이 직접 정의한 값이 아니므로 이번 independent oracle에서 기대값을 만들지 않는다.

## 비교 모델과 독립 산술

규범 출처는 [CSS Grid Layout Module Level 1, 2025 CRD §11.5.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#extra-space) 및 [§11.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-grow-tracks)이다. [사전 비교 근거](../spec/internal/evidence/c11-4-span-stage-oracle-precomparison-2026-10-26/README.md)는 기존 네 input의 planned-increase 산술을 기록한다. [`derive_expected.py`](../spec/internal/evidence/c11-4-span-stage-oracle-precomparison-2026-10-26/derive_expected.py)는 유리수 산술로 [`expected.json`](../spec/internal/evidence/c11-4-span-stage-oracle-precomparison-2026-10-26/expected.json)을 만든다.

입력은 네 `minmax(0px,max-content)` column, 0px gap, definite 220px content width다. Span-2 items A(80px, tracks 1–2)와 B(120px, tracks 2–3)가 한 track에서 겹치고, span-3 item C(200px)는 tracks 1–3을 덮는다. 모든 track의 initial base size는 0px, initial growth limit은 ∞다. §11.5.1은 같은 span-2 group의 per-item planned increase를 해당 track별 최댓값으로 반영해 `[40,60,60,∞]`를 만든다. C는 앞 세 growth limit 합 160px에서 필요한 40px을 세 track에 `40/3`씩 분배해 `[160/3,220/3,220/3,∞]`를 만든다. §11.5 step 5는 span 밖의 네 번째 track growth limit ∞를 base 0으로 끝내고, §11.6은 남은 definite free space를 finite growth limits까지 분배한다.

| trace 지점 | base sizes (px) | growth limits (px; `None`은 ∞) |
| --- | --- | --- |
| `Initialize` | `[0,0,0,0]` | `[∞,∞,∞,∞]` |
| span-2 batch 뒤 | `[0,0,0,0]` | `[40,60,60,∞]` |
| span-3 batch 뒤 | `[0,0,0,0]` | `[160/3,220/3,220/3,∞]` |
| `IntrinsicSizingComplete` 뒤 | `[0,0,0,0]` | `[160/3,220/3,220/3,0]` |
| `MaximizeTracks` 뒤 | `[160/3,220/3,220/3,0]` | `[160/3,220/3,220/3,0]` |

Span-2-only cases는 `Initialize`, span-2 이후 `[40,60,60,∞]`, `IntrinsicSizingComplete` 뒤 `[40,60,60,0]`, `MaximizeTracks` 뒤 base/growth `[40,60,60,0]` 및 남은 free space `60px`를 모두 검사한다. Span-2/3 cases는 위 두 batch snapshots 및 final ideal `[160/3,220/3,220/3,0]`과 남은 free space `20px`를 검사한다. 해당 fixture의 source order controls는 `[A,B]`, `[B,A]`, `[A,B,C]`, `[C,B,A]`이며, 모든 trace에서 span batch order는 `[2]` 또는 `[2,3]`이어야 한다. 이 oracle은 fixed/finite upper limits, fit-content, fr, gap, auto minimum, row axis, text/image contribution 및 arbitrary placement를 일반화하지 않는다.

## Chrome final oracle의 별도 역할

기존 [Chrome 155 multi-span observation](../spec/internal/evidence/c11-4-multispan-ordering-physical-android-2026-10-21/chrome155-observation.json)과 [입력·규범 산술·판정 기록](../spec/internal/evidence/c11-4-multispan-ordering-physical-android-2026-10-21/README.md)은 최종 used columns와 item frames를 확인한다. Span-2-only controls의 Chrome columns는 `[40,60,60,0]px`; span-2/3 cases의 1/64px quantized Chrome columns는 `[53.328125,73.328125,73.34375,0]px`다. Span-2/3의 independent normative ideal은 `[53⅓,73⅓,73⅓,0]px`이며 Chrome quantization과 분리한다. Chrome은 Taffy 내부 `IntrinsicItemBatch` snapshot을 노출하지 않으므로 final geometry만 기존 assertion으로 검증하고 중간 기대값은 §11.5.1/§11.6에서만 계산한다.

## 구현 범위와 controls

- 변경 대상은 기존 [`grid_multispan_chrome_oracle_tests.rs`](../crates/spinon-layout/src/grid_multispan_chrome_oracle_tests.rs)의 test-only assertion이다. 새로운 product/runtime 경로, module declaration 또는 Taffy algorithm 변경은 포함하지 않는다. Helper가 이미 구성한 fixed-size HostDocument nodes, explicit placements, typed track input 및 trace를 재사용한다.
- 각 case는 한 번의 definite `Columns` pass여야 한다. 현재 helper의 첫 pass `.find()`만 보지 말고 pass 수를 확인해 예기치 않은 재실행에서 snapshot을 잘못 고르지 않는다. 입력에 cyclic percentage가 없고 item widths가 definite라 intrinsic inline contribution도 pass 사이에 달라질 이유가 없다. 추가 pass가 관측되면 임의로 선택하지 말고 원인을 계획 검토 관문으로 되돌린다.
- 각 control의 pass에서 `Initialize` base `[0,0,0,0]`와 growth trace `[None,None,None,None]`, 모든 `IntrinsicItemBatch`, `IntrinsicSizingComplete`, `MaximizeTracks` snapshot을 검사한다. Span-2-only와 span-2/3 각각의 final base/growth 및 remaining-free-space를 비교한다. Taffy serializer는 `growth_limit.is_finite().then_some(value)`를 사용하므로 `None`은 CSS 기대의 `+∞`를 위한 trace encoding이지만 실제 `+∞`, `-∞`, NaN을 서로 구분하지 않는다. 따라서 이 test는 `None`으로 직렬화된 unbounded/non-finite state만 확인하며 내부값이 실제 `+∞`였음을 증명한다고 주장하지 않는다. 기대 finite value는 actual `Some(finite)`이고 absolute delta가 `≤1e-4 CSS px`일 때만 통과한다. 이 stage tolerance는 Chrome final 1/64px quantization tolerance와 분리한다. 이 fixture의 Taffy vector는 CSS track N개에 대해 `2N+1` entry가 gutter/track으로 교대한다. [Taffy `initialize_grid_tracks`](../vendor/taffy/src/compute/grid/explicit_grid.rs#L287)는 첫/마지막 grid-line boundary gutter를 collapsed sentinel로 표시한다. auto-fit이 없는 이 fixture에서는 내부 zero-gap gutter와 CSS tracks는 collapsed가 아니므로 모든 stage에서 이 정확한 entry/flag pattern을 확인한다. CSS track 값은 `!is_gutter && !is_collapsed`로 vector order를 유지해 추출하며 raw `internal_index` 0..3을 기대하지 않는다. `is_gutter`/`is_collapsed` flag와 CSS track count도 검사한다. Per-item `MaxContent` query는 A/B/C owner NodeId와 80/120/200px에 연결한다.
- Negative controls: (1) fixed-track-only calculation은 `FixedTrackEarlyReturn`을 보이고 `IntrinsicItemBatch`를 만들지 않는지 확인한다. (2) 같은 column start/end line을 입력하면 `LayoutError::InvalidGridPlacement`로 거부되고 성공 `LayoutOutput`/trace가 없는지 확인한다([placement validation](../crates/spinon-layout/src/grid_placement.rs#L242)). (3) 모든 고정 fixture batch는 `crosses_flexible_track=false`여야 하고 span별 예상 batch 수는 정확히 `[1]` 또는 `[1,1]`이어야 한다. Flex track interaction 및 `true` batch generation은 이번 slice에서 제외하며, true batch가 non-flex selector에 포함되는 구현은 batch count/assertion에서 실패해야 한다. 이 세 control은 새 track function이나 placement 지원을 추가하지 않는다.
- CSS cascade/HTML, C15 text measurement, C14 image intrinsic input, row-axis stage, FFI/app fixture, Android/iOS UI/surface, WPT는 제외한다. 이는 pure Rust layout trace test이며 사용자 화면을 바꾸지 않는다.

## 진행 단계와 검토 관문

1. 계획 review 전에 source, span indices, definite contribution values, CRD planned-increase math, initial/intrinsic/final stage distinction 및 Chrome quantization을 독립 대조한다. 불일치가 있으면 구현 없이 plan/evidence를 고친다.
2. 계획 자체를 별도 실패 관점 20개로 검토한다. 계획 PR diff는 별도 30관점으로 검토한다. 둘 다 끝나기 전 구현을 시작하지 않는다.
3. 계획 승인 뒤 기존 Rust test file에 snapshot assertions와 negative controls만 추가한다. CSS/runtime/product implementation 및 roadmap/status count는 변경하지 않는다.
4. `cargo test --locked -p spinon-layout chrome155_multi_span_contributions_are_independent_of_item_source_order`, focused test filter, `cargo fmt --all -- --check`, `cargo clippy --locked -p spinon-layout --all-targets -- -D warnings`를 실행한다. `python3 spec/internal/evidence/c11-4-span-stage-oracle-precomparison-2026-10-26/derive_expected.py` output은 committed `expected.json`과 일치해야 한다. 앱 실행, browser capture, simulator 또는 pixel test는 test-only slice의 완료 조건이 아니다.
5. 구현/runtime test 변화는 계획 review와 다른 새 실패 관점 20개로 검토한다. 실제 PR diff는 독립된 30관점으로 검토한다. 발견 결함을 수정한 뒤 영향을 받은 관점을 다시 검토한다.
6. 이 test-only 근거만 관련 C11.4 parent plan과 연결한다. Stage B 또는 C11.4 완료, 공개 CSS support, 전체 WPT 적합성, 공식 완료 수 증가를 표시하지 않는다.

## 완료 판정과 잔여 범위

Forward/reversed input에서 initialization, span batch order, batch별 base/growth snapshot, completion 및 span-2/span-2+3 maximize state가 독립 §11.5.1/§11.6 계산에 맞고 existing Chrome quantized final geometry/item frame regressions가 유지되어야 한다. Finite comparator, fixed-track early-return, malformed placement와 non-flex batch-count controls도 통과해야 한다. Focused Rust checks, 새 implementation/runtime review20 및 PR diff review30 이후 이 **overlapping span-stage test slice**만 완료로 보고한다.

이 slice는 다른 max/min track function, multi-span의 모든 경쟁 조합, row axis, flex/automatic-minimum sizing matrix, intrinsic text/image assembly, CSS cascade, runtime output 및 WPT를 닫지 않는다. Row-axis stage snapshots는 별도 [후속 계획](./c11-4-row-axis-span-stage-oracle.md)으로 추적한다. 따라서 C11.4 Stage B와 공식 `72/213`은 미완료로 유지한다.
