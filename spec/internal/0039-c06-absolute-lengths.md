# 0039 · C06.3 절대 길이 단위와 CSS px 정규화

- **상태:** 내부 runtime 구현 계약 · 공개 웹 CSS 지원 선언 아님
- **내부 계약 숫자 버전:** 출시 전 `0.1.0` 고정
- **문서 식별자:** `0039`는 내부 문서 번호이며 제품·계약 숫자 버전이 아니다.
- **기준:** Chrome `154.0.8037.98` · Stylo `0.22.0` · Taffy `0.14.0`
- **계획:** [C06.3 절대 길이 단위 계획](../../plan/c06-3-absolute-lengths.md)
- **비교 기준:** [고정 Chromium oracle](../../tests/fixtures/css/references/c06-absolute-lengths-v1.json)
- **구현 검토와 실행 근거:** [구현 실패 경로 20개와 Simulator 기록](evidence/c06-absolute-lengths-implementation-2026-10-10.md)

## 지원 범위

현재 runtime Flex author-style profile에 이미 연결된 `width`, `height`, `flex-basis`, physical `margin`, `padding`, `row-gap`, `column-gap`의 절대 CSS 길이 `px`, `in`, `cm`, `mm`, `Q`, `pt`, `pc`를 다룬다. 새로운 property allowlist나 authored-unit parser를 추가하지 않는다. Stylo computed value가 절대 길이를 CSS px로 정규화하고 Spinon은 typed `CSSPixelLength::px()`를 읽어 기존 `LengthPx` layout DTO와 Taffy 입력에 전달한다.

| authored 값 | computed CSS px 예시 | 의미 |
|---|---:|---|
| `96px` | `96px` | CSS pixel |
| `1in` | `96px` | `1in = 96 CSS px` |
| `2.54cm` | `96px` | CSS 기준 인치의 센티미터 환산 |
| `25.4mm` | `96px` | CSS 기준 인치의 밀리미터 환산 |
| `101.6Q` | `96px` | 4분의 1 밀리미터 단위 |
| `72pt` | `96px` | CSS point |
| `6pc` | `96px` | CSS pica |

단위 환산에는 장치 DPI, Android `dp`, iOS point, drawable pixel 또는 device pixel ratio를 사용하지 않는다. 동일 CSS viewport에서 DPR만 달라져도 layout CSS frame은 같아야 한다. 이 계약은 화면의 실제 물리 길이, font sizing·text shaping, 일반 브라우저 전체 호환성을 보장하지 않는다.

## 값 계산과 실패 경계

- 계산 문자열을 재파싱하지 않는다. cascade 결과의 typed `Length`를 `px()`로 읽고 finite 여부를 검사한다.
- 크기 값은 finite nonnegative만 layout에 전달한다. signed margin은 음수 값을 보존한다. padding과 gap은 nonnegative 규칙을 따른다.
- `var()`가 Stylo에서 절대 길이로 계산되면 그 typed CSS px 값을 그대로 사용한다.
- Stylo가 상수 절대식 `calc(1in)`을 computed CSS px로 해소하면 그 결과를 사용한다. mixed-unit `calc(1in + 10%)`처럼 계산 의미가 남는 `Calc` variant는 C06.5 전까지 `UnsupportedComputedValue`로 실패한다.
- 알 수 없는 authored unit은 CSS parser/cascade의 invalid declaration 처리를 따른다. 유효한 앞선 선언이 있으면 그것을 보존한다. 지원하지 않는 값이나 비유한 typed 값을 임의로 0 또는 px로 바꾸지 않는다.
- CSS font-relative 단위, viewport/container 단위, CSSOM, border/text layout, Grid, 외부 CSS 로더는 이 계약에서 다루지 않는다.

## 비교 기준 및 허용 오차

고정 fixture는 Chrome `154.0.8037.98` revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`에서 26개 노드의 CSSOM computed value와 frame을 저장한다. Rust 비교는 각 node의 `x`, `y`, `width`, `height`를 개별적으로 비교하고 각 축의 최대 허용 오차는 `0.5 CSS px`다. 7개 단위가 크기·flex basis·spacing 입력에 걸쳐 모두 `96`, `48`, `6`, `3 CSS px` 동치 결과를 보이고, `-4.5pt` margin은 `-6 CSS px`, `2.54cm` custom property는 `96 CSS px`가 되는지 typed 값으로 별도 확인한다.

### Runtime fixture

실제 JS runtime fixture는 26개 노드 장면을 생성한다. Android API 37 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 다음을 확인한다.

- V8 evaluation `status=0`
- `layout=ready`, `boxes=26`
- C06.3 scene가 Android SurfaceView·iOS UIKit canvas를 통해 WGPU draw path까지 도달하고 `presented` 결과를 반환
- 화면 캡처에 색상별 단위 비교 행이 보임

Android emulator는 실행 당시 GL/ANGLE과 SwiftShader software renderer를 사용했다. iOS 결과는 Metal backend의 Simulator 결과다. 어느 쪽도 실기기 hardware GPU 성능이나 전체 플랫폼 호환성을 증명하지 않는다. 플랫폼 viewport frame을 320×800 Chrome oracle frame과 직접 비교하지 않는다.

## 반복 실행

저장소 루트에서 Android runtime GPU fixture를 켜 빌드한 뒤 `emulator-*` 대상만 설치·실행한다.

```sh
SPINON_ENABLE_C04_RUNTIME_GPU=1 mise exec -- bun run build:android
adb -s emulator-5554 install -r platforms/android/app/build/outputs/apk/debug/app-debug.apk
adb -s emulator-5554 shell am start -n dev.spinon.bootstrap/.MainActivity \
  --ez spinon_c063_absolute_lengths true
```

iOS Simulator는 다음 실행 인자를 사용한다.

```sh
SPINON_ENABLE_C04_RUNTIME_GPU=1 mise exec -- bun run build:ios-sim
xcrun simctl install booted build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app
xcrun simctl launch --terminate-running-process booted dev.spinon.bootstrap \
  --spinon-c063-absolute-lengths
```

Rust 비교 테스트는 `mise exec -- cargo test --locked -p spinon-style-to-layout absolute_length`이고 Chromium 고정 oracle 확인은 `node --test tools/css-reference/c06-absolute-lengths.test.mjs`다.

## 상태 경계

C06.3의 제한된 내부 runtime fixture 계약만 구현한다. C06 상위 및 C06.4–C06.6은 미완료다. 이 문서는 공개 API, 출시 호환성, 성능 우위 또는 전체 CSS 지원 선언이 아니다.
