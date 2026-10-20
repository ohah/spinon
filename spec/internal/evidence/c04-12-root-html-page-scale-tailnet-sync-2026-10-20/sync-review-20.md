# Tailnet 동기화 실패 관점 검토 20개

| # | 실패 관점 | 확인 및 판정 |
| ---: | --- | --- |
| 1 | 저장소 main 문서가 아닌 오래된 다른 checkout을 원본으로 쓰는가 | 현재 기본 checkout과 PR #388 merge SHA를 기록하고 선택 파일의 입력 hash를 고정한다. |
| 2 | 문서 편집 중 무관한 미추적 캡처 로그가 preview에 섞이는가 | 입력 목록은 명시 선택 source로 만들고 broad C04.12 임시 로그와 C11.4/C12.4 미추적 자료는 제외한다. |
| 3 | preview target이 예상 위치가 아닌 임의 디렉터리를 가리키는가 | helper는 `~/Library/Application Support/SpinonPreview`만 target으로 허용한다. |
| 4 | backup이 원본 preview 외부나 다른 실행과 공유되는가 | task·pass별 `SpinonPreviewBackups` 경로를 고정한다. |
| 5 | symlink를 따라 기존 파일을 덮거나 root 밖으로 나가는가 | source, target, backup의 디렉터리와 파일 항목에서 symlink를 거부한다. |
| 6 | Rspress 버전이 프로젝트 정책과 다르게 바뀌는가 | 저장소에 고정된 `@rspress/core` 2.0.22와 lockfile을 그대로 사용한다. |
| 7 | HTML roadmap이 Markdown과 같은 route로 잘못 복사되는가 | `spec/public/roadmap.html`을 Rspress public root의 `roadmap.html`로 매핑하고 실제 route를 연다. |
| 8 | Markdown 상대 링크가 staging 경로를 노출하는가 | 선택된 route는 `/spinon/`로 rewrite하고 미선택 문서는 GitHub `main` 링크로 바꾼다. |
| 9 | page-scale 증거 링크가 빠지거나 이전 파일명으로 남는가 | 계획, pixel 분석, 20-point/30-point review와 이미지 파일을 선택 manifest 및 browser route에서 대조한다. |
| 10 | PR review 또는 계획에 병합 대기 표현이 남는가 | 대상 세 문서에서 pending 문구를 검색하고 merge SHA/시각을 대조한다. |
| 11 | C04.12 상위 항목이 하위 slice 때문에 완료 처리되는가 | 상위 checkbox는 unchecked로 유지하고 하위 page-scale mapping만 checked로 둔다. |
| 12 | 공식 완료 집계가 잘못 증가하는가 | 상위 항목 및 집계 source는 바꾸지 않고 browser에서 기존 값을 확인한다. |
| 13 | branch protection이나 check 결과를 CI 통과로 과장하는가 | PR check rollup이 비어 있고 required status check가 없음을 분리해 명시한다. |
| 14 | 교체 직전 preview 파일이 백업되지 않는가 | 교체 대상 기존 bytes를 pass별 backup에 쓰고 크기·SHA-256을 검증한다. |
| 15 | 서버가 열고 있는 HTML을 truncate해 제공 중 응답이 깨지는가 | copy 직전 교체 대상 `lsof`를 기록하고 same-directory temp write 후 atomic rename한다. |
| 16 | 동기화가 사용 중인 Tailnet 서버를 종료하는가 | PID 1191의 생존을 확인하고 유지한다. |
| 17 | 동기화가 사용자의 tmux 세션을 종료하거나 바꾸는가 | tmux 세션을 조회·변경·종료하지 않는다. |
| 18 | 이전 preview의 다른 페이지를 선택 동기화가 삭제하는가 | helper는 source에 포함된 파일만 생성/교체하며 삭제 작업은 없다. |
| 19 | 로컬 복사만 성공하고 외부 HTTP route는 오래된 바이트를 주는가 | 각 선택 파일에 HTTP 200, byte 길이 및 served SHA-256 비교를 수행한다. |
| 20 | Agent Browser가 작업 뒤 계속 실행되거나 다른 세션까지 닫히는가 | 시작 전 session list를 확인하고 이번 실행의 named session만 종료한 뒤 목록과 process를 재확인한다. |

모든 판정은 최종 `README.md`, 선택 입력 manifest, preflight/copy/served 결과 및 Agent Browser 기록으로 대조한다. 이 문서는 웹 페이지 표시 확인이며 앱 runtime 재검증은 아니다.

## 실행 결과

- pass01: 파일 1,673개 중 157개 생성, 105개 교체, 1,411개 동일. 교체 backup 105/105 및 served hash 1,673/1,673 일치, 삭제 0개.
- pass02: 파일 1,685개 중 126개 생성, 116개 교체, 1,443개 동일. 교체 backup 116/116 및 served hash 1,685/1,685 일치, 삭제 0개.
- pass03: 파일 1,691개 중 120개 생성, 116개 교체, 1,455개 동일. 교체 backup 116/116 및 served hash 1,691/1,691 일치, 삭제 0개. 교체 전 열린 파일 0개.
- pass06: 파일 1,711개 중 120개 생성, 120개 교체, 1,471개 동일. 교체 backup 120/120 및 served hash 1,711/1,711 일치, 삭제 0개. 교체 대상 열린 계획 HTML과 pass04 JSON은 기록 후 atomic rename했다.
- Tailnet server PID 1191은 세 pass 모두 유지했다. 열려 있던 기존 파일은 same-directory atomic rename으로 교체했고 tmux 세션은 건드리지 않았다.
- Agent Browser pass01, pass04, pass05, pass06 session을 각각 닫은 뒤 session list가 비었고 agent-browser browser process도 없었다. 시뮬레이터 캡처는 다시 실행하지 않았다.
