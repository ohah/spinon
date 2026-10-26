# C11.4 generic direct-text 계획 PR diff 검토 — 30 관점

**검토 checkout:** /private/tmp/spinon-c11-4-row-axis-mixed-text-image-20261026  
**branch / HEAD:** plan/c11-4-generic-text-intrinsic-input-20261026 / a33b245983fae050b66efb9ff259b3fe1132a051  
**base 확인:** 로컬 origin/main은 040a5360be92f73e141797aa7dcd4ef00ab24942; git rev-list --left-right --count HEAD...origin/main은 0 3이다. 이 보고서는 현재 로컬 ref를 기준으로 하며 remote fetch를 하지 않았다.  
**범위:** child plan, pass01/02/03 review, precomparison README/fixture/JSON/screenshot, parent plan와 실제 source/API 경계를 읽기 전용으로 검토했다. 테스트, browser, simulator는 실행하지 않았고, plan/evidence/product code도 수정하지 않았다. 요청된 새 review report만 만들었다.

## 판정

**PASS 24 / PARTIAL 5 / FAIL 1.** 최소 direct-text positive slice는 현재 C11.4 fixture API에 연결할 수 있도록 충분히 좁고, Chrome/독립 기대값과 아직 실행하지 않은 runtime acceptance도 구분한다. 다만 후보가 PR-ready한 branch diff는 아니다: worktree payload가 untracked이고 branch가 로컬 origin/main보다 3 commits 뒤이며 parent index에 link가 없다. 또 unsupported CSS author declarations의 preflight negative contract가 StylesheetSource input과 현재 cascade gate의 실제 범위에서 일치하지 않는다.

## 새 PR diff 실패 관점 30개

