# C11.1 · parsed Root HTML Grid named areas와 `minmax(auto, fr)` 제한 구현 계획

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [named-area 기본 runtime](./c11-root-html-grid-template-areas.md) · [`minmax(auto, fr)` no-area profile](./c11-root-html-grid-auto-minimum.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**계획 실패 관점 검토:** [서로 다른 20개 경계](./c11-root-html-grid-auto-minimum-named-areas-plan-review-20.md)

**계획 PR diff 적대 검토:** [실제 변경의 독립 30개 관점](../spec/internal/evidence/c11-root-html-grid-auto-minimum-area-precomparison-2026-11-03/plan-pr-diff-review-30.md)

## 결손과 목표

기본 parsed Root HTML 경로는 definite axis와 visible overflow 조건에서 `minmax(auto, finite non-negative fr)` explicit track을 처리한다. 기존 named-area 경로는 [2×2 `header`/`side`/`content` area map](../crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs)의 제한 track profile만 받아 `minmax()` 전체를 거부한다. 이 작업은 이미 연결된 두 기능의 조합을 고정된 area profile에서 검증하고 기본 route에 한정해 연다.

CSS Grid 2025-03-26 CRD의 [§7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizing-functions), [§11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing), [§11.7](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-flex-tracks)에 따라, 100px definite content axis에서 `minmax(auto,1fr) 40px`와 80px non-spanning item automatic minimum은 `[80px,40px]`을 만든다. `minmax(0,1fr) 40px` control은 `[60px,40px]`이다. 2px root border를 포함하면 auto-minimum scene의 content overflow는 20px, border 바깥으로 보이는 spill은 18px이다. `minmax(auto,0fr)`는 80px automatic minimum을 유지한다.

구현 전 비교는 규범식에서 계산한 [expected.json](../spec/internal/evidence/c11-root-html-grid-auto-minimum-area-precomparison-2026-11-03/expected.json)과 Agent Browser Chrome 156 [관측 JSON](../spec/internal/evidence/c11-root-html-grid-auto-minimum-area-precomparison-2026-11-03/chrome-observation.json)을 분리했다. [고정 HTML](../tests/fixtures/css/c11/root-html-grid-auto-minimum-area-precomparison.html), [화면](../spec/internal/evidence/c11-root-html-grid-auto-minimum-area-precomparison-2026-11-03/chrome/chrome156-agent-browser.png), [SHA-256 manifest](../spec/internal/evidence/c11-root-html-grid-auto-minimum-area-precomparison-2026-11-03/SHA256SUMS.txt)도 보존했다. Chrome은 교차 관찰이며 기대값의 출처가 아니다.

## 지원 범위와 fail-closed 입력

- `RuntimeRootHtmlTextPaintV1` 기본 Root HTML 경로 및 현재 지원하는 정확한 rectangular `header`/`side`/`content` 2×2 area map에만 적용한다.
- Grid에 direct child 세 개를 두고 기존 `header`, `side`, `content` area alias로 각각 배치한다. Columns matrix는 `[minmax(auto, fr), 40px]` columns와 `[20px,20px]` rows를, rows matrix는 `[40px,40px]` columns와 `[minmax(auto, fr),40px]` rows를 사용한다. `fr`은 finite·non-negative이며 0을 포함한다.
- auto-minimum 축은 같은 computed-style snapshot의 definite positive content-box length여야 한다. 해당 축의 min size는 `auto`, max size는 `none`이고 aspect ratio는 없어야 한다. Grid와 contributing item의 overflow는 visible이어야 한다.
- Positive item은 해당 축에 고정된 80px item 및 40px neighbor를 사용한다. Root border 2px, gap/margin/padding 0, 두 번째 row/column fixed 40px neighbor는 비교 fixture와 같은 조건을 유지한다.
- CSS typed `MinMax { min: Auto, max: Fraction(finite non-negative) }`와 기존 named-area alias resolution만 사용한다. CSS string을 다시 parse하거나 별도 FFI/runtime profile을 만들지 않는다.
- `minmax(auto, <length-percentage>)`, `min-content`/`max-content`, auto repeat, implicit track/area, non-default flow, span, nested Grid, scroll-container item, non-visible overflow, arbitrary area map, extra/missing direct child, unsupported placement는 제외한다.
- 이름을 바꾼 ID/class positive control에서도 같은 결과를 내야 한다. DOM identity나 이름이 지원 여부를 결정해서는 안 된다.

## 계획 PR 수락 기준

1. [독립 기대값](../spec/internal/evidence/c11-root-html-grid-auto-minimum-area-precomparison-2026-11-03/expected.json), Chrome 관측, input HTML과 screenshot hash를 서로 독립적으로 기록한다. browser result가 규범 계산과 어긋나면 원인을 해결하기 전 구현하지 않는다.
2. Columns·rows의 auto-minimum, `minmax(0,1fr)` control 및 `minmax(auto,0fr)` boundary의 full frames, used tracks, root-border 바깥 spill을 검토한다.
3. Named-area admission과 기존 no-area admission을 함께 검토한다. min/max-size, definite axis, aspect ratio, item overflow, spanning/area semantics, CSS cascade fallback, source/style revision, no-partial-output를 별도 negative boundary로 둔다.
4. 계획 문서를 코드 변경 전에 20개의 독립 실패 관점으로 검토하고, 계획 PR의 실제 diff를 프로젝트 규칙에 따라 30개의 독립 관점으로 적대 검토한다.

## 구현 PR 수락 기준

1. Named-area validator가 정확한 2×2 area map과 위의 축별 track 조합만 허용한다. 다른 기존 named-area profile, no-area profile, line-name/auto-repeat rejection 동작은 유지한다.
2. Rust tests에서 Chrome 고정 입력의 두 축 auto minimum/control/0fr 결과와 item/root frame을 검증한다. definite-size/min/max/aspect-ratio/finite-fr/item overflow/area shape/direct-child count/placement/revision negatives는 node/property-bound 오류로 실패하고 partial layout을 공개하지 않아야 한다.
3. Positive item의 ID/class 변경 control과 area alias/DOM source order control을 포함한다. 기존 C11.1 no-area auto-minimum 및 named-area/percentage/fit-content/fr regression tests를 실행한다.
4. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 동일 Root HTML source의 여섯 장면을 실행한다. Geometry는 기대값과 ±0.5 CSS px를 비교한다. Border 밖 columns 표본은 root-relative `(110,30)`, rows 표본은 `(50,110)`에서 auto/0fr가 `#70a8d8`이고 zero-min control이 `#ffffff`인지 PixelCopy와 Metal drawable로 exact 비교한다. AVD software renderer와 iOS compositor 이전 drawable 한계를 기록한다.
5. C11 capture payload, FFI export, fixture-test route 또는 product ABI를 추가하지 않는다. 기본 FFI/native artifact에서 새 capture symbol/string/payload가 없는지 기존 auto-minimum slice와 같은 대조로 확인하고, 실행 source fixture를 복구한 뒤 원본 파일 hash를 확인한다.
6. 새 구현/runtime 검토 20개와 실제 구현 PR diff 검토 30개를 별도로 수행한다. PR 본문에는 Android/iOS 화면 및 비교 결과 링크를 싣고 Agent Browser에서 이미지 렌더링을 확인한 뒤 브라우저 프로세스를 닫는다.
7. PR 병합은 사용자에게 이전에 승인된 Rebase and merge 흐름을 따른다. Merge 후 `spec/internal/0059-c11-grid-layout.md`, `spec/STATUS.md`, C11 plan 및 public/Tailnet roadmap을 결과에 맞춰 동기화한다. 이 제한 조합은 C11.1/C11 또는 공식 `72/213`을 완료 처리하지 않는다.

## 제외 범위

일반 CSS author admission, arbitrary named areas, 일반 Grid feature completeness, WPT runner/conformance, implicit tracks, min-content/max-content sizing, writing mode/RTL, 실기기/hardware GPU, iOS compositor 이후 pixels, C11.1/C11.3/C11 전체 완료 판정은 이 slice에 포함하지 않는다.
