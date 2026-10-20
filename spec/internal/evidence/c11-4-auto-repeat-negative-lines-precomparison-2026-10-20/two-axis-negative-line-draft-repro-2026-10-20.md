# 양축 auto-repeat negative line draft 재현 · 2026-10-20

## 실행 결과

임시 auto-track adapter 검증 중 실행한 명령:

```text
mise exec -- cargo test -p spinon-layout auto_repeat --lib
```

결과는 8 passed / 4 failed였다. `numeric_negative_line_uses_expanded_auto_repeat_track_count` one-axis control은 통과했다. 아래 세 양축 repeat case는 `InvalidGridPlacement { property: "grid-row-start", reason: "line이 explicit track 범위를 벗어나 implicit track이 필요합니다" }`로 실패했다.

- `auto_repeat_leading_auto_track_lists_cycle_backwards_on_both_axes`
- `auto_repeat_scalar_auto_track_sizes_cover_leading_implicit_tracks`
- `auto_fit_keeps_repeat_count_and_collapses_empty_repeats_after_leading_placement`

네 번째 실패 `auto_repeat_rejects_non_px_implicit_track_lists`도 `UnsupportedGridAutoColumns`를 기대했지만 같은 row placement 오류가 먼저 반환됐다. 이 draft test source는 계획 정정 전에 작성된 미병합 작업이며, 정식 회귀는 수정 계획을 병합한 뒤 구현 브랜치에 반영한다.

## 원인 경로

1. `normalize_dense_grid_container`는 columns auto-repeat이 있으면 `repeat_columns = true`로 선택한다.
2. `normalize_auto_repeat_grid_items`는 columns line을 그대로 보존하지만 rows를 `fixed_start/fixed_end`로 취급하고 `fixed_count = container.rows.len()`(authored rows pattern 길이 3)을 지정한다.
3. 함수는 양축 repeat 여부와 무관하게 `resolve_axis(..., fixed_count, false)`를 호출한다.
4. `grid_auto_placement/axis.rs::endpoint`가 negative line을 `grid_placement::explicit_line`에 넘긴다. `-9 / -7`은 authored count 3을 기준으로 leading explicit boundary 밖이라고 거부된다.
5. Vendored Taffy는 layout 후반에 실제 row repeat count와 explicit row count를 계산해 raw negative line을 올바르게 정규화할 수 있지만 이 입력은 Taffy adapter 전에 실패한다.

반대 방향으로 rows auto-repeat을 먼저 선택하면 columns negative line이 같은 방식으로 실패한다. 한 축만 auto-repeat일 때 repeat axis의 raw negative line은 Spinon에서 선행 resolve하지 않아 Taffy까지 도달하는 control이다.

## 범위와 상태

이 결과는 정적 추정이 아니라 Rust layout draft test의 관찰이다. 동일 실행에서 Chrome oracle 18-case와 simulator는 실행하지 않았다. 기존 C11.4 status와 공식 완료 수는 변경하지 않는다.
