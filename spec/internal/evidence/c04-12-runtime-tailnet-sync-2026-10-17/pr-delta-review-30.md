# C04.12 Tailnet roadmap sync PR 변경 실패 관점 30개

검토 대상은 `spec/public/roadmap.html` 설명 정정과 이 폴더의 배포·브라우저 검증 자료다. 모든 판정은 저장소 산출물 또는 실제 로컬/Tailnet 응답으로 확인했다.

1. **오래된 DOM-only 설명 잔존** — 통과. 배포된 문구에서 DOM 보존만 확인한다는 표현이 사라졌고 viewport·scheme cascade 실행을 적었다.
2. **viewport 효과를 단순 DOM 보존으로 축소** — 통과. Android/iOS CSS viewport 계산을 status와 실행 기록에 맞춰 표시한다.
3. **환경 입력과 cascade 연결 누락** — 통과. OS light/dark 선호가 `prefers-color-scheme`에 반영되고 revision이 변한 표본을 적었다.
4. **지원되지 않는 전체 media query 과장** — 통과. 전체 media-query 지원을 주장하지 않고 제한 profile로 서술했다.
5. **`meta color-scheme`이 OS 선호를 바꾼다는 오해** — 통과. document scheme snapshot과 OS 선호 cascade를 구분했다.
6. **CSS `color-scheme`의 UA 효과 과장** — 통과. computed snapshot까지만 보존한다고 명시했다.
7. **Canvas system color 지원 오인** — 통과. UA Canvas 색상 연결을 미지원으로 적었다.
8. **native 화면·form control 테마 지원 오인** — 통과. native 배경과 form control 테마를 미지원으로 적었다.
9. **`page_scale`이 실제 확대 변환에 적용된다는 오해** — 통과. snapshot 기록과 renderer transform 적용을 분리했다.
10. **HTML layout이 구현됐다는 오해** — 통과. 미지원 범위를 유지했다.
11. **HTML paint가 구현됐다는 오해** — 통과. 미지원 범위를 유지했다.
12. **빈 Simulator 화면을 HTML 동작 캡처로 오인** — 통과. README와 화면 이름에서 문서 Preview임을 구분했다.
13. **C04.12 완료 처리** — 통과. `STATUS.html`의 C04.12 체크가 해제돼 있고 문서도 미완료를 유지한다.
14. **완료 수 임의 증가** — 통과. 공식 대장 동기화 결과는 71/205로 기존 수를 유지한다.
15. **공식 수와 원시 체크박스 수 혼합** — 통과. browser 기록은 roadmap의 공식 71/205와 STATUS의 원시 항목을 다른 값으로 구분한다.
16. **내부 계약 링크 경로 오타** — 통과. Tailnet에서 0077 페이지를 직접 열어 제목과 내용을 확인했다.
17. **기본 앱 근거 링크 누락·깨짐** — 통과. Tailnet README route를 열고 DOM/V8 근거 내용을 확인했다.
18. **환경 cascade 근거 링크 누락·깨짐** — 통과. Tailnet README route를 열고 viewport·scheme 전환 기록을 확인했다.
19. **기존 parser/Chromium 근거 링크 누락** — 통과. 기존 route가 문서의 링크 목록에 남아 있고 최종 payload hash 검증에 포함됐다.
20. **정적 route와 raw Markdown 경로 혼동** — 통과. 생성 HTML과 raw spec 원본이 manifest의 서로 다른 목적 경로로 기록됐다.
21. **Tailnet host 오타** — 통과. 실제 Self DNS name `macstudio.tailed42f2.ts.net`을 사용하고 Agent Browser가 URL을 열었다.
22. **잘못된 Preview root에 쓰기** — 통과. preflight와 copy verification 모두 사전 승인된 SpinonPreview root를 기록한다.
23. **다른 Preview 파일 제거** — 통과. preflight 및 copy verification의 삭제 목록은 비어 있다.
24. **기존 교체 파일 무백업 덮어쓰기** — 통과. 교체 2개 모두 고유 백업 root에 저장하고 이전 SHA-256과 대조했다.
25. **사전 계획 후 대상 변경 덮어쓰기** — 통과. 복사 직전에 모든 대상 byte 수와 SHA-256을 preflight 값과 재대조했다.
26. **복사된 payload의 원본 손상** — 통과. 137개 source 파일을 destination별 기대 hash와 다시 비교했다.
27. **로컬 응답만 검증하고 Tailnet 미검증** — 통과. 두 endpoint에서 각각 137개 요청을 실행했다.
28. **HTTP 200이지만 응답 내용이 stale** — 통과. 총 274개 응답의 body size와 SHA-256이 빌드 기대값과 일치한다.
29. **선택 빌드를 전체 문서 검증으로 과장** — 통과. 임시 설정의 dead-link 검사 제한과 선택 빌드 범위를 README에 공개했다.
30. **브라우저·공유 세션 잔류** — 통과. 격리 Agent Browser 세션을 닫고 page count 0, browser launch 비활성, daemon PID 부재를 기록했다. tmux 세션은 건드리지 않았다.
