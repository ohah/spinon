# C11.4 anonymous Grid text computed `word-spacing` 실행 근거

## 범위와 기준

이 실행은 `ParagraphStyle.word_spacing`에서 Parley intrinsic text sizing, anonymous Grid item, Columns/Rows track까지 이어지는 24개 고정 fixture를 확인한다. 계획과 독립 기대값은 [사전 비교](../c11-4-anonymous-grid-text-word-spacing-precomparison-2026-10-31/README.md)에 기록했다. 일반 HTML/CSS admission이나 C11.4/C11 전체 지원을 뜻하지 않는다.

기준 checkout은 `ca0627b89c2a76b82347627dceccbc6e0bd84872`이며, 구현 소스는 이 commit 위의 현재 작업 트리 변경이다. 파일별 SHA-256은 [`source-revision.json`](./source-revision.json)에 연결했다. 앱은 `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1`로 빌드했고, capture runner는 빌드 산출물과 설치 산출물의 SHA-256 일치를 실행 전에 검사한다.

## 시뮬레이터 행렬

| 대상 | 실행 결과 | 설치 산출물 SHA-256 |
|---|---:|---|
| Android API 37 ARM64 AVD (`sdk_gphone16k_arm64`, `emulator-5554`) | 24/24 PASS | `18a004b615f812a4c4e371759cd952028cc77d9e2c1d5cd04f11345fded04d74` |
| iPhone 17 Pro / iOS 26.2 Simulator (`iPhone18,1`, `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`) | 24/24 PASS | `d9cd0f9d9dc0b99f545f2e4aab5bcb1dd564bdf9772696583ec4a3e1b750bfd3` |

각 행은 computed `wordSpacing`, anonymous text source/node, HostDocument revision, intrinsic min/max inline·block contribution, line 수·advance, item frame, Columns/Rows 최종 track 크기와 trace chunk 완결성을 확인했다. 48개 실행 모두 `status=0`, `layout=ready`, assertion/argument error 부재였다. 원본은 플랫폼별 [Android matrix와 scene log](./android-avd/) 및 [iOS matrix와 scene log](./ios-simulator/)다. 최종 빌드 로그는 [Android](./android-build.log)와 [iOS](./ios-build.log), iOS 24-case 재실행 출력은 [capture log](./ios-capture.log)다. iOS executable 재빌드에서 hash가 바뀌어 최종 matrix를 새 설치본으로 다시 실행했다. 검증은 Chrome 156 사전 비교에서 고정한 scene별 CSS px 기대값과 exact 비교이며 플랫폼 screenshot pixel 비교는 아니다.

## 빌드와 코드 검사

- `cargo fmt --all -- --check`: 통과.
- Text-layout Ahem word-spacing, Runtime anonymous Grid 24-scene, FFI 24-scene generation focused test: 각 1/1 통과.
- 전체 관련 패키지 회귀: `spinon-text-layout` 74/74 통과, `spinon-runtime` fixture feature 352 통과·3 ignored, `spinon-ffi` fixture feature 175 통과·2 ignored.
- CSS fixture boundary test: 1/1 통과. 독립 기대값 검증: 24/24 exact.
- `cargo clippy --locked -p spinon-ffi --all-targets --features c04-runtime-gpu-fixture-test -- -D warnings -A clippy::suspicious_else_formatting`: 통과. 예외한 lint는 변경하지 않은 기준 코드 `runtime_layout.rs:960`의 기존 `else if` 줄바꿈 경고다. 예외 없이 실행한 동일 명령은 그 한 경고로 중단됐다.
- Android fixture app과 iOS Simulator fixture app build: 성공. iOS 빌드에는 기존 deprecation/cast, AppIntents metadata skip, duplicate debug-map 경고가 있었지만 빌드는 성공했다.
- 기본/fixture-test release static archive 분리: [artifact-boundary evidence](./artifact-boundary/README.md).

## 화면 캡처가 증명하는 범위

각 플랫폼의 `representative-screen.png`는 앱 제목과 `status=0 layout=ready boxes=1` 상태를 보여주지만, WGPU 장면 영역은 검게 표시되어 글자 픽셀을 확인할 수 없다. 따라서 이미지는 앱 실행 상태만 보여주는 참고 화면이며 glyph paint, PixelCopy, Metal drawable readback, compositor 이후 픽셀 일치의 근거로 사용하지 않는다. Android/iOS screenshot SHA-256은 각각 `0847a3e2e2646b17469f7a3fb363e0edd851a14c4616b71485ede5c428b8e9d1`, `b2eb0bc0183bf2162b18d2f7c9ed55b12a014acb3951ff645b39f093fd3f7aef`다.

실기기, hardware GPU, 임의 글꼴·길이 단위, leading whitespace, 다른 whitespace mode/writing mode, inline styled run, 일반 CSS admission, WPT와 전체 C11 적합성은 이 실행에서 검증하지 않았다. 공식 C11.4/C11 check 및 완료 수는 바뀌지 않는다.
