# C11.4 · definite percentage auto-repeat 이웃 track 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [fixed-px 이웃 track slice](../spec/internal/evidence/c11-4-auto-repeat-neighbors-2026-10-20/README.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 계획 · 구현 결과가 아니다. C11.4 상위 체크와 공식 완료 수 72/213은 유지한다.

## 결손과 목표

현재 auto-repeat 입력은 repeat 앞·뒤의 explicit neighbor를 유한하고 0 이상인 px track으로만 받는다. Cascade의 is_auto_repeat_neighbor_track와 layout의 auto_repeat_neighbor_track_is_fixed_px가 percentage neighbor를 거부한다. Taffy 0.15.0의 repeat-count 계산은 다른 definite track의 used space를 해당 축의 definite content size로 계산하므로, 거부 조건을 지우기 전에 percentage basis와 overflow·gap 경계를 확인해야 한다.

CSS Grid Layout Module Level 1, 2025 CRD §7.2.3.2는 definite preferred/max size가 있으면 content box를 넘지 않는 가장 큰 양의 repeat count를 선택하고, repeat fragment 밖의 track과 gutter도 그 계산에 포함한다. §7.2.1은 definite grid axis에서 percentage track sizing을 해당 축의 grid container content size로 해석한다. 이 계획은 해당 축이 definite이고 양수인 제한 RuntimeBlockFlexGridTrackSizingV1 profile에 finite nonnegative percentage neighbor를 추가한다. auto-fill/auto-fit의 percentage minimum/maximum 처리, 실제 track sizing, implicit placement 및 일반 author CSS admission은 바꾸지 않는다.

## 사전 비교와 독립 기대값

[Chrome 156 Agent Browser 사전 비교](../spec/internal/evidence/c11-4-auto-repeat-percentage-neighbors-precomparison-2026-10-24/README.md)는 13개 고정 scene의 computed track 목록과 item frame을 보존한다. 기본 content axis는 240px이고 gap은 4px다. 10% prefix와 5% suffix는 24px·12px이므로 minmax(40px, 1fr)의 auto-fill 반복 수는 4이고 used tracks는 [24,46,46,46,46,12]다. 한쪽 12.5% neighbor의 30px used size는 네 repeat track을 남기며, definite 136px border-box는 padding/border를 뺀 116px content basis를 사용한다. Auto-fit은 네 번 반복한 뒤 placement에 따라 빈 repeat track과 그 gutter를 collapse한다. 80% + 30% neighbor 합계가 content axis보다 큰 경우에는 최소 1회 반복을 유지한다.

Acceptance는 repeat count와 expanded line/track order를 exact로 비교하고, used track size와 item frame은 Chrome 값에서 1/64 CSS px 안에 들어야 한다. Count만 맞거나 frame만 맞으면 실패다.

## 구현 범위와 제한

- repeat fragment 양쪽의 유한·0 이상 percentage GridTrackSize를 definite percentage basis가 있는 경우에만 허용한다. finite nonnegative 검증과 해당 axis의 definite-size gate를 유지한다.
- columns와 rows를 각각 검사한다. percentage basis는 border-box가 아니라 양쪽 padding/border를 뺀 해당 축의 content size다.
- fixed px neighbor 회귀를 유지하고, zero-percent neighbor는 0px used size 및 track/gap 위치로 검사한다.
- positive percentages, 양쪽 neighbor, prefix-only·suffix-only, gap, auto-fill/auto-fit, border-box, overflowing neighbors를 고정한다.
- negative/non-finite percentage, indefinite/cyclic axis, intrinsic/fraction/calc neighbor 및 지원되지 않는 track component는 기존 typed failure로 닫는다. 일반 repeat() grammar와 CSS admission을 넓히지 않는다.
- Taffy의 count-only sizing과 runtime item/frame trace를 대조한다. Taffy가 이미 definite percentage neighbor를 산정하는 경로가 확인되지 않으면 별도 dependency 변경을 계획 검토 뒤에 추가한다.

## 13-scene 고정 matrix

