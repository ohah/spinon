# C11 Root HTML Grid typed math gap 구현·runtime 실패 관점 검토

검토 기준은 계획 PR #1077에서 고정한 typed-math gap diff와 기본 Root HTML Grid runtime 경로다. 계획 검토와 별도로 실제 projection·layout 호출, 실패 경계, Android/iOS simulator 결과를 확인했다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | math gap이 모든 CSS/Grid profile에 새로 허용되는가 | 허용하지 않는다. Root HTML gap 검증만 해당 typed math를 받으며 Flex 입력 변환은 수정하지 않았다. |
| 2 | 기존 `normal`, px, 직접 percentage가 달라지는가 | 기존 직접 gap resolver 분기와 계산을 유지했다. Root HTML percentage/px 회귀 테스트가 통과했다. |
| 3 | shorthand `gap`이 한 축 또는 반대 축을 잘못 덮는가 | `gap:calc(5% + 2px)`에서 column 12px, row 7px의 2×2 frame을 확인했다. |
| 4 | legacy `grid-gap`이 별도 계산 경로로 빠지는가 | `grid-gap:calc(5% + 2px)`도 같은 longhand used value와 frame을 냈다. |
| 5 | percentage 검색이 큰·깊은 트리에서 제한을 우회하는가 | admission과 evaluator 모두 depth 32, node 256, operator arity 64 제한을 검사한다. 긴 트리/잘못된 arity는 오류다. |
| 6 | row percentage가 width를 기준으로 계산되는가 | row-gap은 `style.height`를 사용한다. fixed-height 100px의 calc 행 배치에서 기대 y 좌표와 일치했다. |
| 7 | column percentage가 height를 기준으로 계산되는가 | column-gap은 `style.width`를 사용한다. fixed-width 200px의 calc 열 배치에서 기대 x 좌표와 일치했다. |
| 8 | pure-length calc가 필요 없는 percentage basis 때문에 거부되는가 | Root HTML gap resolver는 percentage가 없을 때만 evaluator에 0 basis를 전달한다. auto-axis 직접 resolver 단위 검사가 `calc(5px + 7px) = 12px`를 확인한다. |
| 9 | percentage 식이 indefinite/auto 축에서 허용되는가 | 해당 축 크기가 definite finite nonnegative 값이 아니면 admission 또는 layout 검증에서 거부한다. row/column 각 실패가 회귀 검사에 있다. |
| 10 | percentage 식에서 min-size 제한을 무시하는가 | min-width 조건을 projection integration에서 확인했고 기존 guard가 `min-height` 축에도 적용된다. |
| 11 | percentage 식에서 max-size 제한을 무시하는가 | max-width와 max-height calc-gap 입력이 각각 해당 property로 거부됨을 확인했다. |
| 12 | calc ID 누락이 다른 값으로 조용히 대체되는가 | `MissingCssMath`를 반환하는 Root HTML gap 단위 검사를 추가했다. |
| 13 | 다른 node 소유 calc 식을 재사용할 수 있는가 | node ID 불일치는 `CssMathBindingMismatch`로 거부하는 단위 검사를 추가했다. |
| 14 | row/column property-bound 식을 바꿔 끼울 수 있는가 | RowGap에 ColumnGap 식을 주입하면 binding mismatch로 거부하는 단위 검사를 추가했다. |
| 15 | 곱셈·나눗셈·min/max/clamp 차원 검증을 우회하는가 | projection은 기존 typed AST를 복사하고 기존 evaluator의 차원 검증을 사용한다. 산술·min/max/clamp 사용값 회귀가 통과했다. |
| 16 | 음수·NaN·무한대 결과가 비유한 gap이나 음수 gap으로 Taffy에 들어가는가 | 기존 `LayoutCssMathValue::resolve`의 속성별 range censor를 유지한다. 음수/NaN 0 처리와 양의 극한 유한값 경계 테스트가 통과했다. |
| 17 | implicit row의 확대 track 계산에서 calc gap이 빠지거나 중복되는가 | row extent와 expanded-row 검증에 같은 math 목록을 전달했다. percentage auto-row list와 calc row-gap 조합의 item 위치가 기대값과 일치했다. |
| 18 | column-flow의 percentage implicit column에서 projection deferral 후 extent 검사가 사라지는가 | style 단계는 calc gap aggregate 검사만 layout 단계로 넘기고, layout 단계가 50% implicit column과 calc gap의 전체 extent를 다시 검사한다. x=52px, column=100px 회귀가 통과했다. |
| 19 | named-area span에서 typed gap이 빠지거나 두 번 적용되는가 | named-area candidate가 공통 gap validator를 호출한다. header span과 아래 행·두 번째 열의 frame이 독립 기대값과 일치했다. |
| 20 | overflow, harness, 증거가 구현 범위보다 크게 주장되는가 | Android API 37 ARM64 AVD/SwiftShader 및 iPhone 17 Pro iOS 26.2 Simulator에서 동일 1-grid fixture의 7 box/frame과 overflow 화면을 기록했다. 화면은 visual smoke로만 다루며 PixelCopy/Metal readback, 실기기·hardware GPU, 일반 CSS 지원을 주장하지 않는다. |

추가로 첫 경계 테스트가 definite width/height 없는 row-flow 입력을 사용해 Root HTML Grid의 기존 축 크기 gate에서 실패했다. 이를 CSS math resolver 결함으로 확대하지 않았다. integration 입력을 유효한 definite-axis 범위로 고치고, pure-length의 basis 독립성은 gap resolver 단위 검사로 분리해 검증했다.

재검증: Root HTML gap integration 8/8, Root HTML gap resolver 단위 3/3, `spinon-layout` 전체에서 기존 C15 실패 1개만 제외한 379/379, `spinon-style-to-layout`에서 기존 비관련 실패 2개만 제외한 321/321, scoped Clippy, `cargo fmt --all -- --check` 통과. 제외한 기존 테스트의 원인·원본 로그는 [패키지 회귀 기록](./package-test-note.md)에 둔다.
