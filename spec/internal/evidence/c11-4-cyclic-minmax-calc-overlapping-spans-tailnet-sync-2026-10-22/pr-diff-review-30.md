# C11.4 Tailnet preview PR diff 적대 검토 · 30개 실패 관점

**범위:** PR #500 병합 상태를 공식 C11.4 문서와 Tailnet 정적 미리보기에 반영하는 문서 변경. 공개 API, 앱 runtime, 제품 CSS 계약 또는 완료 상태는 바꾸지 않는다. 각 항목은 별개의 실패 경로를 확인한다.

| # | 실패 경로 | 판정 및 근거 |
|---:|---|---|
| 1 | Rebase 전 head SHA를 merge SHA로 기록 | 통과. PR #500의 merge SHA는 2f3f45c6aa27f427b36e906fa3a72fd8d864e402다. |
| 2 | 문서 branch가 #500 merge를 포함하지 않음 | 통과. branch 시작점은 #500 merge 이후 main이고 docs 커밋은 그 뒤에만 있다. |
| 3 | 미추적 workspace 파일이 선택 build에 유입 | 통과. commit 4e7da7c의 git archive와 기록된 tree를 사용했다. |
| 4 | 선택 source에 symlink 또는 비정규 entry 포함 | 통과. archive·기준 root·선택 root 검사에서 symlink는 0개다. |
| 5 | 서로 다른 build pass가 같은 root/output을 덮어씀 | 통과. pass26-initial root와 output을 분리했고 final pass도 고유 경로를 사용한다. |
| 6 | 문서 build가 RSPress pin을 벗어남 | 통과. build log가 Rspress 2.0.22를 표시한다. |
| 7 | preview 전용 dead-link 설정이 기본 docs config에 유입 | 통과. dead-link/image 옵션은 evidence의 선택 config에만 있다. |
| 8 | roadmap의 planned/final track 값이 섞임 | 통과. [34,39,39], [42,47,47], [32,32,32]가 source와 Agent Browser DOM에서 같다. |
| 9 | 고정 fixture를 C11.4 전체 지원으로 과장 | 통과. roadmap은 C11.4 전체 미검증과 77/213 유지를 명시한다. |
| 10 | STATUS의 merge 시각 또는 SHA가 다름 | 통과. 2026-10-22 05:11:19 UTC와 PR #500 merge SHA가 표시된다. |
| 11 | STATUS parent checkbox나 완료 집계가 올라감 | 통과. C11.4 checkbox는 unchecked, 공식 집계는 77/213이다. |
| 12 | 계획이 simulator scene 수를 과장 | 통과. plan과 browser 화면은 Android/iOS 각각 6/6을 기록한다. |
| 13 | 내부 인덱스 상대 링크가 없는 파일을 가리킴 | 통과. plan, 실행 근거, 20개 구현 review, 20개 implementation PR review, Tailnet README가 build route로 생성됐다. |
| 14 | roadmap PR 링크가 잘못된 PR을 가리킴 | 통과. Agent Browser DOM의 PR #500 href가 github.com/ohah/spinon/pull/500이다. |
| 15 | RSPress route가 Tailnet 경로와 다름 | 통과. roadmap, STATUS, plan, runtime evidence URL을 Agent Browser에서 직접 열었다. |
| 16 | Tailnet에 표시된 Android/iOS image가 깨짐 | 통과. 두 image 모두 complete=true, 각각 1080×2400 및 1206×2622다. |
| 17 | PR 본문용 simulator PNG가 다른 artifact를 가리킴 | 통과. merge SHA를 포함한 raw GitHub URL에서 같은 두 PNG의 원본 크기를 확인했다. |
| 18 | 한글·값·SHA가 build/render 과정에서 손상 | 통과. roadmap, STATUS, plan DOM과 화면에서 텍스트 및 숫자를 대조했다. |
| 19 | 자동 진행 수가 77/213에서 달라짐 | 통과. roadmap·STATUS·plan에 77/213이 표시된다. |
| 20 | 지원 범위가 CSS border/compositor 이후 검증을 주장 | 통과. CSS border stroke, Android 실기기·hardware GPU, iOS compositor 이후 pixels, 일반 CSS admission, WPT와 전체 C11.4를 미검증으로 적었다. |
| 21 | GitHub Pages 또는 gh-pages에 배포 | 통과. Pages workflow를 실행하지 않았고 gh-pages를 변경하지 않았다. |
| 22 | 기존 Tailnet 파일이 backup 전에 교체 | 통과. 기존 파일 254개를 바꾸기 전에 40,784,640 bytes를 저장하고 254/254 hash를 확인했다. |
| 23 | preflight 이후 source build가 달라짐 | 통과. copy 단계가 각 파일 source byte 수·SHA-256을 preflight와 재비교했다. |
| 24 | preflight 뒤 바뀐 target을 그대로 덮어씀 | 통과. copy 단계가 target 존재·이전 byte 수·SHA-256을 재비교했다. |
| 25 | symlink를 통해 target/backup 경로 밖에 쓰기 | 통과. target/backup 각 경로 구성요소를 lstat 검사하고 symlink에서 중단한다. |
| 26 | 다른 process가 연 파일을 교체하거나 preview listener가 바뀜 | 통과. rename 전 target lsof 관찰은 0건, listener PID 1191은 유지됐다. |
| 27 | partial write 또는 기존 temp 파일 덮어쓰기 | 통과. exclusive temp create, fsync, 같은 directory의 atomic rename을 사용한다. |
| 28 | 동기화가 기존 사용자 파일을 삭제 | 통과. manifest no_deletes=true이며 create/replace/same만 수행했다. |
| 29 | local copy만 성공하고 HTTP 응답 bytes가 다름 | 통과. 4,331/4,331 HTTP 200, byte 수·SHA-256 일치, 불일치 0이다. |
| 30 | browser cleanup이 사용자 Chrome이나 Tailnet server를 종료 | 최종 Tailnet 화면 확인 뒤 전용 Agent Browser session만 닫고 PID 2771 계열 종료, 사용자 Chrome PID 20311과 listener PID 1191 존속을 마지막에 확인한다. 결과는 PR 본문에 기록한다. |

검토 화면과 DOM은 이 디렉터리의 Agent Browser evidence에 저장했다. build/copy/backup/local/served hash 원본 manifest는 gzip 파일로 보존한다. PR 병합 전 #30의 실제 프로세스 상태도 확인한다.
