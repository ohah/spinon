# C11.4 `fit-content()` 옆 `auto` 행 구현 관측

## 대상과 범위

- clean worktree의 `origin/main` 기준 commit: `4f6b38a4e6528667a0079df88a69bbfc909b86e8` (계획 PR merge commit이며 구현 소스 자체를 포함하지 않음)
- 격리 branch: `feat/c11-4-fit-content-auto-neighbor-oracle-20261026`
- test file: `crates/spinon-layout/src/grid_fit_content_auto_neighbor_oracle_tests.rs`
- module registration: `crates/spinon-layout/src/lib.rs`
- focused test 실행 시점의 두 Rust source SHA-256과 `lib.rs` 등록 diff SHA-256: [`implementation-source.json`](./implementation-source.json)
- compiler/toolchain: `rustc 1.96.1 (31fca3adb 2026-06-26)`, `cargo 1.96.1 (356927216 2026-06-26)`

변경은 pure Rust oracle test와 그 `#[cfg(test)]` 등록으로 제한했다. test 입력은 explicit `rows=[]`, `auto_rows=[fit-content(40px), auto]`, 60×300px definite grid, zero gap, `align-content:normal`, 두 auto-height direct wrapper와 각 wrapper의 10px nested block이다. 기존 adapter나 제품 알고리즘은 수정하지 않았다.

## 관측 결과

`cargo test --locked -p spinon-layout css_grid_2025_fit_content_row_leaves_space_for_neighbor_auto_row`가 기록된 source SHA와 동일한 Rust source에서 `1 passed, 0 failed`로 끝났다. stdout 원본은 [`implementation-test.raw.txt`](./implementation-test.raw.txt)에 있다. `cargo fmt --all -- --check`도 해당 최종 코드에서 exit 0으로 끝났으며 stdout/stderr 원본은 [`implementation-format.raw.txt`](./implementation-format.raw.txt)에 보존했다. 이후 implementation source는 변경하지 않았다.

독립 기대값 JSON의 `spinon_unit_test_root_width_px`는 WPT가 containing block 폭을 쓰는 것과 구분해 Spinon 단위 테스트의 60px root 폭을 가리킨다. 기존 precomparison 근거와 새 구현 관측을 포함해 evidence 디렉터리의 `SHA256SUMS` manifest 18개 항목 모두 `shasum -a 256 -c`에서 일치했다.

통과한 test assertions는 다음 경계를 확인한다.

- input의 두 track은 explicit rows가 아닌 실제 positive implicit rows이며 authored order가 `[FitContent(40px), Auto]`다.
- 유일한 track-sizing trace는 grid root에 속하고 Rows pass는 하나다.
- `IntrinsicSizingComplete → MaximizeTracks → ExpandFlexibleTracks → StretchAutoTracks` 순서이며 stretch stage는 한 번이고 `applied:true`다.
- CSS row bases는 각 pre-stretch stage에서 `[10,10]px`, stretch 뒤 `[10,290]px`다.
- 두 CSS rows 사이와 양 끝의 gutter 세 항목을 track과 구별한다. 경계 gutter만 collapsed sentinel이고 세 gutter base는 모두 0px이다.
- root `(0,0,60,300)`, 첫 direct item `(0,0,60,10)`, 두 번째 `(0,10,60,290)`, nested content frames `(0,0,60,10)` 및 `(0,10,60,10)`을 각각 `1e-4 CSS px` 허용치로 검사한다.

이는 고정된 CSS Grid 2025 CRD/WPT 기대값과 대조하는 단일 Rust 단위 테스트다. WPT testharness, 전체 CSS 적합성, public CSS admission, Android/iOS, simulator, browser runtime은 실행하거나 검증하지 않았다. 전체 C11.4는 미완료이고 `spec/STATUS.md`와 공식 완료 집계 `72/213`은 바꾸지 않았다. 구현 PR을 만들기 전, 계획 20-view와 독립된 새 implementation/runtime 20-view가 남아 있다.
