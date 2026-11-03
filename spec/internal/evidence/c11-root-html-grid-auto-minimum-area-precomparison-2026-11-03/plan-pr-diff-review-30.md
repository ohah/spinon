# C11.1 named-area auto-minimum 계획 PR 변경분 적대 검토 30개

검토 대상은 이 계획 PR의 실제 staged diff 전체다: C11 상위 계획에 추가한 next-candidate 문구, [신규 bounded plan](../../../../plan/c11-root-html-grid-auto-minimum-named-areas.md), 별도 [계획 검토 20개](../../../../plan/c11-root-html-grid-auto-minimum-named-areas-plan-review-20.md), Chrome [precomparison fixture](../../../../tests/fixtures/css/c11/root-html-grid-auto-minimum-area-precomparison.html), 독립 expected/Chrome observation, screenshot과 SHA manifest. Binary screenshot은 생성 후 직접 열어 여섯 장면과 border overflow를 확인했다.

| # | 실제 변경에서 공격적으로 확인한 결함 경로 | 판정 |
| --- | --- | --- |
| 1 | C11/C11.1 checkbox 또는 공식 완료 수를 candidate 계획만으로 올림 | 통과. 상위 plan 문구가 미구현 후보라고 명시한다. |
| 2 | 신규 plan을 공식 STATUS 완료 항목처럼 노출 | 통과. `spec/STATUS.md`를 변경하지 않았고 C11 checkbox는 그대로다. |
| 3 | C11 계획의 최신 병합 항목 #1109와 순서 충돌 | 통과. 후보 문구는 #1109 기록 뒤, 더 오래된 slice들 앞에 놓였다. |
| 4 | 기존 no-area auto-minimum 계약과 named-area 후속 계획을 중복 서술하면서 범위가 다름 | 통과. 기존 named-area validator의 거부 경계와 이번 조합 범위를 연결했다. |
| 5 | CSS Grid 규범 계산과 Chrome 관측을 하나의 oracle로 취급 | 통과. `expected.json`과 `chrome-observation.json`은 별도 파일이며 README도 source-of-truth를 구분한다. |
| 6 | 예상값이 screenshot 관찰 후 역산된 값 | 통과. expected는 CSS track sizing 식으로 먼저 산출했다고 기록하고 Chrome은 교차 확인으로만 쓴다. |
| 7 | CRD URL/버전이 현재 normative reference와 다름 | 통과. 고정 2025-03-26 CSS Grid CRD와 관련 track sizing sections를 링크한다. |
| 8 | `minmax(auto,1fr)`의 free-space 계산이 neighbor track을 제외 | 통과. 100px content axis에서 40px fixed neighbor를 뺀 control 60px을 명시한다. |
| 9 | `minmax(0,1fr)` control이 다른 scene styles나 viewport를 사용 | 통과. 같은 fixture/root dimensions/items를 사용하고 track minimum만 바뀐다. |
| 10 | `minmax(auto,0fr)`의 zero-factor valid boundary가 빠짐 | 통과. columns/rows 각 별도 scene으로 80px auto minimum을 확인한다. |
| 11 | rows matrix를 columns CSS나 배경 축에 잘못 매핑 | 통과. root, used tracks, item frame이 축별로 기록되고 기대와 관측이 일치한다. |
| 12 | area matrix가 header를 columns axis에서 span하게 해 positive contribution을 잘못 귀속 | 통과. side item은 첫 column의 단일 area이며 빈 header의 별도 frame도 수집한다. |
| 13 | rows scene의 header가 sizing axis를 span해 contribution을 분산 | 통과. header는 첫 row만 차지하고 second row의 side/content는 각각 40px이다. |
| 14 | CSS area alias가 작성한 child 순서와 다르게 placement됨 | 통과. fixture의 header/side/content computed item frame을 모두 별도로 비교한다. |
| 15 | area profile보다 넓은 arbitrary map/admission을 요구 | 통과. exact existing 2×2 named-area profile만 계획에 허용한다. |
| 16 | second fixed track/row 폭이 expected 40px와 다름 | 통과. HTML computed used tracks와 independent values가 양축에서 일치한다. |
| 17 | root border를 content axis에 포함해 spill 값을 20px/22px로 오기 | 통과. content overflow 20px과 outer-border 넘어선 18px을 구분한다. |
| 18 | outside-border sample이 overflow child 내부를 실제로 찍지 않음 | 통과. auto scene sample은 content item의 blue pixel 내부, zero control sample은 root 밖 white 영역이다. |
| 19 | sample point가 device-scale 변환 없이 CSS coordinate로 해석됨 | 통과. precomparison DPR 1과 root-relative 위치를 보존하고 native gate에서 scale 기록을 요구한다. |
| 20 | zero-control outside sample의 배경색이 browser canvas 기본값에 우연히 의존 | 수정 후 통과. fixture body를 흰색으로 명시하고 screenshot을 다시 캡처했다. |
| 21 | expected surface RGBA를 screenshot pixel extraction 결과라고 과장 | 통과. README는 CSS fixture 색상에서 산출했고 screenshot은 시각 자료라고 제한한다. |
| 22 | Chrome browser version 또는 viewport 누락으로 관측 재현 불가 | 통과. HeadlessChrome 156, Agent Browser 0.31.1, 1280×633 CSS px, DPR 1을 기록한다. |
| 23 | fixture 또는 screenshot이 기록된 SHA와 불일치 | 통과. staged source에서 SHA256SUMS 검증이 네 파일 모두 통과한다. |
| 24 | screenshot이 일부 장면을 잘라 border 바깥 spill을 보여주지 않음 | 통과. full-page image를 직접 확인했고 여섯 장면이 모두 보인다. |
| 25 | Chrome screenshot/DOM observation을 Spinon runtime 또는 WPT 결과로 잘못 해석 | 통과. README와 plan이 oracle-only와 runtime/적합성 미검증을 반복해 구분한다. |
| 26 | 기본 HTML runtime 대신 fixture-only product profile/FFI route로 기능을 열 위험 | 통과. plan은 기존 default Root HTML route를 요구하고 새 FFI/fixture profile을 금지한다. |
| 27 | native capture 전에 사용한 Root HTML source를 복구하지 않는 위험 | 통과. 구현 gate에 original source hash 보존과 복구 검사를 포함한다. |
| 28 | Android AVD 또는 iOS Simulator 결과를 hardware/physical-device 결과로 승격 | 통과. AVD software renderer와 iOS compositor 이전 drawable 제한을 요구한다. |
| 29 | plan review 20개를 runtime review 또는 PR review로 재사용 | 통과. 계획은 새 runtime 20개와 실제 구현 diff 30개를 따로 요구한다. |
| 30 | 계획 PR 승인 뒤 CI/merge/roadmap sync 경계 누락 | 통과. 기존 Rebase and merge 승인, live checks 점검, merge 후 status/spec/roadmap sync를 계획한다. |

수정 사항: 첫 review pass에서 outside-border negative-control 색상이 canvas 기본값에 의존하는 점을 발견했다. Fixture body background를 명시하고 Chrome 캡처를 다시 실행했다. 최종 staged diff에서 30개 관점은 모두 통과했다. 이는 계획 PR 검토이며 구현 후 검증을 대체하지 않는다.
