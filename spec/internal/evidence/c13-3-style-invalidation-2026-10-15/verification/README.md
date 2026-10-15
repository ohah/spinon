# C13.3 style invalidation · 회귀 검사

최종 코드에서 다시 실행한 회귀 명령과 결과다. 일반 `.log` 사본은 마지막 공백만 정리했고, 원본 bytes는 같은 이름에 `.raw.gz`를 붙여 보존한다.

| 검사 | 결과 | 원본 |
|---|---:|---|
| `cargo test --locked --workspace --quiet` | 866 통과, 4 ignored, 0 실패 | [로그](workspace-tests.log) |
| `cargo test -p spinon-ffi --lib --features c13-3-gesture-fixture-test` | 45 통과, 2 ignored, 0 실패 | [로그](ffi-feature-tests.log) |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | 통과 | [로그](clippy.log) |
| `cargo fmt --all -- --check` | 통과 | [로그](rustfmt.log) |
| CSS precomparison·Simulator capture·fixture module boundary Bun suite | 14 통과, 0 실패 | [로그](css-reference-tests.log) |
| `git diff --check` | 통과 | [로그](diff-check.log) |

Simulator scenario 6/6과 screenshot pixel assertion은 [최신 Android/iOS 실행 기록](../20261015T111411692Z-d846f2a9/README.md)에 있다. 기본·opt-in app artifact 비교는 [경계 검사](../artifact-boundary/README.md)에서 별도로 기록한다.
