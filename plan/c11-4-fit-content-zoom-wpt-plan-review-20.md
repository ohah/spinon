# C11.4 zoomed `fit-content()` 계획 실패 관점 검토 20개

검토 대상은 [zoomed fit-content WPT runtime 계획](./c11-4-fit-content-zoom-wpt.md), pinned source, [사전 비교 및 Stylo→Taffy source audit](../spec/internal/evidence/c11-4-fit-content-zoom-precomparison-2026-10-29/README.md)다. 소스 감사만 수행했고 Spinon 구현/runtime 검증은 하지 않았다.

| # | 실패 관점 | 확인·계획 반영 | 판정 |
|---:|---|---|---|
| 1 | Live WPT가 pin과 다른 revision일 수 있음 | Source revision과 SHA-256을 pins JSON에 고정했고 live fetch hash도 별도 기록한다. | PASS |
| 2 | Chrome 13/13을 Spinon 통과로 오인할 수 있음 | README는 upstream harness만 기록하고 runtime은 미실행으로 명시한다. | PASS |
| 3 | 다른 fit-content assertion을 대상이라 주장할 수 있음 | `zoomedGrid`와 assertion name을 정확히 지정하고 CSSOM expected `40px`을 고정한다. | PASS |
| 4 | authored zoom이 computed style에서 빠질 수 있음 | Stylo 0.22 specified/effective zoom을 같은 style revision에서 추출·검증하도록 한다. | PASS |
| 5 | specified zoom과 descendant effective zoom을 혼동할 수 있음 | Root specified/effective는 2, item·child는 1/2인 owner 행렬을 exact 검사한다. | PASS |
| 6 | CSS absolute lengths가 이미 Stylo에서 확대됐는지 놓칠 수 있음 | Stylo 0.22 absolute-length conversion과 Spinon grid `length.px()` projection을 source audit로 확인했다. Taffy에 전달된 후 재확대하지 않는다. | PASS |
| 7 | computed CSSOM 40px을 Taffy used track result로 오인할 수 있음 | CSS Viewport draft의 unscaled computed serialization과 zoom-scaled used lengths를 구분한다. WPT CSSOM은 40px, planned Taffy track은 80px이다. | PASS |
| 8 | auto minimum이 cap 결과를 바꿀 수 있음 | Item `min-height:0`을 exact input과 mutation control로 고정해 minimum contribution을 0으로 둔다. | PASS |
| 9 | 400px author child가 zoom 후 contribution 800px이 되는 것을 놓칠 수 있음 | Author value 400px과 effective-zoom Taffy contribution 800px을 별도로 기록한다. | PASS |
| 10 | Root height를 unscaled 300px과 visual 600px 사이에서 혼동할 수 있음 | WPT author/computed CSSOM input 300px과 zoomed runtime frame 600px을 나눠 적는다. | PASS |
| 11 | Width normalization이 row CSSOM 결과를 바꿀 수 있음 | Chrome original/60px normalized clone 모두 unscaled `gridTemplateRows:40px`이다. | PASS |
| 12 | Normalized case의 geometry를 누락할 수 있음 | Probe는 root, item, child original/normalized rect를 각각 보존한다. | PASS |
| 13 | Scale이 layout 이후 중복 적용될 수 있음 | Stylo typed length가 이미 zoomed Taffy input이라는 code audit를 반영하고 layout/frame 단계에서 재확대하지 않도록 요구한다. | PASS |
| 14 | Zoom 값을 inline source에서만 읽어 가짜 PASS를 만들 수 있음 | Computed specified/effective zoom과 typed track projection을 같은 revision으로 검사한다. | PASS |
| 15 | 지원 범위가 일반 CSS zoom으로 새어 나갈 수 있음 | Exact fixture route/feature에만 허용하고 nested/ancestor zoom과 다른 scene은 거부한다. | PASS |
| 16 | CSSOM result와 internal Taffy contribution trace가 섞일 수 있음 | WPT CSSOM `40px`과 Taffy limit/track/contribution `80/80/800px`을 별도 oracle로 유지한다. | PASS |
| 17 | diagnostic paint가 sizing input을 바꿀 수 있음 | Background-color만 사용하고 layout-affecting paint properties는 scene에서 금지한다. | PASS |
| 18 | Screenshot만으로 child overflow를 판정할 수 있음 | Android PixelCopy/iOS Metal fixed samples를 별도 acceptance gate로 둔다. | PASS |
| 19 | Fixture code가 product archive로 유입될 수 있음 | Default/product negative control과 fixture-test positive control을 모두 요구한다. | PASS |
| 20 | 한 WPT assertion이 C11.4 완료로 승격될 수 있음 | STATUS/roadmap은 계획 단계, 미실행 runtime, 미완료 checkbox와 공식 `72/213`을 유지한다. | PASS |

**결과:** corrected plan의 20개 경계를 재검토해 20/20 PASS. Source audit는 계획 입력을 바로잡았으며, 구현/runtime proof는 아직 없다.
