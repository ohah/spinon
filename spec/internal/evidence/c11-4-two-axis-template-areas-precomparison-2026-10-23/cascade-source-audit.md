# 양 축 repeat + areas cascade gate 소스 추적

**기준 revision:** `8fb233be476aded68d1c33151b2c5f00c73912d0` (PR #543 merge SHA)

**구현 상태:** 아래 소스는 추적만 했다. 이번 계획 수정 PR에서는 Rust 코드를 변경하지 않았다.

## Cascade

`crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs`에서 columns와 rows를 각각 `computed_grid_tracks`로 읽고 `columns_auto_repeat`/`rows_auto_repeat`을 보존한다. 그 직후 `template_areas.is_some() && columns_auto_repeat.is_some() && rows_auto_repeat.is_some()`이면 `grid-template-columns` unsupported 오류를 반환한다. 이 guard가 현재 조합의 cascade 차단점이다.

`crates/spinon-style/src/stylo_dom/cascade/incremental/element.rs`의 call site는 `allow_integer_repeat`, `allow_intrinsic_tracks`, `allow_auto_repeat`를 따로 전달한다. `allow_auto_repeat`은 `profile == RuntimeBlockFlexGridTrackSizingV1`일 때만 true다. `RuntimeBlockFlexGridTemplateV1`은 `grid-template-areas`/integer repeat 입력은 허용할 수 있지만 `auto-repeat`은 허용하지 않는다.

따라서 cascade 조건을 무조건 제거하면 안 된다. 계획 대상 profile에서만 조합 guard를 풀고, 다른 profile은 현재 repeat 거부를 그대로 둔다. 예상 제어는 TrackSizing에서 두 repeat metadata와 area metadata를 모두 계산하는 성공, TemplateV1에서 auto-repeat 거부, invalid area/repeat input의 기존 실패다.

## Style projection

`crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs`는 auto-repeat이 있을 때 `RuntimeBlockFlexGridTrackSizingV1` 외 profile을 거부하고, 추가로 area와 두 auto-repeat이 함께 있으면 같은 오류를 반환한다. 대상 profile guard는 유지하고 area + two-axis 추가 거부만 제거한다.

## 검증할 경계

- Cascade 성공 뒤 `ComputedCssGridLayout`이 columns/rows auto-repeat과 areas를 각각 보존해야 한다.
- Projection 이후 `GridContainerInput`과 Taffy style에서 같은 두 axes와 areas가 유지돼야 한다.
- TemplateV1 및 이전 profile, 같은 axis 중복 repeat, invalid area는 fail-closed로 남는다.
- Cascade 오류는 snapshot/layout publication 이전에 반환되어 이전 revision이나 partial frame을 새 결과로 게시하지 않는다.
- 기존 12-scene dual-axis repeat과 13-scene single-axis area matrix, 신규 9-scene matrix를 분리해 회귀한다.
