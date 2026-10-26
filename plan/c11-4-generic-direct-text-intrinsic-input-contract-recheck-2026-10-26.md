# C11.4 direct-text contract targeted recheck

**검토 상태:** PASS · 기능 blocker 없음  
**기준:** branch `plan/c11-4-generic-text-intrinsic-input-20261026`, source HEAD `040a5360`; candidate 변경은 미커밋 상태다.  
**범위:** hyphens 계약 보정 후 inline preflight, parser 관찰, collector/fallback 경계와 문서 주장을 source 및 기존 test evidence에 대조했다. 읽기 전용으로 수행했으며 테스트·브라우저·시뮬레이터를 실행하지 않았다.

| 관점 | 판정 | 근거 |
| --- | --- | --- |
| 인식되는 inline property controls | PASS | `visibility:collapse`와 `font-variant-ligatures:none`을 item inline style로 주고 실제 C11.4 cascade API의 결과가 item node와 property가 정확히 일치하는 `UnsupportedInlineAuthorProperty`인지 검사한다. 다른 오류나 snapshot 반환도 실패 처리한다. [test](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_text/generic_direct_text_tests.rs#L748), [preflight](../crates/spinon-style/src/stylo_dom/cascade/runtime_grid.rs#L299) |
| `hyphens:none`의 parser 경계 | PASS | 별도 probe는 cascade snapshot 반환과 해당 item 진단의 `UnknownProperty`를 확인한다. 이를 rejection error 또는 hyphens 계산 지원으로 세지 않는다는 계획·evidence 표현과 일치한다. [test](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_text/generic_direct_text_tests.rs#L775), [계획](./c11-4-generic-direct-text-intrinsic-input.md#L84) |
| raw scanner 및 stylesheet rejection 주장 | PASS | C11.4 inline property 검사기는 Stylo inline declaration의 parsed property ID를 사용한다([구현](../crates/spinon-style/src/stylo_dom/cascade/runtime_layout.rs#L392)). hyphens 전용 raw scanner는 추가되지 않았다. 기존 stylesheet preflight는 `text-orientation`에 한정된다([구현](../crates/spinon-style/src/stylo_dom/cascade/runtime_grid.rs#L274)). 계획은 일반적인 unsupported stylesheet declaration rejection을 주장하지 않고 negative controls도 inline으로 한정한다([계획](./c11-4-generic-direct-text-intrinsic-input.md#L82)). |
| special adapter·candidate·fallback 경계 | PASS | generic collector는 기존 special adapter들 뒤, legacy root text scan 앞에서 호출된다([dispatch](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_text.rs#L71)). non-Grid와 2개 미만 candidate는 `None`을 반환해 기존 경로를 선택하며, 2개 초과는 fail closed, 정확히 2개는 source/style 검증으로 진행한다([collector](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_text/generic_direct_text.rs#L26)). 계획도 fallback 선택 자체를 성공 보장으로 설명하지 않는다. |

기존 구현 관찰 기록에는 수정 전 `7 passed; 1 failed`가 `hyphens`의 잘못된 error 기대 하나였음, 보정 후 focused test `9/9`, C11.4 관련 회귀 `19/19`와 `30/30`, fmt 및 Clippy 통과가 기록돼 있다([evidence](../spec/internal/evidence/c11-4-generic-direct-text-intrinsic-precomparison-2026-10-26/implementation-test-observation-2026-10-26.md)). 이 targeted recheck는 그 명령들을 다시 실행하지 않았다. 이 근거는 local Rust fixture/test 경계이며 simulator/app artifact, PR/CI 또는 전체 Stage A 검증은 아니다. 공식 C11.4 상태 `72/213`은 유지한다.

**문서 동기화:** 완료. Child와 parent plan은 이 report를 연결하고 4/4 PASS 및 현재 후속 gate를 기록한다. 공식 C11.4 상태 `72/213` 유지, simulator/app artifact와 전체 Stage A는 미검증이다.
