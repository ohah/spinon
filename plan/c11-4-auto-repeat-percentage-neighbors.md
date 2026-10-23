# C11.4 · definite percentage auto-repeat 이웃 track 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [fixed-px 이웃 track slice](../spec/internal/evidence/c11-4-auto-repeat-neighbors-2026-10-20/README.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 계획 PR #577과 제한 구현 PR #580이 Rebase and merge됐다. 구현/runtime 실패 관점 20개와 별도 PR diff 실패 관점 30개 검토, Android/iOS 13-scene runtime matrix가 통과했다. PR #580 merge SHA는 `33b00d077147ad9f2102af3220daa2c6cf74e2f9`다. 병합 당시 required checks는 설정되지 않았고 GitHub는 check을 보고하지 않았다. C11.4 상위 체크는 미완료이며 공식 완료 수 72/213을 유지한다.

## 병합된 제한 구현과 남은 결손

PR #580 이전에는 repeat 앞·뒤의 explicit neighbor를 유한하고 0 이상인 px track으로만 받았다. Cascade와 layout은 percentage neighbor를 거부했다. Taffy 0.15.0의 repeat-count 계산이 다른 definite track의 used space를 해당 축의 definite content size로 계산하므로, percentage basis와 overflow·gap 경계가 수용 조건이었다.

CSS Grid Layout Module Level 1, 2025 CRD §7.2.3.2는 definite preferred/max size가 있으면 content box를 넘지 않는 가장 큰 양의 repeat count를 선택하고, repeat fragment 밖의 track과 gutter도 그 계산에 포함한다. §7.2.1은 definite grid axis에서 percentage track sizing을 해당 축의 grid container content size로 해석한다. PR #580은 이 기준에 맞춰 definite positive content axis에서만 유한·0 이상 percentage neighbor를 제한 profile에 허용하고, percentage 합산 및 basis 곱셈 overflow는 거부한다. auto-fill/auto-fit percentage minimum/maximum, actual track sizing 확장, implicit placement와 일반 author CSS admission은 이 slice에 포함하지 않는다.

## 사전 비교와 독립 기대값

[Chrome 156 Agent Browser 사전 비교](../spec/internal/evidence/c11-4-auto-repeat-percentage-neighbors-precomparison-2026-10-24/README.md)는 13개 고정 scene의 computed track 목록과 item frame을 보존한다. 기본 content axis는 240px이고 gap은 4px다. 10% prefix와 5% suffix는 24px·12px이므로 minmax(40px, 1fr)의 auto-fill 반복 수는 4이고 used tracks는 [24,46,46,46,46,12]다. 한쪽 12.5% neighbor의 30px used size는 네 repeat track을 남기며, definite 136px border-box는 padding/border를 뺀 116px content basis를 사용한다. Auto-fit은 네 번 반복한 뒤 placement에 따라 빈 repeat track과 그 gutter를 collapse한다. 80% + 30% neighbor 합계가 content axis보다 큰 경우에는 최소 1회 반복을 유지한다.

## 수용 oracle과 Chrome 차이 판정

이 slice의 필수 수용값은 CSS Grid Layout Module Level 1, 2025 CRD §7.2.1·§7.2.3.2에서 독립 계산한다. Chrome 156은 같은 입력을 관측하는 호환성 비교값으로 보존하되, 규범값과 다를 때 Chrome 값을 규범 기대값으로 덮어쓰지 않는다. 차이는 browser compatibility 차이로 명시하고 runtime 결과는 규범값과 별도로 보고한다.

`col-fit-first`와 `row-fit-first`는 240px content axis, 24px prefix, 12px suffix, 4px gap, 네 번 반복을 사용한다. 첫 반복 track에만 item을 놓으면 나머지 세 auto-fit track이 collapse한다. collapse된 track 양쪽 gutter도 없어지므로 prefix와 첫 반복 track 사이의 gutter 하나만 남고 첫 반복 track은 `240 - 24 - 12 - 4 = 200px`이다. Chrome 156은 같은 고정 CSS에서 `196px`을 관측했다. 2px item을 해당 track 끝에 정렬한 DOM-only 진단에서는 Chrome이 x/y `232px`, 규범 계산이 x/y `236px`이다. 이는 4px의 Chrome 관측 차이며 원인은 확정하지 않는다. suffix track item의 x/y `238px` 관측만으로는 gutter 차이를 구분할 수 없다. 규범 결과의 한 gutter/200px과 Chrome 관측의 두 gutter/196px이 같은 suffix 시작 좌표를 내기 때문이다.

