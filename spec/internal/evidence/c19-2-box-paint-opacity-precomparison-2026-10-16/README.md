# C19.2 box paint와 opacity 사전 비교

## 기준과 고정 환경

- 작업 전 main commit: `d97de7c4ba8492368fa163ffbbbb9357518055a3` (tree 및 구현 전 증거는 브라우저 캡처 후 기록).
- 기준 브라우저: 계획에서 지정한 Chrome 155.0.8059.85. Agent Browser 세션과 CDP `Browser.getVersion` 증거를 캡처한다. 이전 C19.1과 같은 설치 Google Chrome이며 Chrome for Testing 패키징 여부는 별도 검증되지 않았다.
- 실행 환경은 viewport `1280×633` CSS px, DPR 1, `color-scheme: light`, `forced-colors: none`, `color-gamut: srgb`, `lang=ko-KR`로 고정한다. 실제 OS·브라우저·표면 정보는 `browser-metadata.json`에 기록한다.
- CSS 장면은 [chromium-box-paint-fixture.html](./chromium-box-paint-fixture.html)에 고정한다. oracle 캡처는 Agent Browser로 연 이 HTML의 실제 DOM/화면 결과다. 터미널 브라우저는 사용하지 않는다.

## 제품 코드 변경 전 사전 판정 기준

각 sample point는 fixture `samplePoints`에 CSS pixel 좌표로 고정하고, screenshot은 DPR 1이라 같은 좌표의 표면 RGBA8을 측정한다. 샘플을 antialias edge에서 띄운다.

1. `rootBackdrop`은 sRGB 출력 표면의 불투명 바탕 `[32,49,75,255]`이어야 한다.
2. `sourceAlpha`는 반투명 배경색을 root surface 위에 source-over 한 값이다. `alphaTimesGroupOpacity`는 CSS 색 alpha와 group opacity를 모두 적용한 값이며 alpha는 각각 0.5와 0.5다.
3. `groupSingle`과 `groupOverlap`은 파랑·노랑 자식 두 개가 한 `.5` opacity group으로 합성된 pixel을 분리한다. overlap은 자식 각각에 `.5`를 준 구현의 값과 달라야 한다.
4. `nestedOpacity`는 안팎 `.5` group 두 개의 결과다. `opacityZero`, `opacityHalfSingle`, `opacityHalfOverlap`, `opacityOneOverlap`은 element와 child paint, 겹침의 group opacity 경계를 검사한다.
5. `clippedZOverlap`, `clippedRed`, `clippedAway`는 overflow clip 경계와 z-index 순서를 검사한다. 표면 표본은 clip 내부, clip 내부 겹침, clip 바깥으로 고정한다.
6. `currentColorBorder`는 `color: rgb(24 192 128 / 50%)`에서 온 8px solid border를 root surface 위에 합성한다. `borderInterior`는 border가 아닌 root surface 색이어야 한다.
7. 각 hit-test sample은 같은 문서에서 opacity 0/.5/1만 바꿔도 동일한 Node ID 또는 miss를 보존한다. `clip-outside`의 실제 Chrome target은 기록하지만, 구현의 hit-test clip 계약은 해당 장면의 painted clip 규칙에 맞춰 확인한다.
8. Chrome의 computed-style serialization, DOM hit target, 실제 screenshot, 표본 RGBA를 별도 보존한다. 캔버스 합성 예측이나 CSS 계산식을 실제 화면 캡처로 대체하지 않는다.
9. 구현 시 color alpha와 opacity는 별도 source values로 유지한다. 단순 channel clamp나 자식별 opacity 적용은 pass 기준이 아니다. GPU/시뮬레이터 pixel 허용 오차는 이 Chrome 캡처를 확보한 뒤 제품 코드 변경 전에 exact RGBA로 고정한다.
10. negative controls는 alpha 폐기, child 별 opacity 분배, nested group 하나 생략, clip 미적용, z-order 역전, opacity 0 hit 제거를 각각 구별해 실패시켜야 한다.

## 캡처 산출물

- `chromium-box-paint-observations.json`: 브라우저 버전, computed style, fixed hit target, fixed pixel coordinates/RGBA 및 source hash.
- `chromium-box-paint-oracle-agent-browser.png`: 실제 DOM 장면 화면 증거.
- `browser-metadata.json`: Chrome/OS/viewport/profile과 실행 파일·framework hash.
- `sampled-pixels.json`: screenshot PNG에서 직접 읽은 표면 RGBA8 및 sampler command/source hash.
- 이 README의 관찰값과 hash를 기록한 뒤 제품 코드 변경을 시작한다. fixture는 구현 후 바꾸지 않는다.

## 고정된 Chrome 관찰값

