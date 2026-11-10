# Tailnet 동기화 후속 PR 적대 검토 30개 관점

**대상:** #1335 병합 뒤 RSPress 선택 산출물을 기존 Tailnet 미리보기 서버에 전달하고, 로컬·원격 경로와 화면 근거를 추가한 문서 증거 변경.

1. 소스 HEAD와 tree를 혼동하지 않는가 — PASS: manifest는 `83e5bdd7c440d6049d614e8ee3a9194c83f206ac`와 tree `13af91e245fecebd077eee2e3b60da59673322eb`를 함께 기록한다.
2. 선택 빌드가 저장소의 임의 파일을 따라가지 않는가 — PASS: 입력 파일 경로의 각 구성요소에서 symlink를 거부한다.
3. evidence directory 순회가 symlink를 따라가지 않는가 — PASS: 디렉터리·자식 항목의 symlink를 거부하고 일반 파일만 복사한다.
4. Markdown 이미지 경로가 저장소 밖으로 나갈 수 있는가 — PASS: 정규화된 경로가 저장소 root 안인지 확인한다.
5. route 경로 traversal을 허용하는가 — PASS: 절대 경로, 역슬래시, 빈 구성요소, `.` 및 `..` route를 거부한다.
6. 같은 route의 서로 다른 입력을 조용히 덮어쓰는가 — PASS: 중복 route는 SHA-256이 같을 때만 허용한다.
7. 선택 목록에 의도하지 않은 저장소 전체 파일이 들어가는가 — PASS: 문서, 계획, fixture, 세 evidence directory를 명시적으로 선택한다.
8. RSPress 버전이 프로젝트 고정값과 다른가 — PASS: `rspress.config.ts`는 설치된 `@rspress/core` 2.0.22를 사용한다.
9. Tailnet 시작 페이지와 404 경로를 덮어쓰는가 — PASS: `index.html`과 `404.html`은 publication에서 제외한다.
10. selected root와 publication의 파일 수·크기 집계가 빠졌는가 — PASS: 182개/90,244,678 bytes와 263 route/103,888,398 bytes를 기록한다.
11. 공식 완료 수를 제품 구현 수치로 부풀리는가 — PASS: `78/214`를 기록하고 `c11Complete: false`를 유지한다.
12. C11 또는 C11.4 체크 상태를 완료로 바꾸는가 — PASS: 선택 단계에서 두 항목이 미완료인지 검사한다.
13. 기존 #1331 구현 기록의 merge SHA가 빠졌는가 — PASS: roadmap 카드, 계약 문서, 두 local link route를 검사한다.
14. #1334 계획 merge 정보가 잘못 연결되는가 — PASS: plan merge SHA와 세 local link route를 검사한다.
15. 칸반 중복 제목을 다시 노출하는가 — PASS: 선택된 roadmap에서 `kanban-heading`이 하나인지 검사한다.
16. 전체 문서 링크 검사를 한 것처럼 주장하는가 — PASS: manifest와 README에서 `fullDocsLinkValidation: false` 및 해당 한계를 명시한다.
17. 기존 preview에 없는 route를 실수로 제거하는가 — PASS: 절차는 create/replace/same만 수행하고 삭제는 0으로 기록한다.
18. 다른 서버의 8768 포트를 잘못 갱신하는가 — PASS: listener가 하나이며 예상된 `tailnet-preview-server.mjs`인지 확인한다.
19. preview server를 재시작하거나 종료하는가 — PASS: 기존 PID `90711`을 확인했고 재시작하지 않았다.
20. Tailnet 연결 불능 상태에서 복사를 진행하는가 — PASS: 사전 점검에서 backend `Running`과 local HTTP 200을 요구한다.
21. 교체 전에 열려 있는 route를 덮어쓰는가 — PASS: preflight의 교체 대상 `lsof` open handle 수는 0이다.
22. 이전 파일 백업을 기존 경로에 덮어쓰는가 — PASS: 고유 pass 경로가 이미 있으면 중단하고 기존 파일을 보존한다.
23. 대체 파일 백업 전에 원본을 덮어쓰는가 — PASS: 29개 교체 대상의 이전 bytes를 먼저 별도 rollback root에 복사하고 hash를 확인한다.
24. preflight 이후 대상 변경을 감지하지 못하는가 — PASS: copy 직전 대상 존재 상태와 hash를 다시 검사한다.
25. preflight 이후 publication 원본 변경을 감지하지 못하는가 — PASS: 각 source의 SHA-256을 copy 직전에 재검사한다.
26. 부분 기록 파일이 live route에 노출되는가 — PASS: 고유 임시 파일에 쓴 다음 같은 파일시스템에서 rename하고 최종 hash를 검사한다.
27. 기존 파일 권한을 의도치 않게 바꾸는가 — PASS: replace는 기존 mode를 유지하고 create는 `0644`로 생성한다.
28. 동기화 뒤 로컬 또는 HTTPS route 일부가 빠지는가 — PASS: 263/263 local hash 및 HTTPS HTTP 200/hash를 확인했다.
29. 이전 29개 파일의 rollback 사본이 실제로 보존되었는가 — PASS: 29/29 backup hash를 원본 이전 hash와 비교했다.
30. 화면 캡처나 PR 이미지를 열지 못하거나 모바일 페이지가 잘리는가 — PASS: Agent Browser desktop 1440×1000 및 mobile 390×844 캡처를 확인했고, 양쪽에서 78/214와 NEXT 카드를 보았다. 모바일 document/body 폭은 390px이고 칸반 내부만 가로 스크롤하며, 세션 종료 후 목록은 비었다.

## 검토 범위와 한계

- `git diff --check`, JSON 파싱, PNG 크기와 SHA-256, 기록된 route hash·HTTP 상태를 점검한다. 제품 테스트, CI, 앱 실행 또는 iOS/Android runtime 결과를 새로 주장하지 않는다.
- 변경은 Tailnet 정적 문서 route 전달과 증거에 한정된다. PR 병합 뒤 사용자 요청대로 브라우저 세션을 종료하고, 작업이 끝난 task build 및 병합된 브랜치만 정리한다.
