# C11.4 Grid auto minimum · 계획 실패 관점 검토 20개

**대상:** [overflow scrollability 계획](./c11-4-overflow-scrollable-auto-minimum.md), [Chrome 사전 비교](../spec/internal/evidence/c11-4-overflow-scrollable-auto-minimum-precomparison-2026-10-28/README.md), [CSS Grid §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto), [CSS Overflow §3](https://www.w3.org/TR/2025/WD-css-overflow-3-20251007/#overflow-properties).

**검토 경계:** 구현 전 plan/evidence를 대상으로 한 별도 설계 검토다. 제품 코드, runtime, simulator 또는 CI 검증은 포함하지 않는다.

| # | 서로 다른 실패 관점 | 계획에서 요구하는 방지 조건 | 판정 |
| ---: | --- | --- | --- |
| 1 | Chrome에서 기대값을 역산해 circular oracle 생성 | Ahem glyph/line-height와 W3C 조건으로 독립 산술을 JSON에 분리한다. | 통과 |
| 2 | `clip`을 scroll container로 잘못 분류 | Overflow 규범이 `clip`과 `hidden`을 다르게 구분하며 visible/clip 양쪽을 비교한다. | 통과 |
| 3 | `auto`/`scroll` 값을 검사하지 않고 `hidden`만 대표값으로 일반화 | hidden, auto, scroll을 각각 Columns/Rows 입력으로 둔다. Auto adapter가 Hidden으로 매핑되는 점도 검증한다. | 통과 |
| 4 | Grid가 auto-minimum이 아닌 track을 써서 case가 무의미 | target axis를 하나의 `minmax(auto,1fr)` track으로 고정한다. | 통과 |
| 5 | span 여러 track 조건이 자동 최소 크기를 0으로 만들어 기대 차이를 가림 | item은 한 개 target-axis track만 span하고 다른 CSS Grid 조건을 고정한다. | 통과 |
| 6 | Columns만 시험하고 row-axis mapping이 회귀 | 동일한 independent oracle을 Columns와 Rows 양쪽에서 검증한다. | 통과 |
| 7 | row text에 min-content 10px이 생기지 않아 false positive | Ahem 10px line-height의 단일 glyph와 5px root height를 고정한다. | 통과 |
| 8 | column text 줄바꿈으로 min-content가 100px이 되지 않음 | 공백 없는 10 glyph 한 줄과 loaded Ahem font를 입력·metadata로 고정한다. | 통과 |
| 9 | viewport/device scale 변동을 CSS geometry 차이로 오해 | Browser metadata에 viewport `1280×633`, DPR 1을 기록하며 oracle은 CSS px 기반이다. | 통과 |
| 10 | font 미로딩 fallback으로 잘못된 min-content 계산 | `document.fonts.check`와 `fonts.status`를 저장하고 Ahem digest를 기록한다. | 통과 |
| 11 | item frame만 확인해 track auto minimum 오계산이 숨겨짐 | 최종 track bases와 item frame을 각각 assertion한다. | 통과 |
| 12 | track/frame 우연히 같아서 automatic-minimum source 분기 회귀를 놓침 | contribution trace에서 content-based, overflow-zero, specified-zero, zero-track을 분리한다. | 통과 |
| 13 | explicit `min-width/height:0`을 overflow auto-minimum 결과로 혼동 | visible overflow에서 별도 specified-minimum control을 둔다. | 통과 |
| 14 | `minmax(0,1fr)`을 item minimum 0과 같은 원인으로 해석 | item min 0 control과 track min 0 control을 각각 검사한다. | 통과 |
| 15 | Grid outer 크기를 content contribution과 혼동 | CSS Grid dimensions와 used track/item dimensions를 기대 JSON에 별도 필드로 둔다. | 통과 |
| 16 | shorthand의 축 coupling으로 다른 overflow value가 계산에 적용 | 각 비교 입력은 shorthand 하나로 양축 동일 값을 지정하고 computed overflow를 확인한다. | 통과 |
| 17 | scrollWidth를 scrolling API 성공이나 clipping 증명으로 과대 해석 | README에서 scroll extents와 UI scroll/clip 동작을 분리한다. | 통과 |
| 18 | Taffy layout unit test를 Android/iOS 앱 검증으로 과대 해석 | 앱/runtime/simulator 검증을 acceptance gate에 넣지 않고 근거에도 주장하지 않는다. | 통과 |
| 19 | 고정 Taffy vendor 분기 외 일반 CSS parsing 지원을 암시 | adapter mapping과 vendored source만 확인하며 parser/admission과 C13 동작을 명시적으로 제외한다. | 통과 |
| 20 | 좁은 regression test를 전체 C11.4 완료나 공식 count 증가로 표현 | Status checkbox와 `72/213`을 유지하고 계획·구현·PR 검토 결과를 별도로 기록한다. | 통과 |

**판정:** PASS 20/20, 계획 blocker 0개. 독립 기대값, 양축·다섯 overflow 분류, source-aware trace 및 좁은 C11.4/C13 경계가 구현 전에 고정됐다. 구현 runtime 검토 20개와 PR diff 검토 20개는 실제 변경 뒤 새로운 관점으로 수행한다.
