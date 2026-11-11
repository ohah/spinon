# 구현·런타임 실패 관점 20개

검토 대상은 최종 six-fit row-span 구현과 RuntimeSession 동작이다. 계획 문서의 계획 검토와 별개다. 각 항목은 고유한 입력·불변 조건을 대상으로 한다.

| # | 실패 관점 | 확인 근거와 결과 |
|---:|---|---|
| 1 | 6개 위치의 cap 순서가 바뀌면 style projection이 일부 순열을 거부하는가 | Style unit test가 typed cap 720순열 전부를 수용했다. 통과. |
| 2 | 같은 authored cap order를 layout admission이 projection 뒤에서 다시 거부하는가 | Layout unit test가 720순열과 positive implicit row 6을 수용하고 row count 3은 거부했다. 통과. |
| 3 | track list가 5개 또는 7개 member여도 six-fit profile로 들어오는가 | Style·layout profile test와 RuntimeSession의 seven-member rejection에서 거부했다. 통과. |
| 4 | cap multiset에서 항목이 빠지거나 중복 cap이 다른 cap을 대신하는가 | 중복·누락, 알 수 없는 px/percentage, 일반 Auto의 typed profile rejection을 확인했다. 통과. |
| 5 | 음수·NaN·무한대 cap이 finite profile로 허용되는가 | Layout·style profile rejection tests가 negative/non-finite 입력을 거부했다. 통과. |
| 6 | unresolved `calc()`가 percentage basis 없이 시뮬레이터에 진입하는가 | typed Calc rejection과 parsed Root HTML atomic rejection에서 거부했다. 통과. |
| 7 | 20% cap이 잘못된 basis를 사용하거나 definite basis에서 44px로 계산되지 않는가 | RuntimeSession trace가 typed `20%`, definite `220px` basis, 최종 `44px` growth limit을 확인했다. 통과. |
| 8 | 7 used rows에서 explicit control row를 implicit row로 잘못 세는가 | Trace는 explicit 1, negative implicit 0, positive implicit 6, total 7을 확인했다. 통과. |
| 9 | span automatic minimum의 적용 대상 또는 contribution source가 잘못 분류되는가 | Trace에서 적격 text item 5개의 AutomaticMinimum source, span 수, no-flex 조건과 payload line floor를 확인했다. 통과. |
| 10 | 모든 scene에서 `min-height:auto`/`0` 외에 mixed minimum을 허용하는가 | dark media가 한 item만 `0`으로 바꾸는 입력은 partial layout 없이 거부했고 같은 session의 valid light revision은 회복했다. 통과. |
| 11 | span 시작/끝 경계가 바뀌거나 row-span 길이가 달라도 positive profile이 유지되는가 | 잘못된 row span, span placement와 extra implicit row를 RuntimeSession rejection에서 확인했다. 통과. |
| 12 | DOM에 direct child가 빠지거나 추가되어도 고정 topology로 잘못 해석하는가 | missing active child와 추가 row item은 atomic rejection했다. 통과. |
| 13 | row 대신 column flow 또는 dense packing이 적용돼도 simulator가 행 trace로 처리하는가 | column flow와 dense row flow 변이를 거부했다. 통과. |
| 14 | Grid owner·explicit row/column 수가 profile과 달라도 six-fit 경로로 들어오는가 | 예상 밖 explicit column·row 개수 변이를 거부했다. 통과. |
| 15 | RTL, nonzero row gap, scrollable overflow에서 고정 좌표 모델이 적용되는가 | RTL, 1px row gap, `overflow:auto` 변이를 fail-closed했다. 통과. |
| 16 | 폰트·line-height·payload가 oracle 전제와 다르지만 동일 결과로 취급되는가 | serif, 12px line-height, 비대상 텍스트 변이를 거부했다. 통과. |
| 17 | `align-content:center` 또는 indefinite Grid height가 stretch/percentage 모델에 들어오는가 | unsupported alignment와 `height:auto` percentage basis를 거부했다. 통과. |
| 18 | stage trace에서 다른 node·축·pass·gutter·track 수를 선택해 결과가 조용히 달라지는가 | simulator는 owner trace 유일성, 최신 row pass, zero gutter, 7 visible tracks를 확인하고 각 span phase 뒤 Taffy trace와 대조한다. 구현 경로를 검토했고 유효 trace 전체는 matrix/trace tests로 확인했다. 통과. |
| 19 | 1/64px residual 배분을 final f32 row 값만으로 추정해 DOM order에 따라 흔들리는가 | phase replay와 stage별 대조를 사용한다. 720 orders 전체 matrix에서 forward/reverse row·item·payload geometry가 일치했다. 통과. |
| 20 | six-fit replay 불가나 trace 불일치가 Taffy fallback으로 숨거나 기존 five-fit 결과를 깨뜨리는가 | six-fit replay 실패 시 fail-closed, five-fit fallback은 보존된다. Six-fit matrix 4/4와 five-fit non-matrix regressions 9/9 통과했다. 통과. |

주요 실행 로그는 [RuntimeSession matrix](runtime-matrix-final.log), [trace](runtime-trace-tests.log), [atomic rejection](runtime-rejections-tests.log), [same-session recovery](runtime-recovery-test.log), [five-fit regressions](five-fit-regressions-final.log)다. Android/iOS 기본 route smoke는 이 row-span fixture를 실행하지 않으므로 위 런타임 검토의 근거로 확대하지 않았다.
