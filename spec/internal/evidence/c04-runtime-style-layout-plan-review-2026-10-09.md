# C04 Runtime CSS→Taffy 계획 적대 검토

검토 대상: [`plan/c04-runtime-style-layout.md`](../../../plan/c04-runtime-style-layout.md)
검토 대상 SHA-256: `231207e4138d6bfd9f43ad7f215f945fddcacdaba9792c5a9024f2c038055778`
검토 성격: 구현 전 계획 검토. 코드를 검토하거나 실행한 결과가 아니다.

## 서로 다른 실패 관점 20개

| # | 공격 관점 | 계획에서 확인한 실패 가능성 | 판정 및 반영 |
|---:|---|---|---|
| 1 | C04.8 UA JSON 소비자 호환성 | 새 profile의 속성 집합이 기존 7개 UA 필드의 직렬화 결과를 바꿀 수 있다. | C04.8 JSON은 superset 계산 결과 중 원래 7개 속성만 기존 schema·키·revision으로 직렬화한다. 기존 고정 JSON shape 회귀 fixture를 둔다. |
| 2 | cascade 중복 비용 | UA 계산과 Taffy 계산 준비를 위해 같은 DOM을 두 번 cascade할 수 있다. | `RuntimeFlexLayout` 단일 Stylo 결과에서 기존 UA·신규 layout projection을 분리한다. 재계산은 금지한다. |
| 3 | allowlist 누락 선언 | 미지원 선언을 무시하면 Chromium과 다른 프레임을 성공으로 게시할 수 있다. | CSS parser가 정규화한 선언 이름을 명시 allowlist와 대조하고, 밖의 선언은 layout만 실패시킨다. |
| 4 | CSS 문법 오류의 recovery | 잘못된 선언 하나 때문에 브라우저가 적용할 나머지 유효 선언까지 버릴 수 있다. | Stylo syntax diagnostics를 보존하고 파서 recovery 결과로 계산한다. layout 실패 사유와 parser diagnostic을 구분한다. |
| 5 | CSS fragment root 오해 | 앱 root를 document element로 취급하면 `:root`와 root display blockification 결과가 변한다. | C04.8의 fragment-root view를 그대로 유지하고 HostRoot를 CSS 노드로 만들지 않는다. |
| 6 | HostRoot 다중 자식 위치 충돌 | 각 자식 root를 viewport 원점에 배치하면 여러 화면이 겹친다. | 첫 slice는 root 하나만 허용한다. 복수 root는 위치를 추측하지 않고 layout 상태 전체를 실패시킨다. |
| 7 | 빈 문서와 `empty` 구별 | 비어 있는 HostRoot를 오류나 stale 이전 frame으로 노출할 수 있다. | HostRoot가 비어 있으면 완료된 `empty` 상태와 해당 revision을 게시하고 이전 frames를 지운다. |
| 8 | text와 inline formatting | text node를 생략하면 CSS line box·자식 순서와 다른 geometry를 만들 수 있다. | text가 있는 subtree와 inline/replaced/list formatting은 임의 축약하지 않고 전체 layout을 실패시킨다. |
| 9 | DOM 순서와 frame 순서 | map 정렬 순서를 paint 또는 child 순서로 오해할 수 있다. | layout frames는 HostDocument preorder로 직렬화하고 node ID만으로 순서를 추론하지 않는다. |
| 10 | 오래된 문서 결과 publish | 계산 도중 JS가 DOM을 다시 바꿔 과거 frame이 최신으로 읽힐 수 있다. | 다섯 revision tuple을 요청·완료 양쪽에 싣고 최신 key가 아닌 결과는 버린다. |
| 11 | 환경 크기와 pixel scale 혼합 | device scale을 Taffy viewport에 곱하면 CSS px 좌표가 두 배 계산될 수 있다. | Taffy는 CSS px viewport를 쓰고 device scale은 CSS environment snapshot의 입력으로만 유지한다. |
| 12 | worker queue의 무제한 증가 | 연속 DOM update가 모든 layout 요청을 대기시키면 결과가 입력보다 계속 늦어진다. | C04.8의 capacity-one latest-wins 슬롯을 공유하며 중간 요청을 무제한 저장하지 않는다. |
| 13 | cascade 성공과 layout 실패 혼합 | inline formatting 같은 미지원 값이 UA style 조회까지 고장 내거나, 반대로 layout 실패를 ready로 보일 수 있다. | cascade/UA snapshot과 layout 상태를 독립적으로 보존한다. layout 실패가 기존 UA 결과를 지우지 않는다. |
| 14 | CSS 계산 시간 회귀 은폐 | Taffy 비용을 기존 `computationDurationUs`에 합치면 C04.8 측정 의미가 바뀐다. | C04.8 필드는 cascade 시간으로 유지하고 Taffy projection 구간은 layout JSON에서 별도로 측정한다. |
| 15 | unsupported 단위의 조용한 default | `%`, `em`, `rem`, `auto` margin 등을 0이나 auto로 바꾸면 오차가 은폐된다. | 지원 serialization을 고정하고 그 밖의 computed value를 속성·노드 오류로 반환한다. root nonzero margin도 실패 처리한다. |
| 16 | 음수·NaN·무한대 frame | 비정상 좌표가 JSON의 `null`이나 잘못된 크기로 유출될 수 있다. | 게시 전 전체 frame을 유한 좌표·nonnegative 크기로 검증하며 하나라도 실패하면 frames를 원자적으로 비운다. |
| 17 | 장시간 UI thread 조회 | 계산 완료를 기다리거나 Stylo/Taffy를 FFI 호출 스레드에서 실행할 수 있다. | worker만 계산한다. 복사 API는 현재 상태만 JSON으로 복사하며 기다리지 않는다. |
| 18 | 작은 FFI buffer와 session 해제 경합 | truncated JSON이 유효 결과로 오인되거나 use-after-free가 생길 수 있다. | required capacity/NUL guard를 유지하고 모든 API에 C04.8과 같은 live-session·free 비병행 계약을 둔다. |
| 19 | 무제한 원문 오류 출력 | 앱이 큰 CSS 값을 넣어 오류 JSON·로그를 과도하게 키울 수 있다. | 오류 JSON은 안정 코드·node ID·속성명만 내고 입력 CSS 원문을 포함하지 않는다. |
| 20 | 좁은 slice를 제품 CSS·GPU 완료로 오해 | Taffy snapshot만으로 전체 CSS나 화면 표시가 제공되는 것처럼 문서화될 수 있다. | C04 parent는 미완료로 두며 GPU·text·Grid·CSSOM·author stylesheet·실기기를 명시 비범위로 남긴다. |

