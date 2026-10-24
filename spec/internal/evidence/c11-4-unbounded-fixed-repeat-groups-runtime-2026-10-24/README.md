# C11.4 fixed-repeat 그룹 수 제한 제거 · 구현/runtime 근거

**범위:** 한 grid track list에서 positive integer fixed-repeat 그룹 수를 두 개로 제한하던 adapter 상한을 없앴다. 기존 10,000 expanded descriptor 한도와 제한된 computed breadth profile은 유지한다. 이 opt-in runtime slice는 C11.4 전체 완료가 아니며 C11.4 공식 checkbox와 `72/213` 집계는 그대로다.

## 구현 계약

- 한 축·한 track list에 auto-fill 또는 auto-fit 하나와 임의 개수의 positive integer fixed-repeat 그룹을 authored order대로 허용한다.
- 각 fixed pattern은 computed finite nonnegative `LengthPx` 1–2개다. fixed/ordinary/auto-repeat 확장 결과는 10,000 track 이하이어야 한다.
- group metadata는 track-list component index에 직접 저장·조회한다. `list.values` 길이를 기준으로 fallible reserve하고, allocation failure는 node/property 맥락의 기존 cascade error로 변환한다.
- outer/fixed/auto line names, 두 번째 auto-repeat, percentage/unresolved `Calc`, flex·intrinsic·`minmax()` fixed breadth, template areas 조합, 동시 dual-axis 확장, 일반 CSS admission은 계속 지원하지 않는다.

## 독립 기대값과 Chrome 비교

[Chrome 156 사전 비교](../c11-4-unbounded-fixed-repeat-groups-precomparison-2026-10-24/README.md)의 네 fixed-repeat 열/행 장면과 CSS Grid 2025 CRD §7.2.3.1을 사용했다. fixed/ordinary breadth는 99px, gap은 8px이며 auto-repeat 5개는 독립 계산 23.4px이다. count 5의 필요 공간은 303px, count 6은 331px이므로 320px 축에서는 5개가 맞다. Chrome CSSOM 직렬화는 23.3906px이고 허용 오차는 1/64 CSS px다. 행 장면 `grid-row:5/6` 시작점은 74px다.

## 코드·fixture 검증

- `cargo test --locked -p spinon-style --lib --quiet`: 218 passed.
- `cargo test --locked -p spinon-ffi --lib --features c04-runtime-gpu-fixture-test --quiet`: 118 passed, 2 ignored. 새 12-scene matrix 단위 테스트도 별도 실행해 통과했다.
- `node --test tools/css-reference/fixture-module-boundary.test.mjs`: 11 passed. scene payload와 iOS allowlist는 opt-in fixture-test 경계에 있다.
- `cargo fmt --all -- --check`, 변경 crate별 Clippy(`-D warnings`)를 통과했다.
- 기본 `spinon-ffi` build와 fixture-test positive control의 scene marker·fixture ABI 대조는 [artifact 경계 보고서](./artifact-boundary.md)에 있다.

## Android API 37 ARM64 AVD

`emulator-5558` / `sdk_gphone64_arm64`에서 기존 fixed-repeat 10개와 신규 네-group columns/rows 장면 2개, 총 **12/12**가 통과했다. 모든 장면의 node frame·track trace·draw/present가 검증됐고 PixelCopy 표본은 **15/15 RGBA exact**다. 설치 APK와 로컬 APK SHA-256은 `e3f09e840368aeee42c44fd7eae59b0e0509773c66cc749f9235beed2dd8b4e2`로 일치한다.

최종 결과와 장면별 원본 log·PNG: [Android AVD 실행 디렉터리](./android-avd-c114-unbounded-fixed-repeat-groups-android-avd-20261024T112345503Z-95bd1ce5/). 빌드 log: [`android-build-pass03.log`](./android-build-pass03.log). 이 결과는 Android emulator 실행이며 Android 실기기나 hardware GPU 검증이 아니다.

## iOS Simulator

iPhone 17 Pro / iOS 26.2 Simulator에서 같은 12개 장면이 **12/12** 통과했다. node frame·track trace·draw/present를 확인했고 Metal drawable 표본은 **15/15 RGBA exact**다. 표본은 OS compositor 제출 전이다.

최종 결과와 장면별 원본 log·PNG: [iOS Simulator 실행 디렉터리](./ios-simulator-c114-unbounded-fixed-repeat-groups-ios-simulator-20261024T111939751Z-2c9a24a2/). 최종 Xcode 빌드 원문(공백 포함, gzip 보존): [`ios-build-pass03.log.gz`](./ios-build-pass03.log.gz).

양 플랫폼 capture의 source dirty diff SHA-256은 `e13793c2e08e4c406b10114769f484ee91356c62cdfebda4e651859b7dc2d735`로 같다. Chrome 사전 비교와 runtime 결과는 fixture scene geometry, track size, item frame을 각각 대조한다.

## 남은 범위

세 번째 fixed-repeat을 포함한 이 제한 grammar 외의 모든 breadth·line-name·template-area profile, 일반 CSS admission, WPT, Android 실기기/hardware GPU, iOS compositor 이후 pixels, 전체 C11 및 C11.4는 미완료다. 내부 계약 `0059`는 `0.1.0`을 유지하고 공식 완료 집계 `72/213`을 올리지 않는다.

[구현/runtime 실패 관점 20개](./implementation-runtime-review-20.md) · [PR diff 실패 관점 30개](./pr-diff-review-30.md) · [계획](../../../../plan/c11-4-unbounded-fixed-repeat-groups.md) · [계획 실패 관점 20개](../../../../plan/c11-4-unbounded-fixed-repeat-groups-plan-review-20.md) · [계약 0059](../../0059-c11-grid-layout.md).
