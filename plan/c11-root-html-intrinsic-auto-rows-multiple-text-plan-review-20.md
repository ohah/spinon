# 여러 direct text item implicit-row 계획 적대 검토 20개

**검토 대상:** [`c11-root-html-intrinsic-auto-rows-multiple-text.md`](./c11-root-html-intrinsic-auto-rows-multiple-text.md) · **비교 근거:** [Chrome 156 및 독립 기대값](../spec/internal/evidence/c11-root-html-intrinsic-auto-rows-multiple-text-precomparison-2026-11-06/README.md) · **기준 코드:** `ddfb2782fb6b5d3b8eeeff0df9c1c24f58d95121` (`main`, 2026-11-06)
**판정:** 20개 관점 검토 후 계획 보정사항을 반영했다.

1. **규범 범위:** CSS Grid §7.6, §11.1, §11.5를 기준으로 implicit row 및 item contribution을 설명한다. 이 slice를 전체 sizing algorithm 완료로 확대하지 않아 통과.
2. **Auto-placement 위치:** 첫 item 뒤의 long/short text item은 각기 다른 row에 놓이고 마지막 fixed item도 독립 row에 배치되는 장면을 고정해 통과.
3. **Contribution identity:** 기대값은 long item 20px, short item 10px를 따로 둔다. 합계 높이만 비교하지 않고 item별 identity를 확인하도록 계획해 통과.
4. **Source-order 역전:** 초기 계획에 역전 oracle이 빠졌음을 발견했다. Agent Browser에서 text 두 item의 순서만 바꿔 rows `[10,10,20,34]px`와 frame을 확인했고 관측·기대 JSON·계획에 추가해 해결.
5. **Text 문법 경계:** 기존 한 문자열 allowlist를 확장하면서 임의 text로 잘못 확대할 위험을 확인했다. `A` token과 ASCII 공백 하나만 허용하는 grammar와 다른 문자/공백 거부를 명시해 해결.
6. **줄 수 출처:** Chrome 줄 수를 simulator 예상값으로 복사할 위험을 확인했다. 20px/10px는 독립 기대값이고 native runtime은 각 runtime의 실제 text measurement와 별도로 비교하도록 명시해 통과.
7. **Computed style:** font size만 보고 text 동작을 선택할 위험을 확인했다. weight/style/line-height/whitespace/wrapping/direction/writing-mode/text-transform/alignment typed-property 조건을 계획에 고정해 통과.
8. **Font fallback:** `monospace`가 모든 OS에서 같은 face/advance임을 가정할 위험을 확인했다. font metrics parity를 주장하지 않고 각 플랫폼 line count/frame을 실제로 확인하도록 명시해 통과.
9. **Text snapshot 수명:** NodeId만 일치하고 style/layout revision이 달라지는 입력을 놓칠 위험을 확인했다. owner, text node, style/layout revision 동시성을 요구해 통과.
10. **Owner 혼합:** 다른 Grid owner의 text가 candidate에 섞일 위험을 확인했다. owner 한 개 및 item→owner identity 유지 조건을 명시해 통과.
11. **Hidden item:** 숨겨진 잘못된 text가 visible item 수나 contribution을 바꾸는지 확인하지 않을 위험을 확인했다. 기존 visibility 처리를 보존하고 hidden-item regression을 요구해 통과.
12. **Nested/anonymous text:** 여러 text source 형태를 한 direct text로 오인할 위험을 확인했다. direct child 하나만 허용하고 inline markup/anonymous item을 거부하도록 명시해 통과.
13. **Atomic failure:** 두 번째 item에서 오류가 나도 앞 item frame이 남을 위험을 확인했다. 모든 rejection에서 completed frame/paint 부재를 확인하도록 명시해 통과.
14. **Track/resource 상한:** 여러 항목을 허용하며 10,000 implicit-track 정책을 우회할 위험을 확인했다. 10,000/10,001 경계를 무거운 text measurement 없이 확인하도록 계획에 추가해 해결.
15. **Cardinality 성능:** 단일 text cap 제거 후 item마다 전체 style list를 다시 검색해 O(n²) 동작이 될 위험을 발견했다. 미리 만든 node index를 재사용하도록 구현 요구에 추가해 해결.
16. **Fixed track 기준:** 이번 plan이 intrinsic-column 조합의 선행 slice와 겹칠 위험을 확인했다. positive fixed explicit column/row route에만 확장을 한정하고 결합 intrinsic-column behavior는 그대로 둬 통과.
17. **Track trace:** final row sizes만 확인하고 contribution phase가 빠진 결과를 통과시킬 위험을 확인했다. row trace의 contribution identity/kind/value와 최종 stage를 확인하도록 통과.
18. **Overflow 해석:** 화면 아래로 넘친 item을 잘림/paint 확인으로 오해할 위험을 확인했다. 계산 geometry만 overflow 근거로 기록하고 pixel clip 주장을 제외해 통과.
19. **Artifact 경계:** 새 성공 경로가 fixture-test 전용 ABI에 실수로 들어갈 위험을 확인했다. 기본 Root HTML route를 유지하고 default artifact/fixture boundary를 검증하도록 통과.
20. **로드맵 판정:** 제한 slice 병합만으로 C11.4 또는 공식 count를 완료 처리할 위험을 확인했다. 내부 계약·STATUS·roadmap 근거만 추가하고 C11.4/C11 및 공식 완료 수는 유지하도록 명시해 통과.

## 보정 후 판정

발견된 계획 결손은 source-order 역전 oracle, text grammar의 정확한 경계, resource cap 시험, 그리고 다중 item style lookup의 성능 위험이었다. 각 항목을 계획과 사전 비교 근거에 반영했다. 남은 미검증 항목은 구현 PR의 테스트·runtime 검토에서 확인하며 계획 PR에서는 기능 완료로 표시하지 않는다.
