# C11.4 zero-length percentage `calc()` auto-repeat 계획 실패 관점 20개

구현 전에 규범 판정, Chrome 비교 입력과 코드 경계를 서로 다른 실패 경로로 검토했다. 구현/runtime 검토나 PR diff 검토에 재사용하지 않는다.

| # | 실패 관점 | 계획의 대응 |
|---:|---|---|
| 1 | CSS 문법에서 zero length 항이 invalid라고 잘못 간주한다. | 2025 CRD §7.2.1의 nonnegative length-percentage를 기준으로 삼고, zero length + positive percentage 입력만 고정한다. |
| 2 | `calc(0px + 10%)`을 zero minimum으로 계산한다. | definite 120px 축에서 `0 + 0.1×120 = 12px`를 독립 계산 JSON에 고정한다. |
| 3 | columns percentage에 height나 viewport를 사용한다. | columns content inline size를 120px로 고정하고, 해당 축의 12px control과 비교한다. |
| 4 | rows percentage에 width나 viewport를 사용한다. | rows content block size를 120px로 고정하고, 해당 축의 12px control을 별도로 비교한다. |
| 5 | border-box와 content-box를 혼동한다. | grid content box를 120px, padding과 border를 0으로 고정한다. |
| 6 | definite percentage를 cyclic/indefinite basis로 확장한다. | 이번 slice는 양수 definite axis만 입력하고 zero/indefinite/cyclic basis를 명시적으로 제외한다. |
| 7 | auto-repeat이 minimum 대신 1fr maximum으로 반복 수를 계산한다. | 반복 수 판정과 flexible leftover 분배를 분리해 `floor(120/12)=10`을 고정한다. |
| 8 | 9개 반복이나 11개 반복의 off-by-one을 놓친다. | 여덟 Chrome scene의 expanded track count가 정확히 10임을 검사한다. |
| 9 | percentage 항을 버려도 control과 우연히 맞는다. | 120px 축의 12px control과 auto-fill/auto-fit 결과를 전부 대조한다. |
| 10 | `auto-fill`을 `auto-fit`처럼 빈 track collapse 한다. | 단일 item에서 auto-fill `[12]×10`과 auto-fit `[120,0×9]`를 독립 acceptance 값으로 둔다. |
| 11 | auto-fit collapse가 item placement보다 먼저 일어난다. | 한 in-flow item을 둬 첫 반복만 점유시키고, 뒤 9개가 placement 후 접히는지 검사한다. |
| 12 | rows fixture의 auto-placement가 결과를 가린다. | row case는 한 item과 column auto-flow만 사용하며 모든 반복 track을 관찰한다. |
| 13 | subpixel 반올림이 표준 결과와 실패를 모호하게 만든다. | 120의 10%인 정수 12px를 택해 분수 track rounding을 배제한다. |
| 14 | 0%와 0px minimum을 새 지원 범위에 섞는다. | `calc(0px + 0%)`는 fail-closed regression으로 계속 거부한다. |
| 15 | 음수 percentage/계산 연산을 함께 허용한다. | `calc(0px - 10%)`, 추가 항과 `max()`는 negative controls로 유지한다. |
| 16 | Stylo가 calc AST를 다른 형태로 정규화할 수 있다. | cascade 시험은 고정 문자열뿐 아니라 실제 computed typed value도 검사하고, 관측한 정규화 형태에 맞춘다. |
| 17 | 새 수용 경로가 일반 CSS profile에 노출된다. | 변경은 C11.4 opt-in `RuntimeBlockFlexGridTrackSizingV1`에 한정하고 default product archive를 검사한다. |
| 18 | Taffy의 zero-size repeat division이 반복 상한까지 track을 만든다. | explicit positive fixed axis와 zero padding/border만 허용하고, 그 외는 Taffy 전에 typed diagnostic으로 거부한다. |
| 19 | fixture가 실제 앱 host 경로를 우회한다. | V8-authored HTML → HostDocument → Stylo → typed input → Taffy → WGPU frame 경로로 AVD/iOS Simulator를 검증한다. |
| 20 | simulator 화면이나 Chrome 관측을 제품 완료/WPT로 과장한다. | 화면은 frame 보조 근거로 한정하고 C11.4 체크와 공식 `72/213`을 그대로 둔다. |

`calc(0px + 10%)`은 Chrome 156에서 calc 대조군의 track과 item frame이 모두 일치했다. 계획된 구현은 확인 전이며, 위의 zero-basis preflight는 알고리즘에서 0으로 나누는 경로를 막기 위한 별도 조건이다.
