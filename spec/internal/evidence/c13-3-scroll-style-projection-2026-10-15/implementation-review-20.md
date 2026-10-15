# C13.3 typed style projection 구현 적대 검토 20개

검토 대상은 이 단계의 Rust 변경과 실행 결과다. 앞선 계획/router 리뷰는 재사용하지 않았다.

| # | 실패 관점 | 확인과 결과 |
| ---: | --- | --- |
| 1 | process-wide Stylo preference가 C12 속성 지원을 새어 바꾼다 | preference는 C13.3 writer guard 동안만 켜고 drop에서 되돌린다. 전체 workspace에서 C12 `offset-path` 진단 회귀가 없다. |
| 2 | C13.3 진행 중 다른 thread가 inline CSS를 파싱한다 | 모든 일반 `StyloDocumentView` 생성 경로가 reader guard를 가진다. 동시 반복 회귀 테스트 통과. |
| 3 | 다른 thread가 직접 stylesheet registry에 CSS를 등록한다 | 공개 `StylesheetRegistry::append`가 reader guard를 취한다. workspace stylesheet 테스트 통과. |
| 4 | cascade 내부 stylesheet append가 동일 thread에서 잠금을 재취득해 멈춘다 | cascade는 guard-held 전용 append 경로를 사용한다. 전체 workspace가 완료되어 교착이 재현되지 않았다. |
| 5 | 이미 기본 설정으로 파싱된 inline declaration을 재사용해 overscroll을 잃는다 | C13.3 진입에서 동일 snapshot/root semantics의 private view를 다시 구성한다. typed longhand/shorthand 테스트 통과. |
| 6 | 재구성에서 `:root` 일치 여부·합성 HTML mount가 달라진다 | 원 view의 `root_matches_root_pseudo`, HTML/quirks/base URL, 합성 root 플래그를 그대로 넘긴다. layout projection 테스트 통과. |
| 7 | panic/error 뒤 preference가 켜진 채 남는다 | RAII guard drop에서 false로 되돌린다. 모든 반환 경로가 guard scope를 벗어난다. |
| 8 | 일반 cascade가 C13.3 전용 preference를 읽는다 | 모든 profile cascade에 reader/writer preference guard를 먼저 취한다. 전체 workspace 회귀 통과. |
| 9 | C13.3 선언이 inline-only로만 처리된다 | author stylesheet shorthand와 longhand, 중첩 `@media` preflight를 실행한다. focused tests 통과. |
| 10 | 유효 `pan-left/right/up/down`를 Stylo가 `auto`로 조용히 바꾼다 | raw declaration preflight가 네 방향 토큰 모두를 cascade 전에 구체적인 오류로 거부한다. |
| 11 | author stylesheet의 방향 pan 값이 중첩 규칙에서 빠진다 | `@media` rule body를 재귀 스캔하고 `pan-right`를 거부한다. 테스트 통과. |
| 12 | `var()`가 제한 값을 감춘 채 `auto`로 계산된다 | dynamic `var()`를 원 값 검증 전 profile 오류로 거부한다. inline negative control 통과. |
| 13 | 잘못된 일반 token을 directional value로 오인한다 | 유효 touch-action token grammar와 축 중복을 검사한 뒤 방향 토큰만 오류화한다. 다른 invalid token은 directional error가 아니다. |
| 14 | `!important`, comment, 대소문자가 토큰 경계를 바꾼다 | cssparser token/priority 처리, comment skip, ASCII case-insensitive keyword match를 사용한다. |
| 15 | logical overscroll property가 물리 축과 혼용된다 | inline allowlist에서 `overscroll-behavior-inline`을 거부하고 logical stylesheet 입력도 오류 처리한다. |
| 16 | shorthand의 한 값이 두 번째 축에 초기값으로 잘못 들어간다 | CSS shorthand cascade에서 두 축이 `contain`이 되는 테스트 통과. |
| 17 | 뒤의 physical longhand가 earlier shorthand보다 낮은 우선순위가 된다 | `overscroll-behavior:contain` 뒤 `overscroll-behavior-x:none` 테스트가 x none/y contain을 확인한다. |
| 18 | 값이 자식에게 상속된다 | root에서만 지정하고 child를 비운 테스트에서 child initial `auto/auto`와 `touch-action:auto`를 확인한다. |
| 19 | Stylo의 다른 touch-action flag를 supported pan으로 축약한다 | exact auto/none/pan-x/pan-y만 매핑하고 다른 계산 bitset은 `Unsupported`로 남긴다. `manipulation`, `pan-x pan-y` 테스트 통과. |
| 20 | 새 profile이 이전 overflow/layout 결과를 잃거나 증분 재사용으로 잘못 보인다 | 기존 overflow와 clip margin 필드를 복사하고 style-to-layout profile 테스트 통과. 증분 reuse는 거부해 full cascade 경로만 허용한다. |

코드의 잔여 경계는 [실행 근거](README.md)에 기록했다. 이 검토는 renderer gesture 통합이나 Simulator 동작을 대신하지 않는다.
