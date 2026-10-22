# PR #497 Tailnet 미리보기 동기화 실패 관점 검토 · 20개

pass25 대상과 파일 반영 절차를 각각 다른 실패 경로로 확인했다.

| # | 검토 관점 | 결과 |
|---:|---|---|
| 1 | 이전 미리보기 tree를 덮어쓰는가 | 통과. 새 selected root와 output 이름을 pass25로 분리했다. |
| 2 | 이전 백업을 재사용하거나 손상시키는가 | 통과. pass25와 pass25-final 백업 root를 나누고, 기존 백업이 있을 때 원본 길이와 SHA-256이 다르면 중단한다. |
| 3 | 선택 문서 집합이 구버전 상태 대장인가 | 통과. 현재 main의 STATUS, 내부 README, Grid 계약, 계획, 로드맵과 관련 evidence를 root에 복사했다. |
| 4 | root overlay가 무관한 문서 집합을 삭제하는가 | 통과. pass24 selected root를 복사하고 지정 파일만 갱신했다. source와 preview에서 삭제하지 않았다. |
| 5 | selected source 안 symlink가 path 경계를 우회하는가 | 통과. sync script의 recursive file listing이 symlink와 일반 파일 외 항목을 거부한다. |
| 6 | preview target 경로가 symlink 또는 비정상 directory인가 | 통과. 각 대상 경로의 root와 부모를 lstat로 검사하고 symlink를 거부한다. |
| 7 | preflight 이후 source가 바뀌어 다른 bytes가 복사되는가 | 통과. 복사 직전에 preflight의 byte 길이와 SHA-256을 다시 대조한다. |
| 8 | preflight 이후 target이 바뀌어 예기치 않은 파일을 덮는가 | 통과. 복사 직전에 이전 target 존재 여부·크기·SHA-256을 다시 비교한다. |
| 9 | 백업 파일이 기존 데이터를 조용히 덮는가 | 통과. 새 백업은 exclusive create로 만들고, 재사용하는 백업은 preflight 원본 hash와 일치해야 한다. |
| 10 | 원본 backup 완료 전 preview target을 교체하는가 | 통과. backup bytes와 SHA-256을 확인한 다음에만 같은 디렉터리 임시 파일을 원자 rename한다. |
| 11 | 부분 쓰기 후 잘못된 target이 보이는가 | 통과. 임시 파일을 exclusive create하고 fsync한 뒤 rename한다. |
| 12 | 파일을 열어 둔 무관한 앱과 충돌하는가 | 통과. 교체 직전 lsof를 확인한다. open handle이 있으면 지정된 idle Tailnet listener 외 PID는 거부한다. |
| 13 | PID 1191이 다른 프로세스로 재사용됐는데 허용되는가 | 통과. 명령행에 tailnet-preview-server.mjs가 있는지 확인한다. |
| 14 | browser client가 연결된 상태에서 listener inode를 바꾸는가 | 통과. listener 외 active network handle이 있으면 중단한다. 실제 copy 시 listener의 대기 socket만 보였다. |
| 15 | 초기 sync backup과 최종 sync backup이 서로 덮이는가 | 통과. 첫 반영은 pass25, 뒤이은 selected build는 pass25-final 별도 root를 사용했다. |
| 16 | 최종 대상에서 누락·크기 변경이 남는가 | 통과. 최종 1,764개 target과 103개 backup을 모두 local hash로 검증했다. |
| 17 | 서버가 local file 대신 stale 또는 잘못된 bytes를 내보내는가 | 통과. 최종 1,764개 HTTP 응답 모두 status 200, 길이, SHA-256이 local file과 일치했다. |
| 18 | 전체 docs 성공을 선택 preview 성공으로 과장하는가 | 통과. pinned RSPress 2.0.22 선택 build만 기록했고 전체 문서 build나 GitHub Pages 배포로 표현하지 않았다. |
| 19 | RSPress가 sync script/config를 콘텐츠 module로 읽는가 | 통과. 첫 실패를 보존하고, 실행 도구를 selected content root에서 제외한 뒤 build 성공을 확인했다. |
| 20 | UI가 오래된 로드맵을 보여주거나 browser process가 남는가 | 통과. Agent Browser에서 77/213, PR #497, plan, runtime screenshots와 미완료 STATUS를 확인했다. 전용 session을 닫아 process 0개를 확인하고 일반 Chrome과 preview server를 보존했다. |

근거는 같은 디렉터리의 preflight/copy/verify/served JSON, selected build 로그와 Agent Browser screenshots다. 실제 sync 단계별 대상 수와 backup root는 sync-summary-pass25.json에 고정했다.
