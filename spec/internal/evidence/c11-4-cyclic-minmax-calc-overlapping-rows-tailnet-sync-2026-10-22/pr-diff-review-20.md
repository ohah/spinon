# PR #504 Tailnet 동기화 근거 diff 실패 관점 20개

PR #504가 보관하는 merge metadata, 선택 문서 build, 파일 교체·백업, served bytes, Agent Browser 화면과 review 자료를 서로 다른 변경 실패 경계로 살폈다. 기준은 PR의 `main...HEAD` evidence diff이며, roadmap/STATUS 내용 자체는 PR #503 merge commit에서 이미 확인했다.

| # | 독립 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | PR #503 merge timestamp 또는 SHA가 Tailnet preview와 다른 commit을 가리킴 | PR metadata는 2026-10-22 06:31:18 UTC와 `36acdd4f48f23499835530704fa8aef20590a6e5`를 고정한다. |
| 2 | required check 상태가 추정으로 적힘 | PR #503의 빈 status-check rollup과 main protection API의 `required_status_checks=null`을 report와 표시 화면에 분리해 기록했다. |
| 3 | Tailnet 업데이트가 C11.4 전체 완료로 오해됨 | report·roadmap·STATUS·plan의 완료 수는 `77/213`, C11.4는 unchecked다. |
| 4 | PR #504가 실제 roadmap 변경을 포함하는 것처럼 설명됨 | README와 PR body는 page metadata 변경은 PR #503에서 merge됐고 #504는 동기화 evidence를 보관한다고 구분한다. |
| 5 | snapshot root가 최신 source revision인지 검증되지 않음 | 선택 root, RSPress summary, browser verification JSON 모두 source commit `530691ee889b03f40e79e216bab042efd5427e09`을 기록한다. |
| 6 | selected input에 미추적 workspace 자료가 섞임 | prep script는 Git commit object에서 문서와 evidence를 읽고 metadata가 untracked workspace evidence 제외를 기록한다. |
| 7 | 이전 preview file이 삭제되거나 누락됨 | target 계획과 copy report 모두 delete 0을 기록한다. 기존 4,534 target 중 동일 3,847개를 쓰지 않았다. |
| 8 | 덮어쓰는 bytes가 복구 불가 | 280개 이전 target bytes를 전용 pass28-final backup root에 저장해 hash로 확인했다. |
| 9 | preflight 이후 target 변경을 덮어씀 | sync script는 매 교체 직전 target hash/byte 수를 preflight에 재대조한다. |
| 10 | 교체 도중 부분 파일을 server가 읽음 | temp file에 쓰고 fsync한 뒤 같은 directory 내 atomic rename을 사용한다. |
| 11 | target에 접근하는 다른 사용자 process를 무시함 | lsof 결과가 있으면 exact preview PID/command와 idle listener 외 사용을 거부한다. PID 1191을 종료하지 않았다. |
| 12 | RSPress/tooling 버전이 evidence와 다름 | build log와 summary가 고정 RSPress 2.0.22를 기록하고 package pin을 변경하지 않았다. |
| 13 | plan/STATUS/evidence route alias가 누락됨 | build summary가 8개 문서 경로와 두 최종 simulator image asset을 필수 route로 확인한다. |
| 14 | 이미지 링크는 있으나 정적 site에 실제 bytes가 없음 | README HTML의 Android/iOS `img src`와 두 hashed `static/image` output을 확인했다. |
| 15 | build metadata 주장과 산출 site가 달라짐 | 4,534 files·229,005,804 bytes·tree SHA-256 `b60c17e4...b8b990fb`를 기록했다. |
| 16 | target 파일이 source manifest와 불일치 | local verification 4,534/4,534과 backup verification 280/280이 통과했다. |
| 17 | HTTP server가 local target과 다른/stale bytes 반환 | HTTP served verification이 4,534/4,534 status 200·bytes·SHA-256 일치를 기록한다. |
| 18 | endpoint 검증만으로 실제 페이지 렌더를 과장함 | Agent Browser에서 roadmap, STATUS, plan, runtime evidence를 별도 열어 merge SHA·`77/213`을 확인했다. |
| 19 | PR body screenshot URL이 404거나 실제 화면과 다름 | PR body의 roadmap/STATUS 이미지가 각각 1280×633, `complete=true`이고 브라우저 스크린샷에서 보인다. |
| 20 | 브라우저 또는 배포 범위가 사용자 요청 밖으로 확장됨 | Agent Browser만 사용했다. GitHub Pages/`gh-pages` 변경, Tailnet server 종료, Android 실기기 연결은 없다. |

## 검토 결론

diff에서 추가 수정이 필요한 blocker를 찾지 못했다. 동기화 결과는 private Tailnet 미리보기의 문서 bytes와 실제 rendered page에 한정한다. simulator evidence는 전체 CSS/Grid 호환성이나 C11.4 완료 판정이 아니다.
