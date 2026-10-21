# 검증 명령 및 결과 요약

| 검사 | 결과 | 원본 근거 |
| --- | --- | --- |
| `mise exec -- cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test c114_nested_grid_text_multispan -- --nocapture` | 2 passed | `runtime-multispan-focused.log` |
| `mise exec -- cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test c114_nested_grid -- --nocapture` | 7 passed | `runtime-nested-grid-controls.log` |
| `mise exec -- cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test c114_mixed_text_image_inputs_share_one_revision_and_span_sizing_pass -- --nocapture` | 1 passed | `runtime-mixed-span-control.log` |
| 신규 FFI allowlist + 기존 direct-leaf text multi-span FFI control (`c04-runtime-gpu-fixture-test`) | 각 1 passed | `ffi-multispan-focused.log`, `ffi-direct-text-control.log` |
| `mise exec -- cargo clippy --locked --workspace --all-targets -- -D warnings` | exit 0 | `workspace-clippy.log` |
| runtime/FFI fixture-feature all-target Clippy (`-D warnings`) | 각각 exit 0 | `fixture-runtime-clippy.log`, `fixture-ffi-clippy.log` |
| `mise exec -- cargo fmt --all -- --check` | exit 0 | `format-check.log` |
| `mise exec -- bun test tools/css-reference/fixture-module-boundary.test.mjs` | 6 passed | `fixture-module-boundary.log` |
| `node --check tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs` | exit 0 | `capture-js-check.log` |
| `git diff --check` | exit 0 | `diff-check.log` |
| Android `build:android`, fixture env flags enabled | `BUILD SUCCESSFUL` | `android-simulator-build.log` |
| iOS `build:ios-sim`, fixture env flags enabled | `BUILD SUCCEEDED` | `ios-simulator-build.log` |
| 신규 four-scene Android/iOS capture | 각 4/4, trace/draw checks 통과 | linked `result.json` in [README](./README.md) |
| 기존 direct-leaf multi-span Android/iOS controls | 각 4/4 통과 | linked control `result.json` in [README](./README.md) |
| 기본/fixture-test FFI archive strings boundary | default 신규 ID 0개, positive control 4개 | [artifact-boundary.md](./artifact-boundary.md) |

Workspace Clippy는 신규 fixture feature를 끈 기본 workspace matrix이고, feature-specific Clippy는 신규 fixture/test cfg를 켠 matrix다. 두 결과를 대체 관계로 해석하지 않는다.
