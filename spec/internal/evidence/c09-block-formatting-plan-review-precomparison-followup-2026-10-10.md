# C09 계획 재검토 · 구현 전 fixture 통합

## 검토 범위

`plan/c09-block-formatting.md`의 Chromium fixture 설명을 실제 고정 reference와 대조했다. 이전 계획 검토표를 재사용하지 않고, fixture harness에서 HostRoot runtime 입력으로 변환되는 경계와 현재 관찰 coverage만 다시 확인했다. C09 제품 구현은 C08.1 PR #105가 병합될 때까지 시작하지 않는다.

## 독립 실패 관점

| # | 계획에서 공격한 실패 경로 | 결과와 반영 |
| ---: | --- | --- |
| 1 | 한 HTML 문서의 여러 case wrapper를 하나의 HostRoot로 넘겨 fragment 경계를 깨뜨림 | runtime 비교는 inventory case 하나씩 materialize하고 단일 app root만 HostRoot로 연결하도록 계획에 명시했다. |
| 2 | fixture wrapper를 제품 DOM에 복사해 API node 수·부모 관계를 오염함 | wrapper와 다른 overlay case는 runtime product tree에서 제외한다. |
| 3 | viewport 보조 입력을 DOM node나 paint box로 세어 HostRoot 계약을 바꿈 | viewport는 별도의 containing-block 입력으로 전달한다고 계약에 고정했다. |
| 4 | 절대 배치된 wrapper가 일반 app root의 containing block을 바꿈 | wrapper는 fixture 전용 `flow-root`; 앱 subtree에는 단일 normal Block root만 남기도록 변환 경계를 명시했다. |
| 5 | 23개 overlay wrapper가 서로 layout 간섭을 일으킴 | wrapper는 모두 같은 원점의 고정 320×240 독립 formatting context다. 겹침은 sibling flow를 만들지 않으며 각 case DOM subtree를 따로 비교한다. |
| 6 | `body` 또는 `documentElement`를 HostRoot 요소로 착각함 | CSS root semantics와 body 특례는 profile에서 제외하고 HostRoot 직속 fragment root 의미를 유지했다. |
| 7 | UA 기본 `html/body` margin이 viewport 좌표에 섞임 | fixture는 양쪽 기본 margin·padding을 0으로 재설정하고 harness geometry를 검증한다. |
| 8 | 긴 문서에서 `display:none`의 원점 rectangle을 wrapper-relative 좌표로 바꾸며 음수 위치를 만듦 | wrapper 원점 고정 후 숨김 부모·자식의 0 rectangle을 직접 검증한다. |
| 9 | `getComputedStyle().width` content width를 border-box frame과 비교함 | reference는 computed CSS 값을 `properties`에, used rectangle을 `rect`에 분리 저장하고 225px/240px 조합을 고정했다. |
| 10 | device pixel 좌표를 CSS px 결과로 혼동함 | CDP viewport는 320×240 CSS px이며 DPR 1·2 양쪽을 저장해 geometry 불변을 검사한다. |
| 11 | DPR만 바꿨는데 CSS 결과가 달라지는 오라클을 허용함 | runner와 test가 computed·Typed OM·rectangle 전체 동일성을 요구한다. |
| 12 | oracle가 개발자 Chrome과 다르거나 revision이 바뀜 | CLI `154.0.8037.98`과 DevTools revision을 함께 고정하고 불일치면 생성 실패한다. |
| 13 | HTML/inventory 수정 뒤 stale JSON을 비교함 | JSON에 두 원본 digest를 저장하고 테스트에서 현재 파일 bytes와 검증한다. |
| 14 | fixture만 바뀌어도 같은 reference ID가 재사용됨 | reference ID에 inventory, HTML, capture script digest를 모두 포함했다. |
| 15 | 자동 capture가 승인된 기준 JSON을 덮어씀 | 출력은 exclusive-create를 사용하고 기존 출력 negative check에서 실패·원본 SHA 유지가 확인됐다. |
| 16 | definite containing block 폭 또는 `box-sizing` 전달 누락을 C09.1에서 놓침 | percentage child의 definite parent와 border-box, auto-width min-width clamp case를 추가했다. |
| 17 | horizontal auto margin과 vertical auto margin을 같은 규칙으로 구현함 | 수평 가운데 정렬과 vertical auto margin 0 사용값을 별도 case로 둔다. |
| 18 | flow-root의 외부 bottom margin을 parent last child 및 다음 형제에서 틀리게 계산함 | 재검토에서 처음 계획한 fixture가 first-child/top 경계만 충분히 보인다는 공백을 찾았다. parent-last-child와 following sibling case를 추가했다. |
| 19 | fixture transform 또는 float가 rectangle oracle를 왜곡하거나 C09 외부 기능을 섞음 | capture가 `position:static`, float/clear 없음, transform 없음 및 제한된 display profile을 거부 조건으로 검사한다. |
| 20 | Chromium oracle 준비를 모바일 구현 완료로 확대 해석함 | STATUS·계약·계획은 runtime 미구현, Android/iOS 미검증, C08.1 병합 전 코드 금지를 각각 유지한다. |

## 확인 결과

재검토에서 flow-root last-child/bottom 외부 margin의 fixture 공백을 발견해 별도 case로 보완했다. 현재 고정 기준은 23개 case·76개 app node이며 C09.1 10/30, C09.2 9/30, C09.3 4/16이다. 남은 미검증 경계는 Chromium fixture를 실제 runtime case로 변환하는 Android/iOS 경로와 Taffy/Stylo geometry 비교다. 그 작업은 C08.1 병합 이후 별도 구현·검증으로 진행한다.
