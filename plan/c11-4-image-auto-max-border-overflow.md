# C11.4 · direct PNG auto/max mixed track의 테두리 밖 overflow

**상태:** 구현 전 계획 · C11.4 및 공식 `72/213`은 미완료로 유지한다.
**기준 source:** `origin/main` `29449e4f794e53255909b58ed8fb2fd4995dd5d2`에서 분기했다.
**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)
**기존 제한 slice:** [direct PNG auto/max mixed-track](./c11-4-image-auto-max-mixed-track.md)
**사전 비교:** [Chrome geometry·overflow·surface 표본](../spec/internal/evidence/c11-4-image-auto-max-border-overflow-precomparison-2026-10-26/README.md)

## 목적과 범위

이미 병합된 `image-auto-max-mixed-tracks`와 같은 direct HTML PNG intrinsic sizing을 사용해, 1px border를 가진 작은 Grid의 auto-min track이 content box와 오른쪽 border를 넘어도 image item geometry 및 box paint가 보존되는지 확인한다. 기존 장면은 border가 없고 root 폭이 100px이므로 테두리 밖 overflow를 입증하지 않는다.

새 scene은 `image-auto-max-mixed-tracks-border-overflow` 하나다. `24×16` 고정 PNG, `minmax(auto,max-content) 17px` columns, 5px column gap, auto row, `12×40` border-box root, 네 방향 1px root border를 사용한다. root content width는 10px이다. 규범 기대값은 columns `[24,17]px`, row `[16]px`, root border-box `(0,0,12,40)`, image box `(1,1,24,16)`이다. Image는 오른쪽 border edge `x=12`에서 바깥으로 13 CSS px 이어진다. 이 장면은 overflow 결과와 경계 위치를 검사하며 overflow 속성을 새로 허용하지 않는다.

이번 runtime fixture가 image texel을 그린다고 주장하지 않는다. 기존 image adapter는 intrinsic size를 layout에 전달하고, 앱 fixture의 image background color(`rgb(229,72,77)`)를 item box marker로 표시한다. 따라서 browser oracle의 PNG texel 표본과 앱의 box-background 표본을 별도 결과로 기록한다. 앱에서 `image_draw_count=0` 또는 동등한 image-paint 부재 marker가 유지되는지 확인하고, 그 값을 image paint 성공으로 오해하지 않는다.

## 고정 입력과 독립 규범 기대값

기준 HTML은 [`grid-image-auto-max-border-overflow-c114-precomparison.html`](../tests/fixtures/css/c11/grid-image-auto-max-border-overflow-c114-precomparison.html)이다. 고정 PNG는 `tests/fixtures/resources/c14-1-png-basic.png`, 24×16 RGBA8, SHA-256 `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2`다. root는 `box-sizing:border-box; width:12px; height:40px; border:1px solid #22c55e`이며 padding은 0이다. 첫 column은 `minmax(auto,max-content)`, 이웃은 fixed `17px`, column gap은 `5px`; image의 authored width/height/margin/padding/border는 각각 auto/auto/0/0/0이다. Image는 자동 배치되어 첫 column 하나만 차지한다.

기대값은 simulator 출력이 아니라 다음 규범 산술에서 정한다.

1. CSS Box Sizing의 `border-box` 규칙에 따라 root의 12px border-box에서 양쪽 1px border를 빼면 Grid content width는 10px이고, 높이는 38px이다.
2. CSS Grid §6.6에서 item의 computed overflow가 scrollable이 아니고 item이 `auto` min track을 span하므로 auto minimum은 content-based minimum이다. PNG에는 natural dimensions만 있고 authored preferred/min/max size 및 반대 축 definite size가 없으므로 direct replaced item의 natural width 24px와 height 16px가 해당 축의 intrinsic 최소값·최대값이다.
3. CSS Grid §11.5의 non-spanning-item 단계에서 auto-min column base는 image의 minimum contribution 24px로, max-content maximum의 growth limit은 image의 max-content contribution 24px로 정해진다. Fixed 이웃은 17px, inter-track gutter는 5px이다. 합계 46px는 10px content width보다 36px 크다. `justify-content:start`이므로 첫 track은 root content 시작점에 놓이고 줄어들지 않는다.
4. 최종 columns는 `[24,17]px`, row는 `[16]px`이다. Border를 포함한 root frame은 `(0,0,12,40)`, image frame은 `(1,1,24,16)`이며 item 오른쪽은 `x=25`, root border-box 오른쪽은 `x=12`다. image box의 border 바깥 horizontal extent는 13px다.
5. Border는 CSS background/content와 다른 paint layer다. 측정 표본은 image가 겹치지 않는 y=30.5의 좌·우 1px border, PNG가 root border를 넘어서는 x=12.5/y=8.5, image 바깥 배경 x=26.5/y=8.5에 둔다.

