# C11.4 direct element-text intrinsic 입력 계획 독립 검토 pass04 — 20 관점

**검토 checkout:** `/private/tmp/spinon-c11-4-row-axis-mixed-text-image-20261026`  
**branch / 기준:** `plan/c11-4-generic-text-intrinsic-input-20261026` / HEAD `040a5360be92f73e141797aa7dcd4ef00ab24942`  
**base 대조:** `origin/main`도 `040a5360be92f73e141797aa7dcd4ef00ab24942`; `git rev-list --left-right --count HEAD...origin/main`은 `0 0`이다.  
**범위:** 현재 child plan, precomparison evidence, 실제 C11.4 cascade/runtime/layout API와 프로젝트 규칙을 새 관점으로 대조했다. 이전 pass01–03 표의 관점·판정을 재사용하지 않았다. 계획 본문, evidence, 제품 소스/test는 수정하지 않았다. 테스트·browser·simulator를 실행하거나 commit하지 않았다.
**worktree 경계:** child plan과 evidence는 아직 untracked이므로 이 검토는 현재 worktree plan 검토이며 committed PR payload 검토가 아니다.

## 판정

**PASS 16 / PARTIAL 4 / FAIL 0.** 수정된 unsupported-negative 계약은 실제 C11.4 inline preflight의 `UnsupportedInlineAuthorProperty { node, property }` 경계에 맞는다. 이 source boundary는 확인됐지만, 세 property-specific control은 아직 실행되지 않았다. 따라서 구현 acceptance 증거와 source-level 예상 경계를 구분했다.

계획은 구현 가능한 최소 fixture-test slice와 현재 API 경로를 대부분 고정한다. 구현을 시작하기 전에 두 계약 표현을 분명히 하는 편이 안전하다: helper 결과가 실제 `StyleLayoutOutput.layout` 안에 있다는 점, 그리고 generic candidate의 direct item 허용 개수를 정확히 두 개로 제한할지 여부다. 나머지 PARTIAL은 세 property control의 실행 evidence와 현재 plan review index의 신선도 경계다.

## 독립 실패 관점 20개

