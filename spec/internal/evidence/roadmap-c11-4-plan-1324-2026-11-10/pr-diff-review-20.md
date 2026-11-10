# PR #1325 roadmap diff · 독립 실패 관점 검토 20개

1. **기준 PR 정체성** — GitHub PR #1324의 제목·URL과 roadmap 링크를 대조했다. 계획 PR을 구현 PR로 표시하지 않았다.
2. **머지 시각** — PR의 `mergedAt` `2026-11-10T01:54:27Z`와 UTC 표기를 대조했다.
3. **머지 커밋** — roadmap의 SHA가 GitHub의 `f14b9bc640ed770d132b09f065e967a0bee79596`과 일치한다.
4. **기존 최신 구현** — #1322가 최신 구현 slice로 남아 있고 #1324가 이를 대체한 구현으로 서술되지 않는다.
5. **계획과 구현 구분** — #1324 문구가 계획 병합, 제품 구현 대기라고 명시하며 구현 완료를 주장하지 않는다.
6. **대상 row span** — 계획 링크의 제목과 변경 문구가 one-fit repeated-text row-span 범위를 가리킨다.
7. **위치 범위** — positions 1–3만 기록하며 후속 위치로 일반화하지 않는다.
8. **CSS 값 범위** — `min-height:auto`를 대상으로 기록하고 `min-height:0` control을 자동 최소 크기 동작으로 오기하지 않는다.
9. **Profile 격리** — 계획된 제한 profile을 일반 CSS Grid 또는 전역 intrinsic sizing 지원으로 확대하지 않는다.
10. **장면 분모** — paired 5,760 scenes를 baseline 2,880과 candidate 2,880으로 정확히 나눈다.
11. **항목 분모** — 6,048은 automatic-minimum 대상 item occurrence이며 scene 수와 혼동하지 않는다.
12. **Frame 변화 관찰** — 6,036 frame changes를 frame 개선·성공 수로 부르지 않는다.
13. **Content-floor 관찰** — baseline과 candidate 모두 violation 0으로 기록하며 회복된 항목이 있다고 주장하지 않는다.
14. **Overflow 관찰** — candidate visible overflow 0은 비교 관찰이며 overflow를 고쳤다는 인과 주장으로 쓰지 않는다.
15. **Oracle 경계** — Chrome 157 사전 비교로 명시하고 Spinon 구현 결과로 표현하지 않는다.
16. **플랫폼 경계** — Android/iOS runtime 미실행을 명시하고 simulator/device 증거를 암시하지 않는다.
17. **공식 집계** — `78/214`를 유지하고 계획 추가를 공식 완료 수 증가로 계산하지 않는다.
18. **상위 상태** — C11.4와 C11 모두 미완료로 남겨 roadmap의 next 문구와 일치시킨다.
19. **계약 버전** — 내부 계약 `0.1.0`을 그대로 유지한다.
20. **링크 무결성 범위** — 계획·비교 근거 route가 merged source에 존재하고 screenshot URL이 merge commit 안의 이미지 경로를 가리키는지 확인했다.

**결과:** 20개 관점을 확인했다. 결함을 발견하지 못했다. 이 검토는 roadmap 문구와 링크 대상 범위에 한정되며 전체 문서 사이트의 dead-link 검사나 제품 runtime 검증을 뜻하지 않는다.
