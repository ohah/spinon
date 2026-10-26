# C11.4 fit-content 옆 auto 행 런타임 사전 비교

## 목적

PR #662는 `grid-auto-rows: fit-content(40px) auto`의 독립 기대값과 Rust layout-stage assertion을 추가했다. 이 자료는 같은 CSS 동작을 opt-in Android/iOS 앱 fixture 경로에 연결할 후속 계획의 oracle을 고정한다. 여기에는 Spinon runtime 실행 결과가 없다.

## 고정 입력과 출처

- WPT checkout: `build/wpt-c124-source`, revision `c999c58338ee1d223df5ad62e48be1202ced0d37`.
- WPT 파일 `css/css-grid/grid-definition/fit-content-track-sizing.html`: SHA-256 `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736`.
- [기본 grid 크기 CSS (11–15행)](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html#L11-L15) · [대상 DOM 두 item (72–75행)](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html#L72-L75) · [대상 assertion (131–132행)](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html#L131-L132).
- 대상 assertion의 컨테이너 높이는 300px, implicit row list는 `fit-content(40px) auto`, 두 row의 nested content 높이는 각각 10px이다.
- 규범적 기대값은 CSS Grid 2025 CRD §6.6, §11.5, §11.8, §12.2, §12.4–§12.8에서 독립 계산했다. 첫 fit-content row는 content의 10px max-content contribution으로 유지되고, `align-content: normal`은 남은 공간을 auto row에 분배해 rows `[10px, 290px]`를 만든다.

## Chrome 관찰

Agent Browser 0.31.1로 `https://wpt.live/css/css-grid/grid-definition/fit-content-track-sizing.html`을 열어 upstream WPT testharness의 13개 assertion이 모두 Pass인 것을 확인했다. 대상 root의 computed `grid-template-rows`는 `10px 290px`, height는 `300px`이었다. Agent Browser가 browser fetch로 얻은 live source SHA-256은 고정 checkout의 SHA-256과 일치했다.

- [전체 WPT 결과](chrome-wpt-live-fit-content-auto-neighbor.png)
- [대상 assertion과 결과](chrome-wpt-live-fit-content-auto-neighbor-target.png)
- [원시 Agent Browser 관찰](chrome-wpt-live-observation.raw.json)
- [고정 출처](source-pins.json) · [독립 기대값](expected.json)

Chrome은 보조 관찰이며 기대값을 Chrome 출력에서 역으로 만들지 않았다. upstream WPT testharness를 Chrome에서 실행한 기록은 Spinon WPT runner/conformance 결과가 아니다.

## 구현 후 판정

후속 구현은 같은 two-row input을 기존 C11.4 opt-in CSS → HostDocument/Stylo → typed grid input → Taffy → native app path에 연결한다. computed style의 `grid-auto-rows`/`align-content`와 typed input이 예상값인지 먼저 확인하고, LayoutOutput의 pre-render CSS-px frame 및 layout trace의 두 used rows가 정확히 `[10, 290]px`인지, 같은 revision의 root/item frame·trace·draw/present가 있는지 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 확인한다. 계획 문서는 성공 기준만 고정하며 actual runtime pass를 주장하지 않는다.

이 제한 fixture는 general CSS author admission, 전체 WPT, 실기기·hardware GPU, iOS compositor 이후 pixels, C11.4/C11 완료를 주장하지 않는다.
