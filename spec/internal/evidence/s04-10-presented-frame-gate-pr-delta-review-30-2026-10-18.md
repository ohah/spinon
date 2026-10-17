# S04.10 확인 frame registry PR 변경 관점 검토 · 30개 · 2026-10-18

## 검토 대상

S04.10 순수 Rust candidate code, 내부 계약 0079, 계획·상태 대장 연결, 실행 근거와 별도 계획/구현 검토 문서를 포함한 최종 PR 변경 집합을 검토했다. 구현 실패 관점 20개와 다른 PR 통합·문서·게이트 관점으로 구성했다.

## PR 변경 관점 30개

| # | 독립 검토 관점 | 최종 diff 판정 |
| ---: | --- | --- |
| 1 | 작업 ID와 구현 범위가 사용자에게 약속한 S04.10 후속과 일치하는가 | pure Rust prerequisite slice로 제한되어 있다. Android/iOS adapter가 없다. 통과. |
| 2 | 코드·공식 문서·public roadmap가 제품 기능 완성을 암시하는가 | contract·plan·status·roadmap 모두 candidate/model로 한정하고 미완료 경계를 반복한다. 통과. |
| 3 | 공식 S04.10 완료 checkbox가 실수로 바뀌었는가 | `spec/STATUS.md` checkbox는 `[ ]`다. 통과. |
| 4 | 상위 S04 및 S05.2 상태에 완료 전파가 생겼는가 | S04 상위·S05.2를 그대로 미완료로 둔다. 통과. |
| 5 | 0019 부속 checklist가 official status source처럼 취급될 여지가 있는가 | candidate 문구를 추가했으나 공식 완료 판정은 `STATUS.md`로 돌린다. 통과. |
| 6 | 기존 S04.9 static hit-test 근거를 displayed-frame 검증으로 재사용하는가 | 0019/status에서 S04.9가 displayed-frame evidence가 아님을 유지한다. 통과. |
| 7 | R13의 `Queue::present`나 offscreen readback을 실제 presentation으로 올려 부르는가 | 기존 결과를 capability/submit/readback으로 유지하며 새 code는 외부 token signal을 요구한다. 통과. |
| 8 | 광학 scanout, compositor 이후 출력 또는 present timing을 주장하는가 | 계약·근거·PR 범위에 모두 미검증이라고 적었다. 통과. |
| 9 | Android·iOS platform callback이 연결됐다고 오인할 수 있는가 | 구현/API 명세에 callback 연결이 없고 platform 미검증임을 명시했다. 통과. |
| 10 | native input timestamp가 실제 과거 화면 frame을 골라 준다고 오인할 수 있는가 | registry는 마지막 확인만 보유하며 clock/order 기반 과거 선택을 하지 않는다고 명시한다. 통과. |
| 11 | callback 재정렬 또는 serialization이 검증됐다고 오인할 수 있는가 | 단일 owner는 호출 계약일 뿐 실제 플랫폼 ordering은 별도 gate다. 통과. |
| 12 | S05.1의 revision stale policy가 새 registry만으로 구현됐다고 오인할 수 있는가 | revision tuple을 보존하지만 current document admission/dispatch는 외부 소유라고 명시한다. 통과. |
| 13 | S05.2 React click/handler 교체·해제가 완료된 것처럼 보이는가 | S05.2 checkbox와 status를 바꾸지 않았다. 통과. |
| 14 | 새 API가 외부 앱 공개 API/ABI로 게시되는가 | Rust crate export는 internal contract 0079에 묶고 C ABI·public API가 아니라고 표시했다. 통과. |
| 15 | 내부 계약 숫자 버전이 구현 추가만으로 올라가는가 | 새 계약은 `0.1.0-draft`; release compatibility version은 고정 규칙을 따른다. 통과. |
| 16 | Plan review 표를 implementation review로 재사용하는가 | 계획 검토 20개와 코드 검토 20개가 서로 다른 파일/관점으로 존재한다. 통과. |
| 17 | PR review 30개가 implementation review의 문장 재배열에 불과한가 | PR 경계·status·문서 전달·artifact·screenshot·merge 각 항목을 별도 검토한다. 통과. |
| 18 | 새 테스트가 기존 C13 gesture/scroll input queue 회귀를 가리거나 제거하는가 | renderer 전체 117 test에 기존 queue/session tests가 포함되고 유지됐다. 통과. |
| 19 | Rust workspace regressions를 특정 crate 테스트로 축소 보고하는가 | full workspace 결과 1067 passed/4 ignored와 focused 결과를 분리해 기록했다. 통과. |
| 20 | FFI default artifact 경계 검증을 과장하는가 | FFI default unit tests 20개를 report하며 새 FFI ABI가 생겼다고 하지 않는다. 통과. |
| 21 | Clippy/rustfmt 결과가 실제 최종 소스에서 나왔는가 | 최종 코드 변경 후 workspace Clippy, fmt check 및 diff check를 다시 실행했다. 통과. |
| 22 | source/test 파일이 권장 line count를 넘어 책임 분리를 놓쳤는가 | 구현 337줄, 별도 테스트 407줄로 각 한 책임이며 500줄 아래다. 통과. |
| 23 | `RuntimeConfirmedFrame` equality가 large snapshot 전체를 비교하는 뜻으로 노출되는가 | `PartialEq`를 제거했고 테스트도 비교 결과/error variant만 확인한다. 통과. |
| 24 | superseded pending token 폐기 오류가 “already confirmed”로 잘못 분류되는가 | 최신 confirmation frame과 superseded frame을 별도 오류로 구분하고 상태 불변 test가 있다. 통과. |
| 25 | FrameId가 generation 교체 후 재사용되는가 | generation advance 후 증가 ID를 요구하는 별도 test와 contract가 있다. 통과. |
| 26 | foreign registry ID collision이나 generation swap이 이전 token을 받아들이는가 | identity/generation 검사를 분리하고 negative tests로 고정했다. 통과. |
| 27 | pending limit가 snapshot memory bytes/swapchain capacity로 잘못 설명되는가 | frame count 상한으로만 기술하고 byte/swapchain 보장은 부인한다. 통과. |
| 28 | `discard_unpresented`가 callback 부재를 timeout으로 추정하거나 host 책임을 숨기는가 | caller가 더는 표시될 수 없음을 알아야 하며 자동 timeout 없음을 계약에 적었다. 통과. |
| 29 | 순수 Rust 변경에 동작하지 않는 app screenshot을 붙이는가 | 이 diff에는 UI가 없어 PR behavior screenshot을 포함하지 않는다. platform capture를 대신 제시하지 않는다. 통과. |
| 30 | public roadmap note의 문서 경로와 Tailnet/Pages 전달 경계를 혼동하는가 | static note는 계획·계약·근거 route를 연결한다. Tailnet copy/hash/browser 확인은 merge 뒤 별도 sync evidence로 하고 Pages는 배포하지 않는다. 통과. |

## 결론

30개 PR 변경 관점에서 scope/status 과장, 계약 경계, test report 정확성, 오류 의미 및 무관한 worktree 변화 노출을 찾지 못했다. 플랫폼 app/UI 동작은 이 PR의 판정 대상이 아니며 완료 근거로 주장하지 않는다.
