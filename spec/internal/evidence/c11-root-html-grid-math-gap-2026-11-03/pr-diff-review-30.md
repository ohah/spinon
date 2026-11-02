# C11 parsed Root HTML Grid typed math gap 구현 PR 변경 검토 30개

검토 대상은 구현 커밋 `7d62723da`의 실제 변경분과 PR #1078에 추가한 `spec/STATUS.md` 제한 범위 기록이다. 계획 리뷰 30개와 구현/runtime 리뷰 20개를 재사용하지 않고, 실제 데이터 경로·오류 경계·상태 표기별 실패 경로를 다시 확인했다.

| # | 별도 실패 경로 / 경계 | 검토 결과 |
|---:|---|---|
| 1 | `normal` gap이 math record 부재만으로 reject되는가 | 통과 — math가 없을 때 기존 normal/px/percentage resolver 결과를 먼저 사용한다. |
| 2 | gap의 calc ID가 다른 CSS math record로 연결되는가 | 통과 — layout resolver가 gap의 `LayoutCalcId`와 record ID를 직접 비교해 선택한다. |
| 3 | 사용된 calc ID에 record가 없을 때 기본값으로 이어지는가 | 통과 — `MissingCssMath`로 해당 gap property를 반환하며 dedicated resolver test가 있다. |
| 4 | 입력에 중복 math ID가 있어 첫 record가 임의 선택되는가 | 통과 — layout 입력 사전 검증이 ID 중복을 `DuplicateCssMathId`로 거부한 뒤 Grid 계산을 시작한다. |
| 5 | math record의 `NodeId`가 다른 Grid의 식인데 공유되는가 | 통과 — resolver가 요청 node와 record owner를 대조하고 불일치를 거부한다. |
| 6 | `row-gap` 식이 `column-gap`에서 재사용되는가 | 통과 — CSS math property binding도 축별로 비교하고 반대 축 단위 검사가 포함됐다. |
| 7 | column percentage가 height 또는 row percentage가 width를 기준으로 계산되는가 | 통과 — `row-gap`은 height, `column-gap`은 width만 선택한다. 서로 다른 축 크기의 integration scene이 확인한다. |
| 8 | definite zero basis가 미정 값으로 처리되는가 | 통과 — 0은 유한하고 비음수인 definite basis로 허용되며 `75% + 2px` 결과가 2px다. |
| 9 | percentage 식이 auto/indefinite 축에서 임의의 0 basis를 받는가 | 통과 — projection은 finite definite 축을 요구하고 layout resolver도 `Fixed` 외 값을 거부한다. |
| 10 | percentage basis의 음수·NaN·무한대가 계산에 들어가는가 | 통과 — projection과 layout 양쪽에서 finite/non-negative 조건을 확인한다. |
| 11 | percentage gap에 기존 `min-width`/`min-height` 제약이 묵인되는가 | 통과 — 해당 축의 min constraint는 기존 subset대로 거부하며 column·row 사례를 확인했다. |
| 12 | percentage gap에 기존 `max-width`/`max-height` 제약이 묵인되는가 | 통과 — 대응 max constraint가 `none`/`Auto`가 아니면 거부하며 width·height 사례가 있다. |
| 13 | pure-length `calc()`가 없는 축 basis 때문에 불필요하게 거부되는가 | 통과 — percentage 없는 AST에는 0 basis를 전달하고 resolver 단위 검사와 indefinite-width column scene이 확인됐다. |
| 14 | CSS math의 잘못된 차원 조합이 새로 허용되는가 | 통과 — 표현식은 기존 typed AST/evaluator를 거치므로 number·length 차원 오류를 layout 단계에서 거부한다. |
| 15 | unary·clamp AST의 자식 중 percentage가 누락되어 잘못된 basis 검사를 하는가 | 통과 — projection visitor는 negate/invert와 clamp 세 자식, 목록형 연산자의 모든 자식을 순회한다. |
| 16 | CSS math depth/node/arity 제한을 우회해 재귀 비용이 늘어나는가 | 통과 — projection 검사와 기존 layout AST 검사가 depth 32, node 256, 인자 64 상한을 확인한다. |
| 17 | 계산된 음수 gap이 negative Taffy spacing으로 전달되는가 | 통과 — 기존 `RowGap`/`ColumnGap` range censor가 음수를 0으로 제한하며 위치 regression이 있다. |
| 18 | 0/0의 NaN이 frame이나 box 좌표로 전파되는가 | 통과 — existing evaluator가 top-level 결과를 0으로 censor하며 두 번째 item x=10을 확인한다. |
| 19 | 1/0의 양의 무한대가 비유한 gap으로 전파되는가 | 통과 — evaluator의 CSS layout unit 상한을 적용하고 output 좌표 finite stress assertion이 있다. |
| 20 | explicit track 목록에 gutter 수를 한 번 적게/많게 곱해 overflow 검사가 빠지는가 | 통과 — 명시 Grid 경로가 resolved gap과 사용 track 사이 gutter 수를 곱하고 track extent validator에 전달한다. |
| 21 | auto-row expansion에서 implicit row 사이 gap이 누락되거나 이중 적용되는가 | 통과 — expanded row 수의 gutter만 aggregate 검사에 반영하고 row origin regression이 있다. |
| 22 | sparse column-flow의 implicit percentage column에서 math gap deferral 뒤 총 extent 검사가 사라지는가 | 통과 — projection deferral은 typed gap에만 해당하고 layout 경로가 같은 resolved gap으로 implicit extent를 다시 검사한다. |
| 23 | named-area span의 gap이 area 폭 또는 아래 row 위치에 두 번 더해지는가 | 통과 — named-area scene은 spanning header width와 아래 row·다음 column 위치를 각각 독립 기대값과 비교한다. |
| 24 | `min()`/`max()`/`clamp()`의 경계 또는 clamp의 min>max 처리가 바뀌는가 | 통과 — min/max 결과가 다른 표본과 min 우선 clamp 표본이 서로 다른 frame 값으로 구분된다. |
| 25 | `gap` shorthand와 legacy `grid-gap`이 장축별 property binding을 바꾸는가 | 통과 — 두 선언을 같은 integration suite에서 각 축 7/12px geometry로 확인했다. |
| 26 | typed column gap이 있는 relative implicit extent 검증이 무관한 literal 입력까지 건너뛰는가 | 통과 — early return 조건이 `layout_math_values`의 정확한 `column-gap` key에 한정되고 literal은 기존 finite resolver로 검사한다. |
| 27 | calc 오류 후 일부 Grid frame이 남아 성공처럼 보이는가 | 통과 — layout input 검사와 Root HTML validation이 Taffy layout 결과 publish 전에 실패를 반환한다. 이 PR은 오류 결과를 성공 frame으로 변환하지 않는다. |
| 28 | 일반 CSS/Flex/Grid 외 profile에 Root HTML math gap admission이 전파되는가 | 통과 — 변경된 검증은 `is_grid_profile`/Grid container 검사 경로 안에 있고 Flex 또는 새 일반 CSS 지원을 추가하지 않는다. |
| 29 | projection math record가 layout callback에서 유실되거나 다른 node에 묶이는가 | 통과 — `parse_gap`이 property-bound value와 동일 ID를 `math` vector에 넣고 layout 입력의 ownership validation과 callback이 이를 확인한다. |
| 30 | PR 문서가 시뮬레이터 화면을 픽셀 동등성 또는 C11 완료로 과장하는가 | 통과 — Android AVD/SwiftShader와 iOS Simulator를 구분하고 화면을 visual smoke로 제한했다. C11 checkbox와 공식 72/213은 유지하며 전체 CI/WPT/실기기 완료를 주장하지 않는다. |

결론: 이번 PR diff에서 별도 차단 결함을 찾지 못했다. focused·패키지 회귀 결과의 제외 항목은 [패키지 회귀 기록](./package-test-note.md)에 남겼고, 시뮬레이터의 검증 한계는 [실행 근거](./README.md)에 명시했다.
