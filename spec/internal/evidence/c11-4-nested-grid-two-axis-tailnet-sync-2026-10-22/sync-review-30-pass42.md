# C11.4 Tailnet sync PR diff 적대 검토 — pass42

검토 범위는 PR #478 병합 뒤 최신 main의 C11.4 진행 기록을 공개 roadmap·Tailnet 미리보기에 반영한 diff다. 별도 sync 절차, 선택 build, 기존 preview 백업, 원격 응답 hash와 Agent Browser 화면을 각각 확인했다. 각 항목은 서로 다른 실패 경로를 다룬다.

| # | 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | merge SHA가 PR head commit과 혼동될 수 있다 | roadmap·계획 모두 GitHub merge commit `907390f75e0b25b399a47b6c863a19b7e5a326e7`을 쓴다. 통과. |
| 2 | merge 일시의 UTC/KST 변환이 틀릴 수 있다 | GitHub `2026-10-21T22:14:28Z`를 2026-10-22 07:14:28 KST로 표기했다. 통과. |
| 3 | PR check가 없는데 CI green으로 오해할 수 있다 | roadmap·계획은 check rollup이 비어 있었고 required status checks가 설정되지 않았다고 구분한다. 통과. |
| 4 | PR merge로 C11.4 전체 완료가 암시될 수 있다 | 신규 roadmap 행과 계획은 고정 scene 범위를 명시하고 C11.4 전체 미완료를 유지한다. 통과. |
| 5 | 상위 완료 수가 증가할 수 있다 | source STATUS, 선택 build, Agent Browser에서 공식 `72/213`을 확인했다. 통과. |
| 6 | 상태 대장의 C11.4 체크가 잘못 켜질 수 있다 | 렌더된 `C11.4 intrinsic track sizing` checkbox의 `checked=false`를 확인했다. 통과. |
| 7 | 네 장면이 임의 CSS 지원으로 일반화될 수 있다 | roadmap은 네 fixed scene과 일반 CSS admission의 미확인을 함께 기록한다. 통과. |
| 8 | 축별 control을 동시 cyclic scene으로 잘못 설명할 수 있다 | roadmap은 both-axis, columns-only, rows-only, definite control을 별도로 적는다. 통과. |
| 9 | 두 cyclic 축의 track 배열이 역순·오타일 수 있다 | 기대 Columns `[20,10,20]`, Rows `[10,5,10]`을 plan/runtime 근거와 대조했다. 통과. |
| 10 | overflow item이 root 안에 있다고 설명할 수 있다 | `(30,15) 20×10px` item은 `40×20px` root의 오른쪽 10px·아래 5px를 넘는다고 기록한다. 통과. |
| 11 | Android AVD 결과가 실기기 검증으로 읽힐 수 있다 | API 37 ARM64 AVD를 명시하고 Android 실기기는 미검증으로 둔다. 통과. |
| 12 | iOS 결과가 iPhone 실기기 결과로 읽힐 수 있다 | iPhone 17 Pro / iOS 26.2 Simulator로 표기한다. 통과. |
| 13 | PixelCopy 표본이 glyph paint 증거로 확대될 수 있다 | runtime 근거는 text item 배경 표본과 `runtime_text_count=0` 한계를 보존한다. 통과. |
| 14 | Metal readback이 compositor 이후 pixels로 잘못 표기될 수 있다 | roadmap과 runtime evidence는 drawable 제출 전 표본이라고 명시한다. 통과. |
| 15 | fixture-test positive control이 release artifact 보장을 뜻할 수 있다 | 기본 debug staticlib 경계로 한정하고 최종 release/app bundle 주장으로 확대하지 않는다. 통과. |
| 16 | 공개 roadmap 이미지가 이전 scene을 가리킬 수 있다 | Android/iOS 링크를 PR #478의 `cyclic-percent-two-axis-multitrack.png`에 연결했다. 통과. |
| 17 | 내부 계획 링크가 없는 route로 끝날 수 있다 | build summary에서 C11.4 계획·사전 비교·runtime README route를 확인했다. 통과. |
| 18 | 새 계획·근거가 Tailnet의 `/spinon/` prefix에서 깨질 수 있다 | Agent Browser에서 각 상대 경로가 Tailnet `/spinon/` route로 열렸다. 통과. |
| 19 | roadmap 변경이 public source가 아니라 preview 임시 파일에만 남을 수 있다 | source `spec/public/roadmap.html`에도 PR #478 기록을 추가했다. 통과. |
| 20 | plan의 merge metadata가 roadmap과 다를 수 있다 | 둘 다 같은 merge SHA와 시각을 표시하고 선택 build에서 읽었다. 통과. |
| 21 | 선택 build가 pass41 root를 덮어써 복구 근거를 없앨 수 있다 | pass41 root는 read-only 입력으로 쓰고 pass42 전용 root/site를 새로 만들었다. 통과. |
| 22 | unrelated untracked simulator/browser evidence가 배포될 수 있다 | selected root는 tracked main 차이만 복사하고 workspace untracked evidence를 제외했다. 통과. |
| 23 | 선택 산출물에 symlink가 포함되어 target 밖 파일을 따라갈 수 있다 | build summary가 선택 site 전체를 순회해 symlink를 거부하고 target preflight도 path segment별 symlink를 거부한다. 통과. |
| 24 | preview에 남은 파일이 삭제될 수 있다 | 동기화는 build target만 create/replace하고 preflight에서 delete 0을 기록한다. 통과. |
| 25 | 교체 전 bytes가 backup과 다를 수 있다 | 261개 backup 모두 preflight의 이전 byte 수와 SHA-256으로 검증했다. 통과. |
| 26 | copy 도중 정적 서버가 부분 파일을 읽을 수 있다 | 변경 파일은 같은 디렉터리 임시 파일에 쓴 뒤 atomic rename했고 각 target hash를 재확인했다. 통과. |
| 27 | 활성 preview 요청 중 파일 교체가 일어날 수 있다 | copy 직전 lsof에는 PID 1191의 `127.0.0.1:8768` listener만 보였고 client socket은 없었다. 통과. |
| 28 | preview server나 사용자의 tmux 작업을 정리 과정에서 종료할 수 있다 | preview PID 1191은 유지됐고 tmux에 접근하지 않았다. 통과. |
| 29 | 원격 HTTP 200이 내용 동일성을 증명한다고 오해할 수 있다 | 4,505개 전부에 대해 status, byte 수, SHA-256을 선택 build와 비교했다. 실패 0. 통과. |
| 30 | 브라우저·이미지 확인 뒤 Agent Browser가 남을 수 있다 | Agent Browser에서 roadmap/status/plan/runtime 및 두 이미지 natural size를 확인한 뒤 session을 닫고 활성 session·Chrome process 0을 확인했다. 통과. |

근거는 같은 디렉터리의 `selected-build-summary-pass42.json`, `preflight-pass42.json`, `copy-and-backup-pass42.json`, `final-verification-pass42.json`, `served-verification-pass42.json`, `agent-browser-verification-pass42.json`, `browser-close-pass42.json` 및 roadmap screenshot이다. GitHub Pages workflow는 실행하지 않았다.
