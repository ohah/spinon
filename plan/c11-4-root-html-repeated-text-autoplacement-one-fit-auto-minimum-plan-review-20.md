# C11.4 one-fit auto-placement 자동 최소 크기 계획 · 실패 관점 20개

검토 대상은 [bounded 계획](./c11-4-root-html-repeated-text-autoplacement-one-fit-auto-minimum.md)과 [Chrome 157 사전 비교](../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-one-fit-auto-min-precomparison-2026-11-10/README.md)다. 각 관점에서 계획이 제한 조건과 실행 근거를 구체적으로 고정하는지 확인했다.

| # | 적대적 실패 관점 | 실패 경로 | 계획의 방어·근거 | 판정 |
|---:|---|---|---|---|
| 1 | `min-height:auto`를 전 item에 전파 | 모든 Grid item의 자동 최소 크기를 일반 허용해 unsupported author CSS가 admission 됨 | one-fit 9-item repeated-text profile과 여덟 text item만 허용하고 control은 0으로 고정 | 통과 |
| 2 | two-fit gate와 predicate 혼합 | one-fit 요청으로 two-fit 전용 검증을 우회하거나 two-fit 경계가 바뀜 | one-fit·two-fit placement·two-fit span control을 별도로 둠 | 통과 |
| 3 | fit track 개수 누락 | zero-fit이나 multiple-fit list가 one-fit auto minimum으로 통과 | 정확히 하나를 requirement로 두고 zero/two/multiple fit rejection을 계획 | 통과 |
| 4 | authored position 누락 | 일부 위치에서 automatic minimum floor가 cap 아래로 clamp | fit position 1–6 전체 1,440-scene key를 계획 | 통과 |
| 5 | tail multiplicity 느슨함 | 두 min-content·두 max-content·한 auto 조건 외의 리스트가 허용 | 정확한 6-member multiplicity와 30개 순서를 고정 | 통과 |
| 6 | `20px`와 `20%` 의미 혼합 | typed cap 손실로 length와 percentage basis 차이를 확인하지 못함 | 고정 cap matrix의 direct px/% 값과 authored/computed order를 비교 | 통과 |
| 7 | percentage basis 불명 | `%`를 viewport 또는 width 기준으로 계산 | definite 220px block-size와 44/0/264px cap을 명시 | 통과 |
| 8 | cap을 automatic minimum 상한으로 오해 | fit-content 0%·20px가 text content floor를 깎음 | §6.6와 0%·20px floor-positive control을 구현 acceptance에 명시 | 통과 |
| 9 | scrollable item 오적용 | `overflow:auto|scroll`에도 content-based minimum을 주장 | 조건을 visible/non-scrollable로 제한하고 overflow 변형은 rejection 항목으로 둠 | 통과 |
| 10 | computed style와 used minimum 혼동 | computed `auto` 문자열만 확인하고 used track floor를 확인하지 않음 | computed minimum과 실제 row/item/payload geometry를 별도로 검증 | 통과 |
| 11 | explicit control 오염 | 첫 10px control row가 automatic minimum 대상으로 바뀜 | control `min-height:0`, frame `[0,0,40,10]`을 고정 | 통과 |
| 12 | implicit cycle offset | explicit row를 auto-row list index 1로 잘못 세어 fit 위치가 밀림 | explicit row 뒤 8 row의 six-member cycle과 repeated row 7/8을 대조 | 통과 |
| 13 | text 측정 분모 변경 | content/font가 달라져 floor rescue 수가 달라짐 | 기존 ASCII payload, 10px monospace line-height, line-count vector와 hash를 유지 | 통과 |
| 14 | nested payload 과허용 | nested/replaced/inline content가 profile parser를 통과 | 단일 direct payload div와 기존 exact child topology를 유지 | 통과 |
| 15 | 다른 intrinsic row 변화 은폐 | fit row를 고치며 min/max-content 행까지 바뀜 | baseline-paired scene에서 non-fit start track은 유지되고 모든 row floor를 검증 | 통과 |
| 16 | stretch 자유 공간 오분배 | fit/min/max 행이 늘거나 auto occurrence만의 규칙 위반 | start/stretch pair에서 40px free space와 auto-only delta를 확인 | 통과 |
| 17 | overflow를 과대 주장 | 220px 밖 text overflow를 지원한 것처럼 보고 | auto-minimum corpus에서 overflow 0을 결과로 기록하고 overflow 구현 주장은 제외 | 통과 |
| 18 | incomplete trace 공개 | row owner/revision/cycle 정보 누락에도 일부 frames가 발행 | fail-closed 및 published revision/frame atomicity, same-session recovery를 완료 관문에 둠 | 통과 |
| 19 | 회귀 분모 혼동 | one-fit zero, two-fit auto-placement/span 기존 profile을 새 결과로 대체 | 세 regression controls와 기존 text corpus를 별도로 재검증 | 통과 |
| 20 | 리소스·상태 오분류 | 공유 simulator 삭제, 브라우저 잔존, 또는 C11.4 공식 완료로 기록 | task ownership을 확인한 임시 리소스만 제거, 공유 기기 보존, `78/214` 및 parent 체크 유지 | 통과 |

**결과:** 서로 다른 계획 실패 관점 20개를 검토했다. Chrome oracle의 관계는 고정됐으며 RuntimeSession·Android·iOS 결과는 아직 없다. 제품 구현은 계획 PR 병합 뒤 시작한다.
