# C11.4 anonymous Grid text computed `font-size: 0` 제한 계획

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [기존 computed font-size slice](./c11-4-anonymous-grid-text-computed-font-size.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** 계획 PR [#1051](https://github.com/ohah/spinon/pull/1051)이 먼저 병합됐다. 구현 PR [#1052](https://github.com/ohah/spinon/pull/1052)은 2026-11-02T10:29:53Z Rebase and merge됐고 merge SHA는 `472c5a6600603b2e69a0d42d5c5a1f68345094f3`이다. check run은 보고되지 않았고 `main` required status checks도 설정되어 있지 않다. 제한 구현과 Android API 37 ARM64 AVD·iPhone 17 Pro/iOS 26.2 Simulator 4/4 실행, default/fixture-test static archive 경계를 확인했다. C11.4/C11 공식 완료는 아니며 `72/213`을 유지한다.

## 목적과 범위

기존 C11.4 anonymous Grid text computed `font-size` slice는 5px 이상 positive finite 값을 받지만, CSS의 유효한 `font-size: 0`을 명시적으로 제외한다. `ParagraphStyle::validate`도 `font_size <= 0.0`을 거부한다. 이 후속은 고정 Ahem anonymous Grid text fixture에서 computed zero font size를 intrinsic contribution 경로로 전달해 0-width min/max-content와 nonzero explicit line-height를 함께 확인한다.

CSS Fonts Level 4의 2026-09-06 Working Draft는 `font-size`의 `<length-percentage>`를 `[0,∞]` 범위로 정의하고 음수 길이는 invalid라고 한다. CSS Inline Layout Level 3 §5.1 Working Draft는 절대 `<length>` `line-height`를 preferred line height로 사용한다고 정의한다. CSS Grid Layout Level 2의 2025-03-26 Candidate Recommendation Draft §6과 §8.5는 공백만 있는 text sequence는 렌더링하지 않고 anonymous Grid item은 auto-placement 된다고 정의한다. 세 문서의 현재 초안 상태를 그대로 밝히며 최종 표준이라고 표현하지 않는다. Chrome observation은 실제 구현 결과를 별도로 기록한다. [CSS Fonts Level 4 §2.5](https://www.w3.org/TR/2026/WD-css-fonts-4-20260906/#font-size-prop) · [CSS Inline Layout Level 3 §5.1](https://www.w3.org/TR/css-inline-3/#line-height) · [CSS Grid Layout Level 2 §6](https://www.w3.org/TR/css-grid/#grid-items) · [§8.5](https://www.w3.org/TR/css-grid/#auto-placement-algo).

새 입력은 기존 `RuntimeBlockFlexGridTrackSizingV1` opt-in fixture 경로만 사용한다. HTML Grid root `div`, width 100px, auto height, zero padding/border/gap, `grid-template-rows:auto`, horizontal-tb/LTR, Ahem 400, computed `font-size:0px`, explicit `line-height:10px`, normal whitespace collapse/wrap, zero spacing, source order와 auto placement를 고정한다. Root의 직접 텍스트는 `XX XX` 또는 box 앞의 `XX`다. Ahem font-size 0에서 glyph advance는 0이지만 explicit line-height 10px은 유지되어야 한다.

계획은 다음 범위를 열지 않는다: 일반 CSS author admission, 임의 font family/weight/style/text/script, percentage·calc font-size, `line-height:normal`, inline element/span, nested Grid, writing-mode 변경, glyph paint/rasterization, C15 `InlineRunStyle`/raster zero-size support, WPT 적합성, 전체 C11.4 또는 C11 완료. computed zero를 새로 admission하는 route는 이 C11.4 exact fixture뿐이다.

구현 전 route 확인에서 이 fixture는 `RuntimeCalculationProfile::BlockFlexGridTrackSizing`으로 `prepare_c11_4_anonymous_text` 및 `compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs` 경로를 탄다. `root_html_default_render::prepare/attach`는 별도 `RootHtmlDefaultRender` 프로필 경로다([프로필 dispatch](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout.rs#L575), [C11.4 text input](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout.rs#L668), [C11.4 projection](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout.rs#L1051)). 그러므로 이 C11.4 fixture의 layout/track 검증은 glyph rasterizer를 호출하지 않으며 no-paint renderer 변경을 요구하지 않는다. 플랫폼 scene의 draw/present 상태는 runtime harness로 확인하되 glyph pixel이나 raster API 지원으로 확대하지 않는다.

## 규범·독립 기대값과 사전 비교

고정 입력, pinned Ahem, Agent Browser 관측, screenshot과 hash는 [사전 비교 evidence](../spec/internal/evidence/c11-4-anonymous-grid-text-font-size-zero-precomparison-2026-11-02/README.md)에 있다. Chrome 156.0.8078.80에서 `CSS.supports('font-size','0px')`는 true, `-1px`는 false였고 `-0px`는 `0px`로 정규화됐다. Ahem font load를 별도로 확인했다.

독립 계산은 Chrome의 used track 출력에서 가져오지 않는다. pinned Ahem의 `head.unitsPerEm=1000`, `hmtx`의 `X`와 space advance 1000 units는 각각 1em이다. 따라서 computed font-size 0에서 `XX XX`의 min-content 및 max-content inline contribution은 모두 `0 × unitsPerEm = 0px`이다. CSS Inline Layout Level 3 §5.1의 절대 `line-height:10px`은 그대로 preferred line height이므로 한 줄의 auto row는 10px이다. CSS Grid §6과 §8.5에 따라 zero-size direct text sequence도 anonymous Grid item이고 whitespace-only sequence는 렌더링되지 않는다. Zero-size direct text run 뒤에 5×3px box가 오면 text item이 첫 automatic-placement cell을 소비하고 box는 다음 auto row의 y=10px에 놓인다.

| 장면 | 규범 기반 독립 기대값 | Chrome 156 관측 |
|---|---|---|
| `min-content` · `XX XX` | columns `[0px]`, rows `[10px]`, root `100×10px`, direct-text Range `0×0px` | columns `0px`, rows `10px`, root `100×10px`, Range `0×0px`·fragment 1개 |
| `max-content` · `XX XX` | columns `[0px]`, rows `[10px]`, root `100×10px`, direct-text Range `0×0px` | columns `0px`, rows `10px`, root `100×10px`, Range `0×0px`·fragment 1개 |
| zero text then fixed box | columns `[5px]`, rows `[10px,3px]`, root `100×13px`, following box `(0,10) 5×3px` | columns `5px`, rows `10px 3px`, root `100×13px`, following box `(0,10) 5×3px`, text Range `0×0px` |
| collapsible whitespace only | anonymous item 없음, columns `none`, explicit auto row `0px`, implicit track 없음 | columns `none`, rows `0px`, element child 없음, Range fragment 0개 |
| invalid `font-size:-1px` | 선언 무효; inherited 10px control만 허용 | `CSS.supports=false`, computed `10px`, columns `20px`, rows `10px` |

Range rect는 zero-size text fragment 관찰값이다. anonymous Grid item box나 glyph paint로 해석하지 않는다. 다음 fixed box의 row와 frame은 source-order auto placement를 관찰하는 별도 값이다.

## 입력 검증 계약

1. Fixture route, root tag/namespace, direct-text topology, text sequence, Ahem font, font weight/style, writing mode, root width/height, line-height, whitespace, tracks, gaps, spacing과 computed-style revision을 기존 C11.4 fixture gate처럼 모두 검증한다. Scene ID만으로 font-size를 정하지 않는다.
2. Root computed serialization은 `0px`, typed `font_size_css_px`는 finite `0.0`, 같은 node와 style revision이어야 한다. CSSOM에서 `-0px`가 `0px`로 계산되므로 IEEE `-0.0`은 computed zero와 일치할 때 허용하고 `-1`, NaN, ±infinity는 layout publication 전에 거부한다.
3. `ParagraphStyle` text-layout validation은 finite zero를 허용하되 음수·비유한 값은 계속 거부한다. `InlineRunStyle`과 C15 rasterizer는 이 계획으로 zero font size를 받지 않는다.
4. Text layout은 zero advance에서 NaN/Inf, 음수 기여, 잘못된 revision 또는 부분 layout을 게시하지 않는다. Explicit `line-height:10px`은 line/row height를 10px으로 유지한다. C11.4 fixture route는 intrinsic/layout 출력과 Grid track을 검증하며 glyph rasterization을 추가하지 않는다.
5. min/max-content positive scene은 모두 기존 `XX XX` + Ahem font-size 0 입력을 사용한다. Auto-placement scene은 direct text 하나와 exact 5×3px block 하나만 허용한다. Whitespace-only scene은 no-item control이다.

## 구현·검증 관문

1. 계획 PR #1051 병합 뒤 같은 checkout에서 최신 `origin/main` 기반 구현 branch를 만들었다. 이 구현은 순차 진행했고 별도 worktree를 사용하지 않았다.
2. CSS Text/Fonts cascade snapshot의 computed `font-size`와 typed px 값, exact Grid owner, root identity, node provenance, layout/font revision을 대조한다. 명시 route 외에는 admission을 추가하지 않는다.
3. Rust tests에서 `ParagraphStyle` zero acceptance와 negative/non-finite rejection, Parley zero-font layout, min/max intrinsic inline `0px`, explicit line height `10px`, zero-text 뒤 block의 second-row frame, whitespace-only no-item 결과를 각각 검사한다. Failure injection 가능한 allocation/normalization 경로가 있으면 부분 output 부재와 같은 engine 복구도 검사한다.
4. Runtime fixture에는 네 개의 positive/control scene과 invalid computed-value negative matrix를 연결한다. `layout=ready`, revision, text source owner, intrinsic contribution, rows/columns trace, item order/frame과 `drawPresented=true`를 검사한다. 이 route의 screen은 layout/present 상태 자료이며 text rasterization·glyph pixels나 0×0 Range를 item box 증거로 쓰지 않는다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 고정 source에서 runtime scenes를 재실행한다. build/app/source hashes와 draw/present를 보존하고 software renderer 및 compositor 경계를 따로 적는다. 기본 release artifact에는 fixture route·symbol·payload가 빠지고 opt-in positive control에는 들어가는지 검사한다.
6. Focused Rust/runtime/FFI tests, fixture-boundary checks, `cargo fmt`, scoped Clippy와 관련 앱 build를 실행한다. 미실행 CI·플랫폼 gate는 통과로 주장하지 않는다.
7. 구현/runtime 변경 뒤 계획 리뷰와 별개인 새 실패 관점 20개를 작성한다. 최종 구현 PR diff에는 프로젝트 규칙에 따라 별도 30개 관점을 검토하고 발견 결함을 수정한 뒤 영향 관점을 재검증한다.
8. 구현 PR에는 내부 계약 0059의 제한 입력·오류·미지원 범위를 기록한다. C11.4/C11 checkbox와 공식 전체 완료 `72/213`은 이 fixture 하나로 변경하지 않는다. 구현 후 `spec/STATUS.md`, 상위 plan, public roadmap을 실제 병합 범위와 맞춘다. Tailnet 미리보기는 구현 문서 병합 뒤 새 파일 hash를 검증해 갱신한다.

## 완료 판정과 남는 한계

- Chrome precomparison과 독립 계산의 track, row height, item order, zero Range fragment 및 negative control이 모두 기록되어야 한다.
- `font-size:0px`에서 min-/max-content intrinsic inline contribution은 0이고 explicit line-height는 10px이어야 한다. Zero-size text item은 다음 box의 auto-placement 순서를 유지해야 한다.
- 음수/non-finite typed size, stale/wrong owner, style/typed mismatch와 잘못된 root/tree는 partial frame나 trace 없이 거부해야 한다.
- Android/iOS runtime geometry·trace·draw/present와 default-vs-opt-in artifact 경계를 확인해야 한다.
- 이 slice는 일반 CSS author admission, C15 `InlineRunStyle`·rasterizer의 zero font-size support, 모든 font/text layout, WPT 적합성 또는 C11.4/C11 전체 완료를 증명하지 않는다. Root HTML renderer no-paint 처리는 glyph bitmap 생성이나 pixel parity 증거가 아니다.

## 근거

- [Chrome 156 사전 비교와 독립 기대값](../spec/internal/evidence/c11-4-anonymous-grid-text-font-size-zero-precomparison-2026-11-02/README.md)
- [Android/iOS 실행·화면·trace·archive 경계](../spec/internal/evidence/c11-4-anonymous-grid-text-font-size-zero-runtime-2026-11-02/README.md)
- [구현/runtime 실패 관점 20개](../spec/internal/evidence/c11-4-anonymous-grid-text-font-size-zero-runtime-2026-11-02/implementation-runtime-review-20.md)
- [병합 기록](../spec/internal/evidence/c11-4-anonymous-grid-text-font-size-zero-runtime-2026-11-02/merge-PR1052.md)
- [병합 후 상태·로드맵 검토 30개](../spec/internal/evidence/c11-4-anonymous-grid-text-font-size-zero-runtime-2026-11-02/postmerge-status-roadmap-review-30.md)
- [고정 Chrome 입력 fixture](../tests/fixtures/css/c11/grid-track-sizing-c114-font-size-zero-precomparison.html)
- [계획 실패 관점 20개](./c11-4-anonymous-grid-text-font-size-zero-plan-review-20.md)
- [계획 PR diff 실패 관점 30개](../spec/internal/evidence/c11-4-anonymous-grid-text-font-size-zero-precomparison-2026-11-02/plan-pr-diff-review-30.md)
