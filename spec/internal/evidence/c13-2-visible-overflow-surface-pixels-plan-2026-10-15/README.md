# C13.2 visible overflow 표면 픽셀 검증 계획 · 2026-10-15

## 현재 판정

이 디렉터리는 구현 전 Chrome 155 oracle와 표면 검증 계획을 보존한다. Android API 37 ARM64 AVD PixelCopy와 iPhone 17 Pro/iOS 26.2 Simulator Metal drawable의 후속 표본은 [run5 구현 evidence](../c13-2-visible-overflow-surface-pixels-2026-10-15/README.md)에서 각각 5/5 exact로 확인했다. 두 결과는 C13.2 내부 fixture의 선택 pixel만 검증하며 제품 CSS 지원 완료를 뜻하지 않는다.

계획 PR [#175](https://github.com/ohah/spinon/pull/175)의 Chrome oracle image embed는 게시 후 agent-browser에서 400×240으로 로드·표시됨을 확인했다.

## Chrome oracle

- Chrome for Testing 155.0.8059.40, revision `cfaadc5a132d78e1828635aa8405a499f3e14864`; agent-browser 0.31.1, macOS 26.5.1.
- Viewport 400×240 CSS px, DPR 1.
- `c13-2-visible-border-pixel-probe.html`은 현재 C13.2 matrix source `runtime-scroll-container-c13-2.js`를 그대로 읽는다.
- root box는 `(0,0) 380×180`; visible parent border box는 `(10,10) 120×82`, 오른쪽 edge `x=130`; escaped child는 `(143,21) 30×24`다.
- agent-browser DOM geometry에서 표본 `(150,30)`은 `c132-visible-escape` 내부이고 parent 오른쪽 edge보다 20 CSS px 바깥이다. PNG 디코더로 읽은 RGBA는 `227,84,78,255`다.
- 표본 `(136,30)`은 escaped child와 겹치지 않는 parent 바깥 gap이며 `16,24,39,255`다. parent 내부, border strip 및 root background positive controls도 observation JSON에 고정했다.
- 화면 파일 `chrome155-c13-2-visible-border-400x240.png`는 400×240 RGB PNG, SHA-256 `9a0be14fab0e3a75ee4f223f1fae2a16ccb242c5f3ba499684bd5c016f0c81b6`이다. 실제 observation과 fixture/source SHA는 `chrome155-observation.json`에 있다.

## 구현 당시 관문

위 계획은 [PR #176](https://github.com/ohah/spinon/pull/176)에서 구현했다. PixelCopy/Metal 결과는 각각 simulator surface와 compositor 제출 전 drawable에 한정하며, comparator는 sample 변조·parent 안 좌표 이동·stale run/generation·누락/중복 controls를 거부한다.

PixelCopy 및 Metal 결과는 각각 simulator surface와 compositor 제출 전 drawable에 한정한다. 실제 기기, hardware GPU, 최종 display scanout, ink-only overflow 및 C13.2 전체 완료는 이 plan에서 주장하지 않는다.
