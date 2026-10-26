# C11.4 direct-text 최소 입력 계획 fresh review 20개 — historical pass03

**검토 대상:** [계획](./c11-4-generic-direct-text-intrinsic-input.md), [사전 비교 evidence](../spec/internal/evidence/c11-4-generic-direct-text-intrinsic-precomparison-2026-10-26/README.md)  
**원 검토 기준:** worktree HEAD `b58de5df93204ba211a025f20b5bfe3790c1d385` 및 당시 계획/evidence 수정본  
**방법:** 저장소 규칙과 실제 Rust API를 읽기 전용 대조했다. 새 관점 20개를 골랐고 이전 두 리뷰의 행을 옮기지 않았다. 파일 변경은 계획/evidence 문서에만 적용했다. Rust source/test 수정, 테스트, browser 실행은 없었다.

**후속 정합화:** branch는 `040a5360be92f73e141797aa7dcd4ef00ab24942`로 rebase됐고, pass03 뒤 PR diff review에서 unsupported stylesheet rejection claim이 blocker로 지적됐다. 본표는 20행을 당시 모두 PASS로 판정했으나 row 20은 그 발견 뒤 superseded 처리했다. 이전 결과는 historical verdict이며 현재 payload의 fresh review 결과가 아니다. 현재 계획은 미지원 declaration control을 inline style로 옮겼고 StylesheetSource rejection을 주장하지 않는다. 새 독립 계획 review가 필요하다.

판정에서 **PASS는 계획이 구현에 필요한 제한·API·수용 기준을 특정했다는 뜻**이다. Spinon 구현 또는 runtime 동작이 이미 검증됐다는 뜻은 아니다.

## 새 20관점 검토

