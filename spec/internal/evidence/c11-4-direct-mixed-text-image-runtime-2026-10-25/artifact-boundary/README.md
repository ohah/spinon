# C11.4 direct mixed intrinsic fixture 바이너리 경계

Scanner는 macOS ARM64에서 `spinon-ffi` release 정적 archive를 두 번 빌드한다.

- 기본 제품 대조군: `c04-runtime-gpu`
- opt-in positive control: `c04-runtime-gpu-fixture-test,c11-grid-metal-surface-pixels-test`

네 direct mixed scene ID, 기존 C11.4 host ABI, 신규 후보 Rust `runtime_source` symbol을 archive byte와 정의 symbol에서 검사한다. 두 빌드는 일반 workspace target을 덮어쓰지 않도록 전용 ignored target 디렉터리를 공유한다. 로그와 symbol 목록은 매 실행 `runs/artifact-boundary-*` 고유 디렉터리에 기록하고 manifest가 해당 run에 연결한다. Scanner는 마지막 성공 manifest인 [artifact-boundary.json](artifact-boundary.json)을 완료 시점에 원자적으로 교체한다. 이후 실행이 실패하면 기존 성공 manifest는 그대로 남는다.

Staged diff hash에서는 생성 manifest와 `runs/**`를 제외한다. Cargo 명령의 target 경로는 저장소 상대 경로이며 기록한 로그와 toolchain 명령에서 사용자 홈·checkout 경로를 가린다. Scanner는 현재 Rust toolchain과 일치하는 `llvm-nm`을 우선 사용한다. Xcode 17 LLVM reader는 Rust 1.96이 내보낸 LLVM 22 bitcode object를 읽지 못한다.

저장소 루트에서 실행한다.

```sh
python3 spec/internal/evidence/c11-4-direct-mixed-text-image-runtime-2026-10-25/artifact-boundary/scan-ffi-archives.py
```

이 검사는 macOS ARM64 Rust FFI 정적 archive만 대상으로 한다. 완성된 Android APK/iOS IPA의 fixture 제외, 패키지 크기, hardware GPU 동작 또는 compositor 출력은 보증하지 않는다. `prior-attempts/legacy-root-outputs/`는 run별 산출물 구조를 도입하기 전의 기록이다.
