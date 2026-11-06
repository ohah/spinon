# C11.4 두 implicit auto 행 구현 및 RuntimeSession 검토 20개

계획 PR #1230 병합 뒤 구현 코드와 실제 RuntimeSession 결과를 대상으로 서로 다른 실패 경로를 확인했다. 순수 코드 검토와 실행 검증은 표에서 구분한다.

검토 중 text payload가 이미 `String`인데 `into_owned()`를 호출한 compile error와 `match`를 `matches!`로 쓰라는 Clippy 지적을 발견해 수정했다. 수정 후 RuntimeSession·style-to-layout focused tests, Clippy와 formatter를 다시 실행해 통과시켰다.

| # | 실패 경로 | 확인과 결과 |
|---:|---|---|
| 1 | 표시되는 Grid owner가 없거나 둘 이상임 | 기존 Root HTML owner 단일성 검사 뒤에만 profile gate가 호출되는 것을 코드로 확인했다. 변경 owner 수용 범위는 없다. |
| 2 | Grid가 지정한 40×40 content box를 벗어남 | 기존 owner dimension gate가 width/height 및 min/max를 검사한다. 통과. |
| 3 | visible child가 0, 1, 4개 이상임 | 새 count gate는 2 또는 3만 받는다. 네 번째 child mutation은 완성 layout 없이 거부된다. |
| 4 | 첫 child가 비어 있지 않거나 text가 먼저 나옴 | source-order index 0은 `None` direct text만 받는다. reordered fixture는 거부된다. |
| 5 | 두 item legacy scene의 text가 사라짐 | `(2, 1, Some(_))` 경로를 유지하고 기존 ASCII text grammar를 그대로 적용한다. 기존 single implicit-row 테스트 통과. |
| 6 | 세 item scene의 첫 text가 임의 문자인 경우 | 두 implicit row shape는 첫 text `A`만 받는다. `C` mutation은 부분 layout 없이 거부된다. |
| 7 | 세 item scene의 둘째 text가 임의 문자인 경우 | 두 번째 implicit row shape는 `B`만 받는다. `A` mutation은 부분 layout 없이 거부된다. |
| 8 | text item source order가 뒤바뀜 | index와 source를 함께 비교한다. A/B 순서 교체 fixture는 거부된다. |
| 9 | text에 비ASCII·markup·공백 문법 확장이 섞임 | 기존 direct-text ASCII grammar 검사와 A/B exact gate가 순서대로 적용된다. 기존 invalid-text 회귀 통과. |
| 10 | text item이 nested markup 또는 둘 이상의 descendant를 가짐 | 기존 direct-child 검사에서 거부된다. 현재 세 item 수용 fixture는 각 text를 direct child 하나로 둔다. |
| 11 | visible direct child가 `div`가 아니거나 Grid 중첩임 | 기존 item-kind 및 nested Grid 차단을 통과해야 한다. 통과. |
| 12 | hidden child가 item index를 밀어냄 | hidden node를 count와 index에서 제외하고, 남은 visible source order를 exact shape와 대조한다. 통과. |
| 13 | fixed item이 40×10 외의 크기 또는 auto size를 사용함 | 기존 fixed dimension gate와 auto min/max 조건을 유지한다. 통과. |
| 14 | text item이 fixed width/height, 비정적 위치 또는 explicit placement를 사용함 | 기존 auto size, static position, auto placement gate를 유지한다. 통과. |
| 15 | `align-self` 또는 `justify-self`가 자동값이 아님 | 기존 item alignment gate가 거부한다. 통과. |
| 16 | font, line-height, writing mode 또는 text alignment가 달라짐 | 기존 monospace 10px·10px line-height·horizontal LTR gate를 유지한다. 통과. |
| 17 | Grid `align-content`가 `normal`에서 바뀜 | Root Grid gate가 거부한다. 새 테스트와 legacy rejection 테스트 통과. |
| 18 | sizing rejection 뒤 이전 layout이 남거나 새 layout이 부분 게시됨 | invalid input은 `completed=None`으로 거부되고, 같은 RuntimeSession에서 light profile 복귀 후 `[10,15,15]` scene이 다시 게시된다. 통과. |
| 19 | 두 item contribution이 하나의 row로 합쳐지거나 stretch trace가 하나를 누락함 | fixture-test trace는 explicit/implicit count `1/2`, 두 개의 10px minimum contribution, base `[10,10,10]`, stretch 결과 `[10,15,15]`를 확인한다. 통과. |
| 20 | 기존 default artifact에 fixture 경로 또는 title 문자열이 들어감 | `spinon-runtime` release rlib에서 두 marker를 검색해 부재를 확인했다. 새 FFI·feature·product selector는 없다. |

실행 근거와 제한은 [README](./README.md)에 있다. 특히 simulator 화면은 앱의 기본 Root HTML 경로 및 두 text draw를 확인하는 visual smoke이고, pixel parity나 실기기 결과를 뜻하지 않는다.
