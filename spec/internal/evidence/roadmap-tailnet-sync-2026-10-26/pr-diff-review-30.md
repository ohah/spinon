# PR #654 · 변경 실패 관점 검토 30개

검토 대상은 [PR #654](https://github.com/ohah/spinon/pull/654)의 base `fb2671fbc81a65d686f8eae2c069cc86298cfdb7`부터 head `4cdea51d6c53684c758bb16a0f668c4f4cde871f`까지의 전체 docs/evidence diff다. 원격 PR은 OPEN·mergeable이고 로컬 checkout `HEAD`는 별도 작업 브랜치 `be53ebef6ea7b2734bb797338a8b6ddb43be90d0`라서 checkout을 바꾸지 않고 지정 commit object를 직접 대조했다. PR 본문에서 현재 head와 README 링크, 커밋 제목을 확인했다. statusCheckRollup은 비어 있으며 PR 본문도 CI 통과로 주장하지 않는다. 이 검토에서는 브라우저와 테스트를 실행하지 않았다. `git diff --check` 출력은 없었다.

1. **revision 고정** — GitHub base/head는 지정한 전체 SHA와 일치하고, 차이는 모두 `spec/internal/evidence/roadmap-tailnet-sync-2026-10-26/` 아래의 증거 파일이다. **PASS**
2. **제목 언어 규칙** — PR 제목 `docs(c11-4): Tailnet 미리보기 동기화 근거`와 두 커밋 제목은 영어 type/scope 접두어 뒤 설명을 한국어로 쓴다. Tailnet은 제품명으로 원어 사용이 적절하다. **PASS**
3. **PR 상태와 CI** — PR은 OPEN·mergeable이며 review decision과 statusCheckRollup은 비어 있다. **PASS — CI 통과로 해석하지 않음**
4. **라벨** — `documentation`, `chore`, `area: docs`가 있어 작업 유형과 주요 영역을 표시한다. **PASS**
5. **PR 본문·체크박스** — 상태 파일을 바꾸지 않은 이유와 플랫폼 변경 없음이 unchecked 상태와 본문 설명으로 일치한다. 전체 root 빌드 실패와 선택 빌드 성공을 구분하고 빈 GitHub check rollup이 통과가 아님을 쓴다. **PASS**
6. **전체 root 실패 설명** — 수정된 README는 전체 root 실패의 `Module not found` 진단 268회를 기록하고 과거 Rspress 설정의 `@rspress/core` import 실패와 PNG 참조 실패를 예로 든다. 원인을 모두 열거한 것은 아니라고 범위를 제한한다. 보존 로그에서 268회를 독립 집계했다. **PASS**
7. **오류 사례와 로그 연결** — README의 `s03-v8-weak-wrapper-ios-2026-10-04.png`, `s04-hit-test-android-2026-10-07.png`, `s04-hit-test-ios-2026-10-07.png` 예제가 전체 root 원본 log의 module-resolution 진단과 일치한다. **PASS**
8. **선택 빌드 성공 근거** — selected build log에는 `success Pages rendered`가 있고 README는 이를 전체 문서 빌드 성공으로 확대하지 않는다. **PASS**
9. **Rspress 버전 경계** — README는 선택 빌드를 `2.0.22`로 특정한다. 의존성 manifest/lockfile은 PR diff에 없다. **PASS**
10. **선택 빌드 제한** — README와 본문은 전체 `spec/` build와 repository-wide dead-link/dead-image 검사를 검증하지 않았다고 명시한다. **PASS**
11. **빌드 설정 분리** — 전체-root 실패 config와 selected config의 output 경로가 각각 고유 pass01 디렉터리다. `cleanDistPath` 적용 대상은 지정된 build output이다. **PASS**
12. **dead-link/dead-image 검사** — 선택 config가 두 검사를 끄는 사실을 README가 명시해 선택 build를 링크 적합성으로 과장하지 않는다. **PASS**
13. **선택 root 출처** — manifest의 selected source head/tree는 PR base의 `origin/main` revision/tree와 일치한다. 기록한 checkout commit은 달라도 checkout tree가 source tree와 같다고 남겼다. **PASS**
14. **source overlay 범위** — 준비 스크립트는 선택한 12개 main source를 exact bytes로 확인한 뒤 새 hardlink tree에 overlay하고 overlay hash를 기록한다. **PASS**
15. **hardlink base 안전성** — base root는 읽기 입력으로만 사용한다. overlay 교체는 selected root의 hardlink를 끊고 새 파일을 쓰므로 base inode의 내용은 변경하지 않는다. **PASS**
16. **route 집계** — build manifest는 생성 문서 16개와 전달 route 78개를 따로 기록하며 selected route 배열도 78개다. **PASS**
17. **상태 marker** — 선택 빌드 script는 roadmap/STATUS의 `72/213`, C11.4 계획의 PASS marker와 C11.4 checkbox 미완료를 확인한다. **PASS**
18. **경로 누락·중복 통제** — source/route 스크립트는 절대·상위 경로와 symlink를 거부하고 report에 route 개수와 hash를 남긴다. **PASS**
19. **Tailnet 사전 상태** — preflight는 Tailscale online/backend, loopback listener, roadmap HTTP 200과 교체 파일 경로를 기록한다. 이 값은 기록 당시 관측으로 한정한다. **PASS**
20. **교체 대상 open handle** — 15개 replacement target의 `lsof` 검사가 닫힘을 기록하고 sync 직전에도 다시 확인한다. **PASS**
21. **route 변경 수 산술** — 56 create + 15 replace + 7 same = 78이다. PR 본문, README, copy report의 합계가 일치한다. **PASS**
22. **백업 보존** — 15개 교체 대상의 기존 파일 4,289,067 bytes를 별도 backup root에 보관하고 경로·원본 hash를 manifest에 기록한다. **PASS**
23. **교체 원자성과 대상 경계** — sync script는 사전 상태가 바뀌지 않았는지 재확인하고 backup hash를 검증한 뒤 임시 파일과 atomic rename/link로 대상 route를 갱신한다. Tailnet preview route 삭제는 하지 않는다. **PASS**
24. **로컬 결과 hash** — 최종 report는 선택 manifest 대비 local target 78/78 및 backup 15/15를 검증했다고 기록한다. **PASS**
25. **원격 응답 hash** — verifier는 cache-busting query로 GET 응답 78개를 수집해 HTTP 200, byte count, SHA-256을 비교한다. report는 78/78 통과와 총 bytes 일치를 기록한다. **PASS**
26. **원본·정규화 log 구분** — README는 표시용 normalized log와 압축한 raw output을 별도 링크하며 두 gzip artifact 모두 읽을 수 있다. 전체-root log는 실패, 선택 build log는 성공 상태를 각각 보존한다. **PASS**
27. **screenshot 링크** — PR의 raw URL은 diff에 포함된 `tailnet-stretch-plan-agent-browser-pass01.png`를 가리킨다. 현재 head에서 blob을 확인했고 1280×633 PNG이며 URL은 이 파일이 처음 추가된 commit `6738c51d`에 고정돼 있다. 이 검토에서는 화면을 브라우저로 열거나 시각적 내용을 독립 관찰하지 않았다. **PASS — 링크·파일만 확인**
28. **Agent Browser 관찰 artifact** — 세 JSON artifact는 roadmap count/checkbox/link, parent-plan review state, focused-plan stage/expected rows 상태를 기록한다. README는 overview에서 직접 세부 plan 링크가 없는 점을 밝혀 탐색 경계를 과장하지 않는다. **PASS**
29. **프로세스 종료·브라우저 경계** — lifecycle JSON은 Agent Browser session/process 종료와 terminal-browser 미사용을 기록한다. 이 검토에서는 새 브라우저 세션을 열지 않았다. **PASS**
30. **공식 지원·남은 범위** — `spec/STATUS.md`와 public roadmap을 변경하지 않고 C11.4 미완료 및 `72/213`을 유지한다. API·앱·simulator/WPT/full CSS Grid 지원을 주장하지 않으며, 전체 docs build 한계와 Tailnet-only 선택 route 범위를 표시한다. **PASS**

## 판정

**PASS — PR diff 30/30, 차단 항목 0개.** route 집계·backup·local/remote SHA 근거와 선택/전체 빌드의 서로 다른 결과가 본문, README, manifest 및 로그에서 일치한다. PR statusCheckRollup은 비어 있어 CI 통과를 뜻하지 않는다. screenshot link와 PNG artifact의 일치만 확인했고 브라우저에서 실제 렌더링을 검증하지 않았다.
