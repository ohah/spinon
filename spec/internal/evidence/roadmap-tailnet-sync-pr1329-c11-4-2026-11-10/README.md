# PR #1329 계획 병합 후 Tailnet roadmap 동기화 · 2026-11-10

## 기준과 공개 roadmap 수정

- 계획 PR [#1329](https://github.com/ohah/spinon/pull/1329)은 `2026-11-10T07:03:42Z` Rebase and merge됐고 merge SHA는 `e3a1b09e2b21ae3f13c8f4cb884c112f888f1c94`다.
- 공개 `spec/public/roadmap.html`에는 #1329 계획 상태, 다음 구현 대상, 4,320-scene Chrome 사전 비교 링크를 추가했다. 문서 후보 commit은 `0ff0f9d2b8102144d54cd76df086914a75082341`이다.
- 구현 전 기준이므로 C11.4/C11 checkbox는 미완료, 공식 완료 수는 `78/214`, 내부 계약은 `0.1.0`이다. 계획 PR의 GitHub check rollup은 0개다.

## 빌드·선택 경로

- RSPress `2.0.22`로 선택 문서 source 146개(70,082,461 bytes)를 302개 route(87,433,257 bytes)로 빌드했다. 구성은 [선택 root manifest](./selected-root.json), [publication manifest](./selected-publication.json), [빌드 설정](./rspress-pass01.config.ts), [RSPress 로그](./selected-rspress-build-pass01.log)에 기록했다.
- 선택 빌드는 저장소 전체 문서의 dead-link 검사가 아니다. 선택한 문서 집합은 저장소 전체 링크 대상을 포함하지 않아 RSPress `checkDeadLinks`를 끄고 빌드했다. 첫 기본 설정 실행에서 이를 확인한 뒤, 불필요한 전체 실패 로그는 정리했다.
- 로드맵의 두 내부 경로(`/spinon/plan/...` 및 `/spinon/internal/evidence/.../README.html`)와 pinned Chrome 157 screenshot URL을 독립적으로 확인했다. route staging 검증은 [publication manifest](./selected-publication.json)에 있다.

## Tailnet 동기화

- Tailscale backend는 `Running`; 기존 preview server PID `90711`이 loopback `127.0.0.1:8768`에서 응답했다. 서버를 재시작하지 않았다.
- Preflight에서 새 route 107개, 교체 69개, 동일 126개를 확인했다. 변경 대상 open handle은 0개였고 route 삭제는 없었다. 교체 전 69개 파일은 `~/Library/Application Support/SpinonPreviewBackups/roadmap-tailnet-sync-pr1329-c11-4-2026-11-10/pass01`에 백업했다.
- 동기화 뒤 local preview와 HTTPS에서 선택한 302/302 route가 모두 HTTP 200 및 expected SHA-256과 일치했다. 교체 전 rollback backup도 69/69 hash가 일치했다.
- 자세한 결과: [preflight](./preflight.json), [copy 및 backup](./copy-and-backup.json), [served hash 검증](./served-verification.json).

## Agent Browser 화면과 프로세스 정리

- [공개 roadmap](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html)을 Agent Browser `0.31.1`로 확인했다. Desktop `1440×1000`과 mobile `390×844`에서 #1329, merge SHA, `78/214`를 읽었다.
- Mobile document width는 viewport와 같은 `390px`이다. 칸반은 내부 가로 스크롤(`1120px` content / `362px` viewport)로 작동해 문서 전체의 가로 넘침이 없다. 화면: [desktop](./tailnet-roadmap-pass01-desktop-agent-browser.png), [mobile](./tailnet-roadmap-pass01-mobile-agent-browser.png), [#1329 card desktop](./tailnet-roadmap-plan1329-agent-browser.png), [#1329 card mobile](./tailnet-roadmap-plan1329-mobile-agent-browser.png).
- 계획에 연결된 commit-pinned Chrome 157 raw image는 Agent Browser에서 `1280×900`으로 완전히 로드됐다: [화면](./chrome157-precomparison-raw-image-agent-browser.png).
- 이 작업에서 만든 session `spinon-roadmap-pass01-20261110`은 닫았고, session config와 temporary Chrome profile을 제거했다. 종료 후 JSON session 목록에는 기존 `zntc4603-root`만 남았고 작업 세션의 daemon/browser process는 없었다. lifecycle 증거는 [여기](./agent-browser-lifecycle.json), 화면 검사값은 [여기](./agent-browser-verification.json)에 둔다.

## 범위

이 pass는 plan PR 병합 후 공개 roadmap과 Tailnet preview를 동기화한 기록이다. 아직 계획에 적힌 4,320-scene Spinon RuntimeSession 구현 검증은 하지 않았다. 전체 문서 dead-link scan, 실기기, simulator surface pixel readback, WPT 적합성, C11.4/C11 완료는 주장하지 않는다.


이번 문서 작업에서는 iOS Simulator나 Android AVD를 시작하거나 새 profile을 만들지 않았다. `xcrun simctl` 목록에서 S03 이름의 iOS profile은 없었고 기존 iOS profiles는 모두 `Shutdown`이었다. 공유·타 프로젝트 profile은 삭제하지 않았다. 사이트 확인에는 Agent Browser만 사용했고 Terminal Browser는 사용하지 않았다.
