# C11.1 named-area percentage · 구현/runtime 실패 관점 검토 20개

검토 대상은 `1e73158` 구현과 이후 property 진단 수정, 기본 Root HTML Rust 경로 및 Android/iOS Simulator 실행이다. 기존 계획 검토표는 재사용하지 않았다.

| # | 독립 실패 관점 | 확인과 결과 |
| ---: | --- | --- |
| 1 | 행과 열 percentage가 서로 다른 definite basis를 쓰는지 | 120×70 root에서 `[30,90]` columns와 `[14,56]` rows를 Rust frame에서 대조했다. 통과. |
| 2 | 한 축의 fixed px와 percentage 혼합이 direct size를 보존하는지 | `40px 50%`와 `20% 50px`에서 `[40,60]`·`[14,50]` 및 area frames를 확인했다. 통과. |
| 3 | percentage가 없는 named-area fixed-px 경로가 바뀌는지 | 기존 projection/runtime regression을 재실행했다. 통과. |
| 4 | 기존 양축 `1fr 2fr` named-area 경로가 percentage 조건에 흡수되는지 | 기존 named-area fractional regression 5건을 재실행했다. 통과. |
| 5 | `%`와 `fr` 혼합이 실수로 직접 track admission을 통과하는지 | projection과 runtime rejection에서 `25% 1fr`를 거부했다. 통과. |
| 6 | 2개를 넘는 explicit track이 area profile에 들어오는지 | 기존 named-area candidate rejection과 exact direct-list matcher를 확인했다. 통과. |
| 7 | track list가 빈 경우 CSS 기본값을 성공 입력으로 오인하는지 | direct track validator의 empty-list 거부와 실패 시 `completed=None`을 확인했다. 통과. |
| 8 | 음수·NaN·무한 percentage가 Taffy에 전달되는지 | Root HTML track validation에서 finite/non-negative 확인 후 전달하고, projection negative matrix를 재실행했다. 통과. |
| 9 | `0%` track의 layout frame이 paint box 부재 때문에 사라지는지 | Rust snapshot은 side frame 폭 0을 확인한다. 각 앱은 정상 ready이며 paint box 5개를 제출한다. 통과. |
| 10 | `100%` 및 `0%` 합을 특별 처리해 layout을 왜곡하는지 | zero-track 입력 `[0,120]`과 rows `[14,56]`을 runtime 및 양 플랫폼에서 확인했다. 통과. |
| 11 | 100%를 넘는 입력을 100%로 clamp하는지 | `120% 10%`의 `[144,12]`, blue item x=144와 header 폭 156을 확인했다. 통과. |
| 12 | track aggregate가 f32 범위를 넘어 partial layout을 만드는지 | 기존 percentage track/range overflow rejection matrix가 `grid-template-columns` 진단과 무결과 publication을 확인한다. 통과. |
| 13 | percentage basis가 indefinite일 때 추정 폭/높이를 쓰는지 | named-area percentage width·height definite basis 부재를 거부하도록 runtime rejection을 확장했다. 통과. |
| 14 | min/max constraints가 허용된 definite basis를 조용히 제한하는지 | 네 min/max override를 각각 거부하고 실제 위반 property를 진단하도록 수정했다. `max-height`의 기존 `min-width` 오진을 발견·수정 후 재검사했다. |
| 15 | `aspect-ratio`가 percentage 기준 축을 바꾸는지 | named-area percentage 입력의 `aspect-ratio` override를 cascade 진단과 부분 scene 부재까지 확인했다. 통과. |
| 16 | `calc()`·`minmax()`가 direct value처럼 우회되는지 | 두 grammar를 제한 Root HTML profile에서 거부하고 cascade/property diagnostic을 확인했다. 통과. |
| 17 | CSS-invalid 음수 declaration이 computed zero track으로 오인되는지 | 음수 column declaration이 profile의 exact two-track 입력을 만들지 못해 scene publish 전에 거부된다. 통과. |
| 18 | area map의 row/column topology가 올바르지 않아도 이름만으로 통과하는지 | exact 2×2 map과 세 rectangle의 semantic topology validator 및 기존 area negative cases를 확인했다. 통과. |
| 19 | child source order, class 순서 또는 DOM ID가 named area 결과를 바꾸는지 | 입력 source order `content,header,side`와 semantic frame lookup을 대조하고, runtime source order regression을 재실행했다. 통과. |
| 20 | 앱이 새 fixture ABI/scene 경로를 타거나 화면만으로 geometry를 과장하는지 | 기본 `index.html` route, fixture-test 비활성 빌드 로그, HTML hash, ready/draw 로그와 Rust geometry test를 분리 기록했다. PixelCopy/Metal readback은 없다고 명시했다. |

**발견 및 수정:** named-area candidate는 네 min/max 조건 중 무엇이 깨져도 `min-width`를 돌려주고 있었다. 각 computed property를 별도 검사하고 실패 속성을 그대로 반환하도록 수정했다. 모든 위반 case는 부분 layout을 발행하지 않으며, 최종 runtime matrix는 Rust `root_html_grid` 24/24 통과다.

이 검토는 하드웨어 GPU·물리 Android/iOS 기기, Surface PixelCopy/Metal readback, 전체 CSS Grid grammar, WPT runner 적합성을 증명하지 않는다.
