# C11.4 `align-content: normal` §11.8 oracle 구현 근거

## 변경 범위

기준 main `fb1704f950ce29097c482f16d7ec4401b0009493`에 Rust 테스트 커밋 `31c7874a4675bd9182c83aab2bc711a93ced9c34`를 적용했다. 변경은 [`grid_stretch_auto_tracks_oracle_tests.rs`](../../../../crates/spinon-layout/src/grid_stretch_auto_tracks_oracle_tests.rs)의 `css_grid_2025_align_content_normal_stretches_auto_tracks_equally` 단위 테스트 하나다. 생산 알고리즘, style projection, FFI, app, runtime fixture 또는 공개 계약은 바꾸지 않았다.

테스트는 기존 `input(LayoutAlignContent::Normal)` helper를 사용한다. 이는 `align-content:normal`과 `justify-content:flex-start`(`LayoutJustifyContent::FlexStart`) 입력이다. 고정 독립 기대값은 [사전 비교](../c11-4-align-content-normal-auto-track-stage-precomparison-2026-10-26/README.md), [input JSON](../c11-4-align-content-normal-auto-track-stage-precomparison-2026-10-26/input.json), [expected JSON](../c11-4-align-content-normal-auto-track-stage-precomparison-2026-10-26/expected.json)에 있다. 기준은 [CSS Grid 2025 CRD §11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-stretch)이다.

## 관찰 결과

- Row pass는 `IntrinsicSizingComplete → MaximizeTracks → ExpandFlexibleTracks → StretchAutoTracks` 순서이며 각 stage가 한 번 나타난다.
- 앞의 세 stage CSS row bases는 `[10,20,5]px`; 마지막은 `StretchAutoTracks { applied: true }`, `[32.5,42.5,5]px`다. 내부 gutter 두 개는 각 `10px`, 끝 sentinel은 각각 `0px`이고 남은 row space는 `0px`다.
- Root frame은 `(0,0,20,100)`이고 direct child frames는 `(0,0,20,10)`, `(0,42.5,20,20)`, `(0,95,20,5)`다. 유한값과 모든 수치 비교에 절대 오차 `1e-4 CSS px`를 사용했다.

## 검증

| 명령 | 결과 |
| --- | --- |
| `cargo test --locked -p spinon-layout css_grid_2025_align_content_normal_stretches_auto_tracks_equally` | PASS, 1 passed, 0 failed, 276 filtered out |
| `cargo test --locked -p spinon-layout grid_stretch_auto_tracks_oracle_tests` | PASS, 2 passed, 0 failed, 275 filtered out; 새 `normal` 테스트와 기존 `stretch`/`start` 대조군이 포함됐다. [원본 출력](./grid-stretch-auto-tracks-test.log.raw.gz) |
| `cargo fmt --all -- --check` | PASS (formatter 적용 뒤 재검사) |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | PASS |
| `git diff --check` | PASS |

재현 환경의 Rust/Cargo 버전은 [`toolchain.txt`](./toolchain.txt)에 기록했다. 통합된 두 테스트의 원본 stdout은 [`grid-stretch-auto-tracks-test.log.raw.gz`](./grid-stretch-auto-tracks-test.log.raw.gz)에 보존했다.

변경은 `#[cfg(test)]` Rust 단위 테스트뿐이므로 Android/iOS app build와 simulator capture는 이 slice의 검증 대상이 아니다. 이 근거는 일반 CSS parsing/cascade, 앱 렌더링, Chrome/WPT 또는 전체 C11.4를 입증하지 않는다. C11.4 공식 체크박스와 완료 집계 `72/213`은 미완료로 유지한다. 별도 구현 실패 관점 20개 및 PR diff 30관점 검토는 진행 예정이다.
