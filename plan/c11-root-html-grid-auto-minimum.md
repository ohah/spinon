# C11.1 · parsed Root HTML Grid `minmax(auto, fr)` 제한 구현 계획

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [Root HTML bounded `minmax()`](./c11-root-html-grid-minmax.md) · [C11.4 overflow automatic minimum](./c11-4-overflow-scrollable-auto-minimum.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

## 결손과 목표

기본 parsed Root HTML Grid 경로는 finite non-negative `px`/percentage minimum과 `px`/percentage/`fr` maximum을 가진 `minmax()`를 허용하지만 `auto` minimum을 거부한다. C11.4 opt-in fixture 경로에는 automatic minimum의 item `Minimum` contribution과 `minmax(0, 1fr)` contrast가 있으나 기본 Root HTML 작성 경로에는 연결되어 있지 않다.

이 제한 후속은 기존 `RuntimeRootHtmlTextPaintV1` 경로에서 `minmax(auto, finite-nonnegative-fr)`를 explicit column·row track 하나씩 허용한다. definite content axis, `min-width`/`min-height:auto`, max-size `none`, no aspect-ratio와 기본 visible overflow를 요구한다. CSS Grid의 auto minimum을 사용한 track과 `minmax(0, 1fr)` control을 두 축에서 대조해 item의 minimum contribution이 track base size에 반영되는지 확인한다. C11.4 fixture allowlist에 의존하지 않고 scene ID·DOM ID·class 이름을 product success selector로 사용하지 않는다.

이 계획은 모든 `minmax(auto, …)` 또는 CSS Grid 전체 지원을 뜻하지 않는다. Unsupported grammar는 같은 node/property에 귀속한 오류로 layout publication 전에 거부한다. 내부 계약 0059 버전과 C11/C11.1/C11.4 체크 상태는 변경하지 않는다.

## 규범 기준과 사전 비교

CSS Grid Layout Module Level 1, 2025-03-26 CRD의 [§7.2.1 Track Sizing Functions](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizing-functions), [§11.5 Track Sizing Algorithm](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing), [§11.7 Expand Flexible Tracks](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-flex-tracks)를 기준으로 한다. 규범식에서 먼저 계산한 기대값은 [expected.json](../spec/internal/evidence/c11-root-html-grid-auto-minimum-precomparison-2026-11-03/expected.json), Agent Browser Chrome 156 관찰은 [chrome-observation.json](../spec/internal/evidence/c11-root-html-grid-auto-minimum-precomparison-2026-11-03/chrome-observation.json)에 분리 기록한다. [Chrome 화면](../spec/internal/evidence/c11-root-html-grid-auto-minimum-precomparison-2026-11-03/chrome156-agent-browser.png)은 외부 overflow 장면의 시각 확인 자료다. [입력과 결과 hash](../spec/internal/evidence/c11-root-html-grid-auto-minimum-precomparison-2026-11-03/SHA256SUMS.txt)를 고정한다. Chrome은 독립 구현 교차 확인이며 기대값의 출처가 아니다. `0fr` 경계 기대값은 specification-derived 항목으로 분리했고 browser observation으로 표시하지 않는다.

고정 fixture는 [HTML](../tests/fixtures/css/c11/root-html-grid-auto-minimum-precomparison.html)이다. Columns와 Rows 각각에서 100px definite content axis에 `minmax(auto, 1fr) 40px`를 둔다. 해당 axis의 first direct item은 80px fixed size이고 second item은 40px이다. `minmax(auto, 1fr)`에서는 automatic minimum의 80px contribution이 flex track의 base size가 되어 used tracks `[80,40]`과 root border 밖 18px overflow를 만든다. `minmax(0, 1fr)` control에서는 첫 track이 남은 60px을 차지해 `[60,40]`이 된다. Root의 반대 축은 40px definite fixed size다. Margin/padding/gap은 0이고 root border는 2px다.

Chrome capture는 used track sizes, root/item DOMRect, computed min-size/overflow와 viewport/DPR를 기록하고 외부로 나온 second item의 화면을 보존한다. Fixture SHA-256은 [hash manifest](../spec/internal/evidence/c11-root-html-grid-auto-minimum-precomparison-2026-11-03/SHA256SUMS.txt)에 별도로 고정한다. Browser 결과가 규범 계산과 다르면 원인을 확인할 때까지 계획을 구현 준비 상태로 취급하지 않는다.

## 지원 범위와 fail-closed 입력

- Basic parsed Root HTML default route에만 적용하고 existing Stylo typed `MinMax` 값을 사용한다. CSS string을 다시 parse하거나 별도 runtime profile/FFI route를 추가하지 않는다.
- `minmax(auto, fr)`의 `fr` factor는 finite·non-negative여야 한다. zero factor도 CSS-valid 입력으로 분리해 expected behavior를 확인한다.
- 해당 axis는 같은 computed-style snapshot에서 definite positive length size여야 하며 computed min-size는 `auto`, max-size는 `none`, aspect-ratio는 미설정이어야 한다.
- 새 allowlist는 `auto` minimum + `fr` maximum만 추가한다. `minmax(auto, <length-percentage>)`, `minmax(min-content, …)`, `minmax(max-content, …)`, flex minimum, calc breadth, auto-repeat, implicit minmax, named-area interaction, non-default flow/placement는 제외한다.
- Grid item의 automatic minimum은 그 item의 computed overflow에 따른 CSS Grid 규칙을 따른다. 이 slice는 root와 item의 visible overflow만 허용하며 item `hidden`/`auto`/`scroll`/`clip` overrides를 negative로 검사한다. Scroll-container/min-size 0 behavior는 별도 C11.4 evidence에서 다룬다.
- direct children 두 개와 row-wise default auto placement, one supported `minmax()` track, one fixed `40px` neighbor, zero gap, fixed-size `div` items만 positive profile로 받는다. Extra/missing/reordered malformed node structure 및 unsupported styles는 partial frame 없이 거부한다.
- Root item identities/classes는 test labels일 뿐 product selector가 아니다. 동일 CSS의 ID/class 변경 positive control과 changed size/property/axis/revision negative controls를 포함한다.

## 계획 PR 수락 기준

1. CSS Grid CRD 규칙에서 `[80,40]`, `[60,40]`, root/item frame과 border 바깥 18px을 Chrome 관찰 전에 독립 계산하고 JSON에 고정한다.
2. Agent Browser Chrome 156에서 네 장면의 computed styles, used tracks, DOMRect와 source hash를 수집한다. DOM geometry는 root border origin 기준으로 보관한다.
3. Feature/fixture boundary, root axis definiteness, `minmax(auto, 0fr)`의 expected result, min/max override, overflow semantics, ID/class independence, partial publication failure modes를 검토한다.
4. 계획 PR에는 구현 전 비교 자료, 계획 review 20개와 실제 계획 PR diff 30개 관점 검토를 연결한다. C11.1/C11.4 및 공식 완료 수를 올리지 않는다.

## 구현 PR 수락 기준

1. Root HTML explicit-track validator가 exact supported `MinMax { min: Auto, max: Fraction }` shape만 허용하고 기존 breadth converter와 `RootHtmlImplicitRowsV1`/track range checks에 올바르게 연결한다. auto-minimum track과 `minmax(0, fr)` comparison을 같은 box geometry로 비교할 수 있게, finite fr maximum인 `minmax()`를 쓰는 Grid container에만 유한한 non-negative computed px border를 허용한다. 다른 Root HTML border gate와 item border 제한은 유지한다.
2. Rust tests에서 rows/columns positive, `minmax(0, 1fr)` controls, `0fr`, indefinite/zero/negative/non-finite basis/factor, unsupported breadth, min/max override, aspect-ratio, item overflow override, placement/tree/style/document revision negatives를 검사한다.
3. Parsed Root HTML runtime matrix에서 tracks, full frames, source/style/layout revision identity, overflow bounds와 unsupported input의 no-partial-output를 검사한다.
4. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 same-source fixture에서 geometry 및 draw/present를 확인한다. 외부 18px strip은 Android PixelCopy와 iOS Metal drawable에서 좌표·RGBA를 대조한다. AVD software renderer와 compositor 이전 iOS drawable 한계를 보존한다.
5. C11.1 runtime FFI fixture route/symbol/HTML payload를 추가하지 않는다. 기본 FFI staticlib와 Android native artifact에는 새 surface-pixel fixture 심볼이 없어야 한다. Android debug capture harness의 요청·로그 label은 DEX에 남을 수 있으므로 BuildConfig가 `false`이고 fixture HTML payload가 번들에 없는지 함께 확인한다. Default iOS app은 auto-minimum capture 인수 label을 포함하지 않아야 한다.
6. 새 구현/runtime 실패 관점 20개와 별도 implementation PR diff 30개 검토를 하고, 수정이 발견되면 영향 항목과 검증을 다시 수행한다. PR body에 simulator와 Chrome 화면·hash evidence를 넣고 Agent Browser에서 각 이미지를 확인한다.
7. 구현 PR에 내부 계약 0059의 bounded input/error/runtime 한계를 기록하고 simulator·Chrome 화면 및 hash 링크를 싣는다. PR diff review 30개와 CI·merge를 마친 뒤 `spec/STATUS.md`, C11 plan, public roadmap을 merge SHA에 맞춰 동기화한다. C11/C11.1/C11.4 checkboxes와 `72/213`은 그대로 유지한다. Tailnet preview를 안전 동기화하고 served hash, desktop/mobile layout 및 Agent Browser 종료를 확인한다.

## 제외 범위

일반 CSS author admission, 전체 `minmax()` grammar, `min-content`/`max-content` breadth, auto-repeat, named lines/areas, spans, nested Grid, dense/column flow, scrollable overflow, WPT runner/conformance, 실기기/hardware GPU, iOS compositor 이후 pixels 및 C11/C11.4 전체 완료 판정은 포함하지 않는다.
