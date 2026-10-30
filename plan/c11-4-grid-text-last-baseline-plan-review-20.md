# C11.4 direct text multiline last-baseline 계획 적대 검토 20개

**대상:** [계획](./c11-4-grid-text-last-baseline-multiline.md), [사전 비교 자료](../spec/internal/evidence/c11-4-grid-text-last-baseline-precomparison-2026-10-30/README.md), 고정 HTML fixture. 제품 구현 검토가 아니다.

| # | 계획을 깨뜨릴 수 있는 입력·해석 | 판정 근거 | 결과 |
|---:|---|---|---|
| 1 | last baseline을 first baseline으로 처리해 서로 다른 multiline 결과를 놓친다. | 작은 Ahem item의 first/last offset은 8px/18px이고 fixture 및 expected JSON이 이를 따로 고정한다. | PASS |
| 2 | 일반 `Baseline` keyword로 축약되어 Taffy가 어느 baseline을 선택해야 하는지 잃는다. | 계획은 Grid 내부 baseline kind 보존을 필수 계약으로 둔다. 구현 불가면 이 plan은 진행하지 않는다. | PASS |
| 3 | vendored Taffy의 last-baseline TODO를 무시하고 Spinon output에 `last`만 채우면 된다고 가정한다. | 계획은 Taffy Grid item selector와 participation/fallback/shim 변경까지 요구한다. | PASS |
| 4 | item 하나만 baseline-aligned여 shim이 0인데 두 item 정렬을 기대한다. | 두 positive item은 동일한 row의 baseline group에 모두 참가한다. | PASS |
| 5 | auto-placement가 reverse scene을 서로 다른 row에 둔다. | 각 item은 명시된 `grid-row:1`과 의미별 `grid-column`에 배치된다. | PASS |
| 6 | reversed source-order 장면이 열 순서도 바꿔 geometry 차이를 만든다. | columns는 `.small`/`.large`에서 명시하므로 DOM order만 바뀐다. | PASS |
| 7 | wrapper intrinsic Grid가 고정 폭/높이에 관여해 baseline 관측을 왜곡한다. | fixture는 바깥 `.case`를 block으로 유지하고 Grid 외부에 별도의 Grid sizing context를 두지 않는다. | PASS |
| 8 | 작은 문자열이 25px 폭에서 의도대로 두 줄이 아니거나 빈 줄이 생긴다. | 고정 Ahem `XX XX`는 공백에서 두 글자씩 끊기며 line rect가 두 개다. Chrome 관측과 expected JSON 모두 line count를 담는다. | PASS |
| 9 | 큰 문자열이 40px 폭에서 wrap되어 last baseline 계산이 변한다. | `XX`는 20px Ahem glyph 두 개로 40px에 맞고 한 line rect만 있다. | PASS |
| 10 | CSS font fallback이나 비동기 font load가 baseline 값을 바꾼다. | 저장소 Ahem SHA와 browser `document.fonts.check` 10px/20px 결과를 기록한다. | PASS |
| 11 | Ahem ascent ratio를 하강폭 또는 font-size 자체로 잘못 사용한다. | Ahem UPEM/ascent는 1000/800이며 line baseline을 0.8em으로 독립 계산한다. | PASS |
| 12 | last group의 end shim 2px을 row height에 포함하지 않는다. | end shim은 small 2px/large 0px이며 `max(20+2,20+0)=22px`로 row contribution을 고정한다. | PASS |
| 13 | first baseline 회귀를 last-baseline 값과 혼동하거나 기존 one-line 경계를 넓힌다. | first control은 두 item 모두 한 줄이며 baseline 16px, small y=8px, row 20px다. multiline last group의 18px/2px/22px와 분리된다. | PASS |
| 14 | start fallback control에서도 baseline shim이 적용된다. | start control은 두 y=0px, row 20px이며 line baseline은 8px와 16px로 다르다. | PASS |
| 15 | `align-self` override가 부모 `align-items:start`보다 우선하는지 검증하지 않는다. | positive `explicit-self` scene은 parent start와 두 child last baseline을 지정한다. | PASS |
| 16 | geometry가 page coordinates와 섞여 `.case`의 margin/padding 영향을 받는다. | expected 및 observation 값은 각 Grid root에 상대화한다. | PASS |
| 17 | last-baseline fallback/scroll rules가 first-baseline 규칙과 같다고 가정한다. | plan은 per-kind fallback와 scroll clamping 근거를 확인하게 하고 이 slice가 이를 보장하지 못하면 중단한다. | PASS |
| 18 | font/paragraph/last line이 없을 때 item height fallback으로 성공한다. | fixed positive profile 외 missing data/non-finite baseline은 error이며 partial frame을 허용하지 않는다. | PASS |
| 19 | Chrome의 특정 build 관측을 CSS 규범 기대값으로 복사한다. | expected JSON은 font metrics, start/end shim 규칙과 fallback edge에서 독립 계산하고 Chrome JSON은 별도 evidence다. | PASS |
| 20 | 제한 fixture merge가 일반 CSS admission 또는 C11.4 완료로 오해된다. | plan은 fixture-only scope, public API unchanged, C11.4/C11 checkbox 및 72/213 유지 조건을 반복해 명시한다. | PASS |

20개의 서로 다른 입력·알고리즘·계약 경계를 계획 단계에서 확인했다. Taffy의 private last-baseline 전달을 구현 전에 검토하고, first-baseline 회귀가 없음을 확인한 뒤에만 implementation PR을 진행한다.