| # | 독립 실패 관점 | 판정 | 근거와 필요한 조치 |
|---:|---|:---:|---|
| 1 | 계획 후보의 branch 이름과 HEAD가 서로 다른 작업을 가리키는가 | PASS | git branch --show-current는 plan/c11-4-generic-text-intrinsic-input-20261026, HEAD는 a33b2459다. git log에는 같은 HEAD에 image-plan branch ref도 있어 ref 이름 혼동 위험은 있으나 현재 checkout/작업 내용은 지정된 text lane임을 plan/evidence 경로로 구분했다. |
| 2 | 계획 branch가 현재 로컬 base를 이미 포함하는가 | PARTIAL | HEAD a33b2459는 origin/main 040a5360보다 3 commits 뒤다. 그 사이 local origin에는 span-stage test/evidence와 parent plan 변경이 있다. PR 전 최신 main에 rebase한 뒤 충돌 및 최신 parent entry를 다시 확인해야 한다. |
| 3 | 실제 PR diff로 검토 가능한 tracked payload가 존재하는가 | PARTIAL | git status --short에서 child plan, 세 review 문서, evidence 디렉터리가 모두 untracked이고 parent plan 변경은 없다. 이 결과는 현재 worktree의 후보 검토이며 GitHub PR payload나 branch에 커밋된 변경을 확인한 결과가 아니다. |
| 4 | C11.4 parent plan에서 새 text lane을 찾을 수 있는가 | PARTIAL | plan/c11-4-grid-track-sizing.md는 Stage B span-stage 항목을 포함하지만 c11-4-generic-direct-text-intrinsic-input.md 또는 그 evidence를 링크하지 않는다. Child plan은 parent를 역링크한다(L3). PR 통합 전에 parent index에도 child plan과 precomparison evidence 링크를 추가해야 한다. |
| 5 | Child plan이 parent contract와 공식 상태를 식별하는가 | PASS | Child plan L3는 parent c11-4-grid-track-sizing.md, 내부 계약 0059, 공식 상태 anchor를 연결한다. 계획 자체는 parent 완료 선언 대신 C11.4 하위 lane임을 명확히 한다. |
| 6 | 이 계획 후보가 공식 C11.4 완료나 72/213을 올리는가 | PASS | Child plan L5, L121–123은 checkbox와 72/213을 유지하고 제한 fixture input slice만 완료 대상으로 한다. 현재 spec/STATUS.md C11.4도 미완료다. |
| 7 | 변경 범위가 Stage A 전체·제품 parser·track algorithm을 묶어 과장하는가 | PASS | Child plan L5, L11, L119–123은 direct element-text collector를 c11-4-grid-track-sizing-fixture-test feature로 한정하고 Stage A 전체, parser, product entry, Stage B track algorithm, WPT를 제외한다. |
| 8 | 기존 20-view pass01 판정이 재사용 없이 별도 실패 관점을 기록했는가 | PASS | pass01은 20행이며 5 PASS/15 PARTIAL로 집계한다(pass01 L7–34). item participation, mixed source, text style, font source, coordinates 등을 개별 gate로 적었다. |
| 9 | pass02가 pass01과 다른 관점으로 계획/API 부족을 찾았는가 | PASS | pass02는 별도 20행, 3 PASS/17 PARTIAL이며 caller engine/API, mount root, DOM order, final C15 snapshot, source/font revision, simulator/artifact 연결을 구체화했다(pass02 L1–40). |
| 10 | pass03 fresh review가 현재 branch HEAD에서 검토했다고 정확히 기록하는가 | PARTIAL | pass03은 본표 20 PASS지만 header의 기준 HEAD는 b58de5df다(pass03 L1–5). 현재 checkout은 a33b2459이며, 그 사이 product source 변경은 없고 span-stage 문서/evidence 변경만 확인됐다. rebase 후 parent integration hunk를 포함한 최종 diff review를 갱신해야 한다. |
| 11 | pass03이 이전 17 PARTIAL의 처리를 숨기거나 집계에서 누락하는가 | PASS | pass03은 기존 17 PARTIAL을 13개 계획 계약/거부 경계, 2개 명시적 후속 범위, 2개 구현 후 증거 gate로 각 행에 매핑한다(pass03 L44–64). 선행 implementation blocker가 없다는 결론은 해당 분류와 일치한다. |
| 12 | 세 차례 계획 리뷰의 결과를 child plan/parent index에서 따라갈 수 있는가 | PARTIAL | 세 리뷰 파일은 worktree에 있지만 child plan의 본문/상태에는 pass01/02/03 review 링크가 없고 parent index에도 text lane이 없다. PR에서 결과가 보이더라도 연결 경로를 추가해야 이후 코드/roadmap 작업자가 현재 판정과 후속 경계를 찾기 쉽다. |
| 13 | 후보 payload에 제품 소스나 test 구현이 섞여 있는가 | PASS | git status --short는 계획 review 문서, child plan, evidence directory만 새 항목으로 보인다. tracked product source 또는 Rust test 수정은 없다. 실제 Stage A 구현을 이미 했다는 claim도 없다. |
| 14 | Precomparison evidence fixture가 app/parser input인 것처럼 표현되는가 | PASS | Evidence README L47–65와 child plan L88–94는 browser fixture/JSON을 Chrome oracle로만 다루고 Rust HostDocument builder는 미작성·미실행으로 분리한다. Browser script 실행을 Rust parser 동작으로 확대하지 않는다. |
| 15 | HTML fixture hash가 README의 캡처 입력과 같은가 | PASS | fixture.html SHA-256 재계산은 1b2bdf7d…e5b7f52로 README L15의 값과 일치한다. CSS/DOM 변형과 grid-reorder script를 고정 입력으로 추적할 수 있다. |
| 16 | Ahem bytes가 계획의 browser/runtime font input과 같은가 | PASS | tests/fixtures/resources/c15/Ahem.ttf SHA-256 재계산은 b719ecb3…94b8448로 README L11/L15와 일치한다. README L13은 font-ready 및 document.fonts.check('10px Ahem') 관측이라고 한정한다. |
| 17 | Chrome observation JSON이 README에 고정된 파일인가 | PASS | chromium-observation.json SHA-256 재계산은 ae44f2a2…7a15f22로 README L15에 일치한다. JSON은 six scene별 computed item styles, tracks, rects, text rects와 grid-reorder의 final DOM order를 담는다. |
| 18 | PNG screenshot이 다른 viewport나 누락 파일로 바뀌었는가 | PASS | Screenshot SHA-256 재계산은 1d3d1377…226474로 README L15와 일치하며 file은 1280×633 RGB PNG로 확인했다. 이는 browser screenshot이며 app paint 또는 simulator evidence가 아니다. |
| 19 | Browser/font 실행 근거가 source 측정 전제와 연결되는가 | PASS | README L12–16은 Agent Browser CLI/Chrome 156, DPR 1, viewport, document.fonts.ready, 전용 session 종료, local fixture server 종료를 기록한다. 이 기록은 이전 capture evidence이며 이번 review에서 browser를 재실행하지 않았다. |
| 20 | 기존 baseline test를 여섯 custom scene의 Spinon runtime 결과로 과장하는가 | PASS | README L35–45는 공통 builder의 UnsupportedTextNode baseline command 결과 1 passed와 six-scene fixture runtime 미실행을 분리한다. 그 test 결과는 새 collector가 이미 통과했다는 증거로 쓰이지 않는다. |
| 21 | HTML의 grid-reorder DOM mutation을 계획된 HostDocument input과 연결하는가 | PASS | HTML L19–25는 script로 최종 [item-a,item-z]를 만들고 observation JSON은 그 final order를 기록한다. Child plan L90은 Rust builder가 script를 실행하지 않고 같은 final child order를 직접 구성한다고 제한한다. |
| 22 | CSS selector/class owner와 계획의 제한 longhand stylesheet가 다르게 계산될 수 있는가 | PASS | HTML L7–12는 root nowrap/break-all 및 item break-all selector를 구분한다. Child plan L42–76은 .scene, .item, scene/item class를 보존하고 browser shorthand/@font-face/.fixed placement rule을 Rust stylesheet에서 제외한 뒤 실제 Stylo cascade snapshot으로 확인하도록 한다. |
| 23 | 독립 Ahem 계산이 Chrome output을 그대로 oracle로 재사용하는가 | PASS | README L18–29는 10px glyph advance, break opportunity, gap, fixed root size에서 min/max 기대값을 도출하고 Chrome 값은 별도 관측한다. Child plan L17–28과 JSON/rect matrix도 CSS px 값과 local-frame 정의를 분리한다. |
| 24 | Browser JSON에 없는 root computed styles를 이미 관측한 것처럼 주장하는가 | PASS | README L63–65는 root computed property map이 JSON에 없음을 명시한다. 계획은 author fixture hash, actual Stylo typed root snapshot(auto_flow, dimensions, gap, display, alignment)과 item geometry를 별도 검사하고 Chrome root computed map을 소급해 주장하지 않는다. |
| 25 | 계획의 CSS→ParagraphStyle 변환이 실제 필드/API와 맞는가 | PASS | Child plan L35–38, L80–84는 실제 ParagraphStyle fields, font_size_css_px, typed writing style, existing root-html conversion precedent와 C11.4 text helper signature를 지정한다. ParagraphStyle 실제 fields는 crates/spinon-text-layout/src/model.rs:124–142에 있고 helper는 projection.rs:477–496이다. |
| 26 | style map owner와 원본 Text NodeId를 같은 identity로 취급하는가 | PASS | 계획 L35–36/L109는 BTreeMap<NodeId, ParagraphStyle> key와 final snapshot paragraph owner는 item element로, TextSource.node는 direct Text child로 각각 확인한다. 실제 TextSource는 model.rs:232–245에서 분리 field를 갖는다. |
| 27 | font registry identity/revision 또는 raw intrinsic size가 final CSS contribution과 혼동되는가 | PASS | 계획 L28/L109–111은 TextIntrinsicInlineSizes raw width/height, contribution_input()의 preferred/min/max/edge 값, final definite-width C15 snapshot을 나눈다. 같은-engine 전달과 uses_same_font_registry_revision()은 snapshot.rs:253–260, raw contribution 변환은 intrinsic_measurement.rs:222–231의 실제 API를 따른다. |
| 28 | StylesheetSource의 unsupported author declaration이 계획대로 cascade preflight에서 거부되는가 | FAIL — blocker | 계획 L42/L76은 실제 StylesheetSource author sheet를 cascade하고 L84/L112는 allowlist 밖 declaration이 collector 전에 preflight error가 된다고 기대한다. 실제 stylesheet preflight는 author sheet에서 text-orientation만 scan한다(runtime_grid.rs:274–295). 일반 allowlist 검사와 disabled-declaration 검사는 view의 inline styles만 본다(runtime_grid.rs:299–315,397–409; first_unsupported_inline_property는 runtime_layout.rs:392–410). 따라서 계획서가 선언한 hyphens/font-variant-ligatures 등 unsupported stylesheet negative가 preflight error로 닫힌다고 보장할 수 없다. Negative mutation을 inline style로 만들어 실제 UnsupportedInlineAuthorProperty를 assert하거나, stylesheet author-property scan을 별도 설계해야 한다. |
| 29 | 후보 오류가 fallback이나 일부 생성 결과로 흡수될 수 있는가 | PASS | Child plan L98–103은 legacy one-item Ok(None) fallback과 ambiguous/mixed candidate errors를 구조 조건으로 분리하고, candidate source/style/grid mismatch는 특정 오류 및 partial C11_4TextInput 미생성으로 닫는다. 실행 전 acceptance라고 분명히 한다. |
| 30 | simulator/app binary 및 fixture artifact gate를 이미 완료한 것처럼 보고하는가 | PASS | Child plan L114–117/L121–123은 Android/iOS simulator source/build/install hash와 artifact boundary를 구현 후 gate로 둔다. L5와 L123은 플랫폼·artifact 근거가 아직 없고 일반 product runtime 지원이나 공식 C11.4 완료가 아니라고 제한한다. |

