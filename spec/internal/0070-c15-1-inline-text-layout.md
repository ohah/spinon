# 0070 · C15 제한 inline text layout

**상태:** 초안 · **내부 인터페이스 숫자 버전:** `0.1.0` 고정<br>
**작업 계획:** [C15 인라인 텍스트 shaping과 줄 상자](https://github.com/ohah/spinon/blob/main/plan/c15-inline-text-layout.md)<br>
**기준 관찰:** [Chrome precomparison](./evidence/c15-inline-text-precomparison-2026-10-17/README.md) · [plain inline fragment](./evidence/c15-inline-text-precomparison-2026-10-17/plain-inline-fragment-observation.md)<br>
**구현 근거:** [C15.1 Rust shaping/layout](./evidence/c15-1-inline-text-layout-2026-10-17/README.md) · [C15.2a 중첩·장식 inline geometry](./evidence/c15-2a-inline-text-layout-2026-10-17/README.md) · [C15.2b mixed baseline·anonymous flow](./evidence/c15-2b-mixed-baseline-anonymous-flow-2026-10-17/README.md) · [C15.3 Unicode shaping slice](./evidence/c15-3-unicode-shaping-impl-2026-10-17/README.md)

이 문서는 앱 작성자용 HTML/CSS 지원 API가 아니라 `spinon-text-layout`과 `spinon-layout` 사이의 내부 호출 계약을 기록한다. 이 구현은 text geometry snapshot만 제공한다. `spinon-style-to-layout`의 runtime projection과 renderer는 text node를 계속 거부하므로 현재 앱에서 글자를 그리거나 CSS text 지원을 주장하지 않는다.

## 호출 흐름과 소유권

1. 호출자는 완전한 `LayoutStyle` map, 내부 `ParagraphStyle` map, 그리고 `HostDocumentSnapshot`을 `LayoutInput::from_host_document_with_text_paragraphs(...)`에 전달한다. 서로 다른 inline font metric이 필요하면 `InlineRunStyle` map도 함께 전달한다. Root HTML의 collapsible whitespace profile은 두 map을 `HostTextStyleProfile { paragraphs, inline_runs }`로 묶어 layout constructor에 전달한다. paragraph key는 visible HTML element이며 `display:block`이어야 한다.
2. HostDocument adapter는 paragraph의 text node와 inline HTML `span`을 DOM source order대로 수집한다. C15.1은 한 단계 plain span을, C15.2a는 최대 두 단계 span과 고정 px border/padding을 받는다. `display:none` element subtree와 빈 text node는 line geometry에 넣지 않는다. 숨긴 element에는 style이 필요하며 unpaired UTF-16 surrogate는 `InvalidTextEncoding(node)`로 닫는다.
3. 호출자는 명시적으로 등록한 폰트와 `TextLayoutEngine`을 `TaffyLayoutEngine::compute_with_text_layout_engine(...)`에 전달한다. Taffy measure callback은 definite content width에서 실제 shaping을 수행한다.
4. 최종 Block content width가 measure width와 bit-exact하게 같을 때만 같은 계산의 snapshot을 `LayoutOutput.text_layouts`에 넣는다. width가 다르거나, auto-height가 snapshot 높이와 `0.05 CSS px` 이상 다르면 layout 계산은 실패한다.

Grid intrinsic track sizing 준비 단계는 같은 `ParagraphInput`·font registry revision에서 `TextLayoutEngine::intrinsic_inline_sizes(...)`를 호출한다. Parley는 한 번 shape한 layout을 0px와 무제한 폭으로 다시 줄바꿈하며, 결과는 min-content/max-content inline advance와 각 line-break 폭의 block height를 보존한다. 결과는 font revision과 registry identity를 함께 보존한다. cloned engine은 같은 registry identity를 공유하지만 독립 registry의 같은 숫자 revision은 다른 입력으로 취급한다. definite-width `TextLayoutSnapshot`과 별도 값이고, `CalcLayoutTree`는 입력 node마다 한 번 계산한 결과를 해당 tree 계산 동안 캐시한다. 이 측정 primitive가 Grid track sizing을 활성화하거나 text rendering을 추가하지는 않는다.

`LayoutTextSnapshot`은 `LayoutInputRevision`, 원본 `ParagraphInput`, shaped `TextLayoutSnapshot`을 함께 소유한다. `LayoutInputRevision`은 HostDocument generation/document/render-tree, `StyleRevision`, `EnvironmentRevision`을 보존한다. 안쪽 text snapshot은 `FontRegistryRevision`, exact available width, normalized text와 source map, line/run/glyph metrics, line별 inline fragment geometry를 보존한다. `LayoutOutput`과 각 text entry는 같은 revision이어야 한다.

`TextLayoutEngine::new_without_system_fonts()`는 system font discovery를 끈 registry를 만든다. `register_font(family, bytes)`는 호출자가 제공한 폰트 데이터를 소유형 Blob으로 복사하고 성공할 때마다 checked `FontRegistryRevision`을 증가시킨다. 비동기 font loading, system fallback, CSS font matching은 없다. layout 계산은 engine clone의 독립 `LayoutContext`를 사용하며 결과는 호출자에게 소유권을 넘기는 immutable snapshot이다.

## C15.1/C15.2a 입력 profile

- `ParagraphStyle`: non-empty `font_family`, finite positive `font_size`, finite positive 내부 `line_height`, `WhiteSpaceMode::{Normal, NoWrap, PreLine}`, `WordBreakMode::{Normal, BreakAll}`.
- paragraph source: `TextSource { node, inline_ancestors, text }`. HostDocument 변환은 DOM UTF-16을 strict UTF-8로 변환하며 invalid surrogate를 거부한다.
- C15.1 inline element: paragraph 직속의 한 단계 HTML `span`; `LayoutDisplay::Inline`을 명시하고 나머지 `LayoutStyle` 필드는 기본값이어야 하며 paragraph의 font/layout style을 상속한다. 일반 `LayoutStyle::default()`의 `Flex` 표시와 다른 visible display는 inline으로 평탄화하지 않고 `UnsupportedTextParagraph(paragraph)`로 거부한다. 중첩 span, visible non-span, border·padding·margin·size·alternate direction 등 non-default box style도 거부한다.
- C15.2a inline element: 최대 두 단계 HTML `span`; 각 span은 `LayoutDisplay::Inline`이며 나머지 style은 상속형 기본값이어야 한다. 고정 px의 nonnegative border/padding만 허용하며 margin, non-px padding, 음수 또는 overflow 합계, visible non-span, 더 깊은 nesting, non-baseline `vertical-align`은 `UnsupportedTextParagraph`로 닫는다. 장식된 owner에 visible text 범위가 없으면 `InlineBoxWithoutText(node)`다. 지원되는 span의 border/padding은 줄바꿈 입력과 line별 fragment에 포함하지만 vertical edge가 line box height를 늘리지는 않는다.
- 두 profile 모두 `display:none` subtree를 unsupported-value 판정보다 먼저 제외한다. `LayoutDisplay::Inline`은 HostDocument의 text-only source 표시이며 독립 Taffy box로는 거부한다.
- shaping: Parley `0.12.0`을 고정 사용한다. overflow-wrap은 `normal`; text-align은 start; horizontal default direction만 사용한다. line-height는 CSS parser 결과가 아닌 fixture scalar다.
- whitespace source map: `normal`/`nowrap`은 collapsible run을 하나의 output space로 줄이고 각 source code point를 같은 output byte range에 연결한다. 줄 경계에서 제거된 leading/trailing space는 zero-length output range에 남긴다. `pre-line`은 segment break를 `\n`으로 보존하며 CRLF 양쪽 source range를 모두 기록한다.
- source map의 `source_utf16`은 원본 DOM text node 안의 UTF-16 범위, `paragraph_utf8`은 normalized paragraph string 안의 UTF-8 byte 범위다. 한 glyph/cluster의 source는 node와 inline ancestor owner를 잃지 않는다.
- line box와 glyph output은 CSS px로 기록한다. `TextLine`은 advance/hanging advance, top, height, baseline을 보존하고 `TextRun`은 font metrics, source ranges, glyph ID와 origin/advance를 보존한다. C15.2a는 1 CSS px scale의 Parley `quantize=true`를 써 Blink의 정수 ascent/descent에 맞춘 line/glyph baseline을 기록하되 shaped advance와 `TextRun.font_metrics`의 원본 분수값은 보존한다. `TextRun.font_family`는 profile의 요청 family이며 실제 fallback face 식별은 이 slice가 보장하지 않는다. 현재 oracle은 Ahem/Roboto가 제공하는 고정 ASCII 문자만 사용한다. C15.1 `InlineFragment`는 content rectangle이며 C15.2a decorated owner는 border/padding을 더한 rect를 line index별로 보존한다. 줄마다 별도 fragment를 유지하며 union rectangle로 합치지 않는다.

## C15.2b mixed font와 anonymous block flow

- `InlineRunStyle`은 inline owner별 고정 `font_family`, 양수 `font_size`, 양수 `line_height`다. owner ancestry의 가장 안쪽에 지정된 값을 해당 text source에 적용한다. 요청 family는 `TextLayoutEngine`의 명시적 font registry에 있어야 하며 missing family나 유효하지 않은 metric은 오류로 닫는다. CSS cascade, font fallback 선택·식별과 비동기 loading을 제공하지 않는다.
- `TextRun`은 적용된 요청 family·size·line-height 및 paragraph 좌표계의 run baseline을 보존한다. line box baseline을 공유하더라도 glyph run별 font metrics와 line-height를 잃지 않는다. inline fragment의 vertical rect는 run baseline과 해당 font의 quantized ascent/descent로 계산한다.
- Paragraph의 direct children에서 HTML inline `span`과 `display:block` child가 섞이면 HostDocument adapter는 inline source를 `TextFlowItem::Paragraph`로 나눠 direct Block node와 함께 source order로 만든다. text를 가진 direct Block child마다 별도 `ParagraphStyle` entry가 있어야 한다. Block child는 중복될 수 없고, `LayoutInput` visible direct child 순서와 완전히 같아야 한다. out-of-flow child, graph 밖 child와 inline span 내부의 Block은 거부한다.
- `CalcLayoutTree`는 inline paragraph마다 내부 synthetic Block leaf를 만든다. 이 node는 external DOM node로 노출하지 않는다. Taffy가 결정한 owner content width에서 shaping/measuring하고, 출력 `LayoutOutput.text_flow_layouts[owner]`는 item index, paragraph border-box frame, 동일 revision의 input/snapshot을 포함한다. frame 위치는 owner border-box 기준이며 Taffy가 이미 반영한 owner border/padding offset을 다시 더하지 않는다.
- C15.2b oracle은 Ahem `10px/10px`과 Roboto `20px/24px`의 한 줄 baseline `19px`/line height `24px`, 그리고 width `30px` anonymous flow의 parent `30×40px`, block y `10px`, trailing segment y `20px`/height `20px`다. renderer가 snapshot을 그리거나 부모 경계 밖 ink를 clip한다고 의미하지 않는다.
- `display:none` subtree와 빈 text node 제외, source UTF-16/UTF-8 mapping, per-width measure snapshot 규칙, C15.2a의 최대 두 단계 nesting·decorated fragment 범위는 계속 적용한다.

## C15.3 고정 Unicode shaping profile

- `ParagraphStyle`은 내부 fixture 값 `TextDirection::{Auto, Ltr, Rtl}`, `LineBreakMode::{Normal, Strict}`, `LigatureMode::{Normal, None}`을 추가로 받는다. `LineBreakMode`의 기본값은 Parley 0.12.0의 기존 strict 기본 동작을 유지한다. 이 profile은 CSS cascade/API 연결을 제공하지 않는다.
- `TextLayoutSnapshot`은 요청한 세 shaping mode를 보존한다. `TextRun.is_rtl`은 각 Bidi run의 실제 방향이다.
- `TextGlyph.cluster_text_range`는 normalized paragraph UTF-8의 grapheme 범위이며 ligature glyph에는 이어진 ligature atom 전체 범위를 기록한다. `TextGlyph.sources`는 그 범위에 기여한 모든 DOM text node의 UTF-16 범위와 inline ancestor를 보존한다. 한 grapheme가 adjacent text node를 가로질러도 source 소유자를 합치지 않는다.
- `LigatureMode::None`은 Parley `FontFeatures`의 CSS 설정 문법을 사용해 `liga`, `clig`, `dlig`, `hlig`, `calt`를 끈다. 각 `GlyphRun`의 positioned glyph ID 순서가 시각 순서 `Cluster::glyphs()`와 일치하는지 확인하며, source 범위가 없는 glyph는 `InvalidGlyphLayout`로 닫는다.
- glyph ID `0`은 `.notdef`로 취급해 `MissingGlyphCoverage { paragraph, text_range }`를 반환한다. fixture는 `new_without_system_fonts()` registry에 해당 테스트 글꼴 하나만 등록하며, 시스템 폰트나 다른 fixture family fallback을 허용하지 않는다.
- 구현 근거는 고정 글꼴 Hangul, strict CJK punctuation wrapping, combining mark, common ligature on/off, Arabic RTL와 European digits, Noto Emoji ZWJ sequence를 검사한다. UTF-8 cluster 범위와 DOM UTF-16 source range를 함께 assertion한다.
- locked ICU4X data에는 Chinese/Japanese word segmentation model이 없어 CJK fixture 실행에서 경고가 출력된다. strict line-break punctuation fixture는 통과하지만 한자 어휘 분절·일반 CJK wrapping은 확인하지 않았다.
- C11.4 text intrinsic input slice는 Parley 0.12.0에서 한 번 shape한 paragraph를 0 CSS px와 무제한 폭으로 다시 줄바꿈해 min/max-content advance 및 두 결과의 line height를 반환한다. Ahem oracle은 normal `70/100px`, `break-all` `10/80px`, nowrap `100/100px`이다. C15 inline span의 start/end edge는 각 text boundary가 속한 첫/마지막 min-content line에 한 번씩 반영한다. edge만 놓인 Parley line은 contribution width와 block height에서 중복 계산하지 않는다. Chrome 155 supplemental oracle은 normal `75×20px`, nowrap `110×10px`, break-all `15×80px`, pre-line `45×20px`이다. definite C15 snapshot map은 이 과정에서 읽거나 덮어쓰지 않는다. 일반 CJK 분절, 임의 overflow-wrap, CSS font cascade는 보장하지 않는다. [inline edge oracle](./evidence/c11-4-grid-track-sizing-text-inline-edge-2026-10-19/README.md)
- 이 slice는 pure Rust shaping snapshot과 Grid가 소비할 intrinsic text measurement 입력만 만든다. WGPU glyph raster/pipeline, runtime text-node admission, Block/Flex/Grid intrinsic track sizing 결과, overflow/scroll, paint order, clip, hit-test, Android/iOS Simulator text ink는 아직 제공하거나 확인하지 않는다.

## 실패 계약

| 실패 | 결과 |
| --- | --- |
| paragraph owner가 없거나 element가 아님 | `InvalidHostDocumentRoot` 또는 `UnknownTextParagraph` |
| paragraph가 Block이 아니거나 숨김 조상 아래 있음 | paragraph style을 생략하거나 `UnsupportedTextParagraph`로 닫음 |
| C15.1 visible non-span/nested/decorated span 또는 C15.2a 지원 밖 topology/style | `UnsupportedTextParagraph`에 paragraph node와 이유를 포함 |
| 빈/누락 style, 숨김 element의 누락 style, 알 수 없는 style key | `MissingStyle` 또는 `UnknownStyleNode` |
| visible text가 paragraph profile에 속하지 않음 | `UnsupportedTextNode(node)` |
| invalid DOM UTF-16 | `InvalidTextEncoding(node)` |
| available width가 음수이거나 유한하지 않음 | `TextLayout(InvalidWidth)` · 0px는 유효한 definite width이며 글자 advance overflow를 보존 |
| text input이 있는데 text engine이 없음 | `MissingTextLayoutEngine(node)` |
| Taffy가 intrinsic min/max-content width를 요청함 | 별도 `TextIntrinsicInlineSizes` 결과를 반환하고 definite-width snapshot에는 넣지 않음 |
| final content width와 measure key가 다름 | `TextLayoutWidthMismatch(node)` |
| auto-height 결과가 shaped height와 다름 | `TextLayoutHeightMismatch(node)` |
| width/style/font data/glyph mapping이 유효하지 않음 | `TextLayout(TextLayoutError)`의 구체 variant |
| inline style owner가 없거나 border/padding 값·합계가 유효하지 않음 | `TextLayout(InvalidInlineBoxStyle(node))` |
| decorated inline owner에 visible text range가 없음 | `TextLayout(InlineBoxWithoutText(node))` |
| inline owner의 run style family가 registry에 없거나 metric이 유효하지 않음 | `TextLayout(MissingFontFamily)` 또는 `TextLayout(InvalidStyle)` |
| anonymous flow block item이 중복·누락·순서 불일치·out-of-flow이거나 span 내부에 나타남 | `UnsupportedTextParagraph(owner)` |
| 직접 `ParagraphInput`에 세 단계 이상 ancestry 또는 경로가 충돌함 | `TextLayout(UnsupportedInlineDepth(node))` 또는 `TextLayout(InvalidInlineHierarchy(node))` |
| Parley가 역전·범위 밖·UTF-8 경계가 아닌 line/run/cluster byte range를 반환함 | `TextLayout(InvalidGlyphLayout(node))` |
| 고정 글꼴이 입력 Unicode 범위에 glyph를 제공하지 않음 | `TextLayout(MissingGlyphCoverage { paragraph, text_range })` |

부분 frame map이나 fallback font으로 성공 처리하지 않는다. `white-space` 및 `word-break` 미지원 CSS 값을 Rust enum으로 만들 수 없으며, 상위 projection이 허용하지 않는 값은 해당 단계에서 node/property 오류로 닫아야 한다.

## 제외 범위와 소비자 경계

- 이 계약은 style-to-layout cascade 연결이나 `spinon-style-to-render` runtime text node admission을 추가하지 않는다. C15.2b는 pure Rust geometry/shaping만 검증하며 앱 runtime, Android/iOS Simulator 앱 화면, 실제 glyph ink, overflow pixel, clip, hit-test, selection/caret, accessibility는 확인하지 않았다.
- C15.2b는 고정된 mixed-font baseline과 anonymous block flow fixture만 포함한다. 임의 font cascade, inline replaced object, float, ruby, vertical writing은 제외한다. C15.3 pure Rust shaping fixture는 제한된 bidi/script 입력을 추가하지만 runtime painter, overflow/clip/hit-test 통합은 포함하지 않는다. border/padding ink rasterization과 painter는 C15.3 후속 slice, CSS font properties/cascade와 `@font-face`는 C16이다.
- C15 parent/C16/C19 완료를 이 계약만으로 표시할 수 없다. 실행 근거와 상위 상태는 [공식 상태 대장](../STATUS.md)을 따른다.
