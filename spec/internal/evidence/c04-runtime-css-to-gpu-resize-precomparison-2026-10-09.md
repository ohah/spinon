# C04.10 surface resize 구현 전 Chromium 비교 기준

## 판정 범위

이 비교는 surface 크기 변경이 Android·iOS의 CSS viewport 입력까지 전달되는지 확인하기 위한 기준이다. 기존 C04.10 `301×100 CSS px` fixture와 reference를 보존하고, viewport 단위 root를 쓰는 별도 fixture를 추가했다. HTML은 Chromium oracle 전용이며 앱은 같은 DOM 구조의 JavaScript를 실제 V8에 전달한다.

## 고정 입력과 도구

- 계획: `plan/c04-runtime-css-to-gpu.md` · SHA-256 `e6893181941a2820f1db29782a7520703aa2e30d9af3288546ec5a478b79d199`
- Chromium HTML: `tests/fixtures/css/c04/runtime-css-to-gpu-resize.html` · SHA-256 `0f6a7ec944e597c9073588204008d0567873acf7886b7885f9da57615aff168b`
- 모바일 V8 JavaScript: `tests/fixtures/css/c04/runtime-css-to-gpu-resize.js` · SHA-256 `bc352b23c63f7e8fdab1baccc6d35be52657a5399370aee11810b67bd3402407`
- viewport·요소·판정 목록: `tests/fixtures/css/c04/runtime-css-to-gpu-resize-inventory.json` · SHA-256 `daac2f9266a33669dd2ba58bfa9400b49457f31af1a228ad15925396da51acdc`
- Chromium 캡처기: `tools/css-reference/capture-runtime-css-to-gpu-resize.mjs` · SHA-256 `c337cbcf63f2d6b337710ecf46c5fff70732402aaae07d4100f29eb85418b49b`
- Chromium 세션 helper: `tools/css-reference/chromium-session.mjs` · SHA-256 `b010ada41ed75dc2d67ee25abd4266b17930f1de95fc683eebc5cdab6ad2cdc1`
- 고정 reference: `tests/fixtures/css/references/c04-runtime-css-to-gpu-resize-v1.json` · SHA-256 `2eeee75f064e6787313ba661cbee50c597b8c16bbf9cf1b6266df405271c72a4`
- reference ID: `chromium-darwin-arm64-Chrome-154.0.8037.98-daac2f9266a3-c337cbcf63f2-ccffd5c5fe77`

Chromium은 `Chrome/154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`, 실행 파일 SHA-256 `ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954`다. 캡처는 macOS 26.5.1 (25F80), arm64, Node `v24.20.0`에서 실행했다. 각 viewport는 device scale factor 1, locale `en-US`, timezone `UTC`, light, coarse pointer, hover 미지원 조건이다.

## 사전 고정 결과

root의 `width:100vw;height:100vh`는 Chromium computed value에서 현재 viewport의 CSS px로 계산된다. 자식은 고정 크기를 유지한다.

| viewport | root computed width/height | root frame | opaque child | transparent child | hidden child |
| --- | --- | --- | --- | --- | --- |
| `301×100` | `301px` / `100px` | `(0,0,301,100)` | `(0,0,51,31)` | `(62,0,41,31)` | `display:none`, `(0,0,0,0)` |
| `341×128` | `341px` / `128px` | `(0,0,341,128)` | `(0,0,51,31)` | `(62,0,41,31)` | `display:none`, `(0,0,0,0)` |

모든 표시 node의 각 `x/y/width/height`는 Chromium과 각각 비교하며 허용 오차는 최대 `0.5 CSS px`다. 평균으로 다른 좌표 오차를 상쇄하지 않는다. 기존 배경색·투명·숨김 결과는 두 viewport에서 유지되어야 한다.

Android에서 표면의 pixel 크기를 display density로 나눈 값이 CSS viewport 기준이다. iOS에서는 `UIView.bounds` point 크기가 CSS viewport 기준이며, drawable pixel 크기는 point 크기에 화면 scale을 적용해 WGPU에 전달한다. 실기기 결과를 대체하거나 성능을 증명하는 기준은 아니다.

## 재생성

```sh
mise exec -- node tools/css-reference/capture-runtime-css-to-gpu-resize.mjs
node --test tools/css-reference/c04-runtime-css-to-gpu-resize.test.mjs
```

캡처 도구는 기존 reference를 덮어쓰지 않는다. viewport·HTML·JavaScript·도구 변경은 새 비교 검토와 새 reference 파일을 요구한다.
