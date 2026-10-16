# 0070 · C15 제한 inline text layout

**상태:** 초안 · **내부 인터페이스 숫자 버전:** `0.1.0` 고정<br>
**작업 계획:** [C15 인라인 텍스트 shaping과 줄 상자](https://github.com/ohah/spinon/blob/main/plan/c15-inline-text-layout.md)<br>
**기준 관찰:** [Chrome precomparison](./evidence/c15-inline-text-precomparison-2026-10-17/README.md) · [plain inline fragment](./evidence/c15-inline-text-precomparison-2026-10-17/plain-inline-fragment-observation.md)<br>
**구현 근거:** [C15.1 Rust shaping/layout](./evidence/c15-1-inline-text-layout-2026-10-17/README.md) · [C15.2a 중첩·장식 inline geometry](./evidence/c15-2a-inline-text-layout-2026-10-17/README.md)

이 문서는 앱 작성자용 HTML/CSS 지원 API가 아니라 `spinon-text-layout`과 `spinon-layout` 사이의 내부 호출 계약을 기록한다. 이 구현은 text geometry snapshot만 제공한다. `spinon-style-to-layout`의 runtime projection과 renderer는 text node를 계속 거부하므로 현재 앱에서 글자를 그리거나 CSS text 지원을 주장하지 않는다.

## 호출 흐름과 소유권

1. 호출자는 완전한 `LayoutStyle` map, 내부 `ParagraphStyle` map, 그리고 `HostDocumentSnapshot`을 `LayoutInput::from_host_document_with_text_paragraphs(...)`에 전달한다. paragraph key는 visible HTML element여야 하며, C15.1/C15.2a profile에서 `display:block`이어야 한다.
2. HostDocument adapter는 paragraph의 text node와 inline HTML `span`을 DOM source order대로 수집한다. C15.1은 한 단계 plain span을, C15.2a는 최대 두 단계 span과 고정 px border/padding을 받는다. `display:none` element subtree와 빈 text node는 line geometry에 넣지 않는다. 숨긴 element에는 style이 필요하며 unpaired UTF-16 surrogate는 `InvalidTextEncoding(node)`로 닫는다.
3. 호출자는 명시적으로 등록한 폰트와 `TextLayoutEngine`을 `TaffyLayoutEngine::compute_with_text_layout_engine(...)`에 전달한다. Taffy measure callback은 definite content width에서 실제 shaping을 수행한다.
4. 최종 Block content width가 measure width와 bit-exact하게 같을 때만 같은 계산의 snapshot을 `LayoutOutput.text_layouts`에 넣는다. width가 다르거나, auto-height가 snapshot 높이와 `0.05 CSS px` 이상 다르면 layout 계산은 실패한다.

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
| Taffy가 definite width를 주지 않음 | `IndefiniteTextAvailableWidth(node)` |
| final content width와 measure key가 다름 | `TextLayoutWidthMismatch(node)` |
| auto-height 결과가 shaped height와 다름 | `TextLayoutHeightMismatch(node)` |
| width/style/font data/glyph mapping이 유효하지 않음 | `TextLayout(TextLayoutError)`의 구체 variant |
| inline style owner가 없거나 border/padding 값·합계가 유효하지 않음 | `TextLayout(InvalidInlineBoxStyle(node))` |
| decorated inline owner에 visible text range가 없음 | `TextLayout(InlineBoxWithoutText(node))` |
| 직접 `ParagraphInput`에 세 단계 이상 ancestry 또는 경로가 충돌함 | `TextLayout(UnsupportedInlineDepth(node))` 또는 `TextLayout(InvalidInlineHierarchy(node))` |
| Parley가 역전·범위 밖·UTF-8 경계가 아닌 line/run/cluster byte range를 반환함 | `TextLayout(InvalidGlyphLayout(node))` |

부분 frame map이나 fallback font으로 성공 처리하지 않는다. `white-space` 및 `word-break` 미지원 CSS 값을 Rust enum으로 만들 수 없으며, 상위 projection이 허용하지 않는 값은 해당 단계에서 node/property 오류로 닫아야 한다.

## 제외 범위와 소비자 경계

- 이 계약은 style-to-layout cascade 연결이나 `spinon-style-to-render` runtime text node admission을 추가하지 않는다. 앱 runtime, Android/iOS Simulator, 실제 glyph ink, overflow pixel, clip, hit-test, selection/caret, accessibility는 확인되지 않았다.
- C15.2a 밖의 mixed-font baseline, mixed block/inline anonymous box, inline replaced object, float, ruby, vertical writing, bidi/script shaping은 제외한다. border/padding ink rasterization과 painter는 C15.3, CSS font properties/cascade와 `@font-face`는 C16이다.
- C15 parent/C16/C19 완료를 이 계약만으로 표시할 수 없다. 실행 근거와 상위 상태는 [공식 상태 대장](../STATUS.md)을 따른다.
