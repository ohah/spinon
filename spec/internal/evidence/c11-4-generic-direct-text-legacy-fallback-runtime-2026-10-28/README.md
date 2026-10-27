# C11.4 direct-text legacy fallback · 13-scene runtime evidence

## 결과

기존 one-item `TEXT_CASES` 13개에 대해 generic direct-text collector가 `Ok(None)`을 반환하고, 앞선 14개 specialized adapter도 장면을 claim하지 않은 뒤 `c11_4_text::prepare` 전체 dispatcher가 기존 `TEXT_CASES` fallback으로 입력을 준비하는지 확인했다. 반환된 Ahem paragraph style로 intrinsic sizing을 직접 호출해 장면별 고정 min/max-content 폭과 비교했다. Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator app route에서도 각각 13/13 장면을 실행했다.

이 변경은 테스트와 실행 근거 보강이다. 제품 layout 코드, FFI 장면 등록, Android/iOS matcher는 수정하지 않았다. 일반 CSS admission, C11.4 완료, glyph paint 또는 전체 WPT 통과를 주장하지 않는다. 공식 C11.4 checkbox는 미완료이고 집계 `72/213`은 그대로다.

## 검증

- Rust `c11_4_text::` focused tests: 21/21 통과. `all_one_item_text_cases_reach_the_legacy_fallback_with_ahem_intrinsics`가 13개 장면을 순회한다. 별도 negative test는 unknown root/item ID, 변경된 text, 추가 direct item, 잘못된 typed Grid track을 generic collector 뒤 legacy dispatcher가 거부하는지 검사한다. [첫 결과 로그](build/runtime-c11-4-text-tests.log) · [negative control 추가 후 재검증](build/runtime-c11-4-text-tests-recheck.log)
- `cargo fmt --all -- --check`: 통과. [기록](build/rustfmt-check.log)
- `cargo clippy --locked --workspace --all-targets --features spinon-runtime/c11-4-grid-track-sizing-fixture-test -- -D warnings`: 수정하지 않은 `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_grid_layout_auto_tracks/validation.rs:329`의 기존 `clippy::type_complexity` 경고로 실패했다. 해당 파일은 이 변경에서 수정하지 않았다. [실패 로그](build/clippy.log)
- Feature-specific `spinon-runtime` all-target Clippy는 위 기존 경고 하나만 허용한 상태에서 통과했다: `cargo clippy --locked -p spinon-runtime --all-targets --features c11-4-grid-track-sizing-fixture-test -- -D warnings -A clippy::type_complexity`. [첫 로그](build/clippy-feature-specific.log) · [negative control 추가 후 재검증](build/clippy-feature-specific-recheck.log)
- Android fixture-test APK build는 `SPINON_ENABLE_C04_RUNTIME_GPU=1`, `SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1`로 통과했다. [로그](build/android-build.log)
- iOS Simulator fixture-test app build도 같은 flag로 성공했다. Xcode가 기존 Swift deprecation/type-cast 및 V8 debug-map duplicate-symbol 경고를 출력했지만 마지막 결과는 `BUILD SUCCEEDED`다. [로그](build/ios-build.log)
- FFI source, Android matcher, iOS matcher에 각각 13개 scene ID가 있다. runner는 각 `scene_id`, status, layout, trace, draw/present를 검사하며 현재 run의 scene summary 뒤에 draw/present가 있어야 통과한다.
- pass03 app build와 runtime matrix는 제품 source commit `5df6da5665e7df2cb4c3e6ec0d7278ce8b50ad17`을 사용했다. 후속 `7af1322f`는 위 다섯 negative unit control만 추가했으며 native app/runtime source와 pass03 입력은 바꾸지 않았다.
- 변경된 code, 문서와 metadata는 whitespace 검사를 통과했다. 캡처 도구가 출력한 원본 `.log` 13개는 byte 그대로 보존했으며, 이 파일들에는 끝 공백과 빈 마지막 줄이 있어 전체 `git diff --check`는 해당 log에서만 경고한다.

## 채택 실행 · pass03

Runner [source](run-simulator-matrix.sh)는 기존 Android target `emulator-5554` (AVD `spinon_api37_1_compat16k`, API 37, `arm64-v8a`)와 새로 만든 iPhone 17 Pro / iOS 26.2 Simulator (`iPhone18,1`)를 사용했다. 새 Simulator는 이 실행용으로 만들고 실행 뒤 종료·삭제했다. Android 명령은 `adb -s emulator-5554`만 대상으로 삼아 연결된 실기기와 다른 emulator에는 명령하지 않았다. 기존에 켜져 있던 iOS Simulator 두 대와 해당 Spinon process는 보존했다.

[pass03 로그, source/artifact hash 및 사후 확인](runs/c114-legacy-fallback-20261028-pass03/):

- Android: 13/13. 각 scene의 exact ready summary, trace chunks, scene summary 뒤 draw/present가 확인됐다.
- iOS: 같은 검사를 거쳐 13/13. pass03은 이전 iOS draw log가 현재 scene 성공으로 오인되지 않도록 runner를 보강했다.
- 양 플랫폼 log의 `runtime_text_count=0`, `glyph_batch_draw_count=0`이다. screenshot은 layout box와 app surface만 보여 glyph paint 또는 최종 compositor pixel을 입증하지 않는다.
- [Android 대표 화면](runs/c114-legacy-fallback-20261028-pass03/android-avd/screenshot.png) · [iOS 대표 화면](runs/c114-legacy-fallback-20261028-pass03/ios-simulator/screenshot.png) · [기계 검증 요약](runs/c114-legacy-fallback-20261028-pass03/post-run-verification.json)

## 보존한 runner 수정 기록

- [pass01](runs/c114-legacy-fallback-20261028-pass01/)은 첫 Android scene 뒤 중단했다. app matcher는 `status=0`, 예상 root/item frame, `layout=ready boxes=2`를 보고했지만, 복사한 runner가 앞선 two-item matrix의 3-box 조건을 재사용했다. 제품 실패가 아니라 runner 기대값 오류였다.
- [pass02](runs/c114-legacy-fallback-20261028-pass02/)는 box 수를 2로 고친 뒤 양 플랫폼에서 13/13 완료했다. 이후 iOS runner가 현재 정확한 run-ID summary 뒤의 draw/present를 요구하도록 보강했고, pass03에서 전체 matrix를 다시 실행했다.
- 시도별 결과와 log를 보존해 harness 수정과 app matcher 결과를 구분했다.

## 경계와 상태

화면은 대표 layout evidence이지 text ink 증거가 아니다. `runtime_text_count=0`, `glyph_batch_draw_count=0`이며 Android 결과는 emulator, iOS 결과는 Simulator다. 실기기, Android hardware GPU, iOS compositor 이후 pixel, 일반 author CSS admission, WPT harness, 전체 C11.4 지원 또는 CI 통과는 이 근거로 확인되지 않았다. C11.4는 미완료이고 공식 완료 집계는 `72/213`이다.
