# C11.3.3 named-line implicit auto-track 계획 PR 변경 검토 30개

계획 PR의 실제 staged diff를 30개의 독립 실패 관점으로 검토했다. 검토 대상은 계획, 고정 HTML 입력, Chrome 관찰 JSON, 규범 기대값, screenshot, 계획 자체 검토표다. 계획 PR은 아직 구현·runtime 검증 결과를 주장하지 않는다.

검토 중 규범 링크를 편집 중인 CSSWG draft에서 고정된 2025 W3C CRD로 바꾸고, §7.5 anchor를 `#implicit-grids`로 바로잡았다. viewport보다 작은 가상 `screen` 값은 의미가 없어 observation에서 제외했다. 수정된 diff 기준으로 아래 항목을 다시 확인했다.

| # | PR diff 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | C11 상위나 세부 후보를 완료로 표시할 위험 | 계획과 문서는 C11.3.3/C11을 미완료로 유지한다. |
| 2 | 공식 완료 합계를 임의 재계산할 위험 | 저장소 기준 `72/213`을 고정하고 퍼센트를 만들지 않는다. |
| 3 | area-backed 경로와 no-area 경로를 같은 관찰로 취급 | 입력은 `grid-template-areas`를 전혀 지정하지 않고 C11.3.3 gap을 분리한다. |
| 4 | area가 만든 explicit track을 implicit이라고 오분류 | 사전 비교 장면은 explicit grid 밖 definite named-line placement만 생성한다. |
| 5 | CSS named-line 규칙과 fixture shorthand가 어긋남 | `grid-column-start: missing-column` 및 `grid-row-start: missing-row`를 직접 고정한다. |
| 6 | 생략한 occurrence가 첫 named line이라는 전제가 빠짐 | 2025 CRD §8.3과 expected-values basis에 기본 occurrence 규칙을 연결한다. |
| 7 | 이름이 없는 named line이 implicit line을 생성한다는 근거 부족 | §8.3의 부족한 named-line 검색 규칙을 기대값에 명시한다. |
| 8 | leading/signed 검색 방향까지 잘못 허용 | 계획은 signed mode rejection을 보존하고 그 성공을 요구하지 않는다. |
| 9 | 범위 밖 숫자 배치가 새 허용 경로에 들어옴 | projection에서 numeric implicit extent를 막는 경계 확인을 계획 acceptance 전에 둔다. |
| 10 | auto-placement item이 definite named-line 경로와 섞임 | 계획은 automatic-placement 입력을 mode에서 배제하는지 확인한다. |
| 11 | columns 한 축만 구현하고 rows를 누락 | 독립 columns/rows positive 장면과 각각의 computed track 기대값이 있다. |
| 12 | columns auto 결과가 item 위치와 독립적으로만 확인됨 | columns의 track sequence와 root-relative item frame을 모두 고정한다. |
| 13 | 고정 track 대조군이 placement 수를 검증하지 못함 | `30px` fixed control의 track sequence와 item x=110px를 분리해 기록한다. |
| 14 | empty implicit track의 0px와 item track을 혼동 | expected sequence에서 0px empty track 다음의 content track을 구분한다. |
| 15 | auto sizing contribution이 다른 고정값에서 유래 | item CSS width/height와 기대 track contribution 30px/25px를 고정한다. |
| 16 | grid item alignment가 track 계산 폭을 바꿈 | `justify-items`, `align-items`, self alignment를 start로 고정한다. |
| 17 | 기본 margin/padding/border로 frame이 이동 | item의 margin, padding, border를 0으로 고정한다. |
| 18 | gap이나 분배 정렬이 track 폭에 섞임 | row/column gap 0, `justify-content`/`align-content` start다. |
| 19 | root frame과 item frame을 viewport 절대값으로 비교 | observation과 기대값은 item root-relative frame을 함께 저장한다. |
| 20 | root보다 큰 item을 잘라 overflow 회귀가 숨음 | `overflow:visible`; rows는 5px 하단, columns는 우측 경계 밖을 드러낸다. |
| 21 | screenshot과 DOM 관찰이 다른 입력을 사용 | screenshot 대상은 고정 로컬 fixture이며 input SHA-256과 screenshot SHA-256을 기록한다. |
| 22 | 화면이 실제 root 밖 영역을 보여주지 않음 | 육안 확인에서 dark root와 red child가 root 경계를 넘는 위치를 확인했다. |
| 23 | 브라우저 판본이나 DPR 미상으로 비교 재현 불가 | UA의 `HeadlessChrome/156.0.0.0`, viewport 1280×633, DPR 1을 기록한다. |
| 24 | 화면 장치 크기 값이 viewport와 모순처럼 보임 | 의미 없는 가상 `screen` 크기는 observation에서 제거했다. |
| 25 | input hash가 repository baseline과 연결되지 않음 | source commit과 uncommitted fixture의 SHA-256을 각각 기록한다. |
| 26 | 살아 있는 최신 spec을 사용해 규범이 변동 | 편집 중 draft 대신 날짜가 있는 W3C 2025 CRD를 사용하고 세 anchor를 확인했다. |
| 27 | W3C §7.5/§7.6/§8.3 링크가 잘못된 heading으로 연결 | Agent Browser에서 published spec의 각 heading ID를 확인해 anchor를 고쳤다. |
| 28 | Chrome oracle을 Spinon/WPT 적합성으로 과장 | README와 observation에 독립 Chrome 관찰만이며 제품 결과가 아니라고 구분한다. |
| 29 | 계획 PR을 구현 검증 완료로 오해 | 계획의 상태를 구현 전으로 표시하고 runtime/플랫폼 검증을 후속 단계로 둔다. |
| 30 | 테스트·artifact/플랫폼/실패 관점 검증 범위가 불분명 | 구현 계획에 Rust/runtime, Android/iOS Simulator, 기본/opt-in artifact, 20/30 review를 분리했다. |

PR diff의 서식 검사는 `git diff --cached --check`, JSON syntax 검사는 `python3 -m json.tool`로 통과했다. 이 계획 전용 PR에는 제품 build나 runtime test를 실행하지 않았다.
