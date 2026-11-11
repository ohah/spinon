# C11.4 six-fit row-span 계획 PR 변경 검토 30개 관점

검토 대상은 새 bounded plan, 20관점 plan review, Chrome 157 사전 비교 fixture와 raw evidence, `C11.4` 다음 후보를 반영한 계획 인덱스 및 공개 roadmap 변경이다. 구현 코드는 포함되지 않는다. 각 항목은 이번 PR의 별도 실패 경로를 기준으로 확인했다.

| # | 실패 관점 | diff 확인 및 판정 |
| ---: | --- | --- |
| 1 | 계획 단계에서 제품 코드를 함께 바꿔 승인 순서를 우회 | `crates/` diff가 없고 계획·fixture·증거·roadmap만 포함한다. 계획 병합 뒤에 구현을 시작하도록 명시했다. |
| 2 | 제한 slice를 공식 기능 완료로 잘못 승격 | 공식 `78/214`, C11.4/C11 미완료와 내부 계약 `0.1.0`을 계획·roadmap에서 유지한다. |
| 3 | 선행 five-fit row-span 근거를 다른 작업과 혼동 | PR #1360과 계획 링크를 확인했다. |
| 4 | six-fit auto-placement를 row-span 지원으로 간주 | PR #1354의 nine-child non-spanning 경계를 따로 명시했다. |
| 5 | 반복 intrinsic list의 six-fit 공통 admission으로 범위 확대 | 목표를 exact six-fit row-span predicate/profile로 한정하고 기존 profile을 분리했다. |
| 6 | 기존 single·three·four·five-fit 진행 기록이 stale 상태로 남음 | 상위 track-sizing 계획에 #1326/#1331/#1338/#1357/#1360 순서를 갱신하고 새 후보를 뒤에 기록했다. |
| 7 | fit-content 개수가 여섯이 아닌 입력이 fixture에 혼입 | fixture가 여섯 member를 모두 `fit-content()`로 생성하며 720개 순열을 열거한다. |
| 8 | 동일 cap을 반복하거나 필요한 cap을 빼는 order를 허용 | 고정 typed multiset `[0px, 10px, 20%, 30px, 40px, 50px]`의 순열만 만든다. |
| 9 | `%`를 indefinite basis에서 비교 | definite 220px block size를 fixture에 고정했고 독립 계산에서 20%는 44px이다. |
| 10 | authored cap order와 track index가 분리 | scene에 track list와 cap list를 함께 저장하고 관측에 computed/used rows를 포함한다. |
| 11 | explicit row control이 implicit row count를 바꿈 | 빈 explicit row 10px을 row 1에 고정하고 row 2부터 span topology를 둔다. verifier는 모든 장면의 7행을 확인한다. |
| 12 | 첫 topology가 여섯 implicit row 일부를 덮지 않음 | row 2–6에서 다섯 span-2 item을 겹치게 두어 row 2–7을 덮는다. |
| 13 | 둘째 topology의 span-2/3 경계 누락 | 시작 row 2·3의 span-2와 row 4·5의 span-3 네 item이 합쳐 row 2–7을 덮는다. |
| 14 | row placement를 DOM 순서와 혼동 | 두 topology 모두 forward/reverse DOM order를 별도 scene으로 만들고 ID별 frame 차이가 0인지 확인한다. |
| 15 | baseline/candidate 비교에서 item content나 topology도 바뀜 | 동일 scene key에 `min-height:0|auto`만 교차하고 paired key 5,760개를 확인한다. |
| 16 | control item의 minimum이 baseline/candidate 간 바뀜 | 빈 explicit-row control은 항상 `min-height:0`이다. |
| 17 | `align-content`가 별도 변수로 누락 | `start|stretch`를 양쪽 minimum mode와 교차했고 관측 기하 차이는 0이다. |
| 18 | content-floor shortfall을 grid-end overflow로 오인 | payload-floor violation과 220px Grid 끝 overflow를 분리해 기록한다. |
| 19 | baseline 재현만 있고 candidate acceptance가 불명확 | 4,800 baseline shortfalls와 0 candidate shortfalls를 verifier가 각각 판정한다. |
| 20 | item span frame과 covered-row 합산이 어긋남 | 11,520개 scene의 최대 오차가 `0.00005px`이고 허용치 `0.01px` 안이다. |
| 21 | 자동 minimum의 normative eligibility를 잘못 일반화 | §6.6 non-scrollable·auto-minimum track 조건, multi-track flex exclusion 및 §11.2 fit-content minimum function을 별도로 설명한다. |
| 22 | `fit-content()` cap을 fixed max track으로 해석 | §11.2/§11.5 경계를 명시하고 Chrome pair를 cap·minimum 대조로 사용한다. |
| 23 | oracle 버전·환경이 재현 불가 | Agent Browser 0.31.1, HeadlessChrome 157.0.0.0, MacIntel, DPR 1, 1280×900을 README와 JSON에 고정했다. |
| 24 | raw observation이 fixture와 연결되지 않음 | verification JSON에 fixture 경로·바이트 수·SHA-256과 gzip observation 바이트 수·SHA-256이 있다. |
| 25 | verifier 결과가 raw capture와 불일치 | verifier를 재실행했고 11,520 scenes, 720 orders, 5,760 pairs 및 acceptance 전부를 다시 얻었다. |
| 26 | fixture 수정 뒤 capture hash가 stale | 현재 fixture SHA `6469adc1…dd00450`가 verification JSON의 값과 일치한다. |
| 27 | 확대 screenshot을 기하 측정에 사용 | matrix 관측 뒤 대표 scene에만 CSS transform 2.5를 적용한다. README에서 화면을 계산 입력으로 쓰지 않는다고 밝힌다. |
| 28 | screenshot 크기·목적을 과장 | PNG는 1280×900이며 대표 Chrome fixture 화면이다. 앱, Simulator 또는 pixel parity라고 표기하지 않는다. |
| 29 | 재현 후 브라우저/시뮬레이터가 남음 | capture 스크립트는 `finally`에서 전용 Agent Browser session을 닫는다. 현재 프로세스 확인에서도 Agent Browser와 Simulator.app은 없고 기기는 모두 Shutdown이다. |
| 30 | 새 candidate 링크 또는 칸반 stage가 잘못 표시 | plan·precomparison 경로를 검토했고 roadmap의 static latest-merge note와 dynamic `detailNext`를 계획·비교 완료/구현 전으로 일치시켰다. |

## 결론

이 계획 PR 변경에서 차단 결함을 찾지 못했다. 이는 계획 diff의 근거·링크·상태 전이를 검토한 기록이며, 제품 구현/runtime review나 후속 구현 PR의 30관점 검토를 대신하지 않는다.
