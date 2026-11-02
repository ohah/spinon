# C11.4 `full-size-kana` Tailnet 로드맵 발행 검토 30개

검토 대상은 PR #1084의 로드맵 HTML 변경과 해당 미리보기 동기화·브라우저 근거다. 공식 완료 수와 PR #1082의 좁은 구현 경계를 유지하는지, 실제 Tailscale 제공 파일과 링크가 일치하는지 확인했다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 동기화 대상이 현재 preview root인지 | `~/Library/Application Support/SpinonPreview`를 확인하고 해당 경로에 반영했다. |
| 2 | 다른 미리보기 프로세스를 바꾸거나 재시작하는가 | 포트 `8768`의 기존 PID `28040`과 서버 명령을 확인했고, 그 프로세스를 그대로 유지했다. |
| 3 | Tailscale 연결이 끊겼는데 로컬 파일만 수정하는가 | 동기화 전 Tailscale backend가 `Running`이고 실제 URL이 HTTP 200임을 확인했다. |
| 4 | 새 로드맵 파일이 PR branch의 리뷰된 source와 다른가 | `spec/public/roadmap.html` source SHA-256을 기록하고 제공 바이트와 대조했다. |
| 5 | 이전 파일을 백업 없이 덮어쓰는가 | 서로 다른 기존 파일 3개를 고유 backup root 아래 저장하고 각 이전 SHA-256을 확인했다. |
| 6 | backup 경로 충돌로 이전 증거를 덮어쓰는가 | 동기화 전 backup root가 없는지 확인한 뒤 생성했다. |
| 7 | 미리보기에서 기존 자료를 대량 삭제하는가 | 삭제하지 않았다. 바뀐 파일은 원자적으로 교체하고 새 파일과 해시 자산만 추가했다. |
| 8 | symlink가 preview root 밖 파일로 이어지는가 | root와 기존 대상 경로 구성 요소에 symlink가 없는지 확인하고 일반 파일만 기록했다. |
| 9 | 요청 경로에 `..` 또는 절대 경로가 섞이는가 | 모든 동기화 route를 상대 경로로 제한하고 상위 경로 요소를 거부했다. |
| 10 | 부분 복사 파일이 서버에 보이는가 | 같은 디렉터리 임시 파일을 쓴 뒤 rename하고, 기록 뒤 각 target hash를 다시 읽어 확인했다. |
| 11 | PR #1084와 다른 source revision이 발행되는가 | 보고서의 source HEAD는 PR branch `c82639bd1`이며 카드에는 이미 병합된 PR #1082의 SHA를 적었다. |
| 12 | roadmap link가 raw Markdown 또는 없는 route를 가리키는가 | 계획·사전 비교·계약·runtime·검토·상태 문서를 Rspress route로 렌더링했다. |
| 13 | 필요한 페이지 중 일부가 빠져 link가 404가 되는가 | standalone roadmap과 9개 문서 route를 포함해 33개 페이지·자산 대상을 구성했다. |
| 14 | 잘못된 Rspress 버전으로 preview를 만드는가 | 고정된 `@rspress/core 2.0.22`로 선택 route 빌드를 완료했다. |
| 15 | 전체 문서 빌드 실패를 숨기거나 성공으로 보고하는가 | 전체 `spec/` 빌드의 dead-link 실패를 별도로 기록하고 CI 성공으로 세지 않았다. |
| 16 | 필요한 부분 빌드도 실패했는데 반영하는가 | 선택 route 빌드가 성공한 뒤 산출된 파일만 대상으로 삼았다. |
| 17 | 선택 root 밖의 사이트 파일을 교체하는가 | 명시한 C11.4 계획·근거·계약·상태 route와 정적 자산만 복사했다. |
| 18 | 렌더된 문서의 CSS·JS·이미지가 빠지는가 | 선택 빌드의 content-addressed `static/` 자산을 함께 배포했다. |
| 19 | 링크된 README가 `README.html` 이외의 잘못된 경로로 출력되는가 | 카드가 참조하는 각 `.html` route의 실제 build output 존재를 확인했다. |
| 20 | 서버 응답이 404/5xx인데 파일 복사만 보고 완료라 하는가 | 33개 대상 모두 Tailscale URL에서 HTTP 200을 반환했다. |
| 21 | HTTP 200이지만 오래된 응답 본문을 제공하는가 | 33개 응답 본문 모두 manifest SHA-256과 일치했다. |
| 22 | PR #1082의 merge time/SHA를 잘못 쓴다 | 카드의 PR 링크·병합 시각·merge SHA가 GitHub의 merged PR 기록과 맞는다. |
| 23 | 좁은 fixture 행렬을 CSS 지원이나 시각적 parity로 부풀린다 | 10/10은 Android AVD와 iOS Simulator별 layout/intrinsic trace 결과로 분리했고, 제한·미검증 범위를 함께 적었다. |
| 24 | 이번 카드 때문에 공식 완료 수가 바뀌어 보이는가 | 카드와 공식 상태 페이지 모두 `72/213` 유지 및 C11 미완료를 표시한다. |
| 25 | 브라우저에서 제공된 plan route가 빈 페이지인가 | Agent Browser에서 계획 제목·H1과 문서 내용을 확인했고 이미지 1개가 로드되고 깨진 이미지는 0개였다. |
| 26 | 새 카드가 Tailnet 화면에서 누락되는가 | Agent Browser DOM에서 PR #1082 카드, 문구와 10개 링크를 확인했다. |
| 27 | desktop capture가 실제 화면이 아닌 HTML source나 다른 route인가 | Agent Browser로 Tailnet roadmap을 연 상태에서 2560×1400 캡처를 저장했다. |
| 28 | mobile 폭에서 카드나 페이지가 가로로 잘리는가 | 390 CSS px viewport에서 document `scrollWidth=clientWidth=390`, 카드 `scrollWidth=clientWidth=362`를 확인했다. |
| 29 | PR 화면 증거가 잘못된 캡처거나 손상된 PNG인가 | desktop/mobile PNG signature·dimensions·SHA-256을 브라우저 검증 JSON에 기록했다. |
| 30 | Agent Browser나 미리보기 서버를 정리하며 사용자의 서버를 종료하는가 | 사용한 Agent Browser session을 닫고 해당 프로세스가 남지 않은 것을 확인했다. Tailscale preview 서버는 계속 실행 중이다. |

## 판정

선택 route build, 33개 Tailnet 응답의 상태·해시, 계획 페이지의 실제 렌더링, desktop/mobile roadmap 화면을 확인했다. 모바일 문서 폭에서 가로 overflow는 관찰되지 않았다. 전체 `spec/` build의 기존 dead-link 실패는 별도로 남기며, 이것을 전체 docs build 성공으로 표현하지 않는다.
