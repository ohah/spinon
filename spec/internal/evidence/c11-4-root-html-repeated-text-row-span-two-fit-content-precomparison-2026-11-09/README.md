# C11.4 반복 텍스트 row-span 두 `fit-content()` 사전 비교

## 범위

Chrome 157 Agent Browser에서 여섯 member `grid-auto-rows` 목록에 `fit-content()` 두 개를 놓는 제한 fixture 3,840 scene을 수집했다. 이 결과는 CSS Grid 입력과 관계의 사전 비교다. Spinon layout, RuntimeSession, Android/iOS 앱 동작을 검증하지 않는다.

## 실행 및 입력

- Browser: `/opt/homebrew/bin/agent-browser`, `HeadlessChrome/157.0.0.0`, DPR 1, viewport `1280×900`
- Collector: [`capture-chrome.mjs`](./capture-chrome.mjs)
- Fixture: [`root-html-grid-repeated-text-row-span-two-fit-content-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-two-fit-content-precomparison.html)
- Fixture SHA-256은 `verification.json`의 `fixture.sha256` 항목에 있다.
- Collector SHA-256은 `verification.json`의 `collector.sha256` 항목에 있다.
- 구조화된 결과: [`verification.json`](./verification.json)
- 계획 PR 변경 diff의 독립 검토 30개: [`plan-pr-diff-review-30.md`](./plan-pr-diff-review-30.md)
- 전체 장면 기록: [`chrome-observation.ndjson`](./chrome-observation.ndjson)
- 표본 화면: [Chrome 157 matrix sample](./chrome157-dual-fit-content-sample.png)

장면 수는 `15개 fit 위치 pair × 8개 순서가 있는 cap pair 할당 × 4개 고정 tail 순서 × 2개 alignment × 4개 source-order/topology variant = 3,840`이다. 고유한 six-member authored track 목록은 480개다. Representative cap pair는 `(0px, 0%)`, `(10px, 20px)`, `(10px, 20%)`, `(20%, 120%)`이며, definite 220px block basis에서 `20%`는 44px, `120%`는 264px다. 이 cap pair는 지원값 allowlist가 아니라 검증 표본이다. 네 tail order만 비교하며 모든 순열을 포함하지 않는다.

## 결과

- 3,840/3,840 scene에 explicit row 하나와 implicit row 여섯 개가 있었고, 모든 used row 크기는 유한한 0 이상 값이었다. implicit row 최소 크기는 0px이다.
- 두 `fit-content()`를 포함한 authored `grid-auto-rows` 여섯 member와 순서가 computed CSSOM에 보존됐다.
- 각 span frame은 점유한 used row 크기의 합과 일치했다. 최대 관측 오차는 `0.00025 CSS px`다.
- 정방향/역방향 source order 1,920쌍의 semantic frame 최대 차이는 `0 CSS px`다.
- `start`/`stretch` 1,920쌍에서 `auto` track만 늘었고 non-auto track의 최대 변화는 `0 CSS px`다. 각 목록에는 auto track이 하나이므로 여러 auto track 사이의 균등 분배는 이 비교 범위가 아니다.
- Cap sensitivity는 480 group 중 300개에서 나타났다. 전체 used-track range는 최대 60px이었다. Fit track observed range는 position 1–2에서 0–44px, position 3–5에서 0–30px, position 6에서 0–20px였다. 단일 auto track의 최대 stretch 증분은 130px이다. 이는 검증 표본 결과이며 cap 공식 전체의 증명으로 쓰지 않는다.

Cap 관측은 선택한 행렬이 서로 다른 cap 결과를 실제로 구분함을 보여준다. 모든 경우의 cap 상한을 보장하지는 않는다. `fit-content()`의 min-content floor 때문에 used size가 cap보다 클 수 있다.

## 브라우저 정리

Collector가 [`agent-browser-lifecycle.json`](./agent-browser-lifecycle.json)에 `closed: true`를 기록했다. 이후 명시적으로 `close --all`을 실행하고 session 목록이 비었음을 확인했다. Terminal Browser는 사용하지 않았다. Chrome 사전 비교에는 iOS/Android Simulator를 시작하지 않았다.
