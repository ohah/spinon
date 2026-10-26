# C11.4 generic direct-text 계획 PR diff 검토 — 수정 claim targeted pass02, 30 관점

**검토 checkout:** `/private/tmp/spinon-c11-4-row-axis-mixed-text-image-20261026`  
**branch / HEAD:** `plan/c11-4-generic-text-intrinsic-input-20261026` / `040a5360be92f73e141797aa7dcd4ef00ab24942`  
**base 확인:** 로컬 `origin/main`도 `040a5360be92f73e141797aa7dcd4ef00ab24942`; `git rev-list --left-right --count HEAD...origin/main`은 `0 0`이다.  
**검토 범위:** 수정 child plan, parent plan 역링크 diff, 사전 비교 README/fixture/JSON/PNG, 기존 PR diff report와 author targeted addendum, 그리고 변경 claim이 의존하는 cascade/runtime source 및 필수 프로젝트 규칙을 대조했다. 이 보고서 이외의 계획·evidence·제품 파일은 수정하지 않았다. 테스트, browser, simulator는 실행하지 않았다.

## 판정

**PASS 27 / PARTIAL 3 / FAIL 0.** 이전 diff-30의 실패였던 “StylesheetSource 미지원 선언이 cascade preflight에서 거부된다”는 주장은 현재 계획에서 제거됐다. Negative control은 각 미지원 속성을 item의 inline `style`로 보내고 실제 source에 있는 `UnsupportedInlineAuthorProperty` 경계를 기대하도록 고쳤다. 해당 세 속성에 대한 새 실행 증거는 아직 없지만 계획도 실행했다고 주장하지 않는다.

현재 최소 plan slice에 착수할 문서/아키텍처 blocker는 찾지 못했다. 다만 branch에는 실제 PR payload가 아직 없고, 이 fresh review가 child plan의 review index에 아직 연결되지 않았다. 세 property-specific negative control 역시 실행 전 계약이다. 아래 PARTIAL은 이 세 경계만 가리킨다.

## 수정 claim과 PR payload에 대한 30개 독립 관점