사전 비교 자료에는 독립 계산, Chrome CSSOM 관측, end-alignment 진단을 별도 기록한다. Chromium `main` 소스는 collapsed range를 gap geometry에서 건너뛰는 경로를 보여주지만 Chrome 156 바이너리의 정확한 source revision을 연결하지 않았으므로 구현 원인 증거로 일반화하지 않는다. Chrome 관측의 세부와 한계는 [규범·Chromium·Taffy 대조](../spec/internal/evidence/c11-4-auto-repeat-percentage-neighbors-precomparison-2026-10-24/normative-chromium-taffy-audit-2026-10-24.md)에 둔다.

Acceptance는 scene별 repeat count와 expanded line/track order를 exact로 비교하고, independently calculated used track size와 runtime item frame을 1/64 CSS px 안에서 판정한다. Chrome CSSOM 값은 raw browser observation과 exact track order로 별도 검증한다. 규범값과 Chrome이 다르면 discrepancy를 남기며 두 오라클을 하나의 pass 조건으로 합치지 않는다. Count만 맞거나 frame만 맞으면 실패다.

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
| col-fit-first | 양쪽 neighbor, 첫 repeat에 item 배치; count 4, 규범값 [24,200,0,0,0,12], Chrome 관측 [24,196,0,0,0,12] |
| row-fit-first | rows 축에서 첫 repeat에 item 배치; 규범값 [24,200,0,0,0,12], Chrome 관측 [24,196,0,0,0,12] |
| col-fit-empty | 양쪽 neighbor, item 없음; count 4, [24,0,0,0,0,12] |
| row-fit-empty | rows 축, item 없음; 같은 track 결과 |
| col-fill-overflow | prefix 80%, suffix 30%; count 1, [192,40,72] |
| row-fill-overflow | rows 축에서 overflowing neighbor control; 같은 track 결과 |
| col-fill-zero-prefix | prefix 0%, suffix 5%; count 5, [0,40.8,40.8,40.8,40.8,40.8,12] |
| col-fill-border-box | 136px border-box, 10%/5% neighbors, content basis 116px; count 2, Chrome used values [11.59375,43.296875,43.296875,5.796875] |
| row-fill-border-box | 같은 border-box 계산을 rows 축에서 확인 |

Runtime fixture는 V8-authored HostDocument → Stylo computed style → typed layout input → Taffy → WGPU scene/draw/present 경로로 13개 scene을 실행한다. Chrome CSSOM track list와 독립 산술을 scene별로 대조한다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 최신 앱 빌드의 track trace, item frame과 draw/present를 확인한다. PixelCopy와 제출 전 Metal 표본은 선언한 표본 위치에서만 비교하며, 이는 실기기·hardware GPU·iOS compositor 이후 픽셀 일치 근거가 아니다.

## PR #580 병합 구현 slice · 2026-10-24

PR #580에 병합된 cascade/layout 변경은 definite positive content axis에서 finite·nonnegative percentage neighbor를 받도록 연결됐다. 규범 `col-fit-first` 200px과 Chrome 156 CSSOM 관측 196px은 별도 oracle이다. `spinon-layout` percentage-neighbor 3/3, `spinon-style` auto-repeat 16/16, FFI fixture test 1/1 및 전체 workspace tests가 통과했다. workspace Clippy와 rustfmt도 통과했다.

