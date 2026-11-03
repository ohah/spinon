# C11.1 `grid-auto-rows: calc(px + %)` PR 변경분 적대 검토 30개

계획 review 및 구현/runtime review와 별개로 현재 구현 diff와 시뮬레이터 evidence를 세 방향에서 대조했다. 각 항목은 별개의 변경 위험을 대상으로 한다. 기준 diff는 branch `feat/c11-root-html-grid-auto-row-calc`의 `657a703993892d0b83a403e56b110d719c84d022` 이후 변경이다.

| # | 변경 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | C11 부모 완료 checkbox나 공식 완료 집계가 제한 구현만으로 승격되는가 | 그런 변경 없음. `72/213`은 유지된다. |
| 2 | 새 동작이 Root HTML default profile 밖에서도 켜지는가 | profile gate가 `RuntimeRootHtmlTextPaintV1`에 한정된다. |
| 3 | cascade 옵션 구조체에서 해당 flag가 누락되거나 다른 Grid longhand에 적용되는가 | auto-row parsing options에만 전달된다. |
| 4 | 일반 `grid-auto-rows` scalar가 calc rule을 우회하는가 | direct px scalar/list 경로와 calc track list 검증이 분리되어 있다. |
| 5 | 값 AST가 sum이 아닌 경우 허용되는가 | 허용되지 않으며 helper가 최상위 `Sum`을 요구한다. |
| 6 | `Sum`에 2개보다 많은 항이 있을 때 일부만 취하는가 | 항 2개가 아니면 projection/layout 모두 fail-closed다. |
| 7 | px 항이 중복되고 percentage 항이 빠진 경우 통과하는가 | 각 차원 하나씩을 강제해 거부한다. |
| 8 | 두 percentage 항이 허용되거나 길이·비율의 authored 순서가 뒤섞이는가 | 동일 차원 중복은 거부하고 두 허용 단위의 authored tree를 보존한다. |
| 9 | 유한 signed percentage/px 값이 CSS math non-negative range를 벗어나 처리되는가 | `GridTemplateRows` range 처리가 negative result를 0으로 clamp한다. |
| 10 | percentage basis가 viewport width 또는 physical pixel scale로 바뀌는가 | CSS content height 100px 기준이며 두 Simulator의 DPR 차이에도 CSS frame 기대값이 동일하다. |
| 11 | `min-height`, `max-height`, `aspect-ratio`가 basis를 바꾸어도 조용히 계속되는가 | 기존 Root HTML unsupported style 경계에서 거부한다. |
| 12 | definite zero height를 missing/indefinite로 처리하는가 | zero-basis runtime/layout tests에서 `calc(10% + 5px) = 5px`을 확인한다. |
| 13 | row-gap이 percentage basis에 포함되어 계산값을 올리는가 | basis와 별도이며 두 gap 조건 frame assertion이 이를 구분한다. |
| 14 | 100% 초과 결과가 무심코 clamp되어 overflow가 사라지는가 | 121px used row와 후속 item y=127px이 유지된다. |
| 15 | calc list가 explicit row 개수부터가 아니라 첫 explicit row에서 순환하는가 | frame oracle이 explicit `[10,20]` 뒤 implicit pattern의 첫 12px row를 확인한다. |
| 16 | 기존 direct `%`, px 및 `fr` row list가 새 분기에서 회귀하는가 | 이전 implicit-row suite 32/32와 root profile suite 14/14가 통과했다. |
| 17 | calc와 함께 있는 `1fr` track을 전체 list에서 제거하거나 오분배하는가 | separate calc+`1fr` layout/runtime case가 remaining-space 결과를 확인한다. |
| 18 | hidden/visible placement 및 단일 Root HTML Grid 경계가 넓어지는가 | placement/profile 코드는 이를 변경하지 않으며 candidate fixture는 1개 Grid와 visible direct empty items를 사용한다. |
| 19 | 명명된 area, implicit columns, dense/column flow가 함께 통과하는가 | 해당 mode들은 이 patch의 profile gate/validator에 추가되지 않는다. |
| 20 | auto-row list cap 또는 expanded row 수가 `GRID_MAX_TRACKS`를 우회하는가 | projection과 layout의 기존 track cap 검사를 유지한다. |
| 21 | 누적 extent에서 gap 개수 계산이 0/1 track에서 underflow하는가 | `saturating_sub(1)`을 사용하며 empty/expanded track 처리와 함께 검사한다. |
| 22 | percentage product 또는 여러 row/gap 누적이 f32 범위 밖에서 frame으로 공개되는가 | f64 계산 뒤 각 track 및 aggregate extent에 f32 상한을 검사한다. |
| 23 | calc ID가 다른 node 또는 다른 CSS property의 값으로 재결합되는가 | `GridTemplateRows` 및 owner node binding 검사가 layout helper에 있다. |
| 24 | 중복/missing math ID가 map 삽입 순서에 따라 임의 값으로 덮이는가 | 중복 index 생성은 오류, 누락 ID는 property-bound 오류다. |
| 25 | projection 오류 후 일부 frame 또는 이전 성공 scene이 섞여 나오는가 | runtime unsupported calc case는 partial scene 부재를 확인하고 후속 유효 입력 복구를 확인한다. |
| 26 | syntax-invalid CSS와 syntax-valid unsupported CSS가 같은 fallback으로 취급되는가 | 앞선 유효 declaration fallback과 valid `min()` fail-closed를 각각 확인한다. |
| 27 | root tree/style/layout revision이 다른 snapshot에서 합쳐지는가 | runtime revision identity assertion이 layout과 computed style 동일 revision을 검사한다. |
| 28 | Android/Swift 앱 라벨이 현재 row-calc fixture와 불일치하는가 | 둘 다 Root HTML Grid tracks/calc rows를 명시하며 기존 route 이름을 columns 전용으로 오인시키지 않는다. |
| 29 | simulator 캡처를 PixelCopy/Metal readback 또는 pixel exact로 과장하는가 | README는 draw/present와 시각 확인만 기록하고 두 readback 및 compositor 증거가 없음을 명시한다. |
| 30 | fixture payload가 default FFI artifact 또는 새 exported ABI에 들어가는가 | 기본 archive에서 기존 opt-in symbol과 calc payload가 빠지고 positive-control feature archive에서 기존 symbol/payload가 들어간다. 신규 FFI symbol은 없다. |

30개 변경 실패 관점에서 수정이 필요한 결함을 찾지 못했다. Android/iOS build와 focused Clippy·formatter·diff check는 실행 근거에 기록했다. Simulator 결과는 해당 AVD 및 iOS Simulator에만 한정되고 GitHub CI 결과로 확대하지 않는다.
