# C11.4 · overflow automatic minimum both-axis runtime slice

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [기존 14-case Rust oracle](./c11-4-overflow-scrollable-auto-minimum.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**구현 전 비교:** [고정 CSS fixture](../tests/fixtures/css/c11/grid-auto-min-scrollable-overflow-c114-axis-precomparison.html) · [Chrome 관측과 독립 기대값](../spec/internal/evidence/c11-4-overflow-auto-minimum-runtime-precomparison-2026-10-28/README.md)

**계획 실패 관점 검토:** [서로 다른 관점 20개](./c11-4-overflow-auto-minimum-runtime-plan-review-20.md)

## 결손과 목표

기존 [C11.4 overflow automatic minimum 계획](./c11-4-overflow-scrollable-auto-minimum.md)은 `spinon-layout`에 고정한 14개 Rust 입력으로 Columns Ahem text와 Rows natural replaced image를 검사한다. 별도 [span runtime 후속](./c11-4-auto-minmax-span-overflow-runtime.md)은 두-column span의 다섯 `overflow-x` 입력을 앱까지 연결했다. 아직 single-track `minmax(auto,1fr)` matrix의 양 축, item-minimum/track-minimum control, CSS computed overflow의 반대 축 정규화가 app runtime 경로에서 함께 확인되지 않았다.

이 slice는 기존 14개 독립 layout 입력을 C11.4 전용 fixture-test cascade·typed projection·Taffy trace·GPU draw/present 경로에 연결한다. 모든 CSS authoring과 FFI scene은 opt-in `c11-4-grid-track-sizing-fixture-test` 경계 안에 둔다. 일반 `RuntimeBlockFlexGridTrackSizingV1` author allowlist, 제품 CSS admission, C13 clipping/scrolling 및 공개 API는 확대하지 않는다.

## 독립 기준과 입력

CSS Grid 2025 CRD §6.6은 item automatic minimum을 해당 축의 computed overflow가 scrollable인지에 따라 분기한다. CSS Overflow Level 3 §3.1은 한 축이 scrollable일 때 반대 축의 `visible`은 computed `auto`, `clip`은 computed `hidden`이 된다고 정한다. 이번 fixture는 sizing 대상 축에 각 `visible`/`clip`/`hidden`/`auto`/`scroll`을 주고 반대 축 authored value는 `visible`로 고정한다. 따라서 `clip`은 그대로 남고 scrollable target의 반대 축은 `auto`가 된다. 앞선 shorthand 기반 Chrome observation과 geometry를 공유하되, 이 axis-specific computed values는 새 Agent Browser 실행으로 구현 전에 별도로 기록한다.

고정 input과 geometry/minimum 기대값은 기존 [expected.json](../spec/internal/evidence/c11-4-overflow-scrollable-auto-minimum-precomparison-2026-10-28/expected.json)의 14개 case를 재사용한다. Ahem `XXXXXXXXXX`의 Columns min-content는 100px, natural `10×10px` replaced image의 Rows min-content는 10px이다.

| 축 / case | authored 대상 축 / 반대 축 | grid / track / item | Minimum 기댓값 |
| --- | --- | --- | --- |
| 열 `visible`, `clip` | `overflow-x:<value>` / `overflow-y:visible` | grid `60×20`; track/item 너비 `100px` | content 기반 automatic `100px` |
| 열 `hidden`, `auto`, `scroll` | `overflow-x:<value>` / `overflow-y:visible` | grid `60×20`; track/item 너비 `60px` | overflow automatic `0px` |
| 열 item 최소 크기 대조 | 두 축 모두 visible; `min-width:0` | grid/track/item 너비 `60px` | 지정 최소값 `0px` |
| 열 track 최소 크기 대조 | 두 축 모두 visible; `minmax(0,1fr)` | grid/track/item 너비 `60px` | item Minimum 질의 없음 |
| 행 `visible`, `clip` | `overflow-y:<value>` / `overflow-x:visible` | grid `30×5`; column `10px`; track/item 높이 `10px` | content 기반 automatic `10px` |
| 행 `hidden`, `auto`, `scroll` | `overflow-y:<value>` / `overflow-x:visible` | grid `30×5`; column `10px`; track/item 높이 `5px` | overflow automatic `0px` |
| 행 item 최소 크기 대조 | 두 축 모두 visible; `min-height:0` | grid/track/item 높이 `5px` | 지정 최소값 `0px` |
| 행 track 최소 크기 대조 | 두 축 모두 visible; `minmax(0,1fr)` | grid/track/item 높이 `5px` | item Minimum 질의 없음 |

총 14개 scene이며 축마다 일곱 개다. `overflow-x/y` computed value는 정확한 CSS keyword이며 authored value 및 layout `LayoutOverflow` 값과 각각 비교한다. 새 사전 비교에는 로컬 fixture hash, Chrome 빌드·플랫폼·DPR, Ahem 준비 상태, 14개 ID 전체, computed x/y overflow, grid/item rectangle, used track과 natural image 크기를 기록한다. Chrome 결과는 독립 브라우저 관측이며 Spinon WPT 통과가 아니다.

모든 root에는 `align-items:stretch`를 명시한다. Chrome 156에서 `normal`은 5px row track의 replaced image item box를 natural 10px로 유지하지만, `stretch`는 overflow automatic minimum이 0인 control scene의 item box를 track과 같은 5px로 만든다. 기존 Rust oracle 및 이 matrix의 frame 기대값과 일치하는 입력을 고정하기 위해 alignment를 명시한다. Runtime projection은 `allow_c11_4_overflow_auto_minimum_fixture`가 켜진 C11.4 overflow route에서만 root의 `align-items:stretch`를 허용한다. 기본 경로와 다른 Grid profile은 계속 거부한다.

이 route의 track maximum은 `1fr`이다. 각 target pass에서 item `Minimum` contribution의 source/value를 검증하고, flexible maximum은 item `MaxContent` contribution을 요청하지 않으므로 그 contribution이 trace에 없어야 한다. Columns Ahem `100px` 및 Rows image `10px` max-content 입력값을 실제 track-sizer contribution trace와 혼동하지 않는다.

## Runtime adapter와 검증

- Add a dedicated fixed scene catalog and validator under existing `runtime_gpu/fixtures/c11_4` / `runtime_gpu/ffi/fixtures/c11_4` boundaries. Reuse the existing overflow fixture-test cascade/projection path only after validating the exact scene root, single direct item, axis, text/image input, CSS declarations, track list, revision and expected category. Do not admit these declarations through the general profile.
- Columns use the existing Ahem text intrinsic path. Rows use the pinned [10×10 RGBA PNG](../tests/fixtures/resources/c11-4-overflow-auto-minimum-10x10.png), image snapshot and CSS width `10px`, preserving the 10px natural row minimum without aspect-ratio coupling. Its URL and bytes are fixed in the opt-in fixture adapter. The `<img>` authored `overflow-x/y:visible` override is required because its UA default can be `clip`.
- For every scene, compare the exact root/item frame, used track arrays, axis owner, source/value or absence of the Minimum contribution query, max-content contribution where present, all trace stages, document/style/layout/environment revision identity, typed GPU scene identity and `drawPresented` result. Do not infer a correct minimum branch from final geometry alone.
- Verify the root's computed `align-items:stretch` value. Its acceptance is limited to the opt-in overflow automatic minimum projection option so row item frames cannot silently change when defaults or other profiles are used.
- Negative controls mutate each CSS overflow axis, cross-axis computed value, min-size, track minimum, root/item topology, text or image natural dimensions, node/style revision, profile, scene ID and trace completeness. Reject malformed or unlisted inputs before publishing a partial layout or frame.
- Exercise the 14-scene route on Android API 37 ARM64 AVD and iPhone 17 Pro / iOS 26.2 Simulator using the same source revision. Compare device frames/tracks/contributions against the fixed CSS baseline and independent normative values. Store raw logs, JSON reports, screenshots, app/build identity and source hashes. Screenshots are evidence of the simulator output only.
- Sample Android PixelCopy and iOS Metal drawable at fixed points for visible out-of-grid item boxes where the item frame exceeds the root (`column-visible` and `row-visible`). These samples test scene/presentation coordinates outside the root rectangle. They do not establish CSS `clip`/scroll clipping, scrollbar behavior, iOS post-compositor pixels or glyph/image texel painting.
- Build the production `c04-runtime-gpu` FFI artifact with fixture-test off and the opt-in fixture-test positive control. Inspect both archives for this route's identifiers/symbols/payload; static archive results are not whole APK/IPA exclusion evidence.
- Run focused Rust/layout/runtime/FFI tests, `cargo fmt --all -- --check`, the applicable fixture module boundary test, relevant targeted Clippy, Android/iOS simulator app builds, and the 14-scene matrix. Report unrelated baseline warnings and empty GitHub check rollups distinctly.

## 검토, PR 및 완료 판정 경계

구현 전에 독립된 20개 관점 계획 검토와 browser 사전 비교를 마친다. 구현 뒤에는 새 20개 runtime/code 실패 관점 검토를 기록한다. 프로젝트 규칙에 따라 최종 PR diff를 서로 다른 실패 관점 30개로 검토한다. 이는 사용자가 요청한 PR별 20회보다 많다. 결함 수정 후 영향받은 관점은 다시 확인한다. rendered screenshot을 얻으면 Android와 iOS Simulator 캡처를 PR에 첨부한다.

이 slice가 실제로 확인한 범위만 `spec/STATUS.md`, `spec/0001-conformance.md`, 상위 C11.4 계획과 내부 계약에 반영한다. PR #765 병합 정보는 아래 결과와 상태 대장에 기록했다. C11.4 항목은 미완료로 두고 공식 완료 수 `72/213`을 유지한다. 이 runtime slice는 일반 CSS overflow 허용, clipping·scroll 동작, 전체 WPT 실행, hardware GPU·실기기 동작 또는 C11.4 완료를 주장하지 않는다.


## 구현 및 PR #765 병합 결과

고정 14-scene 행렬을 opt-in fixture-test cascade·typed projection·Taffy contribution trace·FFI report·GPU draw/present 경로에 연결했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 14/14 scene을 통과했다. Android PixelCopy와 iOS compositor 제출 전 Metal drawable 표본은 row visible scene의 root 바깥 PNG red 및 item 아래 black을 exact match했다. Product/fixture static FFI archive boundary에서 일반 c04-runtime-gpu archive에는 새 route marker가 없고 fixture-test positive control에는 14 scene ID와 PNG marker가 있다.

최종 소스 검토에서 row image intrinsic revision 전달과 render image attachment 누락을 찾아 수정한 뒤 전체 matrix를 다시 실행했다. contribution trace의 정확히 두 pass 조건이 정상 repeated same-axis subpass를 거부해 축 존재·Columns 시작·연속 sequence 및 각 pass contribution 검증으로 바꿨다. visibility:collapse도 author stylesheet 경계에서 거부하도록 보완했다. 상세 실행 및 제한은 [runtime evidence](../spec/internal/evidence/c11-4-overflow-auto-minimum-runtime-2026-10-29/README.md), [구현/runtime 20관점](../spec/internal/evidence/c11-4-overflow-auto-minimum-runtime-2026-10-29/implementation-runtime-review-20.md)에 둔다.

앞선 PR #764는 2026-10-28T14:30:20Z에 Rebase and merge되어 db9b51c6ae72328c408b2a0ed15d602a90938d47가 main merge SHA가 됐다. PR #765는 2026-10-28T16:52:08Z Rebase and merge됐고 merge SHA는 31103049d4a99a70fb97d3dfc58c96ac67cb173c다. merge 당시 head의 check run과 status context는 없었고 main required status checks도 설정되지 않아 CI 통과로 기록하지 않는다. 이번 PR #765는 C11.4 미완료/72/213을 유지한다. 일반 CSS admission, clipping·scroll 동작, 전체 WPT, 실기기/hardware GPU와 iOS compositor 이후 pixel equality를 주장하지 않는다.
