# C11.4 five-fit row-span 계획 실패 관점 검토 20개

검토 대상은 [five-fit row-span 계획](./c11-4-root-html-repeated-text-row-span-five-fit-auto-minimum.md)과 [Chrome 157 사전 비교](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-five-fit-auto-minimum-precomparison-2026-11-11/README.md)다. 구현 코드는 아직 변경하지 않았다.

| # | 실패 관점 | 계획에서 확인한 경계와 판정 |
|---:|---|---|
| 1 | fit 수를 다섯이라고만 세고 여섯-member list를 검증하지 않는 경우 | 정확히 여섯 implicit member, 다섯 fit과 singleton tail을 요구하고 5/7 member를 거부한다. 통과 기준이 명확하다. |
| 2 | fit 위치 한 가지에만 맞춘 matcher가 다른 authored 위치를 받는 경우 | 6개 fit-position quintuple을 모두 비교하고 tail position을 list 순서에서 보존한다. 통과 기준이 명확하다. |
| 3 | cap 값은 같지만 authored cap과 track 위치가 바뀌는 경우 | 다섯 typed cap의 authored index와 position을 함께 보존하며 120개 순열을 검사한다. 통과 기준이 명확하다. |
| 4 | 중복 cap·누락 cap·새 cap을 다섯-fit으로 오인하는 경우 | 각 cap이 정확히 한 번씩인 typed multiset만 받으며 중복·누락·미등록 값을 negative matrix에 둔다. 통과 기준이 명확하다. |
| 5 | `min-content`, `max-content`, `auto` tail을 한 값으로 취급하는 경우 | singleton tail 값을 개별 관측하고 admission/negative matrix와 stretch 기대를 tail별로 분리한다. 통과 기준이 명확하다. |
| 6 | `fit-content()` cap을 automatic minimum의 content-floor 상한으로 해석하는 경우 | CSS Grid §6.6·§11.5를 별도 판정하고 candidate payload floor를 track cap과 혼동하지 않는다. 통과 기준이 명확하다. |
| 7 | span에 `fit-content()`가 있다는 이유만으로 모든 item을 무조건 eligible로 세는 경우 | scroll-container 여부, span track minimum sizing function, flexible track 부재 조건을 candidate에서 검사한다. 통과 기준이 명확하다. |
| 8 | percentage cap에 definite basis가 없는데 admission하는 경우 | Grid height `220px`를 definite로 고정해 `20%`의 basis를 정하고 indefinite percentage는 거부한다. 통과 기준이 명확하다. |
| 9 | 계산식이 해결되지 않았는데 일부 값만 사용해 진행하는 경우 | 기존 projection에서 허용 typed px로 정규화된 absolute `calc()`만 admission하고 unresolved calculation은 거부한다. 통과 기준이 명확하다. |
| 10 | baseline `min-height:0`과 candidate `auto`가 서로 다른 scene을 비교하는 경우 | 한 scene을 두 minimum mode로 쌍지어 17,280 pair를 비교한다. 통과 기준이 명확하다. |
| 11 | control empty item도 `min-height:auto`로 바뀌어 explicit row 기저를 흔드는 경우 | 빈 explicit-row control은 두 mode 모두 `min-height:0`, `[0,0,40,10]` frame으로 고정한다. 통과 기준이 명확하다. |
| 12 | span-2 연속 topology만 구현하고 span-2/3 기여를 놓치는 경우 | span-2 다섯 item 가족과 span-2/3 네 item 가족을 함께 돌리고 각 item frame을 covered rows에 대조한다. 통과 기준이 명확하다. |
| 13 | DOM order 역전이 placement와 geometry를 조용히 바꾸는 경우 | 두 topology 모두 forward/reverse order를 실행하고 같은 semantic item ID의 frame을 비교한다. 통과 기준이 명확하다. |
| 14 | repeated item 기여를 합치는 과정에서 row span이 겹치는 경로가 빠지는 경우 | 각 item의 explicit row/span과 7개 used row를 재구성하고 모든 item frame을 개별 검사한다. 통과 기준이 명확하다. |
| 15 | `align-content:stretch`가 tail 종류 차이를 숨기는 경우 | Auto tail에서만 free-space 배분을 검사하고 min/max tail은 used rows 불변으로 별도 판정한다. 사전 관측에서 이 차이를 확인해 계획에 반영했다. |
| 16 | 220px grid가 아닌 viewport나 auto height에서 우연히 통과하는 경우 | definite `40×220px`, explicit `40px` column/`10px` row, zero gap을 고정하고 frame을 검사한다. 통과 기준이 명확하다. |
| 17 | text wrap/font 차이를 track-sizing 결과로 오인하는 경우 | payload, monospace, 10px font와 line-height, line count를 고정해 각 관측 환경 안에서 비교한다. Chrome 결과를 app-runtime과 동일시하지 않는다. |
| 18 | CSS frame과 row serialization의 반올림 차이를 과잉 허용하거나 놓치는 경우 | CSS px `0.01` 허용치를 계획하고 precomparison verifier에서 span-frame 오차 최대 `0.00025`를 관측했다. 통과 기준이 명확하다. |
| 19 | stale owner/revision이나 실패 뒤 부분 geometry가 이전 결과에 섞이는 경우 | owner/revision trace, atomic rejection, 부분 게시 부재와 same-session recovery를 runtime gate에 둔다. 통과 기준이 명확하다. |
| 20 | 단일 Chrome scene·시뮬레이터 smoke를 전체 CSS Grid 지원이나 완료로 확대 해석하는 경우 | Chrome 사전 비교, RuntimeSession, Android/iOS visual smoke를 분리하고 public support·pixel parity·hardware·WPT·C11 완료 주장을 명시적으로 제외한다. 공식 `78/214`를 유지한다. |

## 검토 결과

계획의 핵심 admission 경계와 관측 모델에 구현 전 차단이 필요한 누락은 찾지 못했다. Tail별 stretch를 하나의 공통 기대값으로 취급하는 오류는 사전 관측 결과를 반영해 이미 계획에서 분리했다. 이 검토는 제품 구현 검증이나 실제 PR diff 검토를 대신하지 않는다.