## PR-ready 판정과 필요한 수정

- **PR-ready 아님 — branch integration:** local origin/main보다 3 commits 뒤다. 최신 main에 rebase 후 parent plan 변경과 모든 새 file을 포함한 실제 tracked diff를 다시 검토해야 한다.
- **PR-ready 아님 — roadmap linkage:** 최신 plan/c11-4-grid-track-sizing.md에 generic direct-text child plan과 evidence link를 추가해야 한다. 현재는 child → parent 단방향 링크다. pass03의 review 기준 commit도 current base로 갱신하거나 rebase 영향을 명시해야 한다.
- **Acceptance blocker — unsupported CSS author negative:** 계획은 stylesheet cascade를 사용하면서 stylesheet allowlist 거부를 가정한다. StylesheetSource negative는 inline style negative와 다르다. Negative tests를 inline declaration 및 실제 error code로 고정하거나 stylesheet preflight 정책을 명시적으로 설계해야 한다. 이 문제는 six positive input을 구현하는 길 자체를 막지는 않지만, 현재 계획의 fail-closed contract를 충족하지 못하게 한다.
- **Review traceability:** 세 pass review 문서가 payload에 포함될 예정이면 child 또는 parent index에서 pass03 및 historical review를 연결한다. C11.4 checkbox와 공식 72/213은 변경 대상이 아니다.

읽기 전용 review 외 변경은 하지 않았다. 테스트·browser·simulator를 실행하거나 commit/remote PR 상태를 확인하지 않았다.
