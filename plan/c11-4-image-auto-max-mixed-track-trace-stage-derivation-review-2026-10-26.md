# C11.4 image auto/max trace stage derivation review

## 판정

고정된 fixture와 vendored Taffy 0.15.0 소스를 기준으로 독립 산출한 세 stage의 raw vector는 아래와 같다. 최초 draft의 Columns `Initialize` 내부 gutter slot 2 `0/0`은 틀렸고 `5/5`여야 한다. 현재 child plan, expectation JSON, Android/iOS matcher, runtime evidence README는 이 값을 모두 `5/5`로 맞췄다. 기대값 계약은 이 교정본으로 implementation에 전달할 수 있다. 이 검토에서는 test, browser, simulator를 실행하지 않았다.

| Axis | `Initialize` (`base/growth`) | `IntrinsicItemBatch(span=1, crosses_flexible_track=false)` | `IntrinsicSizingComplete` |
| --- | --- | --- | --- |
| Columns | `[0/0, 0/null, 5/5, 17/17, 0/0]` | `[0/0, 24/24, 5/5, 17/17, 0/0]` | `[0/0, 24/24, 5/5, 17/17, 0/0]` |
| Rows | `[0/0, 0/null, 0/0]` | `[0/0, 16/16, 0/0]` | `[0/0, 16/16, 0/0]` |

Columns slot roles are `[leading boundary sentinel, column 0, internal 5px gutter, column 1, trailing boundary sentinel]`. Its `is_gutter` vector is `[true,false,true,false,true]`; `is_collapsed` is `[true,false,false,false,true]`. Rows are `[leading boundary sentinel, row 0, trailing boundary sentinel]`, with `is_gutter=[true,false,true]` and `is_collapsed=[true,false,true]`. Every slot has `infinitely_growable=false`. The sentinels have zero base and growth; the real inter-column gutter at index 2 stays uncollapsed and is 5/5 from initialization onward.

## 독립 유도

The pinned [CSS Grid Layout Module Level 1, 2025 CRD](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/) §§11.3–11.5 treats gutters as fixed-size tracks for sizing, initializes fixed tracks from their resolved minimum/maximum and intrinsic tracks from base zero with an unbounded growth limit, then resolves intrinsic contributions. The fixture sets a definite 100×40px grid, columns `minmax(auto,max-content) 17px`, a 5px column gap, one `auto` row, and one direct 24×16px PNG. The PNG header confirms 24×16; it is also linked by the fixture evidence. The contribution inputs are therefore 24px on the column axis and 16px on the row axis.

Taffy establishes the raw representation and stage boundaries:

- `vendor/taffy/src/compute/grid/explicit_grid.rs:312-317,338-353,444-446` starts with a gutter, appends each explicit track and its following gutter, then collapses the first and last entries. Thus two columns yield five slots and one row yields three.
- `vendor/taffy/src/compute/grid/types/grid_track_counts.rs:27-33` documents the interleaved grid-line/track indexing. `vendor/taffy/src/compute/grid/types/grid_track.rs:101-117` constructs a gutter from the gap sizing function and makes a collapsed boundary sentinel fixed zero.
- `vendor/taffy/src/compute/grid/track_sizing.rs:548-619` resolves each fixed gutter's min/max value into its base/growth size. Therefore the non-collapsed 5px gutter is already 5/5 at `Initialize`; the 17px fixed column is 17/17. The intrinsic column and auto row start with base zero and an internal unbounded growth limit.
- `vendor/taffy/src/compute/grid/track_sizing.rs:790-923` handles the single non-flex span-1 item and records `IntrinsicItemBatch` after applying its base and growth updates. With the definite grid size, the auto minimum takes the image's minimum contribution (24/16); the max-content-alike maximum takes its max-content contribution (24/16). Hence each intrinsic track is finite at this snapshot. There is one batch per axis because the fixture has one item in each axis and no flexible track.
- `vendor/taffy/src/compute/grid/track_sizing.rs:424-440,1256-1273` records `IntrinsicSizingComplete` after intrinsic resolution. Both item tracks already have finite growth limits, so the final fallback for still-infinite limits changes nothing.
- `vendor/taffy/src/compute/grid/mod.rs:1605-1622` serializes vector order, gutter/collapsed flags, sizes, and `growth_limit.is_finite().then_some(...)`. Thus `null` is only the non-finite projection. The Taffy initialization code sets the intrinsic values to positive infinity, but the trace's `null` alone cannot distinguish positive infinity from other non-finite values.

## Cross-check of the corrected expectation contract

The current child plan table, `spec-derived-expectations.json` stages and track-vector model, Android `expectedBases`, iOS `expectedBaseSizes`, and runtime README agree on slot order, flags, and all three stage vectors above. Both native matchers allow a null growth value only for the intrinsic track at `Initialize` and compare all other growth limits against their finite expected values. The README's separate slot note also identifies index 2 as the uncollapsed 5/5 gutter.

The preserved Android and iOS first-run logs contain `internalIndex:2`, `gutter:true`, `collapsed:false`, `baseSize:5`, `growthLimit:5` at `Initialize`. I checked those logs only after deriving the expectation from the pinned spec and Taffy code; they corroborate the derivation and were not used as its oracle. Fixture SHA-256 remains `e3b6edfe8452d3674a9e905de10ddd96774b192ac73c4b3c46a5a42606c92c2b`; PNG SHA-256 remains `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2`.

## 한계와 모호성

There is no remaining numerical ambiguity for these three stage arrays under the stated fixture. CSS Grid specifies track/gutter sizing, not Taffy's raw sentinel slots or their `collapsed` flag; those are established from the pinned Taffy source. The expected `null` also asserts only a non-finite projection, not the specific internal infinity value. This derivation does not establish that an updated app matcher passes or that the runtime slice is complete.
