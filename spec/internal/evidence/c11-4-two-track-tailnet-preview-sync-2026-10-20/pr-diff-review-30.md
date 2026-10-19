# C11.4 two-track Tailnet 갱신 PR diff 적대 검토 30개

**대상:** `docs/c11-4-two-track-tailnet-sync-20261020`의 최종 변경 전체. 공개 roadmap·상태·계획·색인, 선택 root/build·sync helper, 고정 입력 manifest, 실행 기록·화면·검토 문서를 서로 다른 경계에서 확인한다.

| # | 적대 질문 | 확인 결과 |
| ---: | --- | --- |
| 1 | 변경이 PR #365의 잘못된 merge commit을 기록하지 않았나? | 집중 계획, `spec/STATUS.md`, 내부 색인의 SHA가 실제 PR #365 merge commit `a2c137213f72e82b3a8e3cc153e92c8b9db9a653`와 일치한다. |
| 2 | 이번 변경이 C11.4를 완료로 표시하나? | checkbox를 추가·수정하지 않았다. 공개 roadmap과 세 내부 기록 모두 C11.4 미완료라고 유지한다. |
| 3 | 상태 대장 수와 roadmap 표시값을 같은 집계로 혼동하나? | 완전 로드 UI는 72/212 task rows, raw Markdown은 86/225 checkbox다. 문서는 ID 없는 하위 확인 13개와 중복 ID의 상태 map 동작을 밝혀 두 숫자를 분리한다. |
| 4 | `78/0` 초기 로딩 관찰을 결과로 오인하거나 완료 수로 단정하나? | README와 sync review는 중간 hydration 값을 버리고 UI의 최종 `72/212`를 표시값으로 기록하며 raw checkbox 합계와 같다고 하지 않는다. |
| 5 | Simulator 결과를 실기기 또는 compositor 이후 표시로 확장했나? | roadmap·README는 Android API 37 ARM64 AVD 및 iOS Simulator 수준을 명시하고 실기기/hardware GPU/compositor-after를 남은 한계로 둔다. |
| 6 | 두 축/두 platform의 scene 수를 혼동하지 않았나? | 공개 요약은 각 플랫폼 21/21이고, 신규 scene 8개와 기존 scene 13개를 분리해 적는다. |
| 7 | PR #365가 수정한 app allowlist 문제를 숨기거나 해결되지 않은 것으로 썼나? | 요약은 최초 iOS allowlist 누락을 두 platform gate 보완으로 해결했다고 적으며, 세부 route를 연결한다. |
| 8 | Chrome oracle 결과를 Spinon 적합성으로 잘못 부르나? | 링크와 요약은 Chrome 155를 사전 비교라 부르고 WPT·전체 CSS 적합성 미완료를 유지한다. |
| 9 | auto-fit 차이나 전체 repeat grammar를 지원한다고 주장하나? | 공개 요약과 README에 `auto-fit` 축 차이 및 전체 repeat grammar가 남아 있다고 명시한다. |
| 10 | public roadmap의 새 C11.4 기록이 기존 C11.4 이력과 혼동되나? | 날짜·`definite minimum-size + two-track auto-fill` 제목·PR #365 링크로 이력을 구별한다. |
| 11 | roadmap의 계획 링크가 잘못된 staging path를 가리키나? | 선택 root는 `plan/c11-4-auto-repeat-two-track-min-size.md`를 명시적으로 같은 경로에 복사하고 Browser route가 열렸다. |
| 12 | 내부 색인 상대 링크가 `spec/internal` 구조에서 깨지나? | `spec/internal/README.md`의 `../../plan` 및 `./evidence` 링크 대상이 선택 root에 포함된다. |
| 13 | STATUS의 새 기록이 체크리스트 문법을 깨뜨리나? | 기존 C11.4 세부 행 아래 서술형 문장을 추가했고 상위 체크박스 패턴과 nesting을 바꾸지 않았다. |
| 14 | STATUS 상대 링크가 Tailnet route와 다른 배포 경로를 가리키나? | 미리보기 `/spinon/STATUS.html` route는 선택 plan/evidence 문서를 `/spinon/` 내부 route로 변환하며 HTTP·Browser에서 열렸다. |
| 15 | 선택 root가 새 실행 README만 포함하고 oracle 근거를 빠뜨리나? | pass11 선택 목록에 집중 계획, auto-repeat neighbor, Chrome precomparison, Android/iOS 실행 근거를 포함한다. |
| 16 | 선택한 근거를 `internal/`과 `spec/internal/` 중 한쪽에서만 제공하나? | 선택 root manifest는 각 evidence directory를 두 route prefix 모두에 복사한다. |
| 17 | 미선택 staging source나 host path가 공개 미리보기 파일에 남나? | 선택 root redaction과 sanitized output 검사에서 source-local redaction 125개, 추가 bundle redaction 1개, 남은 host path 0개다. |
| 18 | 새 pass 번호가 helper 중 하나에서만 허용되어 재현이 깨지나? | prepare-root, Rspress config, sanitizer, manifest writer, sync helper가 모두 `pass11`을 허용한다. |
| 19 | pass11 build가 과거 run 산출물을 덮어쓰나? | root와 site 경로에 `pass11`이 들어가며 이전 pass 출력은 분리된다. |
| 20 | sync helper가 임의 source build를 받아 복사할 수 있나? | source-relative 경로가 sanitized `c11-4-image-tailnet-site-${pass}` 출력과 정확히 일치해야 한다. |
| 21 | 이 변경이 기존 pass6–pass9 backup을 새 작업 폴더로 이동시키나? | `backupOperation`은 pass11에서만 새 이름을 쓰고 과거 pass에는 기존 `c11-4-image-tailnet-preview-update-2026-10-20`을 유지한다. |
| 22 | Tailnet destination/backup 검증 범위가 약해졌나? | 고정 사용자 `SpinonPreview` 경로, symlink 거부, preflight 재검증 및 별도 backup 절차가 유지된다. |
| 23 | 현재 Tailnet 사용 중인 파일을 교체 전에 확인했나? | 기록된 교체 대상 37개에 대해 open handle 0개를 확인했다. |
| 24 | 교체 전 원본 backup을 검증하지 않고 복사했나? | 기존 37개 파일의 backup byte/hash 검증 결과가 37/37이다. |
| 25 | 미리보기의 선택 밖 파일을 지우나? | preflight와 copy 결과 모두 `no_deletes: true`; 생성 495, 교체 37, 동일 401이다. |
| 26 | local copy 결과만으로 live Tailnet을 성공이라 기록했나? | Tailnet HTTP 응답 933개 모두 byte 수와 SHA-256이 source와 일치한다. |
| 27 | screenshot이 터미널 browser나 앱 화면이라고 표시되나? | 캡처는 Agent Browser로 열어 본 Tailnet 문서 route이며 README에서 앱 화면과 구분한다. |
| 28 | evidence page의 앱 이미지가 링크만 있고 실제 로드되지 않았나? | Browser natural dimensions로 Android 1080×2400, iOS 1206×2622를 확인했다. |
| 29 | PR 본문 그림으로 연결할 roadmap 화면이 실제 route를 보여주나? | `tailnet-roadmap-agent-browser.png`는 새 PR #365 roadmap 항목이 보이는 live Tailnet 화면이다. |
| 30 | 검토/정리 뒤 Agent Browser가 남아 사용자 프로세스와 겹치나? | session close 후 전용 Chrome이 닫혔고 Agent Browser daemon도 더 이상 실행 중이지 않다. |

## 검토 중 수정

- 기존 sync helper의 backup directory가 모든 pass에 새 이름을 적용하는 것을 발견했다. pass11만 새 backup directory를 사용하고 기존 pass6–pass9는 원래 경로를 유지하도록 수정했다. pass11 동기화에서 사용한 경로와 결과는 바뀌지 않았다.
- 최종 diff에서 상태 checkbox·공식 완료 수를 건드리지 않았고, 이미지·API·앱 런타임 변경도 추가하지 않았음을 확인했다.
