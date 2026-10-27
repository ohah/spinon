# C11.4 generic mixed text/image row-axis · 계획 실패 관점 검토 20

검토 대상: [generic row-axis 계획](./c11-4-generic-mixed-direct-text-image-row-axis.md)와 연결된 사전 비교 evidence.

범위: 구현 전 계획의 oracle, admission 계약, 기존 route 경계, 실행·증거 한계를 검토했다. 이 검토에서는 구현, 테스트, 빌드, Simulator 실행을 하지 않았다.

| # | 서로 다른 실패 관점 | 확인 및 판정 |
|---:|---|---|
| 1 | 제한된 fixture slice가 C11.4 전체 완료로 보고됨 | 계획 candidate 상태, 상위 checkbox 미완료와 공식 `72/213` 유지가 명시됐다. PASS |
| 2 | 후속의 중복·선행 관계가 잘못 표기됨 | PR #710 generic column 구현과 fixed-ID row slice를 별도 선행 항목으로 링크했다. PASS |
| 3 | 새 adapter가 이미 genericized된 column 범위를 다시 구현함 | 대상은 두 child가 같은 intrinsic row에 놓이는 row-axis contribution과 generic routing으로 한정한다. PASS |
| 4 | novel ID 지원이 기대값 선택에서 ID 분기로 되살아남 | 새 root/item HostNode ID를 사용하며 selection과 expectations가 ID allowlist에 의존하지 않도록 했다. PASS |
| 5 | 기존 fixed-ID row scene이 generic 경로에 가로채임 | 기존 고정 scene route를 먼저 유지하고 known IDs는 static adapter가 계속 처리하도록 했다. PASS |
| 6 | PR #710 generic column scenes가 row adapter에 잘못 잡힘 | 후속 검증은 새 row topology로 제한하고 generic column scenes가 새 adapter에 잡히지 않는 회귀 조건을 포함한다. PASS |
| 7 | source order와 grid placement를 혼동함 | forward/reversed는 sizing child DOM 순서만 바꾸며 text/image column placement와 기대 frame은 고정한다. PASS |
| 8 | text line-wrap 계산이 Chrome 결과에서 순환 도출됨 | 50px column, Ahem 10px glyph/line-height로 두 line box, 즉 20px를 독립 계산한다. PASS |
| 9 | PNG natural size 또는 contribution이 잘못 적용됨 | 고정 PNG natural size 24×16과 row contribution 16px를 구분해 적었다. PASS |
| 10 | row track 산술이 contribution 최댓값 규칙을 놓침 | 독립 기대값을 `max(20,16)=20px`로 명시하고 CSS Grid §11.5에 연결했다. PASS |
| 11 | min-content/max-content 결과가 같다는 한계가 감춰짐 | 네 scene 모두 20px이고 이 matrix로 두 algorithm 차이를 입증하지 못한다고 명시했다. PASS |
| 12 | 측정용 absolute probe가 runtime sizing item으로 들어감 | `.track-probe`는 HTML oracle 전용 out-of-flow 도구이며 runtime HostDocument에 포함하지 않는다고 제한했다. PASS |
| 13 | viewport 좌표와 runtime root-relative frame이 섞임 | oracle page의 네 root 위치와 runtime에서 비교할 root-relative item frame을 구분한다. PASS |
| 14 | alignment defaults가 oracle과 다르게 authored됨 | root `align-items:start`, 나머지 관련 alignment initial `normal`, item self-alignment `auto` 조건이 구체적이다. PASS |
| 15 | child topology가 nested/추가 item을 허용함 | 정확히 두 direct HTML elements, 단일 text node의 div와 decoded leaf img만 허용하고 변형은 거부한다. PASS |
| 16 | track 함수·수·placement 변형이 허용됨 | 두 columns와 하나의 min/max row, 같은 row, 고정 column placement 및 writing/direction/profile 제약을 fail-closed 조건으로 두었다. PASS |
| 17 | stale document/style/image revision이 한 계산에 혼합됨 | document snapshot, computed-style revision, intrinsic image snapshot의 일관성과 동일 layout revision trace를 검증하도록 했다. PASS |
| 18 | invalid mixed input이 text-only/image-only fallback으로 성공 처리됨 | signature가 잡힌 invalid mixed input은 구체적 error로 끝내고 partial result로 낮추지 않도록 했다. PASS |
| 19 | runtime·FFI scene ID와 HostNode ID가 같은 경계로 취급됨 | route scene ID는 분리하고 Android/iOS matcher가 novel node identities와 source/order/trace/frame을 확인한다. PASS |
| 20 | simulator·검토·merge evidence 범위가 과장됨 | 양 플랫폼 16개 matrix, fixture/product gate, 구현/runtime 20관점, 별도 PR diff 30관점, Agent Browser 이미지 확인과 종료, merge/check 및 roadmap 후속을 적고 물리기기·pixel 등 비목표를 분리했다. PASS |

## 판정

**PASS 20 / PARTIAL 0 / FAIL 0.** 계획은 independent row oracle, identity-independent routing 계약, 기존 fallback 보존과 실행 증거 경계를 구분한다. 이는 계획 검토이며 구현 검증을 대신하지 않는다. 실제 계획 PR diff는 별도 30관점으로 검토한다.