13개 scene JSON과 opt-in FFI fixture adapter를 추가했고, Android/iOS demo runner의 scene allowlist·frame·pixel expectations 및 simulator capture/boundary route를 연결했다. [FFI adapter](../crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_auto_repeat_percentage_neighbors.rs), [scene matrix](../crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_percentage_neighbor_scenes.json), [Android runner](../platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java), [iOS runner](../platforms/ios/Sources/C0410RuntimeGpuDemo.swift), [capture tool](../tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs), [fixture boundary validator](../tools/css-reference/fixture-module-boundary.test.mjs)에 있다. API 37 ARM64 Android AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 13/13 scene이 각각 통과했다. Android PixelCopy와 compositor 제출 전 iOS Metal drawable은 각각 15/15 표본 exact이며, 전체 런·hash·제외 범위는 [runtime 근거](../spec/internal/evidence/c11-4-auto-repeat-percentage-neighbors-runtime-2026-10-24/README.md)에 기록했다. [구현/runtime 실패 관점 20개 검토](../spec/internal/evidence/c11-4-auto-repeat-percentage-neighbors-runtime-2026-10-24/implementation-runtime-review-20.md)를 마쳤다. 독립 PR diff 실패 관점 30개 검토와 PR #580 병합을 마쳤다. Tailnet 공개 미리보기 동기화는 별도 문서 PR 병합 뒤 진행한다.

## 검증 관문

1. 계획 PR #577은 병합됐다. 현재 구현 후보의 출처 revision과 남은 검증을 기록하고, 기존 미추적 자료와 tmux 세션을 보존한다.
2. Cascade is_auto_repeat_neighbor_track, layout validate_auto_repeat_axis와 percentage-basis 생성자를 추적해 두 축의 gate가 일치하는지 확인한다.
3. W3C 규칙으로 count, collapsed gutter 수와 used-size 기대값을 독립 계산하고 Chrome 156 Agent Browser에서 13개 고정 scene 및 별도 end-alignment 진단을 수집한다. 규범값·browser 관측·차이를 분리한 fixture, browser metadata, 관찰 JSON, screenshot과 SHA-256을 보존한다.
4. focused Rust/style/layout tests, 전체 workspace tests, Clippy, rustfmt, CSS fixture validator와 default/fixture-test artifact 경계를 검사한다.
5. Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 13/13 scene을 실행하고 해당 run의 source/build hash를 연결한다.
6. 이 계획 보정은 [별도 실패 관점 20개 검토](./c11-4-auto-repeat-percentage-neighbors-amendment-plan-review-20.md)를 통과했다. 구현/runtime은 [새 실패 관점 20개 검토](../spec/internal/evidence/c11-4-auto-repeat-percentage-neighbors-runtime-2026-10-24/implementation-runtime-review-20.md), PR 변경은 이 두 검토와 별도의 [실패 관점 30개 검토](../spec/internal/evidence/c11-4-auto-repeat-percentage-neighbors-runtime-2026-10-24/pr-diff-review-30.md)를 통과했다.
7. 상태 대장·내부 계약·상위 C11.4 계획에 제한 범위와 근거를 연결하되 C11.4 checkbox와 72/213은 유지한다.
8. Tailnet 미리보기를 선택한 문서만 backup + hash manifest 방식으로 갱신하고 Agent Browser로 실제 화면·이미지를 확인한다. PR 본문에 화면을 넣은 경우 GitHub 표시도 확인한다. 종료 후 이 작업의 Agent Browser session/process를 닫는다.
9. 구현 PR [#580](https://github.com/ohah/spinon/pull/580)을 Rebase and merge로 병합했다. 병합 직전 `mergeStateStatus=CLEAN`, required check 미설정, check 미보고를 재확인했다. 이 문서의 merge SHA는 `33b00d077147ad9f2102af3220daa2c6cf74e2f9`다. 병합 후 Tailnet 미리보기는 문서 PR이 병합된 뒤 sync한다.

## 제외 범위

Indefinite/cyclic percentage basis, negative/non-finite percentages, percentage calc neighbors, intrinsic/flexible neighbors, multiple auto-repeat, unsupported integer-repeat combinations, generic CSS admission, WPT 전체, 실기기와 hardware GPU 검증은 이 slice에 포함하지 않는다. 이 계획이나 후속 제한 fixture는 전체 CSS Grid 또는 C11.4 완료 판정이 아니다.
