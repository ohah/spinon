# Rust 포맷과 lint 최종 기록

- `mise exec -- cargo fmt --all -- --check` — 통과, formatter 차이 없음.
- `CARGO_TARGET_DIR=/Volumes/ZTSBuild/spinon-c15-3-20261017/target CARGO_BUILD_JOBS=1 CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 mise exec -- cargo clippy --locked -p spinon-ffi --all-targets --features c15-3-wgpu-glyph-paint-fixture-test -- -D warnings` — 통과. [로그](feature-clippy-final.log)
- `CARGO_TARGET_DIR=/Volumes/ZTSBuild/spinon-c15-3-20261017/target CARGO_BUILD_JOBS=1 CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 mise exec -- cargo clippy --locked --workspace --all-targets -- -D warnings` — 통과. [로그](workspace-clippy-final.log)

두 Clippy 명령은 2026-10-17 최종 bitmap-buffer bounds-check 보정 뒤 실행했다. formatter는 같은 소스 보정 직후 실행했다.
