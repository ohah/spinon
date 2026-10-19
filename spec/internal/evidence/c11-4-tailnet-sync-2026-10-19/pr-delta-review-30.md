# Tailnet 동기화 PR 변경 적대 검토 30개

검토 대상은 PR #347 뒤의 선택 문서 빌드, Tailnet 복사 기록, Agent Browser 화면, 인덱스 변경이다. 앱 구현 코드는 이 PR 범위에 포함하지 않는다.

| # | 실패 가능성 | 확인 결과 |
|---:|---|---|
| 1 | 기준 커밋이 PR #347 병합 전이라 변경 상태가 빠짐 | `preflight.json`의 source commit은 병합 후 `7e552f2d`다. |
| 2 | 기준 tree가 다른 source revision을 가리킴 | 사전 목록에 source tree SHA를 고정했다. |
| 3 | 선택 빌드가 전체 사이트 검증으로 오해됨 | README에서 선택 빌드임을 명시하고 final 로그를 연결했다. |
| 4 | dead link 검사를 생략한 사실이 숨겨짐 | 선택 설정에서 검사를 끈 사실과 전체 빌드 미수행을 기록했다. |
| 5 | 잘못된 Rspress CLI 경로로 성공을 주장함 | 초기 실패와 수정된 재시도, 최종 로그를 각각 보존했다. |
| 6 | 실패 로그가 사용자 계정 경로를 노출함 | 초기 CLI 실패 로그의 계정 경로를 `<workspace>`로 가렸다. |
| 7 | 로드맵이 최신 PR 설명을 반영하지 않음 | Agent Browser 화면에서 2026-10-19 C11.4 후속 문구를 확인했다. |
| 8 | 로드맵이 구현 전체 완료처럼 보임 | 고정 장면과 glyph paint·일반 matrix·전체 C11.4 한계를 문구에 남겼다. |
| 9 | 공식 상태 대장이 C11.4를 실수로 완료 처리함 | Agent Browser DOM에서 해당 checkbox의 `checked=false`, `disabled=true`를 확인했다. |
| 10 | 계획 문서가 새 Ahem 장면을 누락함 | 계획 화면에서 `min-content-text-normal` 후속과 Ahem 값을 확인했다. |
| 11 | 계획 문서가 glyph paint 검증까지 주장함 | 계획 문구는 fixture renderer가 glyph를 그리지 않는다고 명시한다. |
| 12 | 앱 실행 증거와 문서 스크린샷을 혼동함 | README는 Tailnet 화면을 문서 제공 근거로만 분류한다. |
| 13 | 실기기 검증으로 잘못 표기함 | 이번 동기화에서 AVD/Simulator를 실행하지 않았다고 명시했다. |
| 14 | 예상 파일 수와 실제 복사 범위가 다름 | `preflight.json`과 `copy-verification.json`은 모두 80개다. |
| 15 | target 경로 중복으로 같은 파일을 두 번 처리함 | 사전 목록의 경로 집합과 전체 target 수를 확인했다. |
| 16 | 크기 합계가 목록과 불일치함 | 80개 소스 byte 합은 4,762,311로 기록됐다. |
| 17 | 기존 파일을 무단으로 덮어씀 | 12개 교체 대상의 사전 백업을 전용 pass-1 위치에 만들었다. |
| 18 | 교체 파일 중 일부 백업이 누락됨 | backup 검증에는 12개가 있으며 모두 SHA-256 일치다. |
| 19 | 기존에 같은 파일인 항목을 불필요하게 교체함 | 14개 동일 파일은 `same`으로 분류하고 쓰지 않았다. |
| 20 | 새 경로가 기존 파일을 덮어씀 | 54개 신규 경로를 `created`로 분류하고 복사 후 hash를 비교했다. |
| 21 | 동기화가 사용자 파일을 삭제함 | preflight와 copy 기록 모두 `no_deletes=true`다. |
| 22 | 복사 중 일부 파일이 변형됨 | 80개 소스·대상 byte 수와 SHA-256이 일치한다. |
| 23 | HTTP 200만 보고 잘못된 본문을 통과시킴 | 응답별 byte 수와 SHA-256도 소스와 비교했다. |
| 24 | 미리보기 응답 일부가 누락됨 | served 검증은 80/80 성공, 실패 0이다. |
| 25 | 새 페이지가 참조하는 hashed script/search 자산이 Tailnet에 누락됨 | Agent Browser 후 출력 정적 자산을 비교해 누락된 7개를 찾았다. pass-3에 추가한 뒤 evidence 페이지와 세 이미지를 다시 열었다. |
| 26 | 터미널 분할형 브라우저를 사용함 | Agent Browser만 사용했고 Terminal Browser는 사용하지 않았다. |
| 27 | 사용 완료 후 Agent Browser가 계속 실행됨 | 캡처 뒤 세션을 닫고 세션·프로세스 종료 여부를 확인한다. |
| 28 | 기존 Tailnet listener를 재시작하거나 종료함 | 기존 listener에 파일을 복사했고 listener 수명은 건드리지 않았다. |
| 29 | GitHub Pages 배포를 Tailnet sync로 혼동함 | Pages 배포를 하지 않았다고 기록했다. |
| 30 | PR 이미지 링크가 branch 이동 뒤 깨짐 | 병합 뒤 PR 본문 링크를 `main` raw 경로로 바꾸고 Agent Browser로 재확인한다. |

판정: 문서 동기화와 선택된 파일의 제공은 검증했다. 전체 문서 링크 검사, 전체 사이트 빌드, Pages 배포, 앱 동작은 이 검토 범위 밖이다.
