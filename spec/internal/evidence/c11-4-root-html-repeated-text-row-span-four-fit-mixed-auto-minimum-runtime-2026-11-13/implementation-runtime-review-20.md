# Four-fit mixed minimum 구현·런타임 실패 관점 검토 20개

검토 대상은 current candidate source와 최종 RuntimeSession mixed matrix다. 각 항목은 다른 failure mode를 본다. Oracle의 전체 138,240 scene과 RuntimeSession 표본 3,840 scene은 별도로 계산했다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 승인 조건이 다른 fit count까지 넓어짐 | Style/layout admission은 정확히 4개의 `fit-content()`와 six-member list만 받는다. Mixed runtime gate도 `fit_content_count == 4`와 7개 row 상태를 함께 요구한다. 다른 fit count는 새 profile로 들어오지 않는다. |
| 2 | 허용 cap multiset에 중복·누락·근사값이 들어옴 | 두 admission 계층은 `0px`, `10px`, `20%`, `30px`을 각각 한 번 요구한다. Runtime sample은 15 position set과 20개 cap order를 확인하고 기존 uniform 11,520 matrix가 720 list order를 덮는다. |
| 3 | Style projection과 layout validator가 서로 다른 track list를 받음 | 두 계층의 typed-cap 검사, member count와 tail 수가 일치한다. 기존 uniform endpoint와 candidate 3,840 matrix가 동일 list를 양쪽 경로에 통과시킨다. |
| 4 | row-span 대신 auto-placement topology가 이 profile에 유입됨 | `repeated_text_span_profile`과 topology/implicit-row 검사를 거친 뒤 four-fit mask profile을 계산한다. 이 변경은 nine-item auto-placement matcher를 수정하지 않는다. four-fit auto-placement 5,760 회귀가 통과했다. |
| 5 | explicit-row control이 mixed mask로 자동 전환됨 | Style과 layout 모두 첫 explicit-row item을 `0px`로 고정한다. 같은 RuntimeSession negative control에서 control을 `auto`로 바꾸자 실패했고 partial snapshot이 없었다. |
| 6 | item mask가 모든 span item에 동일하게 전파됨 | 허용 predicate는 각 후속 item을 독립적으로 `0px|auto` 검사한다. Runtime matrix는 topology별 32/16개 mask 전체를 forward/reverse 및 start/stretch와 교차한다. |
| 7 | `min-height`의 다른 computed 값이 0/auto로 잘못 정규화됨 | Style gate는 정확한 `LengthPx(0.0)` 또는 `Auto`만 허용한다. Runtime rejection은 computed `1px`를 확인한 뒤 fail-closed와 same-session recovery를 통과했다. |
| 8 | 첫 mask 비트가 빈 control 대신 실제 text item에 적용됨 | Fixture item order와 `items.first()` control은 같은 순서이며 control은 별도 고정 조건을 받는다. Positive tests는 semantic item ID를 사용해 Oracle placement와 item frame을 비교한다. |
| 9 | non-mixed all-zero endpoint가 새 replay 경로에 잘못 들어감 | Four-fit replay 분기는 실제 span auto minimum과 zero/auto 혼합이 함께 있을 때만 켜진다. Uniform four-fit endpoint 11,520/11,520이 통과했다. |
| 10 | non-mixed all-auto endpoint의 automatic floor가 바뀜 | 기존 all-auto 경로와 final rows 비교를 유지한다. 720 list 각각의 two-endpoint 비교에서 geometry 차이가 `0.01 CSS px` 허용 안에 들었다. |
| 11 | automatic minimum eligibility가 track 함수와 맞지 않음 | Fixture에는 scroll/overflow/flexible-span 변형이 없다. Runtime test는 item의 span track 중 `auto|fit-content()`만 eligible floor 대상으로 계산하고 Auto mask의 shortfall이 0임을 확인한다. |
| 12 | 6 implicit-row 경계를 벗어난 trace를 replay함 | Simulator는 한 explicit row, 6개 auto-row member, 7 track state 및 단일 Grid trace를 검증한다. sample의 모든 row values와 used item/payload frame이 Chrome 값과 비교된다. |
| 13 | spanning contribution 단계 누락 또는 flexible span 수용 | Replay는 trace의 span batch와 contribution phase를 대조하고 flexible-track 교차를 거부한다. 현재 fixed profile의 전체 matrix가 각 track-list context에서 통과했다. |
| 14 | growth limit을 넘어 contribution을 적용함 | 각 intrinsic batch 뒤 track state를 Taffy trace base size와 비교하고 growth limit과 base 관계를 확인한다. mismatch면 replay를 수용하지 않는다. |
| 15 | mixed profile의 1/64px residual total이 소실됨 | `restore_missing_residual_units`는 rounded total 목표와 fallback allocation total을 검사하고 부족한 residual만 복원한다. 최종 3,840 matrix에서 row, item, payload geometry가 통과했다. |
| 16 | residual fallback이 행별 trace 분배를 바꿈 | 복원량은 fallback과 simulated 값 차이 이내로 제한하고, row vector 길이/총합 불일치면 실패한다. 복원 후 최종 track trace 대조가 남아 있어 임의 결과를 수용하지 않는다. |
| 17 | `align-content:stretch`가 non-auto row를 바꿈 | Pair test는 start/stretch의 같은 scene을 묶고 singleton `auto` row만 증가 허용하며 다른 row는 `0.01 CSS px` 안에 유지한다. 전체 20 track-list batch가 통과했다. |
| 18 | stretched auto tail이 row total 또는 final trace에서 벗어남 | 혼합 span residual 비교는 stretched tail 외 행의 trace delta와 전체 `1/64px` Grid 높이 합을 함께 검사한다. 해당 rounding boundary test와 3,840 matrix가 통과했다. |
| 19 | source-order 반전에서 placement/paint order가 바뀜 | 각 track list에서 topology forward/reverse를 pair로 묶어 row, semantic item frame, payload frame을 모두 비교한다. 96개 order pair per list가 matrix에서 통과했다. |
| 20 | rejection 뒤 stale/partial layout이 후속 valid revision을 막음 | Control-auto와 item `1px` negative case는 완료 snapshot을 내지 않는다. 같은 RuntimeSession에서 valid mask로 복구했고 기존 uniform, three-/five-/six-fit 및 auto-placement regression도 유지했다. |

## 실행 근거

- 최종 candidate mixed RuntimeSession: [runtime-mixed.log](./runtime-mixed.log), 6 passed, 0 failed, 3,840 scenes, 340.41초.
- Uniform four-fit endpoint: 11,520/11,520 passed; four-fit auto-placement: 5,760/5,760 passed; focused trace/control/rejection tests: 9 passed.
- iOS와 Android 기본 route startup 범위, Android headless light/dark 통과, fresh cold-boot 및 GUI QEMU 실패 원인은 [runtime README](./README.md)와 [crash 분석](./android/crash-analysis.md)에 분리 기록했다.

이 검토는 전용 fixture와 고정 candidate에 대한 내부 실패 관점 검토다. 전체 CSS Grid, WPT 적합성, mixed fixture의 Android/iOS runtime 실행, 실기기·hardware GPU 또는 C11.4/C11 완료를 뜻하지 않는다.
