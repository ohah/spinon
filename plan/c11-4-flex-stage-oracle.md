# C11.4 · flexible track expansion stage 독립 검증 계획

**상위:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**사전 비교:** [CSS Grid 2025 CRD 유리수 기대값과 Chrome 156 관측](../spec/internal/evidence/c11-4-flex-stage-oracle-precomparison-2026-10-28/README.md) · [기대값 JSON](../spec/internal/evidence/c11-4-flex-stage-oracle-precomparison-2026-10-28/expected.json) · [Agent Browser 화면](../spec/internal/evidence/c11-4-flex-stage-oracle-precomparison-2026-10-28/chrome-flex-stage.png)

**계획 검토:** [실패 관점 20개](./c11-4-flex-stage-oracle-plan-review-20.md) · [계획 PR 변경분 검토 30개](./c11-4-flex-stage-oracle-pr-diff-review-30.md)

**상태 · 2026-10-28:** 계획 PR 준비 중이며 implementation은 시작하지 않았다. 이 계획은 pure Rust test-only `ExpandFlexibleTracks` stage assertion 한 건만 추가한다. CSS Grid production algorithm, runtime, FFI, app, 명세 및 공식 완료 집계는 바꾸지 않는다. C11.4 checkbox와 `72/213`은 미완료로 둔다.

## 확인한 결손

기존 `flex_factor_bound.rs`는 양 축의 `0fr`/`0.5fr` 유효성, auto-repeat ceiling과 과도한 flex-factor 합계 거부를 검사한다. 그러나 독립 CSS Grid oracle에 따라 여러 flexible track 사이 비율을 계산하고, 최소 base size보다 작은 hypothetical size 때문에 track을 동결한 뒤 재분배하는 §11.7 결과를 직접 검사하지 않는다. 기존 `ExpandFlexibleTracks` trace assertions는 flex track이 없는 stretch 입력에서 이전 snapshot과 같은 값을 유지하는지 또는 stage가 존재하는지만 확인한다.

이번 slice는 definite available size에서 다음 네 경우를 직접 구분한다: 1fr:2fr 비율, base minimum으로 인한 freeze와 재계산, 합계가 1보다 작은 factor, 그리고 0fr의 nonzero base floor. 두 case는 Columns, 두 case는 Rows pass를 사용한다. 이는 Stage B의 flexible expansion stage 일부만 확인한다.

## 비교 모델과 독립 계산

기준은 [CSS Grid Layout Module Level 1, 2025 CRD §§11.7–11.7.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-flex-tracks)이다. §11.7은 definite free space에서 §11.7.1의 fr size 계산을 사용한다. §11.7.1은 고정/non-flexible track과 gutter의 base size를 available space에서 빼고 flexible factor 합계를 구하며, 그 합이 1보다 작으면 1로 본다. Hypothetical flexible size가 track base보다 작은 flexible track은 inflexible로 취급해 계산을 반복한다. 기대값은 production Taffy trace가 아니라 [독립 Fraction 산출 스크립트](../spec/internal/evidence/c11-4-flex-stage-oracle-precomparison-2026-10-28/derive_expected.py)에서 이 규칙으로 만든다.

| Case | 축 / available | tracks와 gap | 독립 §11.7 결과 | 남은 공간 |
| --- | --- | --- | --- | ---: |
| `weighted-columns` | Columns / 200px | `minmax(0,1fr) minmax(0,2fr)`, 10px gap | `[190/3,380/3]px` | 0px |
| `frozen-row` | Rows / 200px | `minmax(80,1fr) minmax(0,2fr)`, 10px gap | `[80,110]px`; 1fr track을 먼저 freeze한 뒤 2fr을 다시 계산 | 0px |
| `subunit-columns` | Columns / 200px | `minmax(0,0.5fr) 20px`, 10px gap | `[85,20]px`; 0.5 factor는 합산 시 1로 floor | 85px |
| `zero-fr-row` | Rows / 200px | `minmax(24,0fr) minmax(0,1fr)`, 10px gap | `[24,166]px`; 0fr track의 24px base를 freeze | 0px |

모든 root는 definite `200px` sizing axis와 반대 축 `120px`, viewport와 일치하는 definite size, zero border/padding을 사용한다. 각 두-track axis의 10px gap은 한 번만 뺀다. 상세 입력, 반복에서 사용한 fr fraction과 예상 배열은 `expected.json`에 있다.

같은 HTML fixture의 pinned Agent Browser `0.31.1` / HeadlessChrome `156.0.0.0`, viewport `1280×633 CSS px`, DPR 1 관측은 `[63.328125,126.671875]`, `[80,110]`, `[85,20]`, `[24,166]`이다. 첫 case만 Chrome의 1/64 CSS px track quantization을 보인다. CSSOM/DOMRect는 browser control이며 규범적 분수 기대값을 대신하지 않는다. Chrome output에서 독립 기대값을 역산하지 않았다.