| Scene | 입력과 기대 결과 |
|---|---|
| col-fill-both | 240px columns, prefix 10%, suffix 5%, gap 4px, 4회; [24,46,46,46,46,12] |
| row-fill-both | 같은 입력을 240px rows에 적용; [24,46,46,46,46,12] |
| col-fill-prefix | prefix 12.5%, suffix 없음; 4회, [30,48.5,48.5,48.5,48.5] |
| row-fill-suffix | prefix 없음, suffix 12.5%; 4회, [48.5,48.5,48.5,48.5,30] |
| col-fit-first | 양쪽 neighbor, 첫 repeat에 item 배치; count 4, [24,196,0,0,0,12] |
| row-fit-first | rows 축에서 첫 repeat에 item 배치; 같은 track 결과 |
| col-fit-empty | 양쪽 neighbor, item 없음; count 4, [24,0,0,0,0,12] |
| row-fit-empty | rows 축, item 없음; 같은 track 결과 |
| col-fill-overflow | prefix 80%, suffix 30%; count 1, [192,40,72] |
| row-fill-overflow | rows 축에서 overflowing neighbor control; 같은 track 결과 |
| col-fill-zero-prefix | prefix 0%, suffix 5%; count 5, [0,40.8,40.8,40.8,40.8,40.8,12] |
| col-fill-border-box | 136px border-box, 10%/5% neighbors, content basis 116px; count 2, Chrome used values [11.59375,43.296875,43.296875,5.796875] |
| row-fill-border-box | 같은 border-box 계산을 rows 축에서 확인 |

Runtime fixture는 V8-authored HostDocument → Stylo computed style → typed layout input → Taffy → WGPU scene/draw/present 경로로 13개 scene을 실행한다. Chrome CSSOM track list와 독립 산술을 scene별로 대조한다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 최신 앱 빌드의 track trace, item frame과 draw/present를 확인한다. PixelCopy와 제출 전 Metal 표본은 선언한 표본 위치에서만 비교하며, 이는 실기기·hardware GPU·iOS compositor 이후 픽셀 일치 근거가 아니다.

## 검증 관문

1. 계획 PR 병합 뒤 최신 main에서 구현 branch를 만든다. 기존 미추적 자료와 tmux 세션은 보존한다.
2. Cascade is_auto_repeat_neighbor_track, layout validate_auto_repeat_axis와 percentage-basis 생성자를 추적해 두 축의 gate가 일치하는지 확인한다.
3. W3C 규칙에서 count와 used-size 기대값을 독립 계산하고 Chrome 156 Agent Browser에서 13개 고정 scene을 다시 수집한다. fixture, browser metadata, 관찰 JSON, screenshot과 SHA-256을 보존한다.
4. focused Rust/style/layout tests, 전체 workspace tests, Clippy, rustfmt, CSS fixture validator와 default/fixture-test artifact 경계를 검사한다.
5. Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 13/13 scene을 실행하고 해당 run의 source/build hash를 연결한다.
6. 구현/runtime의 새 실패 관점 20개를 별도로 검토한다. PR diff review는 그 기록과 독립된 실패 관점 30개로 진행하며 발견한 결함을 고친 뒤 영향받은 항목을 재검토한다.
7. 상태 대장·내부 계약·상위 C11.4 계획에 제한 범위와 근거를 연결하되 C11.4 checkbox와 72/213은 유지한다.
8. Tailnet 미리보기를 선택한 문서만 backup + hash manifest 방식으로 갱신하고 Agent Browser로 실제 화면·이미지를 확인한다. PR 본문에 화면을 넣은 경우 GitHub 표시도 확인한다. 종료 후 이 작업의 Agent Browser session/process를 닫는다.
9. PR은 템플릿·작업 유형/영역 라벨·명세·남은 범위를 채우고 Rebase and merge를 사용한다. 병합 직전 live required checks와 merge state를 확인한다.

## 제외 범위

Indefinite/cyclic percentage basis, negative/non-finite percentages, percentage calc neighbors, intrinsic/flexible neighbors, multiple auto-repeat, unsupported integer-repeat combinations, generic CSS admission, WPT 전체, 실기기와 hardware GPU 검증은 이 slice에 포함하지 않는다. 이 계획이나 후속 제한 fixture는 전체 CSS Grid 또는 C11.4 완료 판정이 아니다.
