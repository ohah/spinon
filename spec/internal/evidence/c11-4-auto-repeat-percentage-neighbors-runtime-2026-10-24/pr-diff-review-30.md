# C11.4 percentage auto-repeat neighbors · PR diff 실패 관점 검토 30개

검토 대상은 이 branch의 cascade/layout 변경, 고정 fixture와 capture 경로, Android/iOS runner, 내부 계약·상태·roadmap 문구 및 simulator evidence다. 계획 검토 20개와 구현/runtime 검토 20개를 재사용하지 않고 PR 변경 경계와 회귀 가능성을 별도로 확인했다. C11.4 checkbox와 공식 완료 수는 바꾸지 않는다.

| # | 별도 실패 관점 | 확인한 변경·근거 | 판정 |
|---:|---|---|---|
| 1 | cascade가 auto-repeat fragment 안의 percentage까지 허용하는가 | `is_auto_repeat_neighbor_track`만 prefix/suffix classification에 사용되고 repeat fragment는 기존 fixed-minimum validation에 남는다. | 통과 |
| 2 | cascade가 음수·비유한 percentage를 layout으로 넘기는가 | 새 variant guard는 finite·nonnegative 값만 허용한다. layout direct-input negative tests도 양 축에서 거부한다. | 통과 |
| 3 | repeat 경계가 잘못된 상태에서 이웃을 검증하거나 slice하는가 | `start < end <= tracks.len()` 검사가 neighbor slicing보다 먼저 실행된다. | 통과 |
| 4 | authored/repeated line-name 길이 검사가 이웃 변경으로 우회되는가 | 기존 line metadata length validation이 neighbor gate 전에 유지된다. | 통과 |
| 5 | percentage가 없는 legacy fixed-px profile이 달라지는가 | no-percentage 분기는 기존 finite·nonnegative `LengthPx` predicate를 유지하고 다른 track kind는 거부한다. | 통과 |
| 6 | indefinite axis가 외부에서 전달된 `true` flag만으로 허용되는가 | layout caller는 동일 grid node의 definite width 또는 height flag를 축별 context에 넣고 helper가 다시 확인한다. | 통과 |
| 7 | columns와 rows가 서로의 dimension을 basis로 사용하는가 | property selector는 columns→width/left-right, rows→height/top-bottom으로 짝지어진다. | 통과 |
| 8 | auto dimension 또는 0·음수 dimension이 positive basis로 취급되는가 | dimension은 finite positive `Fixed`여야 한다. | 통과 |
| 9 | min/max 제약이 있는 축까지 새로 지원하는가 | minimum은 fixed zero, maximum은 auto인 기존 제한만 통과한다. | 통과 |
| 10 | border-box 외곽 크기를 percentage basis로 사용하는가 | border-box helper는 padding과 border를 제외하며 matrix는 136px outer / 116px content basis를 검사한다. | 통과 |
| 11 | border-box padding의 다른 단위·음수·비유한 입력이 잘못 산입되는가 | helper는 finite nonnegative `LengthPx` padding만 계산에 넣고 그 외 값은 닫는다. | 통과 |
| 12 | content-box dimension에서 padding을 또 빼는가 | content-box branch는 이미 content dimension인 fixed 값을 그대로 basis로 쓴다. 240px basis scene의 독립 기대값과 맞는다. | 통과 |
| 13 | 개별 percentage×basis가 finite여도 합산 단계가 `f32`를 넘는가 | 전체 neighbor 합을 `f64`로 더해 `f32::MAX` 초과를 거부하며 해당 회귀 입력을 추가했다. | 통과 |
| 14 | percentage 중 하나만 보고 다른 neighbor의 invalid kind를 놓치는가 | percent가 하나라도 있으면 prefix와 suffix의 모든 track을 다시 순회해 허용된 px/percentage만 합산한다. | 통과 |
| 15 | unsupported intrinsic/fraction/calc neighbor가 우연히 통과하는가 | 이웃 match는 fixed px와 percentage만 받으며 나머지는 `false`를 반환한다. | 통과 |
| 16 | 이웃 확장이 auto-repeat fragment의 minimum grammar를 넓히는가 | `repeat_tracks`에는 기존 `auto_repeat_track_has_fixed_minimum`을 별도로 계속 적용한다. | 통과 |
| 17 | 두 축을 동시에 검증하면서 error property나 context가 섞이는가 | caller가 각 축에 다른 property와 definite flag를 제공하고 각 validation 오류는 해당 CSS property를 유지한다. | 통과 |
| 18 | Stylo가 prefix/suffix ordering 또는 repeat range를 변조하는가 | cascade 회귀는 양쪽 percentage와 zero-prefix input의 typed track 순서 및 `(start,end)`를 고정한다. | 통과 |
| 19 | Stylo가 제한 parser 밖의 임의 author CSS를 지원한다고 홍보하는가 | 테스트는 fixed-size `auto-fill`/`auto-fit` fixture profile에 한정되고 일반 admission 확장은 없다. | 통과 |
| 20 | auto-repeat minimum/max와 percentage neighbor 변경이 섞여 이전 gates를 바꾸는가 | 코드 변경은 neighbor predicate에 국한되고 repeated track minimum 검사 및 maximum 검사 경로는 그대로 호출된다. | 통과 |
| 21 | 13개 fixture 기대값이 runtime scene과 같은 ID 집합·순서를 보장하는가 | JSON, 규범 기대값, Chrome observation, FFI catalog test와 capture boundary가 ID 집합·순서를 교차 확인한다. | 통과 |
| 22 | 기준 CSS 숫자와 13개 normative used-track list가 서로 맞지 않는가 | 독립 CRD 기대값은 각 scene의 axis, gap, repeat count, track list와 collapse list를 고정하고 capture에서 비교한다. | 통과 |
| 23 | Chrome의 auto-fit 차이를 규범 기대값에 섞는가 | Chrome CSSOM 관찰과 normative used-track list를 서로 다른 JSON에서 비교하고 196px/200px 차이를 유지한다. | 통과 |
| 24 | auto-fit item placement/collapsed gutter를 단순 track count만으로 통과시키는가 | fit-first·fit-empty 두 축에서 expanded track size, collapse bitmap, item frame과 runtime trace를 각각 검사한다. | 통과 |
| 25 | border-box scene에서 fixture item frame이 basis 산술과 무관하게 고정되는가 | 136px border box·116px content basis의 columns/rows scene 모두 browser oracle 및 runtime track expectation에 연결된다. | 통과 |
| 26 | overflow control이 CSS overflow 동작이나 전체 C11.4 지원으로 과대 해석되는가 | 80%+30% scene은 auto-repeat minimum count/track outcome만 확인하며 일반 overflow CSS admission은 추가하지 않는다. | 통과 |
| 27 | opt-in fixture adapter 또는 JSON payload가 기본 FFI archive로 새는가 | module 선언·include 경로는 `c04-runtime-gpu-fixture-test` gate 안에 있고 default archive full-stream negative control과 opt-in positive control을 모두 남겼다. 결과는 static archive 범위로 한정한다. | 통과 |
| 28 | scene allowlist와 native runner의 expected frame/box count가 어긋나는가 | Android/iOS fixed scene tables와 box count derive가 item 없는 root-only scene을 포함하며 Node boundary test가 양 플랫폼 ID를 확인한다. | 통과 |
| 29 | PixelCopy/Metal 표본이 다른 scene 또는 잘못된 node/좌표에서 exact로 오인되는가 | 샘플 검사는 고정 scene ID, run/generation, sample 수, node identity, interior coordinate와 RGB 기대값을 확인한다. 15/15은 선언 표본만 의미한다. | 통과 |
| 30 | 공식 상태·공개 roadmap이 제한 runtime slice를 C11.4/C13 전체 완료 또는 현재 scroll 작업으로 표시하는가 | `STATUS`/plan/contract는 72/213과 미완료 checkbox를 유지한다. 공개 roadmap은 C11.4만 활성으로 두고 C13은 미완료·비활성, 미구현 public scroll/CSSOM, 부분 native wheel 및 남은 C13.4–6를 명시한다. | 통과 |

최종 evidence는 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에 한정한다. PixelCopy/Metal은 선언한 15개 표본이며 전체 화면 byte parity나 iOS compositor 이후 표시를 주장하지 않는다. GitHub required checks와 merge state는 PR 생성·병합 직전에 별도로 조회한다.
