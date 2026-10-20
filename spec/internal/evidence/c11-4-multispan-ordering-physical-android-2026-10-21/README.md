# C11.4 multi-span source-order oracle와 Android 실기기 실행

## 범위

CSS Grid §11.5의 여러 spanning item 처리에서 span 길이 순서와 DOM source order가 track 결과를 바꾸지 않는지 확인한다. 두 개의 겹치는 span-2 item(80px, 120px)과 span-2/3 item 조합(80px, 120px, 200px)을 forward/reversed DOM order로 각각 실행했다. 기대값은 구현 결과에서 만들지 않았다.

공식 근거는 [CSS Grid Layout Level 1 §11.5 및 §11.5.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items)이다. span이 큰 순서로 처리하고, 각 item의 planned increase를 해당 span 내부 track에만 모아 다음 span 단계 전에 반영하는 규칙에서 다음 기대값을 도출했다.

- span-2 80px item은 첫 두 track에 40px씩 기여한다. 겹치는 span-2 120px item은 다음 두 track에 60px씩 기여한다. 같은 단계에서 track별 planned increase를 합산하므로 Chrome column 결과는 `40px 60px 60px 0px`이다.
- 그 뒤 span-3 200px item은 앞 세 track의 기존 합 160px에 필요한 40px을 더한다. 규범상 이상값은 `53.333…px 73.333…px 73.333…px 0px`이다.
- Chrome 155의 used-track serialization은 1/64 CSS px quantization으로 `53.3281px 73.3281px 73.3438px 0px`이다. item frame과 track은 Chrome의 1/64 CSS px 이내에서 비교했다.

Chrome 155 Agent Browser 사전 관찰 입력은 [HTML fixture](../../../../tests/fixtures/css/c11/grid-track-sizing-c114-multispan-ordering.html), raw 결과는 [chrome155-observation.json](chrome155-observation.json), 화면은 [chrome155-multispan-precomparison.png](chrome155-multispan-precomparison.png)다. 브라우저는 Chrome `155.0.8059.85`, macOS `26.5.1` ARM64, DPR 1이었다. 관찰은 규범 기대와 구분했으며, browser screenshot은 Spinon 실행 증거로 취급하지 않는다.

## Android 실기기

최종 fixture APK는 연결된 Samsung `SM-S731N`, Android 16/API 36, 1080×2340, density 450에서 설치·실행했다. GPU renderer 로그는 Samsung Xclipse 940 / Vulkan이다. 앱 설치 APK SHA-256은 `191f7457528f8691c3d0d03b1d8b28887a7172939a077b80e4d4b79fe4625224`이며 로컬 assemble APK와 기기 설치본의 해시가 일치했다.

네 장면 모두 runtime layout assertion과 `SPINON_C0410_DRAW status=0 presented`를 통과했다. Android Taffy track state는 이상적인 thirds를 f32로 보존했다. Chrome과의 최대 track 차이는 0.010415 CSS px, 최대 item-frame 차이는 0.005207 CSS px로 허용치 1/64 CSS px(0.015625)보다 작다.

| 장면 | 처리된 span 단계 | 기기 column tracks (CSS px) | Chrome 대비 최대 차이 | 결과 | 원본 log |
| --- | --- | --- | ---: | --- | --- |
| `competing-span2-forward` | 2 | `40, 60, 60, 0` | 0 | 통과, 3 boxes, draw/present | [log](runtime-physc114-final-rebuild-20261021-01-competing-span2-forward.log) |
| `competing-span2-reversed` | 2 | `40, 60, 60, 0` | 0 | 통과, 3 boxes, draw/present | [log](runtime-physc114-final-rebuild-20261021-02-competing-span2-reversed.log) |
| `span2-before-span3` | 2 → 3 | `53.333332, 73.333336, 73.333336, 0` | 0.010415 | 통과, 4 boxes, draw/present | [log](runtime-physc114-final-rebuild-20261021-03-span2-before-span3.log) |
| `span3-before-span2` | 2 → 3 | `53.333332, 73.333336, 73.333336, 0` | 0.010415 | 통과, 4 boxes, draw/present | [log](runtime-physc114-final-rebuild-20261021-04-span3-before-span2.log) |

