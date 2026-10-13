# C11.2.3 dense·column 배치 계획 실패 관점 검토 · 2026-10-13

## 검토 대상

구현 전 계획 [C11.2.3](../../../plan/c11-2-3-dense-column-placement.md)과 상위 [C11 Grid 계획](../../../plan/c11-css-grid.md), 선행 [C11.2.2 계약](../../../plan/c11-2-2-sparse-row-placement.md)을 대조했다. 계산 후보는 잠금된 Taffy `0.14.0`이다. 그 버전의 placement 구현은 row/column primary axis와 dense cursor reset을 제공하지만, 이 사실만으로 Spinon adapter·Stylo cascade·runtime·WGPU 경로의 일치를 증명하지 않는다.

## 서로 다른 실패 관점 20개

| # | 공격 관점 | 계획에 고정한 방어·판정 |
| --- | --- | --- |
| 1 | `row dense`와 `column dense`를 하나의 enum처럼 혼동 | 네 computed flow를 분리하고 column은 primary/secondary 축을 바꾼 별도 Chromium 장면으로 고정한다. |
| 2 | C11.2.2의 기본 row sparse 회귀 | 기존 결과를 negative control로 C11.2.3 corpus에 포함하고 이전 profile ID를 보존한다. |
| 3 | auto보다 뒤에 있는 definite item이 먼저 cell을 차지하는 단계 순서를 누락 | definite 두 축 item을 먼저 occupancy에 등록하는 관문을 명시했다. |
| 4 | definite secondary-axis item이 sparse/dense마다 시작 위치가 달라지는 점을 누락 | row·column flow 각각 고정축 item 사례를 분리하고 Chrome의 배치 line을 oracle로 삼는다. |
| 5 | definite primary-axis 위치를 무시해 임의 cell로 이동 | `grid-row`/`grid-column` lock을 flow별 positive·collision fixture로 확인한다. |
| 6 | span된 item이 경계에서 일부만 맞는 후보를 잘못 배치 | 2차원 occupancy rectangle과 span collision scene를 명시했다. |
| 7 | dense를 최종 위치 sort로 구현해 중간 cell 검색 의미를 바꿈 | 검색 시작점을 매 auto item마다 되감는 규칙을 입력 순서대로 검사한다. |
| 8 | dense가 source order나 paint order를 바꾸는 것으로 오해 | visual placement와 DOM/accessibility/paint ordering을 분리 검증하고 child vector를 보존한다. |
| 9 | column flow에서 row·column 축을 일부 계산 경로만 transpose | cursor, span, gap, track 확장을 각각 같은 기준 사례에서 두 축 모두 비교한다. |
| 10 | column flow가 명시 행 끝에서 column 확장을 하지 않음 | positive implicit column 수와 `grid-auto-columns` 적용을 명시 지원 장면에 둔다. |
| 11 | row dense에서 positive implicit row 생성·sizing을 누락 | `grid-auto-rows` 단일 px 값의 확장·rect 사례를 유지한다. |
| 12 | `auto` auto-track이 필요한 장면에 fallback 크기를 성공 반환 | `auto`는 해당 축의 implicit track이 없을 때만 허용하고 생성 시 구체 오류로 닫는다. |
| 13 | CSS의 반복 auto-track list를 단일 값처럼 처리 | 복수 pattern은 이번 profile에서 거부하고 잘못된 성공을 막는다. |
| 14 | negative implicit 또는 explicit line 바깥 배치를 양의 확장으로 취급 | negative implicit 및 explicit-line track 생성은 명시적으로 지원 범위 밖에 둔다. |
| 15 | explicit+implicit track 수 계산의 정수 overflow 또는 Taffy 내부 clamp | checked arithmetic과 Taffy 호출 전 축별 합계 상한 검사를 완료 관문에 추가했다. |
| 16 | indefinite axis/percentage track의 기준 크기가 불명확한데 px로 환산 | C11.2.2 제한 definite-axis/track gate를 그대로 유지하고 unsupported 값은 거부한다. |
| 17 | hidden·nested Grid가 부모 occupancy를 오염 | hidden child 제외와 각 nested container 독립 flow를 별도 fixture로 둔다. |
| 18 | `order != 0`이 item iteration order를 바꿔 dense 결과를 다르게 함 | nonzero `order`를 fail-closed 경계로 고정한다. |
| 19 | reservation 실패 뒤 일부 item/track 결과나 stale frame이 publish | container 전체의 실패 원자성, 각 Vec 예약점 allocator 주입, 같은 worker 후속 복구를 요구한다. |
| 20 | Android PixelCopy·iOS drawable·Chrome 결과를 같은 관찰 계층처럼 주장 | runtime geometry와 surface sample을 분리 기록하고 iOS는 compositor 전, WPT는 Chrome과 Spinon으로 분리한다. |

## 검토 결론

계획 단계에서 기능 범위와 관찰 경계를 서로 다른 실패 경로로 고정했다. 별도 [Chromium 사전 비교](./c11-2-3-dense-column-placement-precomparison-2026-10-13/README.md)에서 25-scene·107-node inventory, Chrome 154 DPR 1·2 reference, WPT 후보 3개의 hash와 Chrome runner 결과를 고정했다. 추가 검토에서 명세의 implicit grid track-count 사전 계산 단계를 분명히 하고, 10,000/10,001 span 후보를 oracle-only에서 구현 후보와 명시적 런타임 기대값으로 옮겼다. 다음은 새 runtime profile 구현이며, Android 실기기와 iOS Simulator 공통 실행 입력 및 실패 주입은 구현 계획의 관문으로 남아 있다. 이 기준 고정은 C11.2.3 runtime 구현 완료를 뜻하지 않는다. 실제 runtime 구현이 끝난 뒤에는 변경 diff와 새 실행 결과를 대상으로 별도의 20개 구현 검토를 수행한다.
