# C11.4 row-span fit-content positions 4–6 계획 실패 관점 검토 20개

**범위:** 계획 [c11-4-root-html-repeated-text-row-span-fit-content-positions-4-6.md](./c11-4-root-html-repeated-text-row-span-fit-content-positions-4-6.md)와 [Chrome 157 사전 비교](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-fit-content-positions-4-6-precomparison-2026-11-09/README.md). 제품 코드는 변경하지 않았다.

| # | 적대적 실패 관점 | 계획의 대응과 판정 |
|---:|---|---|
| 1 | authored grid-auto-rows 위치와 CSS row line 번호를 혼동해 position 4를 row line 4로 구현할 수 있다. | Fit position은 six-member list의 1-based authored index로 고정하고, six implicit rows의 cycle mapping을 Chrome CSSOM과 비교한다. 통과. |
| 2 | explicit row가 auto-row list의 첫 member를 소비한다고 가정할 수 있다. | Explicit row 10px을 list 밖에 두고 seven used rows 중 뒤의 여섯만 implicit cycle로 검사한다. 통과. |
| 3 | CSS repeated-keyword permutations를 5!인 120개로 잘못 계산하거나 중복 order를 검사하지 않을 수 있다. | min-content ×2, max-content ×2, auto ×1의 고유 tail 30개를 set으로 확인하고 각 fit 위치에서 모두 실행한다. 통과. |
| 4 | fit-content를 중복 허용하거나 고정 multiplicity가 흐려질 수 있다. | 한 개 fit-content와 나머지 keyword multiplicity를 고정하고 invalid 목록은 rejection corpus에 둔다. 통과. |
| 5 | row line placements가 여섯 번째 implicit track을 실제 생성하지 않을 수 있다. | span2-six의 마지막 row 6 / span 2와 mixed-span의 row 5 / span 3을 쓰고 used row 7개를 exact 검사한다. 통과. |
| 6 | span 범위 밖의 trace 행을 포함하거나 span frame 합계를 잘못 비교할 수 있다. | 각 scene에서 CSS row start/span, trace count와 일곱 used row를 확인하고 item frame을 해당 track 구간 합과 비교한다. 사전 관측 최대 오차 0px. 통과. |
| 7 | DOM source order 역전이 placement도 바꾸어 잘못된 비교가 될 수 있다. | 고정 semantic item ID와 placement mapping을 유지하고 DOM 배열만 역전한 네 topology 쌍으로 비교한다. 최대 차이 0px. 통과. |
| 8 | span2-only 결과만으로 혼합 span sizing을 대표한다고 일반화할 수 있다. | span2 5-item topology와 span2/span3 mixed topology를 별도 forward/reverse로 둔다. 나머지 span은 계속 지원 밖이다. 통과. |
| 9 | text grammar byte 상한을 초과하는 장문 fixture가 기존 admission과 다른 일을 검사할 수 있다. | A–E payload는 31·27·23·19·15 bytes로 각각 32-byte 제한 이하다. 통과. |
| 10 | Chrome font fallback이나 고정 폭 변화가 예상 line count를 무효화할 수 있다. | Chrome 157·MacIntel·DPR1, 40px width와 10px monospace/line-height를 고정하고 매 payload line count를 검사한다. 관측값은 6/5/4/4/3. 통과. |
| 11 | fit-content used track이 언제나 cap 이하라고 잘못 가정할 수 있다. | CSS Grid min-content floor 가능성을 명시하고 cap 상한을 독립 불변식으로 쓰지 않는다. 통과. |
| 12 | 저 cap과 높은 cap이 구별되지 않아 cap 검증력이 없을 수 있다. | 10px과 20px를 비교한다. 해당 corpus에서 fit track은 position4 48개, position5 48개, position6 24개 scene에서 달랐다. 통과. |
| 13 | percentage cap basis가 indefinite이거나 viewport 기준으로 적용될 수 있다. | definite 220px content block을 사용하고 20% = 44px, 120% = 264px를 기준으로 기록한다. 통과. |
| 14 | 0px와 0%가 같은 입력이라고 단정하면서 percent basis 차이를 놓칠 수 있다. | definite basis 조건 안에서 두 form을 각각 관측하고 scene rows가 동일한지 검사한다. 이 corpus에서는 동일했다. 통과. |
| 15 | align-content:stretch가 max-content/min-content/fit-content track을 바꿀 수 있다. | 매 topology/cap/order에서 start와 stretch를 pair 비교하고 auto가 아닌 implicit track은 그대로인지 검사한다. 사전 비교 통과. |
| 16 | 1/64px rounding으로 row 합과 item box가 다를 수 있다. | Layout trace 기반 final sizes와 Blink layout-unit distribution을 검사하고 0.01px geometry threshold를 고정한다. 사전 비교는 exact 0px. 통과. |
| 17 | nested, inline, replaced payload를 허용 목록 없이 새 profile에 섞을 수 있다. | 단일 direct payload grammar와 computed style을 고정하고 나머지는 pre-publish rejection 대상으로 둔다. 통과. |
| 18 | stale owner/revision 또는 누락된 track trace가 부분 결과를 publish할 수 있다. | owner·revision·trace 모순은 atomic failure로 계획하고 reject 뒤 동일 session recovery를 검증한다. 통과. |
| 19 | opt-in fixture route가 성공해 default Root HTML 경로를 입증한 것처럼 보일 수 있다. | 기본 RuntimeRootHtmlTextPaintV1 경로만 성공 gate로 사용하며 새 FFI/fixture ABI는 계획 범위에 없다. 통과. |
| 20 | bounded slice merge를 C11.4/C11 완료나 공식 숫자 증가로 오해할 수 있다. | 모든 계획·PR·상태 문서에서 두 체크 미완료, 78/214, 계약 0.1.0 유지와 지원 한계를 반복한다. 통과. |

**결론:** 20개 계획 실패 관점을 확인했다. 남은 위험은 계획에서 구현되지 않은 Native trace/runtime 동작이며, 계획 merge만으로 완료를 주장하지 않는다.
