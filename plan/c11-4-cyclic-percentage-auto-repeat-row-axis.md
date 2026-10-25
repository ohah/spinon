# C11.4 · max-content 행 축의 cyclic percentage auto-repeat 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**선행 구현:** [열 축 cyclic percentage auto-repeat PR #623](./c11-4-cyclic-percentage-auto-repeat.md) · [사전 비교](../spec/internal/evidence/c11-4-cyclic-row-percentage-auto-repeat-precomparison-2026-10-25/README.md)

**상태 · 2026-10-25:** 계획 PR #631이 병합되어 구현 중이다. target/control Rust·runtime·FFI fixture, Android/iOS adapter와 API 37 ARM64 AVD·iPhone 17 Pro iOS 26.2 Simulator 실행을 추가했다. 구현/PR 적대 검토와 증거 정리는 진행 중이며, 구현 PR 병합 전까지 C11.4 상태와 공식 완료 수 `72/213`은 그대로 둔다.

## 목적과 범위

PR #623은 fixed scene의 열 축에서 intrinsic max-content 계산 뒤 cyclic percentage track을 최종 layout하는 opt-in 경로를 추가했다. 해당 계획은 cyclic row axis를 명시적으로 제외한다. 이 후속은 그중 `height: max-content`인 한 row-axis scene과 한 definite-height control만 layout admission, semantic trace, 앱 runtime fixture로 확인한다.

사전 가설 `[8px, 8px]`은 Chrome과 독립 CSS Grid 계산에 의해 반박됐다. 대상의 auto-repeat count는 1회로 유지되어야 한다. percentage를 최종 16px content-height basis로 풀면 유일한 row의 used size는 8px이며, 고정 높이 16px item은 row track을 block 축으로 8px 넘는다. 이 slice에는 row auto-repeat count 재계산·재확장 알고리즘을 추가하지 않는다.

## 독립 기대값과 사전 관측

