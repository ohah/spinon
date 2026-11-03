# C11.1 Root HTML Grid item alignment 실행 근거

## 판정 범위

`RuntimeRootHtmlTextPaintV1` 기본 parsed Root HTML route에서 eligible fixed-track Grid container의 `align-items`, `justify-items`, `place-items` computed longhand를 Taffy layout까지 연결했다. 이는 좁은 C11.1 구현 slice다. C11.1/C11 완료, 일반 CSS/Grid 지원, Spinon WPT 적합성 또는 공식 완료 `72/213` 증가를 뜻하지 않는다.

고정 fixture는 [`root-html-grid-item-alignment.html`](../../../../tests/fixtures/css/c11/root-html-grid-item-alignment.html)이며 SHA-256은 `f558d61ca1ea3c70d72284909ba8b213cddd4ec3a2450a72c268750c9cfa4122`다. 독립 기대값은 [expected-values.json](../c11-root-html-grid-item-alignment-precomparison-2026-11-03/expected-values.json)에, Chrome 156 Agent Browser 관찰·39/39 비교와 캡처는 [사전 비교 README](../c11-root-html-grid-item-alignment-precomparison-2026-11-03/README.md)에 있다.

## Rust 비교와 회귀

- Root HTML style/layout regression: [`rust-style-root-grid.log`](rust-style-root-grid.log), 38/38 통과. 새 item-alignment 양성·거부와 기존 Root HTML sizing/content-alignment regression을 포함한다.
- `justify-items` parser의 정확한 오류 property와 제한 LTR `left/right` mapping: [`rust-justify-items-parser.log`](rust-justify-items-parser.log), 1/1 통과.
- Item/content allowlist와 baseline·unknown rejection: [`rust-alignment-allowlist.log`](rust-alignment-allowlist.log), 1/1 통과. 변경하지 않은 C11.4 track-sizing profile regression 14/14도 [`rust-c11-4-profile-regression.log`](rust-c11-4-profile-regression.log)에서 확인했다.
- 기존 `align-content`·`justify-content` 기본 Root HTML runtime: [`rust-runtime-content-alignment.log`](rust-runtime-content-alignment.log), 1/1 통과.
- 새 기본 Root HTML runtime: [`rust-runtime-item-alignment.log`](rust-runtime-item-alignment.log), full fixture의 39 scene·42 item frame 42/42가 독립 기대값과 1/64 CSS px 이내로 일치하고, baseline projection 거부 때 partial scene을 publish하지 않는다. Runtime tests 2/2 통과.
- `cargo fmt --all -- --check` 및 scoped all-target Clippy는 통과했다. Clippy 출력은 [`rust-clippy.log`](rust-clippy.log)에 있다.

Style projection은 기존 fixed-track Grid eligibility와 direct empty `div` item 경계를 재사용한다. Non-eligible Root HTML node/Grid의 non-initial `justify-items`, child `align-self`·`justify-self`, baseline, `safe start`, `legacy`, 미지원 profile의 `left/right`는 거부한다. CSS-invalid declaration의 cascade fallback은 profile rejection과 구분한다.

## Android 기본 앱 경로

- Android API 37 ARM64 AVD `spinon_api37_0_applied_txn`, `emulator-5556`, 1080×2400, 420 dpi, SwiftShader via ANGLE. 최종 source build APK SHA-256 `bd89ac784bdb3666332ec49f7bfbc900b7c0adf999d77db42cc3f71b845cd49d`을 AVD에 설치한 다음 app data와 logcat을 비우고 Root HTML route를 새로 실행했다. 실행 시각은 metadata에 있으며 APK·screenshot hash도 [`android/metadata.txt`](android/metadata.txt)에 연결되어 있다. Root HTML 시작과 `layout=ready boxes=83`, `presented boxes=83`은 [`android/success.log`](android/success.log)에 있다. 최종 실행 뒤 AVD를 종료했다. 실기기는 사용하지 않았다.
- Build flag는 `c04_runtime_gpu=1`; C11 app fixture와 전용 fixture-test flags는 모두 `0`이다. [`android/build.log`](android/build.log)에서 `BUILD SUCCESSFUL`을 확인했다. AGP/compile SDK 호환 안내와 고정 NDK 환경변수 무시 안내가 남았다.
- 화면은 [Android AVD 캡처](android/screen.png)다. 전체 긴 fixture 중 첫 viewport의 시각 smoke만 보여 준다.

