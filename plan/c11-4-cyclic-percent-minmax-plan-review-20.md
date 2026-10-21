# C11.4 cyclic percentage `minmax()` 계획 검토 20개

검토 대상: [cyclic percentage `minmax()` 계획](./c11-4-cyclic-percent-minmax.md), Chrome 156 사전 비교 fixture와 JSON, CSS Grid 2025 CRD §2.3에서 도출한 기대값.

| # | 독립 실패 관점 | 차단 조건과 계획의 대응 |
| ---: | --- | --- |
| 1 | intrinsic sizing 중 cyclic percentage를 최종 track size로 먼저 풀어 container basis를 축소 | intrinsic contribution으로 root 24×16을 먼저 계산한 뒤 final percentage resolution을 분리했다. |
| 2 | `minmax()` 전체를 `auto`로 바꾸어 fixed zero minimum까지 유실 | 계획은 max percentage만의 cyclic 처리와 min `0px` 불변을 명시한다. |
| 3 | max-content width와 max-content height를 동시에 써 column/row 결과가 상호 의존 | 각 scene은 한 축만 intrinsic sizing으로 만들고 반대 축은 고정 입력이다. |
| 4 | definite width control을 cyclic case로 잘못 분류 | control은 40px definite content width로 고정하고 20px track을 요구한다. |
| 5 | root `border-box`나 padding 때문에 50%의 basis가 content size와 달라짐 | 모든 scene에서 border/padding을 0으로 고정하고 box size를 content size와 같게 둔다. |
| 6 | item width/height가 auto라 stretch alignment가 contribution과 frame을 흔듦 | item을 24×16px로 지정해 intrinsic contribution과 사용 frame을 고정한다. |
| 7 | implicit track이 생겨 explicit `minmax()`의 computed track만 비교 | 각 root에 child 하나와 explicit track 하나만 두며 runtime trace에서 track 수를 함께 검사한다. |
| 8 | 12px/8px used track을 root/item frame 크기와 혼동 | 기대 표에 track, root, item을 별도 열로 둔다. |
| 9 | item overflow가 viewport 밖이라 화면 표본에 잡히지 않음 | cyclic overflow는 root 안에서 track 경계를 넘는 x=18/y=12 표본을 잡는다. |
| 10 | root background가 item 색으로 오인되어 track 바깥 표본이 잘못 통과 | root와 item에 고정된 서로 다른 opaque 색을 사용하고 exact pixel color를 판정한다. |
| 11 | browser의 page y offset이 scene frame 기대값에 더해짐 | fixture는 root-local geometry를 기록하며 absolute page position은 acceptance에서 제외한다. |
| 12 | CSSOM serialization 문자열만 비교해 실제 used sizes 오류를 놓침 | computed track list와 DOMRect root/item geometry를 모두 보존하고 앱 stage trace와 대조한다. |
| 13 | `grid-template-columns`와 `grid-template-rows` trace가 뒤바뀜 | 열 scene·행 scene을 분리하고 두 축 trace 이름 및 배열을 검사한다. |
| 14 | Simulator PixelCopy 좌표를 CSS px로 직접 취급하여 density 배수 오류 | 실행 계획에서 raw surface coordinates와 display scale을 기록해 표본 변환을 명시한다. |
| 15 | Android screenshot 전에 새 frame이 제출되지 않아 이전 scene을 읽음 | surface 표본은 해당 scene의 draw/present 완료 뒤에만 캡처한다. |
| 16 | iOS Metal drawable sample을 compositor-after pixel proof로 과장 | 계획은 compositor 제출 전 경계를 분명히 제한한다. |
| 17 | Android AVD 결과를 실기기 호환성이나 hardware GPU로 확대 해석 | Android target을 API 37 ARM64 AVD로 고정하고 실제 기기/하드웨어 GPU를 범위 밖으로 둔다. |
| 18 | fixture ID나 payload가 기본 product archive에 포함되어 opt-in 경계가 깨짐 | 기본 archive negative check와 fixture-test positive control을 별도 acceptance로 둔다. |
| 19 | 특정 scene/style가 실패한 뒤 부분 layout이 성공으로 게시 | source allowlist·node/style/revision 검사와 non-partial failure를 요구한다. |
| 20 | 세 장면 pass를 `minmax()` 일반 지원 또는 C11.4 완료로 표현 | plan, PR, official status에 fixed opt-in 범위와 `72/213` 미승격 조건을 반복 명시한다. |

판정: 계획을 고정된 3-scene opt-in fixture로 제한하면 구현을 시작할 수 있다. CSS Grid 2025 CRD 규범 기대와 Chrome 156 관찰을 분리했고, Chrome version, track/root/item geometry, track 바깥 surface 표본 및 artifact 경계를 acceptance에 포함했다. 이 계획은 `calc()`, percentage minimum, `fit-content(%)`, 동시 cyclic axis, nested Grid, WPT, 실기기 및 일반 CSS 지원을 다루지 않는다.
