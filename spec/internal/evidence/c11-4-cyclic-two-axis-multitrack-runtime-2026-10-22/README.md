# C11.4 동시 cyclic percentage 다중 track runtime evidence

## 판정 범위

네 개의 고정 C11.4 fixture scene을 Rust layout, Android API 37 ARM64 AVD, iPhone 17 Pro / iOS 26.2 Simulator에서 실행했다. 이 결과는 fixture profile의 layout·surface 동작 근거다. C11.4 전체 구현, 공개 CSS 적합성, WPT 또는 실기기 확인으로 확대하지 않는다.

기대값은 [집중 계획](../../../../plan/c11-4-cyclic-two-axis-multitrack.md)의 CSS Grid 2025 CRD 계산과 [Chrome 156 사전 비교](../c11-4-cyclic-two-axis-multitrack-precomparison-2026-10-22/README.md)에 고정했다. Chrome은 보조 oracle이며 독립 규범 계산을 대신하지 않는다.

## Scene 결과

모든 장면의 root frame은 `40×20px`, text item은 `(0,0) 20×10px`, 마지막 Grid item은 `(30,15) 20×10px`다. 마지막 item은 root 오른쪽을 10px, 아래쪽을 5px 넘어간다. 좌표는 root 기준이며 모든 gap, border, padding은 0이다.

| Scene | Columns (px) | Rows (px) | AVD | iOS Simulator |
| --- | --- | --- | --- | --- |
| `cyclic-percent-two-axis-multitrack` | `[20, 10, 20]` | `[10, 5, 10]` | pass | pass |
| `cyclic-percent-columns-multitrack` | `[20, 10, 20]` | `[10, 5, 5]` | pass | pass |
| `cyclic-percent-rows-multitrack` | `[20, 10, 10]` | `[10, 5, 10]` | pass | pass |
| `definite-percent-two-axis-multitrack` | `[20, 10, 20]` | `[10, 5, 10]` | pass | pass |

각 scene에서 root/text/overflow node frame과 track trace가 일치했고, draw/present가 성공했다. Rust regression test는 최종 Columns/Rows 세 track을 각각 대조한다.

## Android API 37 ARM64 AVD

- 대상: `emulator-5558`, `sdk_gphone64_arm64`, API 37 / Android 17, `arm64-v8a`.
- 실행 결과와 scene별 원본 로그: [AVD capture](./android-avd-c114-cyclic-percent-tracks-android-avd-20261021T220632656Z-acc26dfe/result.json), [simulator metadata](./android-avd-c114-cyclic-percent-tracks-android-avd-20261021T220632656Z-acc26dfe/simulator.json).
- 네 scene 모두 `drawPresented=true`, 3 boxes, root `40×20`, text `20×10`, overflow `(30,15) 20×10`이다. Columns/Rows 결과는 위 표와 같다.
- PixelCopy matrix는 scene당 5개, 총 20개를 exact로 읽었다. 표본은 text-item 배경 `(10,5)`=`#e5484d`; overflow item 내부 `(35,17)`; root 오른쪽 바깥 `(45,18)`; root 아래 바깥 `(35,22)`; 두 경계 바깥 `(45,22)`이며 초록 표본은 `#20bf6b`다.
- APK SHA-256과 설치 APK SHA-256은 모두 `83633cb667618f53e21498ed456036705dae01c3c61bc05fdbe813a907be0a9f`다.
- 대표 화면: [Android AVD](./android-avd-c114-cyclic-percent-tracks-android-avd-20261021T220632656Z-acc26dfe/cyclic-percent-two-axis-multitrack.png).

## iPhone 17 Pro / iOS 26.2 Simulator

- 대상: `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`, iOS 26.2, drawable `960×720` / scale 3×.
- 실행 결과와 scene별 원본 로그: [iOS capture](./ios-simulator-c114-cyclic-percent-tracks-ios-simulator-20261021T220642593Z-54e24c0a/result.json), [simulator metadata](./ios-simulator-c114-cyclic-percent-tracks-ios-simulator-20261021T220642593Z-54e24c0a/simulator.json).
- 네 scene 모두 `drawPresented=true`, 3 boxes, frame과 track 배열은 위 표와 같다.
- Metal drawable 표본은 drawable 제출 전에 scene당 5개, 총 20개를 exact로 확인했다. 빨강 text-item 배경과 grid 안·오른쪽·아래·두 경계 바깥의 초록 overflow 표본을 검사했다.
- 앱 SHA-256은 `294d72dfbd115ce3667dae3aa4f2a2265593ea2138804fade7680a239f86ddca`다. pixel matrix를 활성화한 빌드에는 `SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1`을 사용했다.
- 대표 화면: [iOS Simulator](./ios-simulator-c114-cyclic-percent-tracks-ios-simulator-20261021T220642593Z-54e24c0a/cyclic-percent-two-axis-multitrack.png).

