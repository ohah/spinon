# C11.3.3 양 끝 implicit named-line 계획 검토 20개

범위는 [`c11-3-3-both-implicit-named-line-pairs.md`](../../../../plan/c11-3-3-both-implicit-named-line-pairs.md) 및 [Chrome 사전 비교](./README.md)다. 계획의 위험과 누락을 구현 전에 서로 다른 관점으로 검토했다.

1. **Explicit line이 실제 이름과 일치:** 해석된 선이 explicit라면 pair mode에 들어가지 않게 provenance를 검사한다. PASS.
2. **Area alias 충돌:** bare ident가 `name-start/end` area edge를 가리킬 수 있으므로 `grid-template-areas`를 제외한다. PASS.
3. **한 endpoint만 implicit:** 두 endpoint별 표식을 모두 요구하고 혼합 placement를 기대 거부에 둔다. PASS.
4. **Negative occurrence/leading implicit:** 양수 trailing line 경계와 resolved index 검사가 signed mode와 분리된다. PASS.
5. **Numeric line:** 원 authored endpoint가 named line인지 검사해 숫자 line을 새 허용 대상으로 삼지 않는다. PASS.
6. **Named span:** span grammar를 양 끝 line placement로 오인하지 않고 기존 span resolver에 남긴다. PASS.
7. **같은 line 충돌:** §8.3.1 end 제거와 1-track 결과를 `alpha/alpha` scene으로 따로 고정한다. PASS.
8. **역순 충돌:** §8.3.1 swap을 2/1 occurrence scene에 고정한다. PASS.
9. **2개 이상 track의 sizing:** 1/3 scene에서 30px contribution이 15px 두 track으로 분배되는지 본다. PASS.
10. **한 축에만 implicit track:** Rows와 Columns case 모두 포함하고 opposite axis는 definite로 둔다. PASS.
11. **양 축 implicit 배치:** 런타임 negative matrix에서 거부하도록 명시했다. PASS.
12. **item 크기 변동:** width와 height 둘 다 fixed 조건으로 기존 auto minimum 영향 범위를 한정한다. PASS.
13. **여러 layout-participating item:** container 단일 item guard와 추가 child negative scene을 요구한다. PASS.
14. **숨김/비표시 item:** `display:none`은 제외하지만 layout에 참여하는 `visibility:hidden`은 단일 item 판정에서 유지하도록 구현 요구에 명시했다. PASS.
15. **Track-list 반복/복합 grammar:** 자동 sizing은 기존 `auto`/fixed-px subset으로 한정하고 나머지는 거부한다. PASS.
16. **10,000 line 경계:** expanded track ceiling을 pair line 계산 뒤에도 검증하도록 계획에 넣었다. PASS.
17. **정수 overflow:** occurrence→resolved line 변환의 checked arithmetic 및 초과 거부를 포함한다. PASS.
18. **Frame과 track 관측 구분:** CSSOM track list와 item/root frame을 각각 비교해 같은 결과 하나로 합치지 않는다. PASS.
19. **플랫폼 증거 과장:** AVD, iOS Simulator, surface sample, compositor stage를 분리해서 기록한다. PASS.
20. **공식 완료 상태 승격:** C11.3/C11 체크와 `72/213`을 고정하고 C11 candidate log만 갱신한다. PASS.

결론: 범위와 배제 기준이 구현 진입에 충분히 구체적이다. 계획 PR 실제 diff는 별도 검토 파일에서 다시 20개 관점으로 확인한다.
