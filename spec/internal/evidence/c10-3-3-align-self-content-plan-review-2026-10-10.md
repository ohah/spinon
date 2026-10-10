# C10.3.3 계획 적대적 검토

**대상:** [C10.3 Flex 순서와 정렬 구현 계획](../../../plan/c10-3-flex-order-alignment.md)의 C10.3.3 절

**기준:** CSS Box Alignment Level 3, 2026-10-08 Working Draft · CSS Flexbox Level 1, 2025-10-14 Candidate Recommendation Draft · Taffy 0.14.0 잠금 소스
**판정:** 아래 지적은 계획 보완에 반영했다. 비교 fixture와 기능 구현은 아직 시작하지 않았다.

초기 계획은 `normal`, flow-relative/self-relative 위치, overflow-position 및 Box Alignment shorthand를 범위 밖으로 두고 `align-self`·`align-content` 일부 값만 열거했다. 웹 호환 목표의 C10.3.3 계약으로는 근거 없는 축소였고, 특히 `place-content`가 만드는 `justify-content` 처리가 빠져 있었다. 아래 검토 뒤 계획을 여섯 runtime Flex profile의 비-baseline Box Alignment longhand·shorthand로 고쳤다. overflow-position은 W3C 초안에서 at-risk인 사실을 계획에 표시하고 고정 Chrome 비교를 요구한다.

