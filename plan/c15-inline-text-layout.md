# C15 · 인라인 텍스트 shaping과 줄 상자

**상태:** 구현 전 계획 · C15와 C16 상태는 미완료 유지<br>
**상위:** [상태 대장 C15/C16](../spec/STATUS.md) · [범위와 적합성](../spec/0001-conformance.md) · [UI 트리와 이벤트](../spec/0002-ui-tree-events.md)<br>
**연결:** C19.3 text painter는 C15/C16 결과를 소비하기 전 완료할 수 없다.<br>
**사전 관찰:** [C15.1 Chrome 155 비교 입력·값·화면](../spec/internal/evidence/c15-inline-text-precomparison-2026-10-17/README.md) · [C15.2 중첩 inline·line box·anonymous block 기준](../spec/internal/evidence/c15-2-inline-text-precomparison-2026-10-17/README.md)

## 목표

HostDocument의 실제 text node와 inline element를 CSS inline formatting context로 정규화하고, 선택한 font로 Unicode text를 shaping한 결과를 줄 상자·baseline·inline fragment·glyph position으로 연결한다. 이를 Block layout의 실제 available inline-size에 연결하고 렌더 snapshot으로 전달한다. 글자 수에 고정 폭을 곱하는 measure, 공백을 ASCII 문자로 대체하는 measure, line count를 추정하는 measure는 허용하지 않는다.

이 계획은 C15 전체의 목표다. 첫 구현은 작은 수직 slice로 나누며, 각 slice를 통과해도 전체 C15·C16·CSS text 지원을 완료 처리하지 않는다. 내부 시험 profile의 font 등록은 public `@font-face`나 일반 font fallback 지원을 뜻하지 않는다.

## 현재 코드 경계

- `HostDocumentSnapshot`은 `HostNodeKind::Text(DomString)`과 자식 순서를 보존하지만, `crates/spinon-layout/src/host_document.rs`는 보이는 Text node를 `UnsupportedTextNode`로 반환한다.
- `crates/spinon-style-to-layout/src/projection.rs`도 Text node layout projection을 차단한다. `spinon-style-to-render`의 runtime/adapter/stacking 단계도 text를 거부한다.
- `crates/spinon-render`의 runtime paint command와 `spinon-render-wgpu`에는 glyph run·glyph atlas·text draw command가 없다.
- `spinon-layout`은 crates.io `taffy 0.14.0` 알고리즘을 `CalcLayoutTree`에서 직접 호출한다. 공개 `TaffyTree` 예제의 `compute_layout_with_measure`를 바로 붙일 수 있는 구조가 아니며, Taffy upstream도 inline formatting을 downstream text layout과 연동하는 영역으로 두고 있다. Flex-wrap으로 inline flow를 흉내 내지 않는다.
- `spinon-style`에는 Stylo 0.22 computed font style가 있지만, style snapshot→text run→layout→render의 버전된 연결은 없다.

## 기준 명세와 엔진 조사

