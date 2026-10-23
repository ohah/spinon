# C11.4 percentage auto-repeat neighbor · 구현 상태 대조

**기준:** 2026-10-24 · checkout `3a2ee376997878dfe12d843bf85c38901a704d83` · branch `plan/c11-4-auto-repeat-percentage-neighbors-correction-20261024`
**범위:** 로컬 코드 후보와 수정 중인 수용 기준, 공식 상태·공개 로드맵을 대조한다. 구현 완료나 Android/iOS 검증 보고서가 아니다.

## 코드 상태

- `crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs`의 미병합 후보는 finite nonnegative `LengthPx`와 `Percentage`를 auto-repeat 이웃 입력으로 허용한다.
- `crates/spinon-layout/src/grid.rs`는 percentage 이웃을 해당 축의 definite positive content basis가 있을 때만 허용하고 resolved 값의 유한·비음수 조건을 확인한다.
- 이미 실행한 focused Rust 결과에서 `percentage_neighbors_reject_indefinite_non_finite_and_negative_inputs`는 통과했다. `definite_percentage_neighbors_match_the_fixed_thirteen_scene_matrix`는 테스트 기대값이 Chrome 156 CSSOM의 `col-fit-first=196px`인 상태로 실패했고 vendored Taffy 0.15.0은 200px을 반환했다. CSS Grid 2025 CRD §7.2.3.2의 독립 규범 계산도 빈 auto-fit gutter collapse에 따라 200px이다. 계획 보정은 이 규범값과 Chrome 관측을 분리하도록 정했지만, 테스트 기대값·validator의 교정과 그 재실행은 아직 끝나지 않았다.
- checkout HEAD는 `origin/main`과 같다. 코드 변경은 local working-tree 후보이며 main에 병합되지 않았다. 새 runtime app fixture도 아직 없다.

## Chrome·규범·Taffy 대조

- Agent Browser Chrome 156.0.8078.48의 두 `*-fit-first` scene은 CSSOM track list `[24,196,0,0,0,12]`, gap 4px, 시작 정렬 2×2 item frame columns `(38,10,2,2)` / rows `(10,38,2,2)`를 냈다.
- Browser DOM에서만 item을 끝 정렬한 별도 진단은 Chrome first track edge를 columns x=234px / rows y=234px로 관측하고 2px item frame을 x/y=232px에 놓았다. 독립 규범 계산은 200px track과 x/y=236px frame이다. 원본 observation은 [end-alignment probe JSON](./auto-fit-end-alignment-probe-2026-10-24.json)에 둔다.
- Earlier DOM-only suffix item probe의 x/y=238px은 gutter 수를 구분하지 못한다. 규범의 200px+한 gutter와 Chrome의 196px+두 gutter가 같은 위치를 만든다. 이전 자료에서 이 좌표를 “suffix 경계 gutter 관측”으로 해석한 표현을 정정했다.
- [CSS Grid 2025 CRD §7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat)는 빈 auto-fit track과 양쪽 gutter의 collapse를 규정한다. 독립 계산은 첫 반복 track을 `240 - 24 - 12 - 4 = 200px`으로 낸다. Vendored Taffy 0.15.0도 이 gutter 규칙을 적용한다.
- Chromium current `main`의 track collection 및 gap geometry source는 collapsed range 처리 경로를 보여준다. Chrome 156 바이너리 source revision은 매핑하지 않았으므로 원인을 source로 단정하지 않는다. 자세한 링크와 한계는 [규범·Chromium·Taffy 대조](./normative-chromium-taffy-audit-2026-10-24.md)에 있다.
- 계획 보정본은 CSS Grid 규범 계산을 필수 수용값으로 두고 Chrome CSSOM을 별도 compatibility comparison으로 보존한다. 새 [계획 실패 관점 검토 20개](../../../../plan/c11-4-auto-repeat-percentage-neighbors-amendment-plan-review-20.md)를 남겼으며, corrected runtime matrix는 아직 실행·완료 판정하지 않았다.

## 공식 상태와 다음 검증 범위

- C11.4는 미완료이고 공식 완료 수는 72/213이다. 현재 활성 lane은 C11.4다.
- C13 overflow·scroll 항목은 미완료지만 현재 활성 lane이 아니다. C13.2 shadow overflow는 fail-closed이고 public DOM `scroll` event/CSSOM은 없으며 OS-native wheel/trackpad은 부분 검증 상태다.
- Android API 37 AVD와 iOS Simulator에서 percentage-neighbor 후보를 실행하지 않았다. 이들 플랫폼은 corrected 13-scene runtime fixture가 준비된 뒤 별도로 검증한다. C11.4 checkbox와 공식 집계는 유지한다.
