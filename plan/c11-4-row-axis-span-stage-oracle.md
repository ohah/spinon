# C11.4 · row-axis overlapping-span stage snapshot test 계획

**상위:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [기존 column-axis span-stage 계획](./c11-4-span-stage-oracle.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태 · 2026-10-27:** 네 고정 row-axis scene의 Chrome 156 최종 geometry와 CSS Grid 2025 CRD §§11.5.1/11.6의 독립 유리수 기대값을 [사전 비교 자료](../spec/internal/evidence/c11-4-row-axis-span-stage-precomparison-2026-10-27/README.md)에 보존했다. test-only Rust 구현은 `Rows` span-2/span-3 단계 trace, fixed Columns 대조군, 네 forward/reversed frame을 검사한다. focused Rust test 3/3, fmt, 대상 crate all-target Clippy를 통과했다. 구현 실패 관점 검토는 19 PASS / 1 PARTIAL이며, [실행 근거](../spec/internal/evidence/c11-4-row-axis-span-stage-implementation-2026-10-27/README.md)에 제한을 기록했다. Chrome은 final geometry reference이고 앱/runtime 또는 simulator 근거가 아니다. C11.4 Stage B 전체, 공식 체크 및 `72/213`은 미완료다.

## 확인한 결손

병합된 [column-axis span-stage oracle](./c11-4-span-stage-oracle.md)는 겹치는 span-2와 span-3 item이 `Columns` pass에서 만드는 중간 growth-limit snapshots를 검사한다. 구현 테스트는 `LayoutGridTrackSizingPassTrace.axis == Columns`인 pass를 선택하며, 해당 slice는 row axis를 명시적으로 제외했다. 하지만 상위 Stage B는 Grid track sizing을 **행과 열 양쪽**에서 실행·검증하도록 요구한다([B단계](./c11-4-grid-track-sizing.md#L139-L142)). Trace pass에는 이미 `Rows`/`Columns` 축이 보존되므로 새 product/runtime API 없이 독립된 row-axis test case를 만들 수 있다([trace axis](../crates/spinon-layout/src/grid.rs#L109-L112), [pass trace](../crates/spinon-layout/src/grid.rs#L202-L208)).

이번 후속은 기존 `Columns` 테스트 결과를 재사용하거나 그 테스트를 row 결과로 해석하지 않는다. 직접 `Rows` pass와 row stage snapshots를 검사한다. CSS Grid의 분배 규칙은 축 방향에 관계없이 같은 track-sizing algorithm을 적용하지만, 이 후속은 네 고정 row input에 한해서만 그 계약을 확인한다.

## 사전 비교 모델

고정 HTML, Chrome 156 관찰, CSS Grid 2025 CRD 기반 독립 기대값과 해시를 [사전 비교 README](../spec/internal/evidence/c11-4-row-axis-span-stage-precomparison-2026-10-27/README.md)에 둔다. Chrome은 최종 used rows와 item frame만 관찰하며 Taffy 내부 stage를 제공하지 않는다. 따라서 Chrome 값은 final geometry control이고, 각 `IntrinsicItemBatch` 뒤의 base/growth-limit 배열은 §11.5.1에서 따로 계산한다.

모든 네 scene은 220×220px definite Grid, 한 개의 fixed 220px column, 네 개의 `minmax(0px, max-content)` row, 0px gap, `align-content:start`를 사용한다. A(80px, rows 1–2)와 B(120px, rows 2–3)는 span-2 group에서 row 2가 겹친다. C(200px, rows 1–3)는 span-3 group에 속한다. DOM source order만 forward/reversed로 바꾸며 placement와 contribution은 유지한다.

| controls | Chrome used rows | 규범적 ideal rows | 남은 block free space |
| --- | --- | --- | ---: |
| A/B forward 및 reversed | `[40,60,60,0]px` | `[40,60,60,0]px` | 60px |
| A/B/C forward 및 reversed | CSSOM `gridTemplateRows`: `[53.3281,73.3281,73.3438,0]px` | `[160/3,220/3,220/3,0]px` | 20px |

Chrome의 `gridTemplateRows` CSSOM 문자열은 소수 넷째 자리까지 직렬화되며, 관측값은 규범적 ideal과 1/64 CSS px 이내다. Rust trace의 finite stage 값은 exact ideal과 `1e-4 CSS px` 이내, 최종 item/root frame은 root-relative Chrome DOMRect와 `1/64 CSS px` 이내로 비교한다.

### Stage 기대값

| trace 지점 | span-2 only growth limits | span-2/3 growth limits | base sizes |
| --- | --- | --- | --- |
| `Initialize` | `[∞,∞,∞,∞]` | `[∞,∞,∞,∞]` | `[0,0,0,0]` |
| span-2 batch | `[40,60,60,∞]` | `[40,60,60,∞]` | `[0,0,0,0]` |
| span-3 batch | 해당 없음 | `[160/3,220/3,220/3,∞]` | `[0,0,0,0]` |
| `IntrinsicSizingComplete` | `[40,60,60,0]` | `[160/3,220/3,220/3,0]` | `[0,0,0,0]` |
| `MaximizeTracks` | `[40,60,60,0]` | `[160/3,220/3,220/3,0]` | 각각 `[40,60,60,0]`, `[160/3,220/3,220/3,0]` |

여기서 `∞`는 기존 trace serializer의 `None`이다. `None`은 non-finite 상태를 나타내며 실제 내부 `+∞`와 NaN/`-∞`를 구분하지 못한다. 새 test도 이 한계를 넘겨 주장하지 않는다. MaximizeTracks의 남은 공간은 60px 또는 20px로 확인한다.

## 구현 범위

- `crates/spinon-layout/src/grid_multispan_chrome_oracle_tests/row_axis.rs`에 별도 test-only child module을 추가하고 기존 `grid_multispan_chrome_oracle_tests.rs`에서 선언한다. Rust fixture-test module 경계 밖에 새 product module이나 FFI를 만들지 않는다.
- 네 case의 `LayoutInput`은 기존 private document-builder와 `TaffyLayoutEngine`을 재사용한다. Root는 width/height 220px, columns `[220px]`, rows `[minmax(0,max-content);4]`다. Items는 width 220px 및 height A/B/C 값, 모두 column 1에 놓고 row placements만 `[1,3]`, `[2,4]`, `[1,4]`로 고정한다.
- 각 root의 trace에서 `Rows` pass가 정확히 하나인지 고른 뒤, 전체 stage sequence `[Initialize, span batches, IntrinsicSizingComplete, MaximizeTracks, ExpandFlexibleTracks, StretchAutoTracks { applied: false }]`, batch sequence `[2]` 또는 `[2,3]`, row count, zero-gap gutter/boundary flags, Minimum/MinContent/MaxContent contributions, 네 row base/growth arrays, maximize 결과를 검사한다. 각 contribution kind의 item·span·height는 입력 값과 같아야 한다; 검증 대상은 batch 종료 상태이며 batch 내부 transient subpass가 아니다. `Columns` control은 fixed 220px early-return snapshot의 실제 CSS track base size가 220px이고 intrinsic batch가 없는지 확인한다.
- Root output frame은 `(x=0,y=0,w=220,h=220)`이어야 한다. Item frame은 root-relative Chrome observation과 대조해 모든 case에서 `x=0,w=220`을 확인한다. `span2-forward`에서 A `(y=0,h=80)`, B `(y=40,h=120)`; span-2/3에서 A `(0,80)`, B `(53.333…,120)`, C `(0,200)`이다. Source order를 뒤집어도 해당 item IDs의 geometry와 row snapshots가 유지되어야 한다.
- 기존 `assert_stage_snapshot` 및 `assert_span_stage_snapshots`의 규범 배열 검증은 두 축에서 공유할 수 있게 `case id`와 expected span batch를 명시적으로 받도록 좁게 일반화한다. 기존 column-axis assertions와 data는 그대로 둔다. Helpers가 책임에 맞지 않으면 row module 안에서 소규모 wrapper를 두고 product source를 옮기지 않는다.
- 다른 stage arithmetic, contribution API, Taffy algorithm, general CSS support를 변경하지 않는다. Android/iOS 앱 경로·simulator·artifact gate는 새 product/runtime route가 없는 pure Rust trace test의 acceptance가 아니며 실행하지 않는다. Chrome precomparison은 규범 입력의 final geometry reference로만 남긴다.

## 합격 기준과 재현

1. `span2-forward`/`span2-reversed`가 같은 `Rows` stage sequence와 snapshots `[40,60,60,0]`을 만든다.
2. `span2-before-span3`/`span3-before-span2`가 같은 span batch 순서 `[2,3]`, 각 중간 snapshot, `[160/3,220/3,220/3,0]` final rows 및 20px free space를 만든다.
3. 모든 row pass에서 track/gutter/collapse shape와 Minimum/MinContent/MaxContent contribution의 item·span·height 값이 고정 기대값과 일치한다. Fixed `Columns` control과 기존 invalid-placement control도 계속 통과한다.
4. 네 case의 item frames는 고정 Chrome geometry와 `1/64 CSS px` 이내다. Chrome의 quantized values를 normative ideal로 사용하거나 Rust 값을 Chrome 결과에서 역산하지 않는다.
5. focused Rust test, `cargo fmt --all -- --check`, 대상 crate Clippy를 실행한다. 새 product/runtime 코드가 없어 simulator/artifact 결과는 주장하지 않는다.
6. 구현 직후 새로운 실패 관점 20개로 test 변경을 hostile review하고, 별도 새 30관점으로 구현 PR diff를 검토한다. 이 row-axis 표는 이전 column-axis review 재사용이 아니다. 발견된 수정은 영향을 받는 관점에서 다시 확인한다.

이 test-only slice는 track-sizing product algorithm, CSS admission, 앱 runtime, FFI를 바꾸지 않는다. 행 축의 네 고정 입력에서 기존 trace를 확인한 결과이며, 상위 Stage B의 나머지 stages/inputs, Stage C/D 및 전체 CSS Grid 적합성을 완료하지 않는다. C11.4 체크와 공식 `72/213`은 미완료다.

## 검토 기록

- 계획 독립 검토 20관점: [plan review](./c11-4-row-axis-span-stage-oracle-plan-review-20.md).
- 계획 PR diff 독립 검토 30관점: [plan PR diff review](./c11-4-row-axis-span-stage-oracle-plan-pr-diff-review-30.md).
- 구현 실패 관점 20개: [implementation review](../spec/internal/evidence/c11-4-row-axis-span-stage-implementation-2026-10-27/implementation-review-20.md), 19 PASS / 1 PARTIAL.
- 구현 PR diff 30관점 검토는 별도 [PR diff review](../spec/internal/evidence/c11-4-row-axis-span-stage-implementation-2026-10-27/pr-diff-review-30.md)에 기록한다.
