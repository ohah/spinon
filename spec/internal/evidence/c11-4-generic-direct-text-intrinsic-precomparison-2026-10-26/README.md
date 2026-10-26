# C11.4 일반 direct text intrinsic 사전 비교

## 목적

C11.4 Stage A의 다음 후보인 문서 기반 direct Grid item text 입력을 위한 독립 기대값과 Chromium 관측을 고정한다. 이 결과는 Chrome oracle이며 Spinon 구현 통과를 뜻하지 않는다.

## 입력과 재현

- 저장된 fixture: [`fixture.html`](./fixture.html)
- 실제 관측 URL: `http://127.0.0.1:8765/spec/internal/evidence/c11-4-generic-direct-text-intrinsic-precomparison-2026-10-26/fixture.html`
- 글꼴: 저장소 C15 Ahem fixture, SHA-256 `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`
- 브라우저: Agent Browser CLI `0.31.1` · `Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) HeadlessChrome/156.0.0.0 Safari/537.36` · DPR 1 · viewport 1280×633 CSS px
- Agent Browser에서 `document.fonts.ready` 뒤 `document.fonts.check('10px Ahem') == true`를 확인했다.
- 재현하려면 저장소 루트에서 `python3 -m http.server 8765 --bind 127.0.0.1`을 실행하고 위 URL을 Agent Browser로 연다. fixture의 상대 font URL은 저장소 `tests/fixtures/resources/c15/Ahem.ttf`를 가리킨다.
- [Agent Browser 화면](./chromium-agent-browser.png), [전체 DOM 측정](./chromium-observation.json), fixture/font SHA와 JSON hash를 보존했다. Per-item JSON은 `ParagraphStyle`에 대응하는 computed CSS property, identity와 text rect를 기록한다. Fixture SHA-256 `1b2bdf7d8cbac93270bf84e991168957d5d59973b803c8c68f253306ee5b7f52`, font SHA-256 `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`, JSON SHA-256 `ae44f2a2ab1493cc3b7f2426373af15dcc2513769cc28ff505890e4172a15f22`, screenshot SHA-256 `1d3d1377537329d285810f174178d97d2be41ffe815bc3647621acb927226474`다.
- 최종 관측 session `spinon-c114-text-style-observation-20261026`은 `close` 후 `session info`에서 `active:false`, `pid:null`로 확인했다. session daemon/전용 Chrome profile과 HTTP fixture server가 종료된 것을 확인했다. process inventory에 보인 별도 HWPJS Agent Browser/Chrome 및 사용자의 Chrome Crashpad는 이 session에 속하지 않아 종료하지 않았다.

## 독립 기대값과 관측

Ahem의 glyph advance와 line-height는 각각 10 CSS px다. `A BC`의 normal min-content는 가장 긴 unbreakable word `BC`의 20px, max-content는 내부 공백을 포함한 40px다. `white-space-collapse:collapse`에서는 선행·후행 공백과 연속 내부 공백이 접혀 ` A  BC `도 같은 `[20,40]px` track 값을 낸다. `nowrap`은 두 contribution 모두 40px이고, `ABCD` + `word-break:break-all`은 min/max가 각각 10/40px다. `A BCD`는 30/50px다. 각 기대값은 글자 advance·break opportunities·10px gap·100px fixed root width에서 독립 계산했다.

| 장면 / root ID | DOM 순서와 item style | Columns | Grid / item 주요 rect |
| --- | --- | --- | --- |
| `normal` | `A BC` 두 item, normal | `[20,40]px` | root `100×20`; items `(0,0) 20×20`, `(30,0) 40×10`; 첫 item line widths `[10,20]` |
| `nowrap` | `A BC` 두 item, `text-wrap-mode:nowrap` | `[40,40]px` | root `100×10`; items x=`0,50`, 각 `40×10` |
| `break-all` | `ABCD` 두 item, `word-break:break-all` | `[10,40]px` | root `100×40`; 첫 item `(0,50) 10×40`에 네 줄, 둘째 `(20,50) 40×10` |
| content 변형 / `grid-9x` | `item-zeta`와 `item-3` 각각 `A BCD`, normal | `[30,50]px` | root `100×20`; items `(0,100) 30×20`, `(40,100) 50×10` |
| 공백 / `grid-ws` | 두 item 각각 원문 ` A  BC `, computed collapse+wrap | `[20,40]px` | root `100×20`; items `(0,130) 20×20`, `(30,130) 40×10` |
| 순서·style owner / `grid-reorder` | DOM 조작 후 `item-a` (`ABCD`, break-all) → `item-z` (`A BC`, normal), 자동 배치 | `[10,40]px` | root `100×40`; `item-a` `(0,160) 10×40`, `item-z` `(20,160) 40×10` |