첫 앱 시도에서는 임시로 비운 `root-app.js`가 Bun bundler에서 ES module export를 포함해 V8의 script eval에 실패했다. 해당 오류 로그·화면·번들은 [`android/initial-empty-root-app-error.log`](android/initial-empty-root-app-error.log), [`android/initial-empty-root-app-error.png`](android/initial-empty-root-app-error.png), [`android/empty-root-app-bundle-with-esm-export.js`](android/empty-root-app-bundle-with-esm-export.js)에 보존했다. 다음 시도에는 fixture 전용 no-op marker script를 사용해 app eval과 draw/present를 통과했다.

최종 APK 재실행 때 `monkey`는 headless AVD에 physical key가 없어 종료 코드 251을 냈다. 이는 launcher harness 제한이며 제품 실패가 아니다. 원문은 [`android/launcher-monkey-error.log`](android/launcher-monkey-error.log)에 보존했고, 같은 설치 앱을 explicit `MainActivity` intent로 열어 위의 성공 로그와 캡처를 얻었다.

## iOS 기본 앱 경로

- iPhone 17 Pro / iOS 26.2 Simulator (`AC268F2F-76B5-4B8E-90A1-1D35D9CD36ED`). 이번에 부팅한 전용 simulator와 앱은 종료했다.
- 최신 source build 뒤 전용 Simulator에서 기존 앱을 제거하고 다시 설치해 실행했다. Build flag는 `c04_runtime_gpu=1`; C11 및 다른 fixture-test flags는 `0`이다. [`ios/build.log`](ios/build.log)에 `BUILD SUCCEEDED`가 기록되어 있다. 빌드에는 기존 Swift deprecation/cast 및 V8 duplicate debug-map warnings가 남았다.
- 최종 실행 로그는 `layout=ready boxes=83`, `SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass`, `SPINON_C0410_DRAW ... presented boxes=83`을 기록한다. 자세한 내용은 [`ios/runtime.log`](ios/runtime.log)에 있다.
- Simulator executable, bundled root script와 screenshot SHA-256은 [`ios/metadata.txt`](ios/metadata.txt)에 있다. 화면은 [iOS Simulator 캡처](ios/screen.png)다.

## 복원과 한계

앱 빌드 중 `examples/bootstrap/index.html`을 고정 fixture로, `root-app.js`를 no-op marker로 임시 교체했다. 실행 뒤 두 tracked source를 원래 내용으로 복구했고 원래 SHA-256은 각각 `e7ecee32a5c5f645cc90ea7bd0c524caaf9da93fd4c780ff9dbb29d0894dd3b9`와 `b87b53883ce838df86412657e1353f8dc90435049eb8ead6ba8fa478ca8a1c32`로 돌아왔다. 원본과 fixture 입력 사본은 [`input-backups/`](input-backups/)에 있다.

Android/iOS 로그의 `presented`는 draw submission 관찰이다. Screenshots는 시각 확인이며 Android PixelCopy, iOS Metal drawable readback 또는 compositor 후 pixel parity를 검증하지 않는다. 각 화면에는 긴 문서의 첫 viewport만 보이므로 39-scene 전체 검증은 Rust runtime frame 대조가 담당한다. Android는 SwiftShader AVD이고 iOS는 Simulator다. Hardware GPU, 실기기, 외부 앱의 전체 화면 결과를 주장하지 않는다.