최종 run ID는 `physc114-final-rebuild-20261021-01`부터 `-04`다. 복원·비교한 frames, contributions, stage order는 [physical-multispan-observation.json](physical-multispan-observation.json)에 있다. 앱 전용 화면은 [실기기 캡처](physical-final-span3-before-span2-app-view.png)이며 SHA-256은 `8b56433736c53e0568a1c526bdd0d4ea35b20062c01d9198608c06b980b7daa7`이다.

초기 span-2/3 실행에서 frame 수치는 허용치 안이었지만 앱 assertion이 Chrome의 decimal CSS string과 runtime f32 string을 정확히 같다고 요구해 두 장면을 거부했다. Assertion을 1/64 CSS px 수치 비교로 바꾸고 최종 APK를 다시 빌드해 전체 네 장면을 재실행했다. 최초의 assertion-only 거부 log는 [span2-before-span3](runtime-span2-before-span3-validator-prerun.log), [span3-before-span2](runtime-span3-before-span2-validator-prerun.log)에 원본 logcat 형식으로 보존했다.

같은 최종 APK에서 `auto-fill-overflow-neighbors`를 다시 실행했다(run ID `physc114-final-rebuild-20261021-overflow-02`). SurfaceView PixelCopy 표면은 900×675 px, viewport 320×240 CSS px였다. root 안쪽 색, root 밖 배경, root 바깥 item, root 내부 배경 표본 4/4가 정확히 일치했다. 특히 root 바깥 CSS x=91px item 색은 RGBA `(229,72,77,255)`였고, 경계 바깥 x=40px는 검정 배경 `(0,0,0,255)`였다. [원본 runtime log](runtime-physc114-final-rebuild-20261021-overflow-02-auto-fill-overflow-neighbors-pixelcopy.log), [PixelCopy image](physical-final-auto-fill-overflow-neighbors-pixelcopy.png), image SHA-256 `796301f45cf347492a2b21a1534804beacb86cc378dc2a202b06c549c404030c`.

이 PixelCopy는 Spinon SurfaceView 출력 관찰이며 Android OS compositor 이후 화면 검증은 아니다. Overflow 장면은 border stroke를 그리지 않으므로 border 선 색을 비교하지 않는다.

### 연결 후 재검증

2026-10-21 사용자가 연결한 같은 `SM-S731N`에 현재 branch의 fixture APK를 다시 빌드·설치하고 네 장면을 재실행했다. Run ID는 `physc114-connected-1792515035407-01`부터 `-04`이며 4/4에서 frame assertion, 전체 node frame 수, track trace, intrinsic span stage, draw/present가 통과했다. 빌드 APK와 설치 APK SHA-256은 모두 `191f7457528f8691c3d0d03b1d8b28887a7172939a077b80e4d4b79fe4625224`다. [재실행 요약](android-connected-2026-10-21/result.json) · [기기·APK 정보](android-connected-2026-10-21/device.json) · 원본 log: [forward](android-connected-2026-10-21/runtime-competing-span2-forward.log), [reversed](android-connected-2026-10-21/runtime-competing-span2-reversed.log), [span-2 → 3](android-connected-2026-10-21/runtime-span2-before-span3.log), [span-3 → 2](android-connected-2026-10-21/runtime-span3-before-span2.log).

## iOS Simulator