사전 판정 기준은 Rust의 stage CSS track `base_size`가 각 유리수 기대값과 절대 차이 `≤1e-4 CSS px`, Chrome final used-track DOMRect 값은 독립 기대값과 `≤1/64 CSS px`이다. 각 case는 `GridPlacementMode::TrackSizing`, 한 번의 대상 axis pass와 고유 `ExpandFlexibleTracks` stage를 가져야 한다. 2개 CSS track과 internal gutter 1개 및 두 boundary sentinel을 분리해 검사한다. Internal `growth_limit`, `infinitely_growable`, 선택된 flex fraction trace 값은 이 test에서 CSS 기대값으로 만들지 않는다.

## 구현 범위와 대조군

- 유일한 Rust 변경은 `crates/spinon-layout/src/grid_track_sizing_tests/flex_factor_bound.rs`의 test-only case/input helper와 assertion이다. 기대값 source는 checked-in `expected.json`이며 test가 `include_str!`로 이를 읽어 네 case ID의 정확한 집합과 유일성, axis, target/opposite available size, target-axis gap, track descriptor와 rational base sizes를 실제 typed input 및 trace와 비교한다. Gap은 지정 축에만 설정하고 반대 축 gap은 0으로 둔다. 정수와 `numerator/denominator` 형식만 읽고 denominator가 양수인지, 결과가 finite `f32`인지 확인한 뒤 tolerance를 적용한다. test-only `serde_json` dependency가 이미 쓰이는 경로를 재사용하며 Cargo manifest와 production source를 바꾸지 않는다.
- Case는 track factor가 `Fraction`이고 fixed minimum은 `LengthPx`인 `GridTrackSize::MinMax`로 만든다. 각 root/viewport의 width·height를 일치시켜 대상 axis에 indefinite input이 섞이지 않게 한다. `grid_input`에 item을 만들지 않아 contribution/span 동작을 섞지 않는다.
- 각 `LayoutOutput`은 root trace 하나, 해당 axis pass 정확히 하나, `MaximizeTracks` 다음·`StretchAutoTracks` 전의 고유 `ExpandFlexibleTracks` stage를 가져야 한다. CSS track 수, finite base size, 각 track·gap·boundary gutter 순서와 collapsed sentinel을 확인한다. CSS track 선택은 vector 위치나 `internal_index`를 고정하지 않고 trace flags로 한다.
- 기존 `zero_and_sub_one_flex_factors_remain_valid_on_both_axes`, overflow-without-repeat 및 auto-repeat-ceiling controls는 유지한다. 새 test는 finite 0.5/0 factor가 수용되어야 하는 입력만 추가한다. 거부 path나 부분 `LayoutOutput` semantics는 바꾸지 않는다.
- 이 입력은 definite free space, 두 CSS track, 10px internal gap, 빈 grid만 다룬다. Indefinite free space, min-content constraint, spanned item max-content contributions, flex base의 intrinsic content, percentage/calc gap, auto-repeat, CSS admission, WPT, app runtime, Android/iOS simulator와 pixel equality는 제외한다.

## 계획·구현·PR 검토 게이트

1. 계획 자체를 서로 다른 실패 관점 20개로 검토하고, 계획 PR diff를 별도의 30관점으로 검토한 뒤 계획을 Rebase and merge한다.
2. 구현은 위 한 Rust test-only 파일에만 추가한다. `cargo test --locked -p spinon-layout flexible_track_expansion_stage`, `cargo fmt --all -- --check`, `cargo clippy --locked -p spinon-layout --all-targets -- -D warnings`, `git diff --check`를 실행하고 결과를 보존한다. `derive_expected.py`를 다시 실행해 `expected.json`이 바뀌지 않는지 확인한다.
3. 구현 후 새 관점의 test/runtime 경계 검토 20개와 별도 PR diff 검토 30개를 수행한다. 결함 수정 뒤 영향을 받은 관점을 다시 본다.
4. 이 slice는 test-only 확인이다. Simulator/app/runtime 검증이 없음을 명시하고 C11.4 단계 전체나 지원 기능으로 확대하지 않는다. `spec/STATUS.md` C11.4 checkbox와 공식 `72/213`은 그대로 둔다.

## 완료 뒤에도 남는 범위

이 oracle은 고정된 네 definite-empty-grid flex input의 §11.7 snapshot만 확인한다. Grid의 다른 tracks, gap/placement, items와 spans, auto min/max, indefinite/min-content sizing, auto-repeat·named line 결합, 전체 WPT/Stage B와 C11.4의 완료를 검증하지 않는다.
