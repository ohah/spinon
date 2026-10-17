# C15.3 runtime glyph mask WGPU paint 근거

**작업:** 고정 glyph raster mask를 `RuntimeRenderSnapshot`과 실제 WGPU draw command에 연결하고 Android·iOS Simulator 앱 route에서 확인한다.
**구현 계약:** [내부 계약 0072](../../0072-c15-3-wgpu-glyph-paint.md)
**계획:** [C15 inline text layout](../../../../plan/c15-inline-text-layout.md)
**선행 자료:** [C15.3 Unicode shaping 사전 비교](../c15-3-unicode-shaping-precomparison-2026-10-17/README.md)

## 고정 입력과 판정

- Scene은 360×240 CSS px, DPR 1이다. 문장은 `C15 glyph paint coverage 한글 overflow sample`이고 CSS 원점은 `(12px, 12px)`이다.
- 고정 글꼴은 `tests/fixtures/resources/c15/c15-3/NotoSansKR-VF.ttf`, SHA-256 `9e1d729e7e2b36f9ef439da102f8c134c10aabe46f1c843bf0aca5c043b86f76`이다. system font fallback에는 의존하지 않는다.
- 화면상 text clip 오른쪽 경계는 CSS x=160이다. 판정 기준은 고정 mask coverage와 경계 안팎의 sample이다. OS별 전체 화면 antialias 유사도는 판정하지 않는다.

## 구현과 자동 검증

- glyph alpha mask를 runtime snapshot에 보관하고 `DrawText` 순서로 WGPU R8 coverage atlas에서 합성한다. snapshot에는 owner, revision, glyph placement, baseline, scale, color, clip을 보존한다.
- `wgpu_readback_draws_coverage_masks_and_respects_css_clip_edge`가 2×2 coverage `[0,64,128,255]`를 색 표본 `[0,0,0,255]`, `[64,16,0,255]`, `[128,32,0,255]`, `[255,64,0,255]`로 읽었다. clip 밖 표본은 검정으로 남았다. `wgpu_readback_composites_glyph_masks_inside_an_opacity_group`는 text alpha 0.5와 group opacity 0.5를 함께 적용해 `[16,4,0,255]`, `[32,8,0,255]`, `[64,16,0,255]`를 읽었다. [focused readback log](./wgpu-readback-test.log)
- `cargo test --locked -p spinon-render -p spinon-render-wgpu`: render 87개, WGPU 38개, opacity 3개, stacking 4개, 합계 132개 통과. [전체 crate test log](./crate-tests.log)
- 두 atlas texture-dimension 거부 시험과 scroll text 거부 시험도 통과했다 ([atlas](./atlas-limit-tests.log), [scroll](./runtime-text-scroll-test.log)). `cargo clippy --locked -p spinon-render -p spinon-render-wgpu --all-targets -- -D warnings`와 [Clippy log](./clippy.log), `cargo test --locked -p spinon-ffi --features c15-3-wgpu-glyph-paint-fixture-test`(44 통과·2 ignored, [log](./ffi-fixture-tests.log)), `node --test tools/css-reference/fixture-module-boundary.test.mjs`(6 통과, [log](./module-boundary-tests.log)), rustfmt와 `git diff --check`도 통과.
- fixture 코드와 글꼴의 기본 artifact 격리를 정적 archive 전체 byte 검사로 대조했다. 기본 archive에는 전용 FFI symbol·fixture payload·정확한 글꼴 bytes가 없고 opt-in archive에는 모두 있다. [artifact 검사와 archive hash](./fixture-artifact-boundary.txt): 기본 `fb6191e879d25b6fc8a5bd81f249c29e1bcb14e58b74328b2e5f335ce34f4271`, opt-in `f0d7963fcd6f2da2b65e5ae1b95a3b30f99a2c4e4b8273402c747e5af7f6c844`.

## Android AVD

- 실행 대상은 `emulator-5554` 하나로 한정했다. API 37, Android 17, `sdk_gphone16k_arm64` ARM64 AVD이며 실기기 명령은 실행하지 않았다.
- opt-in positive route에서 runtime log는 viewport 360×240, scale 1, `NotoSansKR`, glyph 36개, `runtime_text_count=1`, `glyph_batch_draw_count=1`을 보고했다. WGPU 표면은 945×630이고 backend는 ANGLE/Vulkan/SwiftShader software 경로다. [positive log](./android-api37-android17-final-reviewed-20261017/android-logcat.txt)
- 최종 소스 tree로 다시 빌드·실행했다. [Android 화면 캡처](./android-api37-android17-final-reviewed-20261017/android-screen.png)는 1080×2400 screenshot이고 SHA-256은 `7676acbf100d5cba00a1cbc773ec0a528786e061d5eaf86f7b851da30192d833`이다. [실행 log](./android-api37-android17-final-reviewed-20261017/android-logcat.txt) SHA-256은 `51fd9ce6fc1850bdd89bdbc0e10e60a4303d859d6e5c6f7958b07cf225ff8f2e`; opt-in APK SHA-256은 `10eb762a9b8261b446628f019c0512b2d4053eb64e8b04d813e3fd9b93934b44`이다. [AVD identity](./android-api37-android17-final-reviewed-20261017/android-device.txt)와 [build log](./android-api37-android17-final-reviewed-20261017/build.log)을 보존했다.
- feature-off negative route는 `SPINON_C153_DISABLED fixture-test feature is off`로 거부했다. [negative log](./android-api37-android17-feature-off-negative-20261017/android-logcat.txt)와 [화면](./android-api37-android17-feature-off-negative-20261017/android-screen.png)을 보존했다.

