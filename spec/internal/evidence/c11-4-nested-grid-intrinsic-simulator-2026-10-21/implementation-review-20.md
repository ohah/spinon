# C11.4 nested Grid 구현 실패 관점 검토 20개

검토 대상은 고정 nested Grid direct-layout test, feature-gated runtime fixture adapter, FFI scene source, Android/iOS 앱 verifier, simulator capture parser와 이번 실행 근거다. 계획 검토 20개를 재사용하지 않았다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | opt-in fixture가 일반 제품 CSS 지원이나 C11.4 완료로 잘못 집계되는가 | FFI·runtime fixture는 전용 feature에 묶여 있고 공식 상태 checkbox와 72/213 집계는 그대로다. |
| 2 | 일반 nested Grid가 `subgrid`의 공유 track 의미로 바뀌는가 | 고정 source는 독립 nested `display:grid`이고 subgrid를 사용하지 않는다. 명세에 subgrid는 별도 미지원으로 남겼다. |
| 3 | outer Grid가 다른 수의 child나 중간 wrapper를 받아 잘못 계산하는가 | adapter가 root child 한 개와 그 직접 owner를 검사하고 다른 topology를 거부한다. |
| 4 | 같은 ID를 가진 foreign-namespace 또는 non-element node가 통과하는가 | tag 검사에 더해 HTML namespace 검사와 element-kind 검사를 추가했다. |
| 5 | nested Grid가 root 밖의 node이거나 ID가 바뀌어도 처리되는가 | nested `div` ID와 `HostParent::Node(root)`를 함께 검사한다. |
| 6 | item의 순서·개수·owner가 변해도 두 장면으로 취급하는가 | nested direct child가 정확히 둘이고 고정 item ID·순서·부모인지 검사한다. |
| 7 | leaf가 element가 아니거나 text가 간접 descendant로 바뀌는가 | 각 item에 direct text node 하나만 허용하고 parent와 exact string을 검사한다. |
| 8 | 다른 scene ID나 profile이 fixture 경로에 잘못 진입하는가 | 세 root ID만 allowlist하며 `RuntimeBlockFlexGridTrackSizingV1`만 처리한다. |
| 9 | outer width, height, min/max, box edge, gap, rows 또는 columns 차이가 조용히 통과하는가 | root의 typed dimension과 제한 style subset을 장면별로 비교하고 unsupported style이면 fail closed한다. |
| 10 | nested Grid의 width나 inner track/gap 차이가 outer 결과로 숨겨지는가 | nested의 auto width, `min-content max-content`, auto row, 4px column gap, zero edge를 별도로 검사한다. |
| 11 | one-track와 span-two placement가 서로 뒤바뀌는가 | `Line(2)`와 `Span(2)`를 scene별 expected placement와 대조한다. |
| 12 | 다른 font, font size, wrapping, direction 또는 item dimension으로 oracle이 달라지는가 | direct item의 Ahem 10px, line height, normal wrapping, direction 및 auto dimensions를 검증한다. |
| 13 | 호스트 설치 폰트 차이로 intrinsic width가 달라지는가 | direct-layout test는 system font 없이 고정 Ahem을 등록하며 앱 fixture도 Ahem resource를 사용한다. |
| 14 | nested Grid node를 내부 text leaf와 중복 집계하는가 | raw intrinsic leaf 목록에는 내부 item 두 개만 있고 nested Grid ID가 leaf node로 나오지 않는다고 검사한다. |
| 15 | nested Grid 자체에 leaf measure callback이 없는데 빈 `all(...)` 검사가 성공을 가장하는가 | 실제 trace에서 nested non-leaf callback이 없음을 확인했다. vacuous callback 검사 3개를 제거하고 nested node가 leaf measurement로 기록되지 않음을 명시적으로 검사한다. |
| 16 | parent/nested/leaf frame 또는 item offset이 다른데 track만 맞아 보이는가 | direct-layout/runtime test와 앱 runner가 세 장면의 root·nested·두 item frame을 각각 고정한다. |
| 17 | trace array 순서가 바뀌어 outer와 nested trace가 혼동되는가 | capture parser는 `trace.grid`와 expected node ID를 사용한다. 기존 `trace.node` field 착오와 첫 trace=outer 가정은 수정했다. |
| 18 | contribution이 다른 item·kind·span·값이거나 flex control 값 제한이 비어 통과하는가 | 두 intrinsic scene과 flex scene에서 item ID·kind·span·value를 검사한다. flex는 `Minimum=0/span 2`가 실제 있어야 하고 모든 값이 finite·non-negative·120 이하인지 확인한다. 앞서 발견한 vacuous 상한 검사를 보강했다. |
| 19 | 앱 scene allowlist, box count, PixelCopy/Metal sample 누락·중복·좌표·색 오류를 놓치는가 | Android/iOS 세 ID·frame·box 수를 고정하고 flex 표본 5개를 target node·좌표·RGBA로 검증한다. 첫 AVD의 allowlist 누락 및 Android RGB map 누락을 고친 뒤 각 플랫폼 5/5 exact로 재실행했다. |
| 20 | fixture symbol/payload가 기본 산출물에 섞이거나 실행 APK가 캡처한 APK와 다른가 | 기본 FFI archive의 fixture symbol·scene string 부재와 opt-in positive control 포함을 확인했다. Android local/install APK SHA가 같고 iOS Simulator app SHA를 기록했다. |

## 조치와 남은 경계

검토 중 발견한 두 검증 공백은 고쳤다. definite-flex의 `all(value <= 120)`은 nested contribution이 전혀 없어도 참이 될 수 있어 예상 `Minimum=0/span 2`를 찾는 non-vacuous 검사와 finite·non-negative 상한 검사를 추가했다. nested Grid의 Taffy leaf callback을 기대하던 `all(...)`도 빈 목록에서 참이었으므로 이를 제거하고 non-leaf measurement 부재와 parent contribution trace를 분리해 검사한다.

`implementation-review-20.md`는 fixture scene과 현재 Rust·native adapter 범위만 평가한다. multi-level nesting, subgrid, 일반 CSS parser admission, WPT, 실기기, Android hardware GPU, iOS compositor 이후 pixel을 보장하지 않는다.
