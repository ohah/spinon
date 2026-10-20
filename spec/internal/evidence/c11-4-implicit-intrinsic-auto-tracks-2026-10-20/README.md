# C11.4 implicit intrinsic auto-track 구현 근거

이 근거는 `RuntimeBlockFlexGridTrackSizingV1`의 양의 implicit row/column에 `grid-auto-rows`와 `grid-auto-columns` track-size 목록을 적용한 구현 slice를 검증한다. C11.4 전체 완료나 상위 C11 완료를 뜻하지 않는다.

## 실행 결과

| 대상 | 결과 | 실행 정보 |
| --- | --- | --- |
| Chrome oracle | 20/20 case가 규범-derived track/frame 기대값과 일치 | [사전 비교](../c11-4-implicit-intrinsic-auto-tracks-precomparison-2026-10-20/README.md), Chrome 155.0.8059.85, Agent Browser 0.31.1 |
| Android | 73/73 scene 통과, 각 scene의 frame·track trace·draw/present 통과 | [AVD 결과](android-avd-c114-implicit-intrinsic-android-avd-20261020T032916132Z-bdce45bc/result.json), Android API 37 ARM64 `sdk_gphone16k_arm64` |
| iOS | 73/73 scene 통과, 각 scene의 frame·track trace·draw/present 통과 | [Simulator 결과](ios-simulator-c114-implicit-intrinsic-ios-simulator-20261020T033027363Z-dd058ebc/result.json), iPhone 17 Pro, iOS 26.2 |

두 새 고정축/자동 반대축 scene은 Chrome, Android, iOS에서 동일한 결과를 냈다.

| Scene | Track list | Item frame |
| --- | --- | --- |
| `row-positive-implicit-locked-item` | rows `12px 20px` | x=0, y=12, width=40, height=20 |
| `column-positive-implicit-locked-item` | columns `24px 70px` | x=24, y=0, width=70, height=40 |

border 밖으로 나오는 `implicit-min-content-overflow-border`는 Android PixelCopy와 iOS Metal surface에서 각각 5/5 표본이 정확히 일치했다. 화면은 [Android](android-avd-c114-implicit-intrinsic-android-avd-20261020T032916132Z-bdce45bc/implicit-min-content-overflow-border.png), [iOS](ios-simulator-c114-implicit-intrinsic-ios-simulator-20261020T033027363Z-dd058ebc/implicit-min-content-overflow-border.png)에서 확인할 수 있다.

실기기는 연결하지 않았다. Android 결과는 API 37 AVD이며, Apple 결과는 iOS Simulator다.

## 빌드와 수집기 수정

```sh
SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1 mise exec -- bun run build:android
SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1 mise exec -- bun run build:ios-sim
```

첫 AVD 수집 시 이전 실행 로그까지 포함하는 무제한 `adb logcat -d`가 120초 후 `spawnSync adb ETIMEDOUT`으로 끝났다. 원인과 부분 결과는 [진단 기록](android-avd-c114-implicit-intrinsic-android-avd-20261020T030912380Z-910071e2/failure-diagnosis.md)에 보존했다. 수집기는 현재 app PID와 최근 1,000줄만 읽도록 수정했고, AVD를 다시 부팅한 뒤 위의 73/73 실행을 완료했다.

최종 AVD 실행의 runtime log 3줄은 끝에 붙은 ASCII 공백 1 byte씩만 제거했다. key/value 레코드와 실행 결과는 바꾸지 않았다.

## 코드 검증

| 검사 | 결과 |
| --- | --- |
| `cargo fmt --all -- --check` | 통과 |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | 통과 |
| Layout C11.4 집중 테스트 | 22 passed |
| Style cascade / projection 집중 테스트 | 5 passed / 3 passed |
| FFI 20-scene allowlist + 기본 FFI compile | 1 passed / `cargo check -p spinon-ffi --lib --locked` 통과 |
| 기본/opt-in FFI artifact boundary | [산출물 검사](ffi-artifact-boundary.md): 기본 archive 0/20 scene ID·fixture symbol 없음, opt-in archive 20/20 및 3 fixture symbol 존재 |
| `cargo test --locked --workspace` | 199 passed, 1 기존 실패: `auto_repeat_explicit_neighbor_matrix_preserves_tracks_and_frames` (expected 154, actual 162). 이 실패는 구현 전 `origin/main`에서도 재현했다. |
| `bun run test:css-reference` | C11.4 관련 검사는 통과. 변경하지 않은 runtime viewport, C11.3.2 entry, C13.3 tap route 2개 소스 검사에서 기존 실패 4개. |
| Chrome 사전 비교 SHA-256 | 입력 fixture·폰트·환경·관찰 JSON·화면·규범 기대값 6개 모두 일치 |

최종 코드의 Android APK SHA-256은 `d1861f6d60513bcca51c78e14c7bce4b69b8e27301ad740de0f26716eb60ff02`, iOS app SHA-256은 `192ae0dad69be4250d29550b4b518f1b6aa52c7a9172ab56e6111629074ace8f`다. 구현/runtime 20관점 및 구현 PR diff 30관점 검토를 마쳤다. [PR #375](https://github.com/ohah/spinon/pull/375)은 2026-10-20 Rebase and merge됐고 merge SHA는 `e01f0dfc386e5d77ad0cdf8fb3e3b11df40696f8`다. GitHub required status check은 설정되지 않았다. 이번 상태·계약·계획·roadmap 문서 diff도 [별도 30관점 검토](roadmap-pr-diff-review-30.md)를 거쳤다. Tailnet 미리보기 동기화는 후속 문서 PR에서 다룬다.

## 남은 범위

- C11.4 intrinsic contribution matrix, negative implicit track, nested/anonymous item, full auto-repeat placement, WPT 적합성은 이 근거에서 다루지 않는다.
- [내부 계약 0059](../../0059-c11-grid-layout.md)와 [상태 대장](../../STATUS.md)에 부분 slice와 미완료 범위를 연결했다. PR #375는 merge됐고 C11.4/C11 체크와 완료 수는 그대로다. Tailnet 미리보기 동기화는 별도 후속 PR로 진행한다.