## 계획 보정 기록

초안 검토에서 다음 결함을 찾아 계획에 반영했다.

- 기존 UA cascade JSON과 확장 computed-style profile의 호환 경계를 분명히 했다.
- 스타일 cascade를 중복 실행하지 않고 한 결과에서 UA·layout 출력 두 개를 만든다.
- 미지원 declaration·syntax recovery·layout failure의 관계를 분리했다.
- fragment root 의미, 빈/복수 root 정책, DOM preorder, frame 유효성 및 error payload 한도를 명시했다.
- cascade 시간과 layout 시간의 측정 의미를 분리했다.
- 구현 코드보다 먼저 Chromium 실행 파일 hash가 고정된 CSS·geometry oracle을 생성하도록 순서를 바꿨다.
- 실제 설치된 Chrome `.98`의 revision·binary hash를 계획과 oracle에 함께 고정했다. computed 값 135개와 geometry oracle은 Rust 변경 전에 생성했다.

이 검토는 계획 문서의 실패 경로만 확인했다. 실제 Stylo parser 속성 분류 API, C ABI 구현, Chromium frame oracle, Android·iOS V8 실행은 구현 PR에서 새 관점으로 검토하고 확인해야 한다. 미출시 내부 숫자 계약 버전은 `0.1.0`으로 유지한다.
