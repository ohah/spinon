# C11.4 break-spaces static archive 경계

macOS ARM64 `spinon-ffi` release static archive의 product/fixture-test 쌍만 검사했다.

- 기본 `c04-runtime-gpu` product archive SHA-256: `66a5086113fc98c04bdc27e1a42f862e2b63d6920ec781c3e4797a563518988b`.
- `c04-runtime-gpu-fixture-test` archive SHA-256: `03a49d46d2fef41ab558681284e6095f526ab785f3bb88bffac1bf11ebc8eb3e`.
- Rust toolchain `llvm-nm --defined-only`에서 기본 archive는 `_spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture`와 `_spinon_runtime_gpu_host_eval_c11_4_grid_track_sizing_fixture`를 내보내지 않았다. Fixture-test positive control에는 두 symbol이 모두 있었다.
- `strings -a`에서 기본 archive에는 `anonymous-break-spaces-max-content` marker가 없고, fixture-test archive에는 새 C11.4 scene marker와 전체 26-scene 목록이 있었다.

이 검사는 두 macOS ARM64 static archive 안의 symbol/string 경계만 말해 준다. Android APK 또는 iOS app bundle의 전체 fixture 코드 제외를 증명하지 않으며 simulator 결과와 별개다.
