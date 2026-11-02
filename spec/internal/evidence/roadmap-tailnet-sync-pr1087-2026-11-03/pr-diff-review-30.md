# PR #1087 Tailnet 동기화 변경 실패 관점 30개 검토

검토 기준: 현재 branch의 전체 변경 diff와 선택 root·publication manifest, preflight/copy/verify report, 실제 Tailnet Agent Browser 관찰. 각 항목은 별도 실패 경계를 다룬다.

1. **잘못된 기준 commit** — PR #1084 직후 source HEAD **c82639bd1**을 기준으로 사용하고 현재 source HEAD **52b58b6bd7fde6073e1c5992e0c93a815a53a684**까지 변경 diff hash를 기록했다.
2. **예전 선택 root 재사용** — PR #990이 아니라 가장 최근 PR #1084 sync build root를 기준으로 삼았다.
3. **dirty tracked source 혼입** — 작업 전 main tracked tree가 clean이었고 selected root source hash는 현재 HEAD 파일에서 계산했다.
4. **symlink 입력 복사** — 선택 root 재귀 열거가 symlink를 거부하며 기준 root에 symlink가 없었다.
5. **symlink publication 대상** — 서버 경로 검사와 preflight가 target 경로의 symlink를 거부한다.
6. **경로 탈출** — publication route는 절대 경로, parent traversal, 빈 component와 backslash를 거부한다.
7. **상태 원본 alias 누락** — **/STATUS.md** 및 **/spec/STATUS.md** 원본 경로를 함께 발행하고 HTTP/hash를 검증했다.
8. **standalone roadmap alias 누락** — **/roadmap.html**, **/public/roadmap.html**, **/spec/public/roadmap.html** 세 경로를 manifest와 verify에서 확인했다.
9. **C11 plan source 누락** — minmax 계획과 plan review route가 선택 publication에 있다.
10. **실행 README 누락** — minmax precomparison/runtime evidence README와 counter-sync README route를 필수 route로 확인했다.
11. **화면 이미지 깨짐** — minmax 계획 페이지는 Agent Browser에서 열렸고 이미지 1개 loaded, broken image 0개였다.
12. **정적 자산 누락** — 선택 RSPress site의 모든 생성 파일을 publication manifest에 넣고 170/170 hash를 확인했다.
13. **RSPress 버전 drift** — **2.0.22**를 기록하고 저장소 설정 버전을 유지했다.
14. **전체 빌드 성공 과장** — 선택 빌드만 실행했고 dead-link/dead-image check를 비활성화한 사실 및 전역 문서 검증 범위를 README에 제한했다.
15. **status counter를 정적 문자열로 오판** — 정적 HTML의 placeholder가 아니라 Tailnet 브라우저에서 동적 렌더링된 **215개 표시 · 공식 완료 78/215**를 관찰했다.
16. **중복 ID 행 덮어쓰기 회귀** — 215개 input 중 215개 ID가 고유한지 확인했다.
17. **알파벳 suffix ID 유실** — counter 집계 수정 PR #1087 source를 그대로 발행했고 이전 반례 C06.6a/C06.6b 검증 근거는 해당 PR evidence에 연결되어 있다.
18. **반복 C04.12 상태 전파** — 같은 PR evidence에서 반복 ID별 상태가 분리됨을 확인했으며, Tailnet 카운터는 원본 상태행 개수와 일치했다.
19. **공식 완료 수 잘못 게시** — 렌더링 결과 78/215, checked checkbox 78개였다.
20. **C11 slice를 상위 완료로 승격** — 로드맵 상단과 C11 status는 미완료로 남아 있고 이 PR은 checkbox를 수정하지 않았다.
21. **기존 server process 교체** — preflight와 verify가 preview PID 28040을 기록했고 재시작하지 않았다.
22. **Tailscale 단절 상태에서 복사** — preflight에서 backend Running 및 Mac Studio self host를 요구한다.
23. **변경 대상이 열린 채 덮어쓰기** — preflight와 copy 직전 모두 변경 대상 open handle 0개였다.
24. **preflight 이후 target 경합** — copy는 preflight 대상 existence/hash와 현재 파일을 비교하고 달라지면 중단한다.
25. **source가 preflight 뒤 변경** — copy는 route별 source SHA-256을 preflight 기록과 대조한다.
26. **부분 파일을 서비스** — 각 변경 파일을 같은 디렉터리 임시 파일에 쓴 뒤 atomic rename하고 즉시 SHA-256을 확인한다.
27. **backup 누락 또는 손상** — 교체 19개를 별도 backup root에 복사했고 verify에서 기존 SHA-256 19/19를 확인했다.
28. **무관한 route 삭제** — script는 삭제를 수행하지 않고 report도 noDeletes=true다.
29. **HTTP 응답은 맞지만 실제 브라우저는 오래된 화면** — Agent Browser가 HTTPS page에서 최신 PR #1086 카드와 새 78/215 집계를 직접 읽었다.
30. **좁은 화면 overflow 또는 browser 잔류** — 390px viewport에서 document/body width 390, card width 362였고 close --all 후 Agent Browser process scan은 비어 있었다.

최종 route/hash·backup 결과는 [served-verification.json](./served-verification.json), 렌더 화면 관찰은 [browser-verification.json](./browser-verification.json)에 보존했다.
