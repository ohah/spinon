# PR #839 Tailnet sync evidence 변경 적대 검토 30개

대상 PR #839의 sync evidence 문서, 선택 빌드·copy/verify scripts, manifests, logs와 Agent Browser screenshots. PR #838 roadmap content review와 별도다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | source revision이 Tailnet에 반영한 main과 다르다 | **통과.** 선택 root/publication manifests는 PR #838 merge SHA `e886dfebccc406ac6f7303a462b228ef3f258248`와 tree를 기록한다. |
| 2 | 구현 PR merge revision을 잘못 표기한다 | **통과.** PR #837 merge SHA `12c11815165077d570d4d9c1674b0785047cba67`, merge 시각과 GitHub PR URL을 기록한다. |
| 3 | 정적 roadmap update PR merge identity가 다르다 | **통과.** PR #838 merge SHA `e886dfebccc406ac6f7303a462b228ef3f258248`와 시각을 기록한다. |
| 4 | sync evidence가 GitHub Pages 배포로 과장된다 | **통과.** Spinon Tailnet 개발 미리보기라고 명시한다. |
| 5 | 전체 문서 검증을 한 것처럼 쓴다 | **통과.** 선택 빌드이며 full dead-link/dead-image 검증은 false라고 적는다. |
| 6 | 선택 root에 새 roadmap/status/plan source가 빠진다 | **통과.** source manifest와 required route list가 roadmap, STATUS, C11.4 plan 및 runtime evidence를 각각 포함한다. |
| 7 | RSPress version이 프로젝트 고정 버전과 다르다 | **통과.** RSPress 2.0.22로 기록한다. |
| 8 | build output을 기존 산출물 위에 덮어쓴다 | **통과.** build staging scripts는 unique output 존재를 확인하고, selection retry는 다른 pass02 publication path를 사용한다. |
| 9 | 첫 staging 실패를 성공으로 세거나 감춘다 | **통과.** `build/spinon` 부모 부재의 `ENOENT` 실패와 재귀 경로 생성 수정이 README에 분리 기록된다. |
| 10 | 잘못된 Bun 인자 호출을 build 통과로 세운다 | **통과.** help 출력 실패를 설명하고 filter + `--` 인자 구분자를 사용한 RSPress 성공 실행만 성공으로 기록한다. |
| 11 | publication marker mismatch로 중단된 결과를 삭제한다 | **통과.** pass01의 partial staging은 남겨두고 별도 pass02 output을 사용한다. |
| 12 | current main SHA와 PR merge SHA를 혼동해 page content 검증이 약해진다 | **통과.** publication script는 main source `e886...`와 roadmap card의 PR #837 merge SHA `12c...`를 분리 대조한다. |
| 13 | symlink 또는 traversal path를 publication에 복사한다 | **통과.** 선택 source walk에서 symlink와 비정상 entry를 거부하고 safe route를 검증한다. |
| 14 | publication 파일 수나 byte total을 잘못 기록한다 | **통과.** 102 routes와 11,589,066 bytes가 selected publication manifest에 집계된다. |
| 15 | 새로운 Tailnet target을 기존 동일 파일로 오분류한다 | **통과.** preflight는 path별 size/hash로 create/replace/same을 비교한다. |
| 16 | 변경 파일 backup을 만들지 않는다 | **통과.** 기존 15개 replacement의 byte-exact hash backup을 전용 pass02 path에서 검증한다. |
| 17 | sync가 target 파일을 삭제한다 | **통과.** copy scripts와 결과 모두 `no_deletes=true`다. |
| 18 | preflight 이후 source 또는 target drift를 덮어쓴다 | **통과.** copy 직전 source 및 기존 target hash를 preflight와 다시 비교한다. |
| 19 | 원자 교체 중 partial file을 노출한다 | **통과.** temporary file을 `wx`로 작성하고 fsync 후 rename하며 final target hash를 검사한다. |
| 20 | 다른 process가 target file을 열었는데 강제로 교체한다 | **통과.** 열린 handle PID/command를 확인하며 사전 관측은 빈 목록이다. |
| 21 | 미리보기 server를 재시작하거나 종료한다 | **통과.** server PID 28040과 listener를 확인하고 process를 건드리지 않았다. |
| 22 | 복사 완료를 local disk hash 없이 보고한다 | **통과.** target 102/102와 backup 15/15 local SHA-256 결과를 기록한다. |
| 23 | Tailnet HTTP 200만 확인하고 payload 차이를 놓친다 | **통과.** remote response size와 SHA-256을 모두 source manifest와 비교해 102/102 일치한다. |
| 24 | Agent Browser 대신 terminal browser로 rendered page를 확인한다 | **통과.** roadmap/status/plan은 격리된 Agent Browser session으로 열고 DOM에서 추출했다. |
| 25 | screenshot이 이전 roadmap content를 보여준다 | **통과.** screenshot은 sync 이후 열린 새 #837 card가 첫 progress card임을 확인한 session에서 캡처했다. |
| 26 | status 화면에서 72/213을 다른 state와 혼동한다 | **통과.** DOM은 72/213, C11=false, C11.3=false, C11.3.3=false, C11.4=false, C11.2.2/2.3=true를 따로 기록한다. |
| 27 | 새 plan/evidence route가 링크 표시만 되고 응답이 없다 | **통과.** browser plan title/content 확인과 102-route HTTP/hash 검증으로 route를 확인했다. |
| 28 | browser screenshot을 화면 evidence 없이 언급한다 | **통과.** roadmap/status 1280×633 viewport PNG 두 개를 문서 링크로 포함한다. |
| 29 | 종료 요청 후 Agent Browser session/process를 남긴다 | **통과.** 사용 세션을 닫았고 최종 `active=false`, `pid=null`; 다른 session은 닫지 않았다. |
| 30 | test-only slice로 공식 완료 집계를 올리거나 simulator 결과를 주장한다 | **통과.** C11/C11.4 unchecked와 72/213을 유지하고 앱 runtime/simulator/WPT를 주장하지 않는다. |

**판정:** 30개 독립 관점에서 sync evidence를 검토했다. pass01 실패와 pass02 성공을 구분했고, PR 체크·전체 문서 link 검증·앱 runtime을 통과했다고 과장하지 않는다.
