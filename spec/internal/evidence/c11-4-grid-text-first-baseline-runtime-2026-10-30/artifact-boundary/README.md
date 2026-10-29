# C11.4 first-baseline archive feature boundary

These checks compare the local `spinon-ffi` debug static archive built with the ordinary product feature against the same archive built with the fixture-test feature. They establish whether the named runtime fixture is registered in this archive; they do not establish APK/IPA, release, or whole-product exclusion.

## Product feature build

Command:

```sh
cargo build --locked -p spinon-ffi --lib --features c04-runtime-gpu
```

- Archive: `target/debug/libspinon_ffi.a`
- SHA-256: `3a41d286f3994030446ad5368531513174e600168f6606f2d211fb0bb2e903d0`
- `llvm-nm --defined-only` did not find `_spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture`.
- `strings` did not find any of the four scene IDs (`grid-text-first-baseline-align-items`, `grid-text-first-baseline-start-control`, `grid-text-first-baseline-align-self`, `grid-text-first-baseline-reversed`).
- Debug strings do contain the projection option name `allow_c11_4_grid_text_first_baseline_fixture`. That generic option name is not one of the registered scene IDs or the fixture host symbol.

## Fixture-test positive control

Command:

```sh
cargo build --locked -p spinon-ffi --lib --features c04-runtime-gpu-fixture-test
```

- Archive: `target/debug/libspinon_ffi.a`
- SHA-256: `2bd87666f4bddbc517d45567f5200071cbfd25a6bcd85b306388185cc2d51f46`
- `llvm-nm --defined-only` found `_spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture`.
- `strings` found all four scene IDs.

The archive checks used LLVM `llvm-nm` 22.1.8 at `/opt/homebrew/opt/llvm@22/bin/llvm-nm`; Apple's older `/usr/bin/nm` could not parse the LLVM 22 objects. The positive control confirms the checks can see the gated fixture when it is enabled. The ordinary product result is limited to this debug archive and these exact symbol/string probes.
