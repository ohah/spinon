# Release FFI archive fixture boundary

Both commands build `spinon-ffi` release static archives from the current source. The second is the opt-in positive control.

| Build | Archive SHA-256 | Marker scan |
| --- | --- | --- |
| `cargo build -p spinon-ffi --release --no-default-features --features c04-runtime-gpu` | recorded in [product-archive.sha256](./product-archive.sha256) | zero scene IDs, fixture symbols, or fixture source paths; `rg` returned 1 (no matches) |
| `cargo build -p spinon-ffi --release --no-default-features --features c04-runtime-gpu-fixture-test` | recorded in [fixture-test-archive.sha256](./fixture-test-archive.sha256) | positive: both scene IDs, fixture source path, report symbol, and runtime adapter symbols are present in [fixture-test-marker-scan.txt](./fixture-test-marker-scan.txt) |

Build output is retained in [product-build.log](./product-build.log) and [fixture-test-build.log](./fixture-test-build.log). The product marker scan is intentionally empty; [product-marker-result.txt](./product-marker-result.txt) records the expected no-match result.

This compares the production GPU runtime archive (fixture-test feature off) with the opt-in fixture archive. The fixture route remains absent when `c04-runtime-gpu` is enabled without `c04-runtime-gpu-fixture-test`. It does not prove fixture exclusion from a built APK or IPA.
