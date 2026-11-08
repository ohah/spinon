# C11.4 반복 intrinsic tail 30순서 구현 PR diff 실패 관점 검토 30개

검토 범위는 Style projection·Layout admission·RuntimeSession 회귀·fixture 및 상태/로드맵 문서의 현재 후보 diff다. 기준 HEAD는 `5428aee676358b71b6ddf5f8d0da6697afb7e469` (`origin/main`)이다. 구현/runtime 20관점 검토와 분리해 diff의 경계·회귀·주장 범위를 점검했다.

| # | 실패 관점 | 판정과 근거 |
| --- | --- | --- |
| 1 | 기존 PR #1284 순서가 빠져 회귀하는가 | 통과. `min max auto min max`를 30순서 집합에 명시적으로 포함하고 projection·layout·RuntimeSession matrix에서 확인한다. |
| 2 | 첫 `fit-content`가 아닌 다른 위치를 허용하는가 | 통과. 두 admission helper 모두 길이 6과 `tracks[0]`의 유효 cap을 먼저 요구한다. near-miss projection/layout 테스트는 위치 이동을 거부한다. |
| 3 | 목록 길이 5 또는 7도 후행 multiplicity만으로 허용하는가 | 거부. exact length 6 검사와 Style projection의 short/long negative cases가 있다. |
| 4 | 후행 track의 허용 타입 밖 `fit-content`, 고정 길이, percentage, `minmax`가 통과하는가 | 거부. 후행 member match는 `min-content`, `max-content`, `auto`만 허용하며 다른 variant에서 즉시 false다. |
| 5 | `min-content` 수가 1개 또는 3개가 되어도 허용되는가 | 거부. 후행 count가 정확히 `[2, 2, 1]`이어야 하고 projection/layout에서 바뀐 multiplicity를 거부한다. |
| 6 | `max-content` 수가 바뀌어도 허용되는가 | 거부. 같은 고정 count 검사에서 정확히 2개를 요구한다. |
| 7 | `auto` 누락 또는 중복이 허용되는가 | 거부. count 배열의 세 번째 값이 정확히 1이어야 한다. |
| 8 | 유효하지 않은 cap이 반복 profile을 통과하는가 | 거부. 기존 finite·non-negative px/direct percentage cap helper를 재사용하고 음수·NaN·infinity를 negative test로 유지한다. |
| 9 | 제한 profile이 0%나 120%를 잘못 잘라내는가 | 통과. 30순서 모두 px, 20%, 0%, 120% cap의 typed projection을 검사한다. 이 값은 CSS 전역 적합성 주장이 아니다. |
| 10 | percentage-bearing `calc()`가 새로 허용되는가 | 통과. admission helper가 기존 typed variant만 허용하므로 percentage-bearing calc AST는 이 경로에 추가되지 않는다. |
| 11 | Style projection이 cascade 순서를 재배열하는가 | 통과. 모든 순서에서 typed `GridAutoRows::TrackList`가 입력 순서를 그대로 보존하는지 대조한다. |
| 12 | RuntimeSession computed serialization이 typed 값과 달라지는가 | 통과. 각 scene에서 typed track vector와 `grid-auto-rows` serialization을 authored permutation과 각각 비교한다. |
| 13 | Layout admission이 Style projection보다 좁게 남는가 | 통과. 첫 matrix 실행에서 발견한 Layout의 exact-order gate를 동일한 prefix·multiset 경계로 넓혔고 120 layout scene이 통과한다. |
| 14 | Layout admission이 Style projection보다 넓어지는가 | 통과. 두 helper 모두 같은 길이, 첫 cap, tail count와 variant만 허용한다. |
| 15 | permutation 생성기가 중복을 포함하거나 일부를 빠뜨리는가 | 통과. 각 generator가 leaf 30개와 unique serialization 30개를 확인한다. 세 종류 multiset의 전체 순열 수도 30이다. |
| 16 | 8개 implicit item이 두 번째 track cycle까지 소비하는가 | 통과. layout/runtime 기대값이 8개 item 전체 frame을 대조하며 여섯 번째 경계 뒤의 item도 검사한다. |
| 17 | explicit 10px row가 implicit cycle에 잘못 포함되는가 | 통과. explicit row와 implicit rows를 다른 frame으로 검증하고 10px 시작 위치를 따로 둔다. |
| 18 | `fit-content` cap이 implicit cycle마다 적용되지 않는가 | 통과. 첫 번째와 다음 cycle의 fit-content row 및 20px cap을 기대값과 대조한다. |
| 19 | 반복 fit-content child overflow를 track sizing과 혼동하는가 | 통과. 두 번째 cycle의 60px payload frame을 별도 검증해 row cap과 visible overflow를 분리한다. |
| 20 | 첫 fit-content 위치에 예약한 60px payload가 허용되는가 | 거부. Layout 및 RuntimeSession negative case가 지원 payload topology에서 첫 fit row의 oversized child를 실패시킨다. |
| 21 | `align-content:start`가 auto row geometry를 변경하는가 | 통과. 30순서 × 두 cap form × start matrix에서 각 row/frame을 비교하고 합계를 96px로 확인한다. |
| 22 | `align-content:stretch`가 min/max row까지 늘어나는가 | 통과. free space 4px가 order별 auto occurrence에만 분배되며 총 row size가 100px인지 확인한다. |
| 23 | tail 첫 member가 `auto`일 때 반복 cycle의 두 auto occurrence를 빠뜨리는가 | 통과. 8 implicit rows에서 같은 첫 tail member가 두 번 나타나는 경우를 포함하고 auto stretch count로 계산한다. |
| 24 | item count 7 또는 9에서 검증되지 않은 topology가 허용되는가 | 거부. layout tests가 불완전한 두 번째 cycle과 허용 수 초과를 거부하고 RuntimeSession fixture는 고정 direct-child 수를 유지한다. |
| 25 | unsupported media revision이 partial result를 남기는가 | 거부. same-session test는 fit-content 비선두 dark-mode order의 실패 상태·property/node를 확인하고 completed result가 없음을 검증한다. |
| 26 | 실패 뒤 동일 RuntimeSession이 회복하지 못하는가 | 통과. 유효 light revision 복귀 후 Ready와 완성 render scene, overflow frame을 같은 session에서 확인한다. |
| 27 | 일반 CSS Grid 입력이 bounded Root HTML 예외를 얻게 되는가 | 통과. 변경은 기존 Root HTML typed profile admission 안에만 있고 generic CSS surface나 public API를 바꾸지 않는다. |
| 28 | simulator가 이전 bootstrap fixture를 계속 실행하는가 | 통과. Android APK·iOS bundle의 fixture SHA가 원본과 일치하며 새 process에서 두 플랫폼 모두 `layout=ready boxes=20`을 기록한다. |
| 29 | simulator screenshot을 surface pixel·hardware·실기기 결과로 과장하는가 | 통과. 증거는 OS screenshot visual smoke와 Android draw-presented/iOS layout-ready로 한정하며 PixelCopy·Metal drawable readback·hardware GPU·실기기는 주장하지 않는다. 앱·bootstrap 복원 hash도 별도 기록했다. |
| 30 | 계획 또는 로드맵이 bounded 구현을 C11.4 완료나 CI green으로 올리는가 | 통과. 체크박스와 공식 `78/214`는 유지하고 candidate를 진행 중으로만 표시한다. PR workflow/status 결과와 전체 docs dead-link 검사는 성공으로 주장하지 않는다. |

**결론:** diff에서 30순서 tail 이외의 CSS 입력, 임의 track multiplicity, public contract 변경은 허용하지 않는다. 구현/runtime matrix, same-session reject/recovery, Android/iOS simulator evidence와 문서 주장 범위가 이 경계를 뒷받침한다. C11.4와 C11은 미완료이며 공식 완료는 `78/214`다.
