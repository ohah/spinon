# C11.1 · parsed Root HTML Grid auto-repeat zero minimum

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [현재 Root HTML auto-repeat 제한](./c11-root-html-grid-auto-repeat.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** Agent Browser Chrome 157 사전 비교 10/10과 독립 repeat-count 계산을 고정했다. 계획 실패 관점 검토 20개를 완료했다. 구현은 시작하지 않았다. C11.1/C11 완료와 공식 `78/214`는 그대로다.

**규범 근거:** [CSS Grid Layout Module Level 1, 2025 CRD §7.2.3.1–7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat). 자동 반복은 definite fixed-size track을 요구한다. 반복 횟수를 셀 때 zero-size에 division-by-zero가 생기지 않도록 UA가 track-size floor를 선택하며 1 CSS px을 권장한다. 이 bounded profile은 1px floor를 선택한다. 이 floor는 repetition count에만 쓰고 `minmax(0px, 1fr)`의 actual minimum은 0px로 둔다.

## 목표

현재 기본 `RuntimeRootHtmlTextPaintV1`의 [`RootHtmlAutoRepeatV1`](./c11-root-html-grid-auto-repeat.md) 경로에서 기존 positive fixed-px minimum을 보존하면서 `minmax(0px, 1fr)` auto-repeat를 지원한다. CSS는 parsed Root HTML의 computed track values로 판정한다. 선언 원문 검색이나 fixture ID만으로 지원 값을 추정하지 않는다.

반복 pattern은 기존 profile대로 하나 또는 두 track이며, 각 track은 `minmax(<finite non-negative px>, 1fr)`다. 하나의 zero minimum과 두 개의 zero minimum, zero/positive mixed pattern을 포함한다. auto-fill/auto-fit, columns/rows, 기존 definite positive container size와 finite non-negative px/normal gap 조건을 유지한다. 두 축 반복, implicit track, line names, areas, named placement, flexible max 외 문법, percentage/calc, nested Grid, authored CSS 일반 admission은 지원하지 않는다.

## 반복 횟수의 독립 기대값

현재 제한 profile은 definite container size와 fixed px minimum만 허용한다. pattern에 `P`개 track이 있을 때, count용 size는 각 minimum에 대해 `max(minimum, 1px)`로 둔다. `G`는 axis gap, `S`는 floored minimum 합, `A`는 definite content-box size일 때:

```text
repeatCount = max(1, floor((A + G) / (S + P × G)))
```

식은 반복 pattern 내부 gutter와 인접 반복 사이 gutter를 모두 포함한다. 반복 횟수를 먼저 계산하고, `repeatCount × P`가 기존 `GRID_MAX_TRACKS` quota를 넘는지 overflow-safe arithmetic으로 검사한다. 1px floor를 Taffy track sizing이나 최종 used track minimum에 적용하지 않는다. 모든 `auto-fit` 빈-track collapse는 placement 후 처리한다.

## Chrome 157 사전 비교

[고정 fixture](../tests/fixtures/css/c11/root-html-grid-auto-repeat-zero-min-precomparison.html)와 [사전 비교 증거](../spec/internal/evidence/c11-root-html-grid-auto-repeat-zero-min-precomparison-2026-11-06/README.md)는 다음 10개 case를 고정한다.

| ID | Axis / mode | Definite size / gap | Pattern minima | Independent expected repeats / used tracks |
|---|---|---:|---:|---|
| `fill-11` | columns / auto-fill | 11px / 0px | `[0]` | 11 / 11 × 1px |
| `fit-11` | columns / auto-fit | 11px / 0px | `[0]` | 11 / `[11, 0 × 10]` |
| `fill-gap-11` | columns / auto-fill | 11px / 1px | `[0]` | 6 / 6 × 1px |
| `fill-fractional` | columns / auto-fill | 12.5px / 0.5px | `[0]` | 8 / 8 × 1.125px |
| `fill-two-zero` | columns / auto-fill | 19px / 1px | `[0, 0]` | 5 / 10 × 1px |
| `fill-mixed-two` | columns / auto-fill | 19px / 1px | `[0, 2]` | 4 / `[1, 2] × 4` |
| `fit-mixed-two` | columns / auto-fit | 19px / 1px | `[0, 2]` | 4 / `[19, 0 × 7]` |
| `empty-fit-11` | columns / auto-fit, empty | 11px / 0px | `[0]` | 11 / 11 × 0px |
| `fill-rows-7` | rows / auto-fill | 7px / 1px | `[0]` | 4 / 4 × 1px |
| `fill-rows-mixed` | rows / auto-fill | 19px / 1px | `[0, 2]` | 4 / `[1, 2] × 4` |

Chrome supports this syntax and its observed tracks match the independent calculation. Chrome is an implementation oracle for this fixed fixture, not a CSS specification and not WPT conformance evidence.

## Implementation and acceptance

1. Extend only the existing Root HTML auto-repeat typed validator from positive to finite non-negative fixed-px minima. Preserve the computed-value source, fixed-size container requirement and fail-closed unsupported grammar.
2. Add columns/rows runtime cases for one-track and two-track zero/mixed patterns, auto-fill/auto-fit, empty-track collapse, fractional gaps and a positive-minimum regression. Confirm the final track list uses the declared minimum and flex sizing; the 1px count floor must not leak into track sizing.
3. Test repeat count and expanded-track quota boundaries. Counts at or below `GRID_MAX_TRACKS` retain order and identity. Above-limit counts, integer overflow and malformed/resolved values must fail before any new frame or partial scene is published.
4. Compare the same parsed Root HTML fixture through the default Android API 37 ARM64 AVD and iPhone 17 Pro/iOS 26.2 Simulator route, including `layout=ready`, draw/present, source revision and visible overflow samples where the item content requires them. Bind runs to fixture, app and source hashes. Screenshots are visual smoke unless a surface readback is explicitly captured.
5. Keep all existing positive-minimum Root HTML auto-repeat and C11.1/C11.4 regression suites. Run the affected Rust packages, scoped Clippy, `cargo fmt --all -- --check`, platform app builds and existing artifact-boundary checks. Record pre-existing failures separately; do not claim a broad suite passed if it did not.
6. Review the implementation/runtime from 20 distinct failure perspectives, then review the actual PR diff from 30 distinct failure perspectives. Put available Android/iOS screenshots in the PR and verify their displayed images with Agent Browser. Re-query GitHub status checks and merge only by `Rebase and merge`.
7. Update `spec/STATUS.md`, internal contract 0059 and the public roadmap with the precise bounded scope after implementation merge. Do not mark C11.1, C11.4 or C11 complete, and do not change official `78/214` for this slice.

## Excluded claims

This slice does not prove arbitrary auto-repeat track lists, general CSS Grid support, named lines/areas, two-axis auto-repeat, all WPT cases, physical-device parity, Android hardware GPU, iOS compositor-after pixels or C11 completion. The 1px floor is the chosen profile behavior; the specification suggests rather than mandates this exact floor.

## 계획 검토

계획 자체의 서로 다른 실패 관점 20개는 [검토 기록](./c11-root-html-grid-auto-repeat-zero-min-plan-review-20.md)에 기록한다.
