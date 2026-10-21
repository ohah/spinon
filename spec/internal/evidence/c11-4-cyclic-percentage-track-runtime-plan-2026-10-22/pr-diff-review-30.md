# C11.4 cyclic percentage track runtime 계획 PR — 변경 실패 관점 30개

**검토 대상:** 계획 문단, precomparison HTML, observation/expectation JSON, plan review 20, SHA-256 manifest. 구현 코드는 변경하지 않았다.

1. **변경 범위:** C11.4 하위 runtime fixture 계획만 추가했으며 다른 CSS 항목은 손대지 않았다. 통과.
2. **공식 상태:** `spec/STATUS.md`와 완료 집계를 바꾸지 않아 부분 slice가 완료 수에 섞이지 않는다. 통과.
3. **공개 지원 표현:** README는 내부 opt-in fixture로 한정하고 공개 CSS 지원을 주장하지 않는다. 통과.
4. **규범 출처:** CSS Grid 2025 CRD §7.2.1과 직접 연결했다. 통과.
5. **규범 의미:** cyclic `%`의 intrinsic 단계와 final resolution을 분리해 적었다. 통과.
6. **Chrome 버전 분리:** 기존 Chrome 155 case와 새 Chrome 156 결과를 구별한다. 통과.
7. **pinned oracle 보존:** 기존 155 observation을 수정하거나 대체하지 않는다. 통과.
8. **신규 Chrome 메타데이터:** UA, binary, DPR, viewport를 JSON에 고정했다. 통과.
9. **column basis:** 24px inner inline size의 50%가 12px임을 별도 column case로 고정했다. 통과.
10. **row basis:** 16px inner block size의 50%가 8px임을 다른 row case로 고정했다. 통과.
11. **definite control:** 40px definite inline size의 50% 결과 20px을 control로 둔다. 통과.
12. **intrinsic root 조건:** column/row case에서 각 해당 축의 max-content size를 명시한다. 통과.
13. **item dimensions:** 고정 24×16px item으로 재현을 단순화했다. 통과.
14. **box edge:** border/padding/margin이 0이어서 inner-basis 계산에 숨은 edge가 없다. 통과.
15. **gutter:** gap 0으로 percentage basis와 gutter contribution을 섞지 않는다. 통과.
16. **placement:** 한 item·한 explicit track으로 auto-placement ambiguity를 제외했다. 통과.
17. **alignment:** start 정렬과 zero offset을 입력에서 고정했다. 통과.
18. **overflow 용어:** grid border 밖 overflow라 과장하지 않고 track boundary 초과로 표현했다. 통과.
19. **표면 판정:** geometry와 surface pixel sample을 별도 acceptance로 요구한다. 통과.
20. **sample 좌표:** DPR과 runtime root frame을 기준으로 track 경계 표본을 정하도록 계획했다. 통과.
21. **trace completeness:** final Columns/Rows, scene revision을 acceptance에 둔다. 통과.
22. **partial output:** 계산 오류에서 partial output을 게시하지 않도록 기존 계약을 유지한다. 통과.
23. **fixture isolation:** fixture-test positive와 product/default negative 확인을 모두 요구한다. 통과.
24. **feature flag:** opt-in fixture boundary를 명시해 폴더명만으로 제외를 추정하지 않는다. 통과.
25. **scene ID consistency:** HTML, JSON, plan에서 세 ID가 일치한다. 통과.
26. **fixture location:** 새 CSS fixture는 `tests/fixtures/css/c11/` 아래에 있다. 통과.
27. **asset dependency:** 사전 비교 HTML에 remote asset, external script, font dependency가 없다. 통과.
28. **out-of-scope:** `calc()`, percentage `minmax()`·`fit-content()`, multi-track, nested, WPT를 분리했다. 통과.
29. **review separation:** plan review 20개와 PR diff review 30개가 별도 파일이다. 통과.
30. **unrelated work:** PR은 계획/oracle 7개와 이 별도 review 파일로 한정했고 기존 미추적 evidence는 stage하지 않았다. 통과.

**결론:** 문서/fixture diff에서 차단 결함을 찾지 못했다. 구현 PR은 별도 20-point implementation review와 별도 30-point PR review를 새로 수행한다.
