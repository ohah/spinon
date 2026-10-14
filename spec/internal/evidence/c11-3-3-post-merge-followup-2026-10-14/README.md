# C11.3.3 병합 후 증거 보강 · 2026-10-14

PR #152(`186a0e0`)의 적대 검토에서 남긴 세 가지 PARTIAL은 기능 결함 판정이 아니라 독립된 직접 주입·배치·앱 revision 근거 부족이었다. 아래 검증으로 세 경계를 닫았다. 이 결과는 C11.3.3의 제한 profile 검증을 보강하며 Spinon WPT 적합성이나 전체 CSS Grid 지원 완료를 뜻하지 않는다.

## Typed computed range 방어

Stylo `TemplateAreas`를 직접 구성해 row와 column 각각의 start 0, dimension 초과 end, reversed range, empty range를 `computed_grid_template_areas`에 주입했다. 8개 입력이 모두 node `733`과 `grid-template-areas` 문맥을 보존한 `UnsupportedComputedGridValue`로 fail closed 했다.

- 코드: <a href="../../../../crates/spinon-style/src/stylo_dom/cascade/incremental/grid_areas/tests.rs">`grid_areas/tests.rs`</a>
- 실행: `mise exec -- cargo test --locked -p spinon-style malformed_computed_ranges_fail_closed_with_node_and_property_context`
- 결과: 1 test passed; test 안에서 8 malformed typed inputs를 검사했다.

## Area 없는 bare-ident fallback

Pinned Chrome for Testing 154.0.8037.92, revision `@334b65d254ccc35df4fca82706d1753227b01039`에서 `grid-template-areas:none`이고 두 축에 `[panel]` line이 반복되는 fixture의 item frame은 `(40, 30, 20, 20)`이었다. Rust style-to-layout fixture가 같은 computed frame을 반환했다. 두 축 모두 explicit area alias pair가 없어서 area metadata 없이 authored line occurrence 경로를 확인한다.

- Chrome 관찰: <a href="chrome-154-bare-ident-no-area.json">`chrome-154-bare-ident-no-area.json`</a>
- 재현: `SPINON_CHROMIUM_BIN=<Chrome-154 executable> mise exec -- bun run css:reference:c11-3-3-bare-ident-no-area`
- Rust 실행: `mise exec -- cargo test --locked -p spinon-style-to-layout bare_ident_without_template_area_falls_back_to_first_named_line`
- 결과: Rust 1 test passed; Chrome과 frame 일치.

## 같은 AVD host에서 mutation 후 재평가

별도 `revision-reuse` scene을 Android API 37 ARM64 AVD에서 실행했다. 앱은 동일한 native host handle로 같은 fixture source를 두 번 평가했다. 첫 호출이 2개 node를 만들고, 두 번째 호출은 root template areas만 반전해 같은 item node를 다른 named area로 이동했다. 두 단계 모두 `status=0`, `layout=ready`였고 document revision은 `10 → 11`, render-tree revision은 `2 → 3`으로 증가했다. item NodeId `2`의 frame은 `(0, 0, 80, 40) → (80, 0, 80, 40)`으로 바뀌었으며 마지막 WGPU draw는 2 boxes를 제출했다.

- AVD run: <a href="../c11-3-3-grid-template-areas-runtime-2026-10-14/android-avd-c1133-revision-20261014T032109347Z-0ee50c0d/result.json">`result.json`</a>
- Fixture diagnostic: <a href="../c11-3-3-grid-template-areas-runtime-2026-10-14/android-avd-c1133-revision-20261014T032021358Z-5f461b1d/revision-reuse.log">`position:absolute` profile failure</a>
- app SHA-256: `a1133e2555ee60f6ce660f922015270120da1d6edd70eb231495388fcc8fc9f1`
- Android log: <a href="../c11-3-3-grid-template-areas-runtime-2026-10-14/android-avd-c1133-revision-20261014T032109347Z-0ee50c0d/revision-reuse.log">`revision-reuse.log`</a>
- 실행: `mise exec -- bun run css:capture:c11-3-3-grid-template-areas:android-avd:revision-reuse -- emulator-5554`
- Rust fixture guard: `mise exec -- cargo test --locked -p spinon-ffi --features c04-runtime-gpu-fixture-test revision_reuse_source_mutates_the_area_on_a_reused_runtime_host`
- renderer: ANGLE/Vulkan/SwiftShader software renderer. 이 실행에서 실기기는 연결하거나 사용하지 않았고, emulator 종료도 확인했다.

## 회귀 게이트

- `mise exec -- bun run test`: JS 2/2, CSS reference 413/413, R05 benchmark 29/29, Rust workspace 696 passed·4 ignored·0 failed.
- `mise exec -- cargo clippy --workspace --all-targets --locked -- -D warnings`: 통과.
- `mise exec -- cargo fmt --all -- --check`, `git diff --check`: 통과.
- revision source feature test: 1/1 통과; 전체 상세는 <a href="full-workspace-test.log">`full-workspace-test.log`</a>, <a href="workspace-clippy.log">`workspace-clippy.log`</a>, <a href="ffi-fixture-clippy.log">`ffi-fixture-clippy.log`</a>, <a href="ffi-followup-test.log">`ffi-followup-test.log`</a>에 보존했다.

## 판정 경계

기존 26-scene matrix와 iOS 결과는 변경하거나 합산하지 않았다. Android revision scene은 별도 run ID로 분리했다. 이 후속은 malformed typed range, area 없는 bare ident fallback, 같은 runtime host의 mutation/revision 재사용 근거를 채운다. Android 실기기·hardware GPU·WPT 적합성, compositor 제출 이후 iOS 픽셀 및 전체 CSS Grid는 여전히 검증 범위 밖이다.

- [후속 계획 검토 20개](../c11-3-3-post-merge-followup-plan-review-2026-10-14.md)
- [후속 구현 검토 20개](../c11-3-3-post-merge-followup-implementation-review-2026-10-14.md)
- [PR 적대 검토와 사후 판정](../c11-3-3-template-areas-pr-review-2026-10-14.md)
