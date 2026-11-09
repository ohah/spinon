# C11.4 두-fit auto-placement 자동 최소 크기 계획 · 실패 관점 20개

검토 대상은 [계획](./c11-4-root-html-repeated-text-autoplacement-two-fit-auto-minimum.md)과 [Chrome 157 사전 비교](../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-two-fit-auto-min-precomparison-2026-11-10/README.md)다. 계획 단계에서 입력, CSS Grid automatic minimum 조건, track/list/topology 경계, runtime atomicity, simulator cleanup 및 보고 범위를 서로 다른 관점으로 확인했다.

| # | 공격 관점 | 실패 가능성 | 계획의 방어·근거 | 판정 |
|---:|---|---|---|---|
| 1 | 자동 minimum의 overflow 전제 | scrollable item에도 content-based minimum을 적용해 범위를 넓힘 | visible overflow와 scrollable overflow negative를 고정하고 §6.6 조건을 적음 | 통과 |
| 2 | 자동 minimum의 track 전제 | auto minimum track을 포함하지 않는 item에도 floor를 주장 | fit-content auto minimum 조건과 track별 trace 검증을 명시 | 통과 |
| 3 | fit-content cap과 floor 혼동 | cap를 content-based minimum의 상한으로 잘못 취급 | W3C §6.6 문구와 0px cap의 20px Chrome control을 기록 | 통과 |
| 4 | 기준선 섞기 | min-height:0과 auto scene의 content/track 차이를 입력 탓으로 오인 | 동일 2,880 scene index, CSS input, payload와 Chrome 버전을 맞춤 | 통과 |
| 5 | 0px·0% 손실 | typed cap 값이 계산 과정에서 같은 값으로 뭉침 | authored/computed order와 typed cap을 별도로 보존하고 두 assignment를 검증 | 통과 |
| 6 | percentage basis 불명 | percentage cap를 임의 viewport 또는 border box 기준으로 계산 | definite 220px Grid block size와 direct percentage basis를 명시 | 통과 |
| 7 | fit 위치 누락 | 1–6의 서로 다른 15 pair 중 일부만 수용 | 15 pair 전체 matrix와 exact pair assertion을 둠 | 통과 |
| 8 | cap 방향 전도 | 두 위치에 대한 ordered cap assignment 중 하나를 누락 | 8개 ordered assignment를 고정하고 authored 위치와 연결 | 통과 |
| 9 | tail multiset/order 확장 | 일부 샘플로 임의 tail permutation까지 지원한다고 과장 | 2 min-content + 1 max-content + 1 auto와 정확히 12 order만 허용 | 통과 |
| 10 | explicit row의 cycle offset | control row가 implicit list의 첫 track으로 잘못 계산 | 10px explicit row 뒤 8개 item의 modulo-6 위치와 trace를 확인 | 통과 |
| 11 | auto placement item 순서 | DOM 순서와 row order를 바꾸고도 통과 | 고정 8 item DOM 순서, row top, item frame 대조를 둠 | 통과 |
| 12 | text payload 변경 | line wrapping/font/content 차이를 automatic minimum 효과로 오인 | 기존 단일 payload grammar, monospace 10/10, 고정 line-count를 해시와 관측에 고정 | 통과 |
| 13 | direct-child topology 우회 | nested, inline, replaced child까지 자동 허용 | 단일 direct payload div와 child/style shape를 admission 조건으로 고정 | 통과 |
| 14 | fit floor만 확인하고 다른 row 누락 | min/max/auto track의 frame 또는 content floor 위반을 놓침 | 8개 row 전체의 item/payload frame 및 row ≥ content floor를 대조 | 통과 |
| 15 | stretch 경로 오염 | align-content:stretch가 non-auto track을 늘림 | 1,440개 paired list에서 free space·auto occurrence별 delta와 non-auto delta를 검증 | 통과 |
| 16 | overflow 과장 | baseline overflow를 auto-minimum에서도 유지한다고 잘못 보고 | auto-minimum scene의 text 끝과 220px Grid 경계를 독립 계산 | 통과 |
| 17 | one-fit 회귀 | 새 two-fit admission이 기존 one-fit 경계를 변경 | one-fit 1,440-scene 회귀 및 잘못된 fit multiplicity 거부를 요구 | 통과 |
| 18 | row-span 혼합 | 이미 지원된 two-fit row-span minimum 경계를 auto-placement로 혼합 | 두 topology의 validator/layout gates를 분리하고 각각 회귀 | 통과 |
| 19 | stale trace와 부분 공개 | 누락/다중/다른 revision의 track trace를 받아 partial frame 공개 | fail-closed, atomic rejection 및 same-session recovery를 구현 gate로 고정 | 통과 |
| 20 | 미사용 리소스·완료 과장 | shared simulator 삭제, 브라우저 세션 잔존 또는 78/214·C11.4 과장 | 시작 전 inventory, 전용 임시 resource cleanup, reusable profile 보존, 상태 count 유지 | 통과 |

**결과:** 서로 다른 계획 실패 관점 20개를 모두 검토했다. Chrome 사전 비교와 표준 조건은 근거로 확정됐으며 RuntimeSession·Android·iOS 구현 결과는 아직 없다. 구현 승인 근거로 Chrome 결과를 재사용하지 않는다.
