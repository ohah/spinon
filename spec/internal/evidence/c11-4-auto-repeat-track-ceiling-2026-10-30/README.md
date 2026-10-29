# C11.4 auto-repeat 10,000 track 경계

## 범위

이 test-only slice는 vendored Taffy 0.15.0 adapter의 expanded explicit-track 수를 9,999px·10,000px·10,001px에서 고정한다. Columns와 Rows는 각각 실행한다. 각 입력에서 `used_columns.explicit_track_count` 또는 `used_rows.explicit_track_count`를 non-gutter `Initialize` stage 수와 비교한다. 빈 `auto-fill` track은 유지되고 빈 `auto-fit` track은 collapse된다.

2px prefix, repeated 1px track, 3px suffix를 쓰는 입력 네 개도 검사했다. 10,003px에서는 repeated track 9,998개와 suffix가 남아 합계 10,000개다. 10,004px에서는 prefix를 보존하고 repeated track 9,999개로 채운 뒤 suffix를 자른다. 두 축 모두 통과했다.

최대 실행은 captured track state가 100,000-state trace budget보다 작은지도 확인한다. 한 입력에서 Rows와 Columns를 동시에 최대로 설정하지 않는다. 시각적인 앱 장면이 아니라 Rust layout test이므로 Android/iOS 시뮬레이터 실행과 screenshot은 해당하지 않는다.

## revision과 결과

- 계획: [C11.4 auto-repeat ceiling](../../../../plan/c11-4-auto-repeat-track-ceiling.md). PR [#824](https://github.com/ohah/spinon/pull/824)에서 추가했고 merge commit은 `1086b60250c00996107654048f42cdc644eba950`이다.
- Test source commit: `196a75c8a` (`test(c11.4): auto-repeat track ceiling 경계 검증 추가`). 캡처한 실행은 이 commit과 동일한 tracked source를 사용했다.
- 집중 테스트: 2 passed, 0 failed. 두 테스트는 경계 입력 16개를 검사한다.
- `spinon-layout` Grid 전체 선택: 134 passed, 0 failed.
- `spinon-layout` crate 전체: 312 passed, 0 failed; doc test는 0개다.
- `cargo clippy --locked -p spinon-layout --all-targets -- -D warnings`: 통과.
- `cargo fmt --all -- --check`와 `git diff --cached --check`: 통과.
- 로그: [집중 테스트](./focused-tests.log), [Grid 선택](./grid-tests.log), [crate 전체](./spinon-layout-tests.log), [Clippy](./spinon-layout-clippy.log).
- 계획 검토: [실패 관점 20개](../../../../plan/c11-4-auto-repeat-track-ceiling-plan-review-20.md) · [계획 PR diff 검토](../c11-4-auto-repeat-track-ceiling-plan-2026-10-30/pr-diff-review-30.md).
- 독립 구현/runtime 검토: [실패 관점 20개](./implementation-runtime-review-20.md).
- 최종 변경 검토: [PR diff 실패 관점 30개](./pr-diff-review-30.md).

## 수정한 실패 시도

- 첫 컴파일에서 `LayoutEngine` trait import가 빠져 method resolution이 실패했다. import를 추가해 고쳤다.
- 첫 test filter에는 부모 `grid::` 경로가 빠져 0개가 선택됐다. 경로를 바로잡은 집중 명령은 두 테스트를 실행했다.
- 첫 assertion은 collapse된 auto-fit track도 1px base size를 유지한다고 가정했다. 실제 trace가 collapse를 보여줘, assertion을 기대한 collapse 상태와 정확한 track 수 검사로 고쳤다.
- 첫 source-order 계획은 prefix/suffix 5px를 repeat 계산에서 빼지 않아 경계가 한 track 어긋났다. 병합한 계획은 10,003px fitting과 10,004px cap 입력을 사용한다.
- 처음 `git diff --check`는 저장 로그 끝의 빈 줄을 지적했다. 끝의 빈 줄을 제거한 뒤 통과했다.

## 한계

이 결과는 고정 1px repeat pattern, zero gap, 계산마다 한 축만 최대인 입력, Taffy 0.15.0의 clamp 동작에 한정된다. named line, explicit item placement, multi-track repeat pattern, percentage/intrinsic sizing, implicit track, 전체 auto-repeat grammar, WPT 적합성 또는 앱 pixel은 검사하지 않았다. trace budget이 모든 10,000-track layout의 성공을 보장하지 않는다. C11.4와 C11은 미완료이며 공식 합계는 `72/213`이다.
