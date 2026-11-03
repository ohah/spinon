# C11.1 named-area `minmax(auto, fr)` 계획 실패 관점 검토 20개

검토 대상은 [계획](./c11-root-html-grid-auto-minimum-named-areas.md), 고정 [Chrome fixture](../tests/fixtures/css/c11/root-html-grid-auto-minimum-area-precomparison.html), 독립 [기대값](../spec/internal/evidence/c11-root-html-grid-auto-minimum-area-precomparison-2026-11-03/expected.json), Chrome [관측](../spec/internal/evidence/c11-root-html-grid-auto-minimum-area-precomparison-2026-11-03/chrome-observation.json), [화면과 hash](../spec/internal/evidence/c11-root-html-grid-auto-minimum-area-precomparison-2026-11-03/README.md)다. 아래 관점은 구현 전 계획 자체를 검토한다.

| # | 별도 실패 경계 | 판정 및 조치 |
| --- | --- | --- |
| 1 | C11.1의 공개 완료 판정과 제한 Root HTML 동작을 혼동 | 통과. 계획은 C11.1/C11 체크와 `72/213`을 바꾸지 않는다고 고정한다. |
| 2 | CSS used size 계산에서 border box와 content box를 혼동 | 통과. 100px content axis와 2px 양쪽 border를 분리해 outer box 104px, 바깥 spill 18px을 기록한다. |
| 3 | `minmax(auto,1fr)`의 automatic minimum을 독립적으로 산출하지 않음 | 통과. 80px non-spanning item contribution과 40px neighbor에서 `[80,40]`을 먼저 계산하고 Chrome과 분리했다. |
| 4 | zero-minimum 대조군이 같은 definite axis를 사용하지 않음 | 통과. `minmax(0,1fr)`는 같은 root/item 입력에서 `[60,40]`을 기대한다. |
| 5 | zero `fr` factor의 CSS-valid 경계를 일반 flex-positive 동작과 혼동 | 통과. `minmax(auto,0fr)`를 별도 scene으로 두고 80px minimum 보존을 명시한다. |
| 6 | columns와 rows 구현에서 축을 바꾸거나 percentage basis를 교환 | 통과. 독립 6-scene matrix가 같은 모델을 각각 수평·수직 축에 적용한다. |
| 7 | area가 요구하는 2×2 track 수와 authored template 크기 불일치 | 통과. 고정 named-area map과 각 축 2개 explicit track을 요구한다. |
| 8 | arbitrary area matrix를 우발적으로 허용 | 통과. 기존 `header`/`side`/`content` rectangular map만 admit한다. |
| 9 | spanning header가 columns positive의 automatic minimum을 오염 | 통과. header는 빈 콘텐츠이며 auto-minimum contribution 대상은 단일-track `side`; 별도 frame으로 header 영향을 관찰한다. |
| 10 | rows positive에서 header가 row를 span하거나 child가 잘못된 row로 배치 | 통과. header는 첫 area row만 차지하고 side/content는 fixed second row를 사용한다. |
| 11 | container 자체 min/max-size가 track minimum보다 먼저 clamp | 통과. auto-minimum axis의 root min-size `auto`, max-size `none` 및 positive definite size를 보존한다. |
| 12 | aspect-ratio가 definite axis 또는 transferred size suggestion을 바꿈 | 통과. root aspect-ratio는 positive에서 제외하고 negative에서 명시 거부한다. |
| 13 | item overflow가 scroll container가 되어 automatic minimum을 0으로 바꿈 | 통과. visible만 허용하고 hidden/auto/scroll/clip negative를 요구한다. |
| 14 | item 고정 크기가 automatic minimum 기여를 재현하지 않음 | 통과. 80px contributing item과 40px neighbor의 min-size는 auto로 유지한다. |
| 15 | Grid border 밖 sample 좌표가 보이는 content 밖을 측정 | 수정 후 통과. 최초 계획 초안은 pixel 좌표를 정하지 않았으나, plan/evidence에 columns `(110,30)`, rows `(50,110)`와 auto `#70a8d8`/control `#ffffff`를 추가했다. |
| 16 | item paint 색상 또는 DPR 변환이 sample 비교를 모호하게 함 | 통과. 고정 CSS color와 DPR 1 Chrome reference를 두며 native 입력은 root-relative 좌표와 scale을 기록하게 했다. |
| 17 | ID/class를 성공 allowlist key로 사용하는 구현 | 통과. 이름 변경 positive control을 요구하고 지원은 computed style/tree shape에 묶는다. |
| 18 | CSS parser fallback과 computed unsupported input을 같은 실패로 처리 | 통과. malformed authored declaration의 cascade fallback과 node/property-bound typed rejection을 분리 검증한다. |
| 19 | 실패 중 일부 track/frame만 publication | 통과. unsupported placement/shape는 layout publication 전에 거부하고 frame/draw 부재를 요구한다. |
| 20 | test-only fixture가 기본 ABI/product 경로를 확장 | 통과. 별도 runtime profile·FFI·payload를 추가하지 않고 기존 기본 Root HTML route 및 artifact boundary를 재검증한다. |

결과: 최초 검토에서 surface sample의 정확한 좌표가 빠진 문제를 발견했다. 계획과 expected data를 수정한 뒤 20개 관점 전부 통과했다. 이는 계획 검토이며 구현/runtime 검토나 실제 PR diff 검토를 대체하지 않는다.