| # | 실패 관점 | 판정 | 근거 및 필요한 조치 |
|---:|---|:---:|---|
| 1 | checkout 이름이 다른 lane의 결과를 잘못 가리키는가 | PASS | 현재 branch는 text lane 이름이며 child plan/evidence 경로도 direct-text다. 계획 파일은 worktree에 있어 추후 commit 때 실제 diff 경로를 다시 확인해야 한다. |
| 2 | rebase 뒤 branch가 현재 로컬 base와 어긋나는가 | PASS | HEAD와 `origin/main`이 모두 `040a5360…`; 좌우 commit 차이 `0/0`이다. 이 판정은 로컬 ref 기준이며 remote 상태 주장은 아니다. |
| 3 | 리뷰 가능한 변경이 실제 tracked PR payload인가 | PARTIAL | `git status --short`에서 child plan, evidence, 앞선 review 문서들이 untracked이고 parent plan만 modified다. 보고서는 현 worktree 후보를 검토했으며 GitHub PR의 staged/committed diff는 아니다. PR 전 파일을 의도한 branch payload에 포함해야 한다. |
| 4 | 수정 범위에 계획 외 Rust 코드나 test 변경이 섞였는가 | PASS | 현재 변경 목록은 parent plan 문서 수정과 plan/evidence/review 파일뿐이다. Rust source/test 변경은 보이지 않는다. |
| 5 | parent roadmap에서 direct-text 후보를 찾을 수 있는가 | PASS | [parent plan](./c11-4-grid-track-sizing.md#2026-10-26-direct-element-text-intrinsic-input-계획)이 child plan 및 README를 역링크한다. 수정 위치는 C11.4 진행 기록에 있고 공식 checkbox와 완료 수를 올리지 않는다. |
| 6 | 공식 완료 수나 C11.4 checkbox가 이 계획만으로 승격되는가 | PASS | child plan과 parent entry 모두 계획 단계, 미실행 구현, C11.4 미완료 및 `72/213` 유지라고 적는다. 이는 프로젝트 규칙의 상태 대장을 대체하지 않는다. |
| 7 | 최소 fixture input slice가 Stage A 전체 또는 제품 CSS 지원으로 과장되는가 | PASS | child plan L5, L11, L119–123은 fixture-test direct element-text input으로 범위를 제한하고 HTML parser, 일반 CSS entry, text paint, Stage B 알고리즘과 전체 C11.4를 제외한다. |
| 8 | 기존 original diff-30 FAIL이 고쳐졌다는 기록을 덮어쓰는가 | PASS | [기존 diff report](./c11-4-generic-direct-text-intrinsic-input-plan-pr-diff-review-30-2026-10-26.md)는 이전 base와 당시 FAIL을 보존한다. [author addendum](./c11-4-generic-direct-text-intrinsic-input-plan-pr-diff-review-addendum-2026-10-26.md)은 해당 결과를 historical로 명시하고 fresh review인 척하지 않는다. |
| 9 | addendum이 수정하지 않은 모든 옛 판정을 새 결과처럼 재사용하는가 | PASS | addendum은 base, parent link, review trace, unsupported stylesheet negative 등 영향받은 행만 재대조하고 나머지는 original report에 둔다. 이 보고서는 그 변경 경계를 새로 대조한다. |
| 10 | fresh review 필요 여부가 child plan에서 숨겨지는가 | PARTIAL | child plan L127–129는 pass03/original report가 historical이고 fresh independent plan review가 필요함을 명시한다. 다만 이 새 targeted report는 현재 child plan review index에 링크되어 있지 않다. fresh plan reviewer와 함께 PR payload 전 review 링크 목록을 갱신해야 한다. |
| 11 | rebase된 parent link가 이전 branch 상태를 가리키는가 | PASS | parent 변경과 child의 상대 링크가 같은 checkout의 `plan/` 및 `spec/internal/evidence/`를 가리키며 두 target 파일이 존재한다. |
| 12 | inline negative 보정이 stylesheet negative claim을 다른 곳에 남겼는가 | PASS | child plan L82–84, L112와 evidence README L49, L61은 제한 `StylesheetSource`의 미지원 선언 거부를 주장하지 않는다고 반복해서 경계를 둔다. |
| 13 | inline negative의 error variant가 현재 API에 없는 가상 계약인가 | PASS | [`CssCascadeError`](../crates/spinon-style/src/stylo_dom/cascade.rs#L261)는 `UnsupportedInlineAuthorProperty { node, property }`를 정의한다. addendum과 계획은 같은 variant를 기대한다. |
| 14 | unsupported 속성은 실제 C11.4 inline allowlist에 이미 허용됐는가 | PASS | [`runtime_grid.rs`](../crates/spinon-style/src/stylo_dom/cascade/runtime_grid.rs#L126)의 allowlist에는 `visibility`, `hyphens`, `font-variant-ligatures`가 없다. 따라서 plan의 해당 control은 positive style support를 가정하지 않는다. |
| 15 | preflight가 inline declarations를 읽지 않고 error를 반환한다고 잘못 설명하는가 | PASS | C11.4 cascade는 [`first_unsupported_inline_property`](../crates/spinon-style/src/stylo_dom/cascade/runtime_layout.rs#L392)를 부르며 element inline declaration의 property name을 검사한다. plan은 negative 선언 위치를 item inline `style`로 고쳤다. |
| 16 | error가 computed snapshot 생성 뒤에 발생할 수 있는가 | PASS | [`runtime_grid.rs`](../crates/spinon-style/src/stylo_dom/cascade/runtime_grid.rs#L299-L318)에서 inline allowlist 오류는 `compute_cascade` 호출 전에 반환된다. 따라서 계획은 snapshot 미반환을 기대한다. |
| 17 | 실패 node가 scene root나 Text node로 잘못 매핑되는가 | PASS | scanner는 inline declaration을 소유한 element handle의 `NodeId`를 반환한다. 별도 negative item에 단일 미지원 declaration을 놓으면 기대 node는 item element ID라는 계약이 source와 일치한다. |
| 18 | 세 속성별 실제 거부 동작이 재현되어 증명됐는가 | PARTIAL | source gate와 기존 inline `text-orientation` test precedent는 있지만, `hyphens:none`, `visibility:collapse`, `font-variant-ligatures:none` 각각의 새 control 실행 결과는 없다. README와 plan이 이를 명시하므로 지금 plan review를 막지는 않지만 구현 acceptance 전 exact node/property와 snapshot 미생성 assertion을 실행해 기록해야 한다. |
| 19 | 기존 disabled-inline 검사만으로 세 속성 거부를 설명하는가 | PASS | disabled declaration scan은 text-orientation만 검사한다([`runtime_grid.rs`](../crates/spinon-style/src/stylo_dom/cascade/runtime_grid.rs#L397)). 수정된 계획은 세 속성을 그 scan에 귀속하지 않고 먼저 실행되는 generic inline allowlist scan에 귀속한다. |
| 20 | 지원되지 않는 속성 rejection을 해당 CSS 계산 지원으로 확대하는가 | PASS | child plan L84/L112 및 README L49는 control을 rejection-only라고 하고 visibility/hyphens/ligature 기능 구현을 주장하지 않는다. |
| 21 | positive sheet가 inline-only gate와 모순되는 미지원 longhand를 선언하는가 | PASS | child plan L42–74의 positive CSS는 지원 범위의 longhand만 쓰며 negative 속성은 별도 candidate의 inline control로 분리한다. |
| 22 | current source가 실제 snapshot 대신 hand-authored computed style을 쓰도록 계획하는가 | PASS | child plan L76–78은 Stylo cascade API를 실행해 item 및 root typed snapshot을 먼저 대조하고, hand-authored `ComputedElementStyle` 입력을 명시적으로 금한다. |
| 23 | CSS→`ParagraphStyle` mapping이 추상적 설명만 있고 실제 field 계약이 없는가 | PASS | child plan L35–38, L80–84는 `ParagraphStyle` field, typed font-size/writing style, 기존 converter precedent, text helper 시그니처를 지목한다. plan-only라는 경계도 유지한다. |
| 24 | generic collector가 기존 adapter를 먼저 처리한다는 삽입 위치가 source와 다른가 | PASS | 실제 `prepare_c11_4_text`는 여러 special adapter 이후 `pending = vec![root]` scan을 시작한다([`c11_4_text.rs`](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_text.rs#L69), [scan](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_text.rs#L127)). 계획은 generic collector를 그 둘 사이에 둔다. |
| 25 | 기존 `TEXT_CASES` fallback 보존 주장이 source 흐름과 충돌하는가 | PASS | 현 adapter의 generic-looking tail도 root/item/text identity를 `TEXT_CASES`와 대조한다([`c11_4_text.rs`](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_text.rs#L217)). 계획은 그 fallback 회귀를 유지하고 generic slice를 앞에 둔다. |
| 26 | runtime 호출 경로에 text engine과 paragraph map이 없어 계획이 실제 API에 못 붙는가 | PASS | runtime의 `C11_4TextInput` 분기는 `two_axis_nested_grid=false`일 때 일반 `..._with_text_paragraphs` helper로 연결된다([`runtime_layout.rs`](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout.rs#L579)). child plan은 이 fixture 경로와 기존 API만 사용한다. |
| 27 | scene ID/script order가 Rust parser 실행으로 잘못 주장되는가 | PASS | README L55–65와 child plan L88–94는 Rust builder가 별도 `HostDocumentSnapshot`과 관측된 final child order를 구성할 계획이며 browser JS/HTML parser는 실행하지 않는다고 구분한다. |
| 28 | Chrome geometry와 Rust mount-root geometry의 좌표 기준이 섞이는가 | PASS | child plan L17–28/L92 및 README L55는 browser 문서 좌표에서 각 root origin을 빼고, Rust scene별 mount-root `(0,0)` frame 및 item/text local rect를 비교하도록 정의한다. screenshot pixel을 계산 assertion에 쓰지 않는다. |
| 29 | 수정된 evidence/hash가 기존 observation과 다른 입력을 가리키는가 | PASS | 현 파일 hash는 fixture `1b2bdf7d…e5b7f52`, Ahem `b719ecb3…94b8448`, JSON `ae44f2a2…7a15f22`, PNG `1d3d1377…226474`로 README와 일치한다. PNG는 1280×633 RGB다. |
| 30 | simulator, artifact, text paint 또는 전체 Stage A 근거가 이미 실행된 것처럼 보이는가 | PASS | child plan L114–123은 Android/iOS simulator source/build/install hash 및 artifact 경계를 구현 후 gate로 두며 glyph paint, compositor pixels, general product support와 Stage A 전체를 현재 주장하지 않는다. 이는 project rules의 실행 근거·artifact boundary 구분과 맞는다. |

## 남은 조치와 판정 경계

- **현재 계획 구현의 문서 blocker:** 없음. 이전 stylesheet-negative 불일치는 source-level 계약상 해소됐다.
- **PR payload:** child plan/evidence/review 문서가 아직 untracked이므로 branch 기반 PR diff 검토가 아니다. PR 생성 전에 의도한 문서 payload를 branch에 포함하고 이 보고서 및 fresh plan review를 child review index에 연결한다.
- **구현 acceptance:** 세 inline property control은 아직 실행되지 않았다. 구현 후 각 control의 cascade error (`node`, `property` 정확값), snapshot/layout 산출 부재를 실행 결과로 기록한다.
- 이번 작업은 review-only였다. 제품 코드, 계획 본문, evidence를 수정하지 않았고 테스트/browser/simulator를 실행하거나 commit하지 않았다.