| # | 새 실패 관점 | 판정 | 근거 및 차단 여부 |
| ---: | --- | --- | --- |
| 1 | 제한 stylesheet의 `.scene`/ `.item` selector가 HostDocument에서도 실제로 match되는가 | PASS | `HostElement`은 `AttributeName` keyed attributes를 보존하고([types.rs L200-L221](../crates/spinon-core/src/document/types.rs#L200)), 계획은 모든 root/item class token을 명시한다([계획 L90](./c11-4-generic-direct-text-intrinsic-input.md#L90)). Builder 구현 시 해당 class 속성을 실제 node attribute로 넣으면 CSS selector cascade가 재현된다. |
| 2 | `nowrap` root class와 item `text-wrap-mode` 규칙을 혼동해 두 item 모두 wrap하는가 | PASS | browser stylesheet의 변형 selector는 `.nowrap .item`이고 계획은 root에 `scene nowrap`, item에 `item`을 넣도록 고정했다([fixture.html L7-L15](../spec/internal/evidence/c11-4-generic-direct-text-intrinsic-precomparison-2026-10-26/fixture.html#L7), [계획 L90](./c11-4-generic-direct-text-intrinsic-input.md#L90)). 고정 root/item별 스타일이 아니라 cascade 결과로 ParagraphStyle을 만들어야 한다. |
| 3 | `grid-reorder`에서 break-all이 root 장면 전체에 새지 않고 `item-a`만 소유하는가 | PASS | 원본 CSS에는 `.break-all .item`과 `.item.break-all` 두 선택자가 있고([fixture.html L14](../spec/internal/evidence/c11-4-generic-direct-text-intrinsic-precomparison-2026-10-26/fixture.html#L14)), 계획은 root-level break-all 장면과 `grid-reorder`의 item-level break-all class를 분리한다([계획 L90](./c11-4-generic-direct-text-intrinsic-input.md#L90)). 이 장면에서 두 번째 item은 normal computed style이어야 한다. |
| 4 | 제거한 `.fixed` class의 placement rule 일부가 Rust sheet에 남아 auto-placement 결과를 바꾸는가 | PASS | 계획은 `.fixed` 및 `grid-column` rule을 Rust sheet에서 완전히 제거하고 두 explicit column과 final DOM order만 사용한다고 적었다([계획 L44-L73](./c11-4-generic-direct-text-intrinsic-input.md#L44)). 현 C11.4 track-sizing allowlist에는 Grid template/flow와 text properties가 있지만 placement longhand는 이 profile에 추가되지 않았다([runtime_grid.rs L126-L177](../crates/spinon-style/src/stylo_dom/cascade/runtime_grid.rs#L126)). |
| 5 | row auto-placement 가정이 기본값 추측에만 남아 두 아이템이 다른 축에 놓이는가 | PASS | 제한 sheet에 `grid-auto-flow:row`가 명시되고 root snapshot의 typed `grid_layout.auto_flow`가 `Row`인지 검사한다([계획 L44-L70](./c11-4-generic-direct-text-intrinsic-input.md#L44), [ComputedCssGridLayout L260-L275](../crates/spinon-style/src/stylo_dom/cascade/snapshot.rs#L260)). 기존 profile은 `grid-auto-flow`를 허용한다([runtime_grid.rs L30-L54](../crates/spinon-style/src/stylo_dom/cascade/runtime_grid.rs#L30)). |
| 6 | `align-items:start`를 item의 paragraph property처럼 읽거나 무시하는가 | PASS | root gate는 item converter와 분리되어 `ComputedElementStyle.properties["align-items"]`를 검사한다([계획 L68](./c11-4-generic-direct-text-intrinsic-input.md#L68), [ComputedElementStyle L501-L535](../crates/spinon-style/src/stylo_dom/cascade/snapshot.rs#L501)). 이 값은 Grid root geometry의 정렬 조건이며 ParagraphStyle에 넣는 값이 아니다. |
| 7 | CSS root width/gap과 typed layout input이 같은 값인지 문자열 재해석 없이 검증되는가 | PASS | root acceptance는 `layout_dimensions.width=LengthPx(100)`와 `layout_spacing.column_gap=LengthPx(10)`을 요구한다([계획 L68](./c11-4-generic-direct-text-intrinsic-input.md#L68)). snapshot 타입은 width/height와 spacing을 typed 값으로 보존한다([snapshot.rs L152-L176](../crates/spinon-style/src/stylo_dom/cascade/snapshot.rs#L152), [L332-L336](../crates/spinon-style/src/stylo_dom/cascade/snapshot.rs#L332)). |
| 8 | runtime mount용 synthetic html/body가 scene root의 font나 frame에 영향을 주는가 | PASS | 계획은 실제 `StyloDocumentView::new_html_runtime_mount_shared`를 지정하고 synthetic parent는 cascade 조상으로만 두며 scene root만 helper에 전달한다([document.rs L162-L179](../crates/spinon-style/src/stylo_dom/document.rs#L162), [계획 L88](./c11-4-generic-direct-text-intrinsic-input.md#L88)). `.scene`이 font/width를 지정하고 mount-root frame은 (0,0)이다. browser fixture의 `html, body` reset은 geometry input으로 사용하지 않는다고 범위를 제한했다([fixture.html L6-L10](../spec/internal/evidence/c11-4-generic-direct-text-intrinsic-precomparison-2026-10-26/fixture.html#L6)). |
| 9 | browser shorthand가 C11.4 preflight 밖인데 Rust fixture가 같은 shorthand를 우연히 사용해 cascade가 실패하는가 | PASS | 계획은 `font`, `white-space`, `grid-column`을 피하고 allowlist 안의 longhand만 제출한다([계획 L44-L67](./c11-4-generic-direct-text-intrinsic-input.md#L44)). 실제 allowed text property table도 이 longhand 구성과 맞는다([runtime_grid.rs L126-L177](../crates/spinon-style/src/stylo_dom/cascade/runtime_grid.rs#L126)). |
| 10 | item의 명시값과 상속값을 한꺼번에 root 값으로 복사해 잘못된 owner style을 만드는가 | PASS | 변환은 각 `ComputedElementStyle.node_id == item.id()` 결과를 읽고 typed font size/writing style을 쓴다([계획 L35-L36](./c11-4-generic-direct-text-intrinsic-input.md#L35)). `grid-reorder` JSON은 같은 root 아래 item마다 다른 word-break 값이 있음을 보인다([JSON L559-L678](../spec/internal/evidence/c11-4-generic-direct-text-intrinsic-precomparison-2026-10-26/chromium-observation.json#L559)). |
| 11 | 줄 높이가 CSS `normal`과 수치 10px 사이에서 다른 결과를 내는데 bool field를 빠뜨리는가 | PASS | 실제 ParagraphStyle에는 `line_height`와 `line_height_is_normal`이 별도 존재한다([model.rs L124-L142](../crates/spinon-text-layout/src/model.rs#L124)). 계획은 `10px`와 `false`를 함께 요구하며 `normal`은 이 profile에서 거부한다([계획 L75-L79](./c11-4-generic-direct-text-intrinsic-input.md#L75)). |
| 12 | vertical writing 값을 CSS 문자열 추측으로 구성해 물리 inline 크기와 논리 축을 뒤섞는가 | PASS | ParagraphStyle 변환은 `writing_style_for(item_id)`의 typed `HorizontalTb/Mixed`를 요구한다([계획 L78](./c11-4-generic-direct-text-intrinsic-input.md#L78)). 이 slice의 raw intrinsic 수치는 물리 width/height라고 제한했다([계획 L17, L28](./c11-4-generic-direct-text-intrinsic-input.md#L17)). |
| 13 | `line-break:auto → Strict` 선택이 전 문자의 CSS 의미와 같다고 과장되는가 | PASS | evidence corpus는 대문자 ASCII와 U+0020만 사용하고, 계획도 이 범위에서의 변환만 허용한다([README L18-L29](../spec/internal/evidence/c11-4-generic-direct-text-intrinsic-precomparison-2026-10-26/README.md#L18), [계획 L78-L79](./c11-4-generic-direct-text-intrinsic-input.md#L78)). 다른 script의 line breaking 지원 주장은 없다. |
| 14 | ParagraphStyle에 없는 정렬·transform·spacing 값이 computed CSS에서 조용히 사라지는가 | PASS | 계획은 `text-align`, `overflow-wrap`, `text-transform`, spacing/indent의 기준값을 확인하고 다른 값이면 owner/property를 포함해 거부한다([계획 L36, L79-L81](./c11-4-generic-direct-text-intrinsic-input.md#L36)). C11.4 profile은 이 속성들을 computed property map에 보존한다([runtime_grid.rs L155-L177](../crates/spinon-style/src/stylo_dom/cascade/runtime_grid.rs#L155)). |
| 15 | `grid-ws`를 collector가 먼저 trim/collapse해 browser의 원본 source와 paragraph normalization을 중복 적용하는가 | PASS | evidence에는 원문 `" A  BC "`가 남고 collapse 결과가 `[20,40]px`임이 독립 산술에 적혀 있다([README L19-L29](../spec/internal/evidence/c11-4-generic-direct-text-intrinsic-precomparison-2026-10-26/README.md#L19)). 계획은 `ParagraphInput.sources[0].text`가 원문과 같고 `ParagraphStyle.white_space`가 collapse를 맡는지 검사한다([계획 L109](./c11-4-generic-direct-text-intrinsic-input.md#L109)). |
| 16 | direct Text Node ID와 Grid item paragraph owner Node ID를 같은 것으로 가정하는가 | PASS | `TextSource`는 text node와 원문을 별도 필드로 갖고([model.rs L234-L243](../crates/spinon-text-layout/src/model.rs#L234)), 계획은 map key/snapshot owner를 item으로, `input.sources[0].node`를 실제 Text child로 분리 확인한다([계획 L35, L109-L111](./c11-4-generic-direct-text-intrinsic-input.md#L35)). |
| 17 | 1-item legacy text fixture가 새 2-item collector에 잡혀 기존 `TEXT_CASES` 동작을 잃는가 | PASS | 기존 adapter는 special dispatch 후 subtree text scan에서 한 Text node를 처리한다([c11_4_text.rs L73-L127](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_text.rs#L73)). 새 계획은 one-item shape에서 `Ok(None)`을 반환해 legacy fallback을 유지한다([계획 L102-L108](./c11-4-generic-direct-text-intrinsic-input.md#L102)). |
| 18 | 새 input이 실수로 two-axis nested Grid helper에 들어가 범위보다 큰 계산 허용을 받는가 | PASS | `C11_4TextInput.two_axis_nested_grid`가 runtime 분기 helper 선택을 제어한다([fixtures.rs L67-L71](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures.rs#L67), [runtime_layout.rs L579-L597](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout.rs#L579)). 계획은 direct-text slice에서 `false`를 명시한다([계획 L37](./c11-4-generic-direct-text-intrinsic-input.md#L37)). |
| 19 | font registry의 private identity token을 읽을 수 있다고 가정해 검증 코드/API 범위를 몰래 넓히는가 | PASS | `TextIntrinsicInlineSizes`는 같은 타입 간 `uses_same_font_registry_revision()`을 공개하지만 최종 `TextLayoutSnapshot` token은 private다([snapshot.rs L253-L261](../crates/spinon-text-layout/src/snapshot.rs#L253), [L315-L329](../crates/spinon-text-layout/src/snapshot.rs#L315)). 계획은 raw→contribution identity와 공개 numeric revision을 각각 확인하고 서로 다른 타입 간 token 비교를 주장하지 않는다([계획 L109](./c11-4-generic-direct-text-intrinsic-input.md#L109)). |
| 20 | candidate 구조 오류가 legacy fallback에서 다른 부분 결과를 만들어 실패를 숨기는가 | SUPERSEDED | 원래 pass03는 candidate 오류의 no-fallback/no-partial 경계를 올바르게 확인했지만, 같은 행의 stylesheet rejection 설명은 원본 PR diff report row 28의 blocker와 맞지 않았다. candidate 오류 범위는 유지하고 stylesheet claim은 current acceptance로 재사용하지 않는다. 수정 계획은 inline `style` rejection 경계와 stylesheet scope를 분리한다([현재 계획](./c11-4-generic-direct-text-intrinsic-input.md#L112), [diff addendum](./c11-4-generic-direct-text-intrinsic-input-plan-pr-diff-review-addendum-2026-10-26.md)). |

## 남은 재현 한계와 구현 착수 판정

- **구현 착수 blocker 없음.** 이 계획은 제한 fixture slice의 caller, root/tree/class 입력, actual Stylo cascade, item→ParagraphStyle owner mapping, one-axis helper, raw/contribution/final snapshot assertion과 candidate rejection 동작을 기존 API에 맞춰 정했다. 실제 HostDocument builder와 runtime 동작은 아직 구현되지 않았으므로 구현 완료 gate에서 실행 증거를 만들어야 한다.
- 저장된 Chromium JSON은 root computed-style property map을 담지 않는다. 따라서 Chrome의 root `grid-auto-flow` serialization 자체는 재현할 수 없다. 근거는 hashed authored fixture, CSS의 row 기본값, 같은 행인 관측 geometry이며 Rust에서는 typed Stylo `auto_flow=Row`를 별도로 검증한다. 추후 exact cross-browser root-style equality를 주장하려면 root computed-style JSON field가 추가로 필요하다. 현재 계획은 그 주장을 하지 않으므로 implementation blocker는 아니다.
- 구현 전후 simulator 실행, installed app hash, artifact boundary 증거는 아직 없다. 계획은 이를 구현 후 gate로 한정하며 이 리뷰는 통과·완료 claim을 하지 않는다.

## pass02의 기존 17 PARTIAL 처리

아래는 fresh 관점 표와 별개로 이전 pass02에서 PARTIAL이던 17항목을 새 계획의 선행/후행 gate에 매핑한 것이다.

| 이전 관점 | 현재 처리 | 구현 착수 blocker / 다음 gate |
| ---: | --- | --- |
| 1 | **계획상 해소** | `prepare_c11_4_text`에서 computed item style을 ParagraphStyle로 변환하고 기존 Ahem engine과 fixture helper로 전달하는 경로를 지정했다. 구현 후 call path 회귀가 필요하다. |
| 2 | **계획상 해소** | scene별 별도 HostDocument와 mount root를 지정했다. builder 실제 실행은 slice 구현 acceptance다. |
| 3 | **계획상 해소** | scene root `x,y`를 browser root/item/text rect에서 빼고 Rust root frame (0,0)과 비교한다. |
| 4 | **계획상 해소** | line-local line metrics와 Range rect를 item-local 좌표로 정규화한다. |
| 5 | **계획상 해소** | JS 실행 대신 관측된 최종 `[item-a,item-z]`를 HostNode child order로 만든다. |
| 6 | **계획상 해소** | 이 slice는 computed `order:0`만 허용하고 nonzero 값은 fail-closed 하도록 정했다. |
| 7 | **계획상 해소** | scene별 width/height min/max와 preferred Auto 값을 typed matrix로 정했다. |
| 8 | **계획상 해소** | raw intrinsic source, `contribution_input()`, final CSS/Grid track output의 단계를 구분한다. |
| 9 | **계획상 해소** | MinContent/MaxContent callback probe와 final definite-width C15 snapshot을 분리해 판정한다. |
| 10 | **명시적 후속 범위** | aspect-ratio 및 item min/max/edge transfer는 현재 candidate에서 거부하고 별도 slice에서 검증한다. 현재 범위의 rejection negative control만 선행 구현 acceptance다. |
| 11 | **계획 수정; fresh review 대기** | `hyphens:none`은 candidate item의 inline `style` declaration negative로 구성하고 `CssCascadeError::UnsupportedInlineAuthorProperty { node, property: "hyphens" }`를 기대한다. 이는 inline preflight 경계만 확인하며 stylesheet rejection이나 CSS hyphenation 지원을 뜻하지 않는다. |
| 12 | **현재 profile에서 해소; 지원은 후속** | foreign namespace candidate는 source error로 닫는다. SVG/MathML text 지원은 제외한다. |
| 13 | **계획 수정; fresh review 대기** | `visibility:collapse`는 candidate item의 inline `style` declaration negative로 구성하고 해당 property 이름의 `UnsupportedInlineAuthorProperty`를 기대한다. 이는 visibility variants 계산이나 stylesheet rejection을 확인하지 않는다. |
| 14 | **계획상 해소** | separate HostDocument, classes, final child order, longhand stylesheet와 실제 Stylo cascade API를 지정했다. builder 구현 및 실행은 collector acceptance다. |
| 18 | **구현 후 runtime gate** | Android/iOS simulator source/build/install hash와 frame evidence를 수집하기 전에는 simulator/source-bound 결과를 주장하지 않는다. 이 gate는 collector 코딩을 막지 않는다. |
| 19 | **구현 후 artifact gate** | default/product 대 fixture-test positive/negative control을 실제 archive/app artifact 수준별로 수행한다. 정적 archive만으로 APK/IPA 제외를 주장하지 않는다. |
| 20 | **명시적 후속 범위** | source node와 원문은 현재 slice에서 보존·확인한다. 전 범위 UTF-16↔UTF-8 range remapping acceptance는 후속 slice다. |

**당시 집계:** 원래 pass03은 이전 17 PARTIAL 중 13개를 계획 계약 또는 fail-closed 동작으로 해소했다고 판정했다. aspect-ratio/box transfer와 full source-range remapping 2개는 명시적 후속 범위, simulator/runtime artifact와 binary boundary 2개는 구현 후 증거 gate였다. PR diff review 이후 row 20은 superseded 됐으므로 이 역사적 집계는 현재 payload의 blocker 없음 판정에 사용할 수 없다.

## 후속 PR diff blocker 정합화

pass03 row 20 및 기존 pass02 mapping #11/#13은 미지원 author declaration이 stylesheet preflight에서 닫힌다고 전제했으므로 현재 acceptance를 승인하는 데 재사용하지 않는다. 수정된 계획은 rejection control을 inline `style` attribute로 제한하고, verified API error `CssCascadeError::UnsupportedInlineAuthorProperty { node, property }`와 no-snapshot boundary를 기대한다. Positive `StylesheetSource`에는 계획된 longhand만 둔다. 제한 stylesheet의 unsupported declaration rejection은 이번 계획의 검증 주장 밖이다. 이 정합화는 문서 계획이며 negative controls를 실행한 결과가 아니다.

