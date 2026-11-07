# Tailnet 동기화 PR 변경분 실패 관점 검토 30개

검토 대상은 PR #1248의 Tailnet 동기화 기록, 실행 스크립트, hash·Agent Browser records와 C11.4 상태 링크다. 각 관점에서 실패 경계와 보고 문구를 별도로 확인했다. Blocking finding은 없다.

| # | 별도 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 미리보기 서버가 종료됐는데 실행 중으로 기록 | `tailscale status --json`의 `Running`과 listener를 별도로 확인했다. |
| 2 | 8768 포트를 다른 서버가 사용 | listener PID `28040`의 command가 `tailnet-preview-server.mjs`다. |
| 3 | 현재 source와 preview target hash가 같다고 잘못 보고 | preflight source와 target SHA가 서로 다름을 기록했다. |
| 4 | source는 최신인데 Tailnet HTTPS가 다른 응답을 반환 | 변경 전 두 URL 모두 이전 제공본 SHA를 반환했다. |
| 5 | preview target이 일반 파일이 아닌데 교체 | `stat`으로 regular file을 확인했다. |
| 6 | server나 다른 프로세스가 target 파일을 열어둔 상태에서 교체 | `lsof` open handles가 빈 배열이었다. |
| 7 | 기존 preview 파일 사본을 남기지 않음 | old bytes를 고유 backup 경로에 복사하고 SHA를 확인했다. |
| 8 | 재실행이 기존 backup을 덮어씀 | 고유 backup 경로가 사전에 없음을 검사하고 `COPYFILE_EXCL`을 썼다. |
| 9 | backup의 bytes/hash가 교체 전 target과 달라짐 | 613,422 bytes, SHA `004f4995…`로 전 target과 일치한다. |
| 10 | copy 중 읽히는 부분 파일을 target에 노출 | 동일 디렉터리 temp write 후 fsync, atomic rename을 사용했다. |
| 11 | 동기화한 source가 PR #1247 merge 뒤 main에서 오지 않음 | local main을 fast-forward한 뒤 branch를 만들고 source SHA `504951ff…`를 기록했다. |
| 12 | PR #1246 또는 merge SHA를 잘못 표시 | `ea86bd1ceea03df21ed1a5ba89b8b2aaad965948` marker를 확인했다. |
| 13 | 공식 완료 수가 다른 값으로 표시 | 제공본에 `78/214`가 있고 local source와 같은 bytes다. |
| 14 | C11/C11.4 parent를 완료 표시 | latest card와 STATUS 모두 미완료로 유지한다. |
| 15 | 내부 계약 버전 변경 누락 또는 임의 상승 | `0.1.0` 유지가 source 및 served SHA로 보장된다. |
| 16 | 다음 safe/unsafe 후보를 구현 완료로 소개 | 링크된 계획과 roadmap은 계획 검토 완료, 구현 전으로 분명히 표시한다. |
| 17 | 최신 roadmap card가 오래된 순서로 노출 | desktop/mobile 모두 `#1246 → #1243 → #1240 → #1237`을 관측했다. |
| 18 | desktop viewport와 기록 수치 불일치 | Agent Browser viewport 1440×1000 DPR 1, DOM width 1440이다. |
| 19 | desktop 수평 overflow가 카드 밖에서만 측정됨 | document/body scrollWidth 모두 1440, viewport와 일치한다. |
| 20 | desktop 최신 카드 경계가 화면 밖으로 나감 | x=90, right=1018로 1440px viewport 안이다. |
| 21 | mobile viewport와 기록 수치 불일치 | Agent Browser viewport 390×844 DPR 1, DOM width 390이다. |
| 22 | mobile 수평 overflow가 body에서 누락됨 | document/body scrollWidth 모두 390, viewport와 일치한다. |
| 23 | mobile 카드가 오른쪽 경계를 넘음 | x=14, right=376으로 390px viewport 안이다. |
| 24 | candidate plan link가 public page에서 빠짐 | `a[href="/spinon/plan/c11-root-html-auto-row-safe-center.html"]` 존재를 두 viewport에서 확인했다. |
| 25 | screenshot 저장 크기/뷰포트가 일치하지 않음 | desktop 1440×1000, mobile 390×844 PNG metadata를 확인했다. |
| 26 | screenshot SHA가 다른 파일을 가리킴 | DOM records의 desktop/mobile SHA와 저장 파일 hash를 대조했다. |
| 27 | browser lifecycle을 누락 | 두 Agent Browser session close exit 0, 이후 browser/Chrome PID 없음으로 기록했다. |
| 28 | 금지된 Terminal Browser로 화면 확인 | Terminal Browser를 실행하지 않았다. |
| 29 | Tailscale preview를 위해 live server 재시작 | PID `28040`을 그대로 뒀고 `serverRestarted=false`다. |
| 30 | 미리보기 갱신을 GitHub Pages 배포로 확대 | Pages 및 `gh-pages`를 변경하지 않았다고 기록했다. |

## 판정

source, backup, local HTTP, Tailnet HTTPS, Agent Browser desktop/mobile의 증거 경계가 구분돼 있다. 사용자 데이터·기존 제공 bytes를 보존하고 server lifecycle을 유지했다. PR #1248 변경분에 blocking finding이 없다.
