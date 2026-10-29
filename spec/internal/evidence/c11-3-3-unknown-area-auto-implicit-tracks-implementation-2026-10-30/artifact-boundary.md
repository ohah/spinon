# C11.3.3 기본/fixture Release FFI archive 경계

실행 환경은 `aarch64-apple-darwin`이며 확인 대상은 `target/release/libspinon_ffi.a`다. 두 빌드를 순서대로 수행하고 각 산출물을 다음 빌드로 덮기 전에 심볼과 문자열을 검사했다.

## 기본/product archive

```sh
cargo build --locked --release -p spinon-ffi --lib
nm -g target/release/libspinon_ffi.a | rg 'spinon_runtime_gpu_host_(new|eval)_c11_3_3_grid_template_areas_fixture'
strings target/release/libspinon_ffi.a | rg 'unknown-area-implicit-tracks|grid-template-areas-c1133-runtime-matrix-v2'
```

Release build는 성공했다. 두 검색 모두 결과가 없어 C11.3.3의 new/eval FFI symbol이나 target scene/matrix payload가 기본 archive에 포함되지 않았다.

## opt-in fixture-test archive

```sh
cargo build --locked --release -p spinon-ffi --lib --features c11-grid-metal-surface-pixels-test
nm -g target/release/libspinon_ffi.a | rg 'spinon_runtime_gpu_host_(new|eval)_c11_3_3_grid_template_areas_fixture'
strings target/release/libspinon_ffi.a | rg 'unknown-area-implicit-tracks|grid-template-areas-c1133-runtime-matrix-v2'
```

Release build는 성공했다. `nm`에서 아래 두 exported symbol이 확인됐다.

```text
_spinon_runtime_gpu_host_eval_c11_3_3_grid_template_areas_fixture
_spinon_runtime_gpu_host_new_c11_3_3_grid_template_areas_fixture
```

`strings`에서는 `unknown-area-implicit-tracks`, v2 matrix의 전체 scene ID 목록과 해당 scene의 고정 DOM source를 확인했다.

이 검사는 macOS arm64 FFI static archive의 feature 경계만 증명한다. Android/iOS 최종 앱 bundle에서 모든 fixture payload가 빠지는지까지 증명하지 않는다. 해당 플랫폼 앱의 실행 범위는 [Simulator runtime 결과](./README.md)와 각 플랫폼별 build log에 한정된다.
