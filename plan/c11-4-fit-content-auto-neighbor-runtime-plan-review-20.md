# C11.4 fit-content 옆 auto implicit row 런타임 계획 · 적대적 검토 20관점

**검토 대상:** [`c11-4-fit-content-auto-neighbor-runtime.md`](./c11-4-fit-content-auto-neighbor-runtime.md)와 연결된 2026-10-27 사전 비교, 현재 projection/runtime fixture 코드, pinned WPT 및 CSS Grid 2025 CRD.

**범위:** 계획의 기대값·입력 경계·실행 증거·실패 판정만 읽었다. 계획과 제품 소스는 수정하지 않았다. 테스트나 앱을 실행하지 않았다. 초기 검토는 **PARTIAL (12 PASS / 8 PARTIAL / 0 FAIL)**이었다. 2026-10-27 수정안 재검토를 거친 최종 판정은 **PASS (20 PASS / 0 PARTIAL / 0 FAIL)**이다. 아래 후속 검토가 초기 PARTIAL 표기를 갱신한다.

## 실패 경로별 검토

1. **Pinned WPT source와 live source를 같은 revision으로 확인하지 못할 수 있음 — PARTIAL.** 계획과 `source-pins.json`은 WPT revision `c999c58338ee1d223df5ad62e48be1202ced0d37` 및 pinned SHA-256 `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736`를 고정한다. 다만 live source와 일치한다는 기록은 boolean뿐이고, 실제 관측한 live SHA-256 값이나 원문 response bytes는 보존되지 않았다. [`source-pins.json`](../spec/internal/evidence/c11-4-fit-content-auto-neighbor-runtime-precomparison-2026-10-27/source-pins.json)과 [`chrome-wpt-live-observation.raw.json`](../spec/internal/evidence/c11-4-fit-content-auto-neighbor-runtime-precomparison-2026-10-27/chrome-wpt-live-observation.raw.json)은 서로 다른 양의 증거를 담는다. live digest 값을 기록하거나 고정 source 파일과 직접 재검산할 수 있게 해야 한다. 대상 assertion은 [pinned WPT source](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html#L131-L132)에서 확인된다.

2. **Chrome의 testharness 결과를 Spinon 적합성으로 확대할 수 있음 — PASS.** 사전 비교 원장은 upstream Chrome testharness 결과 `13/13`, target assertion, `10px 290px`을 기록하고, 계획은 이를 Spinon WPT runner 결과로 부르지 않는다. 원장에는 HeadlessChrome 156, URL, test title과 target result가 들어 있다. [`chrome-wpt-live-observation.raw.json`](../spec/internal/evidence/c11-4-fit-content-auto-neighbor-runtime-precomparison-2026-10-27/chrome-wpt-live-observation.raw.json), 계획 16행 및 범위 밖 50행.

3. **대상 test의 두 implicit row와 nested content가 다른 입력으로 바뀔 수 있음 — PASS.** pinned WPT는 `grid-auto-rows: fit-content(40px) auto`와 각 10px nested block을 갖는 두 direct item을 사용한다. 런타임 계획도 두 direct item과 각 10px child를 고정하고 text/image, span, nested Grid를 제외한다. [Pinned WPT target DOM](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html#L72-L75)과 계획 23–24행이 일치한다.

4. **Definite block size·column·gutter가 기대값 산술과 달라질 수 있음 — PASS.** WPT grid height는 300px이고 explicit column은 60px이다. 계획은 300px definite block size, 한 60px explicit column, zero gap/padding/border 및 root `60×300px`을 고정한다. 이 조건에서 border/content box 차이와 row gutter 누락이 없다. 계획 23행, 33행 및 [pinned WPT setup](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html#L11-L15).

5. **`auto` minimum track이 child의 10px minimum contribution을 보존하지 못할 수 있음 — PASS.** 두 item은 기본 auto minimum을 사용하며 10px nested block이 각각 row의 minimum contribution을 만든다. 계획은 `min/max` constraint를 넣지 않는다고 고정해 `min-height:0` controls와 혼동하지 않는다. CSS Grid 2025 CRD §6.6 및 §11.5의 automatic minimum/minimum contribution 규칙과 계획 17·24행, [pinned WPT target DOM](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html#L72-L75)이 같은 입력을 가리킨다.

6. **`fit-content(40px)` argument가 row를 40px로 키우거나 결과를 cap했다고 오해할 수 있음 — PASS.** 기대 content contribution 10px은 40px argument보다 작다. 계획은 max sizing function이 `max-content`인 track과 fit-content clamp 역할을 구분하며, 이 장면의 핵심은 argument가 growth limit에 닿는지보다 해당 track이 stretch 대상이 아닌지를 확인하는 것이라고 서술한다. CSS Grid 2025 CRD §11.5는 fit-content growth limit을 argument로 clamp하고, §11.8은 `auto` max sizing function만 stretch한다. 계획 17행 및 [CRD §11.5 / §11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-content).

7. **`align-content: normal`의 Grid stretch 동작을 `start`나 `stretch`와 혼동할 수 있음 — PASS.** 런타임 CSS는 `normal`을 입력하고, 기대값은 `auto` max sizing function track 하나만 늘어난다고 고정한다. CSS Grid 2025 CRD §11.8은 이 축의 `normal` 또는 `stretch`에서 `auto` max track에 definite positive free space를 나눈다. 계획 17·23·34행 및 [CRD §11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-stretch).

8. **남는 공간을 잘못 계산하거나 fit-content track에 배분할 수 있음 — PASS.** pre-stretch bases `[10,10]`, free space `300−10−10=280`, stretch 대상 index `[1]`, 최종 `[10,290]`이 expected JSON에 독립적으로 고정돼 있다. Chrome 결과를 기대값으로 역산하지 않는다는 계획의 경계도 명시돼 있다. [`expected.json`](../spec/internal/evidence/c11-4-fit-content-auto-neighbor-runtime-precomparison-2026-10-27/expected.json)과 계획 17행.

9. **실제 CSS cascade 이후 `grid-auto-rows`가 의도한 typed track list가 아닐 수 있음 — PARTIAL.** 목표는 CSS→HostDocument/Stylo→typed input→Taffy를 통과한다고 하지만 관측 기준은 source allowlist와 최종 trace다. 기존 [`grid_values.rs`](../crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs#L821-L847)는 `ComputedCssGridAutoRows::TrackList`를 `RuntimeBlockFlexGridTrackSizingV1`일 때만 투영한다. 계획은 computed value와 projected `GridAutoRows::TrackList([FitContent(40px), Auto])`를 관측하거나 검증하도록 요구하지 않아 adapter가 기대 typed input을 만들지 않아도 core 결과만으로 통과할 수 있다. CSS computed value와 typed projection을 pass criteria에 추가해야 한다.

10. **`align-content`, auto flow, gap 또는 sizing profile의 computed 값이 달라질 수 있음 — PARTIAL.** 계획은 CSS source에서 값을 명시하지만 pass criteria는 computed-style snapshot의 값과 해당 root node ownership을 확인하지 않는다. 실제 목표가 Stylo 경계를 통과하는 것이므로 `RuntimeBlockFlexGridTrackSizingV1`, `align-content:normal`, row auto-flow, zero row-gap 및 definite block size가 root snapshot에 적용됐는지 확인해야 한다. 계획 21–26·32–35행과 기존 [`implicit intrinsic text adapter`](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_implicit_intrinsic_text.rs#L41-L57)는 해당 profile·root-style gate가 실제 경계임을 보여준다.

11. **다른 root/item tree가 allowlisted case로 오인될 수 있음 — PARTIAL.** 계획은 allowlisted source와 root/item/nested child의 동일 revision을 요구하지만 case ID, root ID, 두 item ID, nested child ID 및 정확한 element-child 관계를 계획 입력에 적지 않았다. `expected.json`도 `case_id`와 수량만 보존한다. 재현 가능한 allowlist 및 node별 revision 검증에 쓸 identity 목록을 고정해야 한다. 계획 21·32행과 [`expected.json`](../spec/internal/evidence/c11-4-fit-content-auto-neighbor-runtime-precomparison-2026-10-27/expected.json).

12. **WPT의 auto inline size를 바꾸면서 block-axis 입력이 뜻하지 않게 달라질 수 있음 — PASS.** 계획은 root의 CSS `width:60px`을 고정해 WPT의 auto inline size와 다른 점을 밝힌다. explicit column도 60px이며 text, replaced image, percentage 및 aspect-ratio 입력을 제외해 이 차이가 10px row contribution을 바꾸지 않는다는 범위를 설명한다. 계획 25–27행과 expected JSON의 `source_shape`, computed `width_px`/`height_px`, deviation 기록.

13. **두-row implicit pattern 순서 또는 자동 배치 순서가 뒤바뀔 수 있음 — PASS.** `rows=[]`, auto rows `[fit-content(40px), auto]`, direct item 두 개, row auto-flow를 고정하고 trace에는 CSS track 2개만 있어야 한다고 판정한다. gutter는 CSS track count에서 분리한다. 계획 23–24·33행.

14. **트랙 크기는 맞지만 두 item이 잘못된 행 좌표에 놓일 수 있음 — PARTIAL.** 계획과 expected JSON은 root 크기, item block heights `[10,290]`, child heights `[10,10]`를 요구하지만 item/nested-child의 전체 frame 좌표를 정의하지 않는다. 두 item이 같은 y에 겹치거나 child가 item 시작점에서 벗어나도 높이·track 검사는 통과할 수 있다. 기대 frame `(0,0,60,10)`, `(0,10,60,290)` 및 child 위치를 고정해야 한다. 계획 35–38행과 expected JSON의 `expected_runtime`.

15. **여러 sizing pass 중 일부만 골라 stage trace가 맞는 것처럼 보일 수 있음 — PARTIAL.** `expected.json`의 비교 규칙은 “one row pass”를 요구하지만 계획의 실행 항목은 row trace stage 순서와 track 수만 적는다. pass count와 같은 row pass에서 각 stage가 정확히 한 번 나타나는지 acceptance list에 명시해야 한다. 그렇지 않으면 반복 pass의 마지막 결과만 고르는 matcher가 중간 pass 오류를 숨길 수 있다. expected JSON `comparison_rules.trace`, 계획 33–35행.

16. **정수 CSS px oracle이 device px 또는 surface scale과 혼동될 수 있음 — PARTIAL.** 허용 오차는 `0 CSS px`이고 expected JSON은 “fixture-specific scale conversion” 뒤 exact를 요구하지만, 계획은 그 변환식이나 layout viewport/device scale 입력을 정하지 않는다. 플랫폼 frame을 CSS px로 읽는 정확한 source field와 변환·rounding 규칙을 실행 전에 고정해야 한다. 계획 30·38행, expected JSON `comparison_rules`.

17. **trace, frame과 draw/present가 서로 다른 layout revision에서 합쳐질 수 있음 — PASS.** 계획은 root, items, nested children을 같은 tree/style/environment revision에 묶고 frame, style snapshot, track trace, draw/present도 같은 source revision이어야 한다고 명시한다. Missing/stale/partial 결과는 실패다. 계획 30–35행 및 expected JSON `selected_root_and_child_revision`.

18. **잘못된 scene/style을 새 allowlist가 받아들이거나 일부 layout을 반환할 수 있음 — PARTIAL.** 계획은 malformed/unsupported input을 fail-closed라고 규정하지만 test gates는 positive allowlisted scene 하나만 정의한다. Unknown scene ID, 변경된 auto-row order/value, extra/missing item 및 extra explicit row를 거부하고 partial output이 없는 negative control을 focused Rust/FFI tests에 포함하도록 지정해야 한다. 계획 21·26·44행.

19. **fixture-only 변경이 default product artifact에 섞이거나 플랫폼 빌드가 다른 source를 검증할 수 있음 — PASS.** 계획은 opt-in C11.4 route만 사용하고 새 public CSS admission을 금지하며, default-vs-opt-in artifact boundary와 Android/iOS 각각의 final-source build 및 same source/input hash를 요구한다. 제품 코드 영향이 있을 때만 영향 테스트와 contract 0059를 갱신하도록 경계를 둔다. 계획 26·30·44·50행.

20. **시뮬레이터·이미지·PR 근거의 범위를 과장할 수 있음 — PASS.** 계획은 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator만 요구하고 physical device/hardware GPU/iOS compositor 이후 pixel은 범위 밖으로 둔다. 스크린샷은 layout box와 surface 관측을 구분하며 별도 측정 전 pixel equality를 주장하지 않는다. Plan 20/30 reviews, CI-empty 처리, Rebase and merge, Agent Browser 사용·종료 및 `72/213` 미승격도 명시돼 있다. 계획 36·40–50행 및 [`docs/project-rules.md`](../docs/project-rules.md#L21-L24).

## 판정 요약

Normative target과 `[10px, 290px]` 기대값은 pinned WPT 및 CSS Grid 2025 CRD의 min contribution, fit-content max sizing function, `normal` stretch eligibility와 남은 공간 분배에 맞는다. 초기 검토에서 식별한 pass criteria와 source-pin 증거 공백은 아래 후속 검토에서 계획/기대값 수정으로 해결됐음을 확인했다. 최종 기준은 아래 수정안 재검토의 판정이다.

## 2026-10-27 수정안 재검토

수정된 계획, 기대값 JSON, source pins를 읽고 앞서 PARTIAL이었던 여덟 관점을 다시 확인했다. 계획/기대값 파일은 변경하지 않았고 테스트나 앱도 실행하지 않았다.

1. **이전 #1 — PASS.** `source-pins.json:5–8`은 pinned SHA와 실측 live SHA를 모두 기록하고 두 값이 같다. 계획 `15–16`행도 revision, digest, live 관찰의 범위를 밝힌다.
2. **이전 #9 — PASS.** 계획 `27`행과 expected `runtime_identity.computed_style`/`grid_container_input`(`119–153`행)은 computed `grid-auto-rows`, `ComputedStyleProfile::RuntimeBlockFlexGridTrackSizingV1`, `GridPlacementMode::TrackSizing`, `GridAutoRows::TrackList([FitContent(LengthPx(40)), Auto])`를 서로 구분해 고정한다. 소스도 profile과 `GridContainerInput.placement_mode`를 별도 경계로 정의하고 C11.4 profile을 `GridPlacementMode::TrackSizing`으로 투영한다 ([snapshot.rs](../crates/spinon-style/src/stylo_dom/cascade/snapshot.rs#L668-L669), [grid.rs](../crates/spinon-layout/src/grid.rs#L236-L252), [grid.rs](../crates/spinon-layout/src/grid.rs#L272-L293), [grid_values.rs](../crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs#L497-L519)). 이전의 잘못된 field labeling은 수정됐다.
3. **이전 #10 — PASS.** 계획 `27`, expected `129–146`행은 root computed `grid-auto-rows`, `align-content`, auto-flow, row gap, explicit rows와 typed track list/profile gate를 요구한다. Root identity는 별도 고정돼 있다.
4. **이전 #11 — PASS.** 계획 `23`행은 scene/root 및 ordered item/child IDs와 direct-child 관계를 고정한다. expected `120–128`행은 같은 tree order를 보존한다.
5. **이전 #14 — PASS.** 계획 `38`행과 expected `63–94`행은 root, 두 item, 두 child의 전체 `(x,y,width,height)` CSS-px tuple을 고정한다.
6. **이전 #15 — PASS.** 계획 `34`, `37`행은 grid trace 하나, Columns/Rows 각 한 pass, 필요한 stage 각각 정확히 한 번과 순서를 요구한다. expected `104–108`행도 동일한 cardinality를 명시한다.
7. **이전 #16 — PASS.** 계획 `38–39`행 및 expected `103`, `107`행은 track/frame 값을 pre-render `LayoutOutput`에서 직접 읽는 CSS px로 정의하고 scale/DPR 변환을 하지 않는다고 일치하게 명시한다. 이전의 상반된 scale-conversion 문구는 제거됐다.
8. **이전 #18 — PASS.** 계획 `28`행과 expected `148–152`행은 unknown scene, 변경된 row list/extra explicit row, item/child 누락·추가, 중복 identity, stale revision의 fail-closed negative controls를 고정한다.

여덟 관점 모두 수정된 계획과 기대값에서 해결됐다. 새 문구와 typed input 표현을 기존 소스 경계에 대조했으며, 계획·기대값 파일은 이 검토 작업에서 수정하지 않았다. 최종 검토 판정은 **PASS (20 PASS / 0 PARTIAL / 0 FAIL)**이다.

### #18 negative-control coverage 재확인

수정된 계획 `28`행은 나열한 unknown scene, 변경 입력·tree, extra explicit row, stale revision을 각각 focused Rust/FFI tests에서 실행하고 rejection 및 partial output 부재를 확인하도록 요구한다. expected `148–152`행도 거부 조건을 고정한다. 현재 소스는 unknown C11.4 scene을 `Err`로 돌려보내며 fixture test가 이를 확인하고 (`c11_4.rs:1135–1137, 1446`), trace-report test는 다른 runtime revision을 거부한다 (`trace_report.rs:623–657`). 계획 단계 review에서 테스트는 실행하지 않았다. #18은 **PASS**로 유지하며 전체 판정도 **PASS (20 PASS / 0 PARTIAL / 0 FAIL)**이다.

### Pinned WPT source link recheck · 2026-10-27

GitHub에서 pinned revision `c999c58338ee1d223df5ad62e48be1202ced0d37`의 133-line source를 다시 대조했다. #1 target assertion link는 131–132행, #3 target DOM link는 72–75행, #4 setup link는 11–15행을 가리킨다. Source-link 보정이며 검토 결과는 **PASS (20 PASS / 0 PARTIAL / 0 FAIL)**로 변하지 않았다.

### C11.4 fixture HostDocument 경계 재확인 · 2026-10-27

수정된 계획 `24`행의 경계는 현재 fixture 경로와 맞는다. C11.4 runtime session은 `new_with_css_profile(..., root_html: None)`을 호출하고 ([session.rs](../crates/spinon-runtime/src/session.rs#L472-L480)), actor는 `None`일 때 `HostDocumentBridge::new()`를 사용한다 ([actor.rs](../crates/spinon-runtime/src/session/actor.rs#L30-L40)). 별도의 C11.4 FFI fixture route가 allowlisted scene source를 기존 host에 `eval`하고 ([ffi/fixtures/c11_4.rs](../crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c11_4.rs#L67-L76)), 그 source는 `document.createElement`로 scene DOM을 만든다 ([fixtures/c11_4.rs](../crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs#L693-L720)). `ParsedRootHtml` 입력은 actor의 다른 분기다. 따라서 이 fixture가 production root HTML parser, color-scheme 또는 media-query parity를 검증하지 않는다는 범위 선언은 정확하다. 판정은 **PASS**, 전체 검토는 **PASS (20 PASS / 0 PARTIAL / 0 FAIL)**로 유지한다.

### #3/#19 계획 구체성 재확인 · 2026-10-27

수정된 계획 `25`행의 `display: grid`는 pinned WPT의 `.grid` setup과 target element의 class 사용에 맞는다 ([WPT setup](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html#L11-L15), [target](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html#L72-L75)). 계획 `48`행은 기본 runtime artifact, 제품 feature 조합 artifact, opt-in fixture-test artifact를 각각 빌드해 fixture FFI symbol/string/payload가 앞의 두 artifact에서 제외되고 positive-control artifact에 포함되는지 확인하도록 구체화했다. 이는 fixture directory가 binary inclusion을 보장하지 않고 기본·제품·fixture-test 조합의 inclusion을 별도로 확인하라는 project rules `35–37`행과 맞는다. Cargo는 `c04-runtime-gpu` core feature와 이를 포함하는 `c04-runtime-gpu-fixture-test`를 분리하고, runtime GPU 및 FFI fixture modules는 fixture-test feature에서만 연결된다 ([project-rules.md](../docs/project-rules.md#L35-L37), [Cargo.toml](../crates/spinon-ffi/Cargo.toml#L12-L32), [runtime_gpu.rs](../crates/spinon-ffi/src/runtime_gpu.rs#L29-L30), [ffi.rs](../crates/spinon-ffi/src/runtime_gpu/ffi.rs#L11-L12)). 이들은 구현 전 검증 요구사항이며, 본 재검토에서 빌드를 실행한 것은 아니다. #3/#19 모두 **PASS**이고 최종 판정은 **PASS (20 PASS / 0 PARTIAL / 0 FAIL)**로 유지한다.

### #19 세 artifact 조합 확인 · 2026-10-27

계획 `48`행은 세 artifact를 `cargo build --locked -p spinon-ffi --lib` 기본 조합, `--features c04-runtime-gpu`, `--features c04-runtime-gpu-fixture-test`로 각각 구분한다. 기본 및 제품 feature artifact에서는 C11.4 fixture ABI/scene string/payload가 제외되고, fixture-test positive control에는 포함돼야 한다. `llvm-nm`과 문자열 검사 또는 동등한 검증으로 두 exclusion과 inclusion을 보존하도록 요구한다. 이 분리는 project rules `35–37`행 및 Cargo wiring과 일치한다: `c04-runtime-gpu`는 fixture 없는 core feature이고, fixture-test feature는 이를 포함하면서 C11.4 fixture dependency를 opt-in하며, runtime GPU/FFI fixture modules는 fixture-test feature에서만 연결된다 ([project-rules.md](../docs/project-rules.md#L35-L37), [Cargo.toml](../crates/spinon-ffi/Cargo.toml#L12-L32), [runtime_gpu.rs](../crates/spinon-ffi/src/runtime_gpu.rs#L29-L30), [ffi.rs](../crates/spinon-ffi/src/runtime_gpu/ffi.rs#L11-L12)). 실제 artifact 빌드/검사는 구현 gate에 남아 있어 이번 plan review에서 실행하지 않았다. #19은 **PASS**, 전체 판정은 **PASS (20 PASS / 0 PARTIAL / 0 FAIL)**로 유지한다.

### #12 root width 고정 재확인 · 2026-10-27

계획 `25`, `27`행은 root CSS `width:60px`을 명시하고 WPT의 auto inline size와 차이를 공개한다. expected `source_shape`(`4–10`행), computed style(`131–143`행), deviation(`112–114`행)은 width/height 및 차이를 같은 값으로 기록한다. Pinned WPT는 `.grid`에 `display:grid`, 60px explicit column, definite 300px height를 두고 width는 지정하지 않으며, 대상 element는 같은 `.grid` class에서 `fit-content(40px) auto`를 적용한다 ([setup](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html#L11-L15), [target](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html#L72-L75)). Root inline size만 고정하는 계획 차이는 explicit column을 바꾸지 않고, 고정 10px child 외에 text, replaced item, percentage 또는 aspect-ratio contribution도 추가하지 않는다. 따라서 이 차이는 관측 frame의 width를 고정하며 목표 block-axis track sizing `[10, 290]px`과 모순되지 않는다. #12는 **PASS**, 전체 판정은 **PASS (20 PASS / 0 PARTIAL / 0 FAIL)**로 유지한다.

### Pinned WPT target DOM anchor correction · 2026-10-27

Pinned WPT의 DOM 범위는 container 72행, 두 child/item 73–74행, closing tag 75행이다. #3과 #5 및 후속 DOM 참조를 `#L72-L75`로 바로잡았다. #1 assertion `#L131-L132`와 #4 setup `#L11-L15`는 유지했다. 이 링크 정정으로 최종 검토 결과는 변하지 않는다: **PASS (20 PASS / 0 PARTIAL / 0 FAIL)**.
