# C11.4 auto-repeat 재계산 구현·runtime 검토 · 20개 관점

현재 구현, projection/layout feature 경계, 단일 `HostDocument` 테스트, 최종 Android/iOS simulator 실행을 서로 다른 실패 관점에서 검토했다. 10월 27일 최종 캡처를 기준으로 기록한다. review 중 발견한 범위 초과 허용은 WPT item 폭 4개만 받도록 좁히고 90px negative control로 재확인했다.

| # | 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | fixture 전용 absolute probe 코드가 기본 제품 artifact에 들어간다. | `c11-4-grid-track-sizing-fixture-test` cfg가 probe adapter와 static-position 예외를 감싼다. default layout archive에는 probe symbol이 없고 fixture positive control에는 있다. |
| 2 | 알려지지 않은 scene ID가 새 입력을 임의 생성한다. | Rust scene table과 Android/iOS allowlist는 고정 8개 ID만 선택한다. unknown ID는 source lookup에서 `None`이며 fixture source test가 확인한다. |
| 3 | WPT의 상태 순서가 바뀌거나 일부 상태가 빠진다. | runtime `STATES`와 app scene 목록은 max-content 100·80·15·120, min-content 100·80·15·120 순서다. 두 플랫폼 최종 결과에 각 ID가 정확히 한 번씩 있다. |
| 4 | 서로 다른 문서를 만들어 revision 변경처럼 보인다. | Rust 테스트는 `Document` 하나의 `HostDocument`, owner, root·item·probe handles를 8회 재사용한다. generation 불변과 증가한 document/render-tree revision을 검사한다. |
| 5 | style revision이 재계산과 무관하게 고정되거나 역행한다. | 초기 `StyleRevision`에서 상태 전환마다 `checked_next()`를 쓰고 각 결과 trace의 style revision과 비교한다. |
| 6 | 문서 mutation은 기록되지만 실제 dirty subtree가 생성되지 않는다. | 이전 snapshot을 invalidation 분류에 전달하고 매 mutation 후 `dirty_roots()` 비어 있지 않음을 검사한다. |
| 7 | incremental 경로가 아니라 매 상태 full layout만 실행한다. | 첫 요청만 `force_full=true`; 다음 일곱 상태는 이전 snapshot과 이전 style roots를 전달한다. |
| 8 | 마지막 입력의 trace를 이전 layout revision의 결과로 오인한다. | 각 단계 key와 diagnostics source/style revision을 해당 snapshot·style revision에 대조하고 source revision 재사용을 금지한다. |
| 9 | auto-repeat 개수 계산에서 trailing fixed-max track을 repeat로 센다. | 마지막 Columns pass의 non-gutter/non-collapsed used tracks에서 단일 suffix 하나를 뺀 수를 1·2·3·4로 검사한다. |
| 10 | 결과 track 값만 맞고 ordering 또는 suffix 크기가 틀린다. | 각 상태의 전체 최종 used-column sequence를 독립 expected array와 비교한다. `min-content` 15px은 `[20,10]`, 나머지 차이 상태도 별도 기대값이다. |
| 11 | max-content item 폭 축소 후 이전 repeat 수가 캐시된다. | 100→80→15→120 전환에서 최종 columns와 probe geometry가 각각 바뀌며 양 플랫폼 로그가 모두 새 state 기대값과 일치한다. |
| 12 | min-content의 최소 폭 제한이 max-content 결과와 혼동된다. | 15px 상태에서 max-content root content width 60px, min-content root content width 30px이고 최종 track `[20,40]` 대 `[20,10]`을 검사한다. |
| 13 | root border/padding offset이 item frame 기대값에 섞인다. | root border box와 item을 padding-edge-relative 좌표로 따로 비교한다. item은 `[0,10,width,30]`, root 높이는 44px이다. |
| 14 | absolute probe가 track contribution 또는 repeat count에 참여한다. | Android/iOS trace matcher는 contribution item이 in-flow node 2뿐인지 확인한다. probe의 line geometry는 결과 frame으로 따로 비교한다. |
| 15 | WPT 외 absolute probe도 제한 예외를 통과한다. | root style, auto-fill column fragment, trailing minmax, child 개수/순서, placement lines, inset, 크기, edges를 fail-closed 검사한다. named line·gap·reorder·hidden·잘못된 probe controls가 거부된다. |
| 16 | probe 예외가 테스트한 WPT보다 임의의 item 폭까지 허용한다. | 검토에서 발견해 item width를 15·80·100·120px로 제한했다. 새 90px negative control과 positive matrix test가 통과했다. |
| 17 | 0px probe의 정상 frame이 그려진 box 수와 혼동된다. | node frame 목록에는 width 0인 4/5 line probe를 보존하고, Android·iOS·JS matcher의 painted box 수에는 양수 폭 probe만 포함한다. 80px·15px 상태가 이 경계를 실행한다. |
| 18 | root content origin과 absolute containing-block origin을 다르게 계산한다. | runtime은 root border offset을 뺀 좌표로 네 probe를 비교하고 앱 matcher는 같은 border offset을 다시 더해 native frame을 비교한다. |
| 19 | 한 플랫폼만 통과하거나 새 앱 hash가 설치물과 다르다. | 최종 Android API 37 AVD와 iPhone 17 Pro/iOS 26.2에서 각각 8/8이고, 설치 APK/app hash가 빌드 산출물 hash와 일치한다. |
| 20 | simulator screenshot을 실기기, pixel-perfect, 또는 C11.4 전체 증거로 확대한다. | evidence는 두 simulator의 frame/track/trace/draw submission만 주장한다. 실기기, Android hardware GPU, iOS compositor 이후 pixel, 일반 CSS admission, 전체 WPT와 C11.4는 명시적으로 남긴다. |

실행 상세와 source/app hash는 [README](README.md)에 있다. 이 검토는 한 명의 검토자가 분리한 20개 관점 기록이며, 외부 독립 reviewer 승인을 뜻하지 않는다.
