# C11.4 · mixed flexible/fixed track의 `visibility:hidden` item

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [직전 `display:none` mixed-track slice](./c11-4-mixed-flex-hidden-item.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**구현 전 비교:** [Chrome 156 관측과 독립 기대값](../spec/internal/evidence/c11-4-mixed-flex-visibility-hidden-precomparison-2026-10-28/README.md) · [관측 JSON](../spec/internal/evidence/c11-4-mixed-flex-visibility-hidden-precomparison-2026-10-28/chrome156-observation.json) · [기대값](../spec/internal/evidence/c11-4-mixed-flex-visibility-hidden-precomparison-2026-10-28/expected.json)

**계획 검토:** [서로 다른 실패 관점 20개](./c11-4-mixed-flex-visibility-hidden-plan-review-20.md)

**계획 PR diff 검토:** [별도 실패 관점 30개](./c11-4-mixed-flex-visibility-hidden-pr-diff-review-30.md)

**계획 PR 이미지 후속 검토:** [Agent Browser image-load 5관점](./c11-4-mixed-flex-visibility-hidden-pr-diff-review-30-addendum.md)

## 결손과 목표

직전 [PR #734](https://github.com/ohah/spinon/pull/734)는 exact direct HTML `<span style="display:none">`의 텍스트가 mixed flexible/fixed track sizing에서 빠지는 두 fixed scene을 Columns와 Rows에서 확인했다. 해당 adapter의 `visibility:hidden`은 의도적인 거부 control이다. CSS 2.1 §11.2에 따르면 `visibility:hidden` 상자는 생성되어 레이아웃에 계속 참여하고 그려지지 않는다. 이는 `display:none`과 반대 결과를 내야 한다.

이 계획은 동일한 `minmax(min-content,1fr) 40px minmax(0px,2fr)` topology에 hidden 15-character Ahem text와 visible 10-character Ahem text를 같은 첫 track에 겹쳐 놓는다. Columns/Rows 각 축에서 `visibility:hidden` item도 150px min-content contribution과 box frame을 유지하고, visible control과 used track이 같은지 확인한다. 별도 visible controls는 hidden item의 magenta background가 뒤에 놓인 파란 item 위에 실제로 그려지는 positive surface control이다. 범위는 C11.4 opt-in fixture adapter·검증 harness뿐이다. 일반 CSS admission, 공개 API, 전체 `visibility` 지원 또는 C11.4 완료를 주장하지 않는다.

## 규범 및 사전 비교

- CSS 2.1 §11.2: `visibility:hidden`은 box를 투명하게 렌더링하지만 layout에 영향을 유지한다([W3C 규범](https://www.w3.org/TR/CSS21/visufx.html#visibility)).
- CSS Grid Layout Module Level 1, 2025 CRD §11.5.1·§11.7·§11.7.1: item min-content contribution과 flexible track freeze/recalculation을 적용한다([W3C 규범](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-flex-tracks)).
- Chrome 156 observation은 [사전 비교](../spec/internal/evidence/c11-4-mixed-flex-visibility-hidden-precomparison-2026-10-28/README.md)와 [고정 HTML](../tests/fixtures/css/c11/grid-mixed-flex-visibility-hidden-c114-precomparison.html)에 있다. 규범 기대값은 browser output에서 역산하지 않는다.

## 고정 입력과 독립 기대값

모든 scene은 root 크기 `300×120px`(Columns) 또는 `120×300px`(Rows), fixed track `40px`, target-axis gap `10px` 두 개, 첫 `minmax(min-content,1fr)` track, 마지막 `minmax(0px,2fr)` track으로 제한한다. 각 root의 direct child 순서는 visible A text, 15-character span, natural PNG image, C text다. A와 span은 정확히 첫 cell에 겹친다. Span은 `visibility:hidden`이거나 positive control에서만 `visibility:visible`이다. Rows의 A와 span은 `vertical-rl` Ahem 10px text다.

Definite target axis에서 flexible tracks에 남는 크기는 `300 - 40 - 2×10 = 240px`이고 최초 fraction은 `240/(1+2) = 80px`이다. A의 min-content contribution은 `100px`, span의 것은 `150px`이다. 첫 1fr track은 min-content base `150px`에서 freeze된다. 남은 `90px`은 2fr track으로 간다. 따라서 hidden과 visible control 모두 target-axis tracks `[150,40,90]px`, 남은 free space `0px`을 가져야 한다.

정확한 scene ID, root/item frame, CSS used tracks, display/visibility 값, screenshot sample 좌표와 RGB는 [expected.json](../spec/internal/evidence/c11-4-mixed-flex-visibility-hidden-precomparison-2026-10-28/expected.json)에 고정했다.

| Scene | 축 | Span visibility | 기대 tracks | 표면 표본(root-local) |
| --- | --- | --- | --- | --- |
| `column-visibility-hidden` | Columns | `hidden` | `[150,40,90]px` | `(120,60)` 파랑 `#2864dc` |
| `column-visibility-visible-control` | Columns | `visible` | `[150,40,90]px` | `(120,60)` 보라 `#8e44ad` |
| `row-visibility-hidden` | Rows | `hidden` | `[150,40,90]px` | `(60,125)` 파랑 `#2864dc` |
| `row-visibility-visible-control` | Rows | `visible` | `[150,40,90]px` | `(60,125)` 보라 `#8e44ad` |

기대 frame은 root `(0,0,300,120)` 또는 `(0,0,120,300)`, 첫 cell의 A·span `(0,0,150,120)` 또는 `(0,0,120,150)`, PNG `(160,0,24,16)` 또는 `(0,160,24,16)`, C `(210,0,90,120)` 또는 `(0,210,120,90)`이다. A와 span이 겹치므로 DOM order는 고정한다. Span이 A보다 나중이다. Hidden scene에서 span이 잘못 그려지면 sample이 보라색이 되고, visible control sample은 실제 표본 지점이 맞는지 확인한다.

## 제한 fixture 계약

- `RuntimeBlockFlexGridTrackSizingV1` 기존 opt-in 경로에 네 scene만 추가한다. 새 public API, ABI, dependency 또는 일반 stylesheet admission을 만들지 않는다.
- Runtime root는 document-root 직속 HTML `div`; child는 정확히 네 개의 direct HTML element이며 namespace/tag/ID/source order, PNG source·decode/natural size, direct text, placement, axis, font, style와 같은 revision을 고정한다.
- Hidden scene의 span은 `display:block`, `visibility:hidden`, 명시적인 `grid-column:1 / 2; grid-row:1 / 2`이고 intrinsic text input에 들어간다. Visible control은 오직 해당 두 fixed control ID에서 `visibility:visible`을 허용한다. Hidden ID에서 visible로 값을 바꾸거나 control ID에서 hidden으로 바꾸는 혼동은 거부한다.
- Hidden span과 A의 `Minimum`/`MinContent`/`MaxContent` contribution trace는 owner `NodeId`, axis, span, value를 정확히 연결한다. 두 항목이 겹치는 first track은 `150px` min-content base를 고르고, final flexible track은 `[150,40,90]px`이어야 한다. 모든 pass의 track/gutter 배열 길이, stage 순서, freeze와 free-space 값을 비교한다. Chrome quantization은 규범 stage oracle로 쓰지 않는다.
- Hidden span은 full-size item frame과 layout trace에 있어야 하지만 draw 목록과 hidden sample pixel에는 없어야 한다. Visible control은 같은 위치에서 보라색 draw sample을 내야 한다. Layout frame 존재를 painted box로 해석하지 않는다.
- `visibility:collapse`, 다른 visibility/display 값, `display:none`, 임의 hidden subtree, 변형 text/font/writing-mode, child 추가/누락/재정렬, 다른 placement/track/gap/root size/axis/profile, 미등록 scene ID 및 stale revision은 정확한 failure로 fail closed한다. 기존 #734 `display:none` two-scene와 PR #729의 여덟 visible mixed-track scene은 회귀 대조군으로 유지한다.
- Android API 37 ARM64 16 KB AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 기존 8 scene과 새 4 scene을 동일 source revision으로 각각 12/12 실행한다. Frame·track/contribution trace·scene/node identity·draw omission/presence·presentation completion을 검사한다. Android PixelCopy와 iOS compositor 제출 전 Metal drawable에서 hidden/visible root-local sample을 각각 확인한다. 이 결과를 실기기, hardware GPU, iOS compositor-after 또는 전체-screen pixel parity로 확대하지 않는다.
- 기본 product FFI archive는 신규 fixture marker/symbol이 없어야 하고 fixture-test positive control에는 포함한다. Static archive 경계는 APK/IPA 전체 exclusion을 증명하지 않는다. Focused Rust/FFI test, `cargo fmt --all -- --check`, 관련 Clippy, Android/iOS Simulator build와 `git diff --check`를 실행한다.
- 실행 전 simulator boot 상태를 기록한다. 이미 booted device는 유지하고 이 작업이 직접 boot한 simulator만 종료한다. PR 본문에는 full commit SHA가 붙은 Android/iOS screenshot link를 넣고 Agent Browser로 image load/dimensions를 확인한 다음 browser session과 process를 종료한다. Terminal Browser는 사용하지 않는다.

## 계획·구현·병합 게이트

1. 구현 전 비교, 계획 실패 관점 20개와 계획 PR diff 30관점 검토를 완료하고 계획 PR을 Rebase and merge한다.
2. 병합된 계획을 기준으로 같은 checkout에서 제한 fixture·adapter·matcher를 구현한다. 구현/runtime에 별도 새 실패 관점 20개, PR 변경분에 별도 30개를 수행한다. 결함 발견 시 영향받은 matcher, Rust tests와 두 Simulator를 다시 실행한다.
3. Implementation PR merge 후 실제 코드·검증·범위를 계획, 상위 C11.4 plan, 공개 roadmap에 반영한다. 공식 `spec/STATUS.md` C11.4 checkbox와 완료 `72/213`은 바꾸지 않는다.
4. Tailnet roadmap을 현재 main으로 갱신한 뒤 Agent Browser로 링크와 화면을 확인하고 실행에 사용한 Agent Browser session/process를 종료한다.

## 구현과 병합 결과

구현 PR [#738](https://github.com/ohah/spinon/pull/738)은 2026-10-28T06:06:39Z Rebase and merge됐고 merge SHA는 `bf4ab21eebc2f3739a7ae61de6511b92733b9464`다. 계획한 네 scene ID를 기존 opt-in route에 추가해, `visibility:hidden` span이 150px min-content와 전체 layout frame을 유지하면서 paint에서 제외되는지 확인했다. Android API 37 ARM64 16 KB AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 기존 10개 control을 포함해 각각 14/14 scene이 통과했다. Hidden/visible overlap 및 fixed-gap 표본은 PixelCopy와 Metal readback에서 exact match했고, 독립 verifier의 15개 negative control도 모두 거부됐다. 실행 manifest·screenshot·hash는 [runtime evidence](../spec/internal/evidence/c11-4-mixed-intrinsic-flex-fixed-runtime-2026-10-28/README.md)에 있다.

Focused Rust tests, `cargo fmt --all -- --check`, 영향받은 fixture FFI와 library Clippy, 기본/opt-in macOS ARM64 release static archive 경계 검사가 통과했다. `spinon-runtime`/`spinon-style-to-render --all-targets` Clippy는 변경하지 않은 `generic_direct_text_image/fixture.rs:311`의 기존 `clippy::map_identity`에서 실패했다. GitHub PR check rollup은 비어 있었다. 계획 밖의 실기기, hardware GPU, 일반 CSS admission, edge overflow/clipping, 전체 WPT와 iOS compositor-after pixels는 확인하지 않았다. C11.4 상위 checkbox는 미완료이며 공식 `72/213`을 유지한다. 구현과 PR diff의 별도 실패 검토는 [20관점](../spec/internal/evidence/c11-4-mixed-intrinsic-flex-fixed-runtime-2026-10-28/visibility-hidden-implementation-runtime-review-20.md) 및 [30관점](../spec/internal/evidence/c11-4-mixed-intrinsic-flex-fixed-runtime-2026-10-28/visibility-hidden-pr-diff-review-30.md)에 있다.

### 병합 후 source tracking 복구

PR #738 merge tree에는 부모가 `#[path]`로 참조하는 `visibility_hidden.rs`가 빠져 있었다. 당시 로컬의 동일 파일이 미추적 상태로 남아 fixture test와 simulator 빌드에 사용됐다. 후속 PR [#743](https://github.com/ohah/spinon/pull/743)은 이 파일 내용을 변경하지 않고 Git에 추가했으며, 2026-10-28T06:50:44Z에 Rebase and merge됐다(SHA `925113bde86055362c89fa32d505690044a26313`). 병합된 `main`의 focused C11.4 test는 1/1 통과했고 누락 복구 근거는 [별도 기록](../spec/internal/evidence/c11-4-pr738-runtime-source-fix-2026-10-28/README.md)에 있다. 전체 workspace all-target Clippy에서는 기존 FFI 테스트 initializer 누락 필드와 `clippy::map_identity`가 남아 있다. 이 수정은 runtime behavior와 공식 C11.4 상태 또는 `72/213`을 바꾸지 않는다.

## 제외

Visibility inheritance, hidden ancestor와 visible descendant, `visibility:collapse`, animation/mutation, arbitrary item tree, 일반 CSS/Grid 지원, 전체 WPT, product support 선언, 실기기/hardware GPU, iOS compositor-after pixels, 전역 renderer parity와 전체 C11.4는 범위 밖이다.
