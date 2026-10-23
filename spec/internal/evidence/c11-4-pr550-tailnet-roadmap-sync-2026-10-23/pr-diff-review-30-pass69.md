# PR diff 적대 검토 30개 · pass69

## 동기화 범위와 source

1. **main revision pin** — 선택 루트 HEAD와 `origin/main`이 PR #550 병합 커밋 `82f7514...`인지 확인한다. 기록한 source revision은 둘 다 일치한다.
2. **변경 문서 반영** — status, conformance, plan, 내부 index와 0059, 공개 roadmap 및 관련 C11.4 evidence가 main의 파일인지 점검한다. 선택 루트 manifest가 각 source를 연결한다.
3. **선택 루트 출처** — 오래된 pass65 root의 링크를 main 결과로 덮어쓰지 않고, 추적된 갱신 파일을 별칭 경로에도 overlay했는지 검토한다. 준비 manifest는 1,845개 copy와 삭제 0을 기록한다.
4. **심볼릭 링크 경계** — source와 Tailnet tree의 symlink를 따라가지 않는지 본다. 준비·동기화 스크립트는 symlink를 만나면 중단한다.
5. **기본 도구 고정** — 임시 `bunx` RSPress가 아니라 저장소의 고정 바이너리 2.0.22를 썼는지 확인한다. 빌드 요약의 버전과 명령이 일치한다.
6. **MDX 상대 이미지** — 문서 route에서 상대 이미지가 패키지 import로 해석되는 실패를 검토한다. 필요한 `./` 경로를 반영했고 선택 빌드는 종료 코드 0이다.
7. **빌드 범위 과장** — `checkDeadLinks`와 `checkDeadImages` 비활성화, 전체 dirty-checkout 빌드 한계를 확인한다. README는 선택 빌드를 전체 dead-link 통과로 서술하지 않는다.
8. **raw HTML route 누락** — RSPress build가 public raw `roadmap.html`을 출력하지 않는 경계를 확인한다. 별도 검사에서 main source와 제공 파일이 이미 같은 hash여서 Tailnet write가 필요 없었다.
9. **미리보기 target** — 잘못된 서비스 디렉터리로 복사하지 않았는지 확인한다. 스크립트 target은 서버가 제공하는 `SpinonPreview` 절대 경로로 고정됐다.
10. **정적 route path** — roadmap·STATUS·conformance·계획·근거 URL이 `/spinon/` 아래 같은 상대 경로를 쓰는지 본다. 9개 실제 HTTPS route가 source와 byte/hash 일치한다.

## 파일 교체와 보존

11. **경로 탈출** — manifest 경로가 절대 경로나 `..`로 target 밖을 덮을 수 있는지 검토한다. `safeRelative`가 절대·상위 경로를 거부한다.
12. **파일 종류** — 디렉터리·symlink·특수 파일을 일반 파일처럼 교체하지 않는지 확인한다. 대상은 regular file만 허용한다.
13. **사전 변경 경쟁** — preflight 뒤 사용자가 수정한 target을 덮어쓸 수 있는지 본다. 기존 target hash와 byte 수가 preflight 값에서 달라지면 copy가 실패한다.
14. **유일한 백업 경로** — 이전 pass의 백업을 덮어쓰지 않는지 확인한다. 전용 pass69 backup root가 이미 있으면 진행을 거부한다.
15. **백업 내용** — 교체 파일 568개의 기존 hash를 보존하는지 검토한다. 각 hard link/copy의 hash를 원본 target hash와 대조한다.
16. **부분 파일 노출** — server가 복사 중 절반 기록된 페이지를 제공할 가능성을 본다. 임시 형제 파일에 복사한 뒤 rename한다.
17. **삭제 동작** — source에 없는 수만 개의 기존 route·asset을 잘못 제거할 수 있는지 확인한다. 스크립트에는 delete 경로가 없고 target 파일 수가 증가했다.
18. **새 파일 중복 처리** — preflight에서 없던 파일이 중간에 생긴 경우 임의로 덮어쓰지 않는지 검토한다. 예상 source hash와 같으면 이미 적용된 경우로 건너뛰며 다르면 중단한다.
19. **백업 볼륨 가정** — 다른 장치에서 hard link가 실패하는 경우를 본다. 같은 장치면 hard link, 아니면 exclusive copy로 백업한다.
20. **재실행 실패 안전성** — 동기화 일부 뒤 재실행 때 이미 바뀐 target이나 만들어진 backup을 혼동하는지 확인한다. 동일 source는 건너뛰고 기존 backup root 재사용은 거부한다.

## HTTP와 화면

21. **변경 파일 hash** — 로컬 target의 변경 1,171개가 빌드 결과와 일치하는지 확인한다. 전 항목의 크기와 SHA-256이 통과했다.
22. **백업 hash** — 교체 이전 버전 hash가 보존됐는지 확인한다. verify에서 각 백업을 preflight manifest와 비교한다.
23. **웹 서버 응답** — 디스크 파일만 확인하고 server stale cache를 놓치는 실패를 검토한다. 9개 문서 route와 18개 참조 정적 asset을 HTTPS로 가져와 200 및 source hash를 확인한다.
24. **roadmap raw hash** — 72/213 본문을 가진 raw HTML과 Tailnet route가 일치하는지 확인한다. 양쪽이 267,555 bytes와 `d01d12...` hash로 동일하다.
25. **HTML 자산 참조** — CSS·JS·이미지 참조 파일이 누락될 수 있는지 본다. 실제 갱신 route의 18개 참조 정적 파일을 모두 검증했다.
26. **roadmap 표시 집계** — 과거의 잘못된 77/213 문구가 DOM에 남는지 확인한다. Agent Browser는 표시 checkbox 213개, checked 72개, 77/213 부재를 관찰했다.
27. **STATUS raw 집계 분리** — 226/87 raw checkbox를 로드맵 공식 수치로 오인할 위험을 본다. JSON과 README가 두 모집단을 나눠 설명한다.
28. **C11.4 과완료 표기** — 7개 scene 구현을 전체 항목 완료처럼 표시하는지 검토한다. checkbox는 미완료이며 README·화면에 전체 검증 한계가 남아 있다.
29. **화면 증거 진위** — 터미널 출력이나 오래된 캡처를 현재 화면처럼 쓸 위험을 본다. screenshot은 pass69 Agent Browser 세션에서 동기화 후 캡처했고 크기·hash를 기록했다.
30. **프로세스 정리** — 브라우저 종료가 공유 Tailnet server나 무관 세션까지 끌 수 있는지 확인한다. 전용 Agent Browser session만 종료했고 세션/브라우저 프로세스는 0, PID 1191은 유지했다.