`grid-reorder`는 먼저 `item-z`, `item-a` 순서로 만든 후 `insertBefore(item-a,item-z)`를 수행한다. 측정된 `childOrder`는 `[item-a,item-z]`; `item-a`의 computed `word-break`는 `break-all`, `item-z`는 `normal`이다. 이 control은 node ID와 source order가 다르거나 paragraph style을 다른 owner에 붙였을 때 예상값이 달라지는지 판별한다.

<a id="row-axis-trace-expectations"></a>

## 행 축 trace 기대값

`generic-root-normal`의 각 `A BC` item은 Ahem advance와 line-height가 10px이고, normal wrapping의 min-content는 `BC` 두 글자 폭 20px와 두 줄 높이 20px, max-content는 한 줄 높이 10px다. 따라서 두 direct item의 intrinsic block-size 입력은 각각 `min-content=20px`, `max-content=10px`이고 Chrome에서 관찰한 최종 auto row 크기는 20px다.

Chrome/Ahem 사전 계산은 text item별 intrinsic input을 고정한다. 앱 runtime trace는 columns contribution에서 node `2`의 `MinContent=20px`, node `4`의 `MinContent=20px`와 `MaxContent=40px`를 확인한다. Rows 단계에서는 definite column width가 각 item에 적용되므로 node `2`(20px)는 `Minimum/MinContent/MaxContent=20px`, node `4`(40px)는 각각 10px이다. 모두 span 1이고 `crossesFlexibleTrack=false`다. 같은 tuple 반복은 허용하고 다른 item/kind/value는 거부한다. 이 trace 값은 Chrome DOM observation이 아니라 해당 app runtime의 sizing trace acceptance이며, Chrome JSON은 contribution kind를 제공하지 않는다. 최종 serialized trace는 양끝에 collapsed 0px gutter를 포함하고, 10px column gutter를 content track 사이에 둔다.

## 현재 Spinon baseline

