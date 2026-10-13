# FFI fixture 바이너리 포함 경계

- 산출물: `spinon-ffi` macOS debug static library (`target/debug/libspinon_ffi.a`)
- 기본 조합: `cargo build --locked -p spinon-ffi --no-default-features`
- 검증 조합: `c04-runtime-gpu-fixture-test,c11-2-3-grid-fixture-test,c11-grid-metal-surface-pixels-test`
- Rust 1.96/LLVM 22로 만든 archive는 Xcode의 LLVM 17 `nm`이 일부 object attribute를 읽지 못했다. 같은 Rust 툴체인의 LLVM 22 `llvm-nm`으로 FFI symbol을 다시 검사했다.
- 기본 조합에서는 C11.3.1 fixture FFI 심벌과 runtime matrix schema 문자열이 모두 없다. 25MB 전체 symbol 목록은 저장하지 않고, 관심 심벌 필터의 `matching_c11_3_1_symbols=0` 결과를 남겼다.
- 검증 조합에서는 두 항목이 모두 들어간다.
- 이는 Rust static library feature 경계 검사다. Android/iOS 최종 release 앱 번들 크기나 release artifact 제외를 증명하지 않는다.

명령별 빌드 로그와 판정 대상 symbol/payload는 이 디렉터리의 로그에 보관했다.
