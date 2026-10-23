# C11.4 percentage maximum 계획 PR 변경 적대 검토 30개

이 문서는 계획 PR의 실제 staged diff를 대상으로 했다. 계획 자체 실패 관점 20개와 구현/runtime 검토에는 포함하지 않는다.

1. **상태 대장 anchor** — 새 계획의 "#c11-css-grid" 링크는 공식 C11 section을 가리키며 checkbox를 수정하지 않는다.
2. **상위 계획 link** — parent plan의 focused-plan 상대 경로는 새 파일에 연결된다.
3. **precomparison link** — parent plan의 evidence README 경로는 추가 폴더에 연결된다.
4. **추가 위치** — 새 계획 요약은 parent plan 첫 dated section에 들어가 기존 날짜별 기록과 순서를 보존한다.
5. **문서 언어** — parent-plan 변경을 한국어로 고쳐 프로젝트 언어 규칙을 만족한다.
6. **추가 문자** — 첫 diff 검토에서 parent heading 앞 literal plus를 발견해 제거했고 실제 Markdown heading으로 재확인했다.
7. **공백** — 첫 diff 검토에서 Markdown hard-break trailing spaces를 발견해 제거했고 git diff --cached --check에서 깨끗함을 확인했다.
8. **규범 섹션** — fixed-size grammar는 §7.2.3.1, repetition sizing은 §7.2.3.2, track sizing은 §11.4로 나눠 참조한다.
9. **범위 표현** — 문서가 전체 C11.4 구현이나 일반 CSS 지원을 주장하지 않는다.
10. **Profile 경계** — 현재 opt-in RuntimeBlockFlexGridTrackSizingV1 경로로 명시하고 새로운 public API/profile을 열지 않는다.
11. **계산 책임** — 계획은 새 parser/evaluator 또는 vendored Taffy 수정 대신 typed-value gate 확장을 제안한다.
12. **Taffy 근거** — pinned 0.15.0 source의 definite maximum 선택과 max/min flooring 동작에 맞춰 구현 가설을 제시한다.
13. **percentage basis** — columns와 rows에 각자 definite 184px content axis를 주고 반대 축/viewport/border를 배제한다.
14. **box model** — 2px border가 있어도 content basis가 184px임을 oracle의 content-box geometry와 분리해 기록한다.
15. **gap 계산** — 8px gap과 N−1 gutter를 fixed control 및 3/4-track boundary로 검증한다.
16. **maximum 우선 선택** — 25% maximum이면 min 16px에서 반복 수가 3으로 달라지는 실패 대조를 넣었다.
17. **minmax flooring** — maximum 20% < minimum 25% 케이스가 valid CSS와 46px effective max로 고정된다.
18. **zero maximum** — 0% max는 positive 25% min에 floor되고 zero-sized repeat을 만들지 않는 입력으로 분리했다.
19. **calc maximum** — calc는 20px+10% 한 식으로 제한하고 arbitrary math admission을 주장하지 않는다.
20. **subpixel tolerance** — Chrome의 38.390625px 관찰과 ideal 38.4px 차이를 1/64px 한도로 명시했다.
21. **단일 축 matrix** — columns와 rows, auto-fill/auto-fit, min/max flooring, zero max, calc 및 fixed control을 각각 유지한다.
22. **양축 범위 일치** — 첫 계획 diff 검토에서 cross-axis 동시 계산 누락을 찾아 8개 combined scenes를 추가했다. 후속 검토에서 집중 계획 5번의 오래된 제외 문장을 발견해 각 축 184px basis와 동시 반복 범위로 고쳤고, 8개 scene ID 및 20개 계획 검토 항목과 다시 대조했다.
23. **auto-fill 시점** — 빈 반복 track을 유지해 item 두 개가 세/네 track count를 바꾸지 못하게 했다.
24. **auto-fit 시점** — pre-collapse count와 post-placement [46,0,0]을 분리하고 fixed control을 둔다.
25. **scene 총계** — oracle, expectation JSON, plan과 parent summary 모두 28 scenes를 가리킨다.
26. **oracle display** — display:grid 누락으로 생긴 첫 초안 측정은 버렸고 최종 HTML에서 실제 grid computed tracks를 얻는다.
27. **Browser identity** — 관찰 JSON은 Agent Browser Chrome 156, viewport와 DPR을 보존하고 터미널 브라우저를 사용하지 않았다.
28. **screenshot proof** — full-page screenshot은 final 28-scene oracle를 보여주고 PNG hash/크기를 별도 확인했다.
29. **independent expectation** — 규범 수식 및 fixed controls는 Chrome observation과 다른 JSON으로 보존했고 28 scene 비교에서 불일치가 없다.
30. **hash 및 untracked scope** — precomparison SHA256SUMS가 assets와 일치하며 staged path는 9개 계획/evidence 산출물과 이 review 기록으로 한정한다. 기존 미추적 실행 evidence는 stage하지 않았다.

**결과:** 수정 후 30관점을 다시 대조했다. plan review 20개는 별도 파일이며 계획 PR diff 30개나 향후 구현 review에 중복 집계하지 않는다.
