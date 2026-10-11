# C12.4 · Chromium 쌓임 맥락 사전 비교

**상태:** 고정 Chromium reference 생성 완료 · 기능 구현 전
**계약:** [0058](../0058-c12-4-stacking-context.md) · **계획:** [C12.4](../../../plan/c12-4-stacking-context.md)
**입력:** [fixture inventory](../../../tests/fixtures/css/c12/stacking-context-inventory.json) · [HTML 진입점](../../../tests/fixtures/css/c12/stacking-context.html)
**결과:** [고정 Chromium observation JSON](../../../tests/fixtures/css/references/c12-4-stacking-context-v1.json)
**도구:** [capture](../../../tools/css-reference/capture-c12-4-stacking-context.mjs) · [회귀 테스트](../../../tools/css-reference/c12-4-stacking-context.test.mjs)

## 실행 조건

- macOS `darwin/arm64`; Node.js `v26.7.0`.
- 실행 파일 `/Applications/Google Chrome.app/Contents/MacOS/Google Chrome`; `Google Chrome 154.0.8037.98`, DevTools revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`, binary SHA-256 `ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954`.
- network offline, locale `en-US`, timezone `UTC`, viewport `1000×1400` CSS px, DPR 1·2, `screen`, dark mode off, forced colors off, coarse pointer on, hover off.
- 26 scenes, 77 unique element nodes, 23 stacking-effect negative controls, 27 interior pixel samples per DPR.
- WPT source candidates are pinned to `c999c58338ee1d223df5ad62e48be1202ced0d37`. Their execution field remains `not-run`; this is not a WPT pass claim.

## 관찰과 수정

처음 `row-reverse` 겹침 표본은 source-second가 위에 있다고 예상했지만, 고정 Chrome screenshot의 DPR 1·2 모두 source-first의 파랑 `[40, 102, 246, 255]`을 반환했다. 기대값을 억지로 통과시키지 않고 독립된 최소 HTML 재현을 실행했다. `row`에서는 source-second 빨강이, `row-reverse`에서는 source-first 파랑이 겹침점에 표시됐다. 역방향 Flex에서 좌·우 음수 margin을 바꾼 별도 재현도 같은 topmost 결과를 보였다.

그래서 inventory를 pinned Chrome에서 실제로 관찰한 front-to-back 후보와 pixel로 고쳤다. `order` 계산값, DOM source order, top-to-bottom 표본 후보, screenshot RGBA, `elementsFromPoint()`를 서로 다른 자료로 보존한다. `elementsFromPoint()`는 보조 관찰로 저장하며 paint rank의 oracle로 사용하지 않는다. 이 한 표본은 Chrome 154의 경험적 결과다. WPT 실행이나 다른 브라우저의 동일 결과를 뜻하지 않으며, 이를 `order` 전반의 규칙으로 일반화하지 않는다.

## 결과

- `node tools/css-reference/capture-c12-4-stacking-context.mjs` — 성공. 기존 reference가 있으면 덮어쓰지 않으며, `--replace-reference`는 검토된 명시적 재생성에만 허용한다.
- 두 DPR의 모든 computed assertion, 유한 frame, 23 negative-control computed value 및 27 pixel sample이 기대값과 일치했다. 두 DPR의 CSS geometry·computed style·보조 hit-test 관측과 pixel 값이 같았다.
- [DPR 1 screenshot](./c12-4-stacking-context-2026-10-11/chrome-dpr-1.png), SHA-256 `42d5276e2752a11e59cef6c6bb11aabff717b33a05f60008a0497f1727d9763b`.
- [DPR 2 screenshot](./c12-4-stacking-context-2026-10-11/chrome-dpr-2.png), SHA-256 `6e80f2073373a054c829740e2a6b8aecdbeb5c3e4f03167732c8158074fa68d6`.
- 두 캡처를 화면으로 확인했다. 오른쪽 stage는 x=745에서 시작해 x=980에서 끝나므로 1000 CSS px viewport 안에 20 px 여백이 남고, 양쪽 DPR 모두 잘림이 보이지 않는다. 이는 Chrome fixture 캡처의 시각 확인이며 모바일 화면 확인을 뜻하지 않는다.
- `node --test tools/css-reference/c12-4-stacking-context.test.mjs` — 3/3 통과.
- `bun run test:css-reference` — 166/166 통과.
- `git diff --check` — 통과.

## 범위와 남은 검증

이 산출물은 CSS reference와 tooling fixture다. Rust paint builder, V8→Stylo→Taffy→WGPU 런타임, Android 실기기, iOS Simulator, GPU readback 또는 지원 API를 구현·검증하지 않았다. Chrome frame이 외부 layout engine과 비교된 것도 아니다. 0.5 CSS px는 이후 cross-engine 비교의 허용치이며 이번 Chrome 단독 기준 수집에서 달성 오차로 보고하지 않는다.

고정 WPT 후보 5개의 원본 경로와 hash:

| WPT 경로 | SHA-256 | 실행 |
| --- | --- | --- |
| `css/CSS2/zindex/z-index-stack-001.xht` | `71ddebb4643d0a99f4b07e7fa9e8f507c6ce2319be0ba4d3ffff5a37264440d1` | 미실행 |
| `css/CSS2/zindex/z-index-abspos-003.xht` | `d04f95aefe8ba06a49cd567e03595cd35dc580d32e234d5d79535e18cd9852c6` | 미실행 |
| `css/css-position/position-absolute-under-non-containing-stacking-context.html` | `6413b79fab31d82c3e84729e7e7364a93526ae5ffb19ba59c08dc25d9003e7ed` | 미실행 |
| `css/css-flexbox/flexbox-items-as-stacking-contexts-001.xhtml` | `4ca1879dd67d79897770e949064838b97d95c517abc8e8fc343038942bcb5ddb` | 미실행 |
| `css/css-flexbox/flexbox-paint-ordering-002.xhtml` | `ebb748144bddd3b1fc6aa68b0a45d7a869c07db632ced97ab62fc2c7ff2c293d` | 미실행 |

참고한 규격은 [CSS 2.2 Appendix E](https://www.w3.org/TR/CSS22/zindex.html), [CSS Flexbox 2025-10-14 §4.3](https://www.w3.org/TR/2025/CRD-css-flexbox-1-20251014/#painting), [CSS Display 3 §3](https://www.w3.org/TR/css-display-3/#order-property)다. 이 문서는 규격 전체 적합성이나 WPT 통과를 주장하지 않는다.
