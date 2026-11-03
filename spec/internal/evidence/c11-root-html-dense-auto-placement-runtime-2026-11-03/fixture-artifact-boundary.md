# C11.1 Root HTML dense fixture artifact 경계

Rust static archive의 default build와 dense fixture opt-in positive control을 비교했다. `llvm-nm`은 Rust 1.96 toolchain의 LLVM 22.1.2 버전을 사용했다. 재현 명령은 실행 환경에서 검증한 도구의 절대 경로를 사용한다.

```sh
LLVM_NM=/Users/yoonhb/.rustup/toolchains/1.96.1-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/bin/llvm-nm
```

## Default archive

```sh
cargo build --locked -p spinon-ffi --lib
"$LLVM_NM" -g --defined-only target/debug/libspinon_ffi.a | rg 'spinon_runtime_gpu_host_new_c11_1_root_html_dense_placement_fixture'
strings target/debug/libspinon_ffi.a | rg 'spinon-css-c11-root-html-dense-auto-placement-runtime-matrix/v1|row-dense-positive-implicit-row'
```

Build가 성공했다. 두 검색은 모두 일치 항목 없이 종료 코드 1을 반환했다. Default archive에는 신규 fixture symbol과 matrix payload가 없다.

## Opt-in positive control

```sh
cargo build --locked -p spinon-ffi --lib --features c11-1-root-html-dense-placement-fixture-test
"$LLVM_NM" -g --defined-only target/debug/libspinon_ffi.a | rg 'spinon_runtime_gpu_host_new_c11_1_root_html_dense_placement_fixture'
strings target/debug/libspinon_ffi.a | rg 'spinon-css-c11-root-html-dense-auto-placement-runtime-matrix/v1|row-dense-positive-implicit-row'
```

Build가 성공했다. `llvm-nm`에서 `_spinon_runtime_gpu_host_new_c11_1_root_html_dense_placement_fixture`가 정의된 `T` symbol로 확인됐다. `strings`에서 matrix schema와 고정 scene source가 확인됐다.

이는 host `spinon-ffi` static archive의 feature 경계만 확인한다. Android APK/iOS app 최종 크기, release link dead-stripping 또는 배포 artifact 전체의 payload 유무는 주장하지 않는다.
