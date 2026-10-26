# C11.4 direct element-text intrinsic 입력 계획 독립 검토 · pass05

**검토 대상:** `/private/tmp/spinon-c11-4-row-axis-mixed-text-image-20261026/plan/c11-4-generic-direct-text-intrinsic-input.md` 및 연결된 사전 비교 evidence, 상위 계획, 관련 C11.4 API/source boundary.

**기준 상태:** `HEAD`와 `origin/main`은 모두 `040a5360be92f73e141797aa7dcd4ef00ab24942`다. Child plan/evidence는 local worktree candidate이고 parent plan에는 수정분이 있다. 전체 계획 및 원본 증거를 fresh하게 대조했다. 테스트, browser, simulator, commit은 실행하지 않았으며 계획/evidence/product source를 수정하지 않았다.

## 판정

**PASS 17 / PARTIAL 3 / FAIL 0.** 정확히 두 direct text-item candidate와 legacy fallback 경계, `StyleLayoutOutput.layout` 경로, source identity assertions는 현재 API와 맞는다. 기능 계약 blocker는 없다. 남은 partial은 (1) remote PR diff/CI payload 없음, (2) 사전 비교 cleanup 및 baseline 실행의 원문 transcript/source revision provenance 부족, (3) 이 pass05 보고서를 아직 child index에 연결하지 않았고 기존 문구가 fresh review를 pending으로 표시하는 상태다. 구현 시 `hyphens`, `visibility`, `font-variant-ligatures` inline rejection controls는 명시된 acceptance gate로 실행해야 한다.

## Fresh failure perspectives 20개

