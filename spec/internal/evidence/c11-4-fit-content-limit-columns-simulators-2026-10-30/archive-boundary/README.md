# FFI static archive boundary

Two release archives were built from the same source revision with separate `CARGO_TARGET_DIR` values:

```sh
CARGO_TARGET_DIR=build/spinon/c11-4-fit-content-column-axis/archive-boundary/product \
  cargo build --locked -p spinon-ffi --lib --features c04-runtime-gpu --release
CARGO_TARGET_DIR=build/spinon/c11-4-fit-content-column-axis/archive-boundary/fixture-test \
  cargo build --locked -p spinon-ffi --lib --features c04-runtime-gpu-fixture-test --release
```

`strings` and Homebrew LLVM 22.1.8 `llvm-nm` scanned for `column-fit-content-`, `fit_content_limit_columns`, and `fitContentLimitColumns`.

| Archive | SHA-256 | Matching strings | Matching defined symbols |
| --- | --- | ---: | ---: |
| Product `c04-runtime-gpu` | `01edc486beafadfcac2108b0a08e72093e01ce0687ec2ea76dcad0237b28ef94` | 0 | 0 |
| Fixture test `c04-runtime-gpu-fixture-test` | `64800da606d509c2bdeeecc19a6b13205a615c692d4ac64bb29c2bc5ce95f609` | 25 | 25 |

The product archive excludes this slice’s marker and route symbols; the fixture archive is the positive control. The [product build log](./product-build.log) and [fixture build log](./fixture-test-build.log), plus the [string scans](./product-marker-scan.txt) and [symbol scans](./product-symbol-scan.txt), preserve the observations. Xcode’s Apple LLVM 17 `llvm-nm` could not parse LLVM 22 Rust bitcode (`Unknown attribute kind`); the scan was repeated successfully with LLVM 22.1.8. This checks static FFI archives only; it does not prove complete APK/IPA bundle exclusion.
