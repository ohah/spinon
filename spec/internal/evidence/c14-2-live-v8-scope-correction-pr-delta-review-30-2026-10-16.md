# C14.2 live V8 분모 정정 PR diff 검토 30개 — 2026-10-16

검토 대상은 Block inventory의 scope marker, 기존·후속 계획, 공식 상태와 internal evidence index의 정정 diff다. 앞선 계획 재검토 20개와 분리했으며 구현 변경 검토를 대신하지 않는다. 각 관점은 파일·계약·실행 분모의 서로 다른 실패 경계를 확인한다.

| # | 독립 PR 실패 관점 | diff 확인과 결과 |
|---:|---|---|
| 1 | inventory PNG row 수를 숨겨 기존 21-case 보고와 맞지 않게 한다 | README에 22 natural PNG row와 21 implemented case를 모두 밝힌다. 통과 |
| 2 | 제외 사유 없이 case만 조용히 지운다 | `aspect-ratio-auto-override`의 `aspect-ratio:auto <ratio>` 제한을 plan·inventory·review에 기록한다. 통과 |
| 3 | 기존 Chrome observation 값을 사후 수정한다 | observation JSON과 fixture HTML은 변경하지 않고 inventory의 `layoutStage`만 갱신했다. 통과 |
| 4 | original C14.2 plan의 supported subset을 후속 plan이 확대한다 | 21개 Block + 8개 Flex, 29개 runtime gate를 그대로 유지한다. 통과 |
| 5 | 해당 22번째 row가 C14.2 지원 행렬에 몰래 포함된다 | plan의 completion, JS document, report 분모에서 이 row를 명시적으로 제외한다. 통과 |
| 6 | case ID/order 검증이 모든 inventory row를 받아 새 분모가 된다 | 구현 mapping은 marker 없는 21개 Block ID와 정확히 비교하도록 계획한다. 통과 |
| 7 | Flex 수나 oracle coverage가 함께 바뀐다 | Flex inventory, observation, 8개 case 설명은 변경하지 않는다. 통과 |
| 8 | original plan의 unsupported declaration과 inventory kind가 상충한다 | `aspect-ratio:auto <ratio>` fail-closed 설명과 새 `layoutStage` 값이 같은 제한을 가리킨다. 통과 |
| 9 | unsupported label이 상위 C14 전체 지원을 뜻한다 | 문서는 row-level implementation boundary만 표시하고 public CSS 지원을 선언하지 않는다. 통과 |
| 10 | subpixel tolerance 정정과 분모 정정이 섞여 oracle 값을 바꾼다 | 19.984375 raw value와 0.05 tolerance 문구를 유지하고 scope만 설명한다. 통과 |
| 11 | C14.2 완료 체크를 시뮬레이터 전 켠다 | C14.2와 parent C14 checkbox는 unchecked로 남는다. 통과 |
| 12 | 29-case app runtime gate를 다른 count로 표현한다 | plan, status, index는 29/29 supported matrix로 일치한다. 통과 |
| 13 | historical C14.2 result를 앱 V8 검증 완료로 과장한다 | 기존 Chrome/Rust/AVD test executable과 새 미실행 live V8 gate를 구분한다. 통과 |
| 14 | 22개 관측 row 전체를 구현 success로 소개한다 | README는 22행 중 한 unsupported row를 분리한다. 통과 |
| 15 | 구현 전에 계획을 바로잡지 않고 계획 검토를 사후 문서화한다 | 정정 계획 diff에 새로운 별도 계획 검토 20개를 포함한다. 통과 |
| 16 | PR 자체 review를 계획 리뷰와 같은 표로 센다 | 정정 PR의 실제 diff 검토 30개를 별도 파일로 보존한다. 통과 |
| 17 | 상대 링크가 새 검토 산출물을 찾지 못한다 | plan/status/internal index에서 correction review 두 파일을 링크한다. 통과 |
| 18 | Android/iOS runtime, build flag, screenshot scope가 바뀐다 | platform work scope와 simulator 요구는 변경하지 않는다. 통과 |
| 19 | default artifact boundary와 public API/version이 바뀐다 | correction PR은 fixture ABI나 contract version을 변경하지 않는다. 통과 |
| 20 | PR이 근본 원인과 구현 전 남은 검증을 감춘다 | body에 inventory-count mismatch, correction, no new browser capture, remaining live-V8 implementation gate를 기록한다. 통과 |
| 21 | inventory의 unsupported marker가 JSON schema 밖이라 파서가 읽지 못한다 | `layoutStage`는 기존 inventory가 SVG·실패 대조 row를 제외할 때 쓰는 필드와 같은 위치에 추가했고 JSON 구조를 그대로 둔다. 통과 |
| 22 | marker 이름이 `aspect-ratio:auto <ratio>`의 실제 제한과 다른 입력을 가리킨다 | `unsupported_aspect_ratio_auto_with_fallback`을 행 ID, 원 계획의 fail-closed 문구와 대조했다. 통과 |
| 23 | Chrome browser observation에서 제외된 PNG row를 제거해 oracle 이력을 잃는다 | 고정 observation과 원본 HTML은 변경하지 않고 구현 분모에서만 제외했다. 통과 |
| 24 | natural PNG row 22개와 image-success case 21개 수치가 README와 서로 다르다 | precomparison README, 후속 계획, STATUS가 각각 22 observed rows·21 supported Block·8 Flex를 구분하도록 갱신한다. 통과 |
| 25 | C14.2 원 계획은 21개라 말하면서 후속 계획만 22개를 실행한다 | 기존 계획의 범위 단락과 후속 계획의 fixture·report 분모를 모두 21+8로 맞춘다. 통과 |
| 26 | 19.984375 oracle와 tolerance 수정 기록이 scope-only correction에서 바뀐다 | 기존 허용치 문구와 Chrome raw observation을 그대로 유지한다. 통과 |
| 27 | 새 review evidence가 internal index에서 끊긴다 | plan, STATUS, internal README에서 정정 계획 20개와 PR diff 30개 파일로 가는 상대 링크를 확인한다. 통과 |
| 28 | 새 문서 링크가 repository-relative 경로 기준을 혼동한다 | plan에서 `../spec/internal/evidence`, internal README에서 `./evidence`, STATUS에서 `internal/evidence`를 기준으로 링크한다. 통과 |
| 29 | correction PR이 fixture ABI·앱 runtime·public contract를 바꾼 것처럼 보인다 | 변경은 inventory tag와 범위 근거 문서에 한정하고 기능 구현 및 버전 계약을 건드리지 않는다. 통과 |
| 30 | PR 설명이 실제로 수행하지 않은 Chrome 재측정 또는 runtime 성공을 주장한다 | PR 설명과 증거 파일에서 새 브라우저 캡처/앱 runtime은 없고 다음 단계로 남았다고 기록한다. 통과 |

**판정:** 분모 오류의 원인과 범위를 실제 source inventory, C14.2 plan, Rust case list로 대조해 정정했다. 30개 PR diff 관점에서 미해결 P1/P2는 없다.
