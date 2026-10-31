# C11.2.2 fixture archive boundary

Both artifacts were built with `cargo build --locked -p spinon-ffi --lib` and its `c04-runtime-gpu-fixture-test` variant. Archive fingerprint metadata identified the checked variants:

- Core-only `c04-runtime-gpu`: `target/debug/deps/libspinon_ffi-6a4cb817789368bd.a`. `strings` found neither `_spinon_runtime_gpu_host_new_c11_2_2_grid_auto_placement_fixture` nor `_spinon_runtime_gpu_host_eval_c11_2_2_grid_auto_placement_fixture`.
- Opt-in `c04-runtime-gpu-fixture-test`: `target/debug/deps/libspinon_ffi-2f4562df41e0918f.a`. `strings` found both fixture symbols as the positive control.

This verifies the local FFI static archives for these Cargo feature sets. It is not a complete Android APK/iOS app-bundle exclusion or release-artifact size claim.
