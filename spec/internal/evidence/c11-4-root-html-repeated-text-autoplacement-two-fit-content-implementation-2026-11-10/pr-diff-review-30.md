# C11.4 두 fit-content 자동 배치 실제 PR diff 실패 관점 검토

이 표는 후보 브랜치의 실제 코드·fixture·근거 문서 diff를 대상으로 한다. 계획 PR diff 검토 30개와 구현/runtime 검토 20개와는 별도로 수행했다.

| # | 변경 경계에서 확인한 실패 경로 | 판정 |
|---:|---|---|
| 1 | 기존 한-fit helper의 조건을 바꿔 기존 callers까지 무심코 확대 | 기존 single-fit predicate를 보존하고 auto-placement 전용 union을 추가했다. |
| 2 | span predicate에 새 list multiplicity를 넣어 auto-placement 지원이 row-span에 새어 나감 | span list 함수는 그대로 두고 별도 profile로 분리했다. |
| 3 | 두 fit track 중 cap 하나가 직접 px/%가 아님 | layout/style 양쪽 predicate가 지원 cap predicate를 각각 요구한다. |
| 4 | negative·NaN·infinity cap | typed cap 검증 및 별도 rejection controls에서 차단했다. |
| 5 | calc() cap | root style parser 이후 허용 predicate가 거부하며 runtime negative로 고정했다. |
| 6 | 목록 길이가 6이 아님 | 두 code boundary 모두 exact length 6을 요구한다. |
| 7 | min-content multiplicity가 2가 아님 | count vector에서 정확히 2를 요구한다. |
| 8 | max-content multiplicity가 1이 아님 | count vector에서 정확히 1을 요구한다. |
| 9 | auto multiplicity가 1이 아님 | count vector에서 정확히 1을 요구한다. |
| 10 | 나머지 track 종류가 허용 목록 밖임 | layout/style matcher의 wildcard 분기는 false로 닫힌다. |
| 11 | authored fit 위치 15쌍 중 일부가 누락 | 두 fit 개수 predicate는 위치 allowlist 없이 임의의 서로 다른 두 list slot을 허용하고 matrix가 15쌍을 확인한다. |
| 12 | 후행 member 12 순서 중 일부가 누락 | 순서별 허용목록 대신 정확한 multiplicity를 사용하고 layout test가 12개 고유 permutation을 열거한다. |
| 13 | typed px/% 차이를 출력 또는 projection에서 잃음 | computed typed track와 serialized CSS property/order를 각각 검사한다. |
| 14 | fit track list가 intrinsic auto-row branch 외부에서 보편 허용 | 새 helper는 기존 RootHtmlFitContentAutoRowsV1 검증 내에서만 사용한다. |
| 15 | trace enable 조건이 새 auto-placement profile을 놓침 | projection에서 전용 six-member auto-placement predicate를 사용한다. |
| 16 | normal fixed-list CSS layout에 two-fit 허용이 전파됨 | 변경은 제한 parsed Root HTML route predicates에 한정된다. |
| 17 | item-count 또는 text shape 완화 | existing nine-child repeated-text contract를 유지한다. |
| 18 | span topology와 auto-placement topology가 같은 boolean으로 합쳐짐 | repeated_text_profile과 repeated_text_span_profile을 계속 분리한다. |
| 19 | two-fit list만 보고 span min-height:auto를 허용 | auto minimum은 two-fit span profile과 span topology가 함께 참일 때만 허용한다. |
| 20 | 기존 three-item/ five-item profile이 새 six-member gate를 피해 실패 | 이전 four-member와 six-member span/repeated profile 조건을 유지하고 과거 회귀가 통과했다. |
| 21 | explicit row가 auto-row list 순환을 이동시킴 | fixture에는 10px explicit row를 유지하고 Chrome/runtime matrix에서 이후 8행을 비교했다. |
| 22 | item frame은 맞지만 text payload geometry 또는 line count가 틀림 | payload frame, glyph 수, line 수를 별도 비교한다. |
| 23 | visible overflow가 clipping으로 변함 | matrix의 payload bottom을 Grid 220px 경계와 비교한다. |
| 24 | percentage cap가 잘못된 block basis에서 resolve됨 | fixture definite height 220px를 유지하고 20%/120%와 zero controls를 대조했다. |
| 25 | align-content stretch가 non-auto row에 분배됨 | track trace helper의 per-row validation과 1,440 start/stretch pair가 이를 막는다. |
| 26 | auto placement 입력 오류 뒤 이전 layout/render 결과가 남음 | rejected RuntimeSession 결과에 completed output 부재를 확인한다. |
| 27 | invalid environment 뒤 같은 session을 복구하지 못함 | dark invalid-list rejection 뒤 light scheme recovery와 frame equality를 검사한다. |
| 28 | 새 fixture가 Android/iOS app asset 원본에 남아 다음 테스트를 오염 | 두 app asset은 실행 후 baseline SHA-256로 복원됐고 cleanup-state.txt에 기록했다. |
| 29 | 기본 앱 실행이 잘못된 source나 오래된 binary를 검사 | 각 화면 run은 root HTML source hash, layout-ready 및 draw-present 로그를 확인하고 PNG를 보존했다. |
| 30 | 검증 중 생성한 simulator/temp files가 shared profile이나 다음 작업에 남음 | 이번 iOS profile과 Android temp data는 제거됐고 공유 AVD metadata/overlay stat은 전후 동일하다. |

검토 결과 변경 범위 안에서 차단해야 할 미해결 결함은 발견하지 못했다. Runtime matrix, 새 trace-failure unit tests, Android/iOS sentinel logs와 남은 범위는 [실행 근거](./README.md)에 있다. 이 판정은 merge 후 CI 또는 일반 CSS 지원 완료를 의미하지 않는다.
