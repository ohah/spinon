# S04.10 Tailnet 동기화 PR 변경 관점 검토 30개

검토 대상은 PR #292의 S04.10 병합 메타데이터, 동기화 기록, 선택 Rspress 산출물과 Tailnet 브라우저 근거다. 각 항목은 별도의 실패 경로 또는 계약 경계를 확인한다.

| # | 실패 관점 | 확인 및 판정 |
| --- | --- | --- |
| 1 | 문서가 후보 PR을 완료 기능으로 오인 | PR #291과 merge SHA `e906aed109a7af2b9f7ccfd0cd64ba4b806f5869`만 후보 근거로 연결했다. S04.10 checkbox는 미완료다. |
| 2 | S05.2 완료가 S04.10 후보에서 파생 | STATUS의 S05.2 checkbox도 미완료이며 브라우저에서 unchecked/disabled를 확인했다. |
| 3 | 공개 로드맵의 완료 수를 임의로 올림 | Tailnet 페이지가 읽은 공식 수 `74/207`과 미완료 207 표시를 기록했다. |
| 4 | 내부 후보가 플랫폼 callback 구현으로 표현 | README와 roadmap은 순수 Rust registry 후보로 한정하고 callback 미검증을 유지한다. |
| 5 | `Queue::present`나 readback을 presentation timing으로 확대 | 기존 contract와 STATUS의 제출/readback 한계를 보존했다. |
| 6 | 앱 입력 순서·clock 검증을 암시 | 입력 clock/order와 callback 직렬화가 미검증임을 문서에 남겼다. |
| 7 | 광학 scanout을 확인한 것처럼 표현 | status와 evidence 모두 compositor 이후 표시·광학 scanout 확인을 주장하지 않는다. |
| 8 | Simulator/실기기 확인을 새 PR에 포함했다고 표현 | 이 문서 slice에서는 simulator 및 physical device를 실행하지 않았다. |
| 9 | 문서 screenshot을 앱 behavior evidence로 사용 | screenshot 캡션과 README가 문서 페이지 캡처라고 구분한다. |
| 10 | Tailnet 변경을 GitHub Pages 배포로 혼동 | 별도 preview root 복사만 기록하고 Pages workflow를 호출하지 않았다. |
| 11 | 전체 docs build를 선택 build 결과로 과장 | Rspress 선택 route 범위와 full docs build 미실행을 명시했다. |
| 12 | 선택 build의 링크 검사 한계를 숨김 | 전체 Markdown link/image 검사 미실행을 README에 기록했다. |
| 13 | Rspress 버전이 프로젝트 고정을 벗어남 | 빌드 로그에서 `2.0.22`를 확인했다. |
| 14 | `/spinon/` base path 누락으로 Tailnet route 파손 | roadmap, status, contract, evidence route를 Agent Browser에서 열었다. |
| 15 | 문서가 선언한 icon asset 누락 | `/spinon/icon.svg`가 200이고 문서 head의 rel=icon 대상임을 확인했다. |
| 16 | root favicon probe 404를 숨기거나 사이트 오류로 과장 | `/favicon.ico` 404를 network/evidence에 기록하고 선언된 icon route와 분리했다. |
| 17 | roadmap 후보 note가 preview에 반영되지 않음 | Tailnet 페이지 본문에서 2026-10-18 S04.10 confirmed-frame note를 확인했다. |
| 18 | 상태 대장 checkbox가 로드 전 값으로 오인 | STATUS route의 렌더 완료 후 S04.10/S05.2 checkbox의 checked/disabled 값을 읽었다. |
| 19 | 공식 count가 오래된 screenshot에서만 유래 | live Agent Browser 페이지 본문에서 `미완료 207개 표시 · 공식 완료 74/207`을 읽었다. |
| 20 | Tailnet host나 로컬 경로가 증거에 노출 | browser JSON과 network log에서 Tailnet host를 redacted 처리했다. |
| 21 | private preview root 경로를 PR evidence에 노출 | manifest/preflight는 root 이름만 기록하고 절대 user path를 사용하지 않는다. |
| 22 | copy가 preview의 기존 파일을 덮고 backup을 생략 | 변경 파일은 copy 전에 전용 backup root로 저장했고 backup SHA를 비교했다. |
| 23 | 기존 backup 산출물을 두 번째 동기화에서 덮어씀 | 최종 동기화는 별도 전용 backup root를 사용한다. |
| 24 | 동일 파일을 불필요하게 다시 씀 | preflight는 source와 target SHA가 같은 파일을 skip-identical로 분류한다. |
| 25 | 파일 누락 또는 잘못된 bytes가 Tailnet에 제공 | 최종 HTTP verification에서 status, byte length와 SHA-256을 manifest에 대조한다. |
| 26 | local server와 Tailnet edge 응답이 다른데 놓침 | 두 endpoint의 HTTP status와 body hash를 별도로 기록한다. |
| 27 | preview 경로에서 파일을 삭제해 다른 문서를 깨뜨림 | copy plan의 deletion 수가 0이고 delete operation은 없다. |
| 28 | preview listener를 재시작하거나 종료 | 기존 listener는 그대로 유지했고 response 확인으로 생존을 확인했다. |
| 29 | Agent Browser process를 남김 | session 종료 후 task-created process 상태를 별도 JSON에 기록한다. |
| 30 | evidence README/index에서 새 근거로 진입 불가 | 내부 S04.10 색인에서 README 및 30-perspective review로 이어지는 링크를 확인한다. |

검토 중 확인한 한 가지 비성공 요청은 브라우저의 root `/favicon.ico` probe이며, 명시된 `/spinon/icon.svg`는 200이었다. 결과를 숨기지 않고 README와 browser JSON에 기록했다. 별도 수정은 필요하지 않았다.
