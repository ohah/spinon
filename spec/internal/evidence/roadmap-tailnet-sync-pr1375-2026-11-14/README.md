# PR #1375 병합 뒤 상태·로드맵·Tailnet 동기화

## 범위

PR [#1375](https://github.com/ohah/spinon/pull/1375) 구현 병합 기록을 `spec/STATUS.md`, C11.4 계획, 내부 계약 설명과 공개 로드맵에 반영한다. C11.4/C11은 미완료이고 공식 집계는 `78/214`로 유지한다. 다음 C11.4 bounded slice는 아직 선택하지 않았다.

Tailnet 미리보기는 선택한 문서 route만 빌드하고 기존 `/spinon/` 파일을 안전하게 갱신한다. 이미 실행 중인 Tailscale Serve와 `tailnet-preview-server.mjs`는 재시작하지 않는다. GitHub Pages 배포나 전체 문서 배포는 수행하지 않는다.

## 기기·프로세스 정리

검증 직전 `xcrun simctl list devices booted`에는 부팅된 iOS 기기가 없었다. 등록된 iPhone·iPad 기기는 모두 `Shutdown`이고 S03 이름/UDID는 현재 목록에 없었다. `Simulator.app`도 실행 중이지 않았다. 남아 있던 CoreSimulator XPC 서비스는 Apple 백그라운드 서비스라 시뮬레이터 기기나 UI 실행으로 취급하지 않았다. 이번 문서 동기화에는 Simulator를 실행하지 않았다. 이전 빈 Agent Browser 세션과 이번에 만든 세션은 모두 닫았고 최종 세션 목록은 비어 있다. 자세한 기기 목록은 [simulator-cleanup.json](./simulator-cleanup.json)에 있다.

## 검증 결과

첫 Tailnet 동기화는 선택 route 130개를 반영했다. 기존 route 15개를 별도 backup한 뒤 교체했고, 97개를 생성했으며 18개는 같은 hash라 그대로 두었다. 로컬 HTTP와 공개 HTTPS에서 130/130 route의 HTTP 200 및 SHA-256을 확인했다. preview server PID `90711`은 그대로였고 재시작이나 삭제는 없었다. [1차 manifest](./selected-publication.json), [사전 점검](./preflight.json), [복사·backup](./copy-and-backup.json), [제공본 검증](./served-verification.json)을 확인한다. 이후 이 README와 30개 항목 결과, 화면 증거를 포함해 선택 route를 다시 동기화했다. 재실행 manifest는 evidence 디렉터리에 `*-final.json` 이름으로 보존한다.

Agent Browser의 공개 roadmap 캡처는 [desktop 1440×900](./tailnet-roadmap-desktop-agent-browser.png), [mobile 390×844](./tailnet-roadmap-mobile-agent-browser.png), [mobile 칸반 오른쪽 끝](./tailnet-roadmap-mobile-kanban-scroll-agent-browser.png)이다. PR #1375·merge SHA·`78/214`·미완료 상태와 #1372 이력을 확인했다. mobile 문서 너비는 viewport와 같은 390px이고 칸반은 `overflow-x:auto`로 362px viewport 안에서 끝까지 스크롤됐다. [Agent Browser 판정·image hash](./agent-browser-verification.json)에 캡처와 종료 상태를 기록했다.

문서 diff의 서로 다른 30개 실패 관점을 검토했고 수정 후 모두 통과했다. 이 근거는 전체 사이트 링크 검증, mixed Grid mobile runtime/pixel parity 또는 C11.4 구현 완료를 주장하지 않는다.
