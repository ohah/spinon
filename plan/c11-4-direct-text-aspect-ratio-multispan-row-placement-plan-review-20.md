# C11.4 ratio span row placement 보정 계획 검토 20개

**검토 대상:** canonical oracle가 실제로 요구하는 one-item span placement와 계획·입력 기대값의 일치.

| # | 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | 계획이 여전히 `grid-row:1` 입력을 요구하는가 | **수정됨.** 계획과 HTML 모두 row start/end를 `auto`로 고정했다. |
| 2 | computed row placement를 Agent Browser가 관측하는가 | **통과.** observation JSON에 `gridRowStart`와 `gridRowEnd`가 있다. |
| 3 | verifier가 geometry만 보고 placement 불일치를 놓치는가 | **수정됨.** 9개 전 case에서 두 computed row 값이 `auto`인지 확인한다. |
| 4 | column span 계약이 row 자동화 수정과 함께 바뀌는가 | **통과.** column start `1`, end `span 2`는 유지하고 verifier도 계속 검사한다. |
| 5 | row 입력 수정이 track/frame 기대값을 바꾸는가 | **통과.** Agent Browser 156에서 9개 track/row/frame 값은 기존 독립 계산과 같다. |
| 6 | item이 다른 explicit placement로 auto row 요구를 우회하는가 | **통과.** row start와 end 양쪽을 검사한다. |
| 7 | root에 auto row가 없는 경우와 item row auto를 혼동하는가 | **통과.** root의 `grid-template-rows:auto`와 item placement의 auto longhands를 따로 기록한다. |
| 8 | `grid-row:1` 제거가 암묵적 row track 선택을 바꾸는가 | **통과.** root는 auto row 하나이며 auto placement는 같은 단일 row에 배치된다. |
| 9 | no-ratio control에만 placement 검사를 생략하는가 | **통과.** verifier는 ratio/control 구분 없이 아홉 case 전부에 적용한다. |
| 10 | Chrome 결과를 규범 계산의 유일한 출처로 취급하는가 | **통과.** 독립 CSS Grid 11.5.1 계산은 그대로며 Chrome은 corroboration이다. |
| 11 | font readiness가 바뀌어 geometry가 우연히 유지되는가 | **통과.** `fontReady`와 pinned HeadlessChrome 156 검사가 유지된다. |
| 12 | HTML 수정 뒤 stale observation JSON이 남는가 | **수정됨.** Agent Browser를 다시 실행해 아홉 observation을 갱신했다. |
| 13 | canonical fixture SHA가 이전 HTML을 가리키는가 | **수정됨.** README hash를 새 HTML bytes에서 다시 계산한다. |
| 14 | expected-values가 실제 row placement를 설명하지 않는가 | **수정됨.** 고정 placement를 root basis에 추가하고 verifier가 사용한다. |
| 15 | screenshot만 이전 실행이라 metadata 불일치가 보이지 않는가 | **통과.** 새 실행으로 다시 캡처했으며 이미지 bytes와 시각 geometry는 동일하다. |
| 16 | screenshot이 simulator 또는 Spinon runtime 결과로 오인되는가 | **통과.** 기존처럼 Chrome oracle precomparison으로만 표기한다. |
| 17 | AVD/iOS 수용 조건이 CSS oracle와 다른 row placement를 쓰는가 | **수정 기준.** 계획은 row auto를 요구하며 새 simulator scenes도 그 입력을 사용해야 한다. |
| 18 | 보정이 C11.4 구현 완료로 대장에 반영되는가 | **통과.** `spec/STATUS.md`와 공식 `72/213`은 변경하지 않는다. |
| 19 | local implementation draft가 검증 완료로 오해되는가 | **통과.** 상태 문구는 draft 진행과 simulator/runtime 미검증을 구분한다. |
| 20 | downstream runtime trace가 placement 원인을 복원할 수 있는가 | **수용 기준.** computed style과 node placement를 결과와 함께 보존하고 span contribution/frame을 확인한다. |

이 검토는 입력 계약 보정에 한정한다. CSS Grid 전체 지원, runtime 검증, simulator 실행 또는 공식 완료 판정이 아니다.
