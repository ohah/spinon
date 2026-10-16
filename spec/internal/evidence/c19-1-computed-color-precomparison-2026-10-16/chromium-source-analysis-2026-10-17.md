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

2026-10-09 공개된 [CSS Color 4 Candidate Recommendation Draft §14.2](https://www.w3.org/TR/2026/CRD-css-color-4-20261009/#css-gamut-mapping-to-an-rgb-destination)는 SDR 색상이 RGB 디스플레이 또는 sRGB `unorm8` 캔버스의 gamut 밖이면 CSS gamut mapping이 필요하다고 적는다. 구현은 Local MINDE, EdgeSeeker, Ray Trace 중 품질·비용에 맞는 방식을 고를 수 있지만, gamut mapping이 필요한 곳에서는 선택한 방식을 사용해야 한다. Stylo의 Local MINDE는 허용된 방식 중 하나다.

따라서 이 표본에서 Stylo `[0,142,196]`은 현재 CR Draft에 적힌 gamut mapping 방식과 일치한다. Chrome `[0,142,200]`은 그 결과와 다르고, 확인한 Blink의 기본 매핑 비활성 변환 경로 및 단순 채널 clipping 관측과 일치하므로 **현재 CSS Color 4 기준에서는 Chrome 출력이 벗어난 것으로 보인다**. 문서는 Candidate Recommendation Draft로 아직 작업 중이며, capture 때의 Finch assignment도 보존되지 않았다. 이 판정은 해당 Chrome 155/macOS/sRGB/unorm8 표본에 한정하고, 모든 Chrome 플랫폼·경로에 일반화하지 않는다.

## Stylo와 값이 다른 이유

Stylo 결과 `[0,142,196]`은 CSS Color 4 Local MINDE를 적용해 OKLCH의 lightness와 hue를 유지하면서 sRGB 경계 안으로 chroma를 줄인 값이다. Chrome oracle `[0,142,200]`은 같은 paint-stage Local MINDE 출력을 쓰지 않는다. Chromium 소스는 기본 feature가 꺼져 있을 때 OKLCH를 sRGB로 변환하는 paint 경로에서 Local MINDE를 실행하지 않는다는 점을 설명한다. 결과 차이는 입력 parse나 alpha compositing 오류의 증거가 아니라 wide-gamut 색을 sRGB 8-bit Canvas에 표현하는 방식의 차이다.

이 표본만으로 모든 Chrome platform/backend의 gamut mapping 동작, Finch 상태, 또는 Chrome이 CSS Color 4의 특정 대체 알고리즘을 사용한다고 단정할 수 없다. C19.1 oracle는 원본 Chrome 155 macOS/sRGB/unorm8 관찰값으로 고정하고, Stylo Local MINDE 결과를 별도 typed value로 계속 보존한다.

## 소스 파일 식별자

| 파일 | Git blob | 관련 위치 |
|---|---|---|
| `third_party/blink/renderer/platform/graphics/color.cc` | `f0e33b2e98896803209e3e091524d5b1eb38fc74` | `ToSRGB` 575, `ToXYZD50` 618, `toSkColor4f` 839, feature query 853 |
| `third_party/blink/common/features.cc` | `7caf6102fbb68fbe935595536b9975d42ac654b5` | default-disabled feature 229 |
| `ui/gfx/color_conversions.cc` | `aaa3909e8eef9fce4610665e29b9fb888a6a5d7c` | Rec.2020 gamut map 120, OKLab conversion 227, XYZ-D50→sRGB 282 |
