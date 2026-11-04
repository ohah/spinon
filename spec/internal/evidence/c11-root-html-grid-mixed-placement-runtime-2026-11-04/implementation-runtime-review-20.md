# C11.1 mixed explicit/one-axis placement 구현·runtime 실패 관점 검토 20개

검토 대상은 변경된 layout selector/normalizer, typed computed-style admission, Android PixelCopy/iOS Metal fixture adapter 및 최종 19-sample simulator 실행이다. 계획 검토표와 분리했다. 이 문서는 구현 PR diff의 별도 검토를 대체하지 않는다.

| # | 실패 관점 | 확인 및 판정 |
|---:|---|---|
| 1 | 새 route가 Root HTML 이외의 profile로 새어 나가는가 | Selector 호출은 `RuntimeRootHtmlTextPaintV1`에서만 구성되고 기본 placement mode는 그대로다. 통과. |
| 2 | 서로 다른 Grid owner의 partial axis 상태가 공유되는가 | computed selector는 item을 owner별 map으로 묶고 layout validator도 container마다 축을 초기화한다. 한 runtime 문서에서 row-locked·column-locked owner가 각자 통과했다. 통과. |
| 3 | all-auto owner가 mixed mode로 잘못 분류되는가 | mixed selector는 fully definite와 one-axis partial을 각각 하나 이상 요구한다. 기존 all-auto runtime control은 old route에서 2/2 sample exact다. 통과. |
| 4 | fully definite 전용 owner가 새 mode로 잘못 분류되는가 | partial item 수가 0이면 mixed selector/validator가 거부하고 기존 explicit route control을 유지한다. 2/2 sample exact. 통과. |
| 5 | row partial과 column partial이 한 owner에서 섞여도 선택되는가 | selector가 한 owner의 partial axis를 고정하고 반대 축을 거부한다. layout·style integration negative test 통과. 통과. |
| 6 | named line/span, area, repeat, dense/column flow가 새 mode에 들어오는가 | selector의 grammar/flow guard와 layout track validator에서 거부된다. typed named placement 및 기존 unsupported test 통과. 통과. |
| 7 | `display:none`과 `visibility:hidden`을 같은 방식으로 제외하는가 | display-none child는 candidate/occupancy에서 제외되고 visibility-hidden full item은 occupancy를 유지한다. Rust integration 및 PixelCopy/Metal sample로 확인. 통과. |
| 8 | 늦은 source-order fully definite item이 앞선 partial placement보다 늦게 예약되는가 | 첫 18-sample fixture에서 blocker가 같은 행에 없다는 결함을 찾았다. blocker를 row 1/column 2로 수정하고 이후 partial item의 column 3 색 표본을 추가했다. 최종 Android/iOS 각각 19/19 exact. 기존 pass18 입력·결과는 비최종 이력으로 보존. 통과. |
| 9 | definite blockers 예약 뒤 DOM source order가 바뀌는가 | sparse normalizer의 stable `(order, source_order)` key와 row/column 색 표본을 확인했다. 배치 위치는 예약에 따라 달라지고 paint/source ownership은 유지된다. 통과. |
| 10 | positive span이 blocker와 겹치거나 부분만 겹칠 때 contiguous free space를 잘못 고르는가 | row span 및 column span 장면의 독립 Chrome 기대값, Rust frame tests, 두 simulator의 blocker·partial 표본이 일치한다. 통과. |
| 11 | explicit row를 넘는 placement가 implicit row를 조용히 publish하는가 | sparse normalizer가 expanded rows를 반환하면 placement output에 합치기 전에 거부한다. focused mixed implicit-row test 통과. 통과. |
| 12 | explicit column 바깥 fully definite item이 implicit column을 만드는가 | `resolve_axis`의 explicit line bound에 의존하는 mixed-mode 전용 layout·Root HTML integration negative tests를 추가했다. 두 테스트 통과. 통과. |
| 13 | 빈 track list, 비유한/음수 크기, track cap 초과가 누락되는가 | container validator는 finite non-negative px track과 non-empty bounded list를 검사하고 auto-track도 한정한다. 기존 one-axis validator 경계를 재사용. 통과. |
| 14 | zero line, zero/negative span, reversed/negative line 처리에서 산술 오류가 나는가 | `validate_endpoints`, `positive_span`, `explicit_line`을 그대로 거쳐 0·잘못된 span은 거부하고 지원되는 signed/reversed line은 기존 regression 결과를 유지한다. 통과. |
| 15 | placement occupancy 예약 실패가 panic 또는 stale output으로 변하는가 | `reserve_exact` 기반 기존 sparse normalizer를 호출하고 allocation error는 `Result`로 반환한다. mixed route에서 별도 무검증 allocation 경로는 추가하지 않았다. 통과. |
| 16 | text, nested/replaced, positioned item, nonzero order가 새 admission으로 들어오는가 | `supported_item` 및 order/position 검사가 기존 bounded contract를 유지한다. 해당 style gates와 negative tests를 확인했다. 통과. |
| 17 | percentage/intrinsic sizing, edge, gap, alignment, direction이 제한 route로 흘러드는가 | definite fixed dimensions, zero gap/edges, LTR, supported alignment 조건을 `validate_container`에서 유지한다. 통과. |
| 18 | 한 owner의 실패 뒤 일부 frame 또는 expanded track이 노출되는가 | mixed mode는 expanded row 결과를 합치기 전에 거부하고 전체 normalizer error 시 output을 반환하지 않는다. row·column implicit negative tests 통과. 통과. |
| 19 | PixelCopy/Metal adapter가 잘못된 좌표, scene, surface revision 또는 표본 수를 성공 처리하는가 | Android `draw/surface` identity 검사와 19-point exact matcher, iOS run/scene 선택과 19-point drawable matcher를 확인했다. 두 summary 모두 `status=matched`; layout ready/draw presented. 통과. |
| 20 | fixture adapter/sample payload가 기본 앱 artifact에 포함되는가 | Android 기본 APK scan은 새 adapter/run/sample 0건, opt-in APK는 세 항목을 포함한다. iOS 기본 app scan은 새 run/scene/sample 0건, opt-in app은 포함한다. 통과. |

## 최종 focused 확인

- Layout one-axis/mixed module: 9/9, [`layout-focused-final.log`](./layout-focused-final.log)
- Root HTML integration module: 8/8, [`style-runtime-final.log`](./style-runtime-final.log)
- Selector tests: 3/3, [`selector-tests-final.log`](./selector-tests-final.log)
- Android API 37 AVD PixelCopy: 19/19 exact, [`android-avd-logcat-pass19.txt`](./android-avd-logcat-pass19.txt)
- iPhone 17 Pro/iOS 26.2 Simulator Metal drawable: 19/19 exact, [`ios-simulator-log-final.txt`](./ios-simulator-log-final.txt)
- Default/opt-in artifact scans: [Android](./android-default-apk-scan.json) · [iOS](./ios-default-app-scan.json)

PR diff review와 GitHub PR/merge는 이 검토에 포함하지 않았다.
