# C11.4 Tailnet 문서 미리보기 동기화 작업 검토 20개

**검토일:** 2026-10-19
**검토 대상:** PR #340 병합 뒤 선택된 Rspress 문서 출력물을 Tailnet 미리보기에 반영하고 검증한 작업 기록
**범위:** 미리보기 입력 집합, 백업, 파일 교체, HTTPS 응답, Agent Browser 표시 및 종료

## 독립 실패 관점

1. **원본 revision 혼동** — 배포 원본이 병합 전 branch가 아닌 PR #340 merge SHA인지 확인했다. `preflight.json`의 원본 SHA와 동기화 README의 SHA가 `d8881d9818e3629f6159637f40021ee0a95a4fdf`로 같다. 통과.
2. **완료 수 과대 계상** — raw input 보존을 C11.4 완료로 세지 않았는지 확인했다. README와 화면 근거 모두 72/212를 유지하고 C11.4 미완료를 적었다. 통과.
3. **선택 렌더 범위 과장** — 일부 Markdown 경로의 결과를 전체 사이트 렌더 성공이라고 부르는지 확인했다. 15개 Markdown 경로의 선택 렌더로 한정했고 전체 빌드·링크 검사를 주장하지 않는다. 통과.
4. **정적 파일 누락** — Markdown 외에 페이지가 참조하는 자산과 로드맵이 배포 집합에 포함됐는지 확인했다. Rspress 출력과 static roadmap 및 링크된 evidence assets를 포함하는 77개 manifest를 기록했다. 통과.
5. **잘못된 카운트 산식** — 기존·교체·신규·동일 파일 수가 총계와 맞는지 확인했다. 53+24=77, 19+34=53이며 삭제는 0이다. 통과.
6. **동일 파일 불필요 교체** — source와 target 해시가 같은 파일까지 덮지 않았는지 확인했다. manifest는 34개 동일 항목과 19개 교체를 분리한다. 통과.
7. **사전 백업 누락** — 교체 대상의 원본을 보존했는지 확인했다. 미리보기 외부 백업 20개를 기록했고 검증된 수와 실제 항목 수가 같으며 불일치가 없다. 통과.
8. **개인 경로 노출** — 공개 근거에 로컬 사용자 경로나 preview root가 들어가는지 검색했다. build log의 worktree 경로는 `[worktree]`로 바꾸고 manifest의 root는 가렸으며 backup root는 외부 위치만 표시한다. 통과.
9. **Tailnet hostname 노출** — 외부에 공개할 증거에 개인 Tailnet hostname이 남는지 확인했다. browser 증거의 host는 redacted이고 README는 상대 경로만 사용한다. 통과.
10. **활성 reader 파일 덮어쓰기** — `404.html`을 읽는 서버 descriptor가 있을 때 교체를 강행했는지 확인했다. read-only active reader로 별도 표기하고 변경하지 않았다. 통과.
11. **404 백업과 원본 보존** — 제외한 `404.html`이 사전 백업과 다른 상태가 됐는지 확인했다. served 근거의 404 hash가 backup hash와 같다. 통과.
12. **선택 제외 파일 삭제** — sync가 render 대상 밖의 preview 파일을 지웠는지 확인했다. 명시적인 삭제 수는 0이며 삭제 단계도 없었다. 통과.
13. **부분 쓰기 결과** — 복사 도중 미완성 파일이 남았는지 결과 확인과 비교했다. copy verification에 77개 검증, mismatch 0을 기록했다. 통과.
14. **새 파일 가용성** — 24개 신규 자산이 모두 서비스되는지 확인했다. 77개 전체 경로의 HTTPS 결과가 HTTP 200이다. 통과.
15. **오래된 파일 혼합** — 교체 대상이 여전히 이전 내용으로 제공되는지 확인했다. 서비스된 SHA-256 77개가 렌더 원본 SHA-256과 각각 같다. 통과.
16. **로드맵 표시값** — 문서 렌더에서 사용자에게 보이는 완료 수가 기록과 일치하는지 확인했다. Agent Browser viewport screenshot에 `공식 완료 72/212`가 보인다. 통과.
17. **상태 링크 도달** — 상태 페이지가 병합된 PR과 새 필드 설명을 가리키는지 확인했다. screenshot에서 PR #340 및 `grid_item_intrinsic_inputs`를 확인했다. 통과.
18. **계획 페이지 계약 경계** — plan 렌더에서 후보 raw input과 미완료 범위가 보이는지 확인했다. Agent Browser 화면에 C11.4 계획 제목, 필드명과 미완료 범위가 보인다. 통과.
19. **검토 문서 렌더 누락** — 새 PR diff review HTML 경로가 서비스되지 않는지 확인했다. Agent Browser 화면에 20개 검토 문서 제목과 검토 본문이 나타난다. 통과.
20. **브라우저 프로세스 잔류** — 사용 후 요청된 종료를 빼먹었는지 확인했다. 대상 세션 close 성공, 세션 정보 `active=false`·`pid=null`, idle 후 Agent Browser/Chrome 프로세스 0을 저장했다. 세션 목록에 남은 이름은 비활성 기록이다. 통과.

## 결과와 한계

이 검토는 preview 파일 전달과 문서 표시 경로만 다룬다. 모바일 app UI, iOS/Android simulator, physical device, CSS Grid sizing 동작 또는 전체 사이트 dead-link 통과를 입증하지 않는다. 추가로 발견된 동기화 결함은 없었다.
