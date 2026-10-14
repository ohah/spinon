# C13.1 compact 13-case surface matrix · 2026-10-14

## 목표

C13.1의 13개 유효 overflow/clip 행을 Android AVD의 실제 300×100 CSS-px runtime surface 한 화면에 넣고 Chrome 155 oracle과 비교한다. Android `PixelCopy`에서 각 행의 target paint, clip 경계 fallback, parent/root paint를 고정 좌표로 읽는다. iOS Simulator에서는 같은 V8·Stylo·Taffy·WGPU fixture를 Metal surface에 제출하고 화면을 캡처한다.

이 slice는 fixture-test 전용이다. 제품 CSS 지원, 공개 API, 전체 surface pixel equality, DOM event dispatch, WPT 통과 또는 C13.1/C13 완료를 주장하지 않는다. full viewport host hit identity는 [PR #160](https://github.com/ohah/spinon/pull/160)의 별도 검증으로 유지한다.

## 기준 oracle과 장면

- Fixture source를 browser와 app runtime이 공유한다: `tests/fixtures/css/c13/c13-1-compact-surface-matrix.js`; browser wrapper는 `c13-1-compact-surface-matrix.html`로 둔다.
- Browser는 agent-browser `0.31.1`과 Chrome for Testing `155.0.8059.40`, revision `cfaadc5a132d78e1828635aa8405a499f3e14864-refs/branch-heads/8059@{#936}`를 쓴다. Viewport는 `300×100` CSS px, DPR 1, scroll `(0,0)`이다.
- Root는 `(0,0,300,100)`이며 13개 absolute case tile은 3열×5행, `x=(index % 3)×100`, `y=floor(index/3)×20`이다. Tile은 96×18 px, 정상 clip box는 tile 기준 `(0,1,64,14)`에 padding 2 px/border 1 px, target은 content origin에서 88×16 px다. Positioned-contained의 target frame은 tile 기준 `(59,9,14,6)`, positioned-escape의 target은 `(58,8,30,12)`다. Static positioned-escape box는 `top:1px`가 적용되지 않아 tile top에 놓인다. Nested case는 outer clip을 visible로 두고 inner frame `(21,5,34,10)`의 x clip과 visible y overflow를 분리하며 target은 `(23,7,56,12)`다. 마지막 target도 root 경계 안에 있어야 한다.
- case ID는 `visible`, `clip-x`, `clip-y`, `clip-both`, `clip-margin-padding`, `clip-margin-content`, `clip-margin-border`, `clip-margin-hidden`, `mixed-visible-hidden`, `mixed-clip-auto`, `positioned-contained`, `positioned-escape`, `nested-clip`으로 고정한다.
- `overflow-clip-margin`은 네 margin case에서 4 px를 쓴다. 각 case는 정확히 네 surface sample을 제공한다: target/inner paint, target frame 바깥, x축 overflow, y축 overflow. 일반 case는 tile origin 기준 `(5,6)`, `(70,6)`, `(66,6)`, `(5,15)`를 쓴다. Positioned-contained는 `(61,12)`, `(70,12)`, `(66,12)`, `(61,15)`, positioned-escape는 `(61,12)`, `(70,12)`, `(66,12)`, `(61,16)`, nested는 `(25,10)`, `(60,10)`, `(52,10)`, `(25,16)` 순서다. Chrome precomparison에서 52개 좌표를 고정하고 중복을 거부한다. 좌표마다 `elementFromPoint` identity, computed background, browser screenshot의 실제 RGBA를 보관한다. Browser oracle은 frame과 computed overflow를 함께 기록한다. [고정된 Chrome 155 관찰](../spec/internal/evidence/c13-1-compact-surface-matrix-2026-10-14/README.md)은 13 frame/52 sample과 Chrome 155.0.8059.40의 전체 revision을 기록한다.
- 목표는 52 sample(13×4) 전부다. Chrome capture 단계에서 4개 sample의 좌표가 다른 case와 중복되거나 viewport 밖으로 나가면 좌표를 조정하고 기준을 먼저 고정한다. renderer test는 Chrome frame/hit identity·sample metadata와 비교한다.

## 합격 기준

1. 이 plan의 별도 실패 관점 20개 검토를 기록한 다음 Chrome oracle을 capture한다. capture metadata는 browser/revision/UA, viewport, DPR, scroll origin, fixture SHA를 포함한다.
2. 13개 case의 section, box, target, nested inner frame을 Rust runtime report와 0.5 CSS px 이하 오차로 비교한다. missing/duplicate ID, failed profile, overflow axis normalization 변화는 즉시 실패한다.
3. Chrome `elementFromPoint` identity와 Rust `hit_test_css_point` 결과를 52개 고정 CSS 좌표에서 exact node ID 또는 root/null로 비교한다. 이 query는 DOM event dispatch test가 아니다.
4. Android API 37 ARM64 AVD `ro.kernel.qemu=1`의 C13.1 fixture-test route에서 `status=0`, `layout=ready`, 13 case, 52 query sample을 확인한다. SurfaceView `PixelCopy`가 52/52 sample ID·좌표·RGBA를 Chrome 고정 oracle과 exact 비교하고 full `PixelCopy` PNG 및 hash를 보존한다.
5. Android sample 수/색상·좌표 변경, PixelCopy crop/density/orientation mismatch 또는 route collision을 negative control로 거부한다. 기본 FFI artifact에서 신규 fixture symbol/source가 빠지고, 기존 C13.1 fixture-test feature artifact에는 추가 경로가 들어가는지 확인한다.
6. iPhone 17 Pro / iOS 26.2 Simulator에서 같은 13-case source를 새 고유 route로 평가한다. 13/13 geometry 및 52/52 hit query를 독립 Chrome oracle과 비교하고 Metal surface를 화면 캡처한다. Drawable readback이 없으면 없다고 명시한다.
7. Android PixelCopy 화면을 PR 본문에 inline 첨부하고 agent-browser에서 실제 표시와 natural pixel dimensions를 확인한다. 이미지 설명은 AVD surface capture이며 GPU hardware, final compositor, physical device 결과를 뜻하지 않는다.
8. focused/workspace tests, fixture feature boundary, Clippy, rustfmt, iOS/Android build·runtime, `git diff --check` 로그를 evidence에 저장하고, 구현 이후 새 20관점 리뷰와 PR 변경 30관점 리뷰를 분리한다.
9. [상태 대장](../spec/STATUS.md)와 [C13 계획](c13-overflow-scroll-clipping.md)에 측정된 matrix만 추가한다. full surface rendering, DOM event/CSSOM, WPT와 C13.1/C13 완료는 확인 전까지 미완료로 유지한다.

## 실행 결과 · 2026-10-14

- Android API 37 ARM64 AVD (`ro.kernel.qemu=1`, ANGLE/Vulkan/SwiftShader): 41/41 node frame, 52/52 fixed hit identity, 52/52 PixelCopy RGBA exact. `PixelCopy` PNG는 788×263 px이고 SHA-256은 `d585f3c4b06558e292c7d2553edd18636bf75da64edcf9c1971ee665a7ca59d9`다.
- iPhone 17 Pro / iOS 26.2 Simulator: 41/41 frame, 52/52 hit identity. Metal surface 화면을 보존했지만 drawable readback은 수행하지 않았다.
- ADB transport는 정상으로 확인했다. 실패는 fixture build flag 이름 불일치, body-less host document, compact viewport override, parent/child paint order에서 발생했고 모두 수정했다. [실행·로그·화면·feature isolation 근거](../spec/internal/evidence/c13-1-compact-surface-matrix-2026-10-14/runtime-execution.md).
- 전체 workspace test와 Clippy, rustfmt, Android/iOS Simulator build가 통과했다. Android의 중복 C13.1 route 거부와 conflicting build-flag 거부 negative control을 확인했다. 구현 failure review 20개와 PR delta review 30개는 별도 파일로 추가한다.
- 이 결과는 고정 52-point sample, AVD software backend, iOS Simulator 화면 범위다. 전체 surface byte equality, broad frame-outside controls, DOM event/CSSOM, WPT, 실기기/hardware GPU 또는 C13.1 완료를 증명하지 않는다.

## 제외와 후속 경계

- 1280×1100 full viewport host hit identity는 PR #160의 1,408,000-point scan이 이미 다룬다. 이 compact AVD fixture는 전체 300×100 RGBA surface byte scan이 아니라 52개 사전 고정 PixelCopy sample과 PNG다.
- iOS Simulator surface draw/화면 캡처는 Metal drawable readback·compositor/scanout proof가 아니다. 실기기는 이 작업에서 사용하지 않는다.
- touch/mouse event, capture/bubble, default action, CSSOM scroll state, border-radius/transform, float parity, WPT execution은 별도 gate로 남긴다.

## 실행 순서

1. 이 문서와 별도 20관점 plan review를 고정한다.
2. compact fixture source와 Chrome wrapper를 추가하고, agent-browser에서 frame·hit·52 sample metadata를 먼저 기록한다.
3. Rust runtime host test로 compact fixture geometry/hit report를 Chrome과 대조한다.
4. fixture-only FFI entry와 Android/iOS route를 추가한다. 기본 artifact isolation negative control을 포함한다.
5. Android AVD PixelCopy 52점과 iOS Simulator screenshot/runtime query를 실행한다. 물리 기기는 연결하지 않는다.
6. 실행 결과·스크린샷·review 문서를 저장하고 20 implementation/30 PR delta 검토를 각각 수행한다.
7. 최신 main, required CI, mergeability와 linear history를 확인하고 PR을 Rebase and merge한다.
