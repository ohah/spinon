# C11.1 `fr` implicit-column 구현/runtime 실패 관점 검토 20개

검토 대상은 계획 문서가 아니라 현재 구현 diff, 고정 precomparison, focused Rust tests와 Android/iOS Simulator runtime log다. 각 항목은 별도 실패 경로 또는 계약 경계를 확인한다. 구현 slice는 아직 PR 병합 전 후보이며, 아래 결과를 C11.1/C11 완료로 해석하지 않는다.

| # | 실패 관점 | 확인과 결과 |
|---:|---|---|
| 1 | Root HTML 외 profile로 `fr` auto-column admission이 새는가 | Cascade opt-in은 기존 `allow_root_html_fixed_auto_column_lists`와 column flow에 함께 묶였다. Projection은 `RootHtmlColumnFlowV1` 후보를 따로 검증한다. 다른 profile admission 확장은 확인하지 못했다. |
| 2 | Row 또는 dense flow가 새 `fr` list를 받는가 | Cascade의 fraction option은 `auto_flow == Column`에서만 켜지고, projection/layout placement 계약은 sparse column-flow만 받는다. |
| 3 | `grid-auto-rows`에도 실수로 fraction list가 켜지는가 | Auto-row parser는 fraction option을 명시적으로 false로 전달한다. 기존 row profile 경로 변화는 확인하지 못했다. |
| 4 | scalar와 list에서 서로 다른 representation이 생기는가 | `1fr` scalar와 multi-track list 모두 typed track list로 유지된다. Cascade tests는 `1fr`, `1fr 2fr`, mixed list를 확인한다. |
| 5 | list 순환이 explicit-grid 끝이 아니라 첫 implicit column부터 시작하는가 | Layout matrix의 cycle 시작과 composite runtime item x 좌표가 고정 precomparison과 일치한다. |
| 6 | px-only 또는 percentage-only의 이전 의미가 깨지는가 | 기존 fixed/percentage tests를 그대로 유지했고, 새 허용 branch는 fraction variant를 추가하는 한정 변경이다. focused regression 및 C11.1 runtime regression이 통과했다. |
| 7 | negative, NaN, infinity factor가 Taffy로 전달되는가 | Cascade rejects non-finite/negative computed factors; layout validator도 동일 조건을 거부한다. CSS-invalid `-1fr`는 CSS initial fallback을 별도 테스트한다. |
| 8 | 빈 목록·10,000 초과 목록이 allocation 또는 modulo 전에 통과하는가 | Cascade reservation helper, projection quota 및 layout quota가 각각 유지된다. 새 코드에서 empty list admission은 발견되지 않았다. |
| 9 | minmax/fit-content/calc/intrinsic auto track 문법이 direct `fr` 변경과 함께 허용되는가 | 새 branch는 `LengthPx`, `Percentage`, `Fraction` variant로만 제한된다. unsupported grammar negative projection/cascade tests는 유지된다. |
| 10 | implicit column 수가 hidden item이나 direct-child 외 item에 의해 부풀려지는가 | Root HTML collector의 owned item count 경로를 변경하지 않았다. Runtime fixture는 한 Grid와 직접 fixed-size item만 쓴다. |
| 11 | definite width 검사가 relative pattern에만 적용되는가 | Layout·projection validator는 percentage/fraction auto-column이 있을 때만 basis 조건을 적용한다. px-only path는 기존 계약을 유지한다. |
| 12 | width가 indefinite, `auto`, math-backed 또는 음수/비유한일 때 사용할 수 없는 basis를 쓰는가 | computed width와 author `auto`/math property를 확인하고 concrete `width` rejection을 테스트한다. zero width는 유효 definite basis로 남는다. |
| 13 | min-width/max-width가 width basis를 다시 바꾸는가 | Cascade validator와 LayoutStyle boundary 모두 non-auto min/max를 거부한다. projection negative tests에서 두 property를 분리 확인한다. |
| 14 | aspect-ratio typed cascade 또는 serialization만으로 놓치는가 | Typed `layout_aspect_ratio`와 computed `aspect-ratio` property를 모두 확인한다. `aspect-ratio:1` rejection을 parsed Root HTML test에서 확인했다. |
| 15 | fixed·percentage tracks에서 column gap을 빼거나 두 번 빼는가 | Definite width basis와 gap check가 분리되어 있고 Taffy에 Grid width/gap을 각각 전달한다. Chrome precomparison 및 Rust expected starts와 합성 simulator scene이 일치한다. |
| 16 | percentage와 fraction이 섞일 때 percentage basis가 남은 flex space로 잘못 축소되는가 | `%`는 definite width의 비율, `fr`는 fixed/% tracks와 gaps 이후 여유 공간으로 독립 적용한다. 300px composite scene에서 10% track은 30px, `1fr`는 155px다. |
| 17 | factor 합이 1보다 작을 때 남은 자유 공간을 채우도록 확대되는가 | Fixed precomparison의 `0.25fr` 사례에서 1보다 작은 합 floor와 trailing free space를 검사한다. Taffy source의 `max(1.0)` 규칙과 부합한다. |
| 18 | `0fr`가 항상 0px가 되어 item automatic minimum을 잃는가 | 고정 크기 item의 automatic minimum을 가진 `0fr 1fr` test는 첫 track 10px를 기대한다. 기존 Taffy minimum sizing 경로를 보존한다. |
| 19 | 큰 finite factor 합이 f32에서 무한대가 되거나 알고리즘이 무한 반복하는가 | `1e38fr` scalar/list test와 vendored Taffy f64 accumulator/최대 반복 수를 대조했다. 네 track의 합이 f32 한도를 넘는 케이스도 focused layout matrix에 있다. |
| 20 | 실패한 allocation 또는 runtime scene이 partial output을 남겨 다음 valid 요청을 막는가 | Cascade reservation failure/recovery test와 default runtime scene의 layout publication key를 확인했다. Android/iOS는 동일한 fixture input에서 `layout=ready`, 15 boxes를 보고했다. |

## 검토 결과

구현 결함은 발견하지 못했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 실행은 한 개 composite scene의 layout/draw/present 확인이다. 이 리뷰는 Chrome pixel parity, Android PixelCopy, iOS Metal drawable readback, 실기기/hardware GPU, 전체 CSS admission, nested Grid, text/image contribution 또는 WPT 적합성을 주장하지 않는다. 계획 실패 관점 20개 및 계획 PR diff 30개와 별도 검토다.
