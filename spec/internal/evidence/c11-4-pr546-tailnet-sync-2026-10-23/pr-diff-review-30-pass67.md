# Tailnet roadmap pass67 PR diff 검토 30개

이 PR은 병합된 PR #546의 정적 `roadmap.html` 반영 근거와 검증 도구를 추가한다. 다른 route·제품 코드·공식 완료 상태를 바꾸지 않는다.

| # | 실패 관점 | 검토 결과 |
| --- | --- | --- |
| 1 | 잘못된 main revision을 source로 삼는가 | script는 PR #546 merge SHA `849f04fbffc6f9efb85f7269791c7b6c51768724`를 요구한다. |
| 2 | local checkout과 origin/main이 다를 때 계속하는가 | 두 revision을 고정 SHA와 대조하고 불일치 시 중단한다. |
| 3 | working file이 main의 tracked blob과 다른가 | `HEAD:path`와 `origin/main:path` blob ID를 비교한다. |
| 4 | 잘못된 파일을 Tailnet에 복사하는가 | source와 target은 `spec/public/roadmap.html` 및 `SpinonPreview/roadmap.html` 하나로 제한한다. |
| 5 | 기존 target이 수동 변경됐는데 덮어쓰는가 | pass66 manifest의 target hash와 같지 않으면 preflight를 거부한다. |
| 6 | 이미 새 내용인데 재실행해 중복 backup을 만드는가 | source와 이전 target hash가 같으면 거부한다. |
| 7 | 기존 백업 경로를 덮어쓰는가 | 고유 pass67 경로가 이미 존재하면 중단한다. |
| 8 | source나 target이 symlink인 경우 예상 밖 파일을 읽는가 | 두 경로에 일반 파일·비 symlink 조건을 적용한다. |
| 9 | main 내용에 필요한 병합 정보가 빠졌는가 | PR #545 merge 표기와 full SHA를 확인한다. |
| 10 | 공식 완료 수를 잘못 표시하는가 | source에 `77/213` 존재를 확인하고 문단은 C11.4 미완료로 쓴다. |
| 11 | 새 로드맵 항목이 없는 source를 배포하는가 | C11.4 양축 auto-repeat runtime 항목을 사전 확인한다. |
| 12 | Tailnet status 링크 fragment가 사라졌는가 | 기존 `#css-구현-체크리스트` href를 사전 검사한다. |
| 13 | 원본 파일을 바꾸기 전에 복구점을 만들지 않는가 | 같은 볼륨 hardlink backup을 target 교체보다 먼저 만든다. |
| 14 | backup bytes가 기존 target과 다른가 | backup SHA를 이전 target SHA와 즉시 비교한다. |
| 15 | 같은 볼륨이 아니어서 hardlink가 실패할 때 부분 복사를 시도하는가 | hardlink 오류를 그대로 반환하고 copy fallback을 두지 않는다. |
| 16 | preview server가 부분 파일을 읽는가 | source hardlink 임시 파일을 만든 뒤 atomic rename한다. |
| 17 | 교체 실패 시 임시 파일이 남는가 | rename 예외에서 해당 임시 파일만 제거한다. |
| 18 | 관련 없는 파일을 삭제하는가 | 어떠한 delete도 하지 않으며 manifest에도 기록한다. |
| 19 | local target byte 수가 source와 다른가 | verify 단계에서 두 byte 수를 비교한다. |
| 20 | local target hash가 source와 다른가 | verify 단계에서 SHA-256을 비교한다. |
| 21 | backup이 교체 후에도 이전 target인가 | verify 단계에서 backup SHA가 preflight의 이전 SHA와 같은지 확인한다. |
| 22 | HTTPS endpoint가 다르거나 오류 응답을 성공으로 보는가 | 고정 Tailnet URL과 HTTP 200을 요구한다. |
| 23 | remote cache가 오래된 내용을 반환하는가 | 원격 fetch에 `cache: no-store`를 주고 실제 body를 hash한다. |
| 24 | remote body가 main source와 다른데 성공 처리하는가 | remote bytes와 SHA-256을 preflight source 값과 비교한다. |
| 25 | Agent Browser가 구버전 roadmap을 표시하는가 | 제목·PR #545 SHA·34/34·77/213을 화면 DOM에서 확인했다. |
| 26 | overflow geometry 또는 track 값이 누락되는가 | 새 카드의 root/item geometry와 columns/rows 값을 rendered text에서 확인했다. |
| 27 | 체크 범위가 full CSS Grid 지원으로 과장되는가 | 카드와 README 모두 전체 C11.4 미완료 및 미검증 범위를 유지한다. |
| 28 | status 링크가 잘못된 fragment로 이동하는가 | 링크를 클릭해 decoded target이 있고 H2 제목이 “CSS 구현 체크리스트”임을 확인했다. STATUS route 본문 갱신은 이 pass의 범위가 아니다. |
| 29 | browser screenshot이 다른 세션 또는 페이지를 기록하거나 PR 본문에서 깨지는가 | named session `spinon-tailnet-pass67`에서 로드맵과 anchor 화면을 각각 캡처했다. GitHub PR #547 본문 이미지도 Agent Browser에서 `complete: true`, 1440×1200으로 로드됐다. |
| 30 | 이 변경이 무관한 서비스나 tmux를 종료하는가 | PID 1191과 tmux 세션을 유지하고 본 작업의 Agent Browser session만 닫았다. |

## 결과

30개 관점에서 경로, hash, 백업, 원자 교체, 브라우저 화면과 검증 경계를 확인했다. Tailnet에서 갱신한 것은 `roadmap.html` 하나다. `STATUS.html`은 기존 정적 결과가 남아 있을 수 있으므로 최신화로 주장하지 않는다.
