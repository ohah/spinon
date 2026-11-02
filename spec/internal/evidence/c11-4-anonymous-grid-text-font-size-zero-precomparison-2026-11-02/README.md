# C11.4 anonymous Grid text `font-size: 0` 사전 비교

## 상태와 범위

구현 전 계획을 위한 Chrome oracle 및 독립 계산이다. Spinon 구현 결과, WPT 적합성, glyph paint, PixelCopy/Metal pixel parity를 뜻하지 않는다.

고정 입력은 [fixture](../../../../tests/fixtures/css/c11/grid-track-sizing-c114-font-size-zero-precomparison.html)와 pinned [Ahem font](../../../../tests/fixtures/resources/c15/Ahem.ttf)다. 화면 바깥 설명 라벨은 비교 대상 Grid 밖에 있고 각 Grid root의 입력 크기와 CSS는 바꾸지 않는다. Chrome `156.0.8078.80` HeadlessChrome을 Agent Browser로 실행했으며 Agent Browser 프로세스는 실행 뒤 종료했다. Viewport는 1280×633, DPR 1이다. `document.fonts.check('10px Ahem')`은 true였다.

## 규범 근거와 독립 계산

[CSS Fonts Level 4, 2026-09-06 Working Draft §2.5](https://www.w3.org/TR/2026/WD-css-fonts-4-20260906/#font-size-prop)은 `font-size`의 `<length-percentage>`를 `[0,∞]`로 정의하고 음수 길이·퍼센트를 invalid로 둔다. 이는 해당 Working Draft의 근거이며 최종 CSS Fonts 4 Recommendation이라고 주장하지 않는다. Chrome의 parser/computed-style 동작은 이 규범 문구와 따로 아래 표에 기록했다.

[CSS Inline Layout Level 3 §5.1](https://www.w3.org/TR/css-inline-3/#line-height)은 2024-12-18 Working Draft에서 absolute `<length>` line-height를 preferred line height로 사용한다고 정의한다. [CSS Grid Layout Level 2 §6](https://www.w3.org/TR/css-grid/#grid-items)는 2025-03-26 Candidate Recommendation Draft에서 공백만 있는 text sequence가 Grid item으로 렌더링되지 않는다고 정의하고, [§8.5](https://www.w3.org/TR/css-grid/#auto-placement-algo)는 anonymous Grid item의 auto-placement를 규정한다. 이 evidence는 최종 상호운용 결과를 주장하지 않고, 고정 Chrome 관측과 독립 계산에 사용한 명세 기대값으로 구분한다.

기대값은 Chrome 출력을 복사하지 않고 pinned Ahem font table과 CSS sizing 입력에서 계산한다. [`font-size` 양수 선행 slice](../../../../plan/c11-4-anonymous-grid-text-computed-font-size.md#사전-비교와-독립-기대값)에서 확인한 `head.unitsPerEm=1000`, `hmtx`의 `X`와 space advance 1000 units를 재사용한다. 따라서 computed `font-size:0px`에서 `XX XX`의 모든 glyph/space advance와 min-/max-content inline contribution은 0px다. 명시 `line-height:10px`은 그대로이므로 해당 text line의 auto row는 10px이다.

Auto-placement scene에서는 direct text가 zero inline contribution이어도 non-whitespace anonymous Grid item으로 먼저 배치된다. 다음 5×3px box가 y=10px에서 시작한다는 관측은 첫 번째 item이 첫 cell을 차지했음을 보여준다. 이 frame은 anonymous item box를 직접 관측한 값이 아니며 glyph paint도 아니다.

## 구현 전 runtime route 확인


C11.4 fixture는 `RuntimeCalculationProfile::BlockFlexGridTrackSizing`에서 `prepare_c11_4_anonymous_text` 및 `compute_runtime_block_flex_grid_track_sizing_style_layout_with_text_paragraphs`를 사용한다([입력 dispatch](../../../../crates/spinon-runtime/src/session/ua_cascade/runtime_layout.rs#L668), [layout projection](../../../../crates/spinon-runtime/src/session/ua_cascade/runtime_layout.rs#L1051)). `root_html_default_render::prepare`는 별도 `RootHtmlDefaultRender` profile에서만 실행된다([profile dispatch](../../../../crates/spinon-runtime/src/session/ua_cascade/runtime_layout.rs#L575)). 따라서 이 C11.4 fixture 경로는 `rasterize_glyphs`를 호출하지 않는다. 이는 소스 정적 경로 확인이며 Parley zero-size 처리, runtime 동작 또는 glyph rendering 근거가 아니다.

## Chrome 관측

| 장면 | computed style / tracks | root frame | text 또는 child 관측 |
|---|---|---:|---|
| `min-content` | `font-size:0px`, `line-height:10px`, columns `0px`, rows `10px` | 100×10px | text Range 0×0px, fragment 1개 |
| `max-content` | `font-size:0px`, `line-height:10px`, columns `0px`, rows `10px` | 100×10px | text Range 0×0px, fragment 1개 |
| `zero-text-before-box` | columns `5px`, rows `10px 3px` | 100×13px | direct text Range 0×0px; 다음 box는 root-relative `(0,10)`, 5×3px |
| `whitespace-only` | columns `none`, rows `0px` | 100×0px | element child 없음, text Range fragment 0개 |
| `negative-declaration` | authored `font-size:-1px`는 invalid, inherited computed `10px`; columns `20px`, rows `10px` | 100×10px | text Range 20×10px |

별도 parser control에서 `CSS.supports('font-size','0px')`는 true, `CSS.supports('font-size','-1px')`는 false였다. `-0px`도 허용되며 CSSOM specified/computed value가 `0px`으로 정규화되고 direct text Range는 0×0px였다.

Raw machine-readable output은 [chrome156-observation.json](./chrome156-observation.json)에, Agent Browser 화면은 [chrome156-font-size-zero.png](./chrome156-font-size-zero.png)에 있다. Range rectangle은 text fragment 정보이며 anonymous item frame 대용으로 쓰지 않는다. Screenshot은 장면·화면 위치 확인용이지 pixel-equivalence evidence가 아니다.

## 재현 및 파일 해시

저장소 루트에서 `shasum -a 256 -c spec/internal/evidence/c11-4-anonymous-grid-text-font-size-zero-precomparison-2026-11-02/SHA256SUMS`를 실행한다. Manifest는 fixture, Ahem asset, raw Chrome observation, screenshot bytes를 고정한다.
