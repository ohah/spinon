# C18.3 · 정적 PNG `object-fit` / `object-position`

**상태:** 계획 및 사전 비교 완료 · 구현 미착수
**상위:** [C18 이미지·replaced paint 규칙](../spec/STATUS.md) · [내부 계약 0065](../spec/internal/0065-c18-2-alpha-image-compositing.md)
**선행 범위:** C14.1–C14.3 intrinsic frame · C18.1 opaque PNG paint · C18.2 PNG source-over
**규범 기준:** [CSS Images Module Level 3](https://www.w3.org/TR/css-images-3/) §4.5–4.6 · [CSS Values and Units Module Level 3](https://www.w3.org/TR/css-values-3/) `<position>` value

## 목표와 제한

명시된 used width/height를 가진 HTML `img`의 content box 안에서 정적 RGBA8 PNG를 `object-fit`으로 크기 조절하고 `object-position`으로 배치한다. Taffy가 산출한 element frame은 유지하고 이미지의 concrete object rect와 texture sampling만 바꾼다. CSS layout sizing, intrinsic ratio의 재계산, 부모 overflow, source revision, C18.1/2 합성은 별도 동작으로 보존한다.

이 단계의 opt-in 내부 profile은 `object-fit: fill | contain | cover | none | scale-down` 다섯 keyword와 물리 x/y축의 두 `<percentage>` `object-position`을 지원한다. 초기값은 `fill` 및 `50% 50%`다. percentage는 0–100 범위로 clamp하지 않으며 음수·100 초과 값도 CSS Images의 남는 공간 계산에 따라 적용한다. default `image-rendering: auto`를 고정한다. `object-position`의 keyword, 길이, `calc()`, 1·3·4항 문법과 다른 `image-rendering` 값은 이 profile에서 fail closed한다.

입력은 caller-owned opaque RGBA8 PNG 한 장이며 natural size는 24×12다. 앱 fixture가 선택한 `<img>`의 정확한 CSS computed value를 읽는다. URL/network loader, 비동기 decode/replacement, SVG/다른 포맷, alpha-bearing 이미지 surface, border/padding/transform, auto·intrinsic used size, background image, 실기기와 hardware GPU는 범위 밖이다. C18·C14·U05와 공개 CSS/image 지원은 완료 처리하지 않는다.

## 구현 전 비교 모델

- 독립 oracle은 Agent Browser 0.31.1이 제어한 Google Chrome `155.0.8059.85`다. viewport는 `224×72 CSS px`, DPR 1이며 실제 `<img>` 12개에서 natural size, element frame, computed fit/position/filter, screenshot RGBA 표본을 수집했다.
- [HTML 입력](../tests/fixtures/css/c18/image-object-fit-position-c18-3.html) 및 [결정적 PNG 생성기](../tools/fixtures/generate-c18-3-object-fit-pattern.py)가 고정됐다. PNG는 24×12 opaque RGBA8, 4×4 CSS pixel 크기의 서로 다른 색상 셀 18개로 구성되며 encoded SHA-256은 `cc621c7a4be9345a8124b91ebcd5a6ff9bc3a3206ec59fe1228b24ba09cae454`다.
- [Chrome 사전 비교 근거](../spec/internal/evidence/c18-3-object-fit-precomparison/README.md)는 브라우저·입력 hash, 12개 computed value와 frame, 이미지 object geometry 산식, 19개 RGBA 표본을 기록한다. 원본 [PNG screenshot](../spec/internal/evidence/c18-3-object-fit-precomparison/chrome155.png)과 [관찰 JSON](../spec/internal/evidence/c18-3-object-fit-precomparison/chrome155-observation.json)도 함께 고정했다.
- 표본 허용 오차는 RGBA 채널당 0이다. 실제 표면 sample 및 전체 화면 screenshot의 표본은 해당 simulator의 CSS-to-surface mapping과 scene revision을 보존한다. 차이를 tolerance로 숨기지 않는다.

## 제한 동작 계약

1. `fill`은 concrete object size를 image content box와 같게 하며 종횡비를 유지하지 않는다.
2. `contain`은 natural ratio를 유지하며 `min(boxWidth / naturalWidth, boxHeight / naturalHeight)`로 크기를 정한다.
3. `cover`는 natural ratio를 유지하며 같은 축 비율 중 큰 값으로 크기를 정한다. 넘친 이미지는 image content box에서 자른다.
4. `none`은 natural size를 유지한다. concrete object가 content box보다 크거나 위치가 box 밖으로 나가도 content box 밖 픽셀은 보이지 않는다.
5. `scale-down`은 `none`과 `contain`의 concrete object size 중 작은 쪽을 고른다. 사전 fixture에는 축소되는 box와 원본 크기가 유지되는 box를 각각 둔다.
6. 두 position percentage를 `px`로 환산할 때 각 축 offset은 `(contentBoxSize - concreteObjectSize) × percentage`다. 따라서 cover의 음수 free space와 0–100 밖 percentage도 같은 식으로 처리한다.
7. image content clip과 누적 ancestor clip의 교집합을 사용한다. object-position은 element frame, layout geometry, hit-test 대상, paint order를 바꾸지 않는다.
8. raster scaling은 Chrome 155의 고정 `image-rendering:auto` pixel oracle와 맞춰야 한다. C18.1/2 natural-size 및 alpha 회귀가 바뀌지 않는 sampling 경계를 함께 고정한다.
9. fit/position 값의 누락, unsupported computed value, 잘못된 natural dimensions/revision, 계산 overflow, 잘못된 clip/UV는 frame 제출 전에 구체 오류로 거부한다. 이전 revision의 texture/style을 새 장면에 재사용하지 않는다.

## 구현 및 완료 관문

1. 계획 구현 전에 20개 계획 실패 관점 검토를 마쳤다. 구현 PR에서는 새로 20개 코드·cascade·runtime 실패 관점 검토를 하고, 각 발견을 수정한 뒤 영향을 받은 관점을 다시 확인한다. PR 변경분은 별도의 20개 적대 관점으로 검토한다.
2. `spec/internal/0066-c18-3-object-fit-position.md`에 이 profile의 typed value, 계산식, ownership, unsupported value 및 error 경계를 `0.1.0`으로 기록한다. 구현 PR에서만 `spec/STATUS.md` C18.3 하위 항목과 `spec/internal/README.md`를 갱신하고 C18 상위 및 공개 지원은 미완료로 유지한다.
3. Rust 단위 검증은 다섯 fit keyword, free space가 양수/0/음수인 축, negative/>100% position, fractional geometry, `scale-down` 양 분기, zero/non-finite/overflow dimensions, invalid style, UV/clip 교차와 기존 image key identity를 포함한다. test oracle을 제거하거나 바꾸는 negative control은 독립적으로 실패해야 한다.
4. Chrome 155 비교는 이 계획의 12 scene·19 표본을 모두 유지하고 computed value, element frame, concrete object rect, crop UV, filter result를 분리 비교한다. opaque PNG 기준과 C18.1/2 기존 fixture도 회귀한다.
5. Android API 37 ARM64 AVD PixelCopy 및 iPhone 17 Pro / iOS 26.2 Simulator Metal drawable에서 같은 source/revision/12 scene을 확인한다. 전체 화면 screenshot은 별도로 기록한다. 실기기와 iOS compositor 이후 pixel로 확대 해석하지 않는다.
6. 기본/opt-in Cargo 및 실제 Android/iOS 앱 산출물의 fixture ABI·JS·PNG payload 경계를 대조한다. fixture는 전용 C18.3 opt-in 경로에서만 들어간다.
7. `cargo fmt`, 관련 Rust/CSS reference 검사, Clippy, artifact boundary, 기본·opt-in build 및 simulator capture 근거를 저장한다. 미실행 gate는 통과로 표시하지 않는다.
8. Tailnet preview에 계획과 근거를 반영하고 URL/HTTP/hash를 확인한다. GitHub Pages를 배포하지 않는다. plan PR과 implementation PR은 각각 별도로 rebase merge한다.

## 미포함 범위

이 계획은 CSS Images의 모든 `<position>` 문법, `image-rendering` 속성, EXIF orientation, intrinsic/auto sizing, object aspect-ratio negotiation, image loading lifecycle, CSSOM 공개 API, 웹 표준 전체 적합성 또는 모든 GPU surface format의 color exactness를 주장하지 않는다. 이 제한 profile을 벗어난 입력은 성공 경로에서 조용히 기본값으로 바꾸지 않고 명시적으로 거부한다.
