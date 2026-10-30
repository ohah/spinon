# Release FFI archive fixture boundary

The product build enables `c04-runtime-gpu` with the fixture-test feature off. The second build is the opt-in positive control.

| Build | Archive SHA-256 | Definite-axis scene ID / module marker scan |
| --- | --- | --- |
| `cargo build --locked --release -p spinon-ffi --no-default-features --features c04-runtime-gpu` | `df02e0ab0ff705e4599738898ea442205602fcfc0054ab36da43b79fe3eef7bb` | Each of four IDs and `grid_text_aspect_ratio_definite_axis`: 0 occurrences |
| `cargo build --locked --release -p spinon-ffi --no-default-features --features c04-runtime-gpu-fixture-test` | `2c4e93902bf00911c9933bc6848d9741ed0836fc2409a74556c1939fb31eb317` | Each of four IDs: 1 occurrence; module marker: 3 occurrences; Rust module symbol present in `nm -g` output |

Build logs are [product](../product-archive-build.log) and [fixture-test positive control](../fixture-archive-build.log). This verifies the `spinon-ffi` release static archive only; it does not assert final Android APK or iOS app bundle exclusion.
