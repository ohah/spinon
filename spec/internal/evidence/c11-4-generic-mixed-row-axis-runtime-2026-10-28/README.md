# C11.4 generic mixed text/image row-axis runtime · 2026-10-28

## 범위와 결과

새 HostNode ID를 사용하는 direct text/image mixed row adapter를 C11.4 opt-in fixture 경로에 연결했다. 새 generic row 네 scene과 고정 ID row·column 및 generic column 대조군을 포함해 Android와 iOS에서 각각 16/16 scene이 통과했다. 모든 scene은 source order, intrinsic input과 revision, Columns→Rows trace, root/item frame, draw/present를 확인했다.

두 플랫폼은 동일한 소스 commit `0c89ffcd5043ce093bac8e4a1dfadeac062ff3c1`과 tree `c529b9006e573f81263bd6565ffe5dfa32952174`에서 빌드했다. 두 manifest 모두 tracked diff가 비어 있고, 설치된 앱 산출물 hash가 이번 빌드 산출물과 일치한다.

이 제한 fixture는 일반 CSS admission, 전체 WPT, 이미지 texel 또는 glyph pixel equality, 실기기, Android hardware GPU, iOS compositor 이후 pixel을 검증하지 않는다. C11.4는 미완료이며 공식 집계 `72/213`을 유지한다.

이 고정 입력에서는 `min-content`와 `max-content` row가 모두 `20px`이다 (`max(20px text, 16px image)`). 따라서 네 신규 scene은 generic route와 row-axis contribution을 확인하지만 두 intrinsic algorithm 사이의 차이를 입증하지 않는다. 독립 기대값과 Chrome 관측은 [precomparison evidence](../c11-4-generic-mixed-row-axis-precomparison-2026-10-28/README.md)에 기록돼 있다.

## Simulator evidence

- Android API 37 ARM64 16KB AVD `emulator-5554` (`spinon_api37_1_compat16k`): [16/16 manifest](runs/c114-generic-row-axis-android-avd-20261027T204356591Z-0d61069b/android-simulator.json), [generic row scene screenshot](runs/c114-generic-row-axis-android-avd-20261027T204356591Z-0d61069b/android-direct-row-min-content-generic-forward.png), [run files](runs/c114-generic-row-axis-android-avd-20261027T204356591Z-0d61069b/).
- iPhone 17 Pro / iOS 26.2 Simulator `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`: [16/16 manifest](runs/c114-generic-row-axis-ios-simulator-20261027T204503975Z-b0f57eeb/ios-simulator.json), [generic row scene screenshot](runs/c114-generic-row-axis-ios-simulator-20261027T204503975Z-b0f57eeb/ios-direct-row-min-content-generic-forward.png), [run files](runs/c114-generic-row-axis-ios-simulator-20261027T204503975Z-b0f57eeb/).

The Android runner invoked ADB only with `-s emulator-5554`; the connected Samsung phone was not selected. The iOS runner targeted only the simulator UDID above. Both runners terminated the Spinon app after capture. They left the already booted simulators running.

The screenshots show the app host surface and layout boxes. They are not per-glyph or per-texel assertions; the row contribution and exact frame evidence is in the scene runtime logs and manifest observations.

## Code validation

- Focused runtime mixed tests: 13/13 passed — [log](validation/runtime-tests.log).
- C11.4 FFI route and trace tests: 84/84 passed — [log](validation/ffi-tests.log).
- `cargo fmt --all -- --check`: passed — [log](validation/fmt.log).
- Feature-enabled workspace Clippy with `-D warnings`: the unmodified `c11_4_grid_layout_auto_tracks/validation.rs` triggers `clippy::type_complexity`, and the pre-existing generic column fixture triggers `clippy::map_identity` — [full output](validation/clippy.log). Re-running with only those two existing lint classes allowed passed; all other warnings remain denied — [log](validation/clippy-known-baseline-allowances.log).
- Default/opt-in `spinon-ffi` static archive boundary: all four generic row scene IDs are absent by default and present with `c04-runtime-gpu-fixture-test` — [result](validation/archive-boundary/result.txt), [verifier](validation/archive-boundary/verify-archive-boundary.sh). This does not prove fixture exclusion from release APK/IPA bundles.

The sequential simulator runner is [run-simulator-matrix.sh](run-simulator-matrix.sh). Its capture driver records the source commit, source hashes, app hash, simulator details, per-scene logs, screenshots and trace checks.

## Adversarial review

- [Implementation/runtime review — 20 distinct failure perspectives](implementation-runtime-review-20.md).
- [PR diff review — 30 distinct failure perspectives](pr-diff-review-30.md).

## Pull request state

Implementation PR [#713](https://github.com/ohah/spinon/pull/713) was merged with Rebase and merge at `2026-10-27T21:00:41Z`; merge SHA: `679b8d51bc3b4edc2cbd1c4f5921e3b73a12dabe`. Its GitHub status-check rollup was empty, so no CI pass is claimed. The main branch protection response reported zero required approving reviews and did not include configured required status checks. See the captured [merge state](pr-713-merge-state.json), [branch protection](main-branch-protection.json), and [`gh pr checks` output](pr-713-checks.txt).

## Completion state

C11.4 remains incomplete. No official checkbox is promoted and `72/213` remains unchanged. The merge state above is limited to PR #713 and does not change the official completion count.
