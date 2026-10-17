# 0069 · C19.2 box paint와 opacity

**계약 버전:** `0.1.0` 고정

**상태:** 내부 runtime paint 제한 구현. C19 전용 opt-in profile의 Android API 37 AVD와 iOS 26.2 Simulator 표본은 제품 기대값 16/16으로 통과했다. C08 partial-alpha 회귀를 발견해 기존 profile과 C19.2 profile을 분리하고 영향 경계를 재검토했다. PR #232는 2026-10-16에 병합됐고 [PR diff 적대 검토 30개](./evidence/c19-2-pr-diff-review-30-2026-10-17.md)를 기록했다. 2026-10-17 GitHub 재조회에서 PR status check rollup은 비어 있었으므로 full CI 통과로 간주하지 않는다. C19.2의 제한과 parent C19 미완료 상태는 유지한다.

**상위 계획:** [C19 기본 페인트 속성](https://github.com/ohah/spinon/blob/main/plan/c19-basic-paint-attributes.md)

**구현 profile:** `C19BoxPaintOpacityV1` cascade·layout·paint 경로, `RuntimeRenderSnapshot` paint command tree 및 C19.2 opt-in app fixture

## 계약

- C19 전용 `C19BoxPaintOpacityV1` cascade profile의 resolved color/alpha를 runtime render snapshot까지 전달한다. C08/C13 등 이전 profile은 legacy color conversion과 partial-alpha 거부 계약을 유지한다. Paint command는 box draw와 `opacity` group begin/end를 구분한다. 부모 opacity는 subtree를 먼저 합성한 뒤 group 전체에 적용하고, 자식마다 분배하지 않는다.
- background와 지원되는 border paint는 `RuntimePaint::{Opaque,Color}` 입력을 사용한다. CSS color alpha와 element opacity는 독립된 값으로 유지한다. `currentColor`는 현재 element에서 cascade된 `color`를 사용한다.
- paint order는 snapshot의 stacking order를 보존한다. clip은 geometry 생성 때 적용되며 group 안 child overlap은 하나의 opacity layer에서 합성한다. nested opacity group은 순서대로 중첩한다.
- sRGB-compatible surface의 8-bit 표본을 고정 Chrome 155 observation과 비교한다. C19.1의 `oklch(60% 0.15 230deg)` 표본은 제품 Stylo 0.22 Local MINDE `[0,142,196,255]`, Chrome 155 관찰값 `[0,142,200,255]`로 별도 보존한다. 알고리즘 차이를 숨기거나 oracle을 사후 변경하지 않는다.
- C19.2 앱 route, fixture payload와 FFI entry는 `c19-2-box-paint-opacity-fixture-test` opt-in feature에 한정한다. 기본 Android ARM64 build에서 C19.2 FFI symbol/payload와 JNI fixture entry가 없어야 하며 positive control에는 있어야 한다.

## 제한과 실패 경계

- 내부 opacity layer의 최대 중첩 깊이는 64, layer 합계 크기는 256 MiB다. 제한 초과는 draw 전에 오류로 반환한다.
- C19.2 고정 표면 표본 readback은 CSS viewport `360×240`, 16개 내부/overlap/clip 표본을 사용한다. Android drawable은 균일 scale `1..8`의 pixel center 변환을 허용하며 정수 scale에 한정하지 않는다.
- Android API 37 AVD는 GL/ANGLE/SwiftShader를 선택했다. Android surface는 `COPY_SRC`를 제공하지 않아 별도 offscreen WGPU readback과 PixelCopy를 사용한다. 기존 Rgba8 fixed-function 경로는 두 readback 모두 제품 exact `8/16`이었으나, float accumulator와 8-bit premultiplied source contribution을 사용하는 후보 경로는 두 경로 모두 `16/16`이다. 실행 결과와 baseline mismatch는 [Simulator evidence](./evidence/c19-2-runtime-simulators-2026-10-17/README.md)에 함께 기록한다.
- iPhone 17 Pro / iOS 26.2 Simulator (`1080×720`, 3×)와 iPad Pro 13-inch M5 / iOS 26.2 Simulator (`720×480`, 2×)의 Metal drawable 직접 readback은 각각 제품 oracle `16/16`, Chrome exact `15/16`이다. drawable readback은 compositor 이후 캡처가 아니다.
- 정밀 compositor는 이미지가 없는 scene과 `Rgba8Unorm`/`Bgra8Unorm` surface format으로 한정한다. sRGB 인코딩된 `Rgba8UnormSrgb`/`Bgra8UnormSrgb` format은 정확성을 검증하지 않았으므로 기존 그룹 합성 경로를 사용한다. 정확한 표본 gate 통과는 Android hardware GPU, Android/iOS 실기기, iOS compositor-after 결과나 다른 GPU backend의 동등성을 보장하지 않는다. hit-test 불변성, contrast, text glyph 통합도 아직 보장하지 않는다. 이 제한과 남은 적대적 검토 때문에 C19.2와 parent C19를 완료 처리하지 않는다.

## 검증 근거

- [Android/iOS Simulator 실행과 원인 분리](./evidence/c19-2-runtime-simulators-2026-10-17/README.md)
- [구현 실패 관점 20개와 C08 회귀 수정](./evidence/c19-2-runtime-implementation-review-2026-10-17.md)
- 후보 경로의 Android PixelCopy·offscreen 로그와 PNG, iOS Metal sample JSONL/manifest·로그·screenshot은 같은 증거 디렉터리에 있다.
- C19.2 focused WGPU tests는 group-depth/texture-extent/layer-memory-cap/surface-format 경계 4개와 runtime opacity 3개가 통과했다. C19.2 FFI focused tests 2개, C08 partial-alpha 거부 negative control, workspace tests와 Clippy도 통과했다. Android opt-in debug APK 빌드와 API 37 AVD PixelCopy/offscreen, iOS Simulator app build와 iPhone 17 Pro·iPad Pro 13-inch 재실행도 통과했다. 기본/opt-in artifact 경계 결과는 별도 README에 남겨 두었다. 이 검증은 아직 C19.2 또는 parent C19 전체 완료를 뜻하지 않는다.
