# 반복 intrinsic implicit-row 목록 구현/runtime 실패 관점 검토 20개

검토 범위는 여섯-member exact list, typed projection, Grid sizing profile, parsed Root HTML RuntimeSession, 검증 fixture와 simulator 실행 근거다. 각 항목을 실제 변경 코드·검사와 대조했다.

| # | 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | track list 순서가 projection 중 canonicalize되거나 재배열되는가 | 통과. Typed projection 검사에서 `fit-content, min-content, max-content, auto, min-content, max-content`의 kind와 순서를 exact compare한다. |
| 2 | 중복 항목이 실제 CSS 목록과 다르게 허용되는가 | 통과. exact pattern matcher가 두 `min-content`와 두 `max-content`를 각각 요구하며 다른 중복은 거부한다. |
| 3 | auto member 위치가 다른 순서도 우연히 받아들이는가 | 통과. Layout·Style negative controls가 `auto`를 옮긴 인접 순서를 거부한다. |
| 4 | 짧거나 긴 track list가 bounded profile을 벗어나 수용되는가 | 통과. Style projection에서 5·7 member 인접 길이를 거부한다. |
| 5 | px cap에서 음수·무한대가 통과하는가 | 통과. 기존 finite non-negative cap predicate가 유지되고 negative/Infinity projection controls가 오류를 확인한다. |
| 6 | direct percentage cap의 definite basis 없이 추정되는가 | 통과. 기존 Root HTML contract의 definite 100px block-size와 직접 percentage 해석을 유지하며 CSS computed 값 그대로 projection한다. |
| 7 | zero 및 100% 초과 cap이 parser/adapter 단계에서 조용히 clamp되는가 | 통과. typed projection positive control은 0%와 120%를 원래 값으로 보존한다. |
| 8 | `calc()`·다른 track function이 이 exact matcher로 열리는가 | 통과. six-member matcher는 `FitContent`·다섯 intrinsic enum 외 kind를 허용하지 않고 기존 계산값 admission을 그대로 사용한다. |
| 9 | 네-member 선행 profile의 payload 허용 규칙이 바뀌는가 | 통과. 네-member 경로는 별도 payload 배열과 순환 위치를 계속 사용하며 fit-content overflow 허용은 해당 track position에만 적용한다. |
| 10 | singleton·기존 세-member 순서가 새 matcher 때문에 넓어지는가 | 통과. 기존 predicate들은 유지하고 새 predicate를 별도로 합쳤다. focused `fit_content` 회귀 79개가 통과했다. |
| 11 | 여덟 implicit row에서 여섯 track pattern의 cycle index가 밀리는가 | 통과. Layout은 전체 row frames를 검사하고 RuntimeSession은 두 번째 `fit-content`와 두 번째 intrinsic members의 identity/frame을 확인한다. |
| 12 | repeated `min-content`/`max-content`가 첫 occurrence의 row contribution을 공유하는가 | 통과. 두 cycle item에 서로 다른 고정 payload heights를 사용하며 각 item frame과 row height를 별도로 검사한다. |
| 13 | `auto` stretch가 intrinsic/fixed-size tracks까지 변경하는가 | 통과. `start`/`stretch` used rows를 비교해 stretch의 +4px가 네 번째 implicit `auto` track에만 들어간다. |
| 14 | fit-content cap이 second cycle에서 적용되지 않는가 | 통과. 7번째 implicit row의 60px payload에도 row는 20px이고 payload만 grid 바깥까지 넘친다. |
| 15 | 60px overflow payload가 첫 fit-content 위치에서도 허용되는가 | 통과. Layout negative control에서 첫 payload를 60px로 바꾼 입력을 거부한다. |
| 16 | 60px overflow 예외가 다른 row position이나 다른 row profile까지 번지는가 | 통과. six-member profile은 position별 exact height만 허용하며 일반 60px 허용 예외를 추가하지 않았다. |
| 17 | 여덟 item topology가 일곱/아홉 implicit item으로 확장되는가 | 통과. exact 9 visible direct child (명시 row item 1 + implicit item 8)를 layout/style validator와 negative tests에서 강제한다. |
| 18 | item `min-height:0` 제한이 이전 profile 전체에 적용되는가 | 통과. `grid.rs` allowlist는 9-child, positions 1..=8, fixed zero dimension, exact six-member profile 조합으로 제한한다. |
| 19 | style rejection이 partial render state를 publish하거나 session 복구를 막는가 | 통과. dark-scheme alternate order를 property/node 귀속으로 거부하고 partial completion이 없으며 같은 RuntimeSession에서 valid light revision으로 복구한다. |
| 20 | simulator 결과가 fixture hash·실행 artifact·제품 수준 surface parity를 과장하는가 | 통과. 두 기본 app route의 fixture hash를 고정하고 `layout=ready boxes=20` 및 Android draw-presented를 기록했다. screenshots는 visual smoke로만 분류하며 iOS data restore hash를 대조했다. |

### 발견 및 수정

- 첫 validator pass는 아홉 child positions에 필요한 `min-height:0` allowlist를 빠뜨렸다. exact profile guard 아래 positions 1..=8을 추가하고 positive/negative layout 검사를 재실행했다.
- payload validator가 60px height를 모든 fit-content 순환 위치에 허용했다. 두 번째 cycle의 intended position만 받도록 height vector를 exact로 제한하고 첫 cycle의 negative control을 추가했다.
- runtime fixture의 `border:0` shorthand는 현 style collector에서 처리하지 못해 제거했다. explicit item에도 `.item` class를 재사용해 `min-height:0`가 적용되던 것을 분리하고, payload `width:40px`를 nested-payload contract에 맞게 `width:auto`로 수정했다.
- 첫 simulator capture runner가 18 boxes를 예상했지만 실제 root/body를 포함한 layout report는 20 boxes였다. 실행기 기대 count를 20으로 고쳐 두 simulator에서 새 pass를 확인했다.

PR diff의 문서·evidence·통합 변경 검토는 이 표와 중복 계산하지 않고 별도 검토 파일에 기록한다.
