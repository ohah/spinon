# C11.4 cyclic max-content mixed text/image span PR diff 검토 30개

PR [#526](https://github.com/ohah/spinon/pull/526)의 코드, fixture, Android/iOS 검증, evidence, 계획·상태·roadmap 및 PR 표시 자료를 서로 다른 실패 경로 30개로 대조했다. 계획 검토 20개와 구현/runtime 검토 20개를 재사용하지 않았다. 수정한 항목은 해당 관점에서 다시 확인했다.

| # | 독립 실패 관점 | 확인 및 판정 |
| ---: | --- | --- |
| 1 | test-only fixture가 default product feature로 편입되는가 | 모듈 연결이 opt-in fixture feature 뒤에 있고 product-core static archive에서 새 scene·source·host fixture symbol이 빠진다. positive fixture archive에는 같은 marker가 있다. 통과. |
| 2 | 새 공개 FFI/API 계약이 문서 없이 추가되는가 | 변경된 ABI 선언·header·외부 API는 없다. fixture adapter와 test scene만 확장했다. 공개 API가 없다는 사유를 PR template과 plan에 적었다. 통과. |
| 3 | 허용되지 않은 scene ID가 폭넓은 접두사 처리로 받아들여지는가 | runtime source와 앱 검증은 기존 네 ID와 새 네 ID를 고정 목록으로 비교한다. wildcard fallback은 추가하지 않았다. 통과. |
| 4 | `max-content`와 `min-content` root sizing이 혼동되는가 | runtime adapter에 전용 `Width::MaxContent`가 있고 computed dimension도 `MaxContent`로 검사한다. 새 scene은 `width:max-content`를 유지한다. 통과. |
| 5 | definite control에 max-content width mode가 새어 들어가는가 | `max-content-` ID 판별과 `40px` scene 입력을 대조했다. definite 두 장면의 root·tracks·trace 결과가 별도 기대값과 일치한다. 통과. |
| 6 | 역순 장면이 DOM 순서 이외의 CSS 입력도 바꾸는가 | A/B/C item의 ID, text·image, placement와 spans는 고정이고 child 순서만 반전한다. 통과. |
| 7 | 역순 DOM의 source order와 node identity가 뒤바뀌는가 | source order input 배열, Android/iOS trace의 node IDs와 Rust identity별 frames를 서로 대조했다. A/B/C mapping이 일관된다. 통과. |
| 8 | 세 item의 column span과 row span 정의가 서로 다른가 | A/B는 columns span 2, C는 span 3, 세 item 모두 rows span 2다. FFI source와 Rust case 입력에서 같다. 통과. |
| 9 | root border·gap이 content track 기대값에 잘못 포함되는가 | max-content root content width `110px`에 2px gap 두 개와 좌우 border 4px를 더해 border box `114px`가 된다. definite `40px` content width는 border box `44px`다. 통과. |
| 10 | inline max-content contributions가 min-content 값과 섞이는가 | A/B/C의 Columns Minimum·MinContent·MaxContent contribution을 item·span별로 분리해 Rust trace와 Android/iOS 진단에서 확인한다. 통과. |
| 11 | Ahem text의 max-content line width가 frame width로 오인되는가 | A=`60px`, C=`110px` intrinsic width와 item frame을 별도 필드로 검사한다. 글리프 paint를 증명한다고 표현하지 않는다. 통과. |
| 12 | replaced image의 natural size·inline contribution이 불안정한 네트워크 입력에 의존하는가 | 고정 URL과 저장소 24×16 PNG bytes를 사용하고 resource request의 src·decode를 검증한다. CSS width 40px에서 inline contribution은 40px다. 통과. |
| 13 | image aspect ratio의 transferred block contribution이 Rows에 도달하지 않는가 | `40 × 16/24 = 80/3px`을 image contribution과 Rows span-2 trace에서 확인한다. 통과. |
| 14 | cyclic Columns first pass가 resolved track 결과로 오인되는가 | 첫 cyclic pass final `[24,34,48]`과 resolved Columns `[107/3,125/3,86/3]`를 pass index와 별도 Initialize stage로 비교한다. 통과. |
| 15 | percentage minimum이 cyclic intrinsic width 기준으로 resolve되지 않는가 | max-content inner width 110px에서 `10% + 2px = 13px`이며 resolved Columns Initialize `[13,13,13]`을 검사한다. 통과. |
| 16 | column sizing 이후 cyclic Rows initial basis가 다른 축의 값으로 계산되는가 | resolved Rows Initialize `[14/3,14/3]`를 확인하고 image contribution 뒤 최종 Rows `[37/3,37/3]`과 분리했다. 통과. |
| 17 | cyclic/resolved axis pass 순서나 횟수가 바뀌어도 검사가 통과하는가 | Rust test와 두 native validator가 max-content에 Columns→Rows→Columns→Rows 네 pass를 요구하고 definite control의 pass sequence도 별도로 검사한다. 통과. |
| 18 | final track 배열 길이·순서가 잘못되어도 합계만으로 통과하는가 | 세 columns와 두 rows를 각각 배열 순서대로 1/64 CSS px 허용오차 내 비교한다. pass stage에서 gutter·collapsed track은 제외한다. 통과. |
| 19 | A/B/C frame이 source index에만 연결되어 overflow 위치가 틀리는가 | root frame과 각 stable node ID의 x/y/width/height를 검증한다. max-content B x=`119/3`, definite B x=`16`이다. 통과. |
| 20 | root를 벗어나는 definite image가 clip되거나 위치가 달라지는가 | B는 x=16px, 폭 40px로 border 끝 44px을 넘어 12px 밖까지 뻗는다. Android/iOS 화면과 50px image sample이 이를 보여준다. 통과. |
| 21 | direct text child 수·text source가 scene마다 어긋나는가 | scene source에 direct text node 두 개와 image 한 개, 고정 Ahem text 두 입력이 있다. FFI child count와 text source 검사가 통과한다. |
| 22 | image resource가 다른 item ID에 연결되거나 두 번 붙는가 | 네 신규 root ID마다 정확히 하나의 item B image ID를 허용한다. direct child count, image order, URL, decode와 render resource snapshot을 fail-closed로 검사한다. 통과. |
| 23 | Android가 다른 scene allowlist의 trace를 재사용하는가 | Android scene dispatch, pass 축, stage 이름, span 순서, contribution kind/value와 stable node IDs가 이 8개 mixed scene 기준으로 일치한다. 통과. |
| 24 | iOS validator가 Android와 다른 geometry나 stage semantics를 기대하는가 | iOS도 동일한 네 scene geometry·track·contribution·cyclic Initialize 값을 검사한다. Swift tolerance는 1/64 CSS px다. 통과. |
| 25 | malformed/missing trace JSON이 0이나 기본값으로 허용되는가 | trace count, grid ID, pass count, axis, stage, track fields, finite values와 contribution identity가 빠지면 validator가 실패한다. 통과. |
| 26 | simulator capture 기대값이 native validator와 엇갈리는가 | JS fixture는 `maxContentWidth` boolean과 `maxContent` contribution map을 별도 필드로 둔다. 실제 네 run log의 axes·frames·samples와 대조했다. 통과. |
| 27 | 전역 fixture scene 목록이 새 장면을 놓치거나 ID를 중복하는가 | capture matrix count가 275에서 279로 증가하고 global unique-ID assertion이 남아 있다. 네 신규 ID가 요청/실행 목록에 모두 포함된다. 통과. |
| 28 | overflow sample이 전체 border 또는 glyph raster까지 증명한다고 과장되는가 | 표본은 top border `(8,0)`, PNG `(50,6)`, clear `(58,6)` 세 지점만 뜻한다. README와 PR은 전체 stroke·glyph raster를 검증하지 않았다고 제한한다. 통과. |
| 29 | 성공 실행 자료가 실패한 재시도와 섞이거나 platform/device 근거가 모호한가 | 성공 Android API 37 ARM64 AVD·iPhone 17 Pro/iOS 26.2 Simulator의 결과 JSON, install/build 해시와 4 scene log를 각 성공 run folder에서 확인했다. 실패 원인은 재시도 섹션에 따로 남겼다. 통과. |
| 30 | PR 화면·상태 문서가 잘못된 링크나 완료 주장으로 증거를 훼손하는가 | PR 생성 때 본문 raw image URL의 추정 SHA가 HTTP 404였다. 실제 head `a05b1d7d0f9becab02dde015de3267ab1a212f4a`로 고친 뒤 네 URL은 HTTP 200이고 Agent Browser에서 이미지 4/4 `complete`·`rendered`였다. PR 화면을 캡처하고 브라우저 세션·프로세스를 닫았다. 상태 문서는 C11.4·`77/213` 미완료와 시뮬레이터/정적 archive 한계를 유지한다. 수정 후 재확인 통과. |

## 판정

코드·실행·문서 관점에서 수정 후 남은 차단 결함은 찾지 못했다. PR #526은 검토 완료 시 `CLEAN`이었고 status-check rollup은 비어 있었다. GitHub branch-protection API는 `main` required status checks가 설정되지 않았다고 응답했다. Android 실기기/hardware GPU, iOS compositor 제출 이후 pixels, WPT, glyph raster와 일반 CSS admission은 여전히 이 fixture 결과의 범위 밖이다.
