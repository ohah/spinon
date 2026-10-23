# C11.4 percentage auto-repeat neighbors · 실행·artifact 근거

## 판정

현재 checkout의 제한 후보에서 13개 고정 scene이 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 13/13 통과했다. 각 플랫폼에서 track 목록·collapse·node frame·draw/present를 확인했다. Android PixelCopy와 iOS compositor 제출 전 Metal drawable 표본은 각각 15/15 exact다. 고정 표본은 렌더 전체 또는 실제 화면의 pixel equality를 뜻하지 않는다.

실행 검증 기록 당시에는 구현 PR 전 candidate였다. 구현 PR [#580](https://github.com/ohah/spinon/pull/580)은 2026-10-23T20:46:27Z Rebase and merge되어 SHA `33b00d077147ad9f2102af3220daa2c6cf74e2f9`를 남겼다. C11.4 공식 상태는 미완료이고 `spec/STATUS.md`의 공식 표시 수 `72/213`을 바꾸지 않는다. Chrome 156 `col-fit-first` CSSOM 관측 196px은 CSS Grid 2025 CRD로 계산한 200px과 분리한다.

## 코드·빌드 출처

- 기준 commit: `f240901347e8f6c3074c3445a1315e8aab34f06a`
- 기준 tree: `d4031f09c3341b9a22b72ba22a8e39ffbca9f5e3`
- 시뮬레이터 기록의 tracked dirty diff SHA-256: `10c04178e8e222b6e05946a5df49dcf77efbf889364b9480206e6b736aa6ecf4`
- 위 diff hash는 시뮬레이터 build 시점의 tracked diff이며 untracked source 세 개를 포함하지 않는다. 시뮬레이터 실행 뒤에는 evidence 경로를 가리키도록 문서 링크만 갱신했고 Rust/Java/Swift/runtime capture 코드는 바뀌지 않았다. 현재 untracked source hash는 [untracked-source-sha256.txt](untracked-source-sha256.txt)에 기록했다.
- Android APK SHA-256과 설치본 일치는 [Android simulator.json](android-avd-c114-percentage-neighbors-android-avd-20261023T203439598Z-425fea5b/simulator.json) 및 [installed APK 검증](android-avd-c114-percentage-neighbors-android-avd-20261023T203439598Z-425fea5b/installed-apk-verification.json)에 있다: `f8c56499bd1bab53c761b66507f7f1cadd8cad666441c5cc0c11d4ea684b34cd`.
- iOS 앱 SHA-256: `dc3054ba2d9ba84a19e47b41e3c7e44d2cac1c22174d0fea74eb284a956eb60c`; 대상은 iPhone 17 Pro, iOS 26.2 Simulator다.

## 검증 결과

| 항목 | 결과 |
|---|---:|
| `cargo test --locked -p spinon-layout percentage_neighbors` | 3/3 통과 |
| `cargo test --locked -p spinon-style auto_repeat` | 16/16 통과 |
| `cargo test --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test --lib percentage_neighbor` | 1/1 통과 |
| `cargo test --locked --workspace` | 통과 |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | 통과 |
| `cargo fmt --all -- --check` | 통과 |
| Node syntax·fixture module boundary | 7/7 통과 |
| Android/iOS simulator build | 둘 다 통과 |
| Android API 37 ARM64 AVD runtime | 13/13 scene, PixelCopy 15/15 exact |
| iPhone 17 Pro/iOS 26.2 Simulator runtime | 13/13 scene, Metal drawable 15/15 exact |

FFI artifact 경계는 기본 archive와 opt-in fixture-test archive를 각각 release build한 뒤, `strings` 전체 출력이 끝날 때까지 marker scan을 실행해 확인했다. [기본 archive log](ffi-default-build.log), [기본 marker 판정](ffi-default-marker-scan.txt), [기본 hash/크기](ffi-default-sha256.txt)와 [fixture archive log](ffi-fixture-build.log), [fixture marker 판정](ffi-fixture-marker-scan.txt), [fixture hash/크기](ffi-fixture-sha256.txt)를 보존한다. 기본 archive(60,208,296 bytes, SHA-256 `6a0601102f2ebac806e715fb410b99aea751b2f9d7f6ae036ff4b1fbd0e6afa9`)에는 percentage-neighbor scene marker가 없고, opt-in archive(78,274,216 bytes, SHA-256 `cc936570eedf3407df2da128f8138e2d9022207a1f33d3a1cbee1be8e84fdfd6`)에는 module symbol과 `col-fill-both` marker가 있다. 이 결과는 static archive에만 해당하며 APK/IPA 전체 번들 제외를 주장하지 않는다.

최종 Rust/workspace 로그는 검토 가능한 UTF-8 텍스트로 보존했다. PR에 포함한 로그 사본에서만 줄 끝 공백과 끝의 빈 줄을 정리했고, 원본 로그는 checkout에 그대로 남겨뒀다. 원본 SHA-256은 [raw log manifest](raw-log-sha256.txt)에 기록했다. focused/workspace 검사 로그는 [focused layout](layout-focused.normalized.log), [focused style](style-focused.normalized.log), [focused FFI](ffi-focused.normalized.log), [workspace tests](workspace-tests.normalized.log), [workspace Clippy](workspace-clippy.log), [rustfmt](rustfmt.log), [fixture boundary 7/7](fixture-module-boundary.log)에 보존했다. 최종 앱 빌드와 13-scene capture 전체 출력은 [Android build](android-build-final.log), [iOS build](ios-build-final.normalized.log), [Android runtime](android-capture-final.log), [iOS runtime](ios-capture-final.log)에 있다.

## 시뮬레이터 실행 근거

- [Android 성공 행렬](android-avd-c114-percentage-neighbors-android-avd-20261023T203439598Z-425fea5b/result.json): `emulator-5558`, API 37, `arm64-v8a`, `sdk_gphone64_arm64`. 13개 runtime log에서 13/13 draw/present와 15/15 PixelCopy sample이 exact다. 예시 화면: [양쪽 neighbor](android-avd-c114-percentage-neighbors-android-avd-20261023T203439598Z-425fea5b/col-fill-both.png), [auto-fit collapse](android-avd-c114-percentage-neighbors-android-avd-20261023T203439598Z-425fea5b/col-fit-first.png). AVD의 Android path는 ANGLE/SwiftShader이며 hardware GPU 결과가 아니다.
- [iOS 성공 행렬](ios-simulator-c114-percentage-neighbors-ios-simulator-20261023T203501591Z-007b4f1b/result.json): iPhone 17 Pro, iOS 26.2, UDID `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`. 13개 runtime log에서 13/13 draw/present와 15/15 compositor 제출 전 Metal drawable sample이 exact다. 예시 화면: [양쪽 neighbor](ios-simulator-c114-percentage-neighbors-ios-simulator-20261023T203501591Z-007b4f1b/col-fill-both.png), [auto-fit collapse](ios-simulator-c114-percentage-neighbors-ios-simulator-20261023T203501591Z-007b4f1b/col-fit-first.png).

두 capture는 같은 고정 scene 목록을 사용했다.

```sh
node tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs android-avd emulator-5558 --scene-id=col-fill-both,row-fill-both,col-fill-prefix,row-fill-suffix,col-fit-first,row-fit-first,col-fit-empty,row-fit-empty,col-fill-overflow,row-fill-overflow,col-fill-zero-prefix,col-fill-border-box,row-fill-border-box --run-kind=percentage-neighbors --evidence-group=c11-4-auto-repeat-percentage-neighbors-runtime

node tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs ios-simulator 9DCCE836-7659-4914-B6C1-13F54BFFA9DB --scene-id=col-fill-both,row-fill-both,col-fill-prefix,row-fill-suffix,col-fit-first,row-fit-first,col-fit-empty,row-fit-empty,col-fill-overflow,row-fill-overflow,col-fill-zero-prefix,col-fill-border-box,row-fill-border-box --run-kind=percentage-neighbors --evidence-group=c11-4-auto-repeat-percentage-neighbors-runtime
```

Android build used `SPINON_ENABLE_C04_RUNTIME_GPU=1`, `SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1`, and `SPINON_ENABLE_C11_2_3_GRID_FIXTURE_TEST=1` with `bun run build:android`. iOS build used those flags plus `SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1` with `bun run build:ios-sim`. The iOS pixel flag was necessary: the first run had layout/frame and draw/present but returned `status=-90` for the disabled Metal sample hook. The app was rebuilt with the flag and the complete matrix passed.

## 실패한 시도와 수정

실패한 시도는 원인을 추적할 수 있도록 삭제하지 않았다.

1. [Android run 01](android-avd-c114-percentage-neighbors-android-avd-20261023T201838481Z-a9a33226/runtime-col-fill-both.log)은 새 scene ID가 PixelCopy preflight allowlist에 없어 `scene-index-mismatch`로 끝났다. runner에 13개 scene ID admission을 추가했다.
2. [Android run 02](android-avd-c114-percentage-neighbors-android-avd-20261023T202010529Z-acc2263d/runtime-col-fill-both.log)은 item이 없는 scene의 예상 box 수를 기본 2로 잡아, 실제 root-only 1과 다르게 판정했다. Android/iOS 양쪽의 expected box count routing을 scene frame 목록에 맞췄다.
3. [iOS run 01](ios-simulator-c114-percentage-neighbors-ios-simulator-20261023T202226792Z-c1f54bc9/runtime-col-fill-both.log)은 `SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1` 없이 빌드되어 sample hook이 `status=-90`을 반환했다. 해당 빌드 feature를 켜고 재빌드해 통과했다.
4. 구현 검토에서 각 percentage neighbor가 개별적으로 유한해도 prefix와 suffix의 합이 `f32`를 넘으면 layout이 허용하는 경우를 재현했다. gate가 percentage가 포함된 neighbor 전체 used size를 `f64`로 더해 합산 overflow를 거부하도록 고쳤고, 회귀 검사가 통과했다.
5. 초기 archive scan은 `rg -q`의 early close와 `pipefail`을 함께 사용해 opt-in positive control을 오판했다. `rg`가 전체 stream을 소비하도록 scan을 다시 실행했다. 최종 전체-stream scan에서는 기본 archive에 marker가 없고 opt-in archive에 marker가 있음을 확인했다.

## 재현 명령

```sh
cargo fmt --all -- --check
node --check tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs
node --check tools/css-reference/fixture-module-boundary.test.mjs
node tools/css-reference/fixture-module-boundary.test.mjs
cargo test --locked -p spinon-layout percentage_neighbors
cargo test --locked -p spinon-style auto_repeat
cargo test --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test --lib percentage_neighbor
cargo test --locked --workspace
cargo clippy --locked --workspace --all-targets -- -D warnings
```

## 한계와 다음 관문

이 결과는 고정 13-scene 후보와 두 simulator에 한정된다. 실기기, Android hardware GPU, iOS compositor 이후 픽셀, 일반 author CSS admission, WPT 전체는 검증하지 않았다. Chrome 156의 196px 관측 차이는 규범 oracle과 계속 분리한다. C11.4 전체 지원 또는 완료 판정으로 확대하지 않는다. 구현/runtime 실패 관점 20개 검토는 [별도 문서](implementation-runtime-review-20.md), 독립 PR diff 실패 관점 30개 검토는 [검토 기록](pr-diff-review-30.md)에 있다. PR #580 병합은 완료했다. 병합 당시 required check은 설정되지 않았고 GitHub check은 보고되지 않았다. 병합된 문서 상태를 반영한 Tailnet preview 동기화는 별도 문서 PR 병합 뒤 진행한다.