일반 C11.4 entry point [`compute_runtime_block_flex_grid_track_sizing_style_layout`](../../../../crates/spinon-style-to-layout/src/projection.rs#L241)은 paragraph/style input 없이 `compute_runtime_style_layout`을 호출한다. 공통 HostDocument builder는 visible Text node를 text paragraph source로 등록하지 않으면 `LayoutError::UnsupportedTextNode`를 반환한다([`host_document.rs`](../../../../crates/spinon-layout/src/host_document.rs#L370)). 기존 회귀 test도 이 동작을 명시한다([`host_document_contracts.rs`](../../../../crates/spinon-style-to-layout/src/tests/host_document_contracts.rs#L13)).

실행한 baseline command:

```sh
CARGO_TARGET_DIR=/Users/yoonhb/Documents/workspace/spinon/target cargo test --locked -p spinon-style-to-layout layout_rejects_text_in_a_host_document_subtree
```

결과는 `1 passed; 240 filtered out`이며 해당 test는 `StyleLayoutError::Layout(LayoutError::UnsupportedTextNode(_))`를 assert한다. 이는 공통 builder의 현 동작을 실행해 확인한 결과다. 위 custom six-scene fixture 자체를 Spinon에 실행한 것은 아니며, 이 계획에서 런타임 입력을 연결해 그 차이를 해소한다.

현재 fixture adapter fallback은 root ID를 `TEXT_CASES`에서 찾고 item ID와 text literal도 함께 대조한다([`c11_4_text.rs`](../../../../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_text.rs#L217)). 새 root/content가 그 표에 없으면 generic paragraph 수집 대신 `unsupported_c114_text_fixture_root` 또는 `unsupported_c114_text_fixture_case` 경로로 닫힌다.

## Scope boundary

이 사전 비교는 고정 Ahem 글꼴을 쓴 수평 direct element-text Grid input만 다룬다. 임의 폰트 다운로드, anonymous Grid text, 여러 inline run 그룹핑, image/nested Grid text, 일반 CSS admission, 전체 C11.4, WPT 적합성을 증명하지 않는다. `ParagraphStyle.ligatures`는 positive 입력에서 기본값 `Normal`만 기대한다. 이 evidence는 미지원 stylesheet declaration rejection이나 비기본 ligature 계산을 증명하지 않는다. 계획의 rejection-only control은 inline `style` 선언을 사용하고, 계산 지원을 뜻하지 않는다.

<a id="planned-rust-fixture-input"></a>

## Rust fixture input 설계와 구현 test evidence

Browser fixture는 여섯 Grid scene을 한 HTML 문서에서 세로로 배치하므로 JSON의 root/item/text rect y는 문서좌표다. 계획된 Spinon fixture는 각 scene을 별도 `HostDocumentSnapshot`으로 만들고 `StyloDocumentView::new_html_runtime_mount_shared(Arc::new(snapshot.clone()), root)`의 mount root로 전달한다. view의 synthetic html/body는 CSS cascade 조상이고 layout helper root에는 Grid만 전달한다. 각 scene-local frame을 비교할 때 Chrome JSON의 root x/y를 root와 item/text rect에서 빼며, Rust mount-root frame은 `(0,0)`이어야 한다. 이 mapping은 [계획의 CSS px 및 API 계약](../../../../plan/c11-4-generic-direct-text-intrinsic-input.md#hostdocument-fixture와-좌표-계약)에 설명되어 있다.

`grid-reorder` browser input은 script가 `item-z,item-a`를 `item-a,item-z`로 옮긴 뒤 관측했다. JSON `childOrder`에는 최종 `[item-a,item-z]`가 기록되어 있다. 계획된 Rust builder는 JavaScript를 실행하지 않고 그 최종 HostNode 순서를 직접 만든다. 나머지 item text/style도 JSON에 기록된 computed values를 typed fixture input으로 구성한다. 이 방식은 scene identity/style owner를 검증하기 위한 layout fixture일 뿐 HTML parser나 script 실행 경로가 아니다.

고정 builder는 원본 fixture의 cascade selector를 재현하도록 root class `scene`과 변형 class `nowrap`/`break-all`, item class `item`, `grid-reorder`의 `item-a.break-all`을 보존한다. `.fixed` class와 그 하위 `grid-column` 규칙은 Rust sheet에서 제거되므로 만들 필요가 없다. 즉 final child order와 class는 HostDocument/CSS cascade input이며, root/item ID와 text literal은 collector의 branch 조건이 아니다.

Rust fixture에서 browser HTML stylesheet를 그대로 cascade하지 않는다. browser 파일의 `@font-face`, `font`/`white-space` shorthand 및 `.fixed`의 `grid-column`은 이 제한 fixture sheet에 사용하지 않는다. [계획](../../../../plan/c11-4-generic-direct-text-intrinsic-input.md)의 longhand stylesheet로 관측된 positive item style을 구성하고 Ahem bytes는 fixture `TextLayoutEngine`에 등록한다. 이 evidence는 stylesheet의 다른 authored declarations가 거부된다고 주장하지 않는다. 2-item auto-placement은 관측 최종 order에서 동일한 첫째·둘째 column geometry를 낸다. Rust builder는 `ComputedStyleSnapshot`을 hand-author하지 않고 `compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets`의 실제 결과를 검사해야 한다.

브라우저 JSON은 root의 computed CSS property map을 저장하지 않는다. root `display:grid`, 두 explicit tracks, auto rows, 100px width, 10px column gap과 `align-items:start`는 해시로 고정된 HTML author input에 있고, row auto-flow는 CSS 기본값 및 관측된 같은 행의 item geometry로 한정해 판단한다. Rust 제한 sheet는 이 값을 명시적으로 `grid-auto-flow:row`로 쓴다. 구현 수용 시 actual Stylo root snapshot을 `ComputedCssGridLayout.columns/rows/auto_flow/auto_rows`, `ComputedLayoutDimensions.width/height`, `ComputedLayoutSpacing.column_gap`, `ComputedElementStyle.properties["display"]`와 `["align-items"]`로 검사한다. 이 gate는 Rust 계산 입력이 계획과 맞음을 재현하지만 저장된 JSON에 없는 Chrome root computed-style 직렬화를 소급해 증명하지 않는다.

저장된 Chrome JSON은 여전히 Rust 입력이나 root computed-style map을 담지 않는다. 계획에 적은 HostDocument/computed-style builder와 six-scene direct-text helper assertion은 branch candidate unit test로 작성·실행했다. 명령, 9/9 결과, negative controls의 실제 parser 경계는 [구현 test 관찰](./implementation-test-observation-2026-10-26.md)에 기록한다. 이 follow-up은 Chrome JSON의 의미를 바꾸거나 app runtime/simulator 결과를 주장하지 않는다. 기존 browser fixture HTML, JSON, screenshot hash는 수정하지 않았다.
