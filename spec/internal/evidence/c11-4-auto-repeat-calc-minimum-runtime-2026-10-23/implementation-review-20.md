# C11.4 auto-repeat calc 최소값 구현/runtime 실패 관점 검토 20개

이 표는 고정 fixture implementation과 실제 simulator runtime을 새로 검토한다. 기존 계획 검토 20개와 계획 수정 검토 20개, PR diff 검토 30개를 재사용하지 않는다.

| # | 독립 실패 관점 | 코드·실행 대조와 판정 |
| --- | --- | --- |
| 1 | 잘못된 최초 가설 때문에 실제 제품 알고리즘을 불필요하게 수정한다. | cascade test에서 Stylo의 결과가 LengthPx(40)이며 control과 같음을 확인했다. product style/layout 알고리즘은 변경되지 않았다. |
| 2 | 새 계산 코드를 앱 product runtime으로 실수로 추가한다. | 변경된 Rust 코드는 opt-in fixture adapter와 cfg(test) cascade fixture뿐이다. cargo clippy와 simulator app 빌드에서 새 evaluator 경로는 나타나지 않는다. |
| 3 | 같은 CSS 식의 computed track이 control과 달라도 test가 통과한다. | four axis/mode case에서 전체 typed track vector와 명시적 MinMax { LengthPx(40), Fraction(1) }를 비교한다. 2/2 cascade tests가 통과했다. |
| 4 | 퍼센트 calc가 절대 길이로 잘못 허용된다. | calc(20px + 5%) test는 UnsupportedComputedGridValue, root node, grid-template-columns를 동시에 요구한다. 통과했다. |
| 5 | rows/columns 중 하나가 fixture route 또는 style normalization에서 빠진다. | 각각 fill·fit 네 조합을 cascade에서 확인하고 양축 simulator 결과에서 expected node frames와 tracks가 모두 일치했다. |
| 6 | auto-fill이 item 수보다 적은 track을 만들어 implicit rows/columns로 밀어낸다. | 184/40 repeat count는 네 개이며 네 in-flow item이 네 track에 배치됐다. 두 플랫폼에서 4개 item frame과 used tracks가 맞았다. |
| 7 | 1fr free-space distribution이 최소값 40px에 고정된다. | 네 개 최소 track의 남은 24px을 네 fraction이 나눠 46px이 된다. calc와 fixed control 모두 네 46px track으로 관측됐다. |
| 8 | auto-fit collapse를 placement 이전에 계산하거나 occupied track을 접는다. | 한 item이 첫 track을 점유하고 뒤 세 track만 접혔다. 양 플랫폼에서 tracks [184,0,0,0], collapse [false,true,true,true], item 184px를 확인했다. |
| 9 | auto-fill에서 빈 track을 잘못 접는다. | 네 item이 모든 repeat track을 점유한다. 양 플랫폼 결과의 네 collapse flag는 모두 false다. |
| 10 | rows의 auto-placement 방향이 columns 경로와 겹친다. | rows scene은 grid-auto-flow:column을 설정한다. 결과는 네 개의 vertical 48×46 item과 rows [46,46,46,46]이다. |
| 11 | root sizing 값이나 반대 축 fixed track이 잘못되어 주 축 외 geometry가 달라진다. | JSON의 root 184×48/48×184와 fixed cross-axis 48px가 양 플랫폼 root/frame 관측과 일치한다. |
| 12 | screenshot이 맞아 보이지만 node order나 item coordinate가 어긋난다. | 앱 report의 전체 node ID/index/x/y/size를 independent JSON geometry와 비교해 12개 run 모두 통과했다. |
| 13 | 앱 report의 box count만 맞고 node frame 목록이 일부 누락된다. | fill/control은 root+4 items=5, fit은 root+1 item=2로 검사되며 실제 boxes와 observed nodes가 각각 일치한다. |
| 14 | 앱별 helper에서 calc scene과 fixed control을 다르게 처리한다. | Java와 Swift가 동일 scene-ID map, root/frame/item/node/count expectation을 제공했고 두 시뮬레이터에서 같은 결과를 냈다. |
| 15 | 런타임 JS가 fixture dimensions 대신 다른 CSS 값을 적용한다. | opt-in runtime source는 JSON의 width/height/gap/template/item 데이터를 사용한다. 생성 scene ID와 node ID가 capture report와 일치했다. |
| 16 | 런타임 fixture code 또는 payload가 기본 product FFI archive에 섞인다. | 기본 c04-runtime-gpu release archive raw-byte scan에서 새 module marker와 6 ID 전부 absent다. |
| 17 | negative artifact scan만 성공하고 positive control이 fixture를 실제로 포함하지 않는다. | c04-runtime-gpu-fixture-test archive raw-byte scan에서 module marker와 6 ID가 모두 present다. |
| 18 | simulator build가 stale APK나 stale iOS app으로 scene을 통과했다고 오인한다. | 첫 Android 시도는 stale APK가 scene ID를 거부했다. 최신 Android APK와 재빌드한 iOS Simulator binary SHA를 run metadata와 대조한 뒤 최종 6/6씩 통과했다. |
| 19 | simulator screenshot을 pixel/compositor 보증으로 확대 해석한다. | 새 scene에는 pixel sampling을 지정하지 않았다. screenshots는 layout 표시 기록이며 PixelCopy/Metal exact count를 보고하지 않는다. |
| 20 | fixture scene 일부의 draw 또는 presentation이 끝나지 않았는데 matrix를 성공 처리한다. | Android와 iOS 여섯 scene 모두 layout=ready, track trace 완료, drawPresented=true, supersededRetries=0을 기록했다. |

## 최종 판정

고정 pure-absolute calc expression은 현재 cascade와 C11.4 simulator fixture 경로에서 fixed 40px control과 동등했다. percentage 계산, 다른 calc 식, pixel-level equality, 실기기 및 전체 C11.4는 이 검토에서 통과 처리하지 않는다. 상세 원본은 [실행 근거](./README.md), [Android 결과](./android-avd-c114-absolute-calc-minimum-android-avd-20261023T075907042Z-3781412e/result.json), [iOS 결과](./ios-simulator-c114-absolute-calc-minimum-ios-simulator-20261023T080040579Z-bb22adde/result.json)에 있다.
