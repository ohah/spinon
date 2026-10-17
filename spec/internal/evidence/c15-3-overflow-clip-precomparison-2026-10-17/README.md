# C15.3 텍스트 overflow·clip 사전 비교와 Simulator 표면 검증

## 고정 입력과 Chrome 155 기준

- 기준 fixture는 기존 [C15.3 Unicode shaping Chrome fixture](../../../../tests/fixtures/css/c15/c15-3-unicode-shaping-oracle.html)의 `visible-overflow-hit`과 `hidden-overflow-clip`이다. Agent Browser가 연 파일 URI에서 Headless Chrome 155 (`HeadlessChrome/155.0.0.0`)로 확인했고 Ahem font load 및 `document.fonts.check('10px "C15 Ahem"', 'XX')`가 성공했다.
- 두 부모의 content width는 15 CSS px, 높이는 22 CSS px, border는 1 CSS px이다. Ahem `10px/10px` `XX`의 inline box 폭은 20 CSS px다.
- Chrome의 부모 border box는 visible `(16,526) 15×22`, hidden `(43,526) 15×22`이며 text inline box는 각각 `(18,527) 20×10`, `(45,527) 20×10`이다. Parent 오른쪽에서 3px 바깥의 hit test는 visible에서 `visible-text`, hidden에서 `null`이다. [Chrome 전체 화면](./chrome155.png)

## Spinon runtime slice

Renderer는 text clip을 owner render box의 유효 paint clip과 교차한다. visible owner에 조상 clip이 없으면 글리프는 부모 border 밖에서도 유지된다. hidden owner에는 조상의 실제 overflow clip이 적용된다. 0 offset인 scroll ancestor에서는 같은 clip을 보존하지만 non-zero scroll transform과 text가 직접 소유한 scroll container는 기존처럼 명시 거부한다.

고정 C15.3 runtime fixture는 실제 DOM/CSS 경로에서 `overflow:visible`/`hidden`을 설정한 Ahem `XX` owner 두 쌍과 기존 Noto Sans KR text를 한 장면에 연결한다. 이 두 overflow 값은 런타임 owner box의 유효 paint clip으로 전달된다. 각 앱은 글리프의 실제 drawable 표면 픽셀을 읽어 다음을 각각 확인했다.

| 플랫폼과 표면 | `visible` parent 밖 | `hidden` parent 안 | `hidden` parent 밖 |
| --- | --- | --- | --- |
| Android API 37 / Android 17 ARM64 AVD, SurfaceView PixelCopy (`945×630`, viewport `360×240`) | `(77,213) = [9,33,112,255]` ink | `(35,318) = [9,33,112,255]` ink | `(77,318) = [247,249,252,255]` stage background |
| iPhone 17 Pro / iOS 26.2 Simulator, Metal drawable before compositor (`1080×720`, viewport `360×240`, `Bgra8Unorm`) | `(88,244) = [9,33,112,255]` ink | `(40,364) = [9,33,112,255]` ink | `(88,364) = [247,249,252,255]` stage background |

두 플랫폼 모두 `status=matched`, 표본 `3/3`이다. 표본 합격은 parent 안 색상에 속지 않도록 visible overflow와 hidden 내부에서 고정 잉크 RGBA `[9,33,112,255]`를 정확히 요구하며, hidden 외부는 stage 배경 `[247,249,252,255]`를 정확히 요구한다. 초기 조건은 hidden parent의 배경색도 잉크로 오인할 수 있어 이 검토에서 수정했다. Android PixelCopy PNG는 [표면 캡처](./android-pixelcopy.png), OS 화면은 [Android 화면](./android-screen.png)과 [iOS 화면](./ios-screen.png)이다. 앱의 화면 캡처는 시각적 자료이고 clip 합격 판정은 위의 별도 표면 readback에서 나온다.

## 재현·실행 자료

