# C11.4 네 fit auto-placement `min-height:auto` 계획 검토 20개

**검토 대상:** [계획](./c11-4-root-html-repeated-text-autoplacement-four-fit-auto-minimum.md) 및 [Chrome 157 사전 비교](../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-four-fit-auto-min-precomparison-2026-11-11/README.md)

계획 PR 전의 적대 검토다. 아래 항목은 서로 다른 실패 경로를 다루며 구현/runtime 검토로 재사용하지 않는다.

| # | 실패 관점 | 확인 및 계획의 방어 |
|---:|---|---|
| 1 | `fit-content()`가 자동 최소 track에 해당하지 않는다고 잘못 분류 | CSS Grid 2025 CRD §7.2의 min track sizing function `auto`와 §6.6 eligibility를 연결했다. |
| 2 | scrollable overflow item에도 content-based automatic minimum을 적용 | `overflow:visible`로 고정하고 scrollable overflow를 rejection matrix에 포함했다. |
| 3 | 여러 track을 span하거나 flexible track과 교차하는 item의 §6.6 조건을 누락 | 각 text item은 row 하나에만 놓이며 flexible track은 허용하지 않는다. Span과 unsupported placement를 거부한다. |
| 4 | `min-content`·`max-content` tail을 fit track과 같은 eligibility로 계수 | tail을 두 순서의 대조군으로 분리하고 fit-row count와 합산하지 않는다. |
| 5 | 네 fit 위치 조합 중 하나를 건너뛰거나 1-based 위치를 0-based로 잘못 해석 | 여섯 위치 중 네 개 선택 15개 모두를 사전·runtime matrix에서 센다. |
| 6 | tail 순서나 multiplicity를 허용 범위 밖으로 확장 | `min-content ×1`과 `max-content ×1`만 허용하고 두 순서만 positive로 둔다. `auto` tail 및 중복은 범위 밖이다. |
| 7 | cap의 authored index가 fit 위치와 어긋남 | 네 fit cap의 24개 순열을 모두 관측하고 typed cap·list order·trace를 같은 authored index로 대조한다. |
| 8 | `%` cap을 indefinite basis에 적용 | Chrome matrix와 profile은 definite 220px block-size만 허용하며 indefinite basis rejection을 검사한다. |
| 9 | fit cap이 content-based minimum floor를 제한한다고 오해 | §6.6/§11.5 경계를 적고 baseline과 candidate의 item/payload frame을 별도로 비교한다. |
| 10 | 여덟 implicit row에서 여섯-member pattern을 반복할 때 7·8번째 row 위치를 잘못 계산 | row occurrence는 authored list에 modulo로 대응하며 전체 8개 item의 반복 위치를 검증한다. |
| 11 | 첫 explicit-row empty control이 `min-height:auto`로 바뀌어 관측을 오염 | Control은 명시적으로 `min-height:0`이며 candidate input에서도 유지한다. |
| 12 | baseline/candidate를 track list 외에 다른 key로 잘못 짝지음 | 비교 key는 authored track list, alignment와 DOM order를 포함하고 `min-height`만 paired axis로 둔다. |
| 13 | `align-content:stretch`가 auto-sized track을 늘리는 규칙을 잘못 일반화 | track list에는 `auto` member가 없고 start/stretch row equality를 별도 검증해 관측 범위를 명시한다. |
| 14 | reverse DOM order에서 payload ID와 item row 위치를 뒤바꿈 | forward/reverse를 독립 교차하고 source order, item ID, row index와 payload line count를 함께 비교한다. |
| 15 | text font 또는 line-height 변화가 expected floor를 바꿈 | fixture의 monospace 10px/10px와 고정 payload를 저장하고 Chrome version·DPR·OS platform을 기록한다. |
| 16 | CSSOM normalized track list와 authored list 차이를 놓침 | authored typed order와 Chrome computed `grid-auto-rows` 및 used row sequence를 각각 보존한다. |
| 17 | five/six fit, 잘못된 list 길이, invalid cap 또는 다른 style이 positive admission으로 새어 듦 | 개수, multiplicity, length, cap grammar, basis, placement, overflow, owner와 style rejection을 plan에 열거했다. |
| 18 | 실패 후 이전 layout을 일부 갱신하거나 session이 회복되지 않음 | rejection atomicity와 동일 RuntimeSession의 다음 유효 revision recovery를 구현 gate에 포함했다. |
| 19 | 기존 one/two/three-fit auto-placement이나 three-fit row-span 경계가 회귀 | 네 기존 profile의 full positive/negative controls를 유지하고 one/two-fit auto-minimum과 repeated-text regressions를 명시했다. |
| 20 | Chrome oracle 또는 simulator screenshot을 제품·WPT·pixel parity 완료로 과장 | 사전 비교와 app visual smoke를 분리하고 Android/iOS route, PixelCopy/Metal, compositor, hardware, WPT 한계를 계획에 명시했다. |

## 판정

계획은 네 fit auto-placement만 좁혀 추가하며 C11.4/C11 체크와 공식 `78/214`를 바꾸지 않는다. Browser oracle의 5,760 scene corpus, 2,880 paired key, independent verifier, 실행 session 종료 근거를 PR diff 검토와 별도로 연결한다. 이 계획 검토는 계획 PR diff 30개 검토 및 구현 뒤 새로 수행할 20개 implementation/runtime 검토를 대신하지 않는다.
