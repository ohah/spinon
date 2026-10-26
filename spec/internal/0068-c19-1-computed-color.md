# 0068 · C19.1 computed color carrier

**계약 버전:** `0.1.0` 고정

**상태:** 내부 Rust cascade profile; `color`·`background-color` carrier 전용 직렬화, GPU paint와 공개 CSS/DOM API에는 연결하지 않음

**상위 계획:** [C19 기본 페인트 속성](https://github.com/ohah/spinon/blob/main/plan/c19-basic-paint-attributes.md)

**구현 profile:** `C19ComputedColorV1`

## 계약

- Stylo `AbsoluteColor`를 CSS 색 parser로 다시 구현하지 않고 사용한다. snapshot은 원래 색 공간, 해당 공간의 세 component, 유효 alpha, 세 missing-component bit, missing-alpha bit, legacy-syntax bit를 보존한다.
- `converted_srgb`는 Stylo가 계산한 색을 encoded sRGB로 변환한 값이다. 0..1 밖의 RGB를 허용하며, 이 값 자체를 CSS gamut mapping 결과라고 부르지 않는다.
- `gamut_mapped_srgb`는 sRGB 범위 안 색은 직접 변환하고, 범위 밖 색은 Stylo `AbsoluteColor::gamut_map_binary_search(ColorSpace::Srgb)`로 변환한 결과다. Stylo 0.22 Local MINDE 결과는 별도 유지하며 Chrome 결과로 덮어쓰지 않는다. CSS Color 4는 sRGB 출력에서 gamut mapping을 요구하는 경우 세 알고리즘 중 하나를 사용하도록 정의한다. [CSS Color 4 §14.2](https://drafts.csswg.org/css-color-4/#css-gamut-mapping)
- `converted_srgb_clipped_rgba8()`는 `converted_srgb`를 채널별 0..1 범위로 제한하고 8-bit로 반올림한다. 이 경로는 Chrome 155 sRGB/unorm8 관찰값과 맞추기 위한 surface quantization이며 CSS Color 4 gamut mapping 결과가 아니다. Chromium source는 기본 색 렌더 경로에서 gamut-mapping feature를 확인하는 코드를 두며, 해당 feature는 source에서 기본 비활성으로 선언되어 있다. 실제 binary의 source revision과 모든 실행 flag를 이 contract가 보증하지 않는다. [Chromium color.cc](https://chromium.googlesource.com/chromium/src/+/HEAD/third_party/blink/renderer/platform/graphics/color.cc) · [Chromium features.cc](https://chromium.googlesource.com/chromium/src/+/HEAD/third_party/blink/common/features.cc)
- `currentColor`는 cascade에서 계산된 element의 `color` 값을 사용한다. runtime mount에서는 합성 `html/body`도 cascade해 body에서 상속되는 색을 mount root에 전달한다. C19 profile은 이 계산에 layout geometry를 검증하지 않는다. invalid CSS declaration은 CSS parser가 버리고 앞선 유효 선언을 남기며, 이 과정에서 runtime `CssParseDiagnostic` 생성을 요구하지 않는다.
- CSS system colors는 Stylo Servo device가 제공하는 fixed light/dark palette로 계산한다. OS/브라우저별 system palette 수집은 구현되지 않았다. Chrome 155/macOS의 `ButtonFace`와 `Highlight`는 Stylo Servo 값과 다르므로 그 차이를 색 변환 정확도나 Chrome pixel pass로 분류하지 않는다.

## 검증 경계

- pinned Chrome 155 fixture 44행 중 유효 41행, invalid control 3행을 검사한다. 37개 비-system 유효색의 clipped sRGB RGBA8은 Chrome observation과 exact 일치한다. system-color 4행은 현재 Stylo Servo palette에 대해 별도로 단언한다.
- unit test는 missing component/alpha 보존, non-finite source 거부, mutation된 gamut-mapped 범위 거부, Display-P3 out-of-gamut 입력의 CSS Color 4 Local MINDE 결과, Chrome compatibility용 sRGB surface 변환을 분리해 확인한다. runtime mount의 body 상속 사전 실패는 `c19-runtime-body-inheritance-before-fix.log`에 보존했다. [Chrome precomparison](./evidence/c19-1-computed-color-precomparison-2026-10-16/README.md) · [호환성 차이 분류](./evidence/c19-1-computed-color-compatibility-triage-2026-10-16.md)
- Agent Browser screenshot은 기준 Chrome fixture의 화면 증거다. 이 profile은 실제 Android/iOS surface, GPU compositing, alpha blend/group opacity, text glyph, hit-test, contrast, OS system-color parity를 증명하지 않는다.

profile은 computed `color`와 `background-color` 두 Longhand만 `properties`에 직렬화한다. typed `computed_color`·`computed_background_color` carrier가 색 출처와 변환 결과의 기준이며 legacy `background_color`·`background_paint`는 계속 `None`이다. author declaration 허용 목록과 CSS 계산은 이 출력 목록과 별개로 유지한다.

C19 incremental reuse API는 같은 `C19ComputedColorV1` profile·viewport/media environment·document generation을 유지하고 이전 document snapshot보다 새 문서 revision을 계산할 때 이 profile을 지원한다. `dirty_root_ids`에 전달한 inline-style dirty subtree는 다시 cascade하고 필요한 상위 context를 계산한 뒤, 영향을 받지 않는 이전 직렬화 출력을 재사용한다. 이 API가 받는 author stylesheet는 비어 있어야 하며, 이전 profile/viewport/generation 불일치, 유효하지 않은 dirty root, `style_revision`이 같거나 바로 다음 값이 아닌 경우에는 `None`을 반환한다. 호출자는 해당 경우 전체 cascade로 fallback해야 한다. 이 reuse 경로도 GPU/paint를 구현하지 않는다.

이 계약은 C19.1 carrier 경계에만 적용한다. C19 전체는 C15/C16 text painter, GPU paint, hit-test와 contrast 통합 전에는 완료되지 않는다.