| # | 실패 관점 | 판정 | 근거 및 필요한 보완 |
|---:|---|:---:|---|
| 1 | 계획만으로 C11.4 공식 상태나 완료 수를 잘못 올리는가 | PASS | child plan L5/L121–123은 C11.4 checkbox와 `72/213`을 유지한다. 공식 상태 대장 [STATUS](../spec/STATUS.md#c11-css-grid)도 C11.4 미완료로 표시한다. |
| 2 | Stage A 전체, 제품 CSS entry 또는 HTML parser 구현을 이 최소 slice로 포장하는가 | PASS | child plan L5/L11/L119–123은 fixture-test direct element-text input으로 범위를 제한하고 Stage A 전체·일반 CSS admission·parser·text paint·Stage B 알고리즘을 제외한다. |
| 3 | 계획한 text route가 현재 helper API에 연결될 수 있는가 | PASS | [`projection.rs`](../crates/spinon-style-to-layout/src/projection.rs#L477)는 feature-gated `...with_text_paragraphs` helper와 `paragraph_styles`, `TextLayoutEngine` 입력을 제공한다. runtime의 `two_axis_nested_grid=false` 분기는 같은 helper를 호출한다([`runtime_layout.rs`](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout.rs#L579)). |
| 4 | 새 collector가 기존 고정 adapter보다 먼저 실행되어 회귀를 덮는가 | PASS | 계획은 `prepare_c11_4_text`의 special dispatch를 보존하고 legacy root-wide scan 직전에 삽입한다. 실제 special adapter들은 현재 `pending = vec![root]` scan 앞에서 순차 호출된다([`c11_4_text.rs`](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_text.rs#L69)). |
| 5 | fallback과 candidate의 경계가 잘못된 scene을 조용히 부분 수집하는가 | PASS | plan L101–102는 비후보만 `Ok(None)`으로 기존 fallback에 넘기고, direct Grid text candidate의 source/style/grid mismatch는 오류로 닫으며 partial input을 금한다. |
| 6 | candidate의 item 수가 구현자마다 달라질 수 있는가 | PARTIAL | plan L102는 직접 item 후보가 2개 이상이면 candidate라고 하고 item 수 불일치를 오류로 적지만 허용 count를 명시하지 않는다. 여섯 oracle scene은 모두 정확히 두 item이며 L109도 scene당 두 intrinsic input을 기대한다. 착수 전 정확히 2개로 제한할지, 더 많은 item을 지원할지 고정해야 한다. |
| 7 | anonymous/direct/descendant text와 image 혼합을 구별하지 못하는가 | PASS | plan L99/L102는 Grid 직속 anonymous Text, descendant/복수 Text source 및 replaced image+text candidate를 별도 source 오류로 닫는다. 기존 anonymous/mixed adapter의 선행 경로도 보존한다. |
| 8 | source 순서와 item identity가 fixture root ID나 literal에 결박되는가 | PASS | plan L90/L98/L102/L112는 scene ID/root ID/item ID/text literal을 collector dispatch/style key에서 배제하고 source order와 NodeId owner를 사용한다. `grid-reorder` final order는 JSON에도 `[item-a,item-z]`로 기록됐다. |
| 9 | HTML whitespace-only Text node가 Grid item이나 text source로 잘못 추가되는가 | PASS | plan L99는 collapsible HTML whitespace-only Text를 독립 source로 만들지 않고, `grid-ws` 원문 `" A  BC "`는 item source로 보존한 뒤 paragraph `white_space`가 collapse를 맡도록 한다(L109). |
| 10 | computed style map이 paragraph owner가 아닌 Text NodeId에 붙는가 | PASS | plan L35–37은 `ComputedElementStyle.node_id == item.id()`를 찾고 `BTreeMap<NodeId, ParagraphStyle>` key도 item ID로 둔다. 원본 Text node ID/text는 별도 `input.sources`에서 검사한다. |
| 11 | `ParagraphStyle` 변환이 실제 자료형에 없는 CSS 동작을 지원한다고 가정하는가 | PASS | 실제 [`ParagraphStyle`](../crates/spinon-text-layout/src/model.rs#L124)은 plan에 나열된 font/line/white-space/writing/direction/ligature 필드를 가진다. 표현하지 않는 `text-align`, spacing/indent, transform 등은 지정 기본 computed 값만 허용하도록 plan L36/L80–84가 제한한다. |
| 12 | positive StylesheetSource가 cascade profile에 없는 longhand를 요구하는가 | PASS | C11.4 snapshot property 목록은 base layout/Grid properties와 추가 text longhand를 결합한다([`runtime_grid.rs`](../crates/spinon-style/src/stylo_dom/cascade/runtime_grid.rs#L43), [track properties](../crates/spinon-style/src/stylo_dom/cascade/runtime_grid.rs#L155)). plan CSS는 제한 longhand만 사용하고 `@font-face`, font/white-space shorthand 및 `.fixed` placement rule을 제외한다. stylesheet declarations의 일반 거부를 주장하지 않는다. |
| 13 | root typed style acceptance가 저장 JSON에 없는 Chrome computed map을 증명한다고 과장하는가 | PASS | plan L78과 evidence README L63은 root computed property map이 JSON에 없다고 명시하고, author input과 실제 Stylo typed root snapshot을 별도 검사한다. item computed values만 JSON과 비교한다. |
| 14 | inline unsupported property source boundary가 존재하는가 | PASS | C11.4 cascade는 `compute_cascade` 전에 `first_unsupported_inline_property`를 실행하고 allowlist 밖 inline declaration에서 `{ node, property }`를 반환한다([`runtime_grid.rs`](../crates/spinon-style/src/stylo_dom/cascade/runtime_grid.rs#L299), [`runtime_layout.rs`](../crates/spinon-style/src/stylo_dom/cascade/runtime_layout.rs#L392), error variant [`cascade.rs`](../crates/spinon-style/src/stylo_dom/cascade.rs#L261)). `visibility`, `hyphens`, `font-variant-ligatures`는 해당 C11.4 allowlist에 없다. |
| 15 | source 경계가 세 property control의 실행 성공으로 잘못 보고되는가 | PARTIAL | plan L84/L112와 evidence README L49는 property별 rejection controls가 아직 실행되지 않았다고 적는다. 기존 test는 inline `text-orientation`에서 같은 error variant만 실행 확인한다([`orthogonal_writing_mode.rs`](../crates/spinon-style/src/stylo_dom/cascade/runtime_grid_tests/orthogonal_writing_mode.rs#L75)). 세 control 각각의 exact node/property 및 snapshot 미생성은 구현 acceptance 전에 실행해 기록해야 한다. |
| 16 | helper 반환값의 실제 접근 path가 plan acceptance와 일치하는가 | PARTIAL | helper는 `Result<StyleLayoutOutput, _>`를 반환하며 intrinsic inputs, measurements, frames와 text layouts는 `StyleLayoutOutput.layout: LayoutOutput` 안에 있다([`projection.rs`](../crates/spinon-style-to-layout/src/projection.rs#L52), [`LayoutOutput`](../crates/spinon-layout/src/lib.rs#L187)). plan L109–111은 `LayoutOutput.*`라고 직접 표기한다. 착수 전 `output.layout.grid_item_intrinsic_inputs`, `output.layout.text_layouts[item_id]...`처럼 wrapper 경유 path를 적어야 한다. |
| 17 | Ahem 글꼴 등록과 intrinsic/definite layout 사이 registry 소유권이 끊기는가 | PASS | plan L37/L109는 시스템 폰트 없이 저장소 Ahem bytes를 등록한 동일 `TextLayoutEngine` 참조를 helper에 전달한다. `TextIntrinsicInlineSizes`는 numeric revision과 opaque token을 보존하며 `uses_same_font_registry_revision`이 둘 모두 비교한다([`snapshot.rs`](../crates/spinon-text-layout/src/snapshot.rs#L136)). |
| 18 | raw intrinsic 크기가 width/height contribution 후보 또는 최종 CSS contribution과 혼동되는가 | PASS | plan L28/L109–111은 raw `TextIntrinsicInlineSizes`, `contribution_input()`의 width·height min/max/preferred/edge, final definite C15 text layout을 나눈다. 실제 변환은 physical width/height min/max를 source에서 읽고 box fields를 `item_style`에서 보존한다([`intrinsic_measurement.rs`](../crates/spinon-layout/src/intrinsic_measurement.rs#L116), [변환](../crates/spinon-layout/src/intrinsic_measurement.rs#L222)). |
| 19 | Chrome 좌표, DOM order, CSS auto-placement를 Rust layout 좌표와 혼동하는가 | PASS | plan L88–94와 README L55–63은 scene별 mount root `(0,0)`, root 원점 정규화, grid-reorder 최종 child order, no-JS builder를 지정하고 browser JSON이 root computed style을 담지 않는 한계를 유지한다. fixture/font/JSON/PNG의 저장 hash는 README와 일치한다. |
| 20 | 이번 fresh pass04 결과를 child plan review index에서 찾을 수 있는가 | PARTIAL | child plan L127은 pass01/02와 historical pass03만 연결하고, L129–130은 fresh review가 여전히 필요하다고 적는다. 이 pass04 보고서는 아직 링크되지 않았으므로 PR 전 해당 링크와 상태 문구를 갱신해야 한다. |

## 남은 gate

- **구현 전 문서 명확화:** candidate가 정확히 두 direct item만 허용하는지, 다른 count를 어떤 candidate 오류로 닫는지 고정한다.
- **구현 전 실제 API 명확화:** helper 결과 `StyleLayoutOutput`과 내부 `layout` 경유 경로를 intrinsic input/contribution/final text snapshot assertion에 적는다.
- **구현 acceptance 증거:** 기존 inline preflight source와 `text-orientation` precedent는 세 property control 실행을 대신하지 않는다. 각 control의 exact error와 layout/snapshot 미생성을 실행해 남긴다.
- **review 연결:** child plan의 pass04 링크와 현재 게이트 문구를 이 보고서 작성 뒤 갱신한다.
- **PR 경계:** child plan/evidence/review 파일이 현재 untracked이므로 이 검토는 branch PR diff 검토가 아니다. 의도한 문서 payload 포함 여부를 PR 준비 때 다시 확인한다.
- **실행 제한:** 이 리뷰는 source/evidence를 읽기만 했다. 테스트, browser, simulator 및 commit은 실행하지 않았다.
