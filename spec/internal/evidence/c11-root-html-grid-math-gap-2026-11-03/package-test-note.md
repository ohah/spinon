# 패키지 회귀 실행 기록

## 실행 결과

- 환경: macOS 25.5 arm64, Rust/Cargo 1.96.1, 2026-11-03.
- 범위 명령: `cargo test --locked -p spinon-layout -p spinon-style-to-layout`.
- 결과: `spinon-layout` 377개 중 376개 통과, 1개 실패. 실패 테스트는 `c15_text_layout_tests::host_document::host_document_collects_direct_text_and_inherited_spans_in_source_order`이며, 실패 위치는 `crates/spinon-layout/src/c15_text_layout_tests/host_document.rs:305`이다.
- 같은 테스트를 `--exact --nocapture`로 단독 재현했고, 원본 출력을 [`c15-isolated-rerun.log`](./c15-isolated-rerun.log)에 보존했다. 단독 실행도 같은 assertion에서 실패했다.

## 원인 판독과 범위

실패한 테스트는 `Flex`, `Grid`, `FlowRoot` 표시를 모두 `UnsupportedTextParagraph`로 기대한다. 현재 `crates/spinon-layout/src/host_document/text_paragraphs.rs`의 paragraph 수집 분기는 direct child의 `Block | Grid`를 block flow item으로 허용하므로, 배열 순서상 `Grid` 사례에서 기대값과 구현이 충돌한다. 이 C15 테스트와 구현 파일은 이번 변경에서 수정하지 않았다.

이 실패는 통과로 계산하거나 재실행으로 덮지 않는다. C11 gap 변경은 별도 targeted integration 8개에서 통과했다. 전체 패키지 회귀는 이 기존 C15 기대값 불일치가 정리되기 전까지 완전 통과로 보고하지 않는다.

`cargo test --quiet --locked -p spinon-layout -- --skip c15_text_layout_tests::host_document::host_document_collects_direct_text_and_inherited_spans_in_source_order`는 나머지 376개가 통과했다. 이후 전체 `spinon-style-to-layout` 실행은 323개 중 321개 통과, 2개 실패였다. 원본은 [`style-to-layout-suite.log`](./style-to-layout-suite.log)에 보존했다.

- `c07_3_aspect_ratio_tests::comparison::aspect_ratio_computed_values_and_frames_match_pinned_chromium_at_both_dprs`: 기대 문자열 `1.77778 / 1`, 관측 Stylo serialization `1.7777778 / 1` 간 정밀도 차이. 이번 변경 파일과 무관하다.
- `c11_1_root_html_fr_auto_column_tests::parsed_root_html_css_invalid_fraction_falls_back_to_unsupported_auto_columns`: 단독 재현의 실제 결과는 `CascadeDiagnostic`이며 Stylo가 `grid-auto-columns:-1fr` 선언을 cascade 전에 parse error로 거부한다. 상세는 [`c11-fraction-isolated-rerun.log`](./c11-fraction-isolated-rerun.log)다. 해당 테스트와 cascade 경로는 이번 변경에서 수정하지 않았다.

현재 checkout의 세 실패 파일과 C15 구현 파일은 `origin/main` 대비 변경되지 않았다. 이번 diff의 원인으로 확인되지 않은 기존 테스트·직렬화 계약은 변경하지 않았다.

## 최종 후보 source 재실행

2026-11-03에 최종 후보에서 focused gap 검사를 다시 실행했다. Root HTML math gap integration은 8/8, resolver 소유권·basis 단위 검사는 3/3 통과했다. `spinon-layout`은 위 C15 테스트만 제외해 379개가 통과했고, `spinon-style-to-layout`은 위 두 비관련 실패만 제외해 321개가 통과했다. 원본은 [`layout-suite-final-rerun.log`](./layout-suite-final-rerun.log), [`style-to-layout-suite-final-rerun.log`](./style-to-layout-suite-final-rerun.log)이다. 제외한 기존 실패는 앞서 전체 실행과 단독 재현 로그에 남아 있으며, skip 결과를 전체 패키지 통과로 해석하지 않는다.

최종 후보에서 `cargo clippy --locked -p spinon-layout -p spinon-style-to-layout --all-targets -- -D warnings`와 `cargo fmt --all -- --check`도 통과했다. 이 기록은 변경된 두 패키지의 범위 검사이며 전체 workspace test/Clippy 결과를 뜻하지 않는다.
