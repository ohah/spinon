# C04.12 기본 앱 root HTML 시작 경로 · 사전 비교 · 2026-10-19

## Agent Browser 관찰

- 파일: `examples/bootstrap/index.html` (`e7ecee32a5c5f645cc90ea7bd0c524caaf9da93fd4c780ff9dbb29d0894dd3b9`)
- 같은 Chrome page에서 실행한 `examples/bootstrap/root-app.js` (`b87b53883ce838df86412657e1353f8dc90435049eb8ead6ba8fa478ca8a1c32`)
- Agent Browser 0.31.1, HeadlessChrome 155.0.0.0, CSS viewport 320×240, DPR 1, `prefers-color-scheme: dark`
- 결과: title `Spinon`, `lang=ko`, viewport meta `width=device-width, initial-scale=1`, color-scheme meta `light dark`, root computed `color-scheme: light dark`, body background `rgb(30, 140, 70)`, `#app` text `Spinon 루트 HTML`, `data-runtime=ready`.
- 이 결과는 실제 bootstrap HTML과 root script의 독립 기준이다. 모바일 Chrome UA, Android/iOS 화면, GPU 출력, 폰트·일반 CSS 호환성은 판정하지 않는다.
- [320×240 Chrome screenshot](./chrome-index-mobile.png) · SHA-256 `ceac01404e04e35d3a750bfc082c7c09a973194a797225409c9679ffe72128ab`.

## 사전 판정

기본 앱 경로는 이 입력에서 동일한 DOM 내용과 CSS viewport/scheme 환경을 GPU host에 전달하고 root script를 같은 parsed document session에서 성공시켜야 한다. 화면 성공은 asset→host→environment→script→layout/render 순서를 모두 통과한 뒤에만 인정한다. Browser 관찰만으로 Android/iOS surface 동작을 통과 처리하지 않는다.
