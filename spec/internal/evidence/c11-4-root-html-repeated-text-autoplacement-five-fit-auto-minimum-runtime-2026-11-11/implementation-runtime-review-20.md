# C11.4 다섯 fit auto-placement 구현·runtime 실패 관점 20개

검토 범위는 다섯-fit admission predicate, CSS computed-style projection, Root HTML RuntimeSession matrix와 rejection/recovery tests다. 각 항목은 서로 다른 입력 경계나 runtime 불변식을 확인한다. 전체 matrix의 최종 실행은 [17,280-scene raw log](five-fit-matrix-final.log), focused output은 [focused test log](focused-tests-final.log)에 있다.

| # | 실패 관점 | 확인 결과와 근거 |
| ---: | --- | --- |
| 1 | authored track list가 6개보다 짧거나 긴 경우 | layout/style predicate가 길이 6을 먼저 요구한다. Runtime rejection은 5-member와 7-member list를 거부한다. |
| 2 | fit-content 개수가 0인 경우 | 다섯-fit predicate는 fit count 5 외에는 성공할 수 없다. Runtime rejection은 0-fit 6-member list를 거부한다. |
| 3 | fit-content가 6개이고 tail이 없는 경우 | fit_count == 5와 one-tail count 검사가 모두 실패 조건이다. Runtime rejection은 6-fit list를 거부한다. |
| 4 | 네 fit과 두 tail이 섞여 기존 four-fit profile이 잘못 admission하는 경우 | 현재 candidate에서는 tail 둘의 합이 1이 아니므로 거부한다. 초기 negative case는 기존 four-fit 지원과 겹쳐 테스트 입력을 수정했고 최종 rejection log에서 거부를 확인했다. |
| 5 | fit이 다섯이어도 tail이 fixed length·minmax 등 지원 밖 타입인 경우 | tail 분기에서 MinContent, MaxContent, Auto 외 variant는 즉시 거부된다. Runtime fixed-size tail rejection이 이를 확인한다. |
| 6 | 같은 허용 tail이 두 번 있거나 tail이 누락된 경우 | 지원 tail occurrence의 합이 정확히 1이어야 한다. 4-fit/two-tail과 6-fit/no-tail cases가 거부된다. |
| 7 | negative cap이 있는 경우 | 각 fit cap에 finite 및 nonnegative 조건을 적용한다. Runtime negative-cap mutation은 layout output 없이 거부된다. |
| 8 | NaN 또는 무한 cap이 projection에 도달한 경우 | 공용 cap 검사에서 is_finite()가 false라 admission을 거부한다. Unit rejection은 invalid cap 경계를 직접 검사하고 계산 cap은 finite 검사로 fail-closed한다. |
| 9 | percentage cap의 basis가 indefinite인 경우 | Style admission은 resolved percentage track type만 전달하고, Runtime profile은 definite 220px block basis에 고정된다. indefinite grid block-height mutation은 거부된다. |
| 10 | unresolved mixed-unit calc가 computed px로 오인되는 경우 | calc(1px + 2%)를 runtime negative case로 두어 unresolved percentage 계산을 거부한다. |
| 11 | 절대 길이 calc 또는 단위 변환의 정상화를 거부하는 경우 | calc(1px + 2px)와 0.25in/절대 calc positive controls가 computed px cap을 확인한다. |
| 12 | 가능한 fit 위치 하나를 빠뜨리거나 tail 위치를 고정하는 경우 | Layout/style unit tests는 6개 위치 조합과 각 허용 tail을 생성한다. Matrix의 position/tail key 집합도 6가지와 세 tail의 6위치를 모두 assert한다. |
| 13 | authored cap 순서를 정렬·재배치해 computed style을 왜곡하는 경우 | 120개 cap permutation마다 typed auto-row list와 serialized computed grid-auto-rows를 oracle과 비교한다. |
| 14 | baseline과 candidate가 서로 다른 환경 조합을 비교하는 경우 | Matrix는 각 8,640 key를 baseline/candidate 양쪽에 한 번씩 넣고 key 집합 동일성을 assert한다. |
| 15 | candidate min-height:auto가 fit-row content floor보다 작게 배치되는 경우 | 57,600 candidate fit occurrence를 계산하고 floor violation 0을 요구한다. |
| 16 | non-fit tail row가 payload minimum보다 작아지는 경우 | 모든 scene에서 tail payload-floor violation 0을 요구한다. |
| 17 | baseline의 알려진 overflow를 candidate 회귀로 잘못 분류하거나 숨기는 경우 | baseline/candidate overflow counter를 별도로 누적한다. baseline auto/0px payload의 Grid-end overflow 600건은 별도 key로 유지한다. |
| 18 | auto tail의 stretch track sizing을 min/max tail과 합산하는 경우 | start/stretch row pair를 tail/min-height key별로 보존한다. 1,440씩의 auto/0px·auto/auto 차이를 별도 assertion한다. |
| 19 | DOM reverse order, text wrapping, glyph 수 또는 payload frame이 틀리는 경우 | forward/reverse 두 순서, 8개 text node, 개별 glyph count·line count·payload frame을 매 scene Chrome record와 비교한다. |
| 20 | 지원 안 되는 dark revision에서 이전 layout을 게시하거나 같은 session이 복구하지 못하는 경우 | six-fit dark list rejection 뒤 layout 완료 output이 없어야 한다. 그 revision의 computed CSS cascade key와 authored list를 확인한 후 light revision에서 같은 RuntimeSession이 다시 성공해야 한다. |

## 검토 중 발견해 수정한 테스트 결함

초기 same-session rejection assertion은 layout 거부 시 cascade도 미완료일 것으로 잘못 예상했다. Runtime은 정확히 거부된 environment revision에 대한 computed CSS를 정상 제공하므로, 검증을 해당 revision의 cascade key·track list와 layout 완료 output 부재로 바꿨다. 별도의 rejection 입력은 existing four-fit admission과 겹치는 네 fit·두 tail에서 four-fit·한 min-content tail로 고쳐 독립적인 negative를 만들었다. 수정 후 two rejection tests와 17,280-scene matrix가 통과했다. 제품 구현 defect는 발견하지 않았다.

## 구조 검토

profiles.rs의 전체 line count는 변경 뒤 594이며 기준 HEAD에서도 560줄을 넘었다. 이 모듈은 같은 Root HTML Grid auto-placement profile family의 admission predicates를 한 곳에 모은다. 다섯-fit 검사만 분리하면 기존 1–4-fit 및 span predicate와 대칭성이 깨지고, 단일 34줄 submodule이 늘어나므로 이번 좁은 profile 추가에서는 기존 응집 경계를 유지했다. grid_values.rs도 기준부터 큰 projection module이지만 변경은 34줄 admission predicate와 test-module link다. 이를 더 큰 unrelated projection refactor와 섞지 않았다. 이 규모는 향후 grid-value projection을 책임별로 나눌 때 다시 검토한다.

## 결론

20개 runtime 실패 관점에서 구현 결함은 발견되지 않았다. 지원 범위는 고정 profile admission이며, C11.4/C11 전체 완료 판정이 아니다. simulator 화면은 visual smoke이고 compositor-level pixel comparison이나 실기기 증거가 아니다.
