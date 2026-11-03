# C11.1 Root HTML `fit-content(px)` PR diff 적대 검토 30개

검토일: 2026-11-03
PR: [#1093](https://github.com/ohah/spinon/pull/1093)
구현 대상: `f8611f12e` · 기준: `0232adc24652e19a07b275c24f198b6f37c023f4`

구현/runtime 20개 검토와 별도로 실제 PR 변경을 30개 경계에서 다시 확인했다. 코드·typed 값·실패 경계는 소스와 고정 입력을 대조했고, 실행 결과는 해당 항목에 명시했다.

| # | 실패 관점 | 대조 및 판정 |
|---:|---|---|
| 1 | Opt-in이 Root HTML 이외 cascade profile로 새는가 | profile 옵션은 `RuntimeRootHtmlTextPaintV1`에서만 켜진다. 기존 다른 Grid profile suite를 유지한다. 통과. |
| 2 | 음수·NaN·무한대 px limit가 typed 값으로 남는가 | cascade와 projection 모두 finite non-negative 검사를 적용한다. negative matrix에 거부 대조가 있다. 통과. |
| 3 | `fit-content(0px)`가 양수 조건으로 잘못 거부되는가 | 0px는 유효하고 track 검증과 Ahem layout 경로에서 보존한다. 통과. |
| 4 | percentage 또는 `calc()` limit가 px처럼 취급되는가 | cascade와 projection acceptance matrix가 `%` 및 `calc()`를 거부한다. 통과. |
| 5 | 정수 고정 `repeat()`가 직접 track과 다른 검증을 우회하는가 | 확장된 각 track에 같은 limit 조건을 적용하고 고정 repeat case를 검사한다. 통과. |
| 6 | auto-repeat 또는 named line이 허용 범위에 들어오는가 | 기존 auto-repeat·line-name 거부 경로를 유지한다. 통과. |
| 7 | `grid-auto-rows`·`grid-auto-columns`가 explicit opt-in을 물려받는가 | 새 option은 explicit track conversion에만 전달한다. implicit `fit-content()`는 거부된다. 통과. |
| 8 | direct track 경로가 unrelated Root HTML Grid를 잘못 바꾸는가 | fit-content track이 있는 Root HTML에서만 새 placement mode를 선택한다. 기존 Root HTML 회귀 suite를 유지한다. 통과. |
| 9 | named-area tracks에 `%`, `fr`, `auto` 또는 `minmax()`가 섞여 입장하는가 | 새 area helper는 finite px와 px `fit-content()`만 받고 fit track 하나 이상을 요구한다. 거부 matrix가 있다. 통과. |
| 10 | named-area map의 행·열 수와 explicit track 수가 다른가 | 두 축 크기를 각각 비교하며 projection/runtime 불일치 controls가 거부된다. 통과. |
| 11 | parsed area map의 빈 이름·중복 이름·잘못된 rectangle가 사용하는가 | semantic gate는 빈/NUL/중복 이름을 검사하고 CSS parser가 rectangle/range 정합성을 보장하는 경계를 대조했다. 통과. |
| 12 | area 이름이 `fixed`/`fit` 또는 DOM id에 고정되는가 | 임의 이름, 변경한 selector/id, 역순 source item case가 동일한 frames를 낸다. 통과. |
| 13 | area 하나에 item이 중복되거나 item이 빠져도 scene이 끝나는가 | 중복·missing·unknown area controls가 diagnostic과 scene 부재를 확인한다. 통과. |
| 14 | 중첩·비어 있지 않은 요소·익명 비공백 텍스트가 semantic gate를 우회하는가 | 빈 HTML `div` direct child만 받고 나머지는 rejection matrix에서 거부된다. 통과. |
| 15 | `display:none` item이 visible area item으로 취급되는가 | hidden direct child control은 unsupported 진단을 내고 완료 scene을 만들지 않는다. 통과. |
| 16 | auto/0 크기 또는 min/max override가 area 경로에서 잘못 처리되는가 | definite positive px 두 축과 기본 min/max만 허용하며 negative matrix로 검사한다. 통과. |
| 17 | 새 area 경로의 percentage gap이 indefinite axis에서 해석되는가 | 기존 definite-axis gap validator를 재사용한다. 새 경로의 유효 percentage gap runtime case가 통과한다. |
| 18 | 기존 named-area profile의 0이 아닌 gap 거부 계약이 풀리는가 | 기존 `row-gap: 1px` negative control이 실패를 요구한다. 검토 중 guard를 잠시 제거하면 이 control과 다른 두 rejection control이 scene을 발행해 실패했다. 원래 guard를 유지했고 해당 negative control 재실행은 1/1 통과했다. |
| 19 | `fit-content()` min-content floor 또는 max-content cap이 틀리는가 | Ahem independent expected matrix가 cap 아래·사이·위 세 경우와 fixed repeat/row minimum을 검사한다. 통과. |
| 20 | column과 row sizing이 서로 뒤바뀌거나 한 축만 적용되는가 | Ahem rows/columns와 Root HTML direct row/column scenes를 각각 검사한다. 통과. |
| 21 | track item이 Root HTML container 밖에서 강제로 잘리는가 | runtime frames는 root 경계 밖의 fit child를 보존하고 Android/iOS 표면 표본도 안/밖 지점을 확인한다. 통과. |
| 22 | invalid scene 뒤에 이전 completed frame이 남는가 | 지원 밖 runtime cases는 completed scene이 없음을 확인한다. 통과. |
| 23 | layout과 typed style snapshot이 다른 revision에서 결합되는가 | Root HTML test는 layout/cascade key의 generation·document/style/environment revision 일치를 검사한다. 통과. |
| 24 | area metadata allocation 실패가 panic 또는 부분 scene으로 바뀌는가 | area-name/item/usage buffer가 `try_reserve_exact` 후 contextual error를 반환한다. 전용 allocator failpoint는 없으며 이 지점은 정적 검토다. |
| 25 | 새 영역에서 unsupported track이 일반 CSS 값으로 조용히 대체되는가 | cascade fallback/negative cases는 property-bound unsupported 진단과 부분 frame 부재를 확인한다. 통과. |
| 26 | Android capture가 stale surface generation 또는 다른 크기의 화면을 읽는가 | callback 전후 generation, renderer, surface size를 재확인하고 density/viewport를 기록한다. 실행 log와 APK hash가 연결된다. 통과. |
| 27 | Android에서 minmax capture와 fit-content capture가 동시에 dispatch되는가 | 두 intent가 충돌하면 각각의 requested state를 배제하고 오류를 남긴다. 기존 capture 분기와도 대조했다. 통과. |
| 28 | Android 일반 실행에서 fixture capture가 기본 동작으로 켜지는가 | 전용 BuildConfig flag와 명시적 intent가 모두 있어야 하며 기본 경로는 영향을 받지 않는다. build log와 실행 log를 대조했다. 통과. |
| 29 | iOS 임의 argument가 Root HTML default route 또는 다른 capture로 섞이는가 | 전용 compile flag, 단일 허용 argument, run id 형식 검사를 통과해야 한다. simulator build와 capture log를 대조했다. 통과. |
| 30 | PR 화면과 문서가 simulator evidence를 device/전체 Grid 지원으로 과장하는가 | Agent Browser에서 PR #1093과 두 embedded image를 확인했다(이미지 natural size Android 1080×2209, iOS 1206×2622). AVD software renderer, compositor 전 Metal readback, 미완료 C11과 `72/213` 한계를 본문에 유지한다. 통과. |

결론: 검토 후 남은 차단 결함은 없다. 검토 중 발견한 gap 의심은 기존 negative test와 대조해 false positive로 판정했으며 fail-closed guard를 유지했다. PR 화면의 embedded 이미지 두 장은 모두 실제로 로드됐다.
