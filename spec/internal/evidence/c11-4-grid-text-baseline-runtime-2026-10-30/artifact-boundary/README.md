# C11.4 Grid text baseline archive feature boundary

These checks compare the local `spinon-ffi` debug static archive built with the ordinary runtime feature against the same archive built with the fixture-test feature. They check the new named host symbol and scene strings; they do not establish APK/IPA, release, or whole-product exclusion.

## Product feature build

Command:

```sh
cargo build --locked -p spinon-ffi --lib --features c04-runtime-gpu
```

- Archive: `target/debug/libspinon_ffi.a`
- SHA-256: `0b36403d49330661e40ac0a00f0298e3527ce58796aee980f7b4448470d0d`
- `llvm-nm --defined-only` did not find `_spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture`.
- `strings` did not find any `grid-text-first-baseline-*` or `grid-text-last-baseline-*` scene ID.

## Fixture-test positive control

Command:

```sh
cargo build --locked -p spinon-ffi --lib --features c04-runtime-gpu-fixture-test
```

- Archive: `target/debug/libspinon_ffi.a`
- SHA-256: `a6ca09427341687ef3adb2922cb173489995d154729e2840d7ec745875a44903`
- `llvm-nm --defined-only` found `_spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture`.
- `strings` found all four first-baseline and five last-baseline scene IDs, including source/order controls.

The same LLVM tool was used for both builds: `/opt/homebrew/opt/llvm@22/bin/llvm-nm`. This evidence is limited to the local debug static archives and exact probes above.
