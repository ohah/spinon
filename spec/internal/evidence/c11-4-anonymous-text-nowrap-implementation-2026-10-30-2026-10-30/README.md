# C11.4 anonymous Grid text `white-space:nowrap` 구현 근거

**판정:** PR 후보의 제한 opt-in fixture 구현이다. C11.4와 C11은 미완료이며 공식 집계 `72/213`은 그대로다.

## 범위

기존 anonymous Grid text fixture가 `white-space:normal`만 받던 경계를 `normal | nowrap`으로 좁게 넓혔다. 새 normal control과 nowrap scene은 Ahem `10px/10px`, `XX XX`, `width:20px`, 단일 `min-content` column, auto row를 쓴다. 기존 `pre-line`, 비영 `order`, 일반 author CSS admission은 계속 거부한다. Chrome 기준과 입력 hash는 [사전 비교](../c11-4-anonymous-text-nowrap-precomparison-2026-10-30/README.md)에 있다.

## 실행 결과

| 대상 | 장면 | root | anonymous item | min-content column | trace / present |
| --- | --- | ---: | ---: | ---: | --- |
| Android API 37 ARM64 AVD `emulator-5554` | `anonymous-normal-min-content` | 20×20px | 20×20px, 두 줄 | 20px | 7 chunks, draw/present 성공, retry 0 |
| Android API 37 ARM64 AVD `emulator-5554` | `anonymous-nowrap-min-content` | 20×10px | 50×10px, 한 줄 | 50px | 7 chunks, draw/present 성공, retry 0 |
| iPhone 17 Pro / iOS 26.2 Simulator | `anonymous-normal-min-content` | 20×20px | 20×20px, 두 줄 | 20px | 13 chunks, draw/present 성공, retry 0 |
| iPhone 17 Pro / iOS 26.2 Simulator | `anonymous-nowrap-min-content` | 20×10px | 50×10px, 한 줄 | 50px | 13 chunks, draw/present 성공, retry 0 |

- [Android 실행 결과](./android-avd-c114-anonymous-grid-text-android-avd-20261030T141543872Z-5daf0288/result.json), [기기·소스 revision](./android-avd-c114-anonymous-grid-text-android-avd-20261030T141543872Z-5daf0288/simulator.json), [normal 원본 로그 gzip](./android-avd-c114-anonymous-grid-text-android-avd-20261030T141543872Z-5daf0288/runtime-anonymous-normal-min-content.log.gz), [nowrap 로그](./android-avd-c114-anonymous-grid-text-android-avd-20261030T141543872Z-5daf0288/runtime-anonymous-nowrap-min-content.log).
- [iOS 실행 결과](./ios-simulator-c114-anonymous-grid-text-ios-simulator-20261030T141240869Z-a5f37fb0/result.json), [기기·소스 revision](./ios-simulator-c114-anonymous-grid-text-ios-simulator-20261030T141240869Z-a5f37fb0/simulator.json), [normal 로그](./ios-simulator-c114-anonymous-grid-text-ios-simulator-20261030T141240869Z-a5f37fb0/runtime-anonymous-normal-min-content.log), [nowrap 로그](./ios-simulator-c114-anonymous-grid-text-ios-simulator-20261030T141240869Z-a5f37fb0/runtime-anonymous-nowrap-min-content.log).
- Android 캡처: [normal](./android-avd-c114-anonymous-grid-text-android-avd-20261030T141543872Z-5daf0288/anonymous-normal-min-content.png), [nowrap](./android-avd-c114-anonymous-grid-text-android-avd-20261030T141543872Z-5daf0288/anonymous-nowrap-min-content.png).
- iOS 캡처: [normal](./ios-simulator-c114-anonymous-grid-text-ios-simulator-20261030T141240869Z-a5f37fb0/anonymous-normal-min-content.png), [nowrap](./ios-simulator-c114-anonymous-grid-text-ios-simulator-20261030T141240869Z-a5f37fb0/anonymous-nowrap-min-content.png).

캡처는 앱 surface 실행과 draw/present 경로를 보여준다. 이 캡처의 glyph pixels로 줄바꿈을 판정하지 않았다. 줄 수·anonymous item frame·intrinsic size·track 기여는 trace/log와 독립된 Rust 기대값으로 판정했다. PixelCopy exactness, Android 실기기·hardware GPU, iOS 실기기·compositor 제출 뒤의 표면은 확인하지 않았다.

## 로컬 검사

| 검사 | 결과 |
| --- | --- |
| `spinon-layout` anonymous text focused tests, `c11-4-grid-track-sizing-fixture-test` | 10 passed |
| `spinon-runtime` anonymous text focused tests, `c11-4-grid-track-sizing-fixture-test` | 3 passed |
| `spinon-ffi` fixed seven-scene matrix, `c04-runtime-gpu-fixture-test` | 1 passed |
| `tools/css-reference/fixture-module-boundary.test.mjs` | 23 passed |
| `cargo fmt --all -- --check` | passed |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | passed |

## Opt-in artifact 경계

macOS ARM64 Rust 1.96.1 release static archive의 [기본 product feature와 fixture-test positive control 비교](./artifact-boundary/README.md)를 했다. 새 두 scene ID와 C11.4 host FFI exports는 `c04-runtime-gpu` archive에 없고 `c04-runtime-gpu-fixture-test` archive에는 있다.

제품 archive scan에는 이 변경 전에 존재하던 `RuntimeSession::new_runtime_gpu_block_flex_grid_track_sizing_fixture` Rust symbol도 남아 있었다. 이 변경은 그 runtime 내부 경로를 추가하거나 수정하지 않았으며, fixture FFI exports와 새 scene payload가 opt-in 경계 밖으로 나오는 것으로 해석하지 않았다. 따라서 검증 범위는 **이 두 scene의 문자열/payload 및 host FFI export**에 한정하며 전체 runtime의 모든 C11.4 내부 symbol이 없는 것으로 주장하지 않는다. 정적 archive 결과는 최종 APK/IPA 검증이 아니다.

## 실패 및 복구 기록

- 첫 Android 실행은 validator가 새 `height:auto`를 pixel height로 취급해 실패했다. validator를 `ComputedCssDimension::Auto`와 대조하도록 고치고 focused runtime test를 다시 통과시켰다. 원본 로그는 `android-avd-c114-anonymous-grid-text-android-avd-20261030T140406381Z-d529725d/`에 보존했다.
- 다음 실행은 기존 장시간 실행 AVD에서 SurfaceFlinger AIDL을 사용할 수 없어 draw/present가 성립하지 않았다. 해당 결과를 성공으로 세지 않았다. 별도 API 37 ARM64 AVD `spinon_api37_0_applied_txn`을 cold boot해 최종 두 장면을 다시 실행했다. 실패 로그 원본은 [gzip 파일](./android-avd-c114-anonymous-grid-text-android-avd-20261030T140507016Z-79291521/runtime-anonymous-normal-min-content.log.gz)로 보존했다.

Android trace 원본의 줄 끝 공백 때문에 두 `runtime-anonymous-normal-min-content.log` 파일은 byte-preserving gzip으로 저장했다. 로컬 checkout에는 압축 전 파일도 보존한다.

## 검토 및 미검증 범위

- [구현/runtime 적대 검토 20개](./implementation-runtime-review-20.md)
- Chrome 기준, 일반 CSS cascade, 임의 글꼴·스크립트, WPT 적합성, 실제 기기, compositor 이후 pixels, 전체 C11.4와 C11은 검증하지 않았다.
- 공식 상태는 [구현 대장](../../../STATUS.md)에서 `72/213`, C11.4 미완료로 유지한다.