| # | 공격 질문 | 계획의 방어 조건 | 판정 |
| --- | --- | --- | --- |
| 1 | `align-items`가 기존 다섯 값만 받으면 `auto` 또는 `start` 같은 유효 값을 기본값으로 오인하지 않는가? | 지원 문법을 명시하고 `auto`는 invalid, positional set은 모두 지원하며 baseline은 C10.3.4로 분리한다. | 반영됨 |
| 2 | `align-self`의 `auto`, `normal`, overflow modifier 문법을 `align-items` 문법과 잘못 공유하지 않는가? | 두 property 문법을 각각 열거하고 `align-self:auto`만 허용한다. | 반영됨 |
| 3 | `align-content` positional과 distribution 문법을 섞거나 `left/right`를 허용하지 않는가? | `<content-position>`·`<content-distribution>`을 구분하고 `left/right`를 invalid로 둔다. | 반영됨 |
| 4 | CSS parser에서 invalid declaration을 무시하는 것과 typed adapter의 범위 밖 값을 혼동하지 않는가? | invalid authored declaration은 Chrome cascade와 비교하고, adapter에 도달한 out-of-stage 값은 property/node 문맥과 함께 실패시킨다. | 반영됨 |
| 5 | computed keyword `normal`을 곧바로 `stretch` 문자열로 치환해 CSS 값과 사용 결과를 혼동하지 않는가? | computed keyword를 보존하고 Flex used behavior만 별도 대조한다. `align-items`·`align-self`·`align-content`·`justify-content` 각각을 관찰한다. | 반영됨 |
| 6 | `align-self:auto`를 CSS 상속으로 잘못 구현하거나 parent `align-items` 변경 후 오래된 frame을 재사용하지 않는가? | auto는 같은 Flex parent의 computed `align-items`를 사용하며 부모 변경 뒤 자식 frame과 revision을 재계산하도록 fixture를 요구한다. | 반영됨 |
| 7 | `align-self` 대상을 content box로 정렬하고 line 또는 margin box 기준을 누락하지 않는가? | 정렬 subject를 item margin box, container를 해당 flex line으로 고정하고 row·column 및 여러 line의 child를 비교한다. | 반영됨 |
| 8 | `start/end`와 `flex-start/flex-end`를 동의어로 간주해 reverse 축에서 좌표가 뒤집히지 않는가? | `row-reverse`, `column-reverse`, `wrap-reverse`와 positional 값을 교차해 flow-relative와 flex-relative start를 분리한다. | 반영됨 |
| 9 | `self-start/end`를 container의 writing mode로 계산하거나 RTL/수직 writing mode 미지원인데 지원으로 표시하지 않는가? | 현재 단계의 container와 item은 `horizontal-tb/ltr`로 고정하고, item 방향 차이·RTL·다른 writing mode는 C17에 남긴다. | 반영됨 |
| 10 | 양수 cross-axis 여유 공간에서 auto margin을 계산한 뒤 align-self도 동일 공간을 다시 소비하지 않는가? | 한쪽·양쪽 auto margin에서 양수 free space를 먼저 배분하고 align-self 효과가 억제되는지 비교한다. | 반영됨 |
| 11 | auto margin이 있는 item의 공간이 0 또는 음수일 때도 양수 케이스 규칙을 잘못 재사용하지 않는가? | free space 0과 overflow 케이스를 따로 두고 margin 처리 및 overflow 좌표를 Chrome과 비교한다. | 반영됨 |
| 12 | `align-self:stretch`를 fixed cross size 또는 auto margin이 있는 item에 무조건 적용하지 않는가? | computed cross size가 auto이고 해당 축 auto margin이 없다는 eligibility를 직접 분리한다. | 반영됨 |
| 13 | stretch 후 padding/border와 min/max clamp가 item frame 또는 line size에 잘못 더해지지 않는가? | content-box/border-box, padding, border, min/max와 line size 재계산을 교차 비교한다. | 반영됨 |
| 14 | `align-content`를 실제 line 수만 보고 활성화해 `wrap` 컨테이너와 `nowrap`을 합치지 않는가? | multi-line 여부는 `flex-wrap` 값으로 판단하고 `nowrap`과 `wrap`을 각각 검사한다. | 반영됨 |
| 15 | `wrap`인데 실제 line이 한 개일 때 `align-content`를 끄거나, `nowrap`에서도 켜는 반대 오류가 생기지 않는가? | 실제 한 줄 `wrap`에서 적용되는 값과 `nowrap` 음성 대조를 별도 비교 축으로 둔다. | 반영됨 |
| 16 | line이 하나이거나 free space가 0/음수일 때 `space-between` 등을 분배 가능한 값처럼 처리하지 않는가? | 단일 subject fallback과 양수·0·음수 cross free space를 별도 행렬로 고정한다. | 반영됨 |
| 17 | `row-gap`/`column-gap`을 line distribution 전에 중복 차감하거나, wrap-reverse에서 gap 순서를 잘못 바꾸지 않는가? | gap을 포함한 Chrome line geometry와 wrap-reverse 시작·끝 조합을 비교하고 분배 간격 중복 여부를 명시한다. | 반영됨 |
| 18 | `safe`/`unsafe`를 distribution 값에 붙이거나, draft 상태를 안정 표준 및 scroll 동작까지 검증한 것으로 확대하지 않는가? | modifier는 positional 값에 한정하고, at-risk 표기·non-scroll overflow 비교를 기록하며 C13 scroll semantics는 제외한다. | 반영됨 |
| 19 | `place-items`·`place-self`의 두 번째 `justify-*` 값이 Flex item을 움직이거나, `place-content` 한 값의 복제 성분을 무시하지 않는가? | 두 shorthand longhand 확장과 Flex에서 영향 없는 paired `justify-*`를 확인하고, `place-content`는 `justify-content`를 포함해 한 값/두 값 모두 처리한다. | 반영됨 |
| 20 | shorthand·custom property 변경이 incremental cascade에서 누락되거나, Taffy enum을 CSS 계약으로 오인하고 기존 C04 profile까지 바꾸지 않는가? | 여섯 runtime profile의 full/incremental·stylesheet·custom property 경로, C04 격리, CSS keyword/safety 보존, Taffy 0.14 변환, pinned Chrome/WPT·DPR별 0.5px 판정을 완료 gate로 둔다. | 반영됨 |

## 남은 검증 경계

- 이 문서는 계획의 모순과 누락을 검토한 기록이다. Chromium fixture, Rust 구현, Android/iOS 앱 실행의 성공을 뜻하지 않는다.
- WPT의 개별 경로와 fixture case 수는 사전 비교 단계에서 pinned WPT revision에 대조해 확정한다. 디렉터리 전체를 통과한 것으로 간주하지 않는다.
- CSS Box Alignment 초안은 변경될 수 있고 overflow-position은 at-risk다. 이 단계의 기준은 기록한 버전의 문법과 Chrome `154.0.8037.98`의 관찰 결과다.
