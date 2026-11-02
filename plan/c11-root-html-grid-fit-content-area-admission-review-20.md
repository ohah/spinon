# C11.1 fit-content named-area admission 계획 정정 검토 20개

실제 projection source에서 Chrome의 `"fixed fit"` 1×2 area map이 현재 Root HTML admission에 없음을 확인하고, 이름·장면 허용 목록 없이 유효한 explicit area 조합을 연결하도록 정정한 계획을 검토했다.

| # | 실패 관점 | 정정 및 방지 기준 |
| --- | --- | --- |
| 1 | Chrome named-area 기준을 기존 Spinon 지원으로 오인 | source gate가 거부하는 입력임을 근거 note에 명시하고 새 제한 admission으로 계획한다. |
| 2 | named-area 장면을 범위에서 제거해 계획을 쉽게 축소 | `"fixed fit"` Chrome 장면을 typed projection과 default-route runtime matrix에 유지한다. |
| 3 | named-area 검증 규칙이 scene/ID를 조회 | 임의의 CSS-valid area name과 dimension-matched map을 요구하고 ID/node 선택자를 금지한다. |
| 4 | area dimension이 explicit tracks보다 커 implicit track을 만듦 | row/column area dimensions가 explicit track counts와 정확히 같아야 한다. |
| 5 | parsed area를 잘못된 사각형으로 취급 | 기존 Stylo computed rectangular area 표현만 받고 문자열 parser는 변경하지 않는다. |
| 6 | named-area fit-content가 out-of-scope track size를 통과 | 두 축 track을 direct px 또는 `fit-content(px)`로 제한한다. |
| 7 | 음수·NaN·무한 fit-content가 Taffy에 도달 | px cap은 finite이고 non-negative여야 하며 잘못된 값은 node/property diagnostic으로 거부한다. |
| 8 | named-area 조합에서 fit-content가 한 축에도 없음 | 새 area gate에는 적어도 하나의 explicit fit-content(px)가 있어야 한다. |
| 9 | percentage/calc argument가 새 path에 새어 들어옴 | `fit-content(%)`와 `fit-content(calc())`를 계속 거부한다. |
| 10 | area name `"fixed"`/`"fit"` 문자열만 허용 | renamed area projection 대조로 identifier allowlist가 아님을 증명한다. |
| 11 | 장면 허용 목록을 우회하는 구현 | CSS computed layout value만 검사하고 host ID/class/child index 또는 fixture marker에 의존하지 않는다. |
| 12 | 기존 header/side/content 2×2 profiles 회귀 | fixed px, fraction, percentage 기존 named-area tests를 회귀 집합에 유지한다. |
| 13 | auto-repeat과 fit-content named-area를 함께 지원 | auto-repeat, named lines, implicit tracks는 기존 fail-closed gate에 남긴다. |
| 14 | column만 검사하고 fit-content row area를 누락 | computed projection matrix에 column/row fit-content cases를 각각 둔다. |
| 15 | area-created unsized tracks를 explicit fit-content로 오인 | 모든 area axis가 authored explicit tracks와 dimension-match해야 한다. |
| 16 | 실패가 선언·node 정보를 잃고 부분 layout을 남김 | unsupported value는 원래 Grid property/node를 담은 진단으로 fail-closed하며 partial frame을 발행하지 않는다. |
| 17 | projection 통과를 앱 runtime 동작으로 확대 | projection, default Root HTML route, AVD PixelCopy, iOS Metal drawable을 별도 근거 종류로 둔다. |
| 18 | root 경계 밖 overflow가 clip되어 잘못 통과 | 30px root 밖 fit item 표본을 PixelCopy/Metal에서 실제 확인하도록 계획한다. |
| 19 | 계획 정정으로 C11을 완료 처리 | `spec/STATUS.md`, C11/C11.1 checkbox와 official `72/213`은 그대로 둔다. |
| 20 | 계획 검토 전에 제품 코드를 시작 | area admission 정정 PR과 별도 20개 diff review를 병합한 후 code branch로 돌아간다. |

이번 정정은 Chrome 기준과 현재 Root HTML named-area gate 사이의 차이를 명시하고, CSS parser와 공개 계약을 넓히지 않은 채 기능 목표의 area 조합을 유지한다.
