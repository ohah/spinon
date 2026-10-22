# C11.4 minmax calc Tailnet 동기화 PR diff 검토 30개 · pass46

대상은 pass46 동기화 증거 문서·manifest다. 이는 private preview 동기화의 기록 검토이며 앱 동작이나 공개 CSS 지원을 검증하지 않는다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 잘못된 source commit의 문서를 빌드할 수 있음 | source HEAD `48eaefe89d1eaad81cd355e961dadd56e0078c00`가 PR #488 merge commit과 같다. |
| 2 | checkout tree와 `origin/main`이 다를 수 있음 | source tree와 작업 tree가 모두 `475b0b7eb866769ff2a52324441fc7c98974dd29`다. |
| 3 | 이전 preview baseline이 어느 버전인지 모호할 수 있음 | pass45 source commit과 tree를 기록했다. |
| 4 | 시작 상태 검증이 잘못된 preview 결과를 덮을 수 있음 | pass45 선택 source와 현재 served HTML의 기존 hash를 대조했다. |
| 5 | untracked evidence를 현재 source처럼 섞을 수 있음 | selection은 tracked C11.4 paths로 제한했고 다른 untracked evidence는 제외했다. |
| 6 | 오래된 plan을 유지하거나 무관한 계획을 덮을 수 있음 | `git diff`로 바뀐 C11.4 plan만 선택 root에 반영했다. |
| 7 | 공식 status와 roadmap이 서로 다른 결과를 보일 수 있음 | 두 source를 같은 `origin/main` tree에서 복사했다. |
| 8 | public roadmap의 정적 복사본 경로가 누락될 수 있음 | `roadmap.html` 및 `public/roadmap.html` 소스를 선택 root에 반영했다. |
| 9 | status의 두 Rspress route가 한쪽만 갱신될 수 있음 | `STATUS.md`와 `spec/STATUS.md` route를 모두 포함했다. |
| 10 | screenshot 및 evidence route가 404가 될 수 있음 | precomparison/runtime evidence tracked directory 전체를 선택 route alias로 복사했다. |
| 11 | 문서의 이미지가 선택 build에 빠질 수 있음 | Android/iOS small-limit PNG route가 summary의 required routes에 있다. |
| 12 | 새 문서 review 링크가 빌드에 빠질 수 있음 | `roadmap-pr-diff-review-30.html` route를 required list에 넣었다. |
| 13 | 선택 root의 실패 메시지가 실제 이전 root와 다를 수 있음 | review에서 stale `pass42` diagnostic을 찾아 선택 source를 재생성하는 helper가 `pass45`를 가리키도록 고쳤다. |
| 14 | 심볼릭 링크가 preview 밖 파일을 가리킬 수 있음 | 선택 build walk가 symlink를 거부하고 summary는 `noSymlinks=true`다. |
| 15 | Rspress 버전 drift가 출력물을 바꿀 수 있음 | 저장소 규칙의 Rspress `2.0.22`로 고정 build했다. |
| 16 | 선택 build를 전체 문서 검사로 오해할 수 있음 | README에서 전체 build/dead-link/image 검사를 실행하지 않았다고 제한했다. |
| 17 | output size/hash가 재현 불가능할 수 있음 | 3,455 files, 157,300,840 bytes와 site tree SHA-256을 기록했다. |
| 18 | preview root가 엉뚱한 디렉터리일 수 있음 | target은 사전 확인한 `~/Library/Application Support/SpinonPreview`다. |
| 19 | 다른 server를 종료하거나 바꿀 수 있음 | 기존 PID 1191과 `127.0.0.1:8768` listener 하나를 확인하고 보존했다. |
| 20 | Tailnet Serve route가 다른 backend로 향할 수 있음 | `tailscale serve status`에서 Funnel `/`가 `127.0.0.1:8768`을 가리켰다. |
| 21 | tmux 작업 세션을 cleanup 중 종료할 수 있음 | sync 중 tmux 명령을 실행하지 않았고 세션을 유지했다. |
| 22 | 동기화가 기존 preview 파일을 삭제할 수 있음 | preflight·copy 모두 `no_deletes=true`이고 삭제 작업은 없다. |
| 23 | create/replace/same 수가 전체 수와 다를 수 있음 | `441 + 193 + 2,821 = 3,455`로 일치한다. |
| 24 | 이전 preview bytes를 복구할 수 없을 수 있음 | replace 193개, 28,393,658 bytes를 별도 backup root에 기록했다. |
| 25 | backup이 이미 손상되었거나 다른 파일일 수 있음 | copy/backup 및 local verification에서 backup 193/193 hash를 확인했다. |
| 26 | 열려 있는 대상 파일을 덮어써 reader가 깨질 수 있음 | 각 교체 전 `lsof`; 열린 5개는 idle listener만 보유해 backup 후 atomic rename했다. |
| 27 | local 재검증이 동시 Tailnet client 때문에 중단될 수 있음 | server-aware verifier의 active socket guard를 보존하고 file-only verifier로 target 3,455/3,455·backup 193/193을 다시 확인했다. |
| 28 | 원격 endpoint가 성공했지만 오래된 bytes를 줄 수 있음 | Tailnet 3,455 GET이 모두 HTTP 200이고 기대 SHA-256과 일치한다. |
| 29 | 완료 상태가 부분 fixture로 승격될 수 있음 | Browser에서 C11.4 checkbox unchecked와 `72/213`을 확인했다. |
| 30 | simulator 결과가 실기기·compositor·border 검증으로 확대될 수 있음 | Android AVD, iOS Simulator·제출 전 Metal 표본, Ahem text paint와 남은 한계를 구분했다. |

## 판정

문서·선택 build·preview copy의 차단 결함은 발견되지 않았다. 이 검토는 full docs build, WPT, Android 실기기, iOS compositor 이후 pixel 또는 전체 C11.4 검증을 뜻하지 않는다.
