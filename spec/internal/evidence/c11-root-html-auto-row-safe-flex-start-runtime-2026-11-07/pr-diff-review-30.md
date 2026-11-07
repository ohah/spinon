# C11.4 `safe|unsafe flex-start` 구현 PR diff 실패 관점 검토 30개

실제 구현 코드, test 변경, fixture 및 진행 상태 동기화 diff를 서로 다른 실패 경계에서 확인했다.

| # | 실패 관점 | diff에서 확인한 점 | 결과 |
|---:|---|---|---|
| 1 | allowlist가 접두부 문자열 비교를 할 수 있음 | exact computed token match 유지 | 이상 없음 |
| 2 | `safe`만으로 모든 positional value를 허용할 수 있음 | 두 flex-start 조합만 추가, end는 negative control | 이상 없음 |
| 3 | `unsafe` 경로만 추가되고 `safe` 누락 | 두 variant 모두 projection/layout matrix에 포함 | 이상 없음 |
| 4 | plain `start` 허용이 제거될 수 있음 | 기존 값 및 regression 유지 | 이상 없음 |
| 5 | value serialization이 canonical 값으로 치환될 수 있음 | test가 입력한 computed string 그대로 확인 | 이상 없음 |
| 6 | Taffy mapping이 safety enum을 무시할 수 있음 | layout mapping과 `FlexStart` position 검사 | 이상 없음 |
| 7 | 변경이 generic layout mode로 확장될 수 있음 | profile gate의 기존 match arm에만 variant 추가 | 이상 없음 |
| 8 | baseline/distributed values가 함께 열릴 수 있음 | allowlist 폐쇄와 negative matrix 유지 | 거부됨 |
| 9 | safe/unsafe flex-end가 허용될 수 있음 | negative controls를 두 flex-end 값으로 이동 | 거부됨 |
| 10 | style과 layout admission이 불일치할 수 있음 | 같은 두 values를 projection 및 layout tests에서 확인 | 일치 |
| 11 | positive scene 수가 빠질 수 있음 | JSON 기반 positive × 3 values loop | 3개 검사 |
| 12 | overflow scene 수가 빠질 수 있음 | JSON 기반 overflow × 3 values loop | 3개 검사 |
| 13 | 독립 기대값 파일과 테스트 값이 분리될 수 있음 | layout, projection, runtime가 같은 precomparison JSON include | 일치 |
| 14 | 0px gap scene에 overflow 값이 들어갈 수 있음 | scene table에서 `positive → 0px` 고정 | 이상 없음 |
| 15 | overflow gap이 잘못 계산될 수 있음 | `calc(120% - 2px)`와 40px definite basis 유지 | 기대값 46px |
| 16 | item source order가 바뀔 수 있음 | 고정 item과 A/B text node의 frame 순서별 검사 | 이상 없음 |
| 17 | item x 좌표가 틀려도 y만 통과할 수 있음 | x/y 모두 1/64 CSS px 허용오차 검사 | 이상 없음 |
| 18 | item width/height가 변할 수 있음 | 모든 frame dimension exact 비교 | 이상 없음 |
| 19 | rows가 alignment 변화와 함께 stretch될 수 있음 | trace rows `[10,10,10]` 검사 | 이상 없음 |
| 20 | overflow gutters 분배가 바뀔 수 있음 | 양 gutter가 각각 46px인지 확인 | 이상 없음 |
| 21 | projection computed style가 생략될 수 있음 | `properties[align-content]` exact 비교 | 이상 없음 |
| 22 | runtime 결과가 stale revision일 수 있음 | completion key와 cascade completion key 비교 | 동일 |
| 23 | renderer snapshot이 없는데 성공으로 보일 수 있음 | completed render snapshot을 필수 unwrap하고 bounds 검사 | 이상 없음 |
| 24 | negative control 오류가 다른 속성으로 귀속될 수 있음 | `align-content` error ownership 확인 | 이상 없음 |
| 25 | 실패 후 session이 사용할 수 없게 될 수 있음 | valid request로 same-session recovery 검사 | 이상 없음 |
| 26 | old safe-start control이 새 지원값을 계속 거부할 수 있음 | start test negative control을 flex-end로 갱신 | 일치 |
| 27 | Android/iOS에서 fixture route 자체가 실행되지 않을 수 있음 | default app boot·layout-ready·draw/present logs 확인 | 양쪽 통과 |
| 28 | APK 또는 iOS app이 다른 HTML을 포함할 수 있음 | 번들 fixture SHA와 고정 입력 SHA 비교 | 일치 |
| 29 | app/process/build 상태가 실행 뒤 달라질 수 있음 | 설치 앱·build artifacts 해시 보존/복원 기록 확인 | 복원 PASS |
| 30 | 상태·계약 문서가 진행을 완료로 오인시킬 수 있음 | STATUS 체크와 `78/214`를 유지하고, 계약 0059에 제한 profile·미검증 범위와 후보 PR #1256을 기록하며 roadmap에 진행 카드만 표시 | 이상 없음 |

이 검토는 PR diff와 기록된 실행 증거에 한정한다. GitHub check rollup이 비어 있다면 이를 CI 통과로 해석하지 않는다.
