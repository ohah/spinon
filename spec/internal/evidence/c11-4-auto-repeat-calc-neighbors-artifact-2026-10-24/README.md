# C11.4 auto-repeat `calc()` neighbor FFI artifact 경계 · 2026-10-24

## 실행

현재 작업 트리에서 product 조합과 fixture-test positive control을 각각 release static archive로 빌드했다. 두 명령은 `--locked --no-default-features`를 사용했다.

```sh
mise exec -- cargo build --locked --release --no-default-features -p spinon-ffi --lib --features c04-runtime-gpu
mise exec -- cargo build --locked --release --no-default-features -p spinon-ffi --lib --features c04-runtime-gpu-fixture-test
```

각 실행 로그는 [product-build.log](./product-build.log), [fixture-build.log](./fixture-build.log)에 있다. Source revision과 tracked/untracked Rust inputs는 [artifact-source-manifest.json](./artifact-source-manifest.json)과 [source-files.sha256](./source-files.sha256)에 고정했다.

## 결과

[LLVM 22 raw archive scan](./scan-results-llvm22.json)은 14개 scene ID 전체를 product archive의 원시 바이트에서 찾지 못했고, fixture-test positive control에서는 14/14를 찾았다. `spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture`도 product archive에는 없고 fixture-test archive에는 있다. Candidate Rust `runtime_source` symbol은 fixture-test archive에서만 확인했다.

| 산출물 | 크기 | SHA-256 | scene ID | C11.4 fixture FFI symbol |
|---|---:|---|---:|---|
| product `c04-runtime-gpu` | 75,146,600 bytes | `ff54402555349766c86121cdffe7e5e9774cecc104c80043cf58d3b49eeda562` | 0/14 | absent |
| fixture-test positive control | 78,281,488 bytes | `29a0afca924fbedd1998e2e42d9c7f226559a9c50f52f8e5752987917a501165` | 14/14 | present |

[LLVM 22 symbol excerpt](./symbols-fixtureTest-llvm22.txt) contains both the generic C11.4 fixture C ABI entry and candidate `runtime_source`. The new slice adds no C ABI symbol of its own.

## 도구 차이와 한계

Xcode Apple LLVM 17 `llvm-nm` could not parse LLVM 22 bitcode members (`Unknown attribute kind`) and exited 1; its partial output is retained as [diagnostic scan](./scan-results-xcode-llvm17.json). The final symbol comparison used installed Homebrew LLVM 22.1.8, matching the Rust compiler's LLVM 22.1.2, and exited 0 for both archives. Raw-string checks operate on the complete static archives.

이 증거는 macOS ARM64 Rust FFI static archive의 feature boundary에 한정된다. Android/iOS final APK/IPA에서 fixture payload가 제외되는지는 확인하지 않았고, release distribution artifact 전체의 크기·contents를 보증하지 않는다. Archive binary 복사본은 hashes와 재현 명령을 남긴 뒤 제거했으며 Cargo build cache는 유지했다.
