# C11.4 `grid-auto-fit-columns-001.html` 계획 실패 관점 검토 20개

구현 전에 전체 35-scene 계획, pinned WPT source, source-authored frame oracle, 독립 repeat-count 계산, Chrome observation, CSS Grid 2025 CRD 및 현재 fixture 경계를 대조했다. 각 관점은 별도의 실패 유형을 다루며 같은 검사를 말만 바꾸어 반복하지 않는다.

| # | 적대적 질문 | 실제 발견·조치 | 판정 |
|---:|---|---|---|
| 1 | WPT 원문의 일부 scene만 구현하고 35/35라고 할 수 있는가? | `expected.json` 재생성 결과 root 35개·direct item 37개다. 계획과 repeat-count oracle 모두 index 1–35를 요구하고 subset 축소를 금지한다. | 통과 |
| 2 | WPT 파일이나 helper가 바뀐 뒤 예전 기대값을 재사용할 수 있는가? | manifest에 WPT·grid helper·layout helper·harness 파일 hash가 고정되어 있다. 반복 계산기는 WPT hash, helper CSS hash와 실제 track-list declarations를 추가 검증한다. | 통과 |
| 3 | WPT frame 기대값이 Chrome 관측에서 역산되는가? | `derive_expected.py`는 authored `data-offset-*` 및 `data-expected-*`만 추출한다. repeat count는 CRD 규칙과 track 입력으로 독립 계산하고 Chrome CSSOM은 별도 cross-check로 둔다. | 통과 |
| 4 | `.grid { width:200px }`의 border를 content 폭으로 착각하는가? | 반복 count basis는 CSS width인 200px content box다. 2px border는 바깥에 더해져 Chrome 기본 root 외곽 폭은 204px이다. | 통과 |
| 5 | case 2의 13px padding을 200px에서 빼서 반복 수를 잘못 계산하는가? | 초안의 174px 가정은 잘못이었다. WPT box model과 Chrome root 폭 230px, item x=13px·width=180px을 대조해 고쳤다. content-box width와 repeat basis는 200px이며 padding은 content-box를 줄이지 않는다. | 통과 · 초안 오류 수정 |
| 6 | definite max가 min보다 작을 때 max 그대로 count를 계산하는가? | case 4의 `minmax(30px,13px)`와 case 5의 `minmax(30px,6.5%)`는 CRD의 max-floor-by-min 규칙으로 30px을 사용한다. case 3은 13px min보다 큰 30px max를 사용한다. | 통과 |
| 7 | prefix/suffix 또는 fixed `repeat(2,…)`를 한 track이나 무시된 항목으로 세는가? | case 9–14의 prefix `[10,40]`, case 21–25의 fixed prefix `[50,50]`, suffix sizing은 JSON에서 track별로 저장한다. fixed repeat는 두 track으로 확장되어 계산된다. | 통과 |
| 8 | multi-track pattern이나 반복 경계 gutter를 빠뜨리는가? | case 32–33은 2-track pattern `[32,20]`이다. case 32의 gap은 0px, case 33은 20px이며 두 경우 모두 pattern 내부와 copy 사이의 모든 gap을 각각 계산한다. 분수 기반 `required_size`는 track 전체 개수에서 gap 수를 센다. | 통과 |
| 9 | `2em` 크기를 입력 환경과 무관하게 임의로 32px로 박는가? | source authored value는 `2em`; Chrome CSSOM에서 case 32의 비어 있지 않은 2em track은 32px다. case 33의 해당 track은 auto-fit으로 collapse되므로 em basis를 추론하는 데 쓰지 않는다. plan은 case 32의 16px inherited WPT font basis를 fixture 입력으로 고정한다. | 통과 |
| 10 | auto-repeat count의 flexible sizing과 최종 `fr` 크기를 혼동하는가? | case 34–35의 `minmax(50px,1fr)`는 repeat count에는 definite 50px minimum을 사용하고, 최종 flex sizing은 별도 stage로 검사한다. | 통과 |
| 11 | auto-fit을 placement 전에 collapse하거나 spanning item이 차지한 track도 빈 것으로 처리하는가? | 전체 35 scene에서 count 이후 item placement, span occupancy, 그 뒤 empty-track 및 인접 gutter collapse 순서를 별도 matcher 불변식으로 요구한다. | 통과 |
| 12 | 반복 경계의 adjacent line-name groups를 합치지 않거나 중복 occurrence를 잃는가? | repeated pattern line names, fixed neighbors 및 repeated `start`/`foo`/`bar` names를 각 expanded line index와 occurrence별로 보존·대조하도록 계획에 넣었다. | 통과 |
| 13 | numeric, named, negative placement 또는 missing-name fallback을 class/scene ID로 대신 해석하는가? | 원문 `grid-column` 값과 DOM order를 그대로 adapter 입력으로 보존한다. case 11·14 등 implicit fallback 생성도 explicit repeat count와 분리한다. item ID/class를 CSS 의미 키로 쓰지 않는다. | 통과 |
| 14 | Chrome used `gridTemplateColumns`의 implicit column을 auto-repeat count로 잘못 세는가? | case 7, 8, 11, 14, 19는 157px `grid-auto-columns` implicit tracks가 생긴다. explicit repeat-count JSON과 used list 관측을 나누고 implicit count도 별도로 검사한다. | 통과 |
| 15 | grid root 밖으로 나가는 장면 중 overflow 표면 검사가 빠지는가? | WPT offset·width가 root content edge를 넘는 index 7, 8, 11, 14, 19, 31 여섯 장면을 Android/iOS surface matrix에 전부 넣었다. | 통과 |
| 16 | 경계 근처 screenshot만 보고 item이 테두리 바깥에서 그려진다고 결론내리는가? | 각 overflow scene에서 border stroke, border 바깥 item 내부, item 끝 직후 background, 별도 control을 좌표 변환과 기대 RGBA에 연결하도록 요구한다. border pixel은 item fill과 분리한다. | 통과 |
| 17 | WPT의 1px helper 허용치를 구현 acceptance로 그대로 사용해 오차를 숨기는가? | WPT source helper는 baseline 재현용이다. 앱 frame은 source-authored 정수 기대값에 0.02 CSS px를 적용하고 track count·line occurrence·collapse·RGBA는 정확 비교한다. | 통과 |
| 18 | 한 플랫폼 실행이나 geometry-only 결과로 전체 adapter 성공을 주장하는가? | 같은 source commit에서 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 모두 35/35 root 및 37 item을 검사한다. overflow 표면은 여섯 scene 전부 두 플랫폼에서 별도 채취한다. | 통과 |
| 19 | 고정 WPT adapter가 임의 author CSS로 새어 나가거나 제품 artifact에 들어가는가? | opt-in fixture-test feature 아래 exact source/topology/style allowlist와 미등록·변형 입력의 pre-layout rejection을 요구한다. 기본 및 opt-in static FFI archive를 각각 positive/negative control로 비교하고 APK/IPA 전체 제외로 확대하지 않는다. | 통과 |
| 20 | 계획·Chrome·simulator 결과가 C11.4 공식 완료 수 또는 일반 CSS 지원으로 승격되는가? | PR에는 계획과 oracle만 담는다. C11.4 checkbox 및 공식 72/213을 유지하고, Chrome baseline·AVD/Simulator·하드웨어 GPU·실기기·compositor 이후 한계를 분리해 기록한다. | 통과 |

**계획 실패 관점 검토:** 20/20 통과, 미해결 0. 초안에서 발견한 case 2 padding 오류는 oracle 및 WPT box geometry로 수정한 뒤 관련 계산과 문서를 다시 대조했다. 이 계획 검토는 구현/runtime 20회와 계획 PR diff 20회 검토를 대신하지 않는다.
