# C13.1 clip unsupported effects · 구현 실패 관점 20개

대상은 `c131_clip_radius_and_transform_fail_closed`다. clip ancestor, positioned descendant, inline style, author stylesheet, error attribution 및 지원 경계를 각각 확인했다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | `border-radius` shorthand가 지원되는 것처럼 통과함 | PASS — clip root의 shorthand가 radius 관련 unsupported property로 실패한다. |
| 2 | radius가 다른 node에 귀속됨 | PASS — error node는 clip root다. |
| 3 | radius를 무시하고 사각형 clip geometry를 성공 게시함 | PASS — layout `Err`가 필수라 geometry/render snapshot을 게시하지 못한다. |
| 4 | clip ancestor의 transform이 조용히 무시됨 | PASS — root transform은 `unsupported_inline_property`로 실패한다. |
| 5 | child transform만 지원되어 ancestor 교차를 놓침 | PASS — clip root와 transformed descendant case를 별도로 실행한다. |
| 6 | absolute descendant가 flow node로 계산돼 test가 빗나감 | PASS — child 입력은 `position:absolute`와 inset을 명시한다. |
| 7 | child transform이 root로 잘못 귀속됨 | PASS — effect가 있는 정확한 child NodeId를 요구한다. |
| 8 | transform 오류가 다른 property 오류로 대체됨 | PASS — failure code와 property marker를 둘 다 확인한다. |
| 9 | root 스타일 미지원 때문에 effect 오류보다 먼저 실패 | PASS — root sizing/position/overflow는 C13 profile의 기존 지원 입력이고 각 case의 첫 unsupported effect만 검사한다. |
| 10 | cascade 실패를 layout effect rejection으로 혼동 | PASS — inline case는 cascade root 한 개를 확인한 뒤 layout error를 검사한다. |
| 11 | shorthand expansion의 corner 순서를 고정해 brittle해짐 | PASS — 이름에 `radius`가 포함되는지만 검사한다. |
| 12 | stylesheet에서 rounded clip이 허용됨 | PASS — author stylesheet radius control은 cascade error를 반환한다. |
| 13 | stylesheet에서 transform이 허용됨 | PASS — author stylesheet transform control도 명시적으로 거부된다. |
| 14 | stylesheet error property를 확인하지 않음 | PASS — radius와 transform의 각 marker를 별도로 확인한다. |
| 15 | class selector가 대상 요소에 적용되지 않아 control이 무의미함 | PASS — helper가 `.painted` child를 만들고 같은 selector를 사용한다. |
| 16 | inline/style sheet 결과를 같은 단계의 오류로 축약 | PASS — inline layout failure와 stylesheet author-CSS error를 구분해 확인한다. |
| 17 | profile이 C13 아닌 generic request로 우회됨 | PASS — 모든 입력은 `compute_request_for_block_flex_overflow_clip` 경로다. |
| 18 | previous layout/cache가 성공 snapshot을 재사용 | PASS — 각 case는 새 document/snapshot, full recompute, previous state 없음이다. |
| 19 | 새 검증이 C22/C23 구현 책임을 C13에 끌어옴 | PASS — radius/transform은 미지원으로 유지하고 owner 단계 뒤 parity 비교한다. |
| 20 | unit 결과를 simulator/실기기 pixel 증거로 과장 | PASS — 이번 변경은 runtime unit evidence만 기록한다. |

실행 결과 원본은 이 폴더의 `runtime-targeted-test.log`, `workspace-cargo-test.log`, `workspace-clippy.log`다. 이 review는 rounded clipping, transformed painting 또는 hit-test parity를 검증했다고 주장하지 않는다.
