# C11.1 Root HTML implicit row 계획 PR diff 실패 관점 검토 20개

## 판정 범위

PR #1023의 계획·fixture·evidence·상태 대장·공개 roadmap diff를 검토했다. 기준 source는 `a95d4315c`이며, 이 파일은 검토 기록만 추가한다. 제품 코드 변경은 없고 테스트·시뮬레이터 실행도 하지 않았다. `PASS`는 PR diff의 문서 경계와 근거 표시를 확인했다는 뜻이다.

## 검토 결과

| # | 실패 관점 | 확인 내용 | 판정 |
|---:|---|---|---|
| 1 | 계획이 구현된 기능처럼 쓰임 | 계획·STATUS·roadmap·PR 본문 모두 구현 전 상태와 앱 실행 부재를 밝힌다. | PASS |
| 2 | 제한 slice가 C11.1/C11 전체 또는 공개 CSS 지원으로 승격됨 | 부모 checkbox는 그대로 미완료이고 공식 `72/213`을 반복해 명시한다. | PASS |
| 3 | cascade 코드가 `grid-auto-rows`를 전혀 계산하지 않는다고 잘못 설명함 | 현재 Root profile의 단일 fixed length는 `LengthPx`; 거부 지점은 projection이라고 각주와 계획이 구분한다. | PASS |
| 4 | Root HTML named-area 지원이 implicit sizing으로 확장된 것처럼 읽힘 | 신규 mode는 no-area 기본 경로에 한정하며 named-area 조합을 거부한다. | PASS |
| 5 | C11.3.3 placement fixture를 제품 root runtime 계약으로 재사용함 | 전용 `RootHtmlImplicitRowsV1` mode를 계획하고 C11.3.3 mode와 분리한다. | PASS |
| 6 | implicit columns도 함께 허용된 것으로 오해됨 | `grid-auto-columns:auto`, row flow만 명시하고 extra implicit column을 negative control로 둔다. | PASS |
| 7 | 복수 auto-row track 반복을 구현한다고 과장함 | computed single `LengthPx`만 허용하고 list·intrinsic·percentage 계열은 거부한다. | PASS |
| 8 | explicit percentage/fr track과 새 implicit mode의 교차 조합이 검증 없이 포함됨 | 이번 mode는 explicit px tracks와 px gap으로 제한하고 fr/percentage 조합은 후속으로 남긴다. | PASS |
| 9 | item placement, dense/column flow 또는 CSS `order`가 암묵적으로 지원됨 | 모든 visible item은 auto placement와 `order:0`; column/dense와 authored placement는 거부한다. | PASS |
| 10 | anonymous text, nested child 또는 intrinsic image가 빈 fixed item 조건을 우회함 | positive shape는 empty direct block; text/nested/image/intrinsic cases는 negative controls다. | PASS |
| 11 | `display:none` child를 CSS placement occupancy에 포함함 | 계획은 hidden subtree를 제외하며 별도 positive control을 둔다. | PASS |
| 12 | fixture ID/class가 기능 성공을 선택함 | 기본 parsed Root HTML route와 ID/class rename control을 계획하고 fixture dispatch 추가를 금한다. | PASS |
| 13 | zero-height implicit track이 zero track 또는 gap 처리로 사라짐 | expected JSON에 해당 case의 `autoRows:[0]`, rows `[20,0]`, root height `25`를 명시했다. | PASS |
| 14 | zero-row scene에 positive case의 15px 값이 잘못 적용됨 | auto-row sizing을 공통 basis 대신 case마다 기록하고 `zero-row`만 0px로 고쳤다. | PASS |
| 15 | Chrome viewport 좌표와 독립 local expected 좌표를 직접 비교해 오차가 생김 | README에 root offset 차감 규칙과 `(16,69)→(0,25)` 예를 설명했다. | PASS |
| 16 | explicit/implicit row 사이 gutter 계산이 틀림 | 독립 식은 0px·15px 한 row·15px 두 row를 각각 25·40·60px로 고정한다. | PASS |
| 17 | hash가 수정된 fixture/expected 파일과 불일치함 | fixture, observation, screenshot, 수정 expected JSON의 SHA-256을 현재 파일과 대조했다. | PASS |
| 18 | README의 비교 주장을 Chrome 값이나 앱 결과와 혼동함 | Chrome 156 Headless observation은 명시되고 Spinon 출력·native pixel·WPT 결과가 아니라고 한정한다. | PASS |
| 19 | roadmap HTML 문법 또는 새 항목 표시가 깨짐 | Agent Browser로 로컬 roadmap을 열어 제목·새 항목·6개 링크와 표시 box를 확인했다. 세션을 닫고 active session/process가 없음을 확인했다. | PASS |
| 20 | GitHub PR에 작업 범위·label·screenshot 또는 CI 상태를 잘못 전달함 | PR #1023에는 planning/documentation·layout/docs labels와 Chrome screenshot이 있고, PR check rollup이 비어 있음을 성공으로 주장하지 않는다. | PASS |

## 남은 범위

계획 PR은 구현 관문이다. 계획 병합 뒤 코드 구현, 별도 구현/runtime 20개 검토, Rust gate, Android API 37 AVD 및 iOS 26.2 Simulator 검증이 남아 있다. 이 PR의 Chrome screenshot은 앱 동작 증거가 아니다.