| Screenshot | SHA-256 |
| --- | --- |
| Android `cyclic-percent-two-axis-multitrack` | `df6bae3407334a9dc5d67a150536b289cce04a818554b38cc92a2a5cd4995fc4` |
| Android `cyclic-percent-columns-multitrack` | `d4036d48b093604a0defeedaf4832aa26b0d5c5d120c35a15f65f16fa00788f3` |
| Android `cyclic-percent-rows-multitrack` | `d4036d48b093604a0defeedaf4832aa26b0d5c5d120c35a15f65f16fa00788f3` |
| Android `definite-percent-two-axis-multitrack` | `d4036d48b093604a0defeedaf4832aa26b0d5c5d120c35a15f65f16fa00788f3` |
| iOS `cyclic-percent-two-axis-multitrack` | `907363ab357d87ff013757af31ab07b71ac854eb7cf4b4c24f82a7b95ef05211` |
| iOS `cyclic-percent-columns-multitrack` | `5c0b0e07ee349e6f5b4f58e0e8cb535cc25778b9654accbc66618b60e1f65304` |
| iOS `cyclic-percent-rows-multitrack` | `b12f00eec9c71d894a187d0f307067e4ed3c9a08a985f36d03393cd9eb1f13d8` |
| iOS `definite-percent-two-axis-multitrack` | `b12f00eec9c71d894a187d0f307067e4ed3c9a08a985f36d03393cd9eb1f13d8` |

## 고정 소스와 재현성

양쪽 capture의 source revision은 commit `3bbdbd4c379564e98663dee892588c4d4f44b706`, tree `c6309e5d81f8656436b73b5a601613eac0c3ae92`이며 `trackedDirtyPaths`는 비어 있다. capture별 `trackedDiffSha256`은 빈 diff의 SHA-256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`다.

기본 `spinon-ffi` debug staticlib에는 신규 scene ID 문자열이 없고, `c04-runtime-gpu-fixture-test`를 켠 positive-control staticlib에는 있다. 이 검사는 `strings target/debug/libspinon_ffi.a`로 확인했다. 기본 산출물 SHA-256은 `a7a50b7375b4c44b95dba40bf5075508ffe4647f457e8e24f71c0e606d160635`, fixture-test 산출물 SHA-256은 `339064e466fc7d843d3212ca2965249df23e966774a2e0b0536e8d566c2a21a2`다. 두 결과는 release artifact나 최종 앱 번들 전체 크기를 증명하지 않는다.

## 수행한 검증

- `mise exec -- cargo test -p spinon-style-to-layout --features c11-4-grid-track-sizing-fixture-test cyclic_percentages_size_both_three_track_axes_and_preserve_cell_overflow` — 통과.
- `mise exec -- cargo test -p spinon-ffi --features c04-runtime-gpu-fixture-test exposes_cyclic_percentage_two_axis_multitrack_runtime_scenes` — 통과.
- `mise exec -- cargo test -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test exposes_only_the_four_fixed_two_axis_multitrack_text_scenes` — 통과.
- `mise exec -- cargo fmt --all -- --check` — 통과.
- `mise exec -- cargo clippy --locked -p spinon-ffi --all-targets --features c04-runtime-gpu-fixture-test -- -D warnings` — 통과.
- `mise exec -- node --test tools/css-reference/fixture-module-boundary.test.mjs` — 6/6 통과.
- `mise exec -- node --check tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs` — 통과.
- Android/iOS Simulator 앱 빌드와 위 네 scene matrix — 각 4/4 통과.

## 한계

화면의 빨강 표본은 text item의 배경색이다. 앱 renderer 로그에서 `runtime_text_count=0`, `glyph_batch_draw_count=0`이므로 Ahem glyph paint·raster equality는 확인하지 않았다. Android 결과는 AVD의 PixelCopy이며 실기기나 hardware GPU가 아니다. iOS 표본은 Metal drawable 제출 전 readback이며 compositor 이후 화면 표본이 아니다. border stroke, clipping 조합, 다른 percentage/track 문법, WPT, Android 실기기와 전체 C11.4는 범위 밖이다. 공식 C11.4 체크와 완료 수 `72/213`은 유지한다.

초기 반복의 진단 캡처는 workspace에 그대로 남겨 두고 이 evidence commit에는 포함하지 않았다. 위 링크는 source commit이 고정된 최종 성공 실행만 가리킨다.
