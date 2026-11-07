# C11.4 Root HTML `grid-auto-rows:max-content` 계획 실패 관점 검토 20개

계획 단계에서 CSS Grid 규범, Chrome 157 fixture, 독립 기대값, 현재 style/layout admission을 각각 대조했다. 이 표는 구현 전에 계획 자체를 검토하며, 구현/runtime 및 PR diff 검토에는 재사용하지 않는다.

| # | 실패 관점 | 확인·차단 방법 | 판정 |
|---:|---|---|---|
| 1 | CSS 문법상 유효한 `<track-size>+` 값을 profile 전반의 지원으로 과장 | singleton `max-content` 한 값만 plan scope에 고정했다. | PASS |
| 2 | 기존 min-content path와 새 max-content track identity를 혼동 | typed projection에서 `GridTrackSize::MaxContent`를 직접 확인하도록 요구했다. | PASS |
| 3 | Chrome 관측의 동일 geometry를 서로 다른 track algorithm 구분 증거로 과장 | definite inline-size fixture에서는 geometry가 같음을 명시하고 computed/typed value를 별도 관문으로 뒀다. | PASS |
| 4 | fixture의 source가 바뀌어 예상값이 재현되지 않음 | 정확한 tracked HTML fixture 경로와 SHA-256을 고정했다. | PASS |
| 5 | explicit 10px row를 implicit max-content row로 잘못 계산 | 첫 item과 explicit row를 고정하고 두 implicit row의 순서·크기를 별도로 확인한다. | PASS |
| 6 | text line count가 플랫폼별 monospace advance 차이로 달라짐 | browser metric과 simulator metric을 분리 기록하고 10px line-height·실제 line/frame을 검사한다. | PASS |
| 7 | line-height 10px에서 text contribution 20px를 잘못 계산 | Chrome computed used rows와 독립적으로 두 line × 10px 계산을 대조한다. | PASS |
| 8 | 마지막 item의 fixed 34px size가 max-content row contribution을 덮음 | 마지막 item을 별도 implicit row에 두고 row 34px·y=30px·bottom=64px를 확인한다. | PASS |
| 9 | Grid border-box나 viewport overflow clipping이 track geometry를 숨김 | box edges 0, Grid 40×40px, `overflow:visible`, 마지막 item visible bottom 64px를 고정했다. | PASS |
| 10 | auto placement가 implicit row 개수나 source order를 바꿈 | row flow·sparse placement·세 item DOM 순서를 고정하고 각 frame/order를 확인한다. | PASS |
| 11 | singleton MaxContent 확대가 mixed track pattern까지 허용 | multi-track list와 mixed min/max list를 fail-closed negative로 지정했다. | PASS |
| 12 | intrinsic explicit column과 새 implicit row 조합이 우연히 열림 | 해당 조합은 명시적으로 계속 거부하고 positive shape는 px explicit tracks만 사용한다. | PASS |
| 13 | auto-repeat, named area/line 또는 dense flow가 같은 candidate로 통과 | owner candidate의 기존 제약을 유지하고 각각을 negative matrix에 둔다. | PASS |
| 14 | parser가 거부한 declaration과 computed-but-unsupported 값을 같은 결과로 취급 | CSS supports/computed serialization과 projection rejection을 분리 확인한다. | PASS |
| 15 | 다른 grid owner 또는 nested Grid가 제한 profile을 상속 | 단일 root owner·직접 child topology를 유지하고 nested/multiple owner를 거부한다. | PASS |
| 16 | hidden/추가/잘못된 두 번째 text item이 부분 layout을 생성 | 추가·hidden·malformed item negative와 atomic publication absence를 요구한다. | PASS |
| 17 | stale style/document revision의 텍스트 계측을 재사용 | CSS computed style, text source와 render snapshot key의 동일 revision을 검사한다. | PASS |
| 18 | 예약/track 수 상한에서 일부 row만 게시 | 기존 10,000 track 및 checked count 한계를 유지하고 over-limit rejection을 둔다. | PASS |
| 19 | layout failure 뒤 RuntimeSession이 오염되어 재실행 실패 | 오류 property/node attribution, partial-output 부재와 same-session positive recovery를 확인한다. | PASS |
| 20 | 고정 fixture 통과를 공식 C11.4/C11 완료로 확대 | 내부 계약 `0.1.0` 및 공식 `78/214`를 유지하고 미지원 범위를 문서화했다. | PASS |
