# C11.4 FFI static archive boundary

Rust 1.96.1 / macOS ARM64 `spinon-ffi` dev static archive에서 orthogonal scene marker와 fixture C ABI를 확인했다. Scan에는 rustc toolchain의 `llvm-nm`을 사용했다.

| Build | Markers | C11.4 fixture symbols | Archive SHA-256 |
| --- | ---: | ---: | --- |
| Default `cargo build --locked -p spinon-ffi --lib` | 0/4 | 0/2 | `7b7eb94350298d35d811861e9c94835063c3d6aabbb6010ebfec483c308afb04` |
| Positive control `--features c04-runtime-gpu-fixture-test` | 4/4 | 2/2 | `12007e3dbb0e9f840671b2e99e4bd37328bdb47050e5071212cb92aa81dd2565` |

The four fixed markers are the min/max-content × forward/reversed scene IDs. The two positive-control symbols are `spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture` and `spinon_runtime_gpu_host_eval_c11_4_grid_track_sizing_fixture`.

Build logs, symbol scans, marker scans and archive hashes are stored beside this file. This proves the Rust FFI static archive feature boundary only; it does not prove full Android APK or iOS app product exclusion.
