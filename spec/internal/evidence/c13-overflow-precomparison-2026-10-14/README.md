# C13 overflow·클리핑 Chrome 사전 관찰 · 2026-10-14

## 환경과 입력

- agent-browser `0.31.1`의 Headless Chrome `155.0.8059.40`, revision `cfaadc5a132d78e1828635aa8405a499f3e14864-refs/branch-heads/8059@{#936}`, macOS `26.5.1 (25F80)`.
- Chrome 실행 파일 SHA-256 `428dccc1a45819d23c5db4fb37388883f8d9ba64611cbd6c42395eac4a18b6a4`.
- 페이지 viewport `1280×633 CSS px`, DPR 1. Chrome user agent는 `HeadlessChrome/155.0.0.0`로 축약되므로 정확한 build/revision은 `chrome://version` 출력에서 확인했다.
- fixture: [`overflow-c13-1.html`](../../../../tests/fixtures/css/c13/overflow-c13-1.html), SHA-256 `d08ee25be580208253ea0b64885e96924413961304cdf87b72cedb518f147cd8`.
- 원본 DOM/CSSOM 관찰값: [`chrome155-observation.json`](chrome155-observation.json). Headless Chrome은 `--hide-scrollbars`로 실행했으므로 화면 캡처는 스크롤바 시각 모양의 근거가 아니다.
- [고정 화면](chrome155.png), SHA-256 `25e1bc321ac6dfdb2e9412764b1a09ecdf4f3b0d626d26a9cfec0a05237019f6`.

## 관찰값

- 일반 `visible`, `hidden`, `clip`, `scroll`, `auto`에서 computed overflow와 `client*`/`scroll*` 크기를 기록했다. `hidden`·`scroll`·`auto`는 `scrollTop`/`scrollLeft=20`을 반영하고, `clip`은 두 값을 0으로 유지했다.
- 축 상호작용은 `overflow-x:visible; overflow-y:hidden` → computed `auto/hidden`, `overflow-x:clip; overflow-y:auto` → `hidden/auto`였다.
- `overflow-clip-margin:8px`는 computed `8px`였다. screenshot에서 기본 border radius clip과 clip-margin 확장을 함께 확인했다.
- float 대조에서 hidden의 border-box 높이는 64px, clip은 28px, clip+flow-root는 64px였다. 이는 hidden이 새 formatting context를 만들고 clip은 만들지 않는 관찰이다.

이는 C13 구현 전 Chrome oracle 캡처다. Spinon 동작, WPT 적합성, Android/iOS 스크롤바 모양을 증명하지 않는다. 전체 결과와 조건은 [`chrome155-observation.json`](chrome155-observation.json)에 보존했다.
