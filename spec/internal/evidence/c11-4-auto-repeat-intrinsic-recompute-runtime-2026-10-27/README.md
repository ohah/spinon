# C11.4 auto-repeat intrinsic-size 재계산 실행 근거

## 범위

WPT `grid-change-intrinsic-size-with-auto-repeat-tracks-001.html`의 8개 입력을 고정 기대값에 대조했다. 기준 WPT revision은 `c999c58338ee1d223df5ad62e48be1202ced0d37`이며, Chrome 156 사전 관측과 독립 산술 기대값은 [precomparison evidence](../c11-4-grid-change-intrinsic-auto-repeat-precomparison-2026-10-27/README.md)에 있다. 이 실행은 전체 WPT 또는 C11.4 완료 판정이 아니다.

Rust runtime 회귀는 하나의 `HostDocument`와 같은 여섯 node handle을 유지하며 `max-content` 네 폭을 거친 뒤 `min-content` 네 폭으로 바꾼다. 각 변경에서 generation 유지, document/render-tree/style revision 진행, dirty-root invalidation, 최종 track·item·absolute probe geometry 및 trace source revision을 검사한다. Android와 iOS 앱 행렬은 같은 8개 입력을 각각 독립 scene으로 실행하므로, 앱 문서 mutation 재계산 근거로 세지 않는다.

## 최종 시뮬레이터 결과

| 대상 | 실행 결과 | 앱 해시 | 화면 |
| --- | --- | --- | --- |
| Android API 37 ARM64 AVD (`sdk_gphone16k_arm64`, Android 17) | 8/8 통과. 각 scene `drawPresented=true`, track trace 5–8 chunks. 설치 APK와 빌드 APK SHA-256 일치. | `c3d7e9cdbc7f82d9ee4b75a956dc9e6ecfa2a4262daa6003beb7c5403fd55d2c` | [min-content-120 화면](android-avd-c114-intrinsic-auto-repeat-recompute-android-avd-20261027T053932723Z-473bc97f/intrinsic-auto-repeat-min-content-120.png) |
| iPhone 17 Pro / iOS 26.2 Simulator | 8/8 통과. 각 scene `drawPresented=true`, track trace 10–15 chunks. 설치 앱과 빌드 앱 SHA-256 일치. | `eb67aca7a231964159818760c84b7290eb624695bcf7c3cb45a0e27ddff05aa4` | [min-content-120 화면](ios-simulator-c114-intrinsic-auto-repeat-recompute-ios-simulator-20261027T053959272Z-7a820112/intrinsic-auto-repeat-min-content-120.png) |

두 실행은 동일한 base commit `8eca593bc7020d5554e42713d42d06372c5eca33`, 동일한 tracked patch SHA-256 `188293122bda38e6154ea8e6e7c53b56ebb53e851044dbbdc5feaa244d339421`, 동일한 source snapshot SHA-256 `0ffdb3460c594bfc232cdabad6d81baf942a89ce18ca08f644c6101e28fc09b2`를 기록한다. 연결한 Android 화면 SHA-256은 `ab4e12cfe34c34abc401fa5b52ed10d7cc6e01e72feb4df9fdf80dfa8c2ecc46`, iOS 화면은 `2413121bfd9b0c332d743703c1027a5496bdd01d597722dc0ab737ab365fc475`다. 전체 per-scene frames, tracks, trace counts, device fingerprint와 해시는 각 실행의 [Android result](android-avd-c114-intrinsic-auto-repeat-recompute-android-avd-20261027T053932723Z-473bc97f/result.json), [Android simulator metadata](android-avd-c114-intrinsic-auto-repeat-recompute-android-avd-20261027T053932723Z-473bc97f/simulator.json), [iOS result](ios-simulator-c114-intrinsic-auto-repeat-recompute-ios-simulator-20261027T053959272Z-7a820112/result.json), [iOS simulator metadata](ios-simulator-c114-intrinsic-auto-repeat-recompute-ios-simulator-20261027T053959272Z-7a820112/simulator.json)에 있다.

Android 앱은 고정 V8 source `/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8`로 빌드했다. 시뮬레이터 screenshot은 표시 장면과 상태를 확인하지만 Android hardware GPU, 실기기, iOS compositor 이후 픽셀을 증명하지 않는다. `drawPresented`와 trace는 앱의 layout·제출 경로 증거다.

