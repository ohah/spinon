# C13.2 scroll container 구현 slice · 2026-10-15

## 확인한 범위

동일한 opt-in V8 → Stylo → Taffy → Rust render snapshot → WGPU 앱 fixture를 Android와 iOS Simulator에서 실행했다. 초기 고정 장면은 26 frame·9 scroll owner다. 별도 lifecycle script가 x range growth offset 보존, shrink clamp, hidden owner detach, reparent 후 offset reset을 차례로 실행하고, 최종 tree는 27 frame·9 owner다.

- visible owner는 frame `x=10..130`; 빨간 child는 `x=143..173`이어서 border 바깥으로 나오는 것을 frame assertion으로 확인한다. child 중심의 hit-test가 child NodeId를 반환하며 scroll offset 이후에도 target이 유지된다.
- 양 플랫폼 앱 report에서 `border_escape_outside_parent=true`, `border_escape_hit=matched`, `c13_2_lifecycle=passed`와 네 lifecycle assertion을 확인했다.
- 최종 draw는 document revision 138, render-tree revision 31, scroll revision 5, 27 frame과 동일한 9-owner offset 목록 `[40,20;0,0;0,0;0,30;30,0;22,17;42,29;0,0;0,0]`을 보였다. reparent한 owner는 새 tree 위치에 따라 순회 목록 끝으로 이동한다. 두 화면에는 부모 오른쪽의 빨간 target, 이동한 파란 scroll surface 안 marker, 고정 주황 control이 보인다.
- 화면은 Simulator capture를 눈으로 확인했고 render/hit identity는 내부 snapshot assertion으로 확인했다. Android와 iOS runtime summary의 `boxes=26`, `document_revision=130`은 lifecycle 전 초기 평가다. 직후 final draw는 양쪽 모두 `boxes=27`, `document_revision=138`, `scroll_state_revision=5`다. Android 화면 하단 label도 초기 평가 summary를 유지한다. C13.2 경로에는 PixelCopy/Metal drawable pixel readback을 연결하지 않았으므로 픽셀별 자동 비교는 아니다.

## Android API 37 ARM64 AVD

- AVD serial `emulator-5554`, model `sdk_gphone16k_arm64`, Android 17 / API 37. `adb devices -l`에 물리 기기도 함께 표시됐지만 명령은 모두 `adb -s emulator-5554`로만 실행했다. 실기기는 사용하지 않았다.
- 빌드: `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C13_2_SCROLL_FIXTURE_TEST=1 mise exec -- bun run build:android` — 성공.
- 실행: `adb -s emulator-5554 install -r .../app-debug.apk`; `adb -s emulator-5554 shell am start -n dev.spinon.bootstrap/.MainActivity --ez spinon_c132_scroll_fixture true`.
- AVD 화면 surface `1050×630`, CSS viewport `400×240`; final draw scroll revision `5`와 9-owner offset 목록을 runtime log에서 확인했다.
- [전체 화면](android-api37-avd.png), 1080×2400, SHA-256 `f45a90416b362b333c7e824bae27a760750ecfad0b1121187a350c7251b67410`.
- [runtime assertion·draw log](android-api37-avd-runtime.log).
- APK SHA-256 `d141742a9a1c55eb810f69b742ed1c28387dcb845a56f9e87ce6d8c10f1bf693`.

첫 Android 앱 실행에서 `SPINON_C132_EVAL status=-12`가 났다. ADB 연결/설치 실패는 아니었다. 로그에서 fixture 초기화가 `SurfaceHolder`의 양수 크기 준비보다 앞서 시작했고, 직후 `surfaceChanged`가 presentation sequence를 바꿔 lifecycle 마지막 publish를 stale로 거부한 것을 확인했다. C13.2 route 초기화를 surface-ready scheduler로 옮긴 뒤 AVD 재실행에서 status 0과 네 lifecycle assertion을 확인했다.

## iOS Simulator

- iPhone 17 Pro Simulator, model identifier `iPhone18,1`, iOS 26.2, arm64 simulator. 실기기는 사용하지 않았다.
- 빌드: `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C13_2_SCROLL_FIXTURE_TEST=1 mise exec -- bun run build:ios-sim` — 성공.
- 실행: `xcrun simctl launch ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D dev.spinon.bootstrap --spinon-c132-scroll`.
- drawable surface `1200×720`, CSS viewport `400×240`; final draw scroll revision 5와 9개 owner offset을 확인했다.
- [전체 화면](ios-iphone17pro-ios26_2.png), 1206×2622, SHA-256 `46461ab7150040712899aa76652eee20f7d86630ec96ac7bbb7e0c0e6ccbb87a`.
- [runtime assertion·draw log](ios-26-2-simulator-runtime.log).
- app executable SHA-256 `47415e0385fa40b234afb004416f3ee7e4f9ecd7bca75c614635d467bc2f93e3`.

## 입력 및 범위 경계

- [고정 26-frame fixture](../../../../tests/fixtures/css/c13/runtime-scroll-container-c13-2.js) SHA-256 `b39472b1e1f8581ab1bf142c701c5d806d518303d0b464cb4c460349e0807771`.
- [4-phase lifecycle fixture](../../../../tests/fixtures/css/c13/runtime-scroll-container-c13-2-lifecycle.js) SHA-256 `269d3c05b7f2015bff137a212ec83241db4cfc10021dcd2010152824ada9cfb7`.
- `cargo test --locked --workspace`: 797 passed, 0 failed, 4 ignored. [전체 실행 log](workspace-test.log).
- `cargo test --locked -p spinon-ffi --features c13-2-scroll-fixture-test --lib`: 43 passed, 0 failed, 2 ignored. Opt-in fixture ABI 경로 단위 검사다. [실행 log](ffi-feature-test.log).
- `cargo clippy --locked --workspace --all-targets -- -D warnings`: 통과. 첫 실행에서 발견한 too-many-arguments lint는 `RuntimeScrollGeometryContext`로 공통 geometry 입력을 묶어 수정했다. [최종 log](workspace-clippy.log) · [수정 전 lint log](workspace-clippy-initial-failure.log).
- `cargo fmt --all -- --check`, `git diff --check`, `node --test tools/css-reference/fixture-module-boundary.test.mjs`: 각각 통과, boundary test 5/5.
- 기본 `spinon-ffi` archive에는 C13.2 FFI symbol과 `c132-root` payload가 없고, `c13-2-scroll-fixture-test` opt-in positive control에는 둘 다 들어 있다. display-none range fix 뒤 다시 빌드해 검사했다. [정적 archive symbol/string 검사](feature-isolation.txt).
- Rust geometry tests에는 root/body, stale scroll revision rejection, reserve failure 뒤 원자성·복구 negative cases가 있다. root/body scene과 stale/failure negative control은 양 플랫폼 app matrix에 없다. `box-shadow`는 현재 app runtime이 cascade/layout 단계에서 지원하지 않아 fail-closed되므로 ink-only overflow는 simulator pass로 세지 않았다.
- C13.2의 public CSSOM `scrollTop/scrollLeft`, gesture/event, scrollbar, snap, RTL/vertical writing, WPT, physical-device/GPU 동작은 이 실행의 범위가 아니다. app root/body, ink-only, simulator pixel readback, FFI-level stale/failure negative controls와 구현 관점 20개 및 PR delta 관점 30개 review는 별도 경계로 기록한다.