기준은 [CSS Grid Layout Module Level 1, 2025 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes), [§7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat), [§11](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-overview), [CSS Sizing Level 3 §2](https://www.w3.org/TR/css-sizing-3/#definite)다. Browser observation은 이 계산과 별도로 저장하며 expected value를 만들 때 사용하지 않는다.

| Scene | 대상 sizing | 반복 수 | 최종 rows | Root content box | Item frame |
|---|---|---:|---|---|---|
| `cyclic-max-content-auto-fill-rows` | `height:max-content`, item 24×16px | intrinsic count 1, 최종에도 1 | `[8px]` | 24×16px | `(0,0) 24×16px`; track block extent보다 8px 큼 |
| `definite-40px-auto-fill-control-rows` | definite `height:40px` | 2 | `[20px,20px]` | 24×40px | `(0,0) 24×16px` |

정확한 입력은 [CSS fixture](../tests/fixtures/css/c11/grid-cyclic-row-percentage-auto-repeat-c114-precomparison.html), Chrome 156 원본 CSSOM/DOMRect와 화면, 독립 계산, source support audit는 [사전 비교 evidence](../spec/internal/evidence/c11-4-cyclic-row-percentage-auto-repeat-precomparison-2026-10-25/README.md)에 고정했다. Chrome 관측은 기대값과 일치한다. Spinon runtime 출력은 아직 관측하지 않았다.

## 구현 진입 조건 및 제한

- Layout opt-in exact-scene validator는 cyclic target만 허용하며 일반 CSS/HTML admission을 넓히지 않는다. Target의 typed contract는 root `display:grid`, `box-sizing:content-box`, content width 24px, height `max-content`, min-height `auto`(fixture에서 선언하지 않은 CSS initial value이며 `LayoutStyle::Auto`), max-height none, columns `[24px]`, rows `[minmax(0px,50%)]`, auto-repeat `{mode:auto-fill,start:0,end:1,line_names:[]}`, row/column gap 0, margin/padding/border 0, `align-content:start`, `justify-content:start`, `grid-auto-flow:row`, auto rows/columns, template areas 없음, `TrackSizing` placement으로 고정한다. Chrome CSSOM의 resolved `minHeight:0px`는 raw observation으로 남기지만 authored/typed fixed zero로 바꾸지 않는다. Root `LayoutStyle`과 `GridContainerInput`의 나머지 typed/default 필드도 이 scene의 명시된 expected value와 완전히 같아야 하며 추가 explicit/implicit tracks나 line names를 허용하지 않는다. 직접 in-flow item은 정확히 하나이며 `display:block`, 24×16px, `min-width/min-height:0`, row/column 1 placement, margin/padding/border 0, static position, 자식 없음으로 고정한다. Item `LayoutStyle`도 expected typed value와 완전히 같아야 한다. Browser fixture의 absolute marker는 runtime node가 아니다. 이 값들과 root/item child 관계를 정확히 검사하고 변형은 fail-closed한다.
- Definite 40px control은 기존 일반 definite-axis admission 경로로 검증한다. Opt-in 진단이 필요하면 진단/trace 활성화만 별도로 적용하고 control이 cyclic fixture allowlist를 통과했다는 식으로 기록하지 않는다.
- Existing Taffy indefinite-height percentage-row rerun을 재사용한다. Target의 row repeat count는 양 sizing 단계에서 1이어야 하며, definite 16px percentage basis로 재실행한 결과는 `[8px]`이어야 한다. Count가 2로 바뀌면 acceptance 실패다.
- `GridTrackSizingFixturePhase::IntrinsicMaxContent`는 max-content intrinsic pass, `GridTrackSizingFixturePhase::FinalDefinite`는 현재 pass 축의 content-box basis가 정해진 이후의 pass를 뜻한다. Taffy 내부 `AbstractAxis::Block`은 runtime/FFI typed trace의 `LayoutGridTrackSizingAxis::Rows`로 변환되므로 acceptance는 `axis=Rows`를 요구한다. 각 pass record에서 axis, semantic phase, auto-repeat count, percentage basis와 track snapshot을 같은 provenance로 검증한다. `percentageBasis`는 `inner_node_size.get(axis)`와 일치해야 하며, exact target의 첫 pass는 `null`, 재실행은 16px이다. Root min-height의 typed value가 `Auto`이므로 `axis_min_size` fallback도 없고, trace의 기존 `percentageBasis` 값은 null→16px로 충분히 구분된다. Target count는 `1 → 1`, final rows `[8px]`이다. Definite control의 일반 layout 결과는 count 2, percentage basis 40px, rows `[20px,20px]`다. Sequence 순서만으로 phase를 추론하지 않는다. `FinalDefinite` 의미는 특정 width 축에 한정하지 않고 현재 pass의 row/column content-box basis를 나타낸다.
- CSS fixture의 absolute half-height marker는 browser용 시각 관측 보조이며 runtime sizing item에 포함하지 않는다.
- 구현/runtime 실패 관점 20개와 PR diff 실패 관점 30개는 계획 검토 표와 분리한다. 수정된 revision에 영향받은 관점은 재검토한다.
- C11.4 checkbox와 공식 완료 수 `72/213`은 유지한다. 이 제한 scene은 일반 author CSS 지원, WPT 적합성 또는 전체 Grid track sizing 완료를 뜻하지 않는다.

## 소유 경계와 병렬 작업

다음 경로는 서로 다른 작성자가 병렬 작업할 수 있다. 공유 dispatcher 및 실행 목록은 통합 담당자가 순서대로 편집하며 두 작업자가 동시에 고치지 않는다.

1. **Layout/Taffy 핵심:** `crates/spinon-layout/src/grid.rs`, `vendor/taffy/src/compute/grid/mod.rs`, 필요한 opt-in pass context 연결. 일반 row rerun을 재사용하고 row re-expansion은 넣지 않는다.
2. **Rust·runtime 검증:** `crates/spinon-layout/src/grid_track_sizing_tests/`의 새 focused test와 `crates/spinon-runtime/src/session/ua_cascade/runtime_layout_tests/c11_grid_tests/`의 새 acceptance module. 기대 tracks, frames, phase/count/basis와 fail-closed mutation controls를 검증한다.
3. **FFI scene와 공용 report:** 새 `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/cyclic_percentage_auto_repeat_rows.rs`; shared `fixtures/c11_4.rs` dispatcher와 `trace_report.rs`는 이 통합 담당자만 편집한다.
4. **플랫폼 adapter:** Android `C0410RuntimeGpuDemo.java`와 iOS `C0410RuntimeGpuDemo.swift`는 서로 다른 소유 경로로 병렬 편집할 수 있다. scene 이름, frames, expected row tracks는 이 계획 표를 공통 계약으로 사용한다.

병렬 lane은 비중첩 파일 ownership으로 같은 통합 branch에서 진행하고 다른 미커밋·untracked evidence를 정리하거나 덮어쓰지 않는다. Lane 1의 trace 계약을 먼저 고정해 lane 2·3의 작성자가 같은 값과 phase 명칭을 사용하게 한다. Contributor가 별도 commit/branch를 독립 제출할 필요가 있을 때만 별도 worktree를 만든다. Shared dispatcher/report 병합, 양 플랫폼 실행, 전체 artifact 경계 확인은 병렬 편집을 끝낸 뒤 통합 branch에서 수행한다.

## 검증 관문

1. 계획 자체를 독립 실패 관점 20개로 검토하고, 발견 사항을 반영한 plan PR을 Rebase and merge한다. 구현은 계획 PR 병합 이후 시작한다.
2. Focused Rust 테스트에서 target count `1→1`, percentage basis `null→16`, definite control의 일반 경로 결과 count 2/percentage basis 40, row tracks `[8]`/`[20,20]`, root/item geometry와 같은 pass의 phase/axis/basis/track snapshot을 확인한다. Root `min-height`는 `LayoutDimension::Auto`로 투영되어야 한다. Chrome의 resolved CSSOM 값은 `0px`로 직렬화되지만 fixture gate는 이를 typed fixed zero로 받아들이지 않아야 한다. 일반 definite row auto-repeat과 기존 cyclic minmax/column target 회귀를 실행한다. Control은 일반 definite-axis validator를 통과해야 하며 opt-in trace는 diagnostics activation에서만 분리해 켠다.
3. Target의 fail-closed mutation matrix는 layout validator가 root node의 `LayoutError::InvalidGridTrackList { property: "grid-template-rows" }`를 반환하는지 확인한다.

| Mutation ID | 정확한 변형 | 실패 owner / 기대 결과 |
|---|---|---|
| `target-width` | root width 24px → 23px | layout exact-scene validator / `InvalidGridTrackList(grid-template-rows)` |
| `target-columns` | columns `[24px]` → `[23px]` | layout exact-scene validator / 같은 오류 |
| `target-repeat-mode` | `auto-fill` → `auto-fit` | layout exact-scene validator / 같은 오류 |
| `target-row-gap` | row gap 0px → 1px | layout exact-scene validator / 같은 오류 |
| `target-alignment` | `align-content:start` → `center` | Layout typed-input test: exact-scene validator / `InvalidGridTrackList(grid-template-rows)`. Runtime UA profile: style projection 단계의 `UnsupportedComputedValue(align-content)` |
| `target-grid-flow` | `grid-auto-flow:row` → `column` | layout exact-scene validator / 같은 오류 |
| `target-placement` | item row 1 → row 2 | layout exact-scene validator / 같은 오류 |
| `target-min-height` | item min-height 0px → 1px | layout exact-scene validator / 같은 오류 |
| `target-inset` | root top padding 0px → 1px | layout exact-scene validator / 같은 오류 |
| `target-extra-inflow-child` | 두 번째 static in-flow child 추가 | layout scene cardinality / 같은 오류 |
| `target-browser-marker-as-node` | absolute half-height marker를 runtime node로 추가 | layout scene cardinality/position gate / 같은 오류 |

`target-alignment`의 두 경로는 의도적으로 구분한다. Runtime track-sizing UA profile은 grid container의 `align-content:start`만 허용하므로 CSS mutation은 layout exact-scene validator보다 앞선 style projection에서 거부된다. Layout typed-input mutation은 exact-scene validator가 `grid-template-rows` owner로 실패하는지 직접 확인한다.
4. `cargo fmt --all -- --check`, focused tests, `cargo clippy --locked --workspace --all-targets -- -D warnings`, fixture module boundary 및 workspace relevant build를 실행한다.
5. Default/product artifact negative control과 `c11-4-grid-track-sizing-fixture-test` opt-in positive control에서 row scene symbol·문자열·payload의 포함 여부를 비교한다. 정적 library 결과를 APK/IPA 전체 제외로 확대 해석하지 않는다.
6. Android API 37 ARM64 16 KB AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 target/control 각 2 scene을 실행한다. Root/item frames, row tracks, pass별 count/basis, draw/present와 source/build/install hash를 별도로 기록한다. 실기기 GPU, compositor 이후 pixels, glyph paint는 주장하지 않는다.
7. 구현/runtime 변경은 새 실패 관점 20개, 실제 PR diff는 독립 실패 관점 30개로 검토한다. PR 본문에 가능한 simulator 동작 화면을 evidence 경로로 첨부하고, spec, fixture, evidence, 구현 한계와 공식 상태 유지 근거를 연결한다.
8. 구현 merge 후 공식 상태 원본 [`spec/STATUS.md`](../spec/STATUS.md)와 공개 페이지 원본 [`spec/public/roadmap.html`](../spec/public/roadmap.html)을 실제 구현 근거와 같이 갱신한다. C11.4 checkbox와 공식 수 `72/213`은 유지한다. 저장소 lockfile 기준으로 선택 Rspress 출력을 만들고 Tailnet preview와 동기화한다. 알려진 preview root는 `/Users/yoonhb/Library/Application Support/SpinonPreview`, 공개 roadmap route는 [`https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`](https://macstudio.tailed42f2.ts.net/spinon/roadmap.html)이다. 동기화 직전에 preview server의 실제 root/config를 다시 확인하고, 삭제 없이 선택 파일을 갱신하며 교체 전 파일은 작업별 backup에 보존한다. 구현 evidence directory는 `spec/internal/evidence/c11-4-cyclic-row-percentage-auto-repeat-runtime-2026-10-25/`로 두고 그 안의 `tailnet-sync-manifest.json`에 선택 산출물의 source path, served route, byte count, SHA-256을 기록한다. 선택 대상에는 `roadmap.html`, `STATUS.html`, runtime [`README.html`](../spec/internal/evidence/c11-4-cyclic-row-percentage-auto-repeat-runtime-2026-10-25/README.html), 그리고 그 README가 연결하는 Android/iOS target·control screenshot 4개가 포함된다. Local 파일과 각 served response의 HTTP 200·byte count·SHA-256을 대조하고, Agent Browser에서 roadmap route와 [`runtime README route`](https://macstudio.tailed42f2.ts.net/spinon/internal/evidence/c11-4-cyclic-row-percentage-auto-repeat-runtime-2026-10-25/README.html), 그 페이지의 네 simulator screenshot을 렌더링 확인한 뒤 해당 세션·프로세스만 종료한다.

## 계획 실패 관점 20개

1. `height:max-content`가 repeat count 계산 시 definite preferred block size가 아닌가?
2. Target의 repeat count 1과 final percentage track size 8px을 서로 다른 계산 단계로 설명하는가?
3. Cyclic `50%`가 intrinsic sizing에서 `auto`처럼 취급되는 기준을 §7.2.1/§11에 연결하는가?
4. 최종 basis 16px이 Chrome 출력에서 역산된 값이 아니라 유일한 16px item의 intrinsic contribution에서 독립 계산되는가?
5. Auto-repeat count가 percentage resolution 뒤 2로 재확장되지 않는 규범 근거가 있는가?
6. Definite 40px control의 count 2와 tracks `[20,20]`을 별도로 계산하는가?
7. 8px row track보다 8px 큰 fixed item의 overflow를 root sizing 결과와 혼동하지 않는가?
8. Root/item geometry는 border-box와 content-box를 구별하고 0 inset 조건을 반영하는가?
9. Out-of-flow half-height marker가 item count와 sizing contribution에서 제외되는가?
10. Row axis와 column axis의 재계산 경로를 대칭이라고 가정하지 않는가?
11. Initial/final row pass의 trace phase를 `sequence`만으로 유추하지 않는가?
12. Trace에 runtime/FFI `axis=Rows`, count, basis, resolved track list가 같은 pass provenance로 묶이는가? (Taffy 내부 축은 `Block`이다.)
13. Target gate에서 width, height, children, style, placement, repeat mode, gap 및 node 수를 정확히 검사하는가?
14. 유사하지만 다른 percentage, repeat mode, extra child, inset, gap 및 marker input은 fail-closed되는가?
15. Definite row control이 기존 일반 경로에서 처리되는지 확인하고 fixture gate의 긍정 결과와 혼동하지 않는가?
16. Browser raw observation과 규범 기대값이 evidence에서 분리되어 있으며 fixture와 hash로 연결되는가?
17. Chrome headless 버전·viewport·DPR·CSS.supports 결과가 입력 재현 조건에 포함되는가?
18. Runtime test가 simulator scene과 같은 typed input을 실제 session/layout 경로에 전달하는가?
19. Default artifact와 opt-in fixture artifact를 각각 검사하여 gate가 실제 binary boundary를 이루는가?
20. 제한 fixture 성공을 일반 CSS Grid 지원, WPT 적합성 또는 C11.4 완료로 과장하지 않는가?

## 계획 사전 비교 상태

[사전 비교 자료](../spec/internal/evidence/c11-4-cyclic-row-percentage-auto-repeat-precomparison-2026-10-25/README.md)는 Chrome 156 관측과 독립 계산을 보존한다. 구현 중에도 이 기대값을 기준으로 사용한다. C11.4 공식 상태와 완료 수는 구현 PR 병합 전까지 변경하지 않는다.
