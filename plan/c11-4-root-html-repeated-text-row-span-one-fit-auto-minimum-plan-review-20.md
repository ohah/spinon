# C11.4 one-fit row-span `min-height:auto` 계획 검토 20개

계획 문서와 Chrome 157 사전 비교를 대상으로 제품 코드 변경 전에 서로 다른 실패 경계 20개를 검토했다. 검토 중 text item 수를 기존 profile보다 넓게 읽을 수 있는 “여덟 이하” 표현을 발견해 실제 계획 scope인 2개 또는 3개로 좁혔다. 아래 검토는 수정 후 계획에 적용했다.

1. **one-fit과 two-fit predicate 혼합:** 두-fit row-span은 다른 기존 path이며 이번 계획은 한 fit만 허용한다. 계획이 별도 profile로 유지한다. **통과.**
2. **row-span과 auto-placement 혼합:** 이 계획은 명시적 row line만 사용한다. auto-placement profile을 변경하지 않고 rejection에 포함한다. **통과.**
3. **item 개수 과대 admission:** topology는 empty control 1개와 text item 2개 또는 3개다. 넓은 8-item 해석은 계획서에서 제거했다. **수정 후 통과.**
4. **explicit row를 implicit cycle에 포함:** track position 1은 explicit row 다음의 첫 implicit row다. 계획이 cycle 시작과 10px control row를 분리한다. **통과.**
5. **fit position 경계:** 계획 matrix는 positions 1–3에 한정한다. positions 4–6 six-row topology는 reject/regression으로 구분한다. **통과.**
6. **잘못된 automatic-minimum eligibility:** span은 non-scrollable이어야 하며 span 안에 `auto` min track이 있어야 한다. `fit-content()`의 auto minimum 의미를 CSS Grid §7.2에 연결했다. **통과.**
7. **flexible span 조건 누락:** CSS Grid §6.6의 span 내 flexible track 배제 조건을 적었고 현재 track multiset에 flex가 없음을 고정했다. **통과.**
8. **scrollable overflow 혼동:** `visible`만 허용하고 `auto|scroll|hidden` 변형을 rejection 대상으로 둔다. **통과.**
9. **explicit control에 자동 최소 크기 전파:** 첫 empty explicit-row control은 항상 `min-height:0`으로 고정하고 nonzero/auto 변형을 거부한다. **통과.**
10. **mixed item minimum 허용:** 모든 supported text item이 동일 `auto`인 경우만 positive다. mixed `auto|0`과 일부 child 변형은 negative matrix다. **통과.**
11. **cap과 percentage basis 실수:** direct finite px/% 네 값, 220px definite basis, zero percent와 zero length typed distinction을 보존한다. **통과.**
12. **track multiplicity/order 변형:** 한 fit, min-content 2, max-content 2, auto 1 및 tail order 30개를 고정한다. wrong count/order/multiplicity rejection이 있다. **통과.**
13. **text/font fixture drift:** source payload, ASCII grammar, monospace 10px/10px, LTR, wrapping과 direct-child ancestry를 고정한다. 임의 Unicode/style은 scope 밖이다. **통과.**
14. **row-span frame과 track 합 불일치:** 각 item frame을 used row 합과 비교하고 full track trace를 요구한다. **통과.**
15. **DOM order가 track sizing을 바꾸는 경우:** Chrome pair는 semantic placement가 같은 forward/reverse inputs를 포함하고 독립 verifier는 row/item frame을 비교한다. 최대 차이는 0 CSS px다. **통과.**
16. **stretch가 non-auto row를 변경:** Chrome verifier는 `start|stretch`를 pair하고 non-auto row는 동일, auto row만 증가하는지 검사한다. 관측 max non-auto delta는 0 CSS px다. **통과.**
17. **candidate가 baseline과 같은 입력이 아닌 경우:** key에 track position/order/cap/alignment/topology를 고정하고 mode만 `0px|auto`로 다르게 한다. 2,880 paired key가 확인됐다. **통과.**
18. **overflow 복구를 잘못 주장:** 사전 비교에서 baseline floor 위반도 0이다. 문서는 candidate가 overflow를 고쳤다고 말하지 않고 frame/track sizing 차이를 관측했다고 제한한다. **통과.**
19. **oracle/근거 자기참조:** Chrome 157은 고정 관측이고 normative eligibility·track/frame·alignment invariants는 W3C CSS Grid 2025 CRD에서 따로 도출한다. Spinon 후보 결과를 oracle로 사용하지 않는다. **통과.**
20. **플랫폼·수명 증거 과장 및 누수:** Chrome precomparison과 향후 Android/iOS visual smoke를 분리하고, shared simulator profile은 shutdown만 하며 task-owned profile은 제거한다. Agent Browser close 뒤 session/process 확인을 요구한다. **통과.**

사전 비교는 `5,760/5,760` scene, 2,880 baseline과 2,880 candidate를 관측했다. 이 검토는 계획 자체의 실패 관점이며, 계획 PR 변경분이나 구현/runtime 실패 관점 검토를 대체하지 않는다. 공식 C11.4/C11 상태와 `78/214`는 바꾸지 않는다.
