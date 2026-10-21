# Tailnet pass41 evidence PR 적대적 diff 검토 30개

검토 대상은 이 디렉터리의 pass41 README, 실행 스크립트, 산출물 요약, 개별 파일 검증 기록, Agent Browser 캡처와 PR #481 회고 검토다. 아래 항목은 서로 다른 실패 경로와 경계 조건을 각각 대조했다. 이 검토 문서 자체의 항목 수·결론은 아래 표 작성 후 별도로 확인했다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 선택한 소스가 병합되지 않은 후보 commit일 수 있음 | source tree `25f28e3…`가 `origin/main`의 `0791d310…` tree와 같다고 build summary와 준비 기록이 남긴다. |
| 2 | rebase 전 HEAD 차이를 같은 소스로 오인 | HEAD SHA는 다르지만 Git tree SHA가 일치한다고 README가 구분한다. |
| 3 | pass40 원본 root나 site를 덮어씀 | 준비 스크립트는 pass40 selected root를 새 pass41 경로로 복사하고 README는 이전 root/site 보존을 기록한다. |
| 4 | 선택 범위에서 무관한 새 소스가 빠짐 | status, public roadmap, 계획·사전 비교 fixture와 네 C11.4 evidence 디렉터리의 복사 목록이 JSON에 기록돼 있다. |
| 5 | 내부 evidence 링크의 세 문서 root 복제 중 하나가 누락 | 준비 기록은 `internal/evidence`, `spec/internal/evidence`, `public/internal/evidence` 세 목적지를 각각 나열한다. |
| 6 | 빌드 입력에 symlink나 특수 파일이 섞임 | sync 스크립트는 symlink와 일반 파일·디렉터리가 아닌 항목을 거부하고 build summary도 `noSymlinks: true`로 남긴다. |
| 7 | 문서 도구 버전이 의도치 않게 바뀜 | 빌드 요약의 RSPress는 `2.0.22`이며 저장소 규칙의 고정 버전과 같다. |
| 8 | 핵심 route 생성 실패가 총 파일 수 뒤에 숨음 | summary에 roadmap, status, 계획, 사전 비교, 실행 근거, Android/iOS PNG 등 필수 route 15개가 개별 기록돼 있다. |
| 9 | 일부 출력 누락에도 build를 완전 성공으로 표기 | build 요약은 3,027 files, 128,044,998 bytes와 tree SHA-256을 함께 남기며 필수 route 존재를 별도로 기록한다. |
| 10 | dead-link 검사를 끈 구성을 완전한 링크 검사로 오해 | RSPress 선택 설정이 dead-link/image 검사를 끈 사실을 확인했다. README는 이를 보편적인 링크 무결성 증명으로 확대하지 않고 필수 route와 개별 브라우저 결과만 보고한다. |
| 11 | 동기화 전 대상 파일 분류에서 파일이 누락 | preflight와 copy 기록은 같은 3,027 경로를 대상으로 create 298, replace 165, same 2,564를 남긴다. |
| 12 | 이전 preview 파일을 조용히 삭제 | preflight·copy·local verification 모두 `no_deletes: true`; sync 스크립트에는 대상 파일 삭제 단계가 없다. |
| 13 | 교체 전 원본이 백업 없이 사라짐 | replace 165개와 기존 데이터 31,832,668 bytes의 SHA·크기를 별도 backup root에 보존한다. |
| 14 | 중단 후 재실행 시 기존 backup을 다른 원본으로 덮어씀 | backup 생성은 `wx` 배타 생성이고, 기존 backup은 preflight의 크기·SHA와 일치할 때만 재사용한다. |
| 15 | 대상 경로의 symlink가 preview root 밖으로 쓰기를 유도 | root와 각 경로 구성요소를 `lstat`으로 검사해 symlink 및 잘못된 디렉터리/파일 경계를 거부한다. |
| 16 | preflight 뒤 source 또는 대상이 바뀐 채 계속 복사 | copy 직전에 source와 기존 target을 preflight 크기·SHA와 재비교하고 변경 시 중단한다. |
| 17 | 복사 중 부분 파일이 공개되거나 임시 파일을 덮어씀 | 같은 디렉터리에 `wx` 임시 파일을 만들고 `fsync` 후 rename하며 기존 임시 경로가 있으면 중단한다. |
| 18 | preview 서버를 전역 종료하거나 사용 중인 파일을 교체 | 정확한 8768 listener와 실행 파일을 확인하며, 대상 파일의 open handle에 비-preview PID 또는 활성 client socket이 있으면 rename을 거부한다. |
| 19 | 동기화 과정에서 기존 Tailnet 서버나 tmux 세션을 정리 | 실행 기록과 종료 기록은 서버 PID 1191 및 `127.0.0.1:8768`을 보존한다고 남긴다. tmux 세션 조작은 기록돼 있지 않다. |
| 20 | write 성공만으로 대상 내용이 맞다고 간주 | 각 copy 결과는 대상 크기와 SHA-256을 확인하고 `all_verified: true`를 기록한다. |
| 21 | 완료 후 local tree가 원본과 달라짐 | 별도 local verification이 source 3,027개 대상 전체와 백업 165개를 다시 대조해 모두 일치했다고 기록한다. |
| 22 | Tailnet 응답은 200이지만 본문이 오래된 파일임 | remote 결과는 3,027/3,027 HTTP 200뿐 아니라 예상 크기·SHA 일치도 3,027/3,027, failure 0으로 기록한다. |
| 23 | redirect 뒤 다른 URL의 성공을 원 URL 성공으로 숨김 | remote 결과는 요청별 `final_url`, status, 응답 크기, 응답 SHA와 예상 값을 저장한다. |
| 24 | 대량 동시 요청이나 영구 대기로 검증 안정성을 해침 | 검증 스크립트는 concurrency 8과 요청별 30초 timeout을 사용하고 실패 경로를 별도 배열로 기록한다. |
| 25 | 공개 roadmap의 문구가 PR #480 결과와 어긋남 | Agent Browser DOM 결과에 PR #480 merge SHA, 네 scene 요약, 미완료 범위와 `72/213`이 함께 있다. |
| 26 | 공식 status가 하위 slice를 전체 C11.4 완료로 승격 | status DOM은 C11.4 미완료, merge SHA, `72/213`을 확인하고 남은 범위를 명시한다. |
| 27 | 집중 계획의 장면별 기대값이나 행이 누락 | plan DOM은 네 scene과 root·Columns·Rows·item 값이 포함된 표를 기록한다. |
| 28 | 실행 evidence가 Android/iOS 화면 또는 제한을 숨김 | evidence DOM은 네 scene 행과 screenshot 링크 8개를 기록하고, 이미지 DOM은 Android `1080×2400`, iOS `1206×2622` 완료를 확인한다. |
| 29 | PR #480 merge 페이지의 본문 이미지가 깨졌는데 숨김 | Agent Browser DOM은 PR을 `Merged`로 확인하고 Android/iOS raw image 모두 `complete=true`와 원본 크기를 기록한다. |
| 30 | 브라우저 잔류, 잘못된 종료 대상 또는 절차 누락 은폐 | `agent-browser close --all` 뒤 browser scan은 비었고 preview 서버는 유지됐다. README는 초기 준비 assertion 실패가 preview 쓰기 전에 중단된 점과 PR #481 사전 검토 기록 누락 및 회고 검토의 비소급성을 함께 공개한다. |

## 판정과 한계

pass41의 선택 root, 빌드, 파일별 복사·백업, 로컬·Tailnet hash 검증과 Agent Browser 화면 기록은 서로 연결된다. 검증 범위는 사설 Tailnet 미리보기 파일과 화면 렌더링에 한정되며 production 배포나 제품 동작 완료 근거가 아니다. PR #481의 30관점 검토 기록은 병합 전에 남지 않았고, 별도 회고 문서는 그 누락을 사후 검토로 소급 충족했다고 주장하지 않는다.
