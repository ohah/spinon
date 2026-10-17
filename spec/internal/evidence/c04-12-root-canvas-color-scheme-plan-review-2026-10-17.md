# C04.12 Canvas color-scheme 계획 실패 관점 검토

계획을 구현 전에 서로 다른 20개 실패 경로로 검토했다. 이 표는 코드 통과 근거가 아니며 계획의 검증 관문과 제외 범위를 확인한다.

| # | 적대적 관점 | 계획 반영 |
|---:|---|---|
| 1 | OS dark인데 meta/CSS 선언이 없는 문서가 OS dark만으로 Canvas를 검게 만들지 않는가? | Chrome 155 관측상 `normal`/선언 없음은 light Canvas다. metadata와 OS preference를 분리하고 default light 기준을 고정했다. |
| 2 | metadata `dark`가 OS light에서 ignored 되지 않는가? | `dark` only profile을 light OS oracle 행으로 포함했다. |
| 3 | metadata `light dark`가 OS 선호를 지원하는가? | 양 OS 값에서 `Canvas`와 `prefers-color-scheme`을 각각 대조한다. |
| 4 | `only dark`가 light OS에서 light로 자동 override 되지 않는가? | supported-scheme 선택과 user preference를 분리하고 `only dark` 행을 둔다. |
| 5 | 명시적 root CSS `dark`가 meta `light`보다 우선하는가? | computed root property 우선 규칙과 충돌 fixture를 둔다. |
| 6 | 명시적 root CSS `light`가 meta `dark`보다 우선하는가? | 역방향 충돌도 같은 판정 기준으로 검사한다. |
| 7 | root CSS `normal`이 metadata supported schemes를 무시하지 않는가? | `normal`을 metadata fallback으로 정의했다. |
| 8 | root element 대신 body 또는 첫 자식의 `color-scheme`을 잘못 읽지 않는가? | 실제 HTML document root `NodeId`와 revision 고정 computed snapshot을 요구한다. |
| 9 | `prefers-color-scheme`가 used root scheme에 맞춰 OS 값과 달라지지 않는가? | media 결과와 Canvas scheme을 별도 관측·정확 비교한다. |
| 10 | metadata parsing이 CSS color-scheme grammar와 다른 토큰을 허용하지 않는가? | supported token allowlist와 unsupported computed value fail-closed를 명시했다. |
| 11 | 중복 `meta name=color-scheme`에서 임의로 다음 값을 선택하거나 잘못된 문서 동작을 지원으로 오인하지 않는가? | HTML 금지 입력은 fail-closed한다. 고정 Chrome 155 duplicate 관측을 oracle 기록에만 남기고 제품 규칙으로 복제하지 않는다. |
| 12 | 단일 invalid meta content를 dark 또는 stale scheme으로 처리하지 않는가? | 단일 invalid 값은 supported-scheme 없음으로 처리하고 pinned Chrome의 light Canvas와 비교한다. |
| 13 | user preference나 renderer OS theme에 따라 Canvas byte가 달라지는 경우를 숨기지 않는가? | Chromium build와 정확한 sRGB byte를 고정하고 universal palette가 아님을 명시했다. |
| 14 | background 변경이 system status/navigation bar까지 덮는가? | native content root만 수정하고 system UI를 명시적으로 제외했다. |
| 15 | OS scheme 전환 전에 이전 cascade의 색을 새 revision처럼 게시하는가? | environment/style/document revision equality와 stale rejection을 판정 기준에 넣었다. |
| 16 | root CSS cascade 오류나 timeout을 임의 light 값으로 성공 처리하는가? | 실패 시 기존 색 유지 및 식별 가능한 오류로 닫는다. |
| 17 | DPR, page scale, viewport physical pixel을 혼합해 background 영역을 잘못 계산하는가? | background는 단색 Canvas fill이며 viewport·page scale·DPR을 별개로 보존한다. |
| 18 | CSS body/html background, CanvasText, form control까지 지원한다고 오해되는가? | body propagation·CanvasText·controls·scrollbars를 구현/검증 범위에서 제외했다. |
| 19 | Android AVD 결과를 기기 GPU 또는 iOS hardware 결과로 일반화하는가? | 두 환경을 별개로 기록하고 API 37 AVD·iOS Simulator에만 판정을 한정한다. |
| 20 | Canvas fill을 HTML layout/paint나 C04.12 전체 완료로 과장하는가? | DOM/text/layout/paint 제외 및 C04.12 unchecked 유지 조건을 명시했다. |

**계획 판정:** 20개 관점이 각각 계획의 oracle, revision gate, 실패 처리 또는 명시적 범위 제한에 연결됐다. 구현 성공 판정에는 사용하지 않는다.
