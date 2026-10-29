# C11.4 zoomed `fit-content()` 계획 실패 관점 검토 20개

검토 대상은 [zoomed fit-content WPT runtime 계획](./c11-4-fit-content-zoom-wpt.md), pinned source와 [사전 비교 evidence](../spec/internal/evidence/c11-4-fit-content-zoom-precomparison-2026-10-29/README.md)다. 구현을 실행한 검토가 아니다.

| # | 실패 관점 | 확인·계획 반영 | 판정 |
|---:|---|---|---|
| 1 | Live WPT가 pin과 다른 revision일 수 있음 | Source revision과 SHA-256을 pins JSON에 고정했고 live fetch hash도 별도 기록한다. | PASS |
| 2 | Chrome 13/13을 Spinon 통과로 오인할 수 있음 | README는 upstream harness만 기록하고 runtime은 미실행으로 명시한다. | PASS |
| 3 | 다른 fit-content assertion을 대상이라 주장할 수 있음 | `zoomedGrid`와 assertion name을 정확히 지정하고 expected value `40px`를 고정한다. | PASS |
| 4 | authored zoom이 computed style에서 빠질 수 있음 | Stylo snapshot에서 root computed zoom `2`를 같은 revision으로 확인하도록 수용 기준을 둔다. | PASS |
| 5 | root zoom과 descendant zoom 값을 혼동할 수 있음 | Root owner만 `zoom:2`를 허용하고 subtree frame scale을 별도 검증한다. | PASS |
| 6 | zoom-scaled used lengths를 unzoomed fixture coordinates와 혼동할 수 있음 | Grid formula는 normalized fixture coordinates에서 40px을 산출한다. CSS Viewport Editor's Draft는 used lengths에 zoom을 적용하고 computed style length를 unscaled로 반환한다고 구분한다. 계획은 normalized result 40px과 visual row 80px을 분리한다. | PASS |
| 7 | WPT가 검사하는 값과 app visual expectation을 섞을 수 있음 | WPT assertion은 row string 40px, visual geometry는 integration-only로 명시한다. | PASS |
| 8 | auto minimum이 cap 결과를 바꿀 수 있음 | Item `min-height:0`을 exact input과 mutation control로 고정한다. | PASS |
| 9 | 400px child가 max-content contribution을 만들지 않을 수 있음 | Child exact size와 `MaxContent 400px` contribution을 검사한다. | PASS |
| 10 | Root height를 indefinite로 처리할 수 있음 | `height:300px`과 same-revision computed value를 검사한다. | PASS |
| 11 | Width normalization이 row 결과를 바꿀 수 있음 | Chrome original/60px normalized clone 모두 row 40px인 probe를 저장했다. | PASS |
| 12 | Normalized case의 visual scale이 달라질 수 있음 | Item 120×80, child 120×800 값을 original과 probe에 따로 보존했다. | PASS |
| 13 | Surface origin 때문에 2x frame이 틀릴 수 있음 | Root origin을 `(0,0)`으로 정하고 root-local 2x frame 계약을 둔다. | PASS |
| 14 | Zoom이 inline 문자열만 읽어 가짜 PASS를 만들 수 있음 | Computed Stylo zoom을 snapshot/matcher에 요구하고 source-only bypass를 금지한다. | PASS |
| 15 | 지원 범위가 일반 CSS zoom으로 새어 나갈 수 있음 | Exact fixture route/feature에만 허용하고 ancestors/other scenes를 reject한다. | PASS |
| 16 | Taffy logical values와 rendered frames가 섞일 수 있음 | Unzoomed track list와 visual frame을 별도 field/검증으로 유지한다. | PASS |
| 17 | 고정 colors가 sizing에 영향을 줄 수 있음 | Background-color 진단만 허용하고 sizing tree/property는 별도 고정한다. | PASS |
| 18 | Screenshot만으로 child overflow를 판정할 수 있음 | PixelCopy/Metal fixed samples를 별도 acceptance gate로 둔다. | PASS |
| 19 | Fixture code가 product archive로 유입될 수 있음 | Default/product negative control과 fixture-test positive control을 모두 요구한다. | PASS |
| 20 | 한 WPT assertion이 C11.4 완료로 승격될 수 있음 | STATUS/roadmap에서 상위 checkbox와 공식 `72/213`을 그대로 유지한다. | PASS |

**결과:** 계획 경계 20/20 PASS, 구현/runtime 검증은 아직 하지 않았다.
