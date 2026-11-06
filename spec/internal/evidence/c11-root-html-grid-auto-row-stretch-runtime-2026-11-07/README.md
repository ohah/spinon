# C11.4 parsed Root HTML auto implicit-row stretch runtime 근거

## 대상과 기준

- 구현 후보의 고정 HTML은 [사전 비교 fixture](../../../../tests/fixtures/css/c11/root-html-grid-auto-row-stretch-precomparison.html), SHA-256 `8a30b3d0a65f8d3edbfdc79af27d565055b22e30454f7ee687958fd5b96a40d8`다. 독립 기대값과 Chrome 157 관측은 [precomparison 자료](../c11-root-html-grid-auto-row-stretch-precomparison-2026-11-07/README.md)에 있다.
- 목표 장면은 40×40 CSS px Grid, explicit row 10px, 두 줄 direct text의 20px minimum contribution, 기본 `align-content:normal`에 따른 auto implicit row 30px이다. RuntimeSession opt-in trace는 used rows `10px 30px`, explicit/implicit count `1/1`, text lines `2`, text height `20px`, `StretchAutoTracks` 20→30px를 단언한다.
- 구현은 기본 parsed Root HTML route를 통과한다. Android/iOS 앱 검증은 `SPINON_ENABLE_C04_RUNTIME_GPU=1`을 사용했고 C11 fixture feature는 끈 상태였다. 별도 RuntimeSession fixture-test feature는 진단 trace assertion에만 사용했다.
- Rust 검증은 `spinon-layout root_html_intrinsic` 14/14, `spinon-style-to-layout root_html_intrinsic` 20/20, RuntimeSession `root_html_grid_intrinsic_auto_rows` default 19/19와 `c11-4-grid-track-sizing-fixture-test` 19/19다. 추가로 새 default runtime stretch trace 1/1, style candidate gate 1/1, `cargo fmt --all -- --check`, 변경 세 crate all-target Clippy (`-D warnings`)를 통과했다.

## Android API 37 ARM64 AVD — pass02

- 장치: `emulator-5554`, Google `sdk_gphone64_arm64`, Android 17/API 37, ANGLE Vulkan/SwiftShader.
- 설치·실행 결과: `SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass`, `layout=ready`, 5 boxes, generation 2, document revision 2, environment revision 0. 전체 원본은 [runtime log](./pass02/android-runtime.log), 설치·실행 로그는 같은 디렉터리의 `android-install.log`와 `android-launch.log`다.
- APK SHA-256 `48db6dd5748c5bb7717d20ce4b3bc023ebe93acb1dc202b3d973164544c5bc04`. APK 내부 `index.html`은 fixture SHA와 일치한다. 앱 시작 스크립트 SHA는 `3e6e656c69b525232fe87a4404a342a657daacf792c18422736ad356c7f8eb81`이다.
- [Android Simulator screenshot](./pass02/android-simulator.png) (PNG SHA-256 `4c6882a260a39c046d7d40ce25f5ba3c89cfc7b545680489bb3151a81127e717`). 화면은 Grid와 두 줄 텍스트의 visual smoke다.

## iOS Simulator — pass02

- 장치: iPhone 17 Pro, model `iPhone18,1`, iOS 26.2 Simulator, UDID `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`.
- 설치·실행 결과: `SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass`, `layout=ready`, 5 boxes, generation 2, document revision 2, environment revision 0. 전체 원본은 [runtime log](./pass02/ios-runtime.log), build·launch 로그는 같은 디렉터리에 있다.
- 앱 executable SHA-256 `eca866e291d5ffbbe7756c354532bb69bca9d2fed264bc8181d02c61a772a929`. 번들 `index.html`은 fixture SHA와 일치하고 시작 스크립트 SHA는 `3e6e656c69b525232fe87a4404a342a657daacf792c18422736ad356c7f8eb81`이다.
- [iOS Simulator screenshot](./pass02/ios-simulator.png) (PNG SHA-256 `d46b2f3bb973f8cfb6d76d7b0ffd0e5e0d7941c85eb88f818d751e1588406a6e`). 화면은 Grid와 두 줄 텍스트의 visual smoke다.

## 빌드 입력 복원과 pass 분류

- Bundler가 빈 `void 0` 시작 스크립트를 제거한 pass01은 앱에서 fixture를 실행하지 않았으므로 성공 근거로 사용하지 않는다. 그 시도와 원본 입력 백업은 보존되어 있다.
- pass02는 bundling을 보장하기 위해 빌드 중 `examples/bootstrap/root-app.js`에 `document.body.setAttribute("data-runtime", "ready");` side effect를 잠시 넣었다. 빌드 입력 script SHA-256은 `3bded5de6b8cf3dddf83af8ade9eb09828d7c0020cb56c9a7c6c3fd0798d31ad`다. `index.html` fixture는 변경하지 않았다. 빌드 완료 후 bootstrap 파일을 원본 백업으로 복원했고, 현재 source와 `HEAD` 및 보관한 원본의 `root-app.js` SHA-256은 모두 `b87b53883ce838df86412657e1353f8dc90435049eb8ead6ba8fa478ca8a1c32`다. `index.html`도 원본 SHA-256 `e7ecee32a5c5f645cc90ea7bd0c524caaf9da93fd4c780ff9dbb29d0894dd3b9`로 복원됐다.
- 기본 release `spinon-runtime` artifact `target/release/deps/libspinon_runtime-e547016af4e3eeda.rlib` (SHA-256 `d5d630cbef3b304d1d810ffa0601c519e17d910e89622b2f3e82b6654b48a292`)에서 fixture title과 fixture 파일명 marker를 raw-byte scan으로 찾지 못했다. positive control인 default/fixture-test Rust test executables에는 fixture title이 들어간다. 이 입력은 `#[cfg(test)]` test module `include_str!`에서 로드된다. 구현은 새 fixture FFI selector나 ABI를 추가하지 않았다. Android/iOS app build log도 C11 fixture feature가 꺼져 있음을 기록한다.
- pass02의 화면은 fixture 실행과 기본 route 렌더링을 보조 확인한다. 앱 로그만으로 row sizing의 정확한 geometry나 화면 pixel equality를 주장하지 않는다. 정확한 10px/30px track과 trace는 앞서 적은 RuntimeSession assertion에서 확인하며, Android는 emulator/SwiftShader이고 iOS는 Simulator다. 실기기·hardware GPU·PixelCopy·Metal drawable readback·iOS compositor 뒤 픽셀 동등성은 확인하지 않았다.
- 캡처 뒤 Android 앱은 force-stop했고 iOS 앱은 terminate했다. Agent Browser는 사용하지 않은 이 simulator run의 일부에서 `agent-browser session info --json`이 `active:false`를 반환했다.

## 보존된 실행 자료

- `pass02/`에는 양 플랫폼의 build input, 앱 artifact와 번들 자산 hash, build/install/launch/runtime 로그, device 정보, screenshot과 screenshot hash를 보관한다.
- 최상위 `original-bootstrap/`은 build 전 `examples/bootstrap/index.html`, `root-app.js`와 기존 `app-debug.apk` 사본이다. 기존 APK는 근거 보존용이며 새 artifact 또는 PR 산출물로 취급하지 않는다.
