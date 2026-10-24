# C11.4 fixed-repeat + auto-repeat 수정 계획 검토 · 실패 관점 20개

**검토 기준:** [계획](./c11-4-auto-repeat-fixed-repeat.md)의 Stylo computed-AST 경계 수정, CSS Grid 2025 CRD, 9-scene Chrome 156 fixture, [Stylo AST 관찰](../spec/internal/evidence/c11-4-auto-repeat-fixed-repeat-precomparison-2026-10-24/stylo-computed-ast-observation.json). 이 검토는 수정된 계획만 판정하며 코드 구현/runtime 결과를 주장하지 않는다.

| # | 독립 실패 관점 | 검토 결과와 계획상 통제 |
| --- | --- | --- |
| 1 | `calc(20px)` authored form이 cascade 뒤에는 literal `20px`와 구별되지 않을 수 있음 | 실제 Stylo AST와 직렬화 관찰을 고정했다. 수용 기준을 authored string이 아닌 computed `LengthPx`로 정의했다. |
| 2 | `pt`·`cm` 같은 absolute unit도 CSS px로 정규화되어 원 범위보다 넓게 들어올 수 있음 | computed type 기준으로 finite nonnegative absolute length를 허용하고 source unit 제한을 주장하지 않는다. |
| 3 | percentage가 섞인 calc를 constant absolute calc와 같게 취급할 수 있음 | Percentage나 `Calc`로 남는 fixed-repeat value를 거부한다. |
| 4 | 계산식이 현재 Stylo 버전에서 다르게 simplify될 수 있음 | 고정 `calc(20px)` scene 하나를 Chrome oracle에 두고 구현 전 AST를 다시 확인한다. 다른 수식으로 일반화하지 않는다. |
| 5 | negative `LengthPx`가 nonnegative 검사 없이 repeat track에 들어갈 수 있음 | computed value finite성과 `>= 0`을 adapter 경계에서 모두 검사한다. |
| 6 | NaN 또는 infinite float가 `LengthPx`로 들어올 수 있음 | `is_finite()`를 요구하고 실패는 node/property 오류로 반환한다. |
| 7 | CSS parser가 잘못된 repeat declaration을 버려 adapter 테스트가 실행되지 않을 수 있음 | parser diagnostic과 typed adapter error를 분리해 입력별 기대 결과를 기록한다. |
| 8 | computed list에서 fixed group과 auto-repeat 순서를 잘못 읽을 수 있음 | `RepeatCount::Number`와 `AutoFill`이 같은 list에 보존된 AST index를 고정하고 prefix/suffix 모두 검증한다. |
| 9 | auto-repeat 뒤 suffix fixed group이 앞에 놓일 수 있음 | suffix used-track order와 auto-repeat `start/end`를 Chrome·cascade 기대값에 포함한다. |
| 10 | 두 번째 fixed group을 첫 group에 합쳐 지원 범위를 넓힐 수 있음 | 한 mixed list의 fixed repeat은 최대 하나만 허용하고 두 번째 group은 거부한다. |
| 11 | fixed 또는 auto repeat 내부 line name을 버리고 계속 진행할 수 있음 | 혼합 profile에서는 모든 outer/inner 이름 그룹이 비어 있어야 한다. 이름 merge는 범위 밖으로 둔다. |
| 12 | fixed repeat 확장 뒤 line-name outer group 개수가 기존 authored component 수에 머물 수 있음 | 확장 track count와 auto fragment 크기에서 component `N+1` 그룹 수를 checked arithmetic으로 산정한다. |
| 13 | line group vector metadata가 track cap과 별도로 과다해질 수 있음 | expanded outer와 repeated inner metadata byte budget을 기존 한도 안에서 검사한다. |
| 14 | repeat count와 1~2 breadth pattern 곱이 10,000 descriptor 한도를 넘을 수 있음 | checked multiply/add 후 auto pattern과 이웃 track까지 더해 기존 10,000 cap을 적용한다. |
| 15 | fixed count multiplication 또는 group count arithmetic이 overflow할 수 있음 | `checked_mul`, `checked_add`, checked subtraction 실패를 부분 결과 없이 거부한다. |
| 16 | fixed track reserve가 실패한 뒤 일부 cascade style을 반환할 수 있음 | track vector 예약을 선행하고 contextual allocation error를 반환하며 reservation failpoint로 복구를 검사한다. |
| 17 | expanded line-group reserve 실패가 부분 metadata를 반환할 수 있음 | line-group failpoint와 반복 후 정상 재시도에서 오류 context 및 no-partial-output을 확인한다. |
| 18 | columns와 rows의 두 mixed axis를 독립 검사하면서 범위 밖 조합을 실수로 허용할 수 있음 | mixed candidate가 한 축에만 있도록 adapter gate와 거부 case를 둔다. |
| 19 | fixed group 지원 변경이 기존 auto-fit count/collapse 순서를 바꿀 수 있음 | 반복 수는 placement 전에 계산하고 기존 auto-repeat fragment와 neighbor validator를 수정하지 않는다. 기존 8개 case의 expected tracks를 보존하고 새 calc scene도 literal px control과 같은 결과인지 확인한다. |
| 20 | 계획 수정이 공식 지원 또는 완료 수 증가로 읽힐 수 있음 | STATUS·0059·roadmap에 계획/oracle 단계와 `72/213`, C11.4 미완료를 반복하고 내부 계약 `0.1.0`을 유지한다. |

## 결론

수정된 경계는 raw authored CSS를 추정하지 않고 Stylo가 실제 전달하는 computed AST만 사용한다. absolute 값이 computed `LengthPx`로 남으면 수용하며, percentage·unresolved calc·intrinsic·flex는 계속 거부한다. 별도 Chrome calc scene, parser/adaptor 분리, 이름·resource·axis 제한이 이 선택을 검증 가능하게 만든다. 계획 수정은 통과시킬 수 있으나 기능 코드와 simulator는 아직 검사하지 않았다.
