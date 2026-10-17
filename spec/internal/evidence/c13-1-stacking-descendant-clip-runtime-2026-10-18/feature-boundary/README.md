# C13.1 fixture archive 경계

기본과 opt-in artifact를 따로 빌드해 C13.1 복합 stacking fixture의 export·payload 경계를 확인했다. 해시는 `target/debug/libspinon_ffi.a`의 SHA-256이다.

| 빌드 | SHA-256 | 고정 FFI symbol | fixture 문자열 |
| --- | --- | ---: | ---: |
| `mise exec -- cargo build -p spinon-ffi --no-default-features --lib` | `e60d0880e6723ce78c547dcb79ab3e2b849f592a6c9e4b6ee9e0ec3dffa4af5f` | 0 | 0 bytes |
| `mise exec -- cargo build -p spinon-ffi --no-default-features --features c13-1-overflow-clip-fixture-test --lib` | `be70c5aaa278cdb7cbb91b5931805c8164215c8315385cce7420d9d4a77cfa0e` | 2 | 1005 bytes |

기본 artifact에서 `_spinon_runtime_gpu_host_new_c13_1_stacking_descendant_clip_fixture` 및 `_spinon_runtime_gpu_host_eval_c13_1_stacking_descendant_clip_fixture`가 보이지 않았다. opt-in artifact에는 두 symbol이 모두 보였다. `strings`의 default 결과는 0 bytes, opt-in 결과는 fixture markup과 고정 Chrome payload를 포함한 1005 bytes다. 원본 symbol·string 목록은 [default](default-symbol-strings.txt) · [opt-in symbols](opt-in-symbols.txt) · [default strings](default-strings.txt) · [opt-in strings](opt-in-strings.txt).

`xcrun llvm-nm`은 Rust 1.96/LLVM 22 object의 일부 `compiler_builtins` attribute를 Apple LLVM reader가 읽지 못한다는 진단을 남겼다. 그 때문에 해당 tool 출력만으로 archive의 모든 symbol을 판정하지 않았다. 진단 원문은 [default](default-llvm-nm-errors.log) · [opt-in](opt-in-llvm-nm-errors.log)에 보존했다. 고정 두 fixture symbol의 음성·양성 판정은 `llvm-nm` 목록과 별도의 `strings` 확인을 함께 사용했다. 이 결과는 정적 FFI archive에 한정하며 최종 Android APK/iOS app bundle 크기·제품 release artifact 검증이 아니다.
