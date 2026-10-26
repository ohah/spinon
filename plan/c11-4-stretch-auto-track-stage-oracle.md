# C11.4 · auto track stretch stage oracle

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 구현 전 계획 후보. CSS Grid 2025 CRD §11.8의 auto-track stretch 결과를 독립 계산으로 고정하고 Rust trace snapshot에서 확인한다. 계획 실패 관점 20개 검토가 끝나기 전에는 구현하지 않는다. 생산 코드, FFI, 플랫폼 host, 공식 완료 상태는 변경하지 않는다.

## 목적과 경계

현재 `LayoutGridTrackSizingStageKind`는 `StretchAutoTracks { applied }`를 노출하지만 저장소의 Rust track-sizing tests는 이 stage나 `LayoutAlignContent::Stretch`를 직접 검사하지 않는다. span stage oracle은 Initialize, intrinsic span batches, IntrinsicSizingComplete와 MaximizeTracks를 고정했으므로 다음 검증은 §11.8의 별도 공간 분배 단계를 다룬다.

이 slice는 고정 크기의 단일 Grid 입력으로 남은 definite free space가 `auto` 최대 sizing function을 가진 행에만 균등 배분되는지 검사한다. Rust `spinon-layout` test module 하나만 추가한다. Grid algorithm, trace production, style projection, fixture admission, internal contract, Android/iOS app adapter는 수정하지 않는다.

## 규범 기대값

기준은 [CSS Grid Layout Module Level 1, 2025 CRD §11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-stretch)이다. `normal` 또는 `stretch` content distribution에서 이 단계는 `auto` max track sizing function을 가진 행을 대상으로 남은 양의 definite free space를 eligible track들에 균등하게 나눈다. `align-content:start` 대조 입력은 이 단계에서 트랙을 늘리지 않아야 한다.

고정 positive input:

- definite root border/content box `20×100px`, columns `[20px]`, rows `[auto, auto, 5px]`.
- `row-gap:10px`, `align-content:stretch`, `justify-content:start`, border·padding `0`.
- DOM 순서대로 한 행씩 배치된 세 direct block child의 definite height는 `[10px, 20px, 5px]`.
- `LayoutInput` viewport도 `20×100px`이고 root sizing은 `Match`다. `GridContainerInput.placement_mode`는 trace를 활성화하는 `GridPlacementMode::TrackSizing`으로 고정한다. 모든 root size와 viewport size는 서로 일치해야 한다.
- intrinsic completion과 MaximizeTracks 이후 row bases는 `[10,20,5]px`이다. 내부 gutter 두 개가 각각 `10px`이므로 남은 공간은 `100 - (10+20+5) - 2×10 = 45px`이다.
- 두 `auto` max tracks만 각각 `22.5px` 성장한다. §11.8 이후 기대 rows는 `[32.5,42.5,5]px`; fixed third track은 `5px`로 유지된다. 전체 rows와 gutters는 `100px`을 채운다.
- Positive root frame은 `(0,0,20,100)`이고 child frames는 DOM 순서대로 `(0,0,20,10)`, `(0,42.5,20,20)`, `(0,95,20,5)`다.

같은 topology와 크기의 `align-content:start` negative control은 `StretchAutoTracks { applied:false }`와 rows `[10,20,5]px`를 기대한다. unused free space는 `45px`이다.
Negative root frame은 `(0,0,20,100)`이고 child frames는 `(0,0,20,10)`, `(0,20,20,20)`, `(0,50,20,5)`다.

## 구현 계약

- 새 test-only 파일 `crates/spinon-layout/src/grid_stretch_auto_tracks_oracle_tests.rs`에 `css_grid_2025_stretch_auto_tracks_distributes_definite_free_space_equally` 검사를 추가하고, `crates/spinon-layout/src/lib.rs`에 test-only module declaration을 추가한다.
- Test input helper는 root/viewport `20×100px`, `RootSizingPolicy::Match`, 정확한 `GridContainerInput`, `GridPlacementMode::TrackSizing`, root node ID와 direct child 순서를 각 실행에서 고정한다. 이 mode가 빠지면 stage trace가 없는 입력이므로 테스트가 바로 실패해야 한다.
- Rows pass에서 `IntrinsicSizingComplete`, `MaximizeTracks`, `ExpandFlexibleTracks`, `StretchAutoTracks`의 고유 snapshot과 순서를 확인한다. positive input은 첫 두 stage에서 `[10,20,5]`, `StretchAutoTracks { applied:true }`에서 `[32.5,42.5,5]`를 검사한다. negative control은 `applied:false`이고 `[10,20,5]`를 유지해야 한다.
- Snapshot 검사에서는 CSS tracks와 gutters를 분리한다. 세 번째 fixed track을 auto tracks 집계에 포함하거나 row gaps을 free space에서 누락하면 실패해야 한다. 모든 base size는 finite여야 한다. Root/item frames도 위 좌표와 크기에 맞아야 한다.
- Assertions는 독립 산술에서 도출한 track values, root/item frames, stage identity/order를 검사한다. Taffy 결과를 기대값으로 역산하지 않는다.
- 테스트 전용 변경이므로 simulator, Android/iOS app build, browser screenshot은 검증 게이트가 아니다. Chrome 또는 앱 동작이 검증됐다고 주장하지 않는다.

## 검증과 상태

1. 이 계획의 독립된 실패 관점 20개 검토를 구현 전에 완료한다.
2. 구현 뒤 전용 Rust test, `cargo fmt --all -- --check`, `spinon-layout` targeted Clippy `-D warnings`, `git diff --check`를 실행하고 각 결과를 로그로 보존한다. 실행하지 않은 wider workspace checks는 통과로 보고하지 않는다.
3. 계획과 다른 새로운 관점의 구현/runtime 검토 20개를 완료하고, PR diff는 별도 30관점으로 확인한 뒤 Rebase and merge한다.
4. 변경은 `spec/STATUS.md`의 C11.4 checkbox나 공식 완료 집계 `72/213`을 올리지 않는다. C11.4 전체 지원, WPT 적합성 및 플랫폼 runtime parity를 주장하지 않는다.

## 남은 범위

이 검증은 definite positive free space에서의 fixed input `stretch`와 `start` 대조만 다룬다. indefinite free space, definite minimum fallback, flexible tracks, `normal` mapping, content contribution variants, WPT 및 전체 CSS Grid track sizing은 이 slice로 판정하지 않는다.
