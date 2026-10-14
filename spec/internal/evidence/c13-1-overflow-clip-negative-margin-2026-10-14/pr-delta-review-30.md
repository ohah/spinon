# C13.1 음수 clip margin PR 변경 경계 검토 · 30개 · 2026-10-14

검토 대상은 style-to-layout 테스트, 고정 Chrome 관찰, 이 evidence 디렉터리다. 목적은 실제 검증 경계와 PR 표현을 서로 맞추는 것이다.

| # | 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | 변경이 현재 C13.1 branch 밖에 적용 | 작업 경로가 격리 worktree `feat/c13-1-overflow-clip`인지 확인한다. |
| 2 | 기본 checkout의 기존 작업을 덮음 | 모든 파일 편집은 `/tmp/spinon-c13-1-overflow-clip`에서 수행한다. |
| 3 | production runtime code를 의도치 않게 수정 | delta는 테스트와 evidence에 한정한다. |
| 4 | renderer test가 잘못된 오류 종류를 계속 요구 | renderer 쪽 임시 negative test와 불필요 import를 제거하고 style-to-layout 경계로 이동했다. |
| 5 | 원래 clip matrix가 제거되거나 축소 | renderer `c13_1_clip_matrix` 3개 기존 검사를 다시 실행한다. |
| 6 | 새 테스트가 예상 diagnostic 위치를 검사하지 않음 | child node ID 일치를 검사한다. |
| 7 | 문자열 일부만 맞는 unrelated 오류로 통과 | property·음수 값·Stylo 분류 토큰을 각각 확인한다. |
| 8 | invalid cascade 결과가 layout에 전달될 때 허용됨 | `CascadeDiagnostic` 결과를 요구한다. |
| 9 | layout 오류가 unrelated profile error로 대체 | fixture는 C13.1 overflow cascade를 사용하고 결과 variant를 직접 match한다. |
| 10 | valid control이 누락 | `0px` declaration은 layout 성공 대조군으로 검사한다. |
| 11 | Chrome이 음수 syntax를 거부했다고 잘못 기록 | observation과 테스트가 Chrome supports true 및 computed `-1px`를 확인한다. |
| 12 | browser revision이 생략되거나 혼합 | 고정 revision을 테스트와 evidence에 적는다. |
| 13 | UA가 다른 browser 결과와 섞임 | HeadlessChrome 155 UA를 저장·assert한다. |
| 14 | 원본 fixture가 이동/수정된 뒤 evidence가 낡음 | fixture SHA-256을 observation 및 테스트에서 고정한다. |
| 15 | baseline이 바뀌어도 선택 관찰이 유효한 것으로 오인 | baseline observation SHA-256을 확인한다. |
| 16 | JSON case ID가 다른 행으로 교체 | `clip-margin-invalid` ID를 assert한다. |
| 17 | 관찰 값이 `-1px`가 아닌데 테스트 설명만 음수 | case의 computed clip margin이 `-1px`인지 assert한다. |
| 18 | 관찰에서 overflow axis가 달라짐 | x/y 모두 `clip`인지 assert한다. |
| 19 | test include 경로가 evidence와 불일치 | `include_str!` 경로를 새 evidence JSON 위치와 대조한다. |
| 20 | evidence 문서가 제품 동작 변경을 암시 | 제품 코드 미변경 및 fail-closed 관찰만 명시한다. |
| 21 | W3C 문단을 브라우저 acceptance보다 우위로 단순화 | 표준상 유효성 및 Chrome 파싱 관찰을 별도 표에 기록한다. |
| 22 | Chrome 실제 화면/이벤트 전체를 확인했다고 주장 | parser acceptance 관찰 외 paint/event 완전성은 주장하지 않는다. |
| 23 | ADB 실패를 이 host test가 해결했다고 오인 | 이 slice는 ADB/AVD 앱 route를 실행하지 않았다고 분리한다. |
| 24 | 실기기 검증으로 오해 | 실기기를 연결하지 않았다고 기록한다. |
| 25 | 테스트 결과에서 ignored/filtered 차이를 숨김 | style-to-layout 전체 171 passed와 renderer 3 matrix passed를 각 명령별 보고한다. |
| 26 | WPT 또는 17행 matrix 완료로 확대 | 전체 C13.1 17-row runtime/surface와 WPT not-run을 명시한다. |
| 27 | review 문서 재사용 | 이번 negative-margin delta만 대상으로 별도 20/30 파일을 작성한다. |
| 28 | 새 evidence 디렉터리에 관찰만 있고 판정기준 없음 | README에 사전 판정·출처·실행 범위·한계를 포함한다. |
| 29 | PR 본문이 근거 없이 검토 횟수만 나열 | 본문은 수정과 한계를 설명하고 숫자 자체는 제품 근거로 내세우지 않는다. |
| 30 | push 뒤 PR 상태·head가 확인되지 않음 | publish 후 draft/open 상태, head SHA, checks를 재조회한다. |

초기 오류는 실패 경계를 renderer로 잘못 예상한 테스트 설계였다. 변경을 관찰된 cascade 진단 및 layout 차단에 맞추고, valid control을 추가한 뒤 style-to-layout 전체 및 기존 renderer matrix 테스트가 통과했다. 이번 delta만으로 C13.1 완료 또는 PR 병합 준비를 선언하지 않는다.
