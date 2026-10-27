# C11.4 generic mixed text/image row-axis · 계획 PR diff 검토 30

검토 대상 staged diff: row-axis 계획, 계획 실패 관점 검토, 상위/선행 계획 링크, Agent Browser 재관측 및 독립 기대값 evidence.

범위: 실제 계획 PR diff의 내용·참조·증거 경계를 확인했다. 이 문서는 계획 자체의 20관점 검토와 별개다. 구현, 테스트, 빌드, Simulator 실행은 검토 통과로 간주하지 않았다.

| # | PR diff 실패 관점 | 확인 및 판정 |
|---:|---|---|
| 1 | 변경 파일에 구현 코드가 섞임 | staged diff는 계획·검토 문서와 사전 비교 evidence만 포함한다. PASS |
| 2 | 상위 C11.4 계획이 새 slice를 완료로 표시함 | 상위 계획은 generic row-axis 후속을 구현 전이라고 기록하고 상위 체크와 `72/213`을 유지한다. PASS |
| 3 | 선행 PR #710 범위 설명이 과장됨 | PR #710은 novel HostNode IDs의 direct mixed column 네 장면이라고 제한한다. PASS |
| 4 | 기존 fixed-ID row 구현이 없는 것처럼 기록됨 | 새 계획은 fixed-ID row slice와 근거 문서를 별도 선행 구현으로 연결한다. PASS |
| 5 | 계획 파일의 상대 링크가 잘못됨 | 상위 계획, 내부 계약, STATUS, 선행 계획 및 새 evidence 경로를 저장소에서 확인했다. PASS |
| 6 | 계획 검토 파일과 계획 본문이 서로 연결되지 않음 | 계획은 plan review 20과 실제 계획 PR diff review 30을 별개 기록으로 둔다. PASS |
| 7 | precomparison README가 원본 HTML을 재생성했다고 주장함 | README는 기존 fixture를 재관측한 것으로 기술하고 원본 HTML SHA를 보존한다. PASS |
| 8 | Browser 관측을 Spinon 실행으로 보고함 | Browser README는 Chrome reference이며 runtime·Simulator 결과가 아니라고 명시한다. PASS |
| 9 | 브라우저 종류와 실행 버전이 빠져 재현 범위가 모호함 | JSON은 Agent Browser CLI 0.31.1과 HeadlessChrome 156 user agent를 기록한다. PASS |
| 10 | viewport나 device scale 차이로 frame 수치가 오해됨 | 재관측 JSON은 1280×633, scale factor 1을 기록한다. PASS |
| 11 | font 미적재로 text line-box 기대가 불확실함 | JSON은 Ahem loaded를 기록하고 기대값은 별도 JSON에서 50px column의 두 10px line box로 계산한다. PASS |
| 12 | 이미지가 로드되지 않았는데 natural size를 전제로 함 | JSON은 모든 image decode와 24×16 natural size를 기록한다. PASS |
| 13 | DOM source order가 예상과 다름 | 네 장면 각각 text→image 또는 image→text 순서를 명시하고 재관측과 맞는다. PASS |
| 14 | viewport root 좌표를 runtime root-relative frame으로 복사함 | 관측 JSON은 page frame과 root-relative child frames를 별도 필드로 나눈다. PASS |
| 15 | 4개 장면 중 일부 root가 비교에서 누락됨 | Agent Browser JSON과 expected JSON 모두 min/max forward/reversed 네 장면을 포함한다. PASS |
| 16 | 실제 row geometry가 텍스트 단독 높이와 다름 | 독립 산술 `max(20,16)=20`과 Chrome 20px row 관측을 나눠 기록한다. PASS |
| 17 | min-content와 max-content가 이 테스트에서 구별되는 것처럼 표기됨 | plan, README, expected JSON이 네 결과가 같아 algorithm 차이를 입증하지 못한다고 명시한다. PASS |
| 18 | absolute track probe를 sizing child로 셈 | README·JSON·plan은 probe가 out-of-flow이며 runtime HostDocument에서 제외됨을 기록한다. PASS |
| 19 | alignment author style이 oracle과 다름 | JSON은 root align-items `start`, root justify-items `normal`, item self-alignment `auto`를 기록하고 계획의 조건과 맞는다. PASS |
| 20 | row input의 root width와 columns 합이 모순됨 | 고정 columns `50+24`와 root width `74`가 plan·expected JSON·관측과 일치한다. PASS |
| 21 | text와 image frame이 track/column 위치와 모순됨 | text `[0,0,50,20]`, image `[50,0,24,16]`, root `[0,0,74,20]`이 양 evidence 간 일치한다. PASS |
| 22 | JSON artifact 문법 오류로 기계적 소비가 불가능함 | Agent Browser observation과 expected JSON은 valid JSON으로 파싱된다. PASS |
| 23 | 기존 PNG나 screenshot을 불필요하게 중복 저장함 | 새 evidence는 관측 JSON/기대 JSON/README만 추가하고 기존 screenshot hash와 참조를 재사용한다. PASS |
| 24 | 외부 normative reference와 용어 참고 문서의 위상이 혼동됨 | CSS Grid 2025 CRD를 산술 근거로, CSS Box Sizing 2026 WD를 용어 참고로 구분한다. PASS |
| 25 | W3C 링크가 로컬 변경과 무관한 장식으로 포함됨 | 명세 링크는 intrinsic track contribution 규칙과 contribution 용어 설명에 각각 연결된다. PASS |
| 26 | fail-closed 요구가 generic success criteria와 충돌함 | text/source/topology/placement/profile/revision 변형을 error로 분류하고 혼합 아닌 root는 fallback을 유지한다. PASS |
| 27 | 기존 fixed-ID 및 generic column route 보호가 빠짐 | 계획은 known row IDs의 static route와 PR #710 column scenes의 비포획 회귀를 요구한다. PASS |
| 28 | 양 플랫폼 검증 scene 수가 계산과 맞지 않음 | 4개 그룹 × 4개 scene으로 플랫폼별 16개이며 plan은 그룹을 열거한다. PASS |
| 29 | 실행·종료 경계가 simulator나 브라우저 전체 종료로 확대됨 | 계획은 직접 실행한 app process와 Agent Browser 세션/process 종료를 구분하고 Simulator 종료를 요구하지 않는다. PASS |
| 30 | merge 이후 공식 상태와 CI 상태가 과장됨 | plan은 실제 merge SHA/check rollup 확인, empty rollup의 CI 미통과 처리, roadmap sync, C11.4 및 `72/213` 유지 조건을 포함한다. PASS |

## 판정

**PASS 30 / PARTIAL 0 / FAIL 0.** 실제 계획 PR diff는 명시 범위, oracle/evidence provenance, 고정 기대값, 기존 route 보호와 향후 검증 한계를 서로 맞춰 기록한다. 이 판정은 계획 문서 변경에만 적용된다.
