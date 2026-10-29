# C11.4 definite percentage auto-repeat cascade 회귀 수정

## 범위

2026-10-29 기준 `main` (`4ce6cd358b27a23c27422ed26d30b3466afbe054`)에서 기존 cascade test가 유효한 `repeat(auto-fill,10%)`를 unsupported로 잘못 분류했다. cascade 구현은 이미 `ComputedCssGridTrackSize::Percentage(0.1)`과 `AutoFill` metadata를 반환했다. 이 변경은 그 기대값을 열·행 × `auto-fill`·`auto-fit` 네 조합의 positive regression test로 고정하고, 독립적으로 unsupported인 percentage minimum + intrinsic maximum 거부 검사를 유지한다. 제품 코드는 변경하지 않는다.

## 기준과 재현

CSS Grid Layout Module Level 1, 2025 CRD [§7.2.3.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#repeat-syntax)은 `<fixed-breadth>`를 0 이상 `<length-percentage>`로 정의한다. 따라서 definite axis의 bare `10%`는 auto-repeat의 유효한 fixed-size track이다. 기존 direct percentage slice의 계획은 [C11.4 track sizing](../../../../plan/c11-4-grid-track-sizing.md)에 있고, 관련 runtime 경계는 [내부 계약 0059](../../0059-c11-grid-layout.md)와 [percentage auto-repeat layout tests](../../../../crates/spinon-layout/src/grid_track_sizing_tests/positive_percentage_auto_repeat.rs)에 고정되어 있다. layout tests는 직접 `GridTrackSize::Percentage`를 다루고 definite content axis를 요구하며 indefinite/clamped axis를 거부한다.

수정 전 재현 명령:

```sh
cargo test --locked -p spinon-style --lib track_sizing_profile_rejects_out_of_scope_percentage_auto_repeat_minimums -- --nocapture
```

수정 전 결과는 1 test failure다. `unwrap_err()`가 `Ok`에서 panic했고 관측된 root style에는 `columns=[Percentage(0.1)]`, `columns_auto_repeat.mode=AutoFill`이 있었다. 이는 assertion 불일치였고 cascade 계산 실패는 아니었다.

## 수정 및 검증

- definite `124px` border-box root, 양쪽 2px border, 0 padding으로 120px content axis를 만든 뒤, columns/rows 각각에 `repeat(auto-fill,10%)`와 `repeat(auto-fit,10%)`를 검사한다.
- computed track이 정확히 `Percentage(0.1)`이고 auto-repeat mode가 authored mode와 같은지 확인한다.
- `minmax(0%|10%,auto|max-content)`의 unsupported cascade 검사와 node/property 오류 경계를 유지한다.
- 기존 direct percentage layout suite는 definite-axis sizing, auto-fit collapse 및 indefinite/clamped-axis 거부를 계속 검사한다.

실행 결과:

| 검사 | 결과 |
|---|---:|
| `cargo test --locked -p spinon-style --lib auto_repeat -- --nocapture` | 46 passed |
| `cargo test --locked -p spinon-style` | 250 passed |
| `cargo test --locked -p spinon-layout --lib positive_percentage_auto_repeat -- --nocapture` | 13 passed |
| `cargo fmt --all -- --check` | 통과 |
| `git diff --check` | 통과 |
| `cargo clippy --locked -p spinon-style --all-targets -- -D warnings` | 기존 `clippy::too_many_arguments`가 수정하지 않은 `cascade/incremental/grid.rs:438`에서 실패 |
| `cargo clippy --locked -p spinon-style --all-targets -- -D warnings -A clippy::too_many_arguments` | 통과; 위 기존 lint 한 항목만 제외 |

변경은 cascade test와 상태/evidence 문서뿐이므로 Android/iOS app runtime 경로는 달라지지 않는다. 이 변경에서는 앱 시뮬레이터를 실행하지 않았다. Clippy 결과를 무시하거나 제품 코드에 lint 예외를 추가하지 않았다.

## 남은 범위

이 회귀 수정은 C11.4의 일부 cascade 검사만 다룬다. 일반 author CSS 지원, 전체 WPT, 전체 CSS Grid 또는 전체 C11.4 완료를 뜻하지 않는다. C11.4 상태 체크와 공식 완료 수 `72/213`은 그대로 둔다.
