# C19.1 computed color 호환성 차이 분류

## 측정 범위

- Agent Browser 0.31.1이 실행한 Google Chrome 155.0.8059.85의 고정 fixture 44행(유효 41, invalid control 3)을 사용했다. 실행 환경과 바이너리 SHA-256은 [사전 비교 기록](c19-1-computed-color-precomparison-2026-10-16/README.md)에 있다.
- 기준은 CSS computed color를 sRGB/unorm8 Canvas에 그린 RGBA8이다. 반투명 색의 RGB는 premultiply/unpremultiply에서 생기는 ±1 손실을 피하려고 별도 opaque CSS relative-color probe에서 읽고, alpha는 원본 Canvas 표본에서 가져온다. Agent Browser 전체 화면 표본은 CSS box의 실제 출력으로 별도 보존한다. 초기 비교 로그는 `stylo-chromium-comparison.log`다.
- 이 검사는 Chrome 동작 비교다. CSS Color 4 gamut-mapping 적합성을 판정하는 WPT 실행은 포함하지 않는다.

## 재현 결과

HSL 무채색 `hsl(240deg 0% 50%)`에서 Stylo의 OKLCh 왕복은 `[128,127,128,255]`를 만들었다. sRGB 안에 이미 있는 색을 직접 변환하도록 바꾸자 `[128,128,128,255]`가 되어 Chrome과 일치했다.

Stylo Local MINDE 결과를 Chrome 화면 바이트와 직접 비교했을 때 12행이 달랐다. 이 표는 parser/cascade 오류 목록이 아니라 두 sRGB 출력 정책의 차이다.

| 입력 | Stylo 0.22 CSS Color 4 Local MINDE | Chrome 155 기본 sRGB 출력 | 분류 |
|---|---:|---:|---|
| `oklch(60% 0.15 230deg)` | `[0,142,196,255]` | `[0,142,200,255]` | sRGB 출력 변환 |
| `color(display-p3 1 0 0)` | `[255,11,12,255]` | `[255,0,0,255]` | wide-gamut 출력 변환 |
| `color(display-p3 1 0.4 0)` | `[255,100,0,255]` | `[255,87,0,255]` | wide-gamut 출력 변환 |
| `color(display-p3 1 none 0 / 0.5)` | `[255,11,12,128]` | `[255,0,0,128]` | missing component와 wide-gamut 출력 |
| `color(prophoto-rgb 0.2 0.4 0.8)` | `[0,122,154,255]` | `[0,130,224,255]` | wide-gamut 출력 변환 |
| `color(rec2020 0.2 0.4 0.8)` | `[0,120,203,255]` | `[0,119,218,255]` | wide-gamut 출력 변환 |
| `color(xyz-d50 0.2 0.4 0.8)` | `[0,188,213,255]` | `[0,203,255,255]` | XYZ→sRGB 출력 변환 |
| `color(xyz-d65 0.2 0.4 0.8)` | `[0,186,197,255]` | `[0,202,228,255]` | XYZ→sRGB 출력 변환 |
| `color(xyz 0.2 0.4 0.8)` | `[0,186,197,255]` | `[0,202,228,255]` | XYZ alias; 같은 변환 차이 |
| 상속 `currentColor` P3 | `[250,0,101,255]` | `[251,0,101,255]` | wide-gamut 출력 변환 |
| `ButtonFace` | `[220,220,220,255]` | `[239,239,239,255]` | system-color 환경 |
| `Highlight` | `[0,65,198,255]` | `[128,188,254,153]` | system-color 환경·alpha |

현재 Stylo Servo device는 system color를 고정 팔레트로 계산한다. 이는 Chrome/macOS의 system palette와 다르며 viewport의 `color-scheme: light`만으로 맞춰지지 않는다. CSS invalid declaration 3개는 앞선 유효 `color`와 transparent `background-color`를 보존한다. cascade는 의도대로 복구했으며, inline CSS parse discard에는 `CssParseDiagnostic`이 생성되지 않아 해당 진단을 요구하지 않는다.

두 결과를 구분해 carrier가 원본 색 공간, 매핑 전 sRGB 변환, CSS Color 4 Local MINDE 출력을 모두 보존하도록 했다. Chrome 기본 렌더 출력 비교는 매핑 전 sRGB 값을 unorm8로 양자화하며, 37개 비-system 유효색이 모두 Chrome 155와 정확히 일치한다. 네 system-color 행은 해당 Stylo Servo palette와 비교한다. 이 테스트는 Chrome macOS의 system palette를 재현했다고 주장하지 않는다. 성공 로그는 `chromium-srgb-compatibility-test.log`다.

runtime mount의 `body { color: color(display-p3 0.9 0.1 0.4) }`와 root `color: inherit` 회귀 테스트는 초기 profile이 합성 `html/body` 상속을 생략해 sRGB로 잘못 계산하는 점을 재현했다. C19 cascade에 합성 document style context를 추가하고 layout 전용 geometry validation은 유지하지 않아 Display-P3 inheritance와 `[251,0,101,255]` Chrome 출력이 통과한다. 수정 전 log는 `c19-runtime-body-inheritance-before-fix.log`다.

## 해석 경계

- CSS Color 4는 sRGB/unorm8 등 출력 gamut 밖 색에 대해 gamut mapping을 요구하고, 세 알고리즘 중 하나를 구현하도록 설명한다. 값은 script에 직접 노출되지 않아 자동 테스트가 어렵다고도 명시한다. [CSS Color 4 §14.2](https://drafts.csswg.org/css-color-4/#css-gamut-mapping)
- 현재 Chromium source의 `Color::toSkColor4f()`는 `ToSRGB(IsBakedGamutMappingEnabled())`를 호출한다. `kBakedGamutMapping`은 feature source에서 기본 비활성으로 선언되어 있다. [Chromium color.cc](https://chromium.googlesource.com/chromium/src/+/HEAD/third_party/blink/renderer/platform/graphics/color.cc) · [Chromium features.cc](https://chromium.googlesource.com/chromium/src/+/HEAD/third_party/blink/common/features.cc). 두 링크는 HEAD source이며, 캡처된 설치 바이너리의 exact source revision이나 실행 feature list를 증명하지 않는다.
- 따라서 Chrome RGBA8과 Stylo Local MINDE는 같은 기대값으로 취급할 수 없다. 제품 출력 정책을 고르기 전에는 C19.1의 exact Chrome pixel check를 통과로 기록하지 않는다. system-color 차이도 변환 알고리즘 차이와 합치지 않는다.

## 상태

이 기록은 구현 중 발견한 차이 분류다. carrier는 Chrome clipped output과 CSS Color 4 Local MINDE output을 모두 보존하고, 이후 paint consumer가 경로를 고르도록 한다. system-color 환경 차이는 별도로 남긴다. `spec/STATUS.md`의 C19 완료 checkbox는 그대로 미완료다. PR 전에는 서로 다른 실패 관점의 구현 검토와 별도 PR 변경 검토를 기록한다.
