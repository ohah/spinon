# C11.4 mixed image intrinsic track runtime evidence

이 evidence는 `image-auto-max-mixed-tracks` direct-PNG fixture의 runtime/build/시뮬레이터 기록이다. CSS 기대값은 [독립 규범 기대 JSON](../c11-4-image-auto-max-mixed-track-precomparison-2026-10-26/spec-derived-expectations.json)과 [child plan](../../../../plan/c11-4-image-auto-max-mixed-track.md)에 정의한다. Simulator 결과로 규범 기대값을 역산하지 않는다.

## Stage contract

CSS Grid §11.4의 `Initialize`에서 intrinsic max track growth limit은 규범상 `+∞`로 초기화된다. Trace serializer는 모든 non-finite 값을 `None`/JSON `null`로 projection하므로 그 `null`은 non-finite projection만 확인하며 내부 값이 `+∞`라는 증거가 아니다. 이 case의 span-1 max-content maximum은 §11.5에서 image contribution으로 resolve된다. Taffy는 growth-limit batch update 뒤에 `IntrinsicItemBatch` stage를 기록하므로 여기부터 expected growth limit은 column `Some(24px)`, row `Some(16px)`이다.

Taffy `GridTrackVec` raw vector는 CSS track과 grid line slot을 교차 보관해 `N` CSS track에 `2N+1` entries가 있다. Expected raw values는 다음과 같다. 각 pair는 `baseSize/growthLimit`; 모든 slot의 `infinitelyGrowable=false`다.

| Axis/stage | Raw slot mapping | Expected `baseSize/growthLimit` |
| --- | --- | --- |
| Columns / Initialize | `[leading sentinel, col0, gutter, col1, trailing sentinel]` | `[0/0, 0/null, 5/5, 17/17, 0/0]` |
| Columns / IntrinsicItemBatch and later stages | same 5 slots | `[0/0, 24/24, 5/5, 17/17, 0/0]` |
| Rows / Initialize | `[leading sentinel, row0, trailing sentinel]` | `[0/0, 0/null, 0/0]` |
| Rows / IntrinsicItemBatch and later stages | same 3 slots | `[0/0, 16/16, 0/0]` |

Leading/trailing sentinels are `gutter=true, collapsed=true`, with zero base and growth. The actual inter-column gutter is internal index 2, `gutter=true, collapsed=false`, with `5/5`. CSS track slots are not collapsed. All finite matcher comparisons use `0.0001px` tolerance. This slice does not assert overflow behavior, image paint/readback, or public CSS admission.

## Preserved pre-correction candidate run (failed)

Run ID `c114-image-auto-max-20261026T000220Z-040a5360` is retained without rewriting its raw files. Android API 37 ARM64 AVD and iPhone 17 Pro/iOS 26.2 Simulator both reported a ready layout and root/image frames, and the runtime intrinsic input reported a 24×16 PNG plus the full `AutomaticMinimum` source flags. The old native matcher failed because it expected the former reduced trace arrays and `null` growth through the `IntrinsicItemBatch` stage. Both runs therefore remain failed candidate checks; they do not pass the updated contract.

Raw files:

- Android runtime: [`android-avd/runtime.log`](./runs/c114-image-auto-max-20261026T000220Z-040a5360/android-avd/runtime.log)
- iOS runtime: [`ios-simulator/runtime.log`](./runs/c114-image-auto-max-20261026T000220Z-040a5360/ios-simulator/runtime.log)
- Run source and device metadata are in the same run directory.

The Android log has `SPINON_C0410_DRAW status=0 presented boxes=2`, which proves a surface submission was presented; it also reports `runtime_image_count=0` and `image_draw_count=0`. It does not prove image paint. No screenshots were captured for this failed run. The separate corrected run below is the simulator gate result; do not treat these failed logs as that run.

## Corrected candidate rerun

Updated stage contract review passed before this run. Run ID: `c114-image-auto-max-20261026T002422Z-040a5360-stagefix01`; source worktree HEAD: `040a5360be92f73e141797aa7dcd4ef00ab24942` plus uncommitted candidate changes. The original failed run above remains unchanged.

