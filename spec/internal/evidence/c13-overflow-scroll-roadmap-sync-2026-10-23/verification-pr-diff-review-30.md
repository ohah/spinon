# C13 Tailnet 재확인 evidence PR diff 적대 검토 30개

1. **실제 route** — screenshot과 DOM JSON의 origin은 지정 Tailnet `/spinon/roadmap.html`이다.
2. **실제 문서** — browser title은 `스피논 구현 로드맵`이다.
3. **재요청** — 증거는 PR #565 merge 뒤 새 navigation으로 생성했다.
4. **main merge** — 관찰 설명의 merge SHA가 PR #565의 merge commit과 일치한다.
5. **page fetch** — roadmap source가 main `spec/STATUS.md`를 요청한다.
6. **cache policy** — source의 fetch가 `no-store`임을 확인했다.
7. **비교 대상 동일성** — before/after JSON은 같은 URL과 C13 task title을 가진다.
8. **화면 크기** — after screenshot은 1280×633 PNG다.
9. **DPR** — after observation은 DPR 1을 기록한다.
10. **가시성** — C13 title rect가 viewport 안에 있는 상태에서 캡처했다.
11. **집계 값** — UI는 완료 72, 표시 항목 213을 표시한다.
12. **checked count** — DOM에는 checked 72/213이 있다.
13. **C13 checkbox** — target checkbox의 `checked`는 false다.
14. **disabled** — roadmap checkbox는 읽기 전용으로 disabled다.
15. **접근성 이름** — label은 `overflow·scroll·클리핑 공식 미완료`다.
16. **C13.2 요약** — scroll owner·offset snapshot과 root/body/lifecycle이 표시된다.
17. **C13.2 residual** — ink-only box-shadow overflow가 fail-closed라고 표시된다.
18. **C13.3 router** — typed policy와 delta router 요약이 표시된다.
19. **C13.3 fixture** — Android/iOS direct-pan·wheel 경로가 opt-in fixture로 표현된다.
20. **native wheel 한계** — 플랫폼별 부분 근거를 full support로 바꾸지 않는다.
21. **CSSOM 경계** — public DOM scroll event/CSSOM은 미구현으로 표시된다.
22. **displayed-frame 경계** — gesture fixture의 성공을 displayed-frame input 지원으로 확대 해석하지 않는다.
23. **C13.4** — scrollbar/gutter가 완료 처리되지 않았다.
24. **C13.5** — scroll snap이 완료 처리되지 않았다.
25. **C13.6** — logical axis가 완료 처리되지 않았다.
26. **sticky context** — C12.5 sticky는 C13과 별도 unchecked row로 남는다.
27. **before artifact** — 변경 전 화면과 관찰 JSON은 삭제·덮어쓰지 않았다.
28. **after hashes** — after PNG와 JSON의 SHA-256을 manifest에 추가했다.
29. **status integrity** — Agent Browser 캡처는 대장 checkbox나 집계를 수정하지 않는다.
30. **release scope** — 이 evidence는 Tailnet 문서 렌더만 증명하며 제품 UI/runtime 완료를 주장하지 않는다.

**결과:** 30개 관점을 실제 route, page DOM, screenshot과 source fetch 경계에서 대조했다. 이 PR은 사후 확인 근거만 추가한다.
