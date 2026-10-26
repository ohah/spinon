# C11.4 §11.8 auto-track stretch stage oracle

## 범위

기존 trace가 노출하는 `StretchAutoTracks` snapshot을 검사하는 `spinon-layout` 테스트 전용 작업이다. track sizing, style projection, FFI, Android/iOS adapter는 변경하지 않는다.

규범 기준은 [CSS Grid Layout Module Level 1, 2025 CRD §11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-stretch)이다. `normal` 또는 `stretch` content distribution에서는 남은 양의 definite free space를 max sizing function이 `auto`인 track에 균등하게 배분한다.

## 고정 입력과 기대값

- Root와 viewport는 `20×100px`, `RootSizingPolicy::Match`, border/padding `0`이다.
- Columns는 `[20px]`, rows는 `[auto, auto, 5px]`, row gap은 `10px`, placement mode는 `TrackSizing`이다.
- DOM 순서대로 한 행에 하나씩 배치한 direct child의 고정 높이는 `[10,20,5]px`이다.
- §11.8 이전 row base는 `[10,20,5]px`이고 내부 gutter 두 개의 합은 `20px`이다. 남은 공간은 `100 - 35 - 20 = 45px`이다.
- `align-content:stretch`에서는 `45px`을 두 auto-max row에 나눠 각각 `22.5px` 성장시킨다. 기대 row는 `[32.5,42.5,5]px`이고 마지막 fixed row는 `5px`를 유지한다. row와 gutter 합은 `100px`이다.
- Stretch child frame은 `(0,0,20,10)`, `(0,42.5,20,20)`, `(0,95,20,5)`이다.
- `align-content:start` 대조군은 `StretchAutoTracks { applied:false }`, rows `[10,20,5]px`, 남는 공간 `45px`, child y `[0,20,50]`을 기대한다.

기대값은 규범과 위 산술에서 독립 도출했다. Taffy 출력값을 기대값으로 사용하지 않는다. 테스트는 `Rows` pass에서 `IntrinsicSizingComplete → MaximizeTracks → ExpandFlexibleTracks → StretchAutoTracks` 순서를 확인하고, pre-stretch snapshot·최종 track·gutter·root frame·child frame을 검사한다.

## 검증 결과

| 검사 | 결과 |
| --- | --- |
| `mise exec -- cargo test --locked -p spinon-layout css_grid_2025_stretch_auto_tracks_distributes_definite_free_space_equally` | 1/1 통과 |
| `mise exec -- cargo fmt --all -- --check` | 통과 |
| `mise exec -- cargo clippy --locked -p spinon-layout --all-targets -- -D warnings` | 통과 |
| `git diff --check` | 통과 |

원본 명령 출력은 [focused-test.log](./focused-test.log), [fmt-check.log](./fmt-check.log), [clippy.log](./clippy.log)에 있다. 첫 focused test 실행에서 새 test의 `LayoutEngine` trait import가 빠져 compile이 실패했다. import를 추가한 뒤 재실행해 통과했다.

계획과 구현은 각각 별도 20관점 검토에서 PASS 20/20을 받았다: [계획 검토](../../../../plan/c11-4-stretch-auto-track-stage-plan-review-20-2026-10-26.md), [구현 검토](./implementation-review-20.md). PR diff 30관점 검토는 진행 중이다.

변경 범위는 Rust test뿐이므로 앱 동작을 검증하는 Android/iOS build와 simulator capture는 수행하지 않았다. 브라우저·플랫폼 parity, WPT 적합성 또는 C11.4 완료를 주장하지 않는다. 공식 C11.4 checkbox와 `72/213` 완료 수는 유지한다.
