# C11.4 definite-height percentage `fit-content()` row

이 기록은 pinned WPT [`fit-content-track-sizing.html`](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html)의 percentage argument assertion 하나를 고정한 opt-in 앱 fixture다. Source SHA-256은 `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736`다. 규범 기대값과 기존 Chrome upstream 관찰은 [계획](../../../../plan/c11-4-fit-content-percentage-argument.md)에 분리했다.

## 고정 입력과 판정

- Root content box: `60×300px`; computed `grid-auto-rows:fit-content(50%)`; definite row percentage limit은 `150px`이다.
- 한 implicit row의 final base와 item frame은 `150px`이다. Child frame은 `30×400px`로 row와 root 밖까지 overflow한다.
- Item은 `min-height:0`을 지정한다. Computed overflow는 initial `visible`이며 source에는 explicit `overflow:visible` declaration이 없다.
- Rows trace는 `Initialize`, `IntrinsicItemBatch(span:1)`, `IntrinsicSizingComplete`, `MaximizeTracks`, `ExpandFlexibleTracks`, `StretchAutoTracks(applied:true)` 여섯 단계와 base sequence `[0,0,0,150,150,150]px`를 검사한다. Row growth limit은 Initialize에서 unresolved이고 뒤 단계에는 400px이다. Contribution vector는 `Minimum 0` (`SpecifiedMinimum`), `MinContent 400`, `MaxContent 400`이다.
- Raw trace의 growth limit `400px`은 max-content bound다. CSS Grid 2025 CRD [§7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes)와 [§11.5.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items)에서 fit-content argument가 base-size distribution을 `150px`으로 제한하므로 raw growth-limit field를 used track size로 읽지 않는다.
- 표면 표본은 row 안 child `#ffd166`, row 안 item `#e5484d`, row 아래 root background `#17212b`, root 외부 child `#ffd166`이다. 각 surface에서 네 표본 모두 RGBA exact다.

## Android API 37 ARM64 AVD

AVD `emulator-5554` (`sdk_gphone16k_arm64`, API 37, `arm64-v8a`)에서 장면 1/1, node frame 3/3, row base `150px`, 네 PixelCopy 표본 4/4, draw presentation identity 일치를 확인했다. AVD 결과는 물리 기기나 hardware GPU를 입증하지 않는다.

- 최종 수정된 validator로 다시 빌드한 APK SHA-256: `f8622826361a068f688bdb832e0b686db28ab99dac3829c82ad187e2edbf62a7`
- Source commit `98b8a7ff8964a21534877bb223f54b7180cbfca1`, tracked diff는 비어 있다. Source snapshot SHA-256은 `95c3d59df304ce5bbc50f126f42ecb45e873ebaea7702435f8a91aa368788d02`이며 manifest에는 checkout에 있던 두 unrelated untracked path `tools/tailnet-preview/c11-4-anonymous-grid-border-overflow-pass03.rspress.config.ts`와 `vendor/taffy/Cargo.lock`도 기록되어 있다. 두 path는 PR에 포함되지 않았다.
- [최종 실행 결과 JSON](./android-avd-c114-fit-content-percentage-argument-android-avd-20261028T203836259Z-305b93a9/result.json) · [simulator/source/hash](./android-avd-c114-fit-content-percentage-argument-android-avd-20261028T203836259Z-305b93a9/simulator.json) · [runtime trace와 PixelCopy](./android-avd-c114-fit-content-percentage-argument-android-avd-20261028T203836259Z-305b93a9/runtime-fit-content-percentage-row-wpt.log)

![Android API 37 AVD PixelCopy 실행 화면](./android-avd-c114-fit-content-percentage-argument-android-avd-20261028T203836259Z-305b93a9/fit-content-percentage-row-wpt.png)

## iPhone 17 Pro iOS 26.2 Simulator

iPhone 17 Pro / iOS 26.2 Simulator에서 장면 1/1, node frame 3/3, row base `150px`, 여덟 trace chunk, 네 Metal drawable 표본 4/4, draw presentation identity 일치를 확인했다. Metal readback은 OS compositor 제출 전이다.

