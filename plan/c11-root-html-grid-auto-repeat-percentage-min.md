# C11.1 · parsed Root HTML Grid auto-repeat percentage minimum

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [현재 Root HTML auto-repeat 제한](./c11-root-html-grid-auto-repeat.md) · [0px minimum 후속](./c11-root-html-grid-auto-repeat-zero-min.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 구현 PR [#1220](https://github.com/ohah/spinon/pull/1220)이 2026-11-06T12:13:13Z Rebase and merge로 병합됐다(merge SHA `2e022c9589eacb49ec2d04c5ea0dd1cb3ca575af`). Chrome 157 사전 비교 6/6와 독립 계산을 고정한 뒤 기본 `RuntimeRootHtmlTextPaintV1`의 `RootHtmlAutoRepeatV1`에 definite-axis `minmax(<percentage>,1fr)` minimum을 추가했다. 이 제한 구현은 C11.1/C11 완료나 공식 `78/214`를 바꾸지 않는다. 기존 C11.4 opt-in percentage auto-repeat fixture는 별도 경로다. [실행 근거](../spec/internal/evidence/c11-root-html-grid-auto-repeat-percentage-min-runtime-2026-11-06/README.md)

**사전 비교:** [Chrome observation·독립 기대값·검증 도구](../spec/internal/evidence/c11-root-html-grid-auto-repeat-percentage-min-precomparison-2026-11-06/README.md) · [고정 HTML](../tests/fixtures/css/c11/root-html-grid-auto-repeat-percentage-min-precomparison.html) · [expected.json](../spec/internal/evidence/c11-root-html-grid-auto-repeat-percentage-min-precomparison-2026-11-06/expected.json) · [Chrome 157 observation](../spec/internal/evidence/c11-root-html-grid-auto-repeat-percentage-min-precomparison-2026-11-06/chrome157-observation.json)

## 결손과 목표

구현 전 기본 parsed Root HTML auto-repeat 경로는 한 축의 `repeat(auto-fill|auto-fit, minmax(<px>,1fr))`만 허용했고, Root HTML cascade projection과 layout admission은 `minmax(<percentage>,1fr)` minimum을 거부했다. PR #1220에서 definite width/height가 제공하는 같은 grid content-box axis basis를 repeat-count 계산과 typed percentage minimum sizing에 연결했다. 다음 단락과 표는 구현 전 판정 기준을 보존한다.

기존 기본 Root HTML 계산 경로와 Taffy sizing을 그대로 사용해 단일/2-track pattern의 percentage minimum을 전달한다. auto-repeat count는 고정된 definite content-box 크기와 gap에서 독립 산식으로 검사한다. Count용 1 CSS px floor는 0% minimum의 나눗셈 경계에만 적용하고, 실제 percentage minimum과 최종 used track에는 적용하지 않는다.

## 규범 기준과 독립 oracle

- [CSS Grid Layout Module Level 1, 2025-03-26 CRD §7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat): auto-repeat 횟수 산정과 definite/inflexible sizing function 선택을 따른다.
- [§7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizing): definite grid axis에서 percentage track sizing function이 해당 axis의 container content size를 기준으로 계산되는 규칙을 따른다.
- [Chrome 157 사전 비교](../spec/internal/evidence/c11-root-html-grid-auto-repeat-percentage-min-precomparison-2026-11-06/README.md)는 browser observation과 독립 계산을 나란히 둔다. Expected track/frame은 고정 입력의 비율·gap·flex 배분에서 계산하며 browser 관측값으로 기대값을 만들지 않는다.

6개 장면의 독립 산식은 다음과 같다. `A`는 definite repeated-axis content size, `G`는 같은 축 gap, `P`는 반복 fragment 내 track 개수, `mᵢ`는 percentage를 `A`에 적용한 minimum이다. Count 계산은 각 `mᵢ`가 1px보다 작으면 count에만 1px를 사용한다.

```text
count = max(1, floor((A + G) / (Σ max(mᵢ, 1px) + P × G)))
```

산출물은 다음 경계를 포함한다.

| 장면 | 입력과 독립 결과 |
| --- | --- |
| `columns-fill-single` | 200px 축, 5px gap, 10% minimum → 8개, 각 used track 20.625px |
| `columns-fit-single` | 같은 repeat count, 첫 track만 occupied → `auto-fit` track `[200,0,0,0,0,0,0,0]` |
| `columns-fill-zero-percent` | 0% minimum은 sizing에서 0px, count에서 1px minimum → 34개 |
| `columns-fill-pair` | 200px 축, 2px gap, `[10%,5%]` minimum, 2-track pattern → 5회·10 tracks; min-constrained flex redistribution의 ideal pair `[20,16.4]px` |
| `rows-fill-single` | 160px 축, 4px gap, 20% minimum → 4개 row track 각 37px |
| `columns-fill-overflow` | 200px 축, 120% minimum → count 1, actual track 240px, visible overflow 40px |

2-track scene의 1fr flex algorithm은 먼저 20px minima에 걸리는 tracks를 freeze하고 남은 free space를 나머지 1fr tracks에 분배한다. Chrome은 used values를 1/64 CSS px 단위로 분산 반올림하므로 independent ideal values와 browser/frame 값은 0.02 CSS px 허용치로 비교한다. Track 개수와 authored order는 exact하게 비교한다.

## 입력 범위와 fail-closed 계약

1. Default parsed Root HTML route의 기존 `RootHtmlAutoRepeatV1`만 확장한다. 한 축에만 전체 track list auto-repeat 하나를 두고 반대 축은 unnamed finite non-negative fixed-px track 하나로 고정한다.
2. 반복 fragment는 1~2개 `minmax(<finite non-negative percentage>,1fr)` track이다. 0%, fractional percentage, 100% 초과 값을 허용하며 authored minimum을 clamp하지 않는다. Auto-fill/auto-fit, column/row 양축, zero/fixed-px minimum 회귀를 포함한다.
3. Repeated axis는 definite positive fixed CSS-px width/height여야 한다. min/max size, box edge, gap, direction, placement, item source/order, revision 및 10,000-track 경계는 현재 RootHtmlAutoRepeatV1 제한을 유지한다.
4. 반복 횟수 산정 때 percentage는 동일한 definite content-box axis에 적용한다. checked finite arithmetic 후 quota를 확인하고 expansion 전 초과를 거부한다. 0%는 count 계산에서만 1px floor를 사용하며 typed track에는 원래 Percentage(0)를 전달한다.
5. Taffy는 percentage minimum을 포함한 typed MinMax track을 실제로 sizing한다. Count용 px 환산값을 sizing 입력에 복사하거나 CSSOM/source 문자열에서 재해석하지 않는다.
6. Percentage basis가 없는 axis, non-finite/negative synthetic typed value, `minmax(percent, max-content|auto|px)`, percentage maximum, calc, 이름·neighbor·area·두 축 repeat 등은 기존 diagnostic으로 원자 거부한다. 실패한 layout/revision에서 partial frame이나 scene을 게시하지 않는다.

## 구현 당시 계획한 순서

1. `grid_values::root_html_auto_repeat`의 grammar/profile gate에 finite non-negative percentage minimum만 추가하고, cascade의 computed typed 값 및 source owner identity를 유지한다.
2. layout의 `RootHtmlAutoRepeatV1` contract에서 같은 범위의 `Percentage` minimum만 허용한다. `root_html_auto_repeat_cell_capacity`는 percentage를 repeated-axis definite basis로 resolve해 count를 계산하되 actual `GridTrackSize`는 변경하지 않는다.
3. Layout tests는 px regression, zero percentage, columns/rows, 1·2-track pattern, gap boundaries, 0/100/120%와 10,000-track cap, non-finite/negative input 및 overflow arithmetic을 검사한다. `auto-fit` collapse는 placement 후에 일어나며 repeat range와 gutter 동작을 보존한다.
4. Projection/style tests는 default parsed Root HTML에서 10%·5%·0% values가 동일 typed values로 전달되는지 확인한다. Unsupported maximum, calc, neighbor, indefinite axis, wrong profile과 malformed range를 rejection control로 둔다.
5. Default `RuntimeSession`에서 6개 고정 장면의 computed values, expanded count, used tracks, root-relative frames, overflow 및 revision/source identity를 expected JSON과 비교한다. 기존 positive px/zero-min auto-repeat 및 C11.4 regressions를 유지한다.
6. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 default Root HTML app route를 같은 fixture hash로 실행한다. Capture는 visual smoke로 분류하고 PixelCopy/Metal readback과 simulator/physical-device 증거를 구분한다.
7. 변경 crates의 focused suites, scoped Clippy, `cargo fmt --all -- --check`, default artifact/fixture boundary, mobile builds를 기록한다. Broad suite나 GitHub CI가 실제 green이 아니면 그렇게 기록하지 않는다.
8. 구현/runtime은 새 실패 관점 20개로, 실제 implementation PR diff는 별도 30개로 검토한다. 계획 PR 전에 계획 자체 20개와 실제 계획 PR diff 30개를 검토한다. 결함 발견 시 수정 후 영향받은 항목을 다시 확인한다.
9. 계획 PR을 먼저 병합한 뒤 구현 PR을 별도로 열었다. 구현 병합 후 내부 계약 0059, `spec/STATUS.md`, C11 계획과 public roadmap을 merge SHA 기준으로 갱신한다. Agent Browser와 해당 프로세스는 관찰 작업 후 종료한다.

이 slice는 percentage auto-repeat의 이 1~2 track Root HTML profile만 추가한다. General CSS admission, full C11.1/C11.4, WPT 적합성, all-grid combinations, hardware GPU, physical devices 또는 compositor-after pixel parity로 확대 해석하지 않는다.

## 계획 실패 관점 검토

[서로 다른 실패 관점 20개](./c11-root-html-grid-auto-repeat-percentage-min-plan-review-20.md)를 구현 전에 별도로 적용한다.

## 병합된 구현 결과 · PR #1220

PR [#1220](https://github.com/ohah/spinon/pull/1220)은 2026-11-06T12:13:13Z Rebase and merge됐고 merge SHA는 `2e022c9589eacb49ec2d04c5ea0dd1cb3ca575af`다. 기존 RootHtmlAutoRepeatV1에서 definite repeated-axis의 단일/2-track `minmax(<finite non-negative percentage>, 1fr)` minimum만 허용한다. 0%의 1px floor는 repeat count에만 적용하며 Taffy에는 authored typed percentage를 전달한다. 100% 초과 minimum은 clamp하지 않는다. 지원 밖 maximum·calc·indefinite axis·neighbor/name/area·두 축 repeat은 계속 fail-closed다.

Chrome 157 사전 비교 6/6, layout 11/11, style-to-layout 15/15, Stylo cascade 1/1, 기본 RuntimeSession 1/1(6 scene·27 boxes), C11.4 layout/cascade regression 13/13 및 6/6, scoped Clippy와 rustfmt를 기록했다. `spinon-style`의 더 넓은 `c11_1_root_html` filter는 29개 중 27개 통과·2개 실패였으며, 두 실패는 변경과 별도인 auto-row rejection 기대값에 관한 것이므로 suite 전체를 green으로 기록하지 않았다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 기본 Root HTML route는 같은 fixture에서 layout-ready·27 boxes를 확인했다. Android는 ANGLE/SwiftShader이고 iOS 표본은 compositor 제출 전 Metal drawable이다. 화면은 visual smoke이며 실기기·hardware GPU·WPT·일반 CSS admission·iOS compositor 이후 pixel parity 근거가 아니다. PR check rollup은 비었고 main required status checks도 설정되지 않았다.

[실행·artifact·한계](../spec/internal/evidence/c11-root-html-grid-auto-repeat-percentage-min-runtime-2026-11-06/README.md) · [구현/runtime 실패 관점 20개](../spec/internal/evidence/c11-root-html-grid-auto-repeat-percentage-min-runtime-2026-11-06/implementation-runtime-review-20.md) · [PR diff 실패 관점 30개](../spec/internal/evidence/c11-root-html-grid-auto-repeat-percentage-min-runtime-2026-11-06/pr-diff-review-30.md).

이 bounded implementation은 C11.1/C11 완료나 공식 집계 변경을 뜻하지 않는다. 공식 `78/214`와 내부 계약 숫자 버전 `0.1.0`은 유지한다.
