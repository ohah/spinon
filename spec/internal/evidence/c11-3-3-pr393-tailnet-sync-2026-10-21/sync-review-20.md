# C11.3.3 PR #393 Tailnet 동기화 실패 관점 검토 20개

**검토 범위:** 선택 문서 빌드, Tailnet 파일 교체, 백업, 전체 served hash 검사, 브라우저 확인과 세션 종료. 각 항목은 서로 다른 실패 경로를 다룬다.

| # | 실패 조건 | 근거와 판정 |
| ---: | --- | --- |
| 1 | 다른 source 디렉터리를 선택해 기존 미리보기를 잘못 덮어쓴다. | sync helper가 저장소 내 pass1 빌드 경로만 허용한다. 통과. |
| 2 | 사용자가 지정하지 않은 미리보기 디렉터리를 변경한다. | target은 `~/Library/Application Support/SpinonPreview`로 고정한다. 통과. |
| 3 | 기존 백업을 다른 작업 파일로 덮는다. | pass 전용 backup root를 사용하고 기존 백업의 SHA가 다르면 중단한다. 통과. |
| 4 | 기존 Tailnet 파일을 새 파일로 착각해 무단 덮어쓴다. | preflight가 존재 여부·byte 수·SHA-256을 기록하고 copy 직전에 다시 대조한다. 통과. |
| 5 | preflight 후 선택 산출물이 변한다. | source 전체 경로 집합과 각 항목의 byte 수·SHA를 재검사한다. 통과. |
| 6 | symlink로 선택 root 또는 대상 경로 밖을 수정한다. | source·target·backup 경로의 symlink와 빌드 내 symlink를 거부한다. 통과. |
| 7 | 새 빌드에 없는 기존 페이지까지 삭제한다. | helper에는 삭제 경로가 없으며 preflight와 결과 모두 `no_deletes: true`다. 통과. |
| 8 | 서버가 읽는 HTML을 truncate한 중간 상태로 제공한다. | sibling temporary file을 완성한 뒤 같은 디렉터리에서 atomic rename한다. 통과. |
| 9 | 교체 전 파일 내용이 보존되지 않는다. | 교체 98개 모두 전용 백업에서 byte 수·SHA-256이 검증됐다. 통과. |
| 10 | 이미 같은 파일까지 불필요하게 다시 쓴다. | 사전 SHA가 같은 1,419개는 same으로 분류하고 쓰기를 생략한다. 통과. |
| 11 | 선택 원본 목록에서 Android 실기기 PixelCopy 파일이 빠진다. | 원본 manifest에 900×675 PNG가 포함되고 최종 served 목록에 동일 해시가 있다. 통과. |
| 12 | 로컬 사용자 경로·개인 식별 정보가 정적 문서에 섞인다. | 산출물 정리 결과와 선택 evidence를 확인했다. 화면·저장 근거의 기기 serial은 가렸다. 통과. |
| 13 | 빌드 결과에 잘못된 경로 또는 끊긴 문서 링크가 생긴다. | RSPress 산출물의 roadmap, STATUS, 내부 계약, 후속 evidence route가 생성됐다. 통과. |
| 14 | Tailnet server가 다른 디렉터리의 오래된 내용을 계속 제공한다. | 지정 Tailnet origin의 실제 GET body를 로컬 preview 파일과 대조했다. 통과. |
| 15 | 응답 본문 길이만 같고 파일 내용은 다른데 통과한다. | 전체 1,832개에 HTTP 200, byte 수, SHA-256을 모두 요구한다. 불일치 0. |
| 16 | 표본 URL만 검사해 나머지 파일 stale 상태를 놓친다. | preflight 전체 1,832 경로를 모두 검사하고 1,832개가 일치했다. 통과. |
| 17 | 병합 PR 링크 또는 official completion 상태가 오래된 상태로 보인다. | Agent Browser에서 PR #393 링크와 C11.3.3 후보·미완료 문구를 확인했다. 통과. |
| 18 | 브라우저가 열렸지만 PixelCopy image request가 실패한다. | Agent Browser DOM에서 물리기기 PNG `complete=true`, natural size 900×675을 확인했다. 통과. |
| 19 | 브라우저 검증 결과가 없는데 성공으로 기록한다. | roadmap·STATUS·evidence 세 route를 실제 Tailnet origin에서 열고 화면 캡처를 저장했다. 통과. |
| 20 | 검증을 위해 공유 Tailnet·tmux를 종료하거나 새 browser process를 남긴다. | 이번 Agent Browser 세션만 닫았고 활성 세션 0개·관련 process 부재를 확인했다. Tailnet 서버와 기존 tmux는 유지했다. 통과. |

## 결과

20개 항목을 근거와 대조했다. 1,832개 파일 중 신규 315개, 교체 98개, 동일 1,419개였으며 98개 백업과 전체 복사 결과가 SHA-256으로 일치했다. Tailnet served response도 1,832/1,832개가 HTTP 200·byte·SHA 모두 일치했다. 삭제는 없었다.
