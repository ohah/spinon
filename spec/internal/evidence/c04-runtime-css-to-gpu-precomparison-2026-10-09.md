# C04.10 구현 전 Chromium 비교 기준

## 고정 입력

모바일 앱은 기록된 JavaScript 파일을 그대로 V8에 전달한다. Chromium은 기준 계측을 위해 동등한 HTML fixture를 연다. 정적 fixture 검증은 두 파일의 DOM 순서와 각 인라인 style 값, Android JNI 선언·구현 일치, iOS가 동일한 Rust fixture FFI를 호출하는지 확인한다. 아래 Chromium reference는 HTML·inventory 입력에 대한 oracle이며 V8 실행이나 앱 화면 결과를 뜻하지 않는다.

- 계획: `plan/c04-runtime-css-to-gpu.md` · SHA-256 `637e3bb66a0104c3721b6aaa74c43d6103e495eeebabc280464c16a2c0c5d395`
- HTML: `tests/fixtures/css/c04/runtime-css-to-gpu.html` · SHA-256 `7d6c9023eeacb530b1b6df415c7d9fbf3d981cf378195bf60f5490427a894477`
- 앱에 전달하는 JavaScript: `tests/fixtures/css/c04/runtime-css-to-gpu.js` · SHA-256 `6d90c8d9f178dbaf82d0fd11845dd456962f2729614c1b52ee96a8e68ae27968`
- 노드·환경·판정 입력: `tests/fixtures/css/c04/runtime-css-to-gpu-inventory.json` · SHA-256 `7492b2947d5a628a8ece4abf64df436dd747ca268569de3bfa79fff3b93541ef`
- 캡처 도구: `tools/css-reference/capture-runtime-css-to-gpu.mjs` · SHA-256 `78e344cc93428a6dfecb42c76da9341867fc463ea53e18158badb848d3d1dca7`
- 재사용한 Chromium 수명·CDP 보조 도구: `tools/css-reference/chromium-session.mjs` · SHA-256 `b010ada41ed75dc2d67ee25abd4266b17930f1de95fc683eebc5cdab6ad2cdc1`
- 결과: `tests/fixtures/css/references/c04-runtime-css-to-gpu-v1.json` · SHA-256 `9b61d453cc66847ae07abe8c9c505add25f97d5644d0f242c350f757319b3552`

## 실행 환경

- Chromium `Chrome/154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`
- 실행 파일 `/Applications/Google Chrome.app/Contents/MacOS/Google Chrome` · SHA-256 `ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954`
- macOS 26.5.1 (25F80), arm64, Node `v26.7.0`
- CSS viewport `301 × 100`, device scale factor `1`, locale `en-US`, timezone `UTC`, light color scheme, coarse pointer, hover 미지원
- Chromium은 임시 profile과 임시 CDP 포트를 사용했고 종료를 확인한 뒤 profile을 삭제했습니다.

## 관찰된 기준

| DOM preorder | 표시값 | computed background-color | x | y | width | height |
| --- | --- | --- | ---: | ---: | ---: | ---: |
| `root` | `flex` | `rgb(18, 52, 86)` | 0 | 0 | 301 | 100 |
| `opaque` | `block` | `rgb(51, 102, 255)` | 0 | 0 | 51 | 31 |
| `transparent` | `block` | `rgba(0, 0, 0, 0)` | 62 | 0 | 41 | 31 |
| `hidden` | `none` | `rgb(255, 0, 0)` | 0 | 0 | 0 | 0 |

Stylo computed property는 Chromium 문자열과 정확히 비교합니다. 네 개 geometry 값은 노드별로 각각 절대 오차 0.5 CSS px 이하여야 하며 평균 오차로 실패를 상쇄하지 않습니다.

GPU offscreen readback의 고정 내부 표본은 정수 pixel 좌표입니다. 표본 `(25, 15)`는 `opaque` 내부이므로 기대 RGB는 `(51, 102, 255)`입니다. `(150, 50)`는 `root`의 자식 밖이므로 `(18, 52, 86)`입니다. `(82, 15)`는 투명 자식 안이므로 자식은 paint하지 않고 아래 root 색 `(18, 52, 86)`이 보여야 합니다. `hidden`은 0×0이므로 표본이 없습니다. 각 표본은 채널별 exact match입니다.

이 값은 Chromium computed style·geometry에서 수집한 기준입니다. Chromium OS screenshot을 pixel oracle로 사용하지 않습니다. 구현에서는 동일한 런타임 렌더 장면으로부터 offscreen pixel과 모바일 화면을 따로 확인합니다. 화면 캡처는 surface 제출의 시각 근거이며 픽셀 일치 근거가 아닙니다.

## 생성 명령

```sh
node tools/css-reference/capture-runtime-css-to-gpu.mjs
node --test tools/css-reference/c04-runtime-css-to-gpu.test.mjs
```

reference 파일이 이미 있으면 캡처 도구는 덮어쓰지 않고 실패합니다. 입력·캡처 코드 변경은 새 검토와 새 기준 파일을 요구합니다.