규범 원문은 [CSS Grid Layout Module Level 1, 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto), [§11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-content), [CSS Box Sizing Module Level 3 §3](https://www.w3.org/TR/css-sizing-3/#box-sizing-property), [CSS Images Module Level 3 §4.1](https://www.w3.org/TR/2023/CRD-css-images-3-20231218/#natural-dimensions)이다. 해당 CSS Grid CRD §6.6은 item의 overflow가 scrollable이 아니고 해당 axis에 auto min track이 있으면 content-based automatic minimum을 사용한다고 규정한다. §11.5는 span-1 auto minimum을 minimum contribution에서, max-content maximum growth limit을 max-content contribution에서 구한다.

## Chrome 사전 관측

Agent Browser 전용 session에서 Chrome `156.0.8078.48`, macOS 26.5.1 ARM64, DPR 1, viewport `64×48`로 fixture를 열었다. 원본 DOM 결과와 screenshot은 [precomparison evidence](../spec/internal/evidence/c11-4-image-auto-max-border-overflow-precomparison-2026-10-26/README.md)에 보존한다. Chrome은 root rect `(0,0,12,40)`, `clientWidth=10`, columns `24px 17px`, rows `16px`, image natural size `24×16`, image rect `(1,1,24,16)`를 반환했다. `elementFromPoint(12.5,8)` target은 `image`다. screenshot의 `(12,8)` pixel은 `[255,255,64,255]`로 고정 PNG의 local `(11,7)` pixel과 같고, 이 지점은 root border 바깥이지만 image box 안이다. `(26,8)`은 root 배경 `[16,24,32,255]`; image가 겹치지 않는 좌·우 border 표본 `(0,30)`과 `(11,30)`은 각각 `[34,197,94,255]`다.

Chrome의 image `overflow-x/y=clip/clip`은 metadata로 기록한다. 이는 image content의 자체 box clipping 값이며 Grid item의 root 외부 overflow를 막는 부모 clipping을 뜻하지 않는다. 이 oracle은 C11.4/Spinon computed style 일치나 일반 `overflow` 구현 근거가 아니고, 이 fixture 입력에 대한 geometry 및 screenshot 관찰값이다.

## 구현 경로

변경은 다음 전용 opt-in C11.4 fixture 경계 안으로 제한한다.

- `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_image/cases.rs`에 root width 12, border 1, 기존 auto/max + fixed neighbor track을 갖는 case 하나를 추가한다. 기존 `ImageCase.grid_border`를 사용하고 일반 image/CSS adapter의 admission을 넓히지 않는다.
- `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs`에 같은 root/item ID와 style을 가진 scene 하나를 추가한다. 기존 `ImageSceneWithRowGap`처럼 root border를 구체적으로 고정하고 fixture count·source checks를 갱신한다.
- Android/iOS `C0410RuntimeGpuDemo`의 scene ID, intrinsic result, trace, root/item frame matcher에 같은 scene을 추가한다. 기대 track trace는 기존 mixed-track scene과 같고 root/item frame만 border-box 크기와 좌표를 반영한다.
- Simulator fixture runner에 CSS 좌표 표본을 추가한다. Chrome의 PNG texel과 다른 앱 box marker를 정확히 구분하며, 각 surface의 device-pixel scale, surface origin, pixel format 및 source/app hash를 함께 고정한다. 앱 표본 기대값은 `(12.5,8.5)`의 item background `[229,72,77,255]`, `(26.5,8.5)`의 root 배경 `[16,24,32,255]`, `(0.5,30.5)` 및 `(11.5,30.5)`의 border `[34,197,94,255]`다. 색은 포맷/scale이 확인된 내부 표본에서 exact 비교한다.
- Rust production track sizing algorithm, general CSS cascade/overflow projection, image decoder/paint path에는 변경을 계획하지 않는다. 규범 geometry가 main의 기존 algorithm 결과와 다르면 fixture를 기대값에 맞춰 수정하지 말고 구현을 멈춰 root와 원인을 분리한다.

## 실패 제어와 원자성

다음 조건은 새 image scene의 `prepare`/validator 경계에서 fail-closed해야 한다. 오류가 나면 부분 frame·track trace·draw/present marker·surface sample을 성공으로 보고하지 않는다.

- root/item ID, 24×16 PNG source, `12×40` root, 1px border, gap, alignment, rows 또는 track 함수가 바뀌면 해당 property/detail을 가진 거부를 확인한다.
- root border를 0으로 제거하거나 content-box로 바꾸는 mutation은 scene expectation에 맞지 않아야 한다.
- item을 명시 배치하거나 span, nested child, second image, extra direct child를 추가하면 기존 image adapter guard가 거부해야 한다.
- 기본/product profile에서는 신규 scene ID·fixture marker·fixture-only ABI가 빠지고 opt-in positive control에서는 포함돼야 한다.
- border 바깥 표본을 `#101820`으로 돌려보내는 clip-to-root 변이, item red background를 제거하는 paint 변이, left/right sample 좌표를 한 pixel 옮기는 변이는 전용 pixel gate를 실패해야 한다.

이 slice는 Android/iOS simulator의 고정 fixture evidence만 추가한다. PNG texel paint, 다른 image asset·size·format, 일반 CSS `overflow` 동작, 실제 app의 image texture, 전체 표면 parity 및 arbitrary content는 대상이 아니다.

## 검증 게이트

1. 구현 전 이 계획의 별도 20관점 검토를 통과하고 계획 source/commit SHA를 고정한다.
2. Runtime image adapter focused tests, C11.4 FFI source/scene tests, strict Android/iOS intrinsic-input·trace·frame matcher, Rust formatting/Clippy 및 fixture artifact boundary를 확인한다. 실행하지 않은 검사는 결과에 포함하지 않는다.
3. Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 새 scene과 기존 `image-auto-max-mixed-tracks` control을 각각 실행한다. Capture tool이 앱을 force-stop/terminate하면 실행 전에 현재 기기/앱 session을 확인하고 조율한다. Physical device는 연결하거나 사용하지 않는다.
4. 앱 최신 source hash, APK/app executable hash, install identity, CSS frame·trace, draw/present, surface origin/format, 다섯 표본과 screenshots를 기록한다. Screenshot은 해당 simulator 화면 증거이고 iOS Metal 표본은 compositor 제출 전, Android는 API 37 AVD software-render path임을 명시한다.
5. 구현/runtime 변경을 계획 검토와 별개의 20관점으로 다시 공격한다. 모든 PR diff는 별도 실패 경로 30관점 검토를 거친다. PR screenshot 링크가 실제 렌더되는지 전용 Agent Browser session으로 확인한 뒤 그 session/browser process를 닫는다.
6. `spec/STATUS.md`와 공개 roadmap에는 이 제한 scene 및 근거만 연결한다. C11.4/C11 checkbox와 공식 `72/213`을 변경하지 않는다. 제품 지원 범위 또는 전체 intrinsic sizing 완료로 해석될 수 있는 문구를 쓰지 않는다.

## 성공 기준과 제외

성공은 Chrome geometry/frame, CSS Grid 규범 산술, Android/iOS opt-in fixture frame·trace가 위 고정값에 맞고, 앱 표면에서 item box marker가 root right-border를 넘어 보이며 별도 border/background 표본도 기대 색인 것이다. 이는 overflow/clip 제품 기능 완료, PNG image texture paint, WPT, hardware GPU, physical device, iOS compositor 이후 화면, C11.4 전체 완료를 뜻하지 않는다. 공식 checkbox와 완료 수는 `72/213`으로 유지한다.
