# 검증 로그

각 `.log`는 저장소 루트에서 실행한 명령의 읽기용 stdout/stderr 사본이다. 아래 여섯 로그는 Git whitespace 검사를 위해 줄 끝 공백과 초과 마지막 빈 줄만 정규화했다. 원본 byte는 각 `.log.raw.gz` archive에 보존하며, 압축을 풀면 원본 로그와 byte 단위로 일치한다. 성공 기준은 종료 코드 0이며, Cargo 테스트는 아래 테스트 수를 기록했다.

| 파일 | 검증 | 결과 |
|---|---|---|
| `layout-feature-tests.log` | `spinon-layout`, `c11-4-grid-track-sizing-fixture-test` | 4 passed |
| `runtime-feature-tests.log` | `spinon-runtime`, C11.4 fixture feature enabled | 8 passed |
| `runtime-feature-off-tests.log` | `spinon-runtime`, fixture feature disabled | 2 passed |
| `ffi-feature-tests.log` | `spinon-ffi`, generic GPU fixture feature enabled | 1 passed |
| `fixture-module-boundary.log` | CSS reference fixture/module boundary | 13 passed |
| `evidence-html-links.log` | Runtime evidence page relative links | 14 links resolved |
| `cargo-clippy.log` | `cargo clippy --locked --workspace --all-targets -- -D warnings` | passed |
| `cargo-fmt.log` | `cargo fmt --all -- --check` | passed |
| `git-diff-check.log` | `git diff --check` | passed |
| `default-artifact-build.log` / `fixture-artifact-build.log` | product archive negative control / opt-in fixture positive control | passed |
| `artifact-boundary.json` | default archive omits row symbol/strings; fixture archive includes them | passed |
| `android-build.log` / `ios-build.log` | app fixture build flags and platform build | passed |
| `ios-installed-app.json` | installed iOS Simulator bundle vs built executable and Info.plist | hashes match |

### 원본 로그 byte 보존

- [원본 runtime-definite-40px-auto-fill-control-rows.log.raw.gz](../android-avd-c114-cyclic-percent-tracks-android-avd-20261025T142617417Z-fbb1b04c/runtime-definite-40px-auto-fill-control-rows.log.raw.gz)
- [원본 ffi-feature-tests.log.raw.gz](./ffi-feature-tests.log.raw.gz)
- [원본 ios-build.log.raw.gz](./ios-build.log.raw.gz)
- [원본 layout-feature-tests.log.raw.gz](./layout-feature-tests.log.raw.gz)
- [원본 runtime-feature-off-tests.log.raw.gz](./runtime-feature-off-tests.log.raw.gz)
- [원본 runtime-feature-tests.log.raw.gz](./runtime-feature-tests.log.raw.gz)

Simulator install, launch, runtime trace, screenshot and source metadata are in the sibling Android/iOS capture directories linked from the parent evidence README.
