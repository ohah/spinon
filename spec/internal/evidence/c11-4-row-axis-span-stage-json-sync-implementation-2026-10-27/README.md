# C11.4 row-axis stage 기대값 JSON 연결 근거

**상태 · 2026-10-27:** 행 축의 stage matcher가 독립 사전 비교 자료의 [expected.json](../c11-4-row-axis-span-stage-precomparison-2026-10-27/expected.json)을 직접 읽도록 연결했다. Rust stage 상수와 JSON 사이의 drift 경로를 제거했다. 제품 알고리즘, 앱 runtime, FFI는 변경하지 않았다.

## 변경 범위

- [row_axis.rs](../../../../crates/spinon-layout/src/grid_multispan_chrome_oracle_tests/row_axis.rs)는 expected JSON을 로드해 네 scene의 source order, contribution, span batch, stage snapshot을 검사기에 전달한다.
- [row_axis/expected.rs](../../../../crates/spinon-layout/src/grid_multispan_chrome_oracle_tests/row_axis/expected.rs)는 네 scene의 ID/순서, axis, track 수, definite block-size, contributions, batch label과 stage arrays를 검증한다. `"160/3"` 같은 유리수와 `null` 성장 한계를 처리하며 malformed number/label/array 길이를 거부하는 단위 테스트를 포함한다.
- [span_stage.rs](../../../../crates/spinon-layout/src/grid_multispan_chrome_oracle_tests/span_stage.rs)는 공유 stage snapshot matcher를 맡는다. 행 축은 JSON-derived expectation을 사용하고, 기존 열 축 대조군은 기존 고정 기대값을 유지한다.
- Chrome 156 final geometry 기준과 CSS Grid 2025 CRD에서 계산한 stage oracle은 계속 별도로 유지한다. Chrome geometry를 내부 stage 값으로 해석하지 않는다.

## 검증

| 확인 | 결과 | 원본 로그 |
|---|---|---|
| `cargo test --locked -p spinon-layout grid_multispan_chrome_oracle_tests` | 통과, 6 tests | [focused-tests.log](./focused-tests.log) |
| `cargo fmt --all -- --check` | 통과 | [format-check.log](./format-check.log) |
| `cargo clippy --locked -p spinon-layout --all-targets -- -D warnings` | 통과 | [clippy.log](./clippy.log) |
| `git diff --check --cached` | 통과 | [diff-check.log](./diff-check.log) |

검증 기준 commit은 [validation-base.txt](./validation-base.txt)에, 수정 Rust 파일과 기대 JSON의 SHA-256은 [source-sha256.txt](./source-sha256.txt)에 기록했다. 구현 실패 관점 검토 20개는 [implementation-review-20.md](./implementation-review-20.md)에 있다. [PR #685](https://github.com/ohah/spinon/pull/685) 전체 diff 검토는 [30 PASS / 0 PARTIAL](./pr-diff-review-30.md)이다. PR 상태 조회 시 GitHub check rollup은 비어 있었으며 이를 CI 통과로 간주하지 않았다. 병합은 아직 확인되지 않았다.

## 한계

이는 순수 Rust trace 테스트다. Android/iOS 앱이나 simulator는 실행하지 않았다. Chrome은 최종 geometry control이며 내부 stage trace를 제공하지 않는다. 각 batch 종료 지점만 검사하고 batch 안의 transient subpass를 검증하지 않는다. `null`은 trace serializer의 non-finite sentinel이므로 내부 +∞와 NaN/−∞를 구분하지 않는다. 네 고정 scene 외의 일반 CSS Grid 지원을 주장하지 않으며 C11.4 공식 상태 `72/213`은 그대로 미완료다.