- 최종 수정된 validator로 다시 빌드한 app SHA-256: `452ac53200d3f04dda1670e220c18d3e877359a35e12a6416d32ab0c26fe546a`
- Source commit과 snapshot hash는 Android 결과와 같다. manifest에 있는 두 unrelated untracked path는 PR에 포함되지 않았다.
- [최종 실행 결과 JSON](./ios-simulator-c114-fit-content-percentage-argument-ios-simulator-20261028T203925688Z-404d2893/result.json) · [simulator/source/hash](./ios-simulator-c114-fit-content-percentage-argument-ios-simulator-20261028T203925688Z-404d2893/simulator.json) · [runtime trace와 Metal readback](./ios-simulator-c114-fit-content-percentage-argument-ios-simulator-20261028T203925688Z-404d2893/runtime-fit-content-percentage-row-wpt.log)

![iPhone 17 Pro Simulator Metal 표본 통과 화면](./ios-simulator-c114-fit-content-percentage-argument-ios-simulator-20261028T203925688Z-404d2893/fit-content-percentage-row-wpt.png)

## 빌드와 검사

- Android build: `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 tools/build-android-app.sh` — 최종 validator로 성공. [로그](./android-build-review-final.log)
- iOS Simulator build: `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1 tools/build-ios-sim.sh` — 최종 validator로 성공. Xcode가 vendored V8 archive object의 중복 timestamp debug-map 경고를 출력했으나 build는 성공했다. [로그](./ios-build-review-final.log)
- Runtime focused tests 2/2 ([로그](./checks/runtime-test-review-final.log)), FFI exact-route test 1/1 ([로그](./checks/ffi-test-review-final.log)), runtime/FFI fixture-feature Clippy ([runtime](./checks/runtime-clippy-review-final.log), [FFI](./checks/ffi-clippy-review-final.log)), formatter ([로그](./checks/fmt-review-final.log)), capture-script syntax ([로그](./checks/capture-script-syntax-review-final.log))와 `git diff --check` ([로그](./checks/diff-check-review-final.log)) 통과.
- `fixture-module-boundary.test.mjs` 18/18 통과. [로그](./checks/fixture-module-boundary-review-final.log)
- 최신 source의 macOS ARM64 static FFI archive에서 기본 `c04-runtime-gpu`는 marker 0개, opt-in `c04-runtime-gpu-fixture-test` archive는 fixture marker를 포함한다. [archive 결과와 SHA-256](./archive-boundary/result-review-final.txt) · [product build](./archive-boundary/product-build-review-final.log) · [fixture build](./archive-boundary/fixture-test-build-review-final.log). APK/IPA 전체 exclusion은 검사하지 않았다.
- PR diff 실패 관점 30개 검토는 [별도 기록](./pr-diff-review-30.md)에 있다. 최초 pass에서 찾은 단계별 growth-limit 기대값 오류와 수정된 최종 AVD/iOS 실행을 함께 기록했다.

## 수정한 중간 실패와 범위

초기 Android surface parser는 240px 아래 좌표를 거부해 `y=350` 표본을 만들지 못했다. Percentage scene에만 `320×400` CSS px viewport와 상한 검사를 적용했다. Profile이 명시적 `overflow:visible` style을 거부해 fixture는 initial computed value를 확인하도록 고쳤다. iOS는 새 scene의 viewport와 presentation identity가 서로 다른 시점의 값을 비교해 strict match에서 실패했으며 scene 전용 viewport와 내부 style/layout revision 검사를 맞췄다. Capture runner는 strict failure를 바로 반환하도록 바꿨다. PR diff 검토에서는 track trace validator가 `Initialize` 단계의 미정 growth limit과 이후 `400px` 값을 구분하지 않은 기대값 오류를 찾았다. 기대 trace와 변조 회귀 검사를 고쳤고 최종 AVD/iOS Simulator 실행이 모두 통과했다. [기대값 오류 AVD 로그](./android-avd-c114-fit-content-percentage-argument-android-avd-20261028T203642666Z-8ac55e2a/runtime-fit-content-percentage-row-wpt.log). 이전 실패 실행 자료는 각 timestamp evidence directory에 보존하고 이 README는 마지막 통과 실행을 가리킨다.

이 실행은 pinned WPT assertion 하나의 opt-in runtime 검증이다. Spinon WPT harness 실행, 일반 author CSS admission, 임의 Grid tree, 전체 WPT suite, Android 실기기/hardware GPU, iOS compositor 이후 pixel 및 전체 C11.4 지원을 확인하지 않았다. C11.4는 미완료이며 공식 완료 `72/213`을 유지한다.
