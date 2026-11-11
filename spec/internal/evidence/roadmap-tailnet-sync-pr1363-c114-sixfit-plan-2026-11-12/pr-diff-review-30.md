# Tailnet 로드맵 동기화 PR 변경 검토 30개

검토 대상은 `spec/public/roadmap.html`의 두 CSS 선언 변경과 선택된 Tailnet 반영 증거다. 각 항목은 별도 실패 경로 또는 경계로 확인했다.

1. **제목 flex 축소** — 긴 형제 요약이 제목을 세로 글자처럼 눌렀던 경로를 `flex:none`으로 차단했다.
2. **요약 flex 수축** — 요약 텍스트는 `min-width:0`으로 남은 폭 안에서 줄바꿈된다.
3. **긴 URL·식별자** — `overflow-wrap:anywhere`가 공백 없는 토큰의 가로 넘침을 막는다.
4. **줄 간격** — 명시한 `line-height:1.6`으로 여러 줄 요약의 행 겹침을 피했다.
5. **제목 가독성** — 제목의 기존 28px serif 규칙과 정렬 기준은 유지했다.
6. **요약 시각 계층** — 기존 11px monospace 및 soft 색상을 유지했다.
7. **데스크톱 폭** — Agent Browser 1440px에서 document/body 모두 1440px이고 제목·요약이 나란히 배치된다.
8. **모바일 폭** — Agent Browser 390px에서 document/body 모두 390px이며 가로 overflow가 없다.
9. **작은 viewport** — 요약의 최소 폭을 제거해 flex 자식이 컨테이너를 밀어내지 않는다.
10. **긴 상태 문구** — 현재 roadmap summary의 긴 C11.4 설명이 여러 줄로 표시된다.
11. **다음 후보 표시** — 실제 페이지의 C11.4 six-fit 후보가 데스크톱·모바일 모두에서 확인된다.
12. **완료 수 보존** — 화면은 공식 `78/214`를 표시하고 문서 변경은 상태 원장을 수정하지 않는다.
13. **상위 완료 승격 방지** — C11.4/C11 미완료 문구를 유지한다.
14. **헤더 이외 범위** — 변경 선택자는 `.section-heading` 안에 한정되어 hero와 본문 문단을 건드리지 않는다.
15. **로드맵 SVG 스크롤** — `.roadmap-figure`의 기존 가로 스크롤과 720px 최소 SVG 폭을 수정하지 않았다.
16. **테마 변수** — CSS custom properties와 light/dark 색상 토큰을 건드리지 않았다.
17. **정적 렌더링** — 상태 문구의 표시에는 JavaScript 실행을 새로 요구하지 않는다.
18. **선택 빌드 범위** — Rspress 전체 사이트 성공으로 확대 해석하지 않고 선택 빌드만 기록했다.
19. **dead-link 검증 범위** — 전체 dead-link 검사는 비활성화됐다고 명시했다.
20. **선택된 경로 수** — 동기화 manifest의 36개 경로만 검증한다.
21. **변경 경로 수** — pass02에서 roadmap HTML 1개만 교체되고 35개가 그대로다.
22. **정적 산출물 해시** — manifest가 source, preview, localhost 및 Tailnet 응답의 SHA-256 일치를 기록한다.
23. **HTTP 상태** — 36개 경로가 localhost와 Tailnet에서 HTTP 200으로 검증됐다.
24. **기존 preview 보존** — 확인된 사전 HTML을 pass02 backup 경로에 복사한 뒤 교체했다.
25. **백업 복구 가능성** — 덮어쓴 원본 경로·바이트·해시가 manifest에 남는다.
26. **비선택 파일 안전성** — 동기화 스크립트는 manifest 밖 경로를 삭제하거나 덮지 않는다.
27. **서버 프로세스** — 사용자가 선택한 PID 90711을 확인했고 재시작하지 않았다.
28. **PR/미리보기 상태 구분** — Tailnet 반영 사실을 GitHub merge나 전체 사이트 배포로 표현하지 않는다.
29. **Agent Browser 수명** — 확인 후 세션을 닫았으며 세션 목록과 프로세스 검색이 비어 있다.
30. **근거·상태 정합성** — 수정 전/후 스크린샷, JSON 측정, manifest를 연결하고 CI 0개 및 미완료 상태를 명시한다.

## 검토 결과

이번 검토에서 남은 결함은 발견하지 못했다. 화면 수치는 `agent-browser-verification-final.json`, 복사·HTTP 근거는 `tailnet-copy-manifest-pass02.json`과 `served-verification-pass02.json`에 기록했다. 저장소의 기존 미추적 증거 파일 및 preview backup은 이 변경과 무관하므로 보존했다.
