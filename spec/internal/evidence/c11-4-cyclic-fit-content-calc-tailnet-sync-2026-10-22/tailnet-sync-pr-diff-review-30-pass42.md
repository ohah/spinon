# Tailnet pass42 evidence PR 변경분 적대 검토 30개

검토 대상은 pass42 선택·build·동기화 스크립트, 결과 JSON·로그, 화면 캡처와 `README-pass42.md`다. 각 항목에서 서로 다른 실패 원인, 안전 경계 또는 해석 과장을 확인했다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 계획이 merge되지 않은 source로 preview에 들어감 | workspace와 `origin/main`의 commit SHA는 다르지만 source tree `87af311…`는 일치한다고 준비 기록이 구분한다. |
| 2 | pass41 기반 root 또는 site를 덮어씀 | 입력은 pass41 selected root이고 출력 root·site는 별도 pass42 경로다. |
| 3 | 추적 파일 변경을 소스 상태로 섞음 | 준비 스크립트는 tracked workspace status가 비어 있지 않으면 중단한다. |
| 4 | 선택 복사 문서가 현재 원본과 다름 | status·roadmap·두 계획·부모 계획·두 fixture 복사본마다 bytes와 SHA-256을 기록한다. |
| 5 | 계획 검토나 사전 비교 근거가 빠짐 | 집중 계획, plan review 20, Chrome 관측·독립 기대값·overflow 표본·종료 JSON을 route manifest에 포함한다. |
| 6 | 세 evidence 목적지 중 하나의 파일 계층이 누락 | 신규 root에 `internal`, `spec/internal`, `public/internal` evidence를 각각 복사하고 각 출처를 기록한다. |
| 7 | source 또는 선택 root가 symlink/특수 디렉터리 | 재귀 검사에서 root와 모든 하위 항목을 `lstat`하고 plain directory·file만 허용한다. |
| 8 | 준비 중단 후 resume가 기존 데이터를 덮어씀 | resume는 기반 root에서 허용된 source target 외 파일을 비교하고, 이미 있는 evidence tree는 전체 파일 집합·내용이 같을 때만 재사용한다. |
| 9 | 부분 root에 무관한 추가 파일이 섞임 | resume 때 기반 root에 없는 경로는 허용된 source target/evidence 하위 경로가 아니면 거부한다. |
| 10 | 사전 준비 기록이 재실행 시 다른 내용으로 바뀜 | 기존 JSON이 있으면 새 산출물과 bytes가 같아야만 통과하는 idempotent 기록 방식을 사용한다. |
| 11 | 문서 도구 버전이 고정 정책을 위반 | RSPress summary는 저장소 고정 버전 `2.0.22`를 기록한다. |
| 12 | pass41 build output을 pass42 build가 비움 | config는 새 pass42 root·outDir을 사용하고, 실제 log의 dist 경로도 pass42다. |
| 13 | RSPress의 dist 경고를 숨기거나 실패로 오해 | log는 경고와 성공을 모두 보존한다. 전용 output 경로에서 page render와 build가 완료됐다. |
| 14 | 전체 파일 수는 맞지만 새 계획 route가 누락 | summary가 새 plan, plan review, evidence README, PNG/JSON, fixture 등 개별 필수 route를 검사한다. |
| 15 | 전체 build를 해시 없이 성공으로만 주장 | summary는 3,048개, 129,586,338 bytes 및 전체 tree SHA-256을 기록한다. |
| 16 | preview roadmap을 새 기능 구현 또는 완료로 읽게 함 | README와 캡처는 main의 기존 roadmap 문구와 `72/213`을 보여주며 새 plan은 별도 route로 제공한다. |
| 17 | status 대장과 완료 수를 몰래 변경 | source tree는 main과 같고 status DOM은 `72/213` 및 plan-only 상태를 확인한다. |
| 18 | preflight 목록에서 산출물 경로가 빠짐 | build summary 3,048개와 preflight 3,048개를 대조하며 source root가 pass42 site로 고정돼 있다. |
| 19 | sync 중 기존 preview 파일 삭제 | 스크립트에는 삭제 단계가 없고 preflight·copy·local 결과가 모두 `no_deletes: true`다. |
| 20 | 새 파일 생성·교체·동일 분류가 서로 모순 | preflight와 copy 모두 create `196`, replace `174`, same `2,678`이고 합계가 3,048이다. |
| 21 | replace 이전 bytes가 백업 전에 소실 | 별도 pass42 백업은 174개와 23,476,560 bytes를 기록하고 local verification이 전부 재검증한다. |
| 22 | 이전 백업 파일을 다른 원본으로 재사용 | 기존 backup은 preflight의 byte 수와 SHA에 일치할 때만 재사용하고 신규 파일은 `wx`로 만든다. |
| 23 | 동기화 전후 target 또는 source가 바뀜 | copy 직전 각각 preflight bytes/hash를 다시 비교하고 mismatch면 쓰기를 중단한다. |
| 24 | preview 경로 symlink로 지정 root 밖에 기록 | target root와 각 component를 `lstat`해 symlink 및 잘못된 파일·디렉터리 경계를 거부한다. |
| 25 | 부분 파일이 보이거나 임시 파일을 덮어씀 | 같은 디렉터리 `wx` 임시 파일을 쓰고 `fsync` 후 atomic rename하며 기존 임시 경로가 있으면 중단한다. |
| 26 | 열린 파일이나 활성 연결 중인 서버를 종료·교체 | server PID/단일 listen socket을 확인하고, 교체 target에 비-preview holder 또는 추가 socket이 있으면 거부한다. 서버 PID `1191`은 유지했다. |
| 27 | local 쓰기 성공만으로 복사 정확성을 주장 | 복사 결과와 독립 local verify가 각각 3,048/3,048 target, 174/174 backup hash를 확인한다. |
| 28 | Tailnet HTTP 200을 오래된 본문에도 성공 처리 | 요청별 final URL, status, 길이와 SHA를 비교해 3,048/3,048 HTTP 200·hash 일치, 실패 0을 기록한다. |
| 29 | 문서/이미지 route 또는 PR body image가 깨짐 | Agent Browser DOM은 네 scene·overflow 설명을 확인하고 Tailnet PNG 및 PR body raw screenshot 두 장이 모두 `complete=true`, 1280×633임을 확인했다. 계획·roadmap·status·evidence·PR 화면도 캡처했다. |
| 30 | 확인 후 Agent Browser daemon이 남거나 다른 세션을 종료 | `spinon-tailnet-pass42`와 `spinon-pr484-verify`만 닫았다. 사후 session list/process scan은 비었고 다른 session은 종료하지 않았으며 기존 Tailnet server를 보존했다. |

## 판정

이 PR은 private Tailnet preview 갱신과 그 근거만 추가한다. Chrome 156은 계획 단계 사전 비교일 뿐 Spinon runtime이나 Android/iOS Simulator 검증이 아니며, production 배포·일반 CSS 적합성·C11.4 완료 근거로 확대 해석하지 않는다. `STATUS.md`, public `roadmap.html`, 공식 완료 수는 수정하지 않았다.