## Rust 및 빌드 검증

- `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test c114_auto_repeat_tracks_recompute_after_intrinsic_item_changes_in_one_document -- --nocapture` — 1/1 통과.
- `cargo test --locked -p spinon-layout --features c11-4-grid-track-sizing-fixture-test absolute_auto_repeat_probe_exception_requires_the_exact_wpt_matrix -- --nocapture` — 1/1 통과. WPT 폭 밖의 `90px` item, 잘못된 probe inset·line·width, root gap·named line·순서·hidden 상태를 거부한다.
- `cargo clippy --locked -p spinon-layout --features c11-4-grid-track-sizing-fixture-test --all-targets -- -D warnings` — 통과.
- 이전 focused 검증: style-to-layout auto-repeat projection 8/8, FFI 8-scene source test 1/1, runtime focused test 1/1, fixture-feature Clippy, workspace default strict Clippy, `cargo fmt --all -- --check`, JS syntax 검사 통과.
- Android Debug APK와 iOS Simulator 앱을 보강된 source로 다시 빌드한 뒤 최종 8-scene 행렬을 새로 실행했다. default `spinon-layout`/`spinon-ffi` artifact와 fixture-test positive control의 fixture 경계도 확인했다. 이 검사는 static Rust artifact 범위이며 APK/IPA Release exclusion을 증명하지 않는다.

## 수정 전 실행 기록

초기 시도 디렉터리는 진단 원본으로 보존한다. 최초 layout 검증은 out-of-flow Grid item을 거부했고, 후속 검증에서는 폭 0인 absolute probe가 frame 목록에는 있지만 그려진 box 수에는 포함되지 않는 차이를 발견했다. 검증기는 zero-width probe frame은 그대로 비교하면서 painted box 기대 개수에서는 제외하도록 수정했다. 마지막 scope 검토에서는 예외 검증기가 WPT에 없는 임의의 양수 item 폭을 허용하는 것을 발견해 15·80·100·120px만 허용하고 90px 거부 대조를 추가했다. 수정 뒤 양 플랫폼 앱을 다시 빌드하고 캡처했다.

초기 자료는 [최초 Android 거부](android-avd-c114-intrinsic-auto-repeat-recompute-android-avd-20261027T050449456Z-a1c2189e/), [후속 Android 시도](android-avd-c114-intrinsic-auto-repeat-recompute-android-avd-20261027T050801305Z-6963aa33/), [중간 Android 행렬](android-avd-c114-intrinsic-auto-repeat-recompute-android-avd-20261027T051010896Z-c4af5d98/), [중간 iOS 행렬](ios-simulator-c114-intrinsic-auto-repeat-recompute-ios-simulator-20261027T051040213Z-fe3652bf/), [source hash 재결합 전 Android 행렬](android-avd-c114-intrinsic-auto-repeat-recompute-android-avd-20261027T052756341Z-ad528281/), [source hash 재결합 전 iOS 행렬](ios-simulator-c114-intrinsic-auto-repeat-recompute-ios-simulator-20261027T052938111Z-46551500/), [중간 Android 재검증](android-avd-c114-intrinsic-auto-repeat-recompute-android-avd-20261027T053412908Z-08d1db60/), [중간 iOS 재검증](ios-simulator-c114-intrinsic-auto-repeat-recompute-ios-simulator-20261027T053440887Z-091e02b4/)에 남아 있다. 이를 최종 캡처로 세지 않는다.

이전 Android 시도의 raw logcat 파일 네 개에는 끝 공백이 있어 `git diff --check`에서 제외 대상으로 잡혔다. 원본은 수정하거나 삭제하지 않고 로컬 evidence 디렉터리에 보존했으며, PR에는 공백 검사를 통과한 log와 해당 실행의 parsed `result.json`만 포함한다.

## 완료 범위와 남은 범위

이번 slice는 opt-in fixture 경로의 이 WPT 행렬만 검증한다. 일반 intrinsic-neighbor CSS admission, 임의 absolute Grid placement, 전체 WPT, Android 실기기/hardware GPU, iOS 실기기/compositor 이후 pixels는 미검증이다. `spec/STATUS.md`의 C11.4 checkbox는 미완료로 두며 공식 완료 집계 `72/213`도 바꾸지 않는다.
