# Four-fit row-span mixed minimum 계획 · 구현 전 실패 관점 20개 검토

**대상:** [집중 계획](./c11-4-root-html-repeated-text-row-span-four-fit-mixed-auto-minimum.md) · [Chrome 사전 비교](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-four-fit-mixed-auto-minimum-precomparison-2026-11-12/README.md)

계획과 138,240-scene Chrome matrix를 대조했다. 아래는 서로 다른 failure mode이며, 계획 검토는 제품 구현이나 C11.4 완료 판정이 아니다.

| # | 실패 관점 | 적대 확인과 판정 |
| ---: | --- | --- |
| 1 | Mixed gate가 five-/six-fit 또는 다른 row-span에도 열림 | 현재 style dispatch의 five-/six-fit mask helpers는 track-list boolean에 묶여 있다. 계획은 exact four-fit list·기존 topology를 요구하고 별도 four-fit admission을 추가하도록 했다. 범위 통과. |
| 2 | Track list 길이 또는 intrinsic tail 수가 잘못됨 | Fixture는 네 fit과 `min-content`, `auto`로 정확히 여섯 implicit tracks를 만든다. Verifier가 다른 tail count와 목록 길이를 거부한다. 범위 통과. |
| 3 | Cap type/중복/누락이 혼합 허용 | 기존 typed cap set은 `[0px,10px,20%,30px]` 각각 한 번이다. 계획은 exact typed multiset을 유지하고 중복·누락·미등록 cap rejection을 요구한다. 통과. |
| 4 | Authored cap이 다른 track position에 귀속 | 모든 15 fit-position set과 24 cap order를 관측하고 각 position-to-cap binding을 verifier가 재구성한다. Runtime profile에서도 순서 보존을 gate로 둔다. 통과. |
| 5 | `20%` basis가 indefinite이거나 잘못된 축에 적용 | Fixture의 definite `220px` block axis와 고정 `40×220px` Grid를 유지한다. Unresolved/indefinite percentage는 reject하도록 적었다. 통과. |
| 6 | 남은 두 track의 tail order가 변하거나 하나가 빠짐 | 두 `[min-content,auto]` 순서를 모두 720개 list에 포함하고 exact authored order 및 computed order를 검사한다. 통과. |
| 7 | Scene 수가 잘못 계산되어 mask 일부가 빠짐 | `720×(32×2+16×2)×2=138,240`, topology별 46,080/23,040 및 mask별 1,440 scene을 verifier가 확인했다. 계획 acceptance도 exact denominator를 고정했다. 통과. |
| 8 | Mask bit가 DOM 순서에 묶여 다른 item에 적용 | Fixture mask bit order는 semantic ID A–E/A–D로 고정되고 auto item 목록을 ID로 생성한다. 계획은 DOM reversal에도 같은 semantic mask를 요구한다. 통과. |
| 9 | Reverse DOM order에서 mask owner가 바뀜 | Forward/reverse pair 0 CSS px를 확인했다. Sample과 RuntimeSession verifier가 semantic ID별 item/payload geometry를 pair로 비교하도록 했다. 통과. |
| 10 | Explicit-row empty control이 automatic minimum 대상이 됨 | 모든 scene의 explicit row item은 `min-height:0`과 `[0,0,40,10]` frame이다. Style gate와 runtime rejection/positive controls 양쪽에 고정했다. 통과. |
| 11 | Eligibility를 Grid 전체 단위로 계산하거나 scrollable item을 누락 | 규범 eligibility를 item별 overflow, 실제 covered tracks의 auto minimum, flexible track 부재로 적었다. 고정 fixture의 visible overflow와 실제 span tracks를 verifier가 계산한다. 통과. |
| 12 | `fit-content()` cap을 content minimum의 상한으로 취급 | CSS Grid §6.6/§7.2.1을 연결하고 `fit-content()` auto minimum과 cap을 구분했다. Cap 자체를 floor로 쓰지 않도록 명시했다. 통과. |
| 13 | Item frame이 실제 spanning rows와 일치하지 않음 | 각 item frame과 used-row 합을 독립 검증하며 Chrome max error는 `0.00045px`다. Runtime acceptance 허용 오차 `0.01px`을 명시했다. 통과. |
| 14 | Auto mask는 content floor를 보존하지 못하거나 zero 기준을 자동으로 고침 | Chrome에서 auto shortfall 0, zero baseline shortfalls 28,032/9,216이다. 계획은 이 차이를 제품 결과로 전용하지 않고 RuntimeSession에서 별도 검증한다. 통과. |
| 15 | Expected overflow를 0으로 강제해 실패를 숨김 | Chrome에서 Grid-end overflow 1,392건이 모두 zero-mask scenes에 있다. 계획은 실제 수를 별도 집계하고 0을 선험적 기준으로 강제하지 않는다. 통과. |
| 16 | `align-content:stretch`가 intrinsic tail 또는 non-auto tracks를 늘림 | Chrome paired result에서 non-auto track 변화가 0이다. Runtime verifier가 auto row만 stretch하는지 검사하도록 gate를 적었다. 통과. |
| 17 | Source order나 `align-content`에 따라 semantic geometry가 변함 | 전체 matrix에서 source-order와 non-auto alignment delta가 각각 0이다. 두 관계를 제품 비교 기준에 포함했다. 통과. |
| 18 | Runtime sample이 한 cap/position/tail에 편향 | 선택식 검산에서 20개 unique list index, 15 position sets, 20 cap orders, 2 tail orders, 3,840 scenes를 얻었다. 720개 uniform endpoint는 별도 full regression으로 유지한다. 통과. |
| 19 | Unsupported child/mask가 일부 output을 게시하고 세션이 망가짐 | 계획은 wrong topology/style/owner와 non-0/auto minimum rejection, partial output 금지, 같은 session의 다음 valid revision recovery를 요구한다. 통과. |
| 20 | 계획/앱 smoke를 기능 완료나 전체 C11.4로 과장 | Android/iOS default route는 startup smoke만 증명한다고 제한했다. Merge 후 문서 동기화에서도 C11.4/C11, `78/214`, contract `0.1.0`을 유지한다. 통과. |

## 검토 후 반영

- Runtime sample의 cap rank가 fixture의 swap-recursion permutation 배열을 가리키도록 정의했다. 선택식을 실행해 unique list와 15/20/2 coverage를 확인했다.
- Uniform endpoint 회귀 selector를 현재 four-fit test function 이름과 맞췄다. 기존 “mixed items reject” test는 제거하지 않고 positive mask matrix와 invalid value/control rejection으로 역할을 바꾸도록 계획했다.
- Chrome 관측과 RuntimeSession 결과, 앱 기본 route smoke를 서로 다른 증거로 기록하도록 제한을 구체화했다.

**판정:** 계획은 위 조건을 유지하는 bounded slice로 구현을 시작할 수 있다. 이 판정은 계획 PR diff의 별도 30개 리뷰, plan PR merge 또는 제품 동작 검증을 대체하지 않는다.