| # | 실패 관점 | 판정 | 대조 결과 |
|---:|---|:---:|---|
| 1 | plan branch가 최신 기준 main에서 벗어났는가 | PASS | checkout `HEAD`와 local `origin/main`이 모두 `040a5360be92f73e141797aa7dcd4ef00ab24942`다. |
| 2 | local plan/evidence candidate를 remote PR diff 또는 CI 결과로 잘못 표시하는가 | PARTIAL | Child plan/evidence는 worktree에서 untracked이고 parent plan은 modified다. 이 pass는 실제 GitHub PR head/diff/check verdict가 아니다. |
| 3 | 이 slice 때문에 C11.4 공식 상태나 완료 수가 승격되는가 | PASS | Child plan은 공식 checkbox와 `72/213`을 유지한다. 상위 [Grid 계획](./c11-4-grid-track-sizing.md#2026-10-26-direct-element-text-intrinsic-input-계획)도 계획-only 및 `72/213`을 기록하고 [STATUS](../spec/STATUS.md#c11-css-grid) C11.4는 unchecked다. |
| 4 | 입력 slice를 Stage A 전체, 일반 CSS entry, parser 또는 text paint 완료로 확대하는가 | PASS | Child plan L5/L11/L121–123은 fixture-test direct element-text input으로 한정하고 전체 Stage A, general CSS admission, HTML parser/script, paint, Stage B/C/D와 WPT를 제외한다. 구현 및 플랫폼 근거가 아직 없다고 명시한다. |
| 5 | parent 계획이 child의 범위 또는 증거 상태를 다르게 설명하는가 | PASS | Parent entry L438은 동일 여섯 Chrome scene과 독립 기대값을 연결하고 HostDocument collector/runtime/simulator 미실행, C11.4 미완료, `72/213`을 유지한다. |
| 6 | 새 collector가 기존 scene-specific adapter를 덮어쓰는가 | PASS | Child plan L11/L108은 모든 기존 special dispatch 뒤, legacy `pending = vec![root]` scan 직전에 삽입하도록 고정한다. 현재 `prepare_c11_4_text` source에서도 special dispatch가 그 scan 전에 실행된다. |
| 7 | `Ok(None)`을 fallback의 성공이나 무조건 retry로 오해하는가 | PASS | Child plan L102는 `Ok(None)`을 기존 one-text `TEXT_CASES` fallback 선택으로만 정의한다. Legacy 성공에는 하나의 의미 있는 subtree Text node, 유일한 `div` item/root child, 기존 root/item ID와 text 일치 및 typed style 검사가 모두 필요하다. 현재 adapter source와 일치한다. |
| 8 | generic positive가 두 개가 아닌 수의 text item을 조용히 처리하는가 | PASS | Child plan L102/L109는 positive candidate를 정확히 두 direct text-item으로 제한하고 두 개 초과를 `unsupported_c114_text_source`로 거부한다. 이 부분은 pass04의 count ambiguity를 해소한다. |
| 9 | source 소유권이 불명확한 candidate가 legacy 경로로 새거나 부분 입력을 만드는가 | PASS | Grid direct anonymous Text, 복수/descendant Text, mixed replaced image/text 및 namespace 불일치를 명시 오류로 닫고 typed candidate 오류는 fallback하지 않는다. `C11_4TextInput`은 전체 검증 후 반환하도록 계획한다. |
| 10 | mapping key가 Text node 또는 scene ID에 묶여 item style을 잘못 소유하는가 | PASS | Child plan L35–37은 `ComputedElementStyle.node_id == item.id()`를 찾아 `BTreeMap<NodeId, ParagraphStyle>`를 item ID로 구성하며 실제 Text NodeId/string은 layout source에서 별도 대조한다. ID/root ID/literal은 dispatch에서 배제한다. |
| 11 | builder가 실제 Stylo computed style 대신 수작업 snapshot을 테스트하는가 | PASS | Child plan L71–84와 evidence README L61–63은 실제 `compute_runtime_block_flex_grid_track_sizing_cascade_with_stylesheets` 결과를 검사하도록 한다. Hand-authored `ComputedStyleSnapshot`과 JSON에 없는 Chrome root property map 주장을 명시적으로 금한다. |
| 12 | positive stylesheet가 현재 C11.4 profile에 없는 선언을 요구하거나 일반 stylesheet rejection을 주장하는가 | PASS | `runtime_grid.rs`의 track-sizing allowlist에 필요한 longhand가 있고 Rust sheet는 `@font-face`, `font`/`white-space` shorthand, `.fixed grid-column`을 제외한다. Plan은 제한 inline negative만 제시하며 일반 stylesheet declaration rejection을 주장하지 않는다. |
| 13 | `ParagraphStyle` 변환이 표현하지 않는 CSS 의미를 조용히 지원한다고 주장하는가 | PASS | Child plan L36/L80–84는 없는 alignment/spacing/indent/transform 값을 고정 기본값에서만 허용하고 `visibility`, `hyphens`, ligature 변화, ratio/edge, nonzero `order`, 임의 font/script를 제외한다. CSS text는 ASCII A–Z와 space로 한정한다. |
| 14 | Ahem 독립 intrinsic 계산과 저장된 Chrome 결과가 다르게 적혔는가 | PASS | Recomputed fixture/font/JSON/screenshot SHA가 README와 맞는다. JSON의 여섯 `tracks`는 `[20,40]`, `[40,40]`, `[10,40]`, `[30,50]`, `[20,40]`, `[10,40]`; root/item rectangles, text wraps, 10px Ahem metrics와 계획 표가 일치한다. `grid-ws` raw string도 유지된다. |
| 15 | script 후 DOM 순서와 Rust builder 순서 또는 browser 문서 좌표를 혼동하는가 | PASS | Evidence JSON은 `grid-reorder` 최종 순서를 `[item-a,item-z]`로 기록한다. Plan은 JS를 실행하지 않고 그 순서를 직접 구성하며 browser rect에서 각 root 원점을 빼 scene-local CSS px로 비교한다. Root computed-style JSON 부재를 명시한다. |
| 16 | wrapper/API 또는 raw/contribution/final layout 종류를 혼동하는가 | PASS | Source에서 helper가 `Result<StyleLayoutOutput, _>`를 반환하고 wrapper의 `layout`이 `LayoutOutput`임을 확인했다. Plan의 `output.layout.*` 접근은 맞다. `contribution_input()` 결과는 `source: self.source.clone()`을 보존하므로 `TextIntrinsicInlineSizes::uses_same_font_registry_revision()`로 raw/source identity를 비교할 수 있다. Raw sizes, contribution candidates, intrinsic callback probes, final C15 snapshot은 별도 assertion이다. |
| 17 | font registry 비교가 numeric revision만으로 다른 engine을 같은 것으로 취급하는가 | PASS | `TextIntrinsicInlineSizes::uses_same_font_registry_revision()`은 numeric revision과 opaque token 둘 다 비교한다. Plan은 같은 `&TextLayoutEngine`을 intrinsic/definite 경로로 넘기고 서로 다른 result type 간 token을 직접 비교한다고 주장하지 않는다. |
| 18 | 세 unsupported inline control이 통과한 것으로 과장되거나 잘못된 error 경계에 놓였는가 | PASS | `runtime_grid.rs`에서 inline-property preflight가 `compute_cascade`보다 앞서며 allowlist 밖이면 `{ node, property }`를 반환한다. Child plan L112는 `hyphens:none`, `visibility:collapse`, `font-variant-ligatures:none` control이 아직 실행되지 않았고 snapshot/layout/output이 생기면 안 된다고 정확히 남긴다. 실행은 구현 acceptance gate다. |
| 19 | 사전 비교 산출물의 provenance와 cleanup을 재현 가능한 정도로 과장하는가 | PARTIAL | Fixture `1b2bdf…5b7f52`, Ahem `b719ec…b8448`, observation JSON `ae44f2…15f22`, screenshot `1d3d13…26474` SHA를 재계산했고 screenshot은 1280×633 PNG다. JSON은 Ahem loaded/check true, DPR 1, viewport와 Chrome 156을 기록한다. README는 browser session/server cleanup 및 baseline test 결과를 서술하지만 raw process/close transcript와 baseline run source revision은 별도 저장하지 않았다. 이 결과를 custom six-scene runtime 실행으로 확대하지 않는다. |
| 20 | 새 pass05 결과와 현재 fresh-review gate가 child plan에서 발견 가능한가 | PARTIAL | Pass04 report는 20행 `16/4/0`으로 child index에 연결돼 있다. 하지만 현재 child index의 line 129–131은 새 full review가 여전히 필요하다고 하고 pass05 report link가 아직 없다. 이 보고서를 연결하고 역사적 pass04 이후 상태 문구를 갱신해야 문서 gate가 닫힌다. |

## Blocker와 다음 경계

- **기술 설계 FAIL blocker는 없다.** Exact wrapper path, two-candidate bound, legacy fallback, source/style ownership과 CSS error boundary가 현재 API와 정합적이다. 제한된 implementation slice는 시작할 수 있다.
- 구현 acceptance 전 `hyphens`, `visibility`, `font-variant-ligatures` inline control의 exact node/property error와 snapshot/layout 비생성을 실행해 기록해야 한다. 이 pass는 해당 controls를 실행하지 않았다.
- Plan PR 전에 이 pass05 보고서를 child index에 연결하고 line 129–131의 fresh-review-pending/history 설명을 갱신해야 한다. 현재 worktree는 remote PR diff/CI를 검증하지 않는다.
- Evidence README의 cleanup 및 이전 baseline run은 prose로 기록돼 있다. 원문 process/close transcript와 baseline source revision이 없어 재현 provenance PARTIAL로 유지한다.

이 검토는 문서/source/evidence readback 및 로컬 SHA 계산만 했다. 테스트, browser, simulator와 commit은 실행하지 않았다.
