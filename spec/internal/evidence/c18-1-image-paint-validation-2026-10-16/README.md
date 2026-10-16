# C18.1 최종 로컬 검증 — 2026-10-16

이 폴더는 PR 준비 시점의 Rust·fixture·capture comparator 검증 원본을 보관한다. `cargo` 명령은 저장소 고정 toolchain을 사용하는 `mise exec`로 실행했다.

| 검증 | 결과 | 로그 |
| --- | --- | --- |
| `cargo fmt --all -- --check` | 통과 | [rustfmt.log](./rustfmt.log) |
| `cargo test --locked -p spinon-render -p spinon-render-wgpu` | render 79/79, WGPU unit 22/22, integration 3/3 | [render-tests.log](./render-tests.log) |
| C18.1 opt-in `spinon-ffi` tests | 44 passed, 2 ignored | [ffi-c18-1-tests.log](./ffi-c18-1-tests.log) |
| workspace Clippy `--all-targets -- -D warnings` | 통과 | [workspace-clippy.log](./workspace-clippy.log) |
| C18.1 opt-in FFI Clippy | 통과 | [ffi-c18-1-clippy.log](./ffi-c18-1-clippy.log) |
| fixture-module boundary + comparator Node tests | 16/16 | [node-tests.log](./node-tests.log) |
| Android API 37 AVD PixelCopy/screenshot | 9/9 surface PNG, 9/9 screenshot, 4/4 anchor | [android-pixel-comparison.json](./android-pixel-comparison.json) |
| iPhone 17 Pro iOS 26.2 Simulator Metal/screenshot | 9/9 Metal readback, 9/9 screenshot, 4/4 anchor | [ios-pixel-comparison.json](./ios-pixel-comparison.json) |
| `git diff --check` | 통과 | [diff-check.log](./diff-check.log) |

두 comparator는 각 플랫폼의 원본 process log, 최신 앱 binary hash, 화면 mapping, surface generation을 검증한다. Android는 PixelCopy PNG도 pixel별로 읽는다. iOS Metal readback 표본은 process log에서 비교하며 Simulator full-screen screenshot을 별도로 대조한다. 이 결과는 지정한 AVD·Simulator만 검증한다.