- Chrome `Browser.getVersion`은 `Chrome/155.0.8059.85`, protocol `1.3`, V8 `15.5.35.25`, WebKit revision `537.36 (@0e66b94549a8bd85f03fbdf189b752d6e7b63c32)`를 반환했다. 실행 파일 SHA-256은 `6e156f2271cef44afce9db36a5b920e94518857b94a4160f082696f8c3bd1c9d`, framework SHA-256은 `b1d0216b347be90a898e9961302916aa1dbf4659f96597ea836dd3eb149574b7`이다. 상세 환경은 [browser-metadata.json](./browser-metadata.json)에 있다.
- 고정 screenshot의 직접 RGBA8 표본은 [sampled-pixels.json](./sampled-pixels.json)에 있고, DOM computed style/hit-test와 같은 수치를 묶은 [chromium-box-paint-observations.json](./chromium-box-paint-observations.json)에 있다. 주요 RGB 표본은 다음과 같다.

| 장면 / 좌표 | Chrome RGBA8 |
|---|---:|
| root backdrop `(800,400)` | `[32,49,75,255]` |
| source alpha `(64,80)` | `[143,25,38,255]` |
| opacity group 단일 자식 `(170,80)` | `[16,89,165,255]` |
| 같은 group의 blue/yellow overlap `(205,80)` | `[143,121,38,255]` |
| 중첩 `.5 × .5` group `(332,80)` | `[24,101,56,255]` |
| background alpha `.5 × group opacity .5` `(432,80)` | `[88,37,56,255]` |
| `oklch(60% 0.15 230deg)` `(712,80)` | `[0,142,200,255]` |
| opacity 0 / `.5` red only / opacity `.5` blue overlap / opacity 1 blue overlap | `[32,49,75,255]` / `[143,25,38,255]` / `[16,25,165,255]` / `[0,0,255,255]` |
| clip 안 z-overlap / clip 안 red / clip 바깥 `(430,240)/(455,240)/(470,240)` | `[16,25,165,255]` / `[143,25,38,255]` / `[32,49,75,255]` |
| `currentColor` alpha border / 내부 `(600,212)/(600,248)` | `[28,121,102,255]` / `[32,49,75,255]` |
- opacity 0 표면도 `elementFromPoint(48,248)`에서 `opacity-zero`를 반환했다. clip 바깥 `(470,240)`은 `stage`, 그룹 겹침 `(205,80)`은 위쪽 `group-yellow`, z-overlap `(430,240)`은 `clip-blue`를 반환했다.
- OKLCH 표본은 C19.1과 같은 값이다. Chrome 155의 고정 sRGB surface 표본은 `[0,142,200,255]`, Stylo 0.22 Local MINDE는 `[0,142,196,255]`다. CSS Color 4 Local MINDE는 OKLCH의 lightness/hue를 보존하면서 chroma를 줄이는 경로다. Chrome 값은 이 브라우저에서 관찰한 sRGB screenshot 표본이며 알고리즘 선택에 대한 규범 단언은 아니다. 이번 작업에서도 source color, browser output, product output을 한 값으로 합치지 않고 별도 비교한다.
- 사전 판정은 현재 고정된 screenshot의 표본 RGBA8 exact match다. 표본 위치와 허용오차는 구현 전인 지금 동결한다. 실제 Android/iOS surface의 변환 차이가 확인되면 결과를 별도 플랫폼 차이로 기록하며 이 기준값을 사후 변경하지 않는다.

## WPT 입력 고정

- WPT revision은 `d3519c37fa17d5b2e27979fc2466c724660bdb9f`다. C19.1에서 실행 전 고정한 CSS Color computed/parser 파일 16개의 upstream blob SHA, byte SHA-256, 크기와 URL을 [wpt-source-manifest.json](./wpt-source-manifest.json)에 그대로 복사했다.
- manifest SHA-256: `4a737c140d2157c3d2d1a454aa04d2db7f657f6573914a0cd6dd22f9232b0d08`. 이 fixture는 WPT 실행이나 Spinon WPT 적합성 주장을 만들지 않는다.

## 입력 hash

- fixture HTML SHA-256: `111c2a6fa1797ab7046bd2850cc27115e76bb72ae907d515283493d2022ad922`.
- Chrome screenshot SHA-256: `61d352cd2b5671973637a875003d25a9b9e789fd7237ba874c0297ea3f29692d`.
- browser observations JSON SHA-256: `2ece56f51cbb0e7c682d7aa8802a217767af804e0e737c40828c162de40faa0e`.

## 참고 규범

- CSS Color 4: <https://www.w3.org/TR/css-color-4/>.
- CSS Backgrounds and Borders 3: <https://www.w3.org/TR/css-backgrounds-3/>.
- Compositing and Blending 1: <https://www.w3.org/TR/compositing-1/>.
- WPT source revision과 실제 참조 파일 byte hash는 캡처 전에 manifest로 고정한다.
