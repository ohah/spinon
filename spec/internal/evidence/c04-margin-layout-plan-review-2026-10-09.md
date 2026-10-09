# C04.6 계획 적대 검토 · CSS margin → Taffy

**대상:** [C04.6 구현 계획](../../../plan/c04-margin-layout.md) · **범위:** 계획의 입력·출력·실패 경계

| # | 독립 검토 관점 | 판정 및 계획 반영 |
|---:|---|---|
| 1 | 새 `LayoutStyle` 필드가 기존 코드의 default 배치를 바꾸는가 | default 0 margin으로 고정하고 기존 C04.2·3·4 fixture 회귀를 요구한다. |
| 2 | 음수 margin을 일반 길이 검증에서 거부하는가 | margin은 유한성만 검사하고 padding/gap과 검증 규칙을 분리한다. |
| 3 | NaN·무한대가 Taffy 내부로 들어갈 수 있는가 | 레이아웃 엔진에서 margin 유한성을 검증하고 별도 오류 경로를 둔다. |
| 4 | `margin` shorthand의 1~4 값 확장이 잘못될 수 있는가 | Stylo computed string 전체를 Chromium과 비교한다. |
| 5 | `margin-inline`과 물리 좌우가 RTL에서 뒤바뀔 수 있는가 | logical longhand를 명시 지원하고 computed 물리 값 및 frame을 확인한다. |
| 6 | `margin-block`이 수직 writing mode에서 잘못 투영될 수 있는가 | logical shorthand는 horizontal-tb에서 허용해 LTR·RTL Chromium 결과와 비교하고, `writing-mode` 변경만 거부한다. |
| 7 | `%`가 viewport 높이/너비 기준으로 임의 계산될 수 있는가 | percent computed value는 parser에서 실패시키고 사용할 containing-block 정책을 만들지 않는다. |
| 8 | `auto`가 0으로 바뀌어 auto-margin 분배 차이를 감출 수 있는가 | `auto`는 명시적 미지원 computed value로 거부한다. |
| 9 | `calc()`가 일부 단위만 보존한 채 숫자로 파싱될 수 있는가 | CSS px suffix와 유한 숫자만 받고 나머지는 전체 실패 처리한다. |
| 10 | CSS 문법에서 유효하지만 Taffy가 표현하지 못하는 값이 조용히 기본값 처리되는가 | 실패는 node·property·원래 computed string을 포함한다. |
| 11 | root margin이 부모가 없는 viewport root에서 사라질 수 있는가 | root의 nonzero margin은 지원하지 않고 오류로 거부한다. |
| 12 | 부모와 자식의 margin이 CSS margin collapse처럼 병합될 수 있는가 | 비교 입력을 Flex container로 고정하고 Block margin collapse를 범위 밖으로 둔다. |
| 13 | `gap`과 margin이 같은 간격으로 중복되거나 합산 순서가 바뀌는가 | 두 입력을 개별 case로 두고 모든 item frame을 좌표별 비교한다. |
| 14 | `display:none` 자체의 브라우저 DOMRect를 layout box로 잘못 취급하는가 | hidden item 좌표가 아니라 뒤의 visible sibling 위치를 판정한다. |
| 15 | shorthand 허용이 margin 이외 CSS 속성을 허용하게 되는가 | 새 profile에서 C04.2 속성과 명시한 margin만 허용한다. |
| 16 | 인라인 style 우선순위와 author stylesheet 처리 경계가 혼합되는가 | 현재 adapter의 inline `style` 속성 전체 거부를 유지한다. |
| 17 | Stylo parse diagnostic 후 부분 computed style을 사용할 수 있는가 | 진단 하나라도 있으면 cascade/layout 전체를 실패시킨다. |
| 18 | 오래된 document snapshot과 새 revision 계산이 섞일 수 있는가 | generation/document/render revision 및 style/environment tuple을 보존·비교한다. |
| 19 | 새 profile이 기존 Flex profile의 CSS allowlist를 넓힐 수 있는가 | 별도 profile·entrypoint를 만들고 이전 entrypoint margin 거부를 회귀로 둔다. |
| 20 | 내부 Rust fixture 성공을 앱 CSS 지원이나 GPU 화면 성공으로 해석할 수 있는가 | 상태 대장과 계약에서 runtime·text·GPU·Android/iOS 미포함을 구분한다. |

이 계획 검토는 구현 코드 검토와 별개다. 실제 입력 중 `display:none` 요소의 `getBoundingClientRect()`는 부모 기준 좌표가 layout box를 나타내지 않는 경계와 layout root의 margin 무시 위험을 설계에 반영했다.
