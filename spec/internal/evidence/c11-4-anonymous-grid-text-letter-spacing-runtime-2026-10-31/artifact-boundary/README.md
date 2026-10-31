# C11.4 `letter-spacing` fixture archive boundary

The default and opt-in release builds were compared as macOS arm64 `spinon-ffi` static archives. The scene adapter and validator are inside the `runtime_gpu::fixtures` module, which `crates/spinon-ffi/src/runtime_gpu.rs` includes only with `c04-runtime-gpu-fixture-test`.

| Build | Archive size | SHA-256 | New fixture symbols | 16 scene payload IDs |
| --- | ---: | --- | ---: | ---: |
| Default release | 60,305,776 bytes | `b6ed5b7baeb82c57c9e43877517ab71430cac8a00ac4881a2344e60a673539c3` | 0 | 0/16 |
| `c04-runtime-gpu-fixture-test` release | 81,269,096 bytes | `33742f09c26f30f8b93b1add351720b9c11d312ad912dbf1aaf34e443a930410` | adapter and validator present | 16/16 |

The opt-in archive also excludes the negative-control ID `anonymous-letter-spacing-unlisted`. The positive-control symbol matches are in [`fixture-symbol-matches.txt`](fixture-symbol-matches.txt); [`default-symbol-matches.txt`](default-symbol-matches.txt) records that neither matching adapter nor validator symbol appears in the default archive. The archive scan summary is in [`archive-scan.txt`](archive-scan.txt).

Both builds succeeded. Reproduction commands:

```sh
cargo build --locked --release -p spinon-ffi --lib
cargo build --locked --release -p spinon-ffi --features c04-runtime-gpu-fixture-test --lib
```

The symbol scan used Rust 1.96.1's bundled `llvm-nm` 22.1.2, matching the LLVM version that produced the archive. Build logs are [`default-release-build.log`](default-release-build.log) and [`fixture-release-build.log`](fixture-release-build.log).

This proves the new fixture adapter/validator symbols and scene payloads are absent from the default `spinon-ffi` static archive and present in the opt-in positive control. It does not measure or prove exclusion from complete Android/iOS release bundles; the mobile runs above used the explicit fixture-test build flag.
