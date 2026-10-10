# C07.2 · 테두리 폭 계획 실패 경로 검토

이 검토는 C07.2 계획과 Chrome 154 고정 기준을 기능 코드를 작성하기 전에 대조한다. 20개 항목은 서로 다른 계약·실패 경계다. 처음 대조에서 확인한 fixture 누락은 반영했으며 제품 구현은 시작하지 않았다.

## 검토 기록

| # | 독립 검토 관점 | 대조 근거와 결과 |
| --- | --- | --- |
| 1 | 기준 환경 또는 실행 파일이 바뀌었는데 수치를 그대로 비교하는가 | Chrome product/revision, 실행 파일 digest, HTML·inventory·capture·helper digest를 reference와 일치시킨다. 기준 불일치 시 비교 무효 조건이 계획과 검사에 있다. 통과. |
| 2 | 일부 관찰 node가 누락되거나 중복되어 분모가 줄어드는가 | inventory 50개 ID와 두 DPR observation 순서·개수가 일치하고 중복을 거부한다. reference 무결성 test가 이를 확인한다. 통과. |
| 3 | DPR 변경이 실제 CSS geometry 기대에 섞이는가 | 고정 capture가 DPR 1·2의 값과 모든 rect를 equality 검사한다. 새 기준도 두 관찰이 일치한다. 통과. |
| 4 | 기본 `border-style:none`에서 medium 폭이 외곽 크기를 키우는가 | `default-none` computed width·frame이 각각 0과 `50 × 30`임을 fixture 및 test가 확인한다. 통과. |
| 5 | `border-width` shorthand의 1·2·3·4 값이 잘못된 면에 연결되는가 | 각 shorthand fixture가 네 computed side를 분리한다. 예상 순서 `top/right/bottom/left`가 test에 있다. 통과. |
| 6 | 면 longhand가 shorthand보다 우선하는 cascade가 일부 면만 덮는가 | `side-overrides`, `longhand-cascade`에서 서로 다른 네 값과 외곽 폭을 확인한다. 통과. |
| 7 | 뒤의 `border` shorthand가 앞선 longhand 값을 모두 재설정하는가 | 최초 fixture는 이 순서를 직접 증명하지 못했다. longhand 뒤에 `border:3px solid`을 둔 `border-shorthand-reset`을 추가했고 네 면 `3px`, frame `106 × 36`을 고정했다. 수정 후 통과. |
| 8 | `none`·`hidden`이 0으로 계산되는 조건을 style 기본값과 혼동하는가 | 최초 노드는 폭 선언과 style만 병렬 지정했다. 실제 shorthand의 solid 값을 뒤 longhand가 none/hidden으로 재설정하도록 고쳤고 computed width 0, frame 불변을 확인한다. 수정 후 통과. |
| 9 | 지원하지 않는 표준 비-none style을 none으로 잘못 분류하는가 | fixture가 solid·dashed·dotted·double·groove·ridge·inset·outset을 모두 포함한다. 기존 기준에 없던 세 style을 추가했다. 각 style의 width contribution을 test가 확인한다. 수정 후 통과. |
| 10 | `thin`·`medium`·`thick`을 기기 임의값으로 고정하는가 | Chrome 154 pinned 결과 `1/3/5px`를 직접 확인하고 test가 세 값을 단언한다. 통과. |
| 11 | fractional width, 최소 1px 규칙, zero가 뭉뚱그려지는가 | `.25`부터 `3.5px`까지 경계 sample과 정확한 zero를 DPR별로 캡처하고 test에서 pinned 결과를 검사한다. 보편 브라우저 규칙이라고 확대하지 않는다. 통과. |
| 12 | content-box에서 세로·가로 중 한 축의 padding/border만 빠지는가 | 비대칭 shorthand와 `100×40` + padding 10 + border 2 fixture가 양축 외곽 `124×64`를 제공한다. 통과. |
| 13 | border-box width/height가 padding·border 때문에 이중 가산되는가 | 동일 크기의 content-box/border-box 비교가 `124×64` 대 `100×40`을 고정한다. 통과. |
| 14 | border-box 콘텐츠가 음수가 되면 outer size가 지정 크기보다 작아지는가 | width/height 10, padding 12, border 2가 `28×28`이 되는 실제 결과를 고정한다. 통과. |
| 15 | content-box min/max가 border 외곽 폭으로 잘못 해석되는가 | 최초 fixture에 border를 포함한 content-box 제약 사례가 없었다. min/max 각각 padding 5·border 2를 추가해 frame width 94를 고정했다. 수정 후 통과. |
| 16 | border-box min/max가 content-box min/max 계산과 섞이는가 | 양쪽 profile을 별도 node로 유지하고 border-box 80, content-box 94 결과를 따로 검사한다. 통과. |
| 17 | calc 결과가 layout으로 가는 동안 불필요한 AST 재구성·property binding을 요구하는가 | Stylo 0.22.0 `ComputedValues`의 typed `BorderSideWidth`는 app-unit scalar이며 Chrome `computedStyleMap()`도 `CSSUnitValue`다. 처음 작성한 AST 전달 계획은 불필요해 제거했다. fixture는 비대칭 calc side 결과 `1/2/3/4px`와 frame `106×34`를 유지한다. 수정 후 통과. |
| 18 | `var()` 결과와 invalid declaration fallback이 문자열 재파싱에 의존하는가 | CSS custom property 3px는 Stylo typed scalar로 들어오고, 이후 invalid negative/percentage는 이전 valid 4px를 보존한다. computed 값·frame을 각기 확인한다. 통과. |
| 19 | transparent border 또는 outline이 box geometry 규칙에 섞이는가 | transparent color를 computed property와 frame으로 검사하고 20px outline이 추가 geometry를 만들지 않는 별도 node를 유지한다. 두 paint 제외 경계가 명세에 있다. 통과. |
| 20 | 잘못된 DTO 또는 stale style 결과가 부분 frame으로 유출되는가 | `LayoutBorder`는 CSS px `f32`만 표현하므로 percentage가 layout DTO에 섞이지 않는다. 음수·non-finite는 node/면 오류로 전체 계산을 거부하고, width/style는 same-style revision을 요구한다. 색상 전용 변경은 geometry 값을 바꾸지 않는다. 실제 mutation/error 주입은 구현 후 별도 확인한다. 계획 구조 통과, 실행 검증은 미착수. |

## 반영한 변경과 남은 확인

- 기준 fixture를 기존 43-node 초안에서 50-node로 확장했다. shorthand reset, 나머지 비-none border style, content-box min/max, 비대칭 `calc()`를 포함한다.
- reference를 덮어쓰지 않는 capture 규칙을 유지했다. 재캡처 전 기존 미확정 파일을 `/private/tmp/spinon-c07-2-baseline-backup/`에 버전별로 보관했다.
- 8개 Node reference contract test가 현재 50-node reference·digest·DPR·핵심 관찰을 검사한다. 구현 검토 항목으로 재사용하지 않는다.
- 구현 검토에서는 Taffy 0.14.0 실제 border projection, Stylo 0.22.0 typed border value·style gate, invalid DTO, calculation owner, style revision mutation, 기존 C07.1 regression, Android/iOS runtime 경로를 실제 소스와 실행으로 별도 확인해야 한다.
- 계획 상태는 검토 완료, 구현은 미착수다. 계획에서 지원하지 않은 CSS 또는 simulator/실기기 동작을 확인한 것으로 간주하지 않는다.
