# C07.2 구현 변경 실패 관점 검토

구현 diff, 고정 Chromium fixture, 스타일·레이아웃 단위 검사, Android·iOS runtime 연결을 서로 다른 실패 경계로 다시 검토했다. 계획 검토는 별도 문서 [계획 검토](./c07-2-border-width-layout-plan-review-2026-10-10.md)에 둔다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 출시 전 작업이 crate·내부 계약 숫자 버전을 올리는가 | 아니다. crate와 계약은 `0.1.0`이며 `0044`는 문서 ID다. |
| 2 | 고정 Chromium 실행 파일·revision이 바뀌었는데 기존 기준을 계속 쓰는가 | reference 검사에서 provenance·digest를 확인한다. 입력이 달라지면 비교 검사가 실패한다. |
| 3 | inventory 누락·중복·순서 변경이 50개 기준에 숨어드는가 | 각 ID·fixture digest를 검사하고 두 DPR 모두 50개를 요구한다. |
| 4 | DPR별 Stylo app-unit snapping 차이가 CSS px frame에 새는가 | pinned Chrome CSS px 정규화를 적용하고 DPR 1·2 전체 결과를 동일성 비교한다. |
| 5 | 양수 fractional width를 일반 반올림하거나 매번 다르게 내리는가 | Chrome 154의 `.25…3.5px` 사례를 양 DPR reference 및 단위 test와 대조한다. |
| 6 | 정확한 0과 작은 양수 폭을 똑같이 0으로 처리하는가 | `0`은 0, 양수는 최소 1 CSS px인 경계를 단위 test와 fixture에서 확인한다. |
| 7 | `none`·`hidden`이 폭을 레이아웃에 더하거나 다른 style을 0으로 만드는가 | 네 면의 style gate와 7가지 다른 표준 style을 pinned Chrome 값과 비교한다. |
| 8 | `border`, `border-width` shorthand 확장이나 shorthand reset이 빠지는가 | 1–4개 값, 전체 border reset 및 초기 border-image reset을 fixture와 실제 V8 fixture로 확인한다. |
| 9 | side longhand가 shorthand 뒤 cascade 순서를 잘못 이기는가 | `side-overrides`, `longhand-cascade`, `border-shorthand-reset` 각 node의 computed side와 frame을 확인한다. |
| 10 | 음수·percentage 무효 선언이 앞선 유효 선언을 지우는가 | CSS가 `4px` fallback을 유지하는 두 fixture 결과를 고정 reference와 대조한다. |
| 11 | `calc()`·`var()`를 문자열로 재파싱하거나 잘못된 면에 적용하는가 | Stylo typed computed scalar와 side별 CSS px를 비교한다. layout 경계에서 AST·문자열 재파싱을 하지 않는다. |
| 12 | content-box에서 padding 또는 border를 누락·중복 가산하는가 | asymmetric side·padding 예제의 외곽 frame을 Taffy 직접 검사와 Chrome oracle에서 확인한다. |
| 13 | border-box에 padding·border가 지정 크기 밖으로 더해지거나 최소 외곽 크기가 깨지는가 | 지정 크기보다 padding+border가 큰 사례, min/max 사례와 frame을 대조한다. |
| 14 | Flex shrink가 자식 border 외곽 기여량을 형제 배분에서 빼는가 | 120px 부모와 두 자식의 content/frame 폭 및 sibling x 좌표를 oracle에서 확인한다. |
| 15 | Stylo의 손상된 negative/NaN/infinity typed 폭을 floor·style gate가 0 또는 1px로 숨기는가 | 초안에서 이 결함을 발견해 잘못된 값을 그대로 DTO 검증 경계까지 보존했다. 네 면의 음수·비유한 DTO 거부와 전달 helper test가 통과한다. |
| 16 | border shorthand가 만드는 initial `border-image-*` reset 때문에 유효 선언 전체가 거부되는가 | runtime profile에서 초기값 다섯 가지만 받아 실제 Android·iOS shorthand fixture가 layout ready임을 확인했다. |
| 17 | 비초기 `border-image`가 지원으로 몰래 허용되는가 | author stylesheet와 inline style에서 `round` 등 비초기값을 거부하는 검사가 통과했다. |
| 18 | 제한 runtime 밖의 기존 layout profile이 예기치 않게 border를 소비하는가 | projection은 명시한 runtime profile만 `LayoutBorder`를 전달하고 다른 profile은 0 기본값을 사용한다. 공개 CSS 전체 지원으로 승격하지 않았다. |
| 19 | border-color만 바꿨는데 used width나 frame이 달라지는가, 혹은 세부 최적화를 이미 지원한다고 오해시키는가 | 색만 바꾼 별도 style 계산에서 DTO·frame이 동일하다. 전역 `StyleRevision` 때문에 재계산될 수 있다는 한계를 계획·계약에 적었다. |
| 20 | 모바일 결과를 실제 border paint·hardware GPU·자식별 수치 oracle 증거로 과장하거나 플랫폼 경로를 건너뛰는가 | 두 앱이 동일 JS fixture를 실제 V8로 실행해 6 boxes를 제출했다. 화면·로그는 runtime 연결만 증명한다. Android는 software renderer로 표시했고, iOS의 긴 OSLog는 짧은 C07.2 summary와 별도 draw 로그로 보완했다. |

## 판정

검토 중 발견한 non-finite/negative 값 마스킹 결함은 수정하고 해당 경계 검사를 추가했다. 색상 변경 시 전역 revision에 의한 재계산 가능성은 구현 오류로 숨기지 않고 계약에 한정 사항으로 기록했다. 이 검토 범위에서 미해결 코드 결함은 남지 않았다. border paint, 실기기·hardware GPU, mobile child frame의 숫자별 Chromium 비교, text intrinsic sizing, Grid, 외부 CSS URL은 별도 작업이다.
