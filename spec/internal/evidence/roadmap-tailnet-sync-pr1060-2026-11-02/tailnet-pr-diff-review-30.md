# PR #1060 후 Tailnet C11 로드맵 동기화 검토 30개

검토 대상은 선택 Rspress publication, `SpinonPreview` route 복사, backup·HTTPS hash evidence와 Agent Browser 확인이다. 전체 문서 사이트 build나 native 앱 적합성으로 범위를 넓히지 않고 각각의 실패 경로를 따로 대조했다.

| # | 독립 실패 관점 | 판정과 대조 |
|---:|---|---|
| 1 | 병합 전 PR의 내용을 source로 복사하는가 | 통과. source `HEAD`와 `origin/main`은 PR #1060 merge SHA `42cc95e32920e6a787b7ec46b9b1ff9cd230d603`다. |
| 2 | source에 tracked WIP가 있는 상태로 hash를 고정하는가 | 통과. build scripts가 tracked diff가 비어 있음을 확인한다. |
| 3 | Tailnet service가 실제로 종료됐는데 과거 기록만 믿는가 | 통과. 현재 Tailscale backend `Running`과 로컬 route HTTP 200을 확인했다. |
| 4 | 다른 service가 8768 포트를 차지하는가 | 통과. listener PID 28040의 명령은 `tailnet-preview-server.mjs`다. |
| 5 | HTTPS host가 다른 port나 다른 preview directory를 가리키는가 | 통과. `tailscale serve status`는 기본 host를 `127.0.0.1:8768`에 연결한다. |
| 6 | 동기화 전 페이지가 HTTP 오류인데 body hash만으로 진행하는가 | 통과. local과 HTTPS 모두 기존 상태에서 HTTP 200이었다. |
| 7 | 동기화가 필요 없는 최신 페이지를 오래된 것으로 분류하는가 | 통과. 이전 515,434-byte target에는 #1056/#1059 카드가 없고 source는 519,538 bytes다. |
| 8 | public roadmap 복사본 간 내용이 달라지는가 | 통과. `roadmap.html`, `public/roadmap.html`, `spec/public/roadmap.html` 세 경로에 동일 source bytes를 선택했다. |
| 9 | source나 destination symlink를 따라 엉뚱한 경로를 덮어쓰는가 | 통과. build·sync가 source와 target 경로의 symlink를 거부한다. |
| 10 | `..` 또는 절대 경로가 preview root를 벗어나는가 | 통과. route 검증은 절대 경로, backslash, dot segment를 거부한다. |
| 11 | 두 C11 카드가 roadmap에서 빠지는가 | 통과. build manifest는 #1054·#1056·#1059 카드의 local href를 모두 publication route에 대조한다. |
| 12 | card의 plan `.html` URL과 Markdown source 이름이 어긋나는가 | 통과. 생성 site에 fixed-px implicit-column plan route가 있다. |
| 13 | #1054 precomparison·review 링크가 빠지는가 | 통과. 해당 다섯 local href가 manifest에 있다. |
| 14 | #1056 runtime 또는 precomparison 링크가 빠지는가 | 통과. 카드 세 local href가 manifest에 있다. |
| 15 | #1059 contract, precomparison 또는 runtime 링크가 빠지는가 | 통과. 카드 네 local href가 manifest에 있다. |
| 16 | `STATUS.html`이 이전 Markdown에서 생성되는가 | 통과. source-head에 고정한 selected Rspress build가 현재 STATUS를 렌더링했다. |
| 17 | 생성 상태 페이지에서 공식 숫자가 달라지는가 | 통과. generated route와 Agent Browser에서 `72/213`을 확인했다. |
| 18 | C11 checkbox가 잘못 체크되는가 | 통과. source Markdown 검사와 Agent Browser DOM input 모두 unchecked다. |
| 19 | `STATUS.html`에 #1059 기록이 빠지는가 | 통과. Tailnet body text에서 PR #1059 merge note와 SHA를 확인했다. |
| 20 | 전체 docs build의 실패를 selected build 성공으로 바꿔 쓰는가 | 통과. README에서 전체 build와 bounded selected build를 분리한다. |
| 21 | 첫 실패를 지우거나 재사용된 경로를 덮어쓰는가 | 통과. favicon 누락 pass01 log와 SSR fail pass02 log를 각각 보존했다. |
| 22 | root에 앱 JS를 남겨 Rspress가 이를 route로 실행하는가 | 통과. pass03 root는 Markdown과 embedded image만 포함하고 33개 tracked 입력을 기록한다. |
| 23 | 지원되지 않는 Rspress 버전으로 다른 HTML을 생성하는가 | 통과. lock된 Rspress 2.0.22를 사용했다. |
| 24 | 선택 publication에 untracked evidence가 섞이는가 | 통과. 생성 목록은 tracked files와 selected Rspress output으로 제한한다. |
| 25 | 기존 destination 변경 전에 backup 경로가 덮이는가 | 통과. 고유 pass01 backup 경로가 없음을 preflight에서 확인한다. |
| 26 | backup을 만들지 않고 7개 기존 route를 바꾸는가 | 통과. 교체 전 route 7개 모두 backup hash 7/7을 확인했다. |
| 27 | 대상 파일이 열린 동안 atomic replacement를 수행하는가 | 통과. 교체 대상 open handle은 0이었다. |
| 28 | publication 크기나 TLS 응답이 잘못됐는데 성공으로 처리하는가 | 통과. local·HTTPS 모두 147/147 byte count 및 SHA-256이 일치하고 HTTPS 상태는 전부 200이다. |
| 29 | mobile page overflow를 놓치거나 SVG 내부 clip을 숨기는가 | 통과. document width 390px을 확인하고, overview SVG 오른쪽 일부 clip을 별도 한계로 남겼다. |
| 30 | 브라우저나 preview server를 불필요하게 재시작·방치하는가 | 통과. preview server는 재시작하지 않았고 Agent Browser는 최종 확인 뒤 `close --all`로 종료했다. |

## 판정

선택 publication은 Tailnet preview source의 일부 route만 갱신한다. 전체 문서 링크 검증, native app pixel equality, GitHub Pages 배포는 이번 근거에 포함하지 않는다.
