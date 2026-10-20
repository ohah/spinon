# Tailnet 동기화 PR diff 실패 관점 검토 30개

대상: pass25 초기 갱신, pass26 선택 문서 입력·RSPress 빌드·Tailnet 파일 동기화, Agent Browser 검증과 결과 기록.

1. **이전 문서 기준본을 최신으로 오인** — pass24가 PR #397 기준임을 확인하고 현재 `main`까지의 315개 문서 변경 경로를 pass26에 반영했다.
2. **초기 pass25를 최종 반영으로 표시** — pass25 기록은 보존하되 누락 발견과 pass26 교정 사실을 README와 요약에 구분해 적었다.
3. **원본 revision이 바뀐 상태에서 선택 사이트 생성** — selection manifest에 기준 revision과 현재 SHA `3382436096fabd0ce0684d36f287cd90d4581ed9`를 기록했다.
4. **상태 대장 복제 경로 중 한쪽만 갱신** — `STATUS.md`와 `spec/STATUS.md` 두 URL을 선택 입력과 HTTP hash 검사에 포함했다.
5. **공개 로드맵이 Markdown 문서 복사로 누락** — `spec/public/roadmap.html`을 RSPress의 정적 입력 `public/roadmap.html`에 매핑하고 Agent Browser에서 열었다.
6. **실기기 계획 route 한쪽만 갱신** — `plan/` 및 `spec/plan/` 출력 URL 모두를 제공 검증했다.
7. **증거 README는 있는데 이미지가 없음** — C11.4, C13.3 Android/iOS README와 각 이미지의 HTTP 응답 hash를 확인했다.
8. **C11.4 보충 결과를 공식 완료로 승격** — 상태 대장과 로드맵은 C11.4 미완료 및 기존 완료 수를 유지한다.
9. **새 API나 플랫폼 보장으로 확대** — PR #405 기록을 단일 SM-S731N 보충 자료로 한정했다.
10. **현재 상태에서 링크하는 C13.3 증거가 계속 404** — PR #397 이후 새 C13.3 계획과 Android/iOS 근거를 pass26 선택 입력에 추가했다.
11. **Markdown 상대 이미지가 RSPress package import가 됨** — 원본 파일과 인접 이미지 존재를 확인하고 선택 입력의 네 README 사본에 `./`만 추가했다.
12. **정규화가 repository source를 바꿈** — path repair 전후 hash를 selection-normalization manifest에 기록했고 Git diff에는 source 문서 수정이 없다.
13. **실행 가능한 Rspress config가 문서 route로 컴파일 실패** — 공식 상태·roadmap에서 연결하지 않는 두 운영용 sync evidence 폴더를 선택 입력에서 제외했다.
14. **선택 입력에서 제외한 폴더를 Tailnet에서 지움** — copy-only script는 Tailnet 대상 삭제를 실행하지 않고 README에 구분해 기록했다.
15. **실패한 첫 빌드를 성공으로 기록** — `selected-build-pass26.log`에는 첫 실패, `selected-build-pass26-final.log`에는 보정 뒤 성공을 각각 보존했다.
16. **선택 빌드를 전체 문서 빌드로 확대** — README는 RSPress 선택 빌드만 성공으로 기록하고 전체 저장소 빌드는 실행했다고 말하지 않는다.
17. **dead-link 검사를 끈 사실을 감춤** — 선택 config가 기존 dead-link·dead-image 검사를 비활성화함을 범위에 기록했다.
18. **RSPress 버전을 문서 작업과 함께 변경** — 프로젝트 고정 버전 2.0.22를 사용했고 package manifest나 lockfile을 변경하지 않았다.
19. **선택 입력의 파일 수와 manifest가 불일치** — build source는 1,810개 파일·81,661,358바이트이며 pass26 preflight와 final verify가 같은 집합을 확인한다.
20. **source path 매핑이 조용히 빠짐** — unmapped changed documentation path가 있으면 준비 스크립트가 중단하도록 했다.
21. **심볼릭 링크가 선택 입력 경계를 벗어남** — selection generator와 sync helper가 source/target 경로의 symlink를 거부한다.
22. **동기화 전 대상이 바뀌었는데 덮어씀** — preflight 이후 target hash/크기가 달라지면 copy helper가 중단한다.
23. **기존 파일을 백업하기 전에 교체** — 교체 전 바이트를 pass26 전용 backup root에 exclusive create하고 SHA-256을 재검증한다.
24. **동일 경로 백업을 이전 pass 파일로 오인** — pass25와 pass26은 서로 다른 backup root를 사용한다.
25. **원자 교체 중 서버가 열어 둔 파일을 덮어씀** — 이전 pass에서 열린 단일 파일은 PID 1191의 idle listener만 가진 것을 확인하고 같은 디렉터리 원자 rename을 사용했다. pass26에는 열린 파일 관찰이 없었다.
26. **복사 일부 실패를 전체 성공으로 보고** — copy와 final verify가 각 1,810개 대상 및 106개 backup 검증 결과를 JSON으로 보존한다.
27. **오래된 preview 파일을 삭제** — pass26 copy summary의 `no_deletes`는 `true`; 새/교체 파일 외 대상 경로를 건드리지 않았다.
28. **서비스가 stale 파일을 반환** — 14개 HTML·PNG URL을 local build와 비교해 모두 HTTP 200 및 동일 byte count/SHA-256을 확인했다.
29. **HTML만 열리고 화면 이미지가 깨짐** — Agent Browser에서 C11.4 1080×2340, C13.3 Android 1080×2340 및 iOS 1536×2048 이미지의 complete/natural dimensions를 확인했다.
30. **검증 후 브라우저 또는 preview server를 종료** — Agent Browser를 닫고 최종 프로세스 목록에 browser/Chrome이 없음을 확인했으며 Tailnet 서버 PID 1191은 계속 실행 중이다.

CI 상태를 기록하거나 GitHub Pages workflow를 실행하지 않았다. 이 PR은 Tailnet 내부 미리보기 sync evidence와 재현 도구만 추가한다.
