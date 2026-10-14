# C13.1 음수 clip margin 구현 실패 관점 · 20개 · 2026-10-14

검토 대상은 음수 `overflow-clip-margin`의 cascade diagnostic 및 layout 차단 테스트다. 제품 runtime 코드 변경이나 앱 surface 검증으로 확대하지 않는다.

| # | 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | Chrome의 파싱 수용을 CSS 표준 유효성으로 오인 | 고정 observation은 Chrome `CSS.supports`/computed 결과를 기록하고, Working Draft의 음수 무효 규칙은 별도로 적었다. |
| 2 | 잘못된 stage를 오류 원인으로 단정 | 실패를 재현한 뒤 renderer `UnsupportedComputedValue` 기대를 제거하고 cascade diagnostic 경계를 검사한다. |
| 3 | helper 내부 unwrap가 실제 실패를 숨김 | 새 테스트는 snapshot, view, cascade, layout 호출을 각각 직접 수행한다. |
| 4 | 진단이 다른 node에 붙어도 통과 | diagnostic `node_id`가 fixture child와 일치하는지 확인한다. |
| 5 | 다른 CSS property의 진단으로 통과 | message에 `overflow-clip-margin`이 있어야 한다. |
| 6 | 음수 값이 파서 메시지에서 소실 | message에 `-1px`가 남는지 assert한다. |
| 7 | 값이 아닌 generic parser 오류만 검증 | `OtherInvalidValue` 분류를 확인한다. |
| 8 | cascade가 진단을 버리고도 성공 | cascade 결과의 `diagnostics`에서 child 진단을 직접 찾아야 한다. |
| 9 | 진단이 layout까지 흘러 잘못된 결과 생성 | layout이 `CascadeDiagnostic`으로 실패해야 하며 성공 output을 허용하지 않는다. |
| 10 | 음수 선언이 root에서 발생해 소스 추적 혼동 | 음수 declaration은 의도한 child inline style에만 둔다. |
| 11 | root 부재나 profile 불일치로 우연히 실패 | 정상 root/child fixture와 C13.1 cascade profile로 진입한다. |
| 12 | `0px`도 함께 거부하는 과잉 차단 | 동일한 구조의 `0px` 대조군이 diagnostic 없이 layout 성공해야 한다. |
| 13 | 음수 구문이 잘못되어 property parser 경로를 타지 않음 | 실제 inline declaration 문자열은 `overflow-clip-margin:-1px`이며 Chrome observation case와 맞춘다. |
| 14 | `overflow:clip` 조건이 빠져 다른 profile 상황 검사 | 관찰과 테스트 모두 x/y overflow가 `clip`인지 고정한다. |
| 15 | browser build drift로 observation을 잘못 연결 | Chrome product version, branch revision, UA를 테스트에서 확인한다. |
| 16 | fixture 변경 뒤 옛 oracle 재사용 | 원본 fixture SHA-256을 assert한다. |
| 17 | 선행 전체 observation 변경을 놓침 | baseline observation SHA-256을 assert한다. |
| 18 | `-1px`의 browser acceptance field가 false여도 테스트가 통과 | `negativeClipMarginSupported == true`를 assertion한다. |
| 19 | 테스트가 browser rendering/event 의미까지 증명한다고 과장 | 경계 문서에 검증 범위를 parser diagnostic/layout 차단으로 제한한다. |
| 20 | 선택 단위 테스트만 통과했는데 주변 기존 행 회귀 누락 | style-to-layout 전체 171 test와 renderer의 기존 3-row C13.1 matrix를 실행한다. |

발견 사항은 첫 시도의 예상 오류 stage가 실제 동작과 달랐던 점이다. 테스트를 실제 cascade/layout 경계로 옮기고 유효한 `0px` 대조군을 둔 뒤 관련 테스트가 통과했다. 제품 runtime 코드는 바꾸지 않았다.
