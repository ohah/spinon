# C11.4 · auto-repeat과 typed implicit track sizing 계획

**상위 원본:** [C11 상태 대장](../spec/STATUS.md#c11-css-grid) · [C11.4 전체 track sizing 계획](./c11-4-grid-track-sizing.md) · [CSS Grid Level 1 2025 CRD §7.5–§7.6, §8.3, §11.3–§11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** typed implicit track sizing 14개 fixture를 구현했고 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 기존 124개를 합친 138개 장면을 각각 통과했다. 구현 전 계획은 [실패 관점 20개](./c11-4-auto-repeat-typed-implicit-plan-review-20.md)와 [계획 PR diff 30개](./c11-4-auto-repeat-typed-implicit-pr-diff-review-30.md)로 별도 검토했고, 구현 코드는 [새 실패 관점 20개](../spec/internal/evidence/c11-4-negative-implicit-track-sizing-2026-10-20/auto-repeat-typed-implementation-review-20.md)로 검토했다. 최종 PR diff 30개 검토와 PR 병합은 남아 있다. 이 제한 구현은 C11.4/C11 완료를 뜻하지 않는다.

## 목적

PR #385는 `RuntimeBlockFlexGridTrackSizingV1`에서 현재 제한 grammar의 auto-repeat과 numeric positive/negative line placement를 결합했다. auto-repeat이 없는 같은 profile은 `grid-auto-rows`와 `grid-auto-columns`의 typed `<track-size>+` 목록을 positive implicit tracks에 적용한다. 반면 auto-repeat이 있으면 Taffy 변환기가 fixed `px` scalar/list만 전달하고, 다른 typed track list는 거부한다. 이 후속은 두 구현 경로의 차이를 줄여 auto-repeat의 negative·positive implicit tracks에서 C11.4가 이미 투영하는 track size 타입을 동일하게 계산한다.

## 구현 경계

다음 조건의 `RuntimeBlockFlexGridTrackSizingV1`만 확장한다.

1. 각 축의 `grid-auto-rows`·`grid-auto-columns`는 현재 computed-style projection이 `GridTrackSize`로 보존하는 비어 있지 않은 목록을 받는다. 기존 supported typed forms인 `LengthPx`, percentage, fraction, `auto`, `min-content`, `max-content`, `minmax()`, `fit-content()` 및 기존 CSS math projection의 `calc()`를 대상으로 한다. 신규 CSS parser, 신규 Grid profile, line-name grammar 또는 `repeat()` 문법을 열지 않는다.
2. auto-repeat은 기존 지원 subset대로 각 axis track list에 `auto-fill` 또는 `auto-fit` 한 개까지 허용한다. 각 축의 definite available size는 고정 fixture 입력으로 확인한다. 기존 auto-repeat component, leading/trailing fixed-px neighbor, auto-repeat count, `auto-fit` collapse는 유지한다.
3. fixed implicit track 목록은 explicit grid 뒤에서 authored order로, explicit grid 앞에서는 역순 cycle로 적용한다. negative line의 기준 grid에는 expanded auto-repeat track count가 포함된다. 두 축의 auto-repeat과 서로 다른 row/column auto-track 목록을 독립 적용한다.
4. negative 또는 비유한 fixed lengths, 잘못된 `minmax()` 함수 조합, empty list, unsupported computed value, 미지원 placement/profile과 track 상한 초과에서는 구체적인 오류를 유지하고 부분 `LayoutOutput`을 반환하지 않는다. `0px`는 CSS track size로 유효하므로 거부하지 않는다. Taffy 내부 working allocation의 전역 OOM 복구를 주장하지 않는다.
5. 새 공개 API는 없다. C11.2.2·C11.2.3·TemplateGrid 및 다른 기존 placement profile의 auto-track 허용 범위와 진단은 넓히지 않는다. 내부 계약 숫자 버전은 `0.1.0`을 유지한다.

## 구현 경로

1. `crates/spinon-layout/src/grid.rs`의 `validate_auto_repeat_auto_track_list`는 현재 auto-repeat과 결합된 `grid-auto-*` 목록을 fixed `px`로만 제한한다. `RuntimeBlockFlexGridTrackSizingV1`에서는 앞서 실행되는 `validate_tracks`의 비어 있지 않은 목록, track 타입, 유한성, 비음수 값과 10,000 track 상한 검증을 신뢰해 typed list도 통과시킨다. 다른 profile은 이 확장을 받지 않는다.
2. `crates/spinon-layout/src/taffy_style/grid.rs`의 `to_taffy_grid_auto_tracks`는 현재 `LengthPx` 이외의 list 항목을 거부한다. `calc_handles`를 전달해 각 list 항목을 기존 `to_taffy_track_sizing_function`으로 변환한다. `grid_math_property`는 auto-columns/auto-rows를 기존 axis math identity인 `GridTemplateColumns`/`GridTemplateRows`에 연결하되 진단에는 `grid-auto-*` 이름을 보존한다. auto, scalar, reservation 실패 처리는 유지한다.
3. `crates/spinon-layout/src/lib.rs`의 `validate_css_math_references`에 auto-columns/auto-rows의 top-level, `minmax()`와 `fit-content()` 안의 Calc 참조도 포함한다. ID가 존재하고 같은 node 및 해당 축의 기존 math property에 묶였는지 검증한다. 잘못 결합된 handle을 Taffy에 전달하지 않는다.
4. `crates/spinon-layout/src/taffy_style.rs`가 어느 축이든 auto-repeat을 가진 경우 양 축 auto track list를 Taffy에 전달하는 기존 분기와, auto-repeat 없는 경우 `grid_auto_placement/track_sizing/tracks.rs`가 수행하는 양·음 implicit track 확장은 그대로 둔다. 추가 회귀는 `crates/spinon-layout/src/grid_auto_track_sizing_tests/`와 `crates/spinon-style-to-layout/src/projection/style_values/grid_values/auto_track_tests.rs`에 둔다.
5. runtime 행렬은 `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_negative_lines.rs`와 연결 HTML/evidence에서 확장한다. typed-list, 양·음 placement, auto-fit collapse, nonzero gutter, columns-only/rows-only 및 두 축 동시 auto-repeat을 독립적으로 확인한다.

## 사전 기준과 fixture

[사전 비교](../spec/internal/evidence/c11-4-auto-repeat-typed-implicit-precomparison-2026-10-20/README.md)는 W3C 규범 기대와 Chrome 관찰을 분리한다. 기본 Chrome fixture의 definite content size는 양 축 110px이며, `repeat(auto-fill, 20px)`은 축마다 5 tracks다. gutter case는 content size 132px과 8px gaps에서 같은 5-track repeat count를 가진다. 기본 negative placement `-8/-7`은 두 leading implicit tracks, cycle placement `-9/-8`은 세 leading tracks를 만든다. positive placement `8/9`는 세 trailing tracks를 만든다. 14개 fixture case는 scalar·list cycle control, mixed typed cycle, `auto`, min/max-content, `minmax`, `fit-content`, flex, percentage, `calc()` 및 auto-fit occupied/empty/gutter collapse를 다룬다.

- Normative oracle: CSS Grid Level 1 2025 CRD §7.5–§7.6, §8.3, §11.3–§11.8. Browser가 다른 값을 관찰하면 규범 기대와 observation을 각각 보존한다.
- Chrome observation: Agent Browser HeadlessChrome 155.0.8059.85, viewport 1280×1800 CSS px, DPR 1. 원본 JSON·screenshot·fixture·SHA256SUMS를 사전 비교 폴더에 둔다.
- Integer track sizes, repeat count, signed boundary placement와 integer frames는 exact 비교한다. fractional values는 값마다 1/64 CSS px 이내다. list의 authored order와 leading/trailing cycle은 최종 크기와 독립해 trace로 확인한다.
- `auto`, intrinsic, `fit-content`, flex와 text wrapping은 contribution source·base/growth-limit stage·final used track을 모두 검사한다. Chrome frame 일치만으로 독립 규범 기대가 통과한 것으로 판정하지 않는다.

## 구현 검증 행렬

1. Rust layout 및 style projection에서 양 축 각각 scalar, 단일·다중 typed list, leading/trailing cycle, explicit/implicit 경계 span, negative/positive signed lines, empty/hidden item, `auto-fill`/`auto-fit`, nonzero gutters, 두 축 repeat 교차를 검사한다.
2. `auto`, `min-content`, `max-content`, fixed/percentage/fraction, `minmax()`, `fit-content()`와 지원되는 `calc()`의 typed conversion 및 track stage·contribution source를 확인한다. definite percentage basis를 유지하고 indefinite/cyclic cases는 이 구현에서 허용하지 않는다.
3. `minmax()` 유효성·min/max 순서, negative/non-finite lengths, 유효한 zero-size track control, invalid math handle, empty list, unsupported grammar/profile, list length와 track count 상한, allocation failure control을 각각 검사한다. 오류가 있을 때 부분 frames·scene 제출이 없어야 한다.
4. 기존 C11.4 124-scene 기준과 신규 14개 typed-list route를 합친 Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator 각각 138-scene matrix를 실행한다. 각 ID의 input, revision, expanded track count, used track list, item/root frame, axis trace, draw/present 응답을 검증한다.
5. overflow면 Android PixelCopy와 iOS Simulator Metal drawable에서 root 안·경계·바깥 item 위치를 고정해 표본 비교한다. 이 결과는 AVD software renderer 및 iOS compositor 전 readback으로 표시하며 physical device, hardware GPU 또는 compositor 이후 pixel 근거로 일반화하지 않는다.
6. 변경 범위에 맞는 focused layout/style/runtime 테스트, `cargo fmt --all -- --check`, workspace tests, all-target Clippy, Android/iOS Simulator build gate와 fixture artifact negative/positive control을 실행한다. 이전 C11.2/C11.3 및 C11.4 profile matrix의 기존 실패는 변경 전 baseline과 분리해 보고한다.
7. 구현 뒤 코드와 simulator 동작은 계획 표를 재사용하지 않은 [새 실패 관점 20개](../spec/internal/evidence/c11-4-negative-implicit-track-sizing-2026-10-20/auto-repeat-typed-implementation-review-20.md)로 검토했다. 최종 implementation PR diff는 저장소 규칙대로 30개의 별도 관점으로 검토하고 수정된 관점은 새 revision에서 재검증한다.
8. 계획 PR 병합 뒤 같은 `/Users/yoonhb/Documents/workspace/spinon` 체크아웃에서 `origin/main`으로 전환해 구현 branch를 만든다. 새 worktree는 만들지 않으며 미추적 simulator evidence를 보존한다. 구현 PR 본문에 Android/iOS Simulator 화면을 raw image link로 첨부할 수 있으면 첨부한다. 병합 직전 `gh pr checks --required` 및 merge state를 새로 확인한다. required checks가 없으면 없다고 기록하고 CI 통과로 쓰지 않는다. 승인된 작업 PR은 `Rebase and merge`한다.
9. 실제 구현 범위와 남은 제한을 내부 계약 0059, `plan/c11-4-grid-track-sizing.md`, `spec/STATUS.md`에 기록한다. C11.4 및 C11 완료 체크·완료 수는 올리지 않는다. 최신 `main` 문서 산출물의 Tailnet preview sync는 별도 PR로 처리하고 served hash와 Agent Browser 렌더링을 검증한다.

## 범위 밖

이 slice는 full CSS Grid track sizing, arbitrary `grid-auto-*`/repeat grammar, line names·template areas, indefinite/cyclic percentage, orthogonal writing mode, subgrid, nested/anonymous/out-of-flow item, 전체 WPT, physical devices, Android hardware GPU와 iOS compositor-after pixels를 완료하지 않는다. auto-repeat과 typed implicit sizing profile을 구현하더라도 C11.4, C11 또는 공개 CSS 지원 완료로 승격하지 않는다.

## 2026-10-20 구현 결과

- `RuntimeBlockFlexGridTrackSizingV1`의 auto-repeat 결합 `grid-auto-rows`·`grid-auto-columns` 목록에 기존 typed converter를 연결했다. percentage·fraction·`auto`·intrinsic sizing·`minmax()`·`fit-content()`·`calc()`와 양·음 implicit cycle을 유지하고, Calc ID는 기존 columns/rows math identity와 node에 묶어 검증한다. 다른 placement profile에는 허용 범위를 넓히지 않았다.
- Chrome 155 사전 비교의 14개 typed case를 기존 124개 runtime case와 합쳤다. Android AVD와 iOS Simulator 각각 138/138 장면에서 node frame·expanded used tracks·collapse state·trace completeness·draw/present를 확인했다. 두 실행의 960개 frame scalar와 1,019개 track size가 모두 정확히 일치했다.
- 두 플랫폼에서 overflow 5개 장면의 표면 표본도 맞았다. `auto-fill-overflow-neighbors`는 4/4, 나머지 네 장면은 각각 5/5 exact다. [실행 자료 및 화면](../spec/internal/evidence/c11-4-negative-implicit-track-sizing-2026-10-20/README.md).
- `cargo test --locked --workspace --quiet` 1,182 passed·4 ignored, C11.4 runtime fixture package 209 passed·3 ignored, workspace all-target Clippy와 rustfmt, fixture-module-boundary 6/6, opt-in FFI artifact 경계를 확인했다. 상세 수치는 evidence README를 따른다.
- 구현 실패 관점 20개 검토는 [실행 근거](../spec/internal/evidence/c11-4-negative-implicit-track-sizing-2026-10-20/auto-repeat-typed-implementation-review-20.md)에 기록했다. 이 implementation PR diff의 별도 30개 검토는 PR 변경사항이 고정된 뒤 기록한다. WPT, 실기기, hardware GPU, iOS compositor 이후 표면 픽셀, 전체 CSS Grid와 C11.4/C11은 범위 밖이다.
