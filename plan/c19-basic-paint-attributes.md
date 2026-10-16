# C19 · 기본 페인트 속성

**상태:** 구현 전 계획 · C19 상태는 미완료로 유지
**상위:** [구현 상태 대장 C19](../spec/STATUS.md) · [CSS 호환 범위](../spec/0008-css-compatibility.md) · [GPU·입력 계약](../spec/0002-ui-tree-events.md)
**관련 작업:** S04 GPU 적용기 · C15/C16 텍스트 · S07 화면·접근성 · E02 GPU 품질
**규범 기준:** [CSS Color Module Level 4](https://www.w3.org/TR/css-color-4/) · [CSS Backgrounds and Borders Level 3](https://www.w3.org/TR/css-backgrounds-3/) · [Compositing and Blending Level 1](https://www.w3.org/TR/compositing-1/) · [WCAG 2.2, 1.4.3 Contrast (Minimum)](https://www.w3.org/TR/WCAG22/#contrast-minimum)
**검토:** [계획 실패 관점 20개](../spec/internal/evidence/c19-basic-paint-plan-review-2026-10-16.md) · [계획 PR 변경 검토 20개](../spec/internal/evidence/c19-basic-paint-plan-pr-review-2026-10-16.md)

## 현재 구현 경계

- S04 author fixture 검사는 **background-color**에 불투명한 **#RRGGBB**만 허용한다. Stylo 계산값 변환은 현재 `ComputedBackgroundPaint::Transparent | Opaque`로 제한되고, renderer의 `RuntimePaint`도 `None | Opaque`만 가진다.
- WGPU 사각형 pipeline은 alpha blending 없이 vertex alpha를 항상 1로 쓴다. 이미지 pipeline에는 별도 alpha blending이 있지만, 그것은 CSS box opacity 또는 후손을 포함한 group opacity를 구현한 근거가 아니다.
- 일부 계산 profile은 **color**를 Stylo에서 읽고 border의 `currentColor`에 사용하지만, 실제 glyph painter는 없다. Text node는 renderer에서 거부된다. **opacity**는 runtime paint profile에 전달되지 않는다.
- hit-test는 render box의 frame, paint order, clip을 기준으로 한다. 색의 alpha나 **opacity**를 추가해도 hit target을 바꾸면 안 된다.
- 현재 공식 상태는 C19 미구현이다. 이 계획과 계획 PR은 제품 지원·접근성 적합성·WCAG 준수를 완료로 표시하지 않는다.

## 목표와 완료 정의

Stylo가 계산한 CSS 색 값과 opacity를 revision이 붙은 typed paint snapshot으로 전달하고, sRGB surface에서 기준 브라우저와 같은 background·border·text paint 및 source-over compositing을 만든다. **color**, **background-color**, **opacity**의 계산값과 화면 결과를 같은 fixture에 연결한다. 색상이나 투명도 변화가 geometry 기반 hit-test를 바꾸지 않는지 확인하고, 명시된 foreground·composited background 쌍의 WCAG contrast ratio fixture를 둔다.

C19는 다음 세 단계의 결과가 모두 합쳐져야 완료된다. 각 구현 PR은 독립된 diff 검토 20개를 포함한다. C19 전체의 완료 체크는 마지막 공통 fixture와 C15/C16 text painter 통합을 통과한 뒤에만 갱신한다.

1. **C19.1 · computed color carrier:** CSS Color 4의 `<color>` 문법·computed value를 고정 sRGB 출력 환경으로 변환한다. named/system colors, `transparent`, `currentColor`, hex, rgb, hsl, hwb, Lab/LCH, OKLab/OKLCH와 `color()` 색 공간을 문법별로 고정해 검증한다. Stylo의 absolute color를 그대로 활용하고, 색 변환을 별도 CSS parser로 재구현하지 않는다. `color`의 상속과 초깃값, `background-color`의 transparent 초깃값·비상속, alpha, 유효 값 범위와 오류 cascade를 함께 검사한다.
2. **C19.2 · box paint와 opacity:** `background-color`와 지원되는 border `currentColor`의 alpha source-over를 GPU 사각형에 연결한다. **opacity**는 요소 자신뿐 아니라 자손과 내부 겹침을 먼저 하나의 group으로 합성한 뒤 적용한다. nested stacking context, clip, z-order, opacity 0·중간값·1, 색 alpha와 group opacity의 조합을 포함한다. CSS Color 4 색 공간에서 sRGB 출력으로 갈 때 표준 변환·gamut 처리를 적용하며 out-of-gamut 색을 조용히 단순 clamp하지 않는다.
3. **C19.3 · hit-test·contrast·text 통합:** 색 alpha와 opacity가 0이어도 같은 geometry/clip/paint order의 hit target이 유지되는지 대조한다. foreground와 alpha를 적용해 계산한 최종 background의 WCAG 2.2 contrast ratio를 고정 fixture에서 검증한다. 실제 text glyph의 color 출력은 C15/C16 text painter와 공통 revision fixture에서 연결한다. text painter가 없으면 C19를 완료 처리하지 않는다.

CSS Color 5의 relative color syntax와 `color-mix()`, gradient·background image, arbitrary ICC profile loading, dynamic CSSOM mutation, OS accessibility tree와 공개 DOM/CSS API는 이 C19 계획의 범위 밖이다. 이는 C19 완료나 전체 CSS Color 지원으로 확대 해석하지 않는다. 색상·opacity profile은 내부 runtime 범위로 유지한다.

## 구현 전 비교 기준

구현을 시작하기 전에 다음 자료를 고정한다.

- Agent Browser의 Chrome for Testing **155.0.8059.85**를 기준으로 한다. 실행 파일 SHA-256, CDP revision, OS, locale, viewport, DPR, forced-colors, color-scheme, surface color profile을 oracle 기록에 남긴다. 비교 환경은 `color-scheme: light`, `forced-colors: none`, sRGB 출력이다.
- CSS Color 4에서 위에 열거한 각 `<color>` grammar production, alpha 0/부분/1, clamp 경계, missing component, named/system color, `currentColor`, invalid syntax 및 cascade fallback을 포함한 fixture table을 만든다. 각 row는 입력 CSS, Chromium computed color, canonical color tuple, source hash와 산출 JSON hash를 갖는다.
- paint fixture는 opaque·translucent 배경, 색 alpha와 opacity의 중첩, 겹치는 자식, nested group, clip·stacking order, root surface backdrop을 독립 scene으로 둔다. 표본 좌표는 antialias edge를 피한 내부 pixel과 겹침 지점으로 사전 고정한다.
- hit-test fixture는 각 장면의 빈 공간·경계·겹침 지점을 `elementFromPoint()`의 Node 대응 결과와 비교한다. 같은 fixture에서 색/opacity만 1→부분→0으로 바꿔도 target 집합과 순서가 동일해야 한다.
- contrast fixture는 WCAG 2.2 relative luminance/contrast 계산을 독립 구현과 브라우저에서 대조한다. 고정 16 CSS px text pair의 normal-text 4.5:1 기준, exact-boundary의 양쪽 값, foreground/background alpha compositing, black/white backdrop을 포함한다. 큰 글자 기준과 접근성 tree 판정은 하지 않는다.
- WPT 원본 revision과 사용한 CSS Color 관련 파일의 byte hash는 실행 전에 고정한다. WPT pass는 Chromium oracle 결과로만 분류하고 Spinon WPT 적합성으로 부르지 않는다.

## 판정 기준

- Stylo cascade/computed value와 Chrome oracle의 color/alpha tuple이 각 grammar row에서 일치한다. Color-space conversion의 허용 오차와 8-bit surface quantization 경계는 사전 비교 자료에서 정하고 사후 조정하지 않는다.
- 모든 target scene의 Android API 37 ARM64 AVD PixelCopy 및 iPhone 17 Pro/iOS 26.2 Simulator Metal drawable 내부 표본 RGBA가 oracle과 exact 일치한다. Android backend와 iOS compositor 제출 시점은 각각 실행 README에 적으며, 이 결과를 실기기·hardware GPU·compositor-after 근거로 확대하지 않는다.
- Same-color scene에서 opacity 0/partial/1, nested opacity, source alpha, clip과 overlap 픽셀이 브라우저의 group compositing 결과와 일치한다. 예상 지원 밖 색상이나 invalid conversion은 기본색으로 바꾸지 않고 node/property/stage 진단으로 거부한다.
- 모든 hit-test 지점에서 alpha·opacity만 바꾼 쌍이 같은 NodeId 또는 같은 miss를 돌려준다. clip, half-open edge, paint order 기준은 기존 S04/C13 계약을 보존한다.
- Contrast ratio는 WCAG 2.2 수식의 독립 계산과 fixture가 일치한다. 이는 색 값 계산 근거이지 제품 접근성 tree, WCAG 준수, 실제 anti-aliased text 대비 판정이 아니다.
- 각 핵심 변환/guard를 제거하거나 바꾸는 negative control을 둔다: alpha 폐기, 잘못된 color-space conversion, straight/premultiplied alpha 혼용, group opacity를 각 child에 분산, opacity 0 hit 제외, 잘못된 contrast gamma, 변조한 pixel 좌표. 각 control은 전용 검증이 실패해야 한다.
- C19 app fixture route·고정 data는 opt-in fixture-test feature에 격리한다. 기본·제품 artifact에서 fixture FFI symbol·payload가 없고 opt-in positive control에는 있어야 한다. 제품의 supported CSS runtime 경로와 테스트 fixture 코드를 혼동하지 않는다.
- 전체 Rust workspace, Clippy, rustfmt, 관련 CSS reference checks, Android/iOS simulator app builds와 C18.1–C18.4 paint regressions를 실행한다. baseline 실패는 clean main에서도 재현해 별도 귀속한다. 실행하지 않은 gate는 통과라고 쓰지 않는다.

## 계획 단계 판정

- 현재 작업 PR은 이 계획과 별도의 계획 검토 자료만 추가한다. 구현 전 Chromium fixture, versioned internal contract, product code와 C19 완료 체크는 만들지 않는다.
- C19.1 구현 PR부터 구현 전 비교 JSON/hash와 version `0.1.0` 내부 인터페이스 문서를 함께 고정하고, 작업 전용 계획 검토와 분리된 구현 실패 경계 검토 20개 및 PR diff 검토 20개를 남긴다.
- Android/iOS 실행은 사용자가 허용한 시뮬레이터/AVD만 사용한다. 물리기기 연결·Pages 자동 배포·Tailnet 서버 재시작은 완료 조건이 아니다.