- [CSS 2.2 §9.4.2](https://www.w3.org/TR/CSS22/visuren.html#inline-formatting)는 inline formatting context 안의 inline-level box와 line box 구성을 정의한다. [CSS 2.2 §10.8](https://www.w3.org/TR/CSS22/visudet.html#line-height)는 line-height와 baseline 정렬을 정의한다.
- [CSS Text Level 3 §4](https://www.w3.org/TR/css-text-3/#white-space-property)는 newline·space 처리와 white-space 값에 따른 collapse/preservation을 정의한다. [CSS Inline Layout Level 3](https://drafts.csswg.org/css-inline/)는 세부 line box 논의를 위한 Editor's Draft이며, 최종 규범으로 표시하지 않는다.
- [Parley 0.12.0 release](https://github.com/linebender/parley/releases/tag/v0.12.0)는 2026-10-09 공개됐다. 현재 공개 API에는 shaping·line layout, `white-space-collapse`, CJK `line-break`, inline vertical alignment와 line metrics가 있다. 최소 Rust 버전 1.88은 현재 checkout의 `rustc 1.96.1`보다 낮다. 최근 major API 변경과 text stack 크기를 감안해 `=0.12.0`에 고정하고, wrapper API 하나로 캡슐화한다.
- `parley_engine`은 custom line-break 정책이 필요한 하위 수준 API다. 0.12.0 release notes에 줄 경계에서 ligature/cursive run을 다시 shaping하는 작업이 진행 중이라고 적혀 있으므로 첫 slice는 high-level `parley` API를 사용한다. 재-shaping이 필요한 입력은 구현 전 Chrome/WPT 비교에서 따로 통과해야 한다.
- [Taffy inline issue #627](https://github.com/DioxusLabs/taffy/issues/627)은 inline formatting이 Taffy 안에 완성되어 있지 않고 별도 text engine과 연동될 수 있음을 설명한다. Spinon의 현재 custom tree callback과 Block/Flex/Grid pass에서 available-size와 최종 line layout을 일치시키는 adapter를 구현한다. 이 issue는 Taffy의 영구 API 약속으로 간주하지 않는다.

## 단계와 범위

### C15.1 · block 안의 inline paragraph와 실제 glyph run

- definite content inline-size를 가진 기존 Block context 안에서 direct text와 같은 style을 상속하는 한 단계 `span`을 source order로 inline sequence로 수집한다. C15.1의 span은 별도 border·padding·margin·size·방향 변경이 없어야 한다. 중첩 span과 box decoration은 식별 가능한 오류로 닫고 C15.2에서 다룬다. 모든 `display:none` subtree와 빈 text node를 정확히 제외한다.
- `white-space: normal | nowrap | pre-line`, `word-break: normal | break-all`을 지원 범위로 고정한다. line box 계산에는 oracle fixture가 지정한 내부 line-height scalar를 쓴다. CSS `line-height` 값의 parsing·cascade는 C16 범위다. 그 밖의 white-space/word-break 값은 fallback하지 않고 해당 node/property를 식별하는 오류로 닫는다.
- Ahem 10px와 Roboto variable 20px를 opt-in fixture에서 직접 등록한다. text node 문자열을 실제 shaping API에 전달하고 glyph ID·advance·font metrics를 결과에 보존한다.
- 출력은 immutable `TextLayoutSnapshot`으로 line box, baseline, text run, glyph origin/advance, node별 inline fragment rect를 포함한다. Taffy가 해당 Block의 content width를 결정하는 동안 사용한 measure와 최종 renderer 입력은 같은 font/style/document/environment revision과 같은 available width에 묶는다.
- 한 Block context가 paragraph text만 가진 첫 fixture를 먼저 연결한다. text/inline run 사이에 block-level child가 섞인 anonymous block splitting, flex/grid item 내부 line breaking, inline replaced object, `float`, ruby, vertical writing mode는 후속 slice다. 불완전 profile은 layout 전체를 조용히 성공시키지 않는다.

### C15.2 · nested inline formatting과 fragmentation

- 같은 paragraph 안의 nested inline element, 줄에 걸친 inline fragment, 각 fragment의 border/padding contribution, baseline과 line box height를 붙인다.
- line box 계산은 ascent/descent, explicit `line-height`, inline box baseline을 실제 font metrics에서 가져온다. Ahem baseline fixture와 서로 다른 ascent/descent의 font fixture를 모두 사용한다.
- mixed block/inline children은 CSS anonymous block box 생성 순서를 별도 고정 fixture로 추가한 뒤 연결한다. 구현하지 않은 요소는 식별 가능한 diagnostic을 남긴다.

#### C15.2 고정 acceptance oracle

[Chrome 155 수치와 fixture](../spec/internal/evidence/c15-2-inline-text-precomparison-2026-10-17/README.md)는 C15.1 plain span과 다른 decorated/nested 입력이다. fixture의 DOM geometry는 CSS px에서 `0.05px` 이내로 비교하고, 별도 line fragment를 합친 union rectangle로 대체하지 않는다.

| ID | 입력 | 사전 기대 관찰 |
| --- | --- | --- |
| `nested-short` | Ahem `10px/10px`, paragraph width `50px`; outer span border `1px`/padding `2px`, inner span border `2px`/padding `3px`, text `XX` | paragraph `50×10px`; outer rect `[0,-3,36,16]`, inner rect `[3,-5,30,20]`, text range `[8,0,20,10]`. inline box의 vertical border/padding은 fragment rect를 확장하지만 line box 높이를 키우지 않는다. |
| `nested-wrap` | Ahem `10px/10px`, width `30px`; 두 단계 span에 border/padding `1px`, text `XX XX` | line box `30×20px`; text rect는 `[4,0,20,10]`, `[0,10,20,10]`; outer fragments는 `[0,-2,24,14]`, `[0,8,24,14]`; inner fragments는 `[2,-2,22,14]`, `[0,8,22,14]`. inline-start edge는 첫 fragment, inline-end edge는 마지막 fragment 폭에 반영한다. |
| `ahem-baseline` | Ahem `10px/10px`, baseline marker | line box `30×10px`; baseline은 top에서 `8px` 아래다. |
| `roboto-metrics` | Roboto `20px/24px`, `Hgyp` 두 run과 baseline marker | paragraph `93.4063×24px`; 각 DOM text range advance `46.2031px`; baseline은 top에서 `19px` 아래다. Canvas font ascent/descent `19px/5px`, actual glyph bounds `14.2188px/4.2676px`를 보존한다. Canvas advance `46.1914px`와 DOM geometry advance를 서로 바꾸어 쓰지 않는다. |
| `mixed-baseline` | Ahem `10px/10px` parent 안에 Roboto `20px/24px` baseline-aligned span | 한 줄 높이 `24px`; Roboto run top `0px`; Ahem text top `11px`; 두 font baseline marker는 top에서 `19px` 아래다. 단일 paragraph font scalar만 쓰는 C15.2a 구현 gate가 아니라 per-run font metric consumer의 후속 fixture다. |
| `anonymous-flow` | Ahem `10px/10px`, width `30px`; inline `X<span>XX</span>`, direct block `XX`, trailing inline `X XX` | parent `30×40px`; 선행 inline line y=`0`, block rect `[0,10,30,10]`, 후행 inline line y=`20`,`30`이며 source order를 유지한다. 이 topology를 연결하기 전까지 mixed block/inline은 명시 diagnostic을 반환한다. |

#### 구현 순서와 경계

- C15.2a는 최대 두 단계 HTML inline span, 고정 font/line-height, run font metrics로 계산한 line baseline과 px border/padding fixture를 연결한다. 각 inline owner는 덮은 line마다 별도 fragment를 가지며 text source order와 UTF-16 range를 보존한다. `nested-short`, `nested-wrap`, `ahem-baseline`, `roboto-metrics`를 이 slice의 required cases로 둔다.
- C15.2b는 `mixed-baseline`처럼 서로 다른 inline run font metric을 같은 line box에서 조합하고, `anonymous-flow`의 직접 block child 앞뒤 inline sequence를 anonymous block flow로 정규화한다. 해당 입력이 연결되기 전에는 이 항목을 지원하거나 C15.2 완료로 표시하지 않는다.
- C15.2a는 inline margin, non-baseline `vertical-align`, replaced inline object, 세 단계 이상 nesting, mixed block child, bidi/vertical writing을 지원 범위로 넓히지 않는다. 각 지원 밖 topology/property는 `auto`/0/default style로 강등하지 않고 node와 stage가 드러나는 diagnostic을 반환한다.
- 모든 nested/decorated span geometry는 같은 paragraph snapshot, font/style/document/environment revision과 available width에 묶인다. measure와 final layout의 width가 다르면 해당 snapshot을 재사용하지 않는다.

### C15.3 · Unicode shaping·glyph paint·layout 소비처

- Hangul, CJK line-break, combining sequence, ligature, Arabic/Bidi 및 emoji sequence를 실제 glyph/run 결과로 확인한다. 이 단계에는 font fallback 전 범위를 포함하지 않으며, 각 script를 제공하는 고정 fixture font를 지정한다. 지정 폰트가 없는 환경에서는 system fallback에 의존하지 않는다.
- shaped glyph를 실제 WGPU frame에 그리는 raster/pipeline과 runtime paint command를 연결한다. invisible text를 layout 성공으로 간주하지 않는다. C15 완료 전에는 Android AVD PixelCopy와 iOS Metal drawable에서 text ink와 overflow/clip을 캡처한다.
- text geometry가 Block·Flex·Grid sizing, overflow/scroll, paint order, clip, hit-test, text ink bounds에서 소비되는 경계를 추가한다. 이미지·text의 inline baseline과 replaced inline object는 이 단계가 별도 fixture를 소유할 때만 추가한다.
- C16에서 `font-family/size/weight/style`, CSS `line-height` 값의 parsing·cascade, `@font-face`, asynchronous font load/fallback, letter/word spacing, text-align/decoration/transform/overflow와 전체 cascade 연결을 별도 구현한다. C15 fixture의 fixed-font/internal line-height scalar는 이 계약을 대신하지 않는다.

#### C15.3 고정 acceptance oracle

[Chrome 155 Unicode shaping 사전 비교](../spec/internal/evidence/c15-3-unicode-shaping-precomparison-2026-10-17/README.md)는 고정 글꼴·DOM 관찰·상세 JSON·screenshot을 분리해 보존한다. 이 oracle의 CSS px geometry 오차는 각 run/fragment/line별 0.05px 이내로 비교한다. paint는 text antialias 전체 screenshot similarity로 판정하지 않고 고정 font와 flat-color overlay/clip 경계의 명확한 표본을 비교한다.

| ID | 입력 | Chrome DOM 사전 기대 관찰 |
| --- | --- | --- |
| `hangul-korean` | Noto Sans KR 20px, 한글 테스트 | inline-block rect 97.609375×24px; Canvas advance 96.470169px는 별도 metric으로 기록하고 DOM rect와 혼용하지 않는다. |
| `cjk-strict-break` | Noto Sans KR 20px, line-break:strict, 폭 40px, 甲乙、丙丁 | rect 40×72px; line grouping 甲 / 乙、 / 丙丁; U+3001가 다음 줄 첫 위치에 오지 않는다. |
| `combining-cluster` | Roboto 20px, x + U+0323 | UTF-16 source range [0,2]를 하나의 grapheme cluster로 보존하고 DOM advance 9.921875px를 기록한다. |
| `common-ligature` | Roboto 20px office, ligature enabled / font-variant-ligatures:none | DOM width 49.5625px / 51.25px; Canvas TextMetrics는 두 CSS variant를 구별하지 않으므로 pass oracle로 쓰지 않는다. |
| `arabic-bidi-marks` | Noto Sans Arabic 20px/28px, direction:rtl, 폭 140px, السَّلَامُ 123 | container 140×28px; Arabic joining/marks와 Bidi run을 보존하고 ASCII digits가 LTR 순서로 배치된다. |
| `emoji-zwj` | Noto Emoji 28px/36px, 👩‍💻 | UTF-16 [0,5] 단일 grapheme range, DOM advance 35.546875px; 실제 shaped cluster·glyph identity와 보이는 ink는 runtime evidence로 별도 확인한다. |
| `intrinsic-text` | 한글字, Noto Sans KR 20px | Block/Flex/Grid 각각 56.8125×24px; sizing consumers는 같은 font/style/width revision의 shaping 결과를 사용한다. |
| `visible-overflow-hit` | Ahem 10px XX, parent width 15px, overflow:visible | text advance 20px가 parent 밖으로 이어지고 parent 경계 밖의 ink/hit-test를 paint/clip/hit 소비처에서 검사한다. |
| `hidden-overflow-clip` | 같은 입력, overflow:hidden | 경계 밖 ink가 clip되고 외부 hit point가 text를 반환하지 않는다. DOM oracle에서 외부 point hit 결과는 null이다. |
| `auto-scroll-overflow` | 같은 입력, overflow:auto | parent border-box 폭 15px, clientWidth=13, scrollWidth=21; content overflow를 clip 결과와 혼동하지 않는다. |
| `paint-order-overlap` | text office 뒤에 red overlay 배치 | parent-relative overlay rect [14,1,12,20]가 text ink 영역과 겹치고 paint order상 text 위에 그려진다. pixel oracle는 antialiased glyph 대신 overlay 내부의 불변 색 표본도 확인한다. |

상세 parent/text 좌표, per-grapheme Range, Canvas metric, font loading 상태는 evidence JSON에 고정했다. viewport 배치에 따른 절대 y 좌표는 정답으로 재사용하지 않고 parent-relative geometry와 run/line ordering을 비교한다. Noto Emoji의 pinned font GSUB가 ZWJ sequence를 한 glyph로 매핑한다는 사실은 fixture font의 입력 조건일 뿐, Chrome DOM 관찰만으로 browser glyph ID가 확인됐다고 주장하지 않는다.

연결할 upstream WPT revision은 df642c4142354ffb5a0af6fa41610677f6321092다: css/css-text/line-break/line-break-strict-015a.xht, css/css-text/line-break/line-break-shaping-001.html, css/css-fonts/font-variant-ligatures-01.html, css/css-text/bidi/bidi-lines-001.html. 이 경로들은 구현 시 실행할 후보일 뿐 이 계획 amendment에서는 실행하지 않았고, Chrome oracle도 WPT/Spinon pass가 아니다. [C15.3 계획 검토 20개](../spec/internal/evidence/c15-3-unicode-shaping-plan-review-20-2026-10-17.md)

## 고정 사전 비교 모델

입력·실행 화면·원본 DOM 측정값·font SHA-256은 [사전 비교 자료](../spec/internal/evidence/c15-inline-text-precomparison-2026-10-17/README.md)에 기록했다. Chrome은 Agent Browser의 `155.0.8059.85`, macOS 26.5.1 ARM, locale `ko-KR`, viewport `320×480 CSS px`, DPR 1이며 Ahem/Roboto font download가 완료된 뒤 관찰했다. High-entropy browser version은 저장했지만 실행 파일 SHA와 CDP protocol revision은 없다. 첫 구현 전에 수집을 시도하고, 수집할 수 없으면 그 제한을 fixture manifest와 PR에 명시한다.

첫 C15.1 pass 집합은 다음 10개 fixture ID다.

| ID | 입력 | 사전 기대 관찰 |
| --- | --- | --- |
| `collapsed-spaces` | Ahem 10px, `X   X`, `white-space:normal` | content advance `30×10`; 공백 세 개 중 하나만 10px advance |
| `wrap-at-word` | Ahem 10px, width 30px, `XX XX` | 두 line, 각 text width 20px, block height 20px |
| `long-word` | Ahem 10px, width 15px 및 0px, `XX`/`X` | 두 폭 모두 unbreakable word가 한 line에 남아 text advance가 parent width를 넘는다. 15px 폭의 `XX`는 20px, 0px 폭의 `X`는 10px다. |
| `break-all` | Ahem 10px, width 20px, `XXXXX` | line advance 20/20/10px, block height 30px |
| `preserved-newline` | Ahem 10px, width 30px, `X\nX`, `pre-line` | 두 line, block height 20px |
| `nowrap` | Ahem 10px, width 30px, `XX XX` | 한 line width 50px, block height 10px |
| `plain-inline-fragment` | width 30px, Ahem 10px, border/padding 없는 span `XX XX` | span fragment 2개, 각각 `20×10px`; 줄 끝에서 접힌 공백은 fragment 폭에 포함하지 않음 |
| `baseline` | Ahem 10px, line-height 10px, baseline marker | baseline은 line top에서 `8px` 아래 |
| `proportional-i`, `proportional-W` | Roboto 20px, 각각 `iiii`, `WWWW` | advance `19.4531px` 대 `70.9844px`; 동일 code-unit 수라도 폭은 달라야 함 |
| `ligature-text` | Roboto 20px `office` | 전체 shaped run advance `49.5625px`; 특정 ligature 여부를 이 값만으로 유추하지 않음 |

Geometry 판정은 CSS px에서 line count, parent/frame size, fragment rect, glyph origin/advance별 허용 오차 `0.05 CSS px`로 정한다. Ahem은 fixed font·integer position의 glyph coverage sample을 exact 비교한다. Roboto는 같은 고정 font file로 glyph advance·ink bounds를 비교하며, 다른 rasterizer의 per-channel antialias 차이를 layout pass/fail에 섞지 않는다. OS/browser/surface color difference는 별도 분류한다.

사전 비교 fixture의 `inline-fragment` case는 border 1px를 포함한 C15.2 기준이다. C15.1의 `plain-inline-fragment` 관찰은 동일한 Agent Browser page에서 span border를 끈 변형으로 측정했고, 원본 precomparison fixture와 screenshot은 변경하지 않는다. `long-word`의 0px 폭은 별도 DOM mutation observation으로 기록하고 원본 fixture는 그대로 둔다. 두 slice의 rect를 서로의 acceptance oracle로 섞지 않는다.

C15.1 fixture/font와 observation은 수정하지 않는다. C15.2 전용 fixture와 Chrome observation을 추가할 때는 기존 범위와 섞지 않고 새 artifact hash를 고정한다. C15.2 oracle/범위 amendment는 새 실패 관점 20개로 계획을 검토하고 별도 계획 PR의 diff도 30개 관점으로 검토한 뒤 병합한다. 계획 PR 병합 전에 C15.2 제품 코드를 바꾸지 않는다. WPT는 실행할 파일 path와 WPT commit SHA를 구현 전에 고정하고, Chrome oracle pass를 Spinon WPT pass로 부르지 않는다.

## 내부 자료 모델과 revision 경계

- shaping/layout crate는 DOM이나 renderer에 종속되지 않는 text content/run style/available-size를 입력받고 line/glyph output을 반환한다. 우선 [Parley](https://github.com/linebender/parley) wrapper를 내부 crate로 두어 UI/layout/render 계층에서 upstream API를 직접 사용하지 않게 한다.
- text fragment의 owner `NodeId`, source UTF-16 range, shaped glyph cluster range, baseline, line index와 clip-independent rect를 보존한다. surrogate pair, combining mark, ligature boundary를 UTF-8 byte index나 glyph index와 혼동하지 않는다.
- 각 snapshot은 HostDocument generation/document/render-tree revision, StyleRevision, EnvironmentRevision, font registry revision, paragraph content identity, available inline-size, writing direction/profile을 포함한다. Taffy가 measure를 여러 번 호출해도 width-key가 다른 layout을 재사용하지 않는다.
- font load/fallback 결과와 shaped run은 같은 immutable snapshot에 귀속한다. 문자열·style·font asset·font revision·available-size 중 하나라도 달라지면 cache hit로 승인하지 않는다. layout/render admission에서 revision mismatch는 명시 오류다.
- 이 인터페이스는 앱 작성자 공개 API가 아니다. 구현 PR에서 내부 계약 문서 ID와 `0.1.0` 버전으로 입력·오류·ownership·thread/lifecycle·cache·platform difference를 명세한다. 공개 HTML/CSS 지원을 추가하지 않는다.

## 실패 주입과 적대 확인

`implementation-review-20`은 계획 검토 20개 및 PR diff 검토 30개와 다른 입력/불변 조건을 쓴다. 최소 negative control은 아래와 같다.

1. `font_size × code_unit_count` width 추정으로 바꾸면 Roboto `iiii`/`WWWW` test가 실패해야 한다.
2. whitespace collapse를 끄면 `X   X` 30px advance가 달라져야 한다.
3. default line breaking이 `XX`를 두 line으로 쪼개면 `long-word`가 실패해야 한다.
4. `break-all`을 무시하면 `XXXXX`의 20/20/10 line sequence가 달라져야 한다.
5. newline을 일반 space로 collapse하면 `pre-line` line count가 2에서 바뀌어야 한다.
6. baseline을 ascent, descent, font size 또는 line top과 혼동하면 Ahem baseline marker가 8px에서 벗어나야 한다.
7. inline fragment를 하나의 element union rect로 축약하면 fragment 수/두 border rect가 실패해야 한다.
8. 한 measure pass의 width와 다른 width의 final layout을 조합하면 width 15/20/30 fixture 중 적어도 하나가 실패해야 한다.
9. content/style/font revision만 변경하고 text-layout revision을 유지하면 stale snapshot admission이 실패해야 한다.
10. 숨긴 subtree text, 빈 Text node, split adjacent text node, source order를 각각 변형해도 visible geometry가 CSS 관찰값과 일치해야 한다.
11. 미지원 mixed block child, unsupported white-space 값, corrupt glyph/font result를 `auto`/0/default font으로 강등하지 않고 node·stage가 붙은 오류로 반환해야 한다.
12. overflow line을 parent content width로 clip하지 않고 실제 scroll/hit/paint layer에 전달한다. `overflow:hidden`은 후속 slice 전까지 지원했다고 표시하지 않는다.

각 항목은 별도의 fixture/property/style mutation과 독립 expected condition을 가진다. 하나의 pass 수를 여러 관점으로 재계산하지 않는다.

## 플랫폼과 실행 게이트

- Pure Rust shaping/layout oracle 및 focused fixture matrix를 먼저 통과시킨다. `cargo fmt --all -- --check`, 관련 crate tests, workspace Clippy와 lockfile 검증을 실행한다.
- Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator의 실제 앱 runtime fixture에서 동일한 V8 DOM input, document/style/font revisions, viewport, DPR를 사용한다. 실기기는 사용하지 않는다.
- 양 simulator에서 frame/line/glyph geometry가 Chrome CSS px oracle과 행별 `0.05px` 이내여야 한다. Ahem glyph sample은 Android PixelCopy/iOS Metal drawable의 capture phase를 기록하고 exact coverage 표본을 비교한다. Roboto rasterization의 단순 screenshot similarity로 필수 layout 실패를 가리지 않는다.
- text outside parent border가 생기는 폭 15px long word와 `nowrap` overflow를 screenshot/clip boundary에서 확인한다. 이 시험은 text line ink의 paint와 부모 clip을 각각 기록하고, frame geometry만으로 바깥 픽셀을 입증했다고 하지 않는다.
- 두 platform에서 성공/예상 오류 report, 원본 log, screenshot, capture phase, binary/app SHA와 source revision을 보존한다. simulator 증거를 실기기·hardware GPU·모든 OS font equivalence로 확장하지 않는다.
- fixture·font·C15-only ABI는 목적을 드러내는 opt-in fixture-test feature로 격리한다. 기본 build에서 fixture font payload와 fixture FFI symbol이 없고 positive-control fixture build에는 있어야 한다.

## 완료 판정과 명시적 제외

C15 완료는 C15.1–C15.3 입력 class의 text shaping, inline fragment/line box geometry, painter visibility, revision safety가 함께 확인되고 관련 internal spec와 실행 자료가 등록된 뒤에만 검토한다. 각 implementation PR은 독립 실패 관점 20개, 별도 PR diff 관점 30개를 기록하고, C15 완료 체크는 마지막 C15 통합 PR에서만 갱신한다. C16 속성 전체 또는 C19.3 text painter가 빠져 있으면 C19 전체도 완료할 수 없다.

이 계획과 C15.1은 아래 범위 완료를 뜻하지 않는다.

- CSS `line-height` property parsing/cascade, `@font-face` parsing/loading, arbitrary system fallback, asynchronous font swap, full font matching, letter/word spacing, `text-align`, text decoration/transform/overflow는 C16이다. C15의 line box oracle 값은 내부 fixture scalar에 한정한다.
- block children과 text의 전체 anonymous block splitting, inline replaced element, float, table, ruby, vertical writing, page fragmentation은 별도 CSS stages다.
- full CSS Text/Inline conformance, full Unicode/font fallback matrix, DOM selection/caret/hit text API, text accessibility API와 전체 WPT suite는 포함하지 않는다.
- fixture 결과로 공개 CSS 지원률을 올리지 않는다.

## 작업 순서

1. 기본 계획의 20개 검토와 C15.1/C15.2 Chrome observation/artifact hash를 각각 확인한다. 새 C15.2 범위·oracle amendment는 기존 계획 검토를 재사용하지 않고 별도 실패 관점 20개와 PR diff 30개를 통과한 뒤 병합한다. `spec/STATUS.md` C15 row는 미완료 상태로 계획/근거 링크만 추가한다.
2. 계획 PR 병합 후 구현 직전에 Parley `=0.12.0` API·font ownership·line-break reshaping gate와 Taffy `CalcLayoutTree` adapter를 잠금된 dependency source에서 확인한다. public default profile과 fixture-only font registry 경계를 확정한다.
3. internal C15.1 contract와 style/document/font revision preconditions를 정의한다. 고정 Chrome precomparison, C15 fixture, WPT paths/revision, expected geometry를 구현 전 comparator에 연결한다.
4. actual shaping result로 `TextLayoutSnapshot`을 생성하고 Taffy Block available-size와 연동한다. measure callback 두 번째 호출과 changed width, negative/error/stale snapshot, anonymous inline fragments를 focused Rust tests로 검증한다.
5. Renderer glyph command/raster path와 text hit/clip integration을 별도 slice로 연결하고 Android/iOS simulator runtime fixture를 보존한다. fixture routes와 fonts는 opt-in에만 둔다.
6. 계획 검토와 겹치지 않는 구현 실패 관점 20개를 완료하고 발견한 결함을 수정한다. 그 뒤 별도 PR diff 실패 관점 30개를 실행하고 required checks/mergeability를 확인해 `Rebase and merge`로 진행한다.
7. 각 통과한 부분 작업은 evidence index와 roadmap description에 연결하되 C15/C16/C19의 상위 완료 상태는 그대로 둔다. 전체 C15가 끝날 때만 internal interface spec, STATUS, supported scope를 최종 일치시킨다.