## iOS Simulator

- iPhone 17 Pro / iOS 26.2 Simulator (UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`)에서 실행했다. 실기기는 사용하지 않았다.
- opt-in positive route는 viewport 360×240, scale 1, `NotoSansKR`, glyph 36개, `runtime_text_count=1`, `glyph_batch_draw_count=1`을 보고했다. WGPU Metal drawable texture는 1080×720이다. [positive route log](./ios-simulator-iphone17pro-final-reviewed-20261017/ios-positive-route.log), [전체 실행 log](./ios-simulator-iphone17pro-final-reviewed-20261017/ios-log.txt)
- 최종 소스 tree로 다시 빌드·실행했다. [iOS 화면 캡처](./ios-simulator-iphone17pro-final-reviewed-20261017/ios-screen.png)는 1206×2622 screenshot이고 SHA-256은 `e2463899ec13c76984cb9cc1510a02f2d1fac6501ec49aa8cb33f87a48082754`이다. [최종 positive route log](./ios-simulator-iphone17pro-final-reviewed-20261017/ios-positive-route.log) SHA-256은 `61fe837be562ff8f380fcdef02f9ee3bdffa0073c2a569b352b7094d93398b73`; 전체 log는 [여기](./ios-simulator-iphone17pro-final-reviewed-20261017/ios-log.txt) 있고 [Simulator identity](./ios-simulator-iphone17pro-final-reviewed-20261017/ios-device.txt)와 [Xcode build log](./ios-simulator-iphone17pro-final-reviewed-20261017/ios-xcodebuild.log)도 보존했다. app executable SHA-256은 `d456f618dbab9deb389c021a084f3598be831893c8b1b915e2b7bc7b8ada80de`이다.
- Xcode link는 `BUILD SUCCEEDED`였지만 `libspinon_wgpu_r08_spike.a`와 `libspinon_ffi.a` 사이 `RawWindowMetalLayer` 심볼 중복 경고 4개, V8 archive dSYM 중복-map 경고, `AppIntents.framework`가 없어 metadata 추출을 건너뛴 경고를 남겼다. `RawWindowMetalLayer` 중복은 [C13.1 Simulator 근거](../c13-1-overflow-clip-paint-hit-border-2026-10-14/README.md)에서도 기록된 기존 링크 경고다. 이번 변경에서 해결하지 않았으며 경고 없는 빌드로 표기하지 않는다.
- feature-off negative route는 `SPINON_C153_ARGUMENT_ERROR rejected=true reason=fixture-feature-disabled`로 거부했다. [필터한 rejection log](./ios-feature-off-negative-20261017/ios-negative-route.log)와 [화면](./ios-feature-off-negative-20261017/ios-screen.png)을 보존했다.

## 한계와 상태

Android/iOS 파일은 simulator의 전체 화면 screenshot이며 renderer surface의 `PixelCopy`/Metal drawable readback 결과가 아니다. 실제 surface pixel의 glyph coverage와 clip은 위 focused WGPU readback test로만 확인했다. Simulator 화면은 app route가 glyph batch를 제출하고 눈에 보이는 text를 그린다는 점을 보인다.

이 fixture는 text를 root box에 직접 붙인다. DOM text extraction, computed style/font projection, Block/Flex/Grid intrinsic measurement consumer, ancestor CSS overflow mapping, scroll transform, text hit-test, font fallback/async `@font-face`, color glyph, 실기기와 hardware GPU는 검증하지 않았다. 그러므로 이 slice는 C15.3의 일부 runtime paint 경로 근거이며 C15.3 또는 상위 C15를 완료하지 않는다.

## 독립 실패 관점 검토

- [구현 실패 관점 20개](./implementation-review-20.md)와 [PR 변경 실패 관점 30개](./pr-delta-review-30.md)는 mask 유효성부터 scroll 경계, clip, atlas, cache, opacity group, 플랫폼 feature route, 기본 artifact와 상태 대장까지 별도로 대조한다. 두 Simulator를 최종 소스 tree로 다시 실행한 뒤 30번째 항목도 닫았다.
