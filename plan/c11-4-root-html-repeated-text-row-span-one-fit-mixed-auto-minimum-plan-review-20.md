# One-fit row-span mixed minimum 계획 · 구현 전 실패 관점 20개 검토

**대상:** [집중 계획](./c11-4-root-html-repeated-text-row-span-one-fit-mixed-auto-minimum.md) · [Chrome 사전 비교](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-one-fit-mixed-auto-minimum-precomparison-2026-11-14/README.md)

각 항목은 계획의 별도 반례·불변 조건을 대조한다. Chrome oracle 검토는 제품 RuntimeSession 실행이나 구현 완료 판정이 아니다.

| # | 실패 관점 | 적대 확인과 판정 |
| ---: | --- | --- |
| 1 | Mixed admission이 기존 one-fit 밖의 두 개 이상 fit으로 넓어짐 | 계획은 one-fit predicate에만 새 mask gate를 연결하고 zero-/two-fit 및 다른 count profile은 rejection control로 둔다. admission 경계가 검증 가능하다. |
| 2 | fit-content의 authored position과 cap이 서로 바뀜 | 기존 positions 1–6을 보존하고 typed cap/position을 RuntimeSession trace에서 확인한다. Chrome 사전 fixture는 두 cap set과 전 위치를 모두 관측했다. |
| 3 | 기존 six-member track list에서 tail member가 빠지거나 늘어남 | 계획은 `min-content×2`, `max-content×2`, `auto`의 기존 6-member list와 30개 고유 tail 순열을 고정한다. 다른 길이와 tail 조합은 negative control이다. |
| 4 | Percentage cap이 indefinite 축에서 잘못 계산됨 | 고정 definite `40×220px` Grid와 기존 direct percentage만 사용하며 unresolved percentage/calc 변형은 거부한다. |
| 5 | 3-row와 6-row topologies가 한 predicate로 잘못 혼합됨 | Positions 1–3의 기존 3-row 네 topology와 positions 4–6의 기존 6-row 네 topology를 각각 고정하고 row count 및 placement를 trace에서 검사한다. |
| 6 | Mask bit가 DOM 순서에 붙어 source reversal 뒤 의미가 달라짐 | 계획은 mask를 semantic item ID에 귀속하고 forward/reverse 결과를 60,480쌍으로 비교한다. Chrome 최대 semantic geometry 차이는 0 CSS px다. |
| 7 | 첫 explicit-row empty control이 auto minimum 대상이 됨 | control의 `min-height:0`을 강제하고 computed style/frame을 positive 및 rejection matrix에서 검사한다. |
| 8 | Eligible 조건을 Grid 단위로 계산해 특정 item의 overflow/track span을 놓침 | CSS Grid §6.6의 non-scrollable overflow, covered auto-minimum track, flexible-track 예외를 item별로 검사하도록 계획했다. |
| 9 | `fit-content()` cap을 automatic minimum의 상한으로 취급 | 기준 §6.6·§7.2와 계획은 cap floor와 item content-based minimum을 분리한다. Runtime 기대값은 Chrome 결과와 payload floor를 함께 대조한다. |
| 10 | Span item이 실제 covered row 합보다 작거나 큼 | 모든 row/item/payload frame 및 span row 합을 검사하고 Chrome 독립 verifier의 최대 합산 오차 `0.00025px`보다 보수적인 `0.01px` 한계를 계획했다. |
| 11 | `align-content:stretch`가 non-auto track 크기를 바꿈 | 시작/늘림 pair를 전 matrix에서 비교하고 non-auto track 크기 불변을 별도 검사한다. Chrome 관측에서 최대 차이는 0 CSS px다. |
| 12 | Auto track stretch 뒤 배치된 non-auto item의 위치 이동을 track 크기 변경으로 오판 | 계획은 non-auto row 크기와 item 위치/크기를 분리 검증한다. Chrome fixture에서 뒤쪽 item이 이동할 수 있음을 문서화했다. |
| 13 | Uniform zero/all-auto endpoints가 mixed logic으로 인해 바뀜 | 기존 positions 1–3 및 4–6 uniform full matrices를 positive controls로 유지하고 14,400 Chrome endpoint의 frame/text line 대조를 기준에 둔다. |
| 14 | Auto minimum floor 위반이 zero baseline과 합산되어 숨겨짐 | 적격 auto item별 payload content floor를 별도 카운트하고 baseline 차이는 별도 metric으로 보존한다. Chrome에서 169,344 eligible occurrence의 위반은 0이다. |
| 15 | 이 precomparison에서 나타나지 않은 overflow를 이미 검증했다고 과장 | Chrome fixed input의 Grid-end overflow는 0 scene이다. 계획은 실제 RuntimeSession overflow를 확인하도록 하며 현재 사전 자료를 overflow 재현으로 부르지 않는다. |
| 16 | Runtime matrix denominator가 빠진 topology/mask를 덮어 씀 | 120,960 total, topology별 2,880–34,560 scene, 4/8/16/32 mask 수, 900 unique lists를 검증 대상으로 고정한다. 실행 요약에는 expected/observed 분모를 둘 다 기록한다. |
| 17 | Negative control이 기능을 부분 게시한 뒤 실패함 | Unsupported control/style/track/cap 변형에서 rejection 전후 revision과 frame 불변, 동일 session의 다음 valid input recovery를 요구한다. |
| 18 | 누락·중복·순서 바뀐 trace가 기존 결과와 결합됨 | 계획은 stale/missing/reordered/duplicate trace를 reject하고 observation·semantic mask ID를 immutable fixture hash에 결합하도록 했다. |
| 19 | Simulator boot/smoke가 mixed feature 또는 GPU pixel parity를 증명한다고 표시됨 | Android/iOS는 계획된 mixed route visual smoke로 한정하고 fixture matrix, 실기기, PixelCopy/Metal surface readback 또는 compositor parity와 분리한다. |
| 20 | 계획 PR 이후에도 공식 상태·범위가 완료로 앞서감 | 계획 PR은 C11.4/C11 unchecked, 공식 `78/214`, contract `0.1.0`을 유지한다. 구현은 계획 PR merge 후에만 시작하고 자체 acceptance를 거쳐 별도 PR로 기록한다. |

## 검토 후 반영

- Chrome 사전 입력에서는 Grid content edge overflow가 0 scene임을 계획 요약에 명시하고 overflow 재현 증거로 확장하지 않았다.
- `align-content:stretch` 이후 non-auto item 위치와 non-auto row 크기를 따로 비교하도록 계획 문구를 유지했다.
- 전체 Chrome oracle, 제품 RuntimeSession, simulator visual smoke를 별도 증거 종류와 분모로 기록하도록 완료 조건을 확인했다.

**판정:** 이 bounded 계획은 위 gate를 구현 및 실행 증거에서 지키는 조건으로 구현 PR을 준비할 수 있다. 계획 검토는 별도의 실제 plan PR diff 30개 검토, PR merge 또는 제품 검증을 대체하지 않는다.
