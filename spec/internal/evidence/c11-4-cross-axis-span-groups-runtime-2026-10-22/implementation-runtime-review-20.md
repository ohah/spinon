# C11.4 교차 축 span groups 구현/runtime 실패 관점 검토 20개

**대상:** 새 C11.4 제한 fixture, Taffy cyclic percentage 초기화와 final sizing rerun, Android/iOS Simulator 판정 경로. 계획 검토 20개 및 PR diff 검토와 분리해 작성했다.

| # | 독립 실패 관점 | 결과 및 확인 근거 |
|---:|---|---|
| 1 | 네 장면 외 임의 scene ID나 input이 새 fixture에 진입하는가? | FFI source allowlist와 runtime adapter 모두 고정 네 ID만 허용한다. 단위 검사에서 미등록 ID와 별도 unsupported input을 확인했다. |
| 2 | root tag, owner, `data-node`, style profile이 다른데 부분 layout을 내는가? | root와 child identity/owner/marker/style profile을 검사하며 다른 입력은 `unsupported_c114_cross_axis_span_groups_fixture`로 거부한다. |
| 3 | item 수·순서·중첩이 달라져도 direct text 경로를 재사용하는가? | 두 direct child와 순서별 ID를 exact 검사한다. 각 item은 direct text node 하나를 가져야 한다. |
| 4 | text 내용이 바뀌어 intrinsic contribution이 우연히 통과하는가? | `AA A A`와 `XXXX`를 exact 검사하고 Ahem 등록 font로만 측정한다. |
| 5 | OS font fallback 또는 font revision 누락이 결과를 오염시키는가? | system font 없이 Ahem bytes를 등록하고 두 item intrinsic source의 font registry revision을 검사한다. 사전 비교가 고정 Ahem SHA를 기록한다. |
| 6 | 양축 grid tracks, gap, border, sizing profile이 다른데 고정 수치를 적용하는가? | 3 columns·2 rows typed tracks, 두 2px gap, 2px border, width/height와 box sizing을 검사한다. |
| 7 | calc cyclic minimum에서 `10%`와 absolute `2px`를 함께 0으로 만들 수 있는가? | Taffy 초기화는 0% basis로 calc를 평가해 `2px`를 남긴다. initial trace `[2,2,2]`와 min-content batch `[9,19,19]`를 대조했다. |
| 8 | percentage가 있는 아닌 track 종류에도 fallback이 적용되는가? | fallback은 일반 `Track`, indefinite axis size, percentage 사용 조건을 모두 요구한다. 다른 조건은 기존 initialization을 유지한다. |
| 9 | calc expression의 resolved percentage가 최초 intrinsic sizing에 섞이는가? | indefinite initial pass는 percentage zero basis로 처리하고 최종 definite pass만 resolved content basis를 사용한다. |
| 10 | span-2 gap을 빼먹거나 두 번 빼서 planned increase가 달라지는가? | A와 B의 same-span minimum contribution에서 2px gutter를 한 번 빼고 독립 기대값 A 7px, B 17px per track을 확인한다. |
| 11 | 두 item이 공유하는 가운데 track의 증가량을 합산하는가? | §11.5.1 per-track maximum `[7,17,17]`을 Rust trace의 min-content batch와 대조했다. reverse order에서도 같은 결과다. |
| 12 | intrinsic width를 `[9,19,19]`의 합으로만 계산해 gutter/border가 빠지는가? | trace basis `[9,19,19]`와 4px gutters로 51px content size, 양쪽 border 포함 root 55px를 구별한다. |
| 13 | 최종 percentage resolution에 최초 indefinite available space가 재사용되는가? | column/row percentage rerun에 확정된 `container_content_box`를 `AvailableSpace::Definite`로 전달한다. |
| 14 | final rerun이 이전 intrinsic item contribution과 bases를 초기화하는가? | rerun의 track sizing callback은 기존 `base_size`를 넘기고 `rerun_percentage_*` 초기화 플래그를 끈다. 최종 column/row trace와 기존 cyclic regression 16개가 통과했다. |
| 15 | Columns보다 먼저 Rows가 item의 줄 수를 고정하는가? | trace pass index를 검사해 Columns가 Rows보다 앞서는지 확인한다. A의 line-derived block contribution은 intrinsic 20px, definite 30px다. |
| 16 | Minimum/MinContent/MaxContent가 item ID나 span 경계 없이 합쳐지는가? | 두 axis별로 두 node의 span-2 contribution을 각각 검사하고, A/B의 서로 다른 inline/block 측정값을 유지한다. |
| 17 | DOM reverse control이 placement나 ID까지 바꾸어 source-order 검증이 무효가 되는가? | reverse는 child/source order만 바꾸고 class별 placement 및 Node ID frame을 유지한다. |
| 18 | `f32` stringify 차이로 native frame 검사가 거짓 실패 또는 과도 허용되는가? | x/width native float 관측값은 Chrome oracle보다 최대 약 0.01042px 차이고, 검사는 CSS 1/64px 이내로 한정한다. definite geometry는 exact다. |
| 19 | root border box 밖 item이 실제로 그려지지 않거나 clear sample이 item 위를 찍는가? | PixelCopy와 Metal drawable 모두 root-relative `(50,4)` red item B, `(60,4)` black clear를 forward/reverse 각각 exact 확인했다. iOS는 submit 전 drawable이다. |
| 20 | 이전 app/scene registry, 플랫폼 색상 표 또는 default artifact가 성공을 가장하는가? | 첫 AVD/iOS 시도는 stale prebuilt app의 새 scene 거부로 실패했다. 최신 앱 빌드 뒤 Android RGB allowlist도 보완해 재실행했다. 기본 archive에는 scene/entry marker가 없고 opt-in positive archive에만 있다. |

**결론:** 최초 실행에서 확인된 stale simulator artifact와 Android sample-color allowlist 누락을 수정하고 네 scene을 두 simulator에서 다시 통과했다. 남은 경계는 README의 검증 범위에 적었다. 이 검토는 전체 CSS Grid 지원, Android 실기기/hardware GPU 또는 iOS compositor 이후 pixel을 입증하지 않는다.
