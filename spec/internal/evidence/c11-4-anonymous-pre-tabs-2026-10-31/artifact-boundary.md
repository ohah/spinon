# C11.4 preserved-tab FFI archive 경계

macOS ARM64 Rust 1.96.1에서 `spinon-ffi` static archive의 새 scene ID와 C fixture ABI를 비교했다. 이 검사는 macOS debug archive의 feature boundary만 확인하며 Android/iOS bundle 전체의 byte 크기나 다른 fixture의 격리를 뜻하지 않는다.

## 빌드

```sh
mise exec -- cargo build --locked --no-default-features -p spinon-ffi --lib
mise exec -- cargo build --locked --no-default-features -p spinon-ffi --lib --features c04-runtime-gpu-fixture-test
```

두 명령의 archive는 `target/debug/deps/libspinon_ffi-0098a99528f2b0e6.a` (기본)와 `target/debug/deps/libspinon_ffi-2f4562df41e0918f.a` (opt-in)이다.

| 구성 | bytes | SHA-256 | 새 `anonymous-pre-tab-*` scene ID | 정의된 C11.4 fixture ABI |
| --- | ---: | --- | ---: | ---: |
| 기본, feature 없음 | 516,064,864 | `00cec6fed1f112787523a56181c28734063da991cd040ad61f1bc2ee353a0fd4` | 0/9 | 0 |
| opt-in `c04-runtime-gpu-fixture-test` | 730,111,344 | `4095ed9a54ed831156a8a7e9b429fc354132b9ef065b022fd3e2891f20ba5340` | 9/9 | 2 |

Scene ID는 아홉 개 각각을 archive의 `strings` 출력에서 고정 문자열로 검사했다. 심볼은 Rust 1.96.1 toolchain의 `llvm-nm --defined-only -g`로 확인했다.

```text
_spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture
_spinon_runtime_gpu_host_eval_c11_4_grid_track_sizing_fixture
```

처음 사용한 Xcode `nm`은 Rust 1.96.1 LLVM 22.1.2 bitcode의 attribute kind를 읽지 못했다. 같은 archive를 Rust toolchain의 LLVM 도구로 다시 검사했고, 위 표는 그 결과다. 이 비교는 새 pre-tab scene과 두 fixture symbol의 기본 빌드 부재 및 opt-in 포함을 확인한다.
