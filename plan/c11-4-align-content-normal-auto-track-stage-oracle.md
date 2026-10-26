# C11.4 · `align-content: normal` 자동 트랙 늘림 단계 oracle

**상태:** 계획 자체 검토 PASS 20/20, 계획 PR [#655](https://github.com/ohah/spinon/pull/655) diff 검토 PASS 30/30 후 `fb1704f950ce29097c482f16d7ec4401b0009493`에 병합됐다. 구현 커밋 `31c7874a4675bd9182c83aab2bc711a93ced9c34`에서 테스트를 추가했고 계획한 focused test, fmt, Clippy 검증이 통과했다. 별도 구현 검토 20개와 구현 PR diff 30개는 진행 예정이다. [구현·실행 근거](../spec/internal/evidence/c11-4-align-content-normal-auto-track-stage-oracle-2026-10-26/README.md). C11.4와 공식 완료 집계 `72/213`은 미완료로 유지한다.

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 Grid 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**사전 비교:** [§11.8 고정 입력과 독립 기대값](../spec/internal/evidence/c11-4-align-content-normal-auto-track-stage-precomparison-2026-10-26/README.md)

## 목적과 범위

병합된 §11.8 test-only oracle은 `align-content: stretch`의 positive 입력과 `start` 대조군을 검사한다. 이 후속은 같은 fixed geometry에서 `align-content: normal`도 `StretchAutoTracks { applied: true }` 단계를 거치는지 확인한다. 현재 `taffy_style.rs`는 `LayoutAlignContent::Normal`을 `AlignContent::STRETCH`로 매핑하지만, 이 입력을 직접 검사하는 track-stage test는 없다.

변경은 `crates/spinon-layout/src/grid_stretch_auto_tracks_oracle_tests.rs`의 새 Rust 단위 테스트 하나로 제한한다. 기존 `input`, stage, track, frame helper를 재사용한다. 생산 알고리즘, style projection, Grid trace 생성, FFI, fixture admission, Android/iOS host, 공개 명세와 `spec/STATUS.md`는 수정하지 않는다.

## 규범 기대값

기준은 [CSS Grid Layout Module Level 1, 2025 CRD §11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-stretch)이다. 해당 단계는 관련 축의 content-distribution 값이 `normal` 또는 `stretch`일 때 남은 양의 definite free space를 `auto` max track sizing function을 가진 track에 균등하게 나눈다.

행 축 입력은 root `20×100px`, rows `[auto, auto, 5px]`, `row-gap:10px`, direct child heights `[10,20,5]px`, `align-content:normal`, `justify-content:flex-start`, border/padding `0`이다. 이는 기존 `input` helper의 `LayoutJustifyContent::FlexStart`와 일치한다. Root height와 viewport height는 모두 definite `100px`이고, placement mode는 stage trace를 요청하는 `TrackSizing`이다.

`IntrinsicSizingComplete`, `MaximizeTracks`, `ExpandFlexibleTracks` 뒤의 독립 track bases는 `[10,20,5]px`이다. 두 내부 gutter는 각 `10px`이므로 §11.8 이전 free space는 다음과 같다.

```text
100 - (10 + 20 + 5) - (10 + 10) = 45px
```

`auto` max function인 첫 두 행만 각각 `45 / 2 = 22.5px` 증가한다. 기대 결과는 rows `[32.5,42.5,5]px`, fixed 세 번째 행 `5px` 유지, 남은 행 축 공간 `0px`, child y 좌표 `[0,42.5,95]px`이다. Root frame은 `(0,0,20,100)`이다. 전체 입력과 수치는 [사전 비교 README](../spec/internal/evidence/c11-4-align-content-normal-auto-track-stage-precomparison-2026-10-26/README.md), [입력 JSON](../spec/internal/evidence/c11-4-align-content-normal-auto-track-stage-precomparison-2026-10-26/input.json), [기대값 JSON](../spec/internal/evidence/c11-4-align-content-normal-auto-track-stage-precomparison-2026-10-26/expected.json)에 고정한다.

## 구현 계약

- 별도 `#[test]`를 추가해 `input(LayoutAlignContent::Normal)`을 계산한다. 기대값은 §11.8과 사전 비교의 독립 산술에서 가져오며 `TaffyLayoutEngine` 출력으로 역산하지 않는다.
- Row pass에서 `IntrinsicSizingComplete → MaximizeTracks → ExpandFlexibleTracks → StretchAutoTracks`가 각각 한 번만 있고 순서가 맞는지 확인한다.
- 처음 세 stage의 CSS row bases `[10,20,5]px`, 최종 stage의 `StretchAutoTracks { applied:true }`, CSS row bases `[32.5,42.5,5]px`를 확인한다. Gutter는 CSS track 집계에서 제외하고 두 내부 gutter 각각 `10px`, 양 끝 sentinel `0px`인지 확인한다.
- Root frame `(0,0,20,100)`과 direct children frame `(0,0,20,10)`, `(0,42.5,20,20)`, `(0,95,20,5)`를 확인한다. 기대값 comparator는 기존 test와 같이 finite 입력 및 절대 오차 `1e-4 CSS px`를 사용한다.
- 같은 입력의 기존 `stretch` positive는 동등한 tracks/frames를, `start` control은 `applied:false`, rows `[10,20,5]px`, child y `[0,20,50]px`를 보인다. 새 test는 두 기존 결과를 oracle로 사용하지 않고 독립 기대값과 각각 대조한다.
- 구현 전 독립된 실패 관점 20개 계획 검토와 계획 PR diff 30관점 검토를 완료한다. 구현 후에는 별도의 새 실패 관점 20개 구현 검토와 PR diff 30관점 검토를 수행한다.

## 검증 경계

이 변경은 이미 지원되는 Rust `LayoutAlignContent::Normal` 입력이 §11.8 stage trace에서 어떤 결과를 내는지 확인하는 단위 테스트다. 새 public CSS admission, product runtime, app route, rendering 또는 platform behavior를 추가하지 않는다. 앱 파일과 FFI 경로를 건드리지 않는 계획이므로 Android/iOS app build와 simulator capture는 적용되지 않는다. Chrome/WPT 적합성도 이 test-only slice로 주장하지 않는다.

구현 후 다음 검증을 실행하고 결과를 PR 근거에 연결한다.

```sh
cargo test --locked -p spinon-layout css_grid_2025_align_content_normal_stretches_auto_tracks_equally
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets -- -D warnings
git diff --check
```

Indefinite free space, definite minimum-size fallback, flex tracks, 다른 track sizing function, 다른 item contribution, auto-repeat, CSS cascade와 전체 C11.4는 범위 밖이다. 공식 체크박스 및 `72/213`은 바꾸지 않는다.
