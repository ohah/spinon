# C11.4 Root HTML `grid-auto-rows:max-content` 실행 근거 · 2026-11-07

이 bounded candidate는 parsed Root HTML `RuntimeRootHtmlTextPaintV1` profile의 singleton `max-content` implicit row를 실제 projection·layout·default RuntimeSession 경로에 연결한다. 한 visible Grid owner와 기존 direct-text scene shape를 유지하고, explicit column/row는 각각 단일 positive fixed-px track, flow는 sparse row, gap은 0으로 제한한다. `max-content` typed value를 유지한다. Intrinsic explicit column과의 조합, 여러 implicit track, mixed list, nonzero gap 및 다른 placement topology는 거부한다.

## 기준과 결과

- Chrome 157 사전 비교와 독립 계산은 [precomparison README](../c11-4-root-html-intrinsic-auto-rows-max-content-precomparison-2026-11-07/README.md) 및 [기대값](../c11-4-root-html-intrinsic-auto-rows-max-content-precomparison-2026-11-07/expected-values.json)에 있다. Definite 40px inline size에서 Chrome의 `min-content`, `max-content`, `auto` geometry가 같았으므로 computed/typed track identity를 별도로 검증했다.
- 사용 행 크기는 `[10,20,34]px`; item frame은 `[0,0,40,10]`, `[0,10,40,20]`, `[0,30,40,34]`이며 마지막 item은 40px Grid 아래 y=64px까지 보인다.
- Android API 37 ARM64 AVD 기본 앱 route에서 같은 fixture source hash, cascade/layout 성공, `layout=ready`, 7 boxes, root-app 평가 성공을 기록했다. Renderer는 ANGLE의 SwiftShader 소프트웨어 GPU다. [Android 화면](./android/screenshot.png) · [앱 로그](./android/app.log.gz) · [빌드 로그](./android/build.log.gz) · [설치](./android/install.log) · [실행](./android/launch.log).
- iPhone 17 Pro / iOS 26.2 Simulator 기본 앱 route도 같은 fixture source hash, `layout=ready`, 7 boxes와 root-app 평가 성공을 기록했다. Renderer는 Simulator Metal이다. [iOS 화면](./ios/screenshot.png) · [앱 로그](./ios/app.log.gz) · [빌드 로그](./ios/build.log.gz) · [실행](./ios/launch.log).
- 두 화면 모두 세 번째 녹색 item이 40px Grid 아래로 그려지고 text가 두 줄임을 시각적으로 보여준다. Android는 4+2 glyph, iOS는 3+3 glyph로 wrap했다. 이 차이는 OS/font 관찰이며 고정된 Chrome glyph metric 주장이 아니다.

## 검증

```text
cargo fmt --all -- --check                                  PASS
cargo test --locked -p spinon-layout root_html_intrinsic    17 passed
cargo test --locked -p spinon-style-to-layout root_html_profile 25 passed
cargo test --locked -p spinon-runtime parsed_root_html_max_content_auto_rows 3 passed
cargo test --locked -p spinon-runtime parsed_root_html_intrinsic_auto_rows 7 passed
cargo clippy --locked --workspace --all-targets -- -D warnings PASS
Android API 37 ARM64 AVD build/install/default route         PASS
iPhone 17 Pro iOS 26.2 Simulator build/install/default route PASS
```

Bootstrap `index.html` 및 `root-app.js`는 build 전후 원본 Git hash로 복구했다. 테스트 전 설치되어 있던 Android APK와 iOS app bundle은 build 디렉터리에 복사해 보관했고, 앱 제거 없이 같은 bundle id를 업데이트해 시뮬레이터 앱 data container를 유지했다. 실제 fixture hash는 `420ed5e05f9055cd5f8c6c9a031394fc7fdc3ff2d5348e18ed4c35b0f37e5502`이며 Android APK asset, iOS installed bundle 및 simulator logs에서 같은 값으로 확인했다. [환경·빌드 metadata](./environment.json) · [source revision](./source-revision.json).

iOS build는 `BUILD SUCCEEDED`였고 `RawWindowMetalLayer` duplicate-symbol 및 V8 archive debug-map duplicate 경고를 남겼다. AppIntents metadata skip 경고도 기록됐다. Android/iOS fixture-test feature flags는 0이었다. 경고를 기능 검증 결과로 해석하지 않는다.

## 검증 경계

시뮬레이터 스크린샷은 OS 화면 캡처를 통한 visual smoke다. Android PixelCopy, iOS Metal drawable readback, iOS compositor 이후 pixels, 실기기, hardware GPU, WPT 또는 일반 authored-CSS admission은 검증하지 않았다. Official completion은 `78/214` 유지이며 C11.4/C11은 미완료다. Internal contract version `0.1.0`은 유지한다.

독립 runtime review: [구현/runtime 20개 실패 관점](./implementation-runtime-review-20.md), [PR diff 30개 실패 관점](./pr-diff-review-30.md). 파일 해시 목록은 [SHA256SUMS](./SHA256SUMS)에 있다.