fixture-test flag를 켠 현재 source build를 iPhone 17 Pro / iOS 26.2 Simulator(UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`)에 설치해 네 장면을 실행했다. App executable SHA-256은 `e3b3ec01118a7f8901043061e11db96900a140c46ab7890c95262823fe87a47b`다. 런타임 전체 node frame과 draw/present가 4/4 통과했고 trace에서 span stage가 `[2]` 또는 `[2,3]` 순서로 처리됐다.

| 장면 | 최종 Columns tracks (CSS px) | Span stage | 결과·원본 근거 |
| --- | --- | --- | --- |
| `competing-span2-forward` | `40, 60, 60, 0` | `[2]` | 통과 · [log](ios-simulator-c114-multispan-ordering-ios-simulator-20261020T165531254Z-8ec3cd96/runtime-competing-span2-forward.log) |
| `competing-span2-reversed` | `40, 60, 60, 0` | `[2]` | 통과 · [log](ios-simulator-c114-multispan-ordering-ios-simulator-20261020T165531254Z-8ec3cd96/runtime-competing-span2-reversed.log) |
| `span2-before-span3` | `53.333332, 73.333336, 73.333336, 0` | `[2,3]` | 통과 · [log](ios-simulator-c114-multispan-ordering-ios-simulator-20261020T165531254Z-8ec3cd96/runtime-span2-before-span3.log) |
| `span3-before-span2` | `53.333332, 73.333336, 73.333336, 0` | `[2,3]` | 통과 · [log](ios-simulator-c114-multispan-ordering-ios-simulator-20261020T165531254Z-8ec3cd96/runtime-span3-before-span2.log) |

Chrome 155의 used-track 기대값은 1/64 CSS px 양자화 수치와 비교했다. 최대 track 차이는 `0.010415 CSS px`, 최대 item-frame 차이는 `0.005207 CSS px`다. runner가 전체 node frame, track 수·크기, collapse, intrinsic span stage, trace chunk completeness, draw/present를 확인했다. [Simulator metadata와 앱 hash](ios-simulator-c114-multispan-ordering-ios-simulator-20261020T165531254Z-8ec3cd96/simulator.json) · [검증 결과 JSON](ios-simulator-c114-multispan-ordering-ios-simulator-20261020T165531254Z-8ec3cd96/result.json) · [화면 캡처](ios-simulator-c114-multispan-ordering-ios-simulator-20261020T165531254Z-8ec3cd96/span3-before-span2.png).

이 실행은 iOS Simulator 근거이며 iPhone 실기기 또는 OS compositor 이후 표시 검증은 아니다.

## 구현 및 판정 범위

고정 Chrome oracle은 `spinon-layout` 순수 layout 회귀와 Android/iOS 앱 fixture에서 사용한다. Rust test는 최종 used tracks, 모든 item의 Minimum/MinContent/MaxContent contribution, contribution item/span, span stage 순서, 각 item frame을 검사한다. Android 실기기 앱 fixture는 node frame과 1/64 CSS px tolerance를 확인하고 전체 trace를 보존한다. iOS Simulator runner는 전체 frame·track·collapse와 intrinsic span stage 순서, trace completeness, draw/present를 검사한다.

이 결과는 제한된 고정 크기 direct Grid item의 multi-span ordering만 확인한다. mixed intrinsic/flexible/fixed track 조합, empty/hidden/nested/anonymous item, 다른 automatic minimum, cyclic percentage, arbitrary track grammar, WPT, 전체 CSS Grid 적합성은 검증하지 않았다. C11.4와 C11 상태는 미완료로 유지한다. [Rust/Android 구현 검토](implementation-review-20.md), [iOS fixture/Simulator 검토](ios-simulator-review-20.md), [최종 diff 검토](pr-diff-review-30.md)를 각각 기록했다.

## Fixture 빌드 경계

`mise exec -- cargo build --locked --release -p spinon-ffi --features c04-runtime-gpu` 제품 조합의 `target/release/libspinon_ffi.a`에서는 `strings` 검사로 multi-span scene ID가 나오지 않았다. `--features c04-runtime-gpu-fixture-test` opt-in 빌드에서는 scene ID들이 포함됐다. 이는 해당 정적 라이브러리 두 조합의 fixture 문자열 격리 확인이며 Android release APK 크기나 앱 번들 전체 검증으로 확대하지 않는다.

## 재현 명령

```sh
mise exec -- cargo test --locked -p spinon-layout grid_multispan_chrome_oracle_tests::chrome155_multi_span_contributions_are_independent_of_item_source_order
mise exec -- cargo test --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test multispan_ordering::tests::exposes_fixed_multispan_source_order_cases
mise exec -- env SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 bun run build:android
mise exec -- env SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 bun run build:ios-sim
node tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs ios-simulator ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D --scene-id=competing-span2-forward,competing-span2-reversed,span2-before-span3,span3-before-span2
```

기기 실행 때는 `adb -s <physical-device-serial>`로 대상 기기를 명시한다.
