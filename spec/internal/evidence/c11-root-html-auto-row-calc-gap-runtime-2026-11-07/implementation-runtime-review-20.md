# C11.4 auto-row `calc()` `row-gap` 구현·runtime 실패 관점 검토 20개

최종 코드와 실행 근거를 서로 다른 계약 경계와 실패 경로에서 확인했다. 초반 통합 실행에서 발견한 projection gate 누락과 inner placement validator의 math map 누락은 최종 검토 전에 수정했다. 기존 positional/distributed style profile의 두-item 제한은 넓히지 않고 문서에 남겼다. 아래 검토에서 추가 blocking finding은 없었다.

| # | 실패 관점 | 확인 및 결과 |
|---:|---|---|
| 1 | 계산 AST가 두 차원 값의 합을 보존하는가 | Style projection은 `LayoutLengthPercentage::Calc(id)`와 `RowGap` record를 연결한다. projection 4/4와 RuntimeSession 기본 Root HTML 경로 통과. |
| 2 | percentage와 length의 term 순서를 가정하는가 | 두 순서 `6.25% + 2px`, `2px + 6.25%` 모두 4.5px 독립 기대값 및 120-scene layout matrix와 일치. |
| 3 | 음수 항을 가진 계산에서 부호를 잃는가 | direct unary `Negate` term을 투영하고 `1px - 10%`, `25% - 2px`, `10% - 1px`, `120% - 2px`를 비교. |
| 4 | 한 dimension 두 개 또는 같은 dimension만 있는 식을 허용하는가 | `px + px`와 calc percentage 없는 length는 거부. Layout rejection 및 projection grammar tests 통과. |
| 5 | nested/product/min/max/clamp AST를 이 제한 profile에서 새로 허용하는가 | 기존 two-term predicate 외 shape는 fail-closed. `calc(5px)` runtime input은 실패 후 복구 확인. |
| 6 | NaN 또는 infinity operand가 finite calc로 통과하는가 | projection grammar에서 non-finite term을 거부하고 Layout input validation도 확인. |
| 7 | math ID가 빠졌을 때 panic 또는 zero fallback이 생기는가 | `MissingCssMath { property: "row-gap" }` 경계를 직접 검사. |
| 8 | 다른 Grid node 소유 math ID를 현재 Grid에서 재사용하는가 | owner mismatch는 `CssMathBindingMismatch`로 거부. |
| 9 | 같은 math ID를 다른 property에 바인딩하는가 | `ColumnGap` binding은 `row-gap` admission에서 거부. |
| 10 | 반복 ID가 map insertion 순서에 따라 덮어써지는가 | duplicate math ID는 `DuplicateCssMathId`로 실패. |
| 11 | percentage basis가 viewport, column width 또는 indefinite height로 바뀌는가 | 40px content-box block-size만 사용한다. Auto height와 profile size mismatch 거부를 Layout tests로 확인. |
| 12 | direct negative percentage와 negative calc 결과의 CSS 처리 경계를 혼합하는가 | direct negative declaration은 기존 fatal cascade diagnostic을 유지하며, valid negative calc result만 0px로 clamp. Runtime regression 통과. |
| 13 | 기존 math layout-range clamp를 무시해 거대한 유한 결과가 넘치는가 | `f32::MAX` length term은 `33,554,428` CSS px로 기존 resolver range clamp 후 extent 검사를 통과하는 것을 새 unit test로 고정. 비유한 operand 허용으로 해석하지 않는다. |
| 14 | total Grid extent 검사를 gap 한 번만 기준으로 수행하는가 | profile은 두 row gutters와 30px base row extent를 확인하며 가장 큰 지원 가능한 percentage 경계와 초과 경계를 테스트. |
| 15 | Taffy와 adapter가 gutter를 중복 적용하거나 생략하는가 | 수동 gutter 보정 없이 기존 Taffy sizing/alignment 경로를 유지하고 track trace의 gutter 제외 row sizes 및 item geometry를 독립 기대값과 비교. |
| 16 | positive/negative free space 정렬에서 auto rows와 overflow가 깨지는가 | `normal`, `stretch`, positional/distributed 값 전체를 2/3-item Layout oracle에서 대조; 120%보다 큰 gap scene이 center overflow를 보존. |
| 17 | Chrome 관측값을 기대값 생성기에 복사해 oracle 순환이 생기는가 | 기대값은 signed px + percentage × 40px 공식에서 독립 생성. 120 scenes, 1,500 coordinates, 최대 0.0208333334px 차이. |
| 18 | 2-item positional alignment를 기존보다 암묵적으로 공개하는가 | Projection validator의 기존 three-item requirement를 유지. 2-item 전 alignment matrix는 Layout-only이며 RuntimeSession representative scenes는 세 item으로 제한. |
| 19 | 실패 revision이 이전 layout을 partial/stale 완료값으로 노출하거나 복구를 막는가 | Dark media의 unsupported `calc(5px)`는 `Failed`, `completed=None`; light source로 되돌린 뒤 `Ready`와 complete scene 회복. |
| 20 | native app route, overflow display, 환경 복원이 빠졌는가 | Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 `layout=ready`, draw/present 및 두 text records. pass02 OS PNG pixel bounds에서 마지막 item이 Grid 아래에 있고 기존 설치 APK·iOS executable hash 복원이 일치. |

근거는 [실행 README](./README.md), [120-scene 사전 비교](../c11-root-html-auto-row-calc-gap-precomparison-2026-11-07/README.md), [focused Rust logs](./verification/) 및 [simulator pass01/pass02](./)다. 이 검토는 계획 review 20개나 최종 PR diff review 30개를 대체하지 않는다.
