# C11.4 definite minimum-size `auto-fit` 축별 반복 계획 적대 검토 20개

**검토 대상:** [C11.4 auto-fit 계획](https://github.com/ohah/spinon/blob/plan/c11-4-autofit-min-size-axis-20261020/plan/c11-4-auto-fit-min-size-axis.md), [30-case Chrome fixture](https://github.com/ohah/spinon/blob/plan/c11-4-autofit-min-size-axis-20261020/tests/fixtures/css/c11/grid-auto-fit-min-size-axis-c114.html), [Chrome observation](https://github.com/ohah/spinon/blob/plan/c11-4-autofit-min-size-axis-20261020/spec/internal/evidence/c11-4-auto-fit-min-size-axis-precomparison-2026-10-20/chrome155-observation.json). 이 검토는 구현 코드나 PR 최종 diff를 검토하지 않는다.

| # | 독립 실패 관점 | 결과와 근거 |
| ---: | --- | --- |
| 1 | Chrome의 `auto-fit` count를 규범 기대값으로 잘못 채택하는가? | **통과.** CSS Grid §7.2.3.2를 기준으로 선언하고 Chrome 결과는 divergence로 보존한다. |
| 2 | `auto-fit`과 `auto-fill`이 반복 count부터 달라도 된다고 해석하는가? | **통과.** 계획은 placement 전 같은 count를 요구하고 collapse는 placement 뒤 단계로 분리한다. |
| 3 | definite minimum-size와 definite preferred/max-size 조건을 같은 분기처럼 계산하는가? | **통과.** min-size formula는 3회, definite 200px preferred-size positive control은 2회로 별도 기록한다. |
| 4 | `minmax(40px, 1fr)`의 auto-repeat contribution에서 definite min fallback을 놓치는가? | **통과.** 계획은 `40px` minimum을 반복 수 계산에 쓰고 fixture에 `1fr` max와 fixed-only 대조를 함께 둔다. |
| 5 | 2-track fragment의 두 번째 fixed `20px`을 반복 기여에서 빠뜨리는가? | **통과.** 반복당 기여는 `40px + 20px`으로 명시하고 resolved track list 전체를 저장한다. |
| 6 | 마지막 gutter를 중복 포함하거나 prefix/suffix 사이 gap 하나를 빼는가? | **통과.** both-neighbor 식 `38 + 76n`에 repeat 밖 고정 track, fragment 내부/경계 gap을 포함한다. |
| 7 | prefix-only와 suffix-only의 neighbor 순서를 바꾸거나 양쪽을 같은 값으로 간주하는가? | **통과.** `20 + 76n`과 `10 + 76n`을 별도로 두고 각 축을 관찰한다. |
| 8 | neighbor가 없는 list의 첫/마지막 gutter 개수를 잘못 세는가? | **사후 정정 필요.** 초기 검토는 반복 fragment의 두 번째 gutter와 반복 사이 경계 gutter를 하나로 잘못 셌다. 첫 반복은 `68px`, 다음 반복마다 `76px`이므로 전체 식은 `76n - 8`; 200px minimum-size의 기대 반복 수는 3회다. 계획의 계산 정정 절과 implementation matrix가 이 값을 사용한다. |
| 9 | 190/200/342 boundary에서 ceil/minimum과 floor/preferred-size 선택을 뒤섞는가? | **통과.** minimum 경계 2/3/4와 definite preferred-size 2를 별도로 비교한다. |
| 10 | repeated track 수를 repeat component 횟수로 잘못 나누거나 explicit track을 포함하는가? | **통과.** JSON에는 전체 track list와 repeat 밖 고정 track 수가 남고 count를 fragment track 수에서 산출한다. |
| 11 | row matrix를 column 기대값에 재사용해 axis별 오류를 숨기는가? | **통과.** 각 sizing basis·mode·scene을 columns와 rows에 독립 생성하고 모든 JSON record가 axis를 보유한다. |
| 12 | 물리 width/height와 logical columns/rows 차이를 식별하지 못하는가? | **통과.** `vertical-rl` 진단 case에서 물리 방향을 바꾸며 지원 판정과는 분리한다. |
| 13 | item 하나가 repeat component 안의 어떤 track을 점유했는지 불명확한가? | **통과.** `single`은 첫 반복 track, `span-two`는 첫 두 반복 track에 명시 배치한다. |
| 14 | spanning item이 한 track만 점유했는데 두 개로 기대하는가? | **통과.** 계획은 span된 각 track이 비어 있지 않게 남는지 각각 검사하도록 한다. |
| 15 | out-of-flow item을 in-flow occupancy로 세어 empty track collapse를 막는가? | **검토에서 누락 발견·수정.** Chrome fixture에 두 축의 absolute span control을 추가했고, 기대 모델은 이를 occupancy로 세지 않는다. |
| 16 | empty auto-fit에서 repeated tracks 외의 fixed neighbor나 주변 gutter까지 잘못 접는가? | **통과.** explicit neighbor는 보존하고 empty repeated tracks/adjacent gutter만 접는 기준을 분리했다. |
| 17 | definite 200px control을 min-size 200px case의 기대값으로 오인하는가? | **통과.** fixture `basis`와 plan 표에서 두 조건을 다른 경로와 2/3회 기대값으로 구분한다. |
| 18 | Chrome `getComputedStyle()`의 resolved 0px track을 구현의 pre-collapse repeat count로 잘못 읽는가? | **통과.** 문서는 Browser 관찰과 향후 Spinon layout trace를 분리하고 앱 검증에 repeat range/count를 요구한다. |
| 19 | source revision이 현재 Chrome 바이너리와 같다고 과장하거나 코드만으로 root cause를 단정하는가? | **통과.** `chrome://version` revision과 Gitiles commit을 분리하고 실제 Blink private size 입력은 미측정이라고 기록한다. |
| 20 | simulator·WPT·공개 지원·C11.4 완료를 Chrome oracle 또는 계획만으로 주장하는가? | **통과.** 계획은 Android/iOS fixture 검증을 다음 단계로 남기고 WPT/실기기/GPU/compositor 및 공식 완료 체크를 미완료로 둔다. |

검토에서 발견한 #15 누락은 fixture와 계획을 보강한 뒤 확인했다. 항목 20개는 서로 다른 규범, sizing, placement, provenance, 플랫폼 및 상태 경계를 대상으로 하며 구현 후 검토에서 재사용하지 않는다.
