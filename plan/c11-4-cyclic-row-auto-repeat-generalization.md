# C11.4 · cyclic percentage auto-repeat 행 축 fixture profile 일반화

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**선행 구현:** [max-content 행 축 cyclic auto-repeat PR #632](./c11-4-cyclic-percentage-auto-repeat-row-axis.md)

**사전 비교:** [규범 기대값·Chrome 156 관측·화면](../spec/internal/evidence/c11-4-cyclic-row-auto-repeat-generalization-precomparison-2026-10-26/README.md)

**계획 범위:** PR #632의 exact two-scene row profile을 fixed prefix/suffix, nonzero row-gap, `auto-fill`/`auto-fit`, 겹치는 direct leaf item을 가진 4-scene row profile까지 넓힌다. CSS author 입력을 일반 허용하지 않는다. 행 재확장, 열 동작 변경, C11.4 또는 공식 완료 수 승격은 이 계획에 포함하지 않는다.

## 목적과 규범 판정

현재 `fixture_cyclic_percentage_auto_repeat_scene`는 행 축에서 한 `minmax(0px, 50%)` track, `auto-fill`, zero gap, child 하나만 받는다. Taffy는 layout 시작 시 계산한 `row_auto_repetition_count`를 percentage row sizing rerun에 다시 전달해 기존 repeated row count를 유지한다. 이 동작은 다음 target에서 그대로 유지한다. 이번 변경의 대상은 고정 장면 모양의 gate와 그 runtime matrix다.

규범 기준은 [CSS Grid Layout Module Level 1, 2025 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes), [§7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat), [§11](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-overview), [§11.1 gutter 처리](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#gutters), [CSS Sizing Level 3 §2](https://www.w3.org/TR/css-sizing-3/#definite)다. `height:max-content` target의 첫 block-axis auto-repeat count는 1이다. Cyclic 25% maximum은 intrinsic sizing 중 `auto`처럼 취급된다. Prefix/suffix 8px, repeated-row item의 최대 기여 24px, row-gap 두 개 2px을 더하면 intrinsic/final content-height basis는 `8 + 24 + 8 + 4 = 44px`이다. 최종 percentage maximum은 11px이며 repeat count는 `1 → 1`로 유지된다. `auto-fit` 반복 행은 item이 차지하므로 collapse하지 않는다. Target의 최종 track과 gutter 합은 `8 + 2 + 11 + 2 + 8 = 31px`이고 root content height는 44px이다. `align-content:start`이므로 나머지 13px은 suffix 뒤에 남으며 두 max-content scene 모두 이 root 크기와 잔여 공간을 확인한다.

Definite `height:44px` control에서는 25% track이 11px이다. `16 + 2(n+1) + 11n ≤ 44`를 만족하는 최대 반복 수는 2다. `auto-fit`은 비어 있는 두 번째 반복 track과 그 뒤 gutter를 배치 후 collapse한다. CSS Grid §11.1에서는 collapsed run을 둘러싼 gutter가 한 shared boundary gap으로 겹친다. 이 장면에서는 첫 반복과 빈 두 번째 반복 사이의 2px gutter가 shared gap으로 남고 빈 track 뒤 gutter는 0px이 된다. 따라서 suffix row 시작점은 root 기준 `y=23px`이다. 독립 기대값·Chrome 원본 관측·좌표·재현 입력은 [사전 비교 자료](../spec/internal/evidence/c11-4-cyclic-row-auto-repeat-generalization-precomparison-2026-10-26/README.md)에 따로 둔다.

| Scene | Root sizing | Repeat count | 최종 rows | Root content box |
|---|---|---:|---|---|
| `cyclic-max-content-auto-fill-prefix-suffix` | `height:max-content` | `1 → 1` | `[8,11,8]px` | `24×44px` |
| `cyclic-max-content-auto-fit-prefix-suffix` | `height:max-content` | `1 → 1` | `[8,11,8]px` | `24×44px` |
| `definite-44px-auto-fill-prefix-suffix-control` | definite 44px | `2` | `[8,11,11,8]px` | `24×44px` |
| `definite-44px-auto-fit-prefix-suffix-control` | definite 44px | `2` | `[8,11,0,8]px`; repeat 2 collapsed | `24×44px` |

반복 행에는 24×24px와 16×16px item이 같은 row에 겹친다. 24px item이 sizing contribution 최댓값이며 두 item 크기를 합산하지 않는다. 11px used track을 기준으로 primary item은 13px, secondary item은 5px 넘친다. Browser fixture에는 fixed suffix row의 root-local 시작점을 보여주는 24×1px item도 있지만 runtime input에는 넣지 않는다.

앞선 row re-expansion 가설은 Chrome 156에서 반박됐다. 별도의 40px item·`50%`·prefix/suffix 없음·zero-gap fixture에서 두 rows `[20px,20px]`를 예상했지만 `[20px]` 하나만 관측됐다. 이는 기존 #632의 16px item target `[8px]`과는 다른 입력이다. 행 sizing rerun은 이미 결정된 row list와 count를 유지한다. #623 column fixture는 축 외에 percentage, fixed neighbors, gap, item input도 다르므로 이 행 결과만으로 열 축 동작을 판정하지 않는다. 축 간 단계 차이는 별도 규범 비교가 필요하다.

## 허용 profile과 범위 밖

- Opt-in `c11-4-grid-track-sizing-fixture-test`에서만 검사한다. Existing column cyclic profile은 별도 분기를 유지하고 수정하지 않는다.
- Row track list는 `[LengthPx(8), MinMax(LengthPx(0), Percentage(0.25)), LengthPx(8)]`, auto-repeat range `start=1,end=2` 순서다. 전체 row track list 길이는 3이다. Repeat mode는 `auto-fill` 또는 `auto-fit`이며 repeated track 한 개와 outer/inner line-name 없음만 허용한다.
- Row gap은 2px, column gap은 0px다. Root는 content-box `24×max-content` 또는 definite `24×44px`, zero margin/padding/border, start alignment다. CSS에서 미선언된 root `min-height`는 `LayoutStyle.min_height=Auto`여야 한다. Chrome CSSOM의 `0px` 직렬화값을 authored/fixed zero로 읽지 않는다. 다른 축은 fixed `24px` 한 track이다.
- Runtime input root는 node 1개와 direct child 3개를 가진다. Child 순서는 prefix, primary, secondary다. Prefix item은 border-box `24×8px`, `min-width/min-height:0`, `grid-row:1`, `grid-column:1`로 fixed prefix row를 점유한다. Primary와 secondary는 border-box 각각 `24×24px` 및 `16×16px`, `min-width/min-height:0`, `grid-row:2`, `grid-column:1`로 같은 repeated row를 비-spanning 배치한다. Items는 leaf, zero inset, zero margin/padding/border, static positioning이다. Typed input은 root `children.len()==3`, `grid_items.len()==3`, `positioning.len()==4`를 요구하고 나머지 root/item `LayoutStyle` fields도 allowlisted value와 정확히 비교한다. Browser-only suffix probe는 runtime node가 아니다.
- Target은 auto-repeat pattern을 한 번 유지한다. Definite control은 일반 auto-repeat layout 경로를 사용하며 cyclic opt-in profile success로 표시하지 않는다. Track trace가 필요한 control diagnostics는 별도 enable한다.
- 추가 repeat component, multiple cyclic axes, named lines, template areas, spans, auto placement, implicit row, nested/anonymous/replaced item, out-of-flow runtime child, 다른 track grammar·길이·percentage·gap, arbitrary CSS/HTML admission은 거부한다. 정확한 양의 runtime 입력은 위 네 scene뿐이다.
- 새 public API, CSS support claim, WPT pass, C11.4/C11 completion claim을 추가하지 않는다. 내부 계약 숫자 `0.1.0`과 공식 완료 수 `72/213`을 유지한다.

## 코드 소유 경계와 병렬 lane

계약·scene ID·expected frames/tracks·trace 필드가 이 계획 및 JSON에서 고정된 뒤 다음 비중첩 작업은 병렬 진행할 수 있다.

1. **Layout/Taffy core와 trace:** `crates/spinon-layout/src/grid.rs`, `vendor/taffy/src/compute/grid/mod.rs`. Row-only fixture gate를 위 profile까지 확장하고 prefix/suffix, gap, repeat mode, three-child placement를 검사한다. 기존 row repeat count를 그대로 전달하고 row re-expansion branch는 추가하지 않는다. 이 lane은 한 담당자가 순차 통합한다.
2. **Rust/layout/runtime acceptance:** `crates/spinon-layout/src/grid_track_sizing_tests/cyclic_percentage_auto_repeat_rows.rs`, `crates/spinon-runtime/src/session/ua_cascade/runtime_layout_tests/c11_grid_tests/cyclic_percentage_auto_repeat_rows.rs`. Trace pass별 axis, phase, repeat count, basis, ordered tracks, collapse/gutter flags와 frames를 검사한다. Mutation tests는 root/property 소유자가 기대한 지점에서 fail closed하는지 확인한다. Core trace 계약을 먼저 고정한 뒤 작업한다.
3. **FFI scene:** `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/cyclic_percentage_auto_repeat_rows.rs`에 네 scene을 추가한다. `fixtures/c11_4.rs` dispatcher와 `fixtures/c11_4/trace_report.rs`는 통합 담당자가 직렬로 변경한다.
4. **Android adapter:** `platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java`만 담당한다.
5. **iOS adapter:** `platforms/ios/Sources/C0410RuntimeGpuDemo.swift`만 담당한다.

Android와 iOS adapter는 서로 다른 파일을 소유하므로 1–3번에서 scene/report 계약을 고정한 뒤 두 작업자를 병렬 배정한다. 어느 adapter 작업자도 Rust core, shared dispatcher/report 또는 상대 플랫폼 파일을 수정하지 않는다. 병렬 변경은 통합 branch에서 시작하고 새 worktree를 만들지 않는다. Core/FFI 통합, 앱 실행, artifact boundary와 전체 검증은 두 adapter 결과를 합친 다음 한 번 수행한다.

## 검증 관문

1. 이 계획 변경을 20개의 서로 다른 실패 관점으로 먼저 검토한다. 계획 PR은 검토 수정 및 별도의 PR diff 검토를 마친 뒤 Rebase and merge한다. 계획 PR merge 전에는 implementation을 시작하지 않는다.
2. `spec-derived-expectations.json`과 Chrome observation을 분리해 유지한다. Fixture/evidence SHA-256, Chrome 버전·viewport·DPR와 screenshot을 기록한다.
3. Layout unit tests는 target의 row pass 두 개를 확인한다: `IntrinsicMaxContent` count 1, null percentage basis, row 순서 `[8,24,8]`; `FinalDefinite` count 1, 44px basis, rows `[8,11,8]`. Trace vector는 양끝 gutter, track 사이 gutter, CSS track을 원본 순서로 기록한다. Intrinsic contribution trace는 root grid node의 repeated-row leaf 두 개를 span 1, `MaxContent` 24px/16px으로 기록한다. 두 max-content scene의 root content box는 44px, suffix 시작은 root-local `y=23px`, suffix 뒤 잔여 공간은 13px이어야 한다. Root-relative 정수 CSS frame은 exact, 분수 track size/좌표는 최대 1/64 CSS px 허용오차를 쓴다. 24px/16px item은 11px track보다 각각 13px/5px 크다.
4. Definite control은 일반 경로에서 count 2, basis 44px을 사용한다. `auto-fill` rows는 `[8,11,11,8]`이고 suffix는 root-local `y=36px`에서 시작한다. `auto-fit` rows는 `[8,11,0,8]`, zero-based CSS track index 2가 collapsed이며 suffix는 `y=23px`에서 시작한다. `auto-fit` trace 순서는 edge gutter, prefix, prefix/repeat gutter, occupied repeat, 2px shared boundary gutter, collapsed empty repeat, 0px following gutter, suffix, trailing edge gutter다. 모든 `is_gutter`, `is_collapsed`, `base_size` 값을 확인한다. Track과 gutter 합은 31px이고 definite root는 44px이므로 13px의 남는 공간은 `align-content:start`에 따라 suffix 뒤에 둔다.
5. Fail-closed mutation은 typed layout gate와 runtime style projection을 분리해 검증한다. Typed-input mutation의 owner는 root node의 exact-scene validator이며 반환 오류는 `LayoutError::InvalidGridTrackList { property: "grid-template-rows" }`다. Runtime profile이 computed property를 표현하지 못하는 경우에는 projection에서 먼저 `StyleLayoutError::UnsupportedComputedValue { property }`로 거부한다. 다음 mutation마다 해당 owner와 오류를 확인한다.

| Mutation | 정확한 변형 | 기대 owner와 오류 |
|---|---|---|
| Repeat mode | 장면 ID는 유지하고 `auto-fill`과 `auto-fit`을 서로 바꾼 typed input | Root exact-scene validator / `InvalidGridTrackList(grid-template-rows)` |
| Repeat 범위 | `start=1,end=2`를 다른 index로 변경하거나 track list 길이를 3이 아니게 변경 | 같은 layout 오류 |
| Repeat 구성 | auto-repeat 주변에 추가 repeat component를 넣음 | 같은 layout 오류 |
| 고정 이웃 행 | prefix/suffix를 제거·이동하거나 8px 길이를 변경 | 같은 layout 오류 |
| Cyclic track | `minmax(0px,25%)`의 minimum 또는 percentage를 변경 | 같은 layout 오류 |
| Gap | row gap 2px 또는 column gap 0px을 변경 | 같은 layout 오류 |
| Line·area metadata | row named line을 추가하거나 `grid-template-areas`를 추가 | 같은 layout 오류 |
| Root style | root 크기, `min-height:Auto`, padding, border 또는 typed alignment를 변경 | 같은 layout 오류 |
| Runtime alignment projection | runtime UA profile에서 computed `align-content:start`를 다른 값으로 변경 | Style projection / `UnsupportedComputedValue(align-content)` |
| Child cardinality | prefix/primary/secondary 중 하나를 빼거나 direct child 또는 node를 추가 | Root exact-scene validator / 같은 layout 오류 |
| Child style | child 크기, min-size, margin, padding, border, inset 또는 position을 변경 | 같은 layout 오류 |
| Placement | row/column 시작선을 바꾸거나 span을 주어 implicit row를 요구 | 같은 layout 오류 |
| Browser-only probe | browser fixture의 suffix probe를 runtime node로 추가 | Root exact-scene validator / 같은 layout 오류 |

`min-height:0px`는 typed root의 `LayoutStyle.min_height=Fixed(0)` mutation으로 검사한다. CSS에서 미선언한 root 값은 `LayoutDimension::Auto`다. 기존 50% row와 column target은 회귀 control로 유지한다. 계획 자체의 검토 결과는 [계획 실패 관점 20개 검토 기록](../spec/internal/evidence/c11-4-cyclic-row-auto-repeat-generalization-plan-review-20-2026-10-26.md)에 별도로 남긴다.
6. Runtime matrix는 네 장면으로 `RuntimeSession` 요청 ID/source revision, root/grid/item frame, 양 축 tracks, pass별 phase/count/basis/track·gutter 순서, draw/present 응답을 검사한다. 두 앱은 같은 source commit으로 build하고 Android/iOS build·install hash는 각각 기록한다. Android API 37 ARM64 16 KB AVD와 iPhone 17 Pro/iOS 26.2 Simulator를 사용하며 실기기는 범위에 넣지 않는다.
7. Android PixelCopy와 iOS Simulator Metal drawable에서 root-local CSS pixel 중심을 표본화한다. Adapter는 캡처한 root origin과 실행 DPR을 적용해 정수 device 좌표로 변환하고 원시 RGBA를 기록한다. Runtime scene 색은 불투명 sRGB다: prefix `#2f9e44`, primary `#e5484d`, secondary `#f08c00`, root background `#101820`.

| Root-local sample (CSS px) | Expected color | Assertion |
|---|---|---|
| `(12.5,4.5)` | `#2f9e44` | fixed prefix item |
| `(8.5,15.5)` | `#f08c00` | later-painted overlapping secondary item |
| `(20.5,15.5)` | `#e5484d` | primary item outside the secondary width |
| `(20.5,33.5)` | `#e5484d` | primary item 12.5px past its 11px repeated-track end, still inside the 44px root |
| `(20.5,40.5)` | `#101820` | root background below the item |

   이 fixture에는 CSS border가 없다. 표본은 Grid track 밖으로 item이 넘치는 것과 paint order만 확인하며 border 획이나 root 경계 밖 overflow는 확인하지 않는다. Android hardware GPU 또는 iOS compositor 이후 pixel 결과도 입증하지 않는다. Simulator screenshot은 별도 시각 근거로 보존하고 PR 본문에는 직접 이미지 링크를 넣는다.
8. `cargo fmt --all -- --check`, focused tests, workspace tests, workspace all-target Clippy, Android/iOS Simulator builds, fixture module boundary, default/product artifact negative control 및 fixture-test positive control을 실행한다. 실행하지 않은 CI/build/platform 단계는 미검증으로 쓴다. Static archive 결과를 APK/IPA 전체의 fixture exclusion으로 확대 해석하지 않는다.
9. 구현/runtime 코드와 simulator 동작은 계획 표를 재사용하지 않은 20개 새 실패 관점으로 검토한다. 실제 implementation PR diff는 그와 별도 30개 실패 관점으로 검토한다. 발견을 고친 revision은 영향받은 관점을 다시 확인한다.
10. Implementation PR merge 뒤 실제 변경 근거를 `spec/internal/0059-c11-grid-layout.md`, `spec/STATUS.md`, [`spec/public/roadmap.html`](../spec/public/roadmap.html)과 실행 evidence에 반영한다. C11.4 checkbox와 공식 `72/213`은 유지한다. Tailnet preview는 기존 사용자가 정한 경로에서 실제 root/config와 활성 process를 확인하고 선택 파일만 backup 후 동기화한다. Local·served response hash와 HTTP status를 대조하고 Agent Browser로 roadmap/status/evidence와 platform screenshots를 확인한 뒤 그 run의 Agent Browser session과 process를 종료한다. Terminal Browser는 사용하지 않는다.

## 계획 실패 관점 20개

아래 질문은 계획 자체를 검토하는 항목이다. 구현 후 코드/runtime review 및 PR diff review에서 재사용하지 않는다.

1. Max-content block size를 auto-repeat 계산 때 definite로 잘못 분류하지 않는가?
2. Intrinsic pass의 cyclic percentage `auto` 처리와 final 44px basis resolution을 구분했는가?
3. Target의 final row count가 `1 → 1`이며 row re-expansion을 제외했는가?
4. 반복 행에 겹친 24px/16px item contribution의 최댓값 24px을 쓰며 합 40px로 오산하지 않는가?
5. Root basis `44px = 8 + 24 + 8 + 2×2`를 browser output 역산 없이 도출했는가?
6. Final repeated track `11px = 25% × 44px`를 content box 기준으로 계산했는가?
7. Definite repeat count의 prefix/suffix와 candidate별 gutter 개수가 맞는가?
8. `auto-fill` definite control의 2회 반복 상한 부등식을 검산했는가?
9. `auto-fit`의 initial count는 유지하고 배치 뒤 빈 두 번째 repeat만 collapse하는가?
10. `auto-fit`에서 collapse되는 row와 양쪽 gutter를 trace로 구분하는가?
11. 점유된 `auto-fit` cyclic target 행을 비었다고 오판해 collapse하지 않는가?
12. CSSOM의 resolved `minHeight:0px`를 authored fixed zero로 잘못 해석하지 않는가?
13. Fixed prefix/suffix·gap typed input과 author track-list index가 auto-repeat 범위에 정확히 대응하는가?
14. Runtime child 세 개의 owner/order/placement가 fixture input과 맞고 browser-only marker는 제외되는가?
15. 지원하지 않는 profile mutation을 downstream Taffy sizing 전에 올바른 node/property owner에서 거부하는가?
16. Definite control을 기존 일반 경로에서 처리하고 cyclic fixture 허용 목록 success로 오기록하지 않는가?
17. 기존 column profile과 repeat 재계산 경로가 row-only gate 변경에 영향받지 않는가?
18. Core, runtime, FFI shared report와 각 platform lane의 파일 소유가 겹치지 않는가?
19. Screenshot, CSSOM/DOMRect, expected JSON, source/build hash를 구별해 보존하는가?
20. 네 고정 scene 결과를 일반 CSS Grid, WPT 적합성, 실기기/hardware GPU 또는 C11.4 전체 완료 주장으로 확대하지 않는가?