Build and focused checks:

| Check | Command/result | Evidence |
| --- | --- | --- |
| Formatting | `mise exec -- cargo fmt --all -- --check` — pass | terminal result: exit 0 |
| Runtime focused tests | `mise exec -- cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test c11_4_image -- --nocapture` — `4 passed` | terminal result: exit 0 |
| FFI focused test | `mise exec -- cargo test --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test c11_4::tests::exposes_only_bounded_intrinsic_track_sizing_scenes -- --nocapture` — `1 passed` | terminal result: exit 0 |
| Android feature build | `SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 mise exec -- bun run build:android` — `BUILD SUCCESSFUL` | [build log](./android-build-stage-corrected.log); APK SHA-256 `35b114666be4a347225f895cc8e06f579522a345d0572e77e95cce3861df6d54` |
| iOS feature build | `SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1 mise exec -- bun run build:ios-sim` — `BUILD SUCCEEDED` | [build log](./ios-build-stage-corrected.log); app executable SHA-256 `39c49b4aa5c59bea50a367ccbd8b2b6f44ee8c56ee8c68f6d5047adba2eab0aa` |

Simulator result matrix:

| Check | Android API 37 ARM64 AVD | iPhone 17 Pro / iOS 26.2 Simulator |
| --- | --- | --- |
| Runtime matcher | `status=0`, `layout=ready`, `boxes=2`; `SPINON_C114_GRID_SUMMARY` emitted | `status=0`, `layout=ready`, `boxes=2`; `SPINON_C114_GRID_SUMMARY` emitted |
| Root/item frames | root `100×40`, image `(0,0,24,16)` | root `100×40`, image `(0,0,24,16)` |
| Intrinsic input | PNG natural `24×16`, intrinsic revision `1`; `AutomaticMinimum` all flags and three `None` fields match | same |
| Trace | 5 column slots / 3 row slots; sentinel, gutter, collapsed and infinitely-growable flags match all 6 stages | same |
| Draw/present | `status=0 presented boxes=2`, surface `840×630` | `status=0 presented boxes=2`, surface `960×720` |
| Image-paint limitation | `runtime_image_count=0`, `image_draw_count=0` | `runtime_image_count=0`, `image_draw_count=0` |
| Screenshot | 1080×2400, SHA-256 `c5975c085ed04f203b50380fb83783fd7f001f36be293d8667d7d891a0f141ea` | 1206×2622, SHA-256 `5e8cf86c936c73c19988e60ed5be20052e841824564401a60c7f12d2be64ea8f` |

Raw install/launch/runtime/trace/draw/terminate logs and screenshots are in [the run directory](./runs/c114-image-auto-max-20261026T002422Z-040a5360-stagefix01/). Standalone representative images are [Android](./c11-4-image-auto-max-mixed-track-android-api37-arm64.png) and [iOS](./c11-4-image-auto-max-mixed-track-ios-26.2-simulator.png). The runtime trace confirms `Initialize` has a null projection only for the active intrinsic track slot; `IntrinsicItemBatch` and later stages have finite 24px/16px growth values. Boundary sentinels remain `0/0`, the real column gutter remains `5/5`, and all `infinitelyGrowable` flags are false.

Build, runtime-log, screenshot, and app-binary SHA-256 values are in [the hash manifest](./stage-corrected-sha256sums.txt); changed source/fixture hashes and the local tracked-diff digest are in [the source manifest](./runs/c114-image-auto-max-20261026T002422Z-040a5360-stagefix01/source-hashes.json). The independent trace correction is recorded in [the derivation review](../../../../plan/c11-4-image-auto-max-mixed-track-trace-stage-derivation-review-2026-10-26.md).

Both app processes were force-stopped/terminated after capture and verified absent. The AVD and Simulator remain booted. Screenshots show the app's submitted grid scene; they do not prove PNG raster paint or pixels because both draw markers report zero runtime image draws. No pixel readback, hardware device, public CSS admission, overflow behavior, PR image-link check, PR, commit, or merge is claimed. No Agent Browser was used in this simulator rerun.
