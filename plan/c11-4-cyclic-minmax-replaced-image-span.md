# C11.4 · cyclic `minmax(calc())`와 replaced image multi-span

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [cyclic minmax max-content span-groups](./c11-4-cyclic-minmax-calc-cross-axis-max-content-span-groups.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** Chrome 156 Agent Browser 사전 관측, CSS Grid 2025 CRD 독립 기대값, 고정 HTML 입력과 계획 검토를 기록했다. 네 고정 scene의 Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator 검증이 모두 통과했다. PR 검토·필수 CI·병합과 공식 상태 갱신 전까지 C11.4 checkbox와 완료 수 `77/213`은 유지한다.

## 목적

기존 cyclic `minmax(calc())` multi-span slice는 text item을 사용했고, replaced image multi-span slice는 cyclic percentage track을 사용하지 않았다. 이 계획은 두 동작을 같은 Grid에서 결합해 intrinsic sizing의 cyclic percentage basis와 replaced image의 natural aspect ratio 기반 contribution이 겹치는 고정 입력 네 개를 다룬다.

CSS Grid 2025 CRD §7.2.1은 intrinsic sizing 동안 cyclic percentage track sizing function을 auto처럼 처리하고, 계산된 grid container size를 얻은 다음 percentage를 해석한다. §11.5–§11.5.1은 intrinsic track을 span 길이 순으로 sizing하고, 같은 span 그룹의 planned increase는 track별 최댓값을 적용한다. Chrome 관측은 구현 비교값이며 독립 기대값의 근거로 역산하지 않는다.

## 고정 입력과 네 장면

재현 입력은 [CSS fixture source](https://github.com/ohah/spinon/blob/main/tests/fixtures/css/c11/grid-track-sizing-c114-cyclic-minmax-replaced-image-precomparison.html)다. 가로세로 비율 3:2인 저장소 PNG [`c14-1-png-basic.png`](https://github.com/ohah/spinon/blob/main/tests/fixtures/resources/c14-1-png-basic.png)는 natural size `24×16px`, SHA-256 `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2`다. 모든 scene에서 `<img>`의 CSS `width:auto`이며 CSS height만 지정한다.

네 장면은 같은 3열 × 2행 Grid를 사용한다. 두 축의 track list는 `repeat(3, minmax(calc(10% + 2px), max-content))`와 `repeat(2, minmax(calc(10% + 2px), max-content))`이고, gap은 양 축 `2px`, root border는 `2px`, padding은 `0`이다. 모든 item은 direct replaced child이며 row를 1–3까지 span한다.

| Item | CSS height × auto width | Columns | Rows | 사용 목적 |
| --- | --- | --- | --- | --- |
| A | `8×12px` | `1 / 3` | `1 / 3` | span-2 contribution 12px |
| B | `12×18px` | `2 / 4` | `1 / 3` | 가운데 column을 공유하는 span-2 contribution 18px |
| C | `16×24px` | `1 / 4` | `1 / 3` | 앞선 span-2 group 뒤의 span-3 contribution 24px |

| Scene | Root sizing | DOM order |
| --- | --- | --- |
| `intrinsic-forward` | `width/height:min-content` | A, B, C |
| `intrinsic-reverse` | `width/height:min-content` | C, B, A |
| `definite-forward` | `width/height:80px` | A, B, C |
| `definite-reverse` | `width/height:80px` | C, B, A |

역순 장면은 DOM child 순서만 바꾼다. Identity, image source, CSS 크기, placement와 track expression은 그대로다.

고정 입력은 root에 `align-items:start`를 두고 각 replaced item의 `justify-self`와 `align-self`를 `auto`로 둔다. 이는 item의 used alignment를 start로 유지하면서 fixture-test inline CSS profile이 허용하지 않는 item-level `start` keyword를 일반 admission에 추가하지 않는다. 독립 Chrome 입력도 같은 선언을 사용한다.

## 독립 기대값과 관측

독립 산술, 중간 contribution, 최종 track 및 item frame은 [spec-derived-expectations.json](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-cyclic-minmax-replaced-image-span-precomparison-2026-10-23/spec-derived-expectations.json)에 기록했다.

Intrinsic sizing에서 cyclic percentage 성분을 처리할 때 2px 절대 길이를 유지하는 독립 입력 basis는 각 column과 row `[2,2,…]`다. Span-2 A의 12px contribution은 두 track에 각각 3px, B의 18px contribution은 각 6px planned increase를 제안한다. §11.5.1의 per-track maximum 후 columns는 `[5,8,8]`이다. Span-3 C는 이미 25px인 세 track과 두 gutter에 24px로 들어오므로 더 키우지 않는다. Rows는 초기 두 2px track과 2px gutter를 기준으로 A/B/C가 각각 track당 1/3/5px planned increase를 제안하므로 `[7,7]`이 된다. Content box는 `25×16px`, border box는 `29×20px`이다.

80px definite control은 두 축의 cyclic minimum을 각각 `10% × 80px + 2px = 10px`로 resolve한다. 모든 image contribution이 해당 span의 10px tracks와 gap 안에 들어오므로 columns `[10,10,10]`, rows `[10,10]`, content box `80×80px`, border box `84×84px`가 된다. item은 start-aligned라 intrinsic / definite scene 모두 자연 크기를 유지한다.

Chrome 156 Agent Browser 관측은 forward와 reverse에서 동일한 tracks와 identity별 frames를 반환했다. 관측 screenshot·JSON과 PNG 원본 hash는 [사전 비교 근거](../spec/internal/evidence/c11-4-cyclic-minmax-replaced-image-span-precomparison-2026-10-23/README.md)에 있다. 브라우저 값은 독립 규범 계산과 분리한다.

## 구현 관문

1. 기존 C11.4 fixture-test feature 안에 정확히 네 scene을 opt-in으로 추가한다. scene ID, DOM order, image bytes/hash, item key·intrinsic dimensions·CSS height, placement, 3×2 track list와 gap을 exact allowlist로 고정하고 다른 tree/style/source는 거부한다.
2. ImageResourceSnapshot과 replaced-item contribution 경로에서 image intrinsic dimensions와 specified height/auto width가 A/B/C의 12/18/24px inline contribution 및 8/12/16px block contribution으로 이어지는 것을 trace로 확인한다.
3. Cyclic intrinsic stage의 columns `[5,8,8]`, rows `[7,7]`와 80px control의 resolved minimum 및 `[10,10,10]` / `[10,10]` tracks를 독립 JSON에 대조한다. Span-2 A/B는 같은 planned-increase group에서 track별 maximum으로 결합하고 span-3 C는 뒤에서 처리해야 한다.
4. Root border boxes와 root-relative item frames를 scene·item key로 비교한다. Reverse order에서 track list, intrinsic dimensions, placement와 geometry가 같아야 한다. 허용 오차는 각 track/frame에서 1/64 CSS px다.
5. 기본 release artifact에서 새 fixture scene ID, preparation FFI symbol과 고정 image payload가 제외되고 opt-in fixture-test positive control에 포함되는지 각각 확인한다. 정적 archive 검사는 APK/IPA 전체 또는 출시 artifact를 증명하지 않는다.
6. 최신 source revision의 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 네 scene의 track·frame·trace·draw/present를 확인한다. 사용자가 시뮬레이터를 허용했으므로 실기기 연결은 요구하지 않는다. Android surface PixelCopy와 iOS compositor 제출 전 Metal drawable은 각 플랫폼에서 별도 표본으로 기록한다.
7. 기능 구현 후 새 runtime/code 실패 관점 20개와 PR diff 실패 관점 20개를 따로 기록한다. 발견 수정은 영향을 받는 scene, trace, artifact positive/negative control 및 simulator 관문을 재실행한다.
8. 내부 계약 0059, 상위 C11.4 계획, 공개 roadmap과 실행 evidence만 필요한 범위로 갱신한다. 새 일반 CSS admission이나 public API를 만들지 않으며 C11.4 checkbox와 공식 `77/213`은 유지한다.

## 성공·실패 판정

- 네 scene에서 loaded image natural size는 모두 24×16px이고 CSS 크기는 A 12×8, B 18×12, C 24×16px다. Image decode 실패, source hash mismatch, 다른 item count 또는 DOM topology는 scene을 거부한다.
- Intrinsic tracks는 columns `[5,8,8]`, rows `[7,7]`, root border box `29×20px`다. Definite tracks는 columns `[10,10,10]`, rows `[10,10]`, root border box `84×84px`다.
- Intrinsic item root-relative frames는 A `(2,2,12,8)`, B `(9,2,18,12)`, C `(2,2,24,16)`이다. Definite B의 x는 `14px`, A/C의 x는 `2px`이며 모든 item y는 `2px`다.
- Forward/reverse scene 값은 item identity를 기준으로 같아야 한다. source order에 따라 contribution이 누적되어 다른 track이 나오거나, row pass가 column sizing 전에 돌거나, cyclic percent가 border box 기준을 사용하면 실패다.
- Chrome screenshot은 browser oracle evidence다. 앱 runtime draw/present, border 픽셀 전체, WPT, 실기기·hardware GPU와 iOS compositor 이후 pixel equality로 확대 해석하면 실패다.
- 기본 artifact 제외와 positive control 포함 양쪽을 확인하지 않으면 fixture 격리가 검증되지 않은 것으로 판정한다.

## 제외 범위

네 고정 scene 밖의 임의 CSS parsing/admission, 다른 image format·intrinsic ratio·decode/error/lazy loading, image와 text 혼합, nested/anonymous/out-of-flow item, 다른 span topology, auto-repeat, arbitrary track grammar, WPT, 실기기/hardware GPU, 모든 border stroke 비교, iOS compositor 이후 pixels와 전체 C11.4 완료는 제외한다.

## 검토와 근거

- [Chrome 156 사전 비교·독립 기대값·PNG hash](../spec/internal/evidence/c11-4-cyclic-minmax-replaced-image-span-precomparison-2026-10-23/README.md)
- [계획 실패 관점 20개](./c11-4-cyclic-minmax-replaced-image-span-plan-review-20.md)
- [계획 PR diff 실패 관점 20개](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-cyclic-minmax-replaced-image-span-precomparison-2026-10-23/plan-pr-diff-review-20.md)
- 기존 [cyclic minmax max-content text 계획](./c11-4-cyclic-minmax-calc-cross-axis-max-content-span-groups.md), [image multi-span 계획](./c11-4-image-multispan-contributions.md)과 [internal 0059](../spec/internal/0059-c11-grid-layout.md)

## 구현 결과

네 고정 scene을 Runtime, FFI, iOS harness와 capture validator에 연결했다. Intrinsic forward/reverse는 columns `[5,8,8]`, rows `[7,7]`, root border box `29×20px`와 identity별 A/B/C frame을 확인한다. Definite forward/reverse는 columns `[10,10,10]`, rows `[10,10]`, root border box `84×84px`를 확인한다.

- Style/layout, Runtime scene matrix, FFI scene matrix focused test와 fixture-module boundary 18개가 통과했다. `cargo fmt --all -- --check`와 iOS/Android app build도 통과했다.
- 기본 `spinon-ffi` static archive에는 C11.4 fixture FFI symbol과 scene payload가 없고, `c04-runtime-gpu-fixture-test` archive에는 둘 다 있다. 이는 static archive 확인이며 APK/IPA 전체나 release artifact 증거가 아니다.
- iPhone 17 Pro / iOS 26.2 Simulator에서 최신 tracked source revision의 네 scene tracks, trace, identity별 node frame, draw/present가 통과했다. Metal drawable 표본은 scene마다 3/3이며 compositor 제출 전 값이다.
- Android API 37 ARM64 AVD에서도 최신 tracked source revision의 네 scene tracks, trace, identity별 node frame, draw/present와 Surface PixelCopy 표본이 통과했다. 전용 AVD의 API descriptor, 메모리·userdata 크기와 하드웨어 가속 설정을 바로잡아 boot 완료 및 앱 설치를 확인했다. PixelCopy 표본은 scene마다 3/3이다.
- 두 플랫폼 run metadata, app hash, screenshot과 원본 JSON은 [runtime evidence](../spec/internal/evidence/c11-4-cyclic-minmax-replaced-image-span-runtime-2026-10-23/README.md)에 둔다. AVD/iOS Simulator 결과는 실기기·hardware GPU 결과로 확대하지 않는다.
- [구현 runtime 실패 관점 20개](../spec/internal/evidence/c11-4-cyclic-minmax-replaced-image-span-runtime-2026-10-23/runtime-code-review-20.md)와 [구현 diff 실패 관점 20개](../spec/internal/evidence/c11-4-cyclic-minmax-replaced-image-span-runtime-2026-10-23/pr-diff-review-20.md)를 계획 검토와 분리해 기록했다. Root min/max/aspect-ratio 검사와 Android host 장면·PixelCopy·reversed input 누락을 발견해 보강했다.

시뮬레이터 관문은 완료했다. PR의 적대적 검토와 필수 CI, 병합 및 그에 따른 공식 상태 갱신 전까지 전체 기능 또는 C11.4 완료로 표시하지 않는다. 이 계획은 C11.4 checkbox와 공식 `77/213`을 변경하지 않는다.