- Pure Rust/WGPU: `cargo test --locked -p spinon-render -p spinon-render-wgpu` — 93 + 39 passed. [전체 출력](./rust-render-tests.txt). 첫 vertical-scroll 회귀 입력은 scroll height가 viewport와 같아 유효한 최대 offset이 0이었다. [실패 출력](./rust-render-tests-initial-fixture-error.txt)을 보존하고 scroll content height를 16px로 고쳐 1px y offset이 생성되도록 한 뒤 재실행했다.
- FFI: `cargo test --locked -p spinon-ffi --features c15-3-wgpu-glyph-paint-fixture-test` — 45 passed, 2 ignored. [전체 출력](./ffi-tests.txt).
- Clippy: `cargo clippy --locked -p spinon-render -p spinon-render-wgpu --all-targets -- -D warnings`와 `cargo clippy --locked -p spinon-ffi --all-targets --features c15-3-wgpu-glyph-paint-fixture-test -- -D warnings`가 통과했다. 첫 FFI 실행에서 8-인자 raster helper 경고가 나와 `FixtureTextInput`으로 묶고 재실행했다. [renderer](./render-clippy.txt) · [FFI](./ffi-clippy.txt). `cargo fmt --all -- --check`, `git diff --check`, CSS fixture module boundary 6/6도 통과했다.
- Android: `SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST=1 mise exec -- bun run build:android`; opt-in APK는 `emulator-5554`에만 설치했다. [build log](./android-build.txt)와 [원본 Android logcat](./android-log.txt)에서 PID `9150`, run `a5908d132acd`는 opaque Ahem sample 및 exact-ink 조건과 PixelCopy 표본 `3/3`을 기록한다.
- iOS: `SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST=1 mise exec -- bun run build:ios-sim`; 지정 iPhone 17 Pro Simulator에서 앱을 실행하고 Metal drawable 표면을 읽었다. [build log](./ios-build.txt)와 [원본 iOS log](./ios-log.txt)에 PID `35992`, renderer, 크기, format, 세 RGBA 표본과 exact-ink `status=matched`가 남아 있다.
- 기본 FFI artifact는 C15.3 C ABI 이름과 payload marker 없이 빌드됐고 opt-in positive control에는 두 종류가 포함됐다. [기본/opt-in artifact 경계 기록](./artifact-boundary.txt).
- 기준 base commit은 `073c74f7bd3140f9cdefbac70a344dd2db3d240c`이며, 캡처 당시 Android APK SHA-256은 `2f505214d66b739fd3d5c5cbc0acf044fa3d74908df49735a9868209384eec51`, iOS executable SHA-256은 `a0aa76dd7d225b36cfe70a5700065e07ac7a126406efca0a56a6cce16ef87380`이다. 앱 바이너리는 저장소에 포함하지 않는다. `sha256.txt`는 이 디렉터리의 나머지 18개 파일을 검증하며 자기 자신은 제외한다.

## 실패 경로 검토

- [구현 실패 관점 20개](./implementation-review-20.md) — scroll metadata 누락·offset, owner clip 교차, sample 색 false-positive 및 플랫폼 readback 실패 경계를 확인했다.
- [PR diff 실패 관점 30개](./pr-delta-review-30.md) — 런타임/FFI/Android/iOS fixture-test 경계와 증거·로드맵 일치를 검토했다.

## 판정 경계

이 결과는 고정 Ahem glyph scene에서 CSS `overflow:visible`/`hidden`이 owner clip으로 전달되어 WGPU 표면에 반영되는 제한 동작을 증명한다. 다른 overflow 값, 일반 DOM text admission과 text style-to-glyph projection, 임의 glyph/font fallback, non-zero scroll transform, text hit-test, hardware GPU, 실기기 및 iOS compositor 이후 화면을 증명하지 않는다. 시뮬레이터 결과는 실기기 결과가 아니다. 따라서 C15.3과 상위 C15는 미완료로 유지한다.
