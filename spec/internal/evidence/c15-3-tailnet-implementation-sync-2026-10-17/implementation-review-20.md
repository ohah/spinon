# Tailnet 구현 동기화 실패 경로 검토 20개

검토 범위는 이 증거 디렉터리, 선택적 정적 파일 복사, 기존 read-only preview listener와 실제 제공 응답이다. 각 항목은 독립적인 실패 조건과 남길 증거를 구분한다. 전체 docs strict build, Android/iOS 앱 실행, 삭제 작업을 성공으로 주장하지 않는다.

| # | 적대 조건 | 확인 또는 차단 기준 | 결과 |
|---:|---|---|---|
| 1 | 작업 시작 시 저장소 사용자 변경 | C15 격리 worktree에서만 수정하고 C11 worktree를 보존한다. | 확인 완료 |
| 2 | 잘못된 base commit의 문서 build | build 입력, 현재 main SHA, commit/tree를 preflight에 고정한다. | preflight 기록 |
| 3 | preview root가 바뀌었는데 이전 서버가 응답 | PID/command/log를 확인하고 기존 listener만 사용한다. | PID 1191 유지 |
| 4 | `/spinon` prefix 없는 우발 요청 | 로컬 HTTP 경로가 실제 Tailnet base와 일치하는지 확인한다. | prefix route 사용 |
| 5 | traversal 또는 symlink로 preview root 밖 쓰기 | 허용 파일 목록과 root containment를 검사한다. | 복사 목록 제한 |
| 6 | 파일명 중복으로 다른 evidence 덮어쓰기 | 정규화된 상대 경로의 유일성을 검사한다. | preflight 검사 |
| 7 | source 파일이 preflight 뒤 변경됨 | copy 직전 source hash를 다시 계산한다. | copy 검증 기록 |
| 8 | 대상 파일이 이미 같은 버전 | no-op 처리하고 backup을 만들지 않는다. | manifest 상태 기록 |
| 9 | 대상 파일이 다르지만 백업 실패 | 백업 hash를 재확인하기 전에는 overwrite를 중단한다. | backup 검증 기록 |
| 10 | 복사 도중 디스크 부족 또는 중단 | 복사 후 byte length와 SHA-256가 다르면 성공 처리하지 않는다. | copy 검증 기록 |
| 11 | manifest가 실제 copy 경로와 불일치 | source/target/backup 경로를 항목별로 대조한다. | manifest 대조 |
| 12 | 광범위한 static tree 실수로 전체 복사 | 참조된 asset만 모아 경로 수와 총 바이트를 제한한다. | 44 MB 전체 제외 |
| 13 | HTML이 오래된 hashed JS chunk를 참조 | 모든 직접 script/style ref가 존재하고 해시 일치해야 한다. | served 검증 기록 |
| 14 | route HTML은 200이나 내부 콘텐츠가 이전 버전 | C15.3 식별 문자열과 새로운 세부 링크를 검사한다. | Agent Browser 확인 |
| 15 | fixture 안내는 열리나 폰트 파일 누락 | 모든 세 글꼴과 라이선스 파일의 응답을 검사한다. | served 검증 기록 |
| 16 | 링크 이미지의 Content-Type/bytes 잘못됨 | image route 상태·크기·해시를 확인하고 browser에서 렌더링한다. | screenshot 확인 |
| 17 | HTTP 200을 stale proxy cache가 반환 | no-cache 응답을 새로 열고 제공 바이트 해시를 source와 비교한다. | served 검증 기록 |
| 18 | 링크가 404지만 manifest가 누락 | 대표 내부 링크와 fixture 경로를 직접 요청한다. | HTTP route 기록 |
| 19 | 기존 listener를 불필요하게 재시작/종료 | PID 1191 유지 전후를 비교한다. | 금지 경계 통과 |
| 20 | 브라우저 종료 후 Chrome 프로세스 잔류 | Agent Browser close 후 프로세스 목록을 재확인한다. | 종료 증거 기록 |
