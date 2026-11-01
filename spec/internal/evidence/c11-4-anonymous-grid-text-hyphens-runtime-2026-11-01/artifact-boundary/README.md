# C11.4 `hyphens` fixture archive 경계

macOS ARM64에서 기본 `spinon-ffi` release static archive와 `c04-runtime-gpu-fixture-test`를 켠 positive-control archive를 각각 빌드해 비교했다.

| 조합 | 빌드 | 21개 C11.4 `hyphens` scene ID | `spinon_runtime_gpu_host_eval_fixture` export | SHA-256 |
|---|---|---:|---:|---|
| 기본 | `cargo build --release -p spinon-ffi --lib` | 0/21 | 0 | `4c4886db2c47d5f0ec27bca011079d80c45535aa7df2e872fedf0a6e93cf31ba` |
| fixture positive control | `cargo build --release -p spinon-ffi --lib --features c04-runtime-gpu-fixture-test` | 21/21 | 1 | `591ad62900baf3cfff5f8417b921b187509696b92b2585003a9a2586732adcc0` |

Scene 문자열은 `strings`로 찾았고 export는 Rust 1.96.1 toolchain `llvm-nm --defined-only --extern-only`로 검사했다. 기본 조합에서는 fixture export와 모든 candidate scene ID가 없었고, positive control에는 둘 다 들어 있었다. Build archive는 `/tmp/spinon-c114-hyphens-artifact-boundary/`에 두었으며 저장소에는 hash·검사 결과만 기록한다.

이 검사는 macOS ARM64 Rust static archive에 한정된다. 최종 APK/IPA 전체에서 fixture ABI·문자열·payload가 제거되는지 확인한 결과가 아니며 앱 bundle 제외를 주장하지 않는다.
