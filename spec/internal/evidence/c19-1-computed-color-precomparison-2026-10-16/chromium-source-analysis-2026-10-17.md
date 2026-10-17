# Chrome 155 OKLCH sRGB 변환 소스 확인

## 확인 대상

- C19.1 oracle: Google Chrome `155.0.8059.85`, macOS 26.5.1 arm64, Canvas 2D `srgb` / `unorm8`.
- Agent Browser에서 확인한 `chrome://version`: Chromium revision `0e66b94549a8bd85f03fbdf189b752d6e7b63c32`, `refs/branch-heads/8059@{#1304}`. 기존 capture metadata의 `Browser.getVersion.webKitVersion` 문자열에도 같은 Chromium revision이 포함되어 있다.
- 확인한 예제: `oklch(60% 0.15 230deg)`. Chrome Canvas 표본은 `[0,142,200,255]`, Stylo 0.22 CSS Color 4 Local MINDE는 `[0,142,196,255]`다.

## Chromium paint 변환 경로

동일 revision의 공개 Chromium 소스에서 다음을 확인했다.

1. Blink [`Color::toSkColor4f()`](https://chromium.googlesource.com/chromium/src/+/0e66b94549a8bd85f03fbdf189b752d6e7b63c32/third_party/blink/renderer/platform/graphics/color.cc#839)는 `ToSRGB(IsBakedGamutMappingEnabled())`를 호출한다. `IsBakedGamutMappingEnabled()`는 `kBakedGamutMapping`의 `FeatureList` 상태를 읽는다. [`color.cc` 575, 618, 839, 853행](https://chromium.googlesource.com/chromium/src/+/0e66b94549a8bd85f03fbdf189b752d6e7b63c32/third_party/blink/renderer/platform/graphics/color.cc#575).
2. 같은 revision의 [`features.cc` 229행](https://chromium.googlesource.com/chromium/src/+/0e66b94549a8bd85f03fbdf189b752d6e7b63c32/third_party/blink/common/features.cc#229)은 `kBakedGamutMapping`을 `FEATURE_DISABLED_BY_DEFAULT`로 선언한다.
3. [`color_conversions.cc` 120, 227행](https://chromium.googlesource.com/chromium/src/+/0e66b94549a8bd85f03fbdf189b752d6e7b63c32/ui/gfx/color_conversions.cc#120)의 `OklabGamutMap()`은 고정 lightness·hue에서 chroma를 줄여 **Rec.2020 gamut의 polyhedral 근사** 안에 넣는다. `OklabToXYZD50()`은 `gamut_map`이 true일 때만 이 단계를 호출한다.
4. `gamut_map`이 false이면 OKLCH→OKLab→XYZ-D50 변환 뒤 [`XYZD50ToSRGB()`](https://chromium.googlesource.com/chromium/src/+/0e66b94549a8bd85f03fbdf189b752d6e7b63c32/ui/gfx/color_conversions.cc#282)가 sRGB 행렬과 transfer function으로 바꾼다. 이 변환 함수 자체에는 CSS Color 4 Local MINDE sRGB 경계 탐색이 없다.

해당 Chrome 실행의 `chrome://version` command line에는 `BakedGamutMapping` 활성화 override가 없었다. 다만 capture 당시에 활성화된 Finch variation ID를 feature 이름까지 연결해 보존하지 않았으므로, 원본 capture 프로세스의 최종 `FeatureList` 값을 직접 측정한 증거는 아니다. 따라서 이 분석은 정확한 바이너리 revision의 소스 기본 경로와 현재 동일 버전 Agent Browser 실행의 command line에 근거한다. `[0,142,200]` 관찰값은 이 무매핑 sRGB 변환 경로와 일치하지만, Finch override 부재까지 소급 증명한다고 주장하지 않는다.

## CSS Color 4 기준 판정

2026-10-09 공개된 [CSS Color 4 Candidate Recommendation Draft §14.2](https://www.w3.org/TR/2026/CRD-css-color-4-20261009/#css-gamut-mapping-to-an-rgb-destination)는 SDR 색상이 RGB 디스플레이 또는 sRGB `unorm8` 캔버스의 gamut 밖이면 CSS gamut mapping을 요구한다. 구현은 Local MINDE, EdgeSeeker, Ray Trace 중 하나를 선택할 수 있고 선택한 알고리즘은 해당 매핑이 요구되는 경로에서 사용해야 한다. 세 방식은 모두 OkLCh에서 lightness와 hue를 유지하며 chroma를 줄이는 것을 목표로 한다. Stylo의 `[0,142,196]`은 Local MINDE 구현 표본이다.

이 사실만으로 Chrome의 `[0,142,200]`이 잘못됐다고 판정할 수는 없다. Chromium `refs/branch-heads/8059` 공개 소스에서 Blink `Color::toSkColor4f()`는 Finch `kBakedGamutMapping` 상태를 읽어 `ToSRGB()`에 전달한다. 같은 branch의 해당 feature 기본값은 disabled이며, `OklabGamutMap()`은 Rec.2020 gamut의 polyhedral approximation을 사용한다. feature가 비활성인 경우 그 helper를 거치지 않고 `XYZD50ToSRGB()`로 변환한다. 이는 CSS Color 4 Local MINDE 구현과 다른 경로이므로 관찰값의 의심스러운 원인 후보지만, 이것만으로 전체 Canvas 2D paint/readback 경로의 적합성을 확정하지 않는다.

원본 Chrome 캡처에는 Finch의 최종 feature assignment와 전체 Canvas 변환 경로가 보존되지 않았다. 따라서 이번 기록의 결론은 **Chrome 대 Stylo 불일치 관찰은 확정, 어느 한쪽의 오류 판정은 보류**다. 판정하려면 동일한 Chrome 155 revision에서 feature state를 고정해 재현하고, sRGB `unorm8` 결과를 Local MINDE·EdgeSeeker·Ray Trace 허용 결과 및 downstream clipping/conversion과 비교해야 한다. 이 CR Draft는 작업 중인 문서이며 결과는 해당 버전·플랫폼·경로에 한정한다.

## Stylo와 값이 다른 이유

Stylo 표본은 CSS Color 4 Local MINDE로 sRGB gamut 안에 들어오며, Chrome 표본은 다른 최종 RGB를 기록했다. 소스는 Blink의 feature-gated 변환 경로에서 차이가 생길 수 있음을 보여 주지만, `toSkColor4f()`가 이 Canvas 2D 입력에서 실제로 사용되었는지와 저장소의 마지막 변환·clipping 단계는 원본 실행 자료로 확인되지 않았다. 그러므로 차이를 parse 또는 alpha compositing 문제라고 단정하지 않으며, 반대로 Chrome이 유효한 다른 CSS 알고리즘을 썼다고도 단정하지 않는다.

C19.1 oracle에는 Chrome 155의 역사적 관측값 `[0,142,200,255]`와 Stylo Local MINDE 비교값 `[0,142,196,255]`를 별개로 보존한다. 이 표본은 비교 자료이지 CSS Color 4 conformance 판정은 아니다. 재현에서는 실제 feature list와 Canvas 경로를 기록하고, 세 허용 알고리즘 각각의 sRGB `unorm8` 결과를 함께 보존한다.

## 소스 파일 식별자

| 파일 | Git blob | 관련 위치 |
|---|---|---|
| `third_party/blink/renderer/platform/graphics/color.cc` | `f0e33b2e98896803209e3e091524d5b1eb38fc74` | `ToSRGB` 575, `ToXYZD50` 618, `toSkColor4f` 839, feature query 853 |
| `third_party/blink/common/features.cc` | `7caf6102fbb68fbe935595536b9975d42ac654b5` | default-disabled feature 229 |
| `ui/gfx/color_conversions.cc` | `aaa3909e8eef9fce4610665e29b9fb888a6a5d7c` | Rec.2020 gamut map 120, OKLab conversion 227, XYZ-D50→sRGB 282 |
