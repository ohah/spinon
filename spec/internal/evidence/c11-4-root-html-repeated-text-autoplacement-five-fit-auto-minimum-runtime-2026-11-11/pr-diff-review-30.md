# C11.4 다섯 fit auto-placement PR diff adversarial review

검토 대상은 이 PR의 staged candidate diff 전체다. 앞서 작성한 plan 검토와 implementation/runtime 20 관점 검토를 반복하지 않고, PR에 들어가는 지원 경계·회귀 oracle·실패 처리·증거·명세 표시를 서로 다른 30개 실패 관점으로 확인했다. 각 항목은 source, test, precomparison 또는 증거 문서에 연결해 검토했다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 다른 authored track-list 길이가 새 profile에 잘못 들어오는가 | layout admission은 길이 6을 먼저 요구한다. |
| 2 | fit-content 개수가 5가 아닌 목록이 들어오는가 | 5개 정확 일치 조건과 0·1·2·3·4·6개 거부 테스트가 있다. |
| 3 | 허용되지 않은 tail 종류가 들어오는가 | 유일한 비-fit member만 min-content, max-content 또는 auto일 수 있다. |
| 4 | tail이 둘 이상이거나 없는 목록이 들어오는가 | 남은 member count가 정확히 1이어야 하며 중복 tail 거부를 검사한다. |
| 5 | 음수·NaN·무한대 px cap이 허용되는가 | 공통 cap predicate와 양수성·유한성 거부 사례가 있다. |
| 6 | 음수·NaN·무한대 percentage cap이 허용되는가 | percentage도 같은 유한·비음수 경계를 검사한다. |
| 7 | layout 단계에서 unresolved calc id가 profile을 통과하는가 | layout profile 테스트는 calc id를 거부한다. |
| 8 | 다섯-fit 지원이 row-span profile까지 넓히는가 | placement dispatcher에만 연결하고 span dispatcher에는 추가하지 않았다. |
| 9 | style-to-layout projection이 layout matcher와 지원 범위가 달라지는가 | 동일한 길이·fit 수·tail 경계를 별도 typed-style predicate로 맞췄다. |
| 10 | computed cap 검사에서 계산식이 우회되는가 | computed calc variant는 projection cap 테스트에서 거부한다. |
| 11 | 새 layout matcher가 production dispatcher에서 빠지는가 | 반복 min/max/auto auto-placement predicate의 OR 체인에 연결됐다. |
| 12 | 새 projection matcher가 production dispatcher에서 빠지는가 | projection의 반복 auto-placement predicate에 연결됐다. |
| 13 | fit 위치 순열 일부가 빠지는가 | 6개 위치 조합 각각과 세 tail을 교차해 18개를 검사한다. |
| 14 | tail 위치별 지원이 달라지는가 | 6개 tail 위치와 세 tail 종류가 profile 및 Chrome matrix에 포함된다. |
| 15 | cap 순열 일부가 fixture oracle에 빠지는가 | 다섯 cap의 120개 순열과 2,160 track list를 단언한다. |
| 16 | precomparison 입력이 잘리거나 metadata와 불일치하는가 | metadata 1건과 정확히 17,280 scene인지 런타임에서 확인한다. |
| 17 | source builder가 placeholder를 조용히 잘못 바꾸는가 | 기존 replace-once helper로 authored list, item block, minimum rule, alignment를 치환한다. |
| 18 | 각 scene의 실제 authored list가 oracle metadata와 다른가 | fit 위치·tail·cap을 문자열에서 다시 뽑아 scene metadata와 대조한다. |
| 19 | baseline과 candidate가 다른 scene key를 비교하는가 | 8,640개 key 각각을 중복 거부하며 두 key 집합을 완전 일치시킨다. |
| 20 | cascade가 의도한 typed cap·순서를 보존하지 않는가 | 모든 scene에서 typed auto-rows 및 serialized computed value를 oracle과 비교한다. |
| 21 | min-height 또는 align-content cascade 결과가 잘못되는가 | computed min-height와 computed alignment를 scene마다 검사한다. |
| 22 | Grid 또는 item의 최종 frame이 oracle과 다른가 | Grid 1개와 9개 item·row frame을 모든 scene에서 비교한다. |
| 23 | text paint가 layout 결과와 어긋나거나 빠지는가 | 8개 text node의 payload frame, glyph 수, line baseline 수를 대조한다. |
| 24 | zero/tiny fit cap이 candidate의 automatic minimum을 자르는가 | 57,600 candidate fit occurrence에서 payload floor 위반 0을 요구한다. |
| 25 | tail payload가 candidate에서 잘리거나 candidate item이 Grid 끝을 넘는가 | tail floor 위반 0 및 candidate item frame overflow 0을 단언한다. |
| 26 | baseline의 기존 overflow가 candidate 결과와 혼합되는가 | 21,600 baseline fit-floor 사례와 `auto/0px` overflow 600건을 별도 집계한다. |
| 27 | `align-content: stretch` 차이가 automatic minimum 효과로 오인되는가 | start/stretch 차이를 tail/minimum별로 분리해 예상 두 auto-tail cohort만 단언한다. |
| 28 | profile 밖의 DOM/CSS 변형이 부분 지원으로 통과하는가 | 27개 변형을 독립 reject case로 두고 실패 시 layout output이 없는지 확인한다. |
| 29 | 환경 변경 실패가 이전 frame을 새 revision으로 노출하거나 복구를 막는가 | 실패 revision의 layout output 부재·cascade key를 확인한 뒤 같은 session에서 새 key와 기존 frame으로 복구한다. |
| 30 | PR이 증거 수준을 과장하거나 C11.4 전체 완료로 표시하는가 | iOS/Android는 기본 route visual smoke로 제한하고, C11.4/C11 미완료 및 78/214·0.1.0 유지가 명시돼 있다. |

## 결과

30개 관점에서 수정이 필요한 PR diff 결함을 찾지 못했다. 이 검토는 제한된 five-fit Root HTML profile에 한정된다. 일반 CSS Grid 지원, pixel parity, GPU readback, 실기기 검증 또는 C11.4/C11 전체 완료를 주장하지 않는다. PR 생성 후 GitHub가 반환한 실제 diff와 head SHA도 이 검토 범위와 대조한다.
