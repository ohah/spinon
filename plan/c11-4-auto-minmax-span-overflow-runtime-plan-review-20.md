# C11.4 two-track span overflow runtime 후속 계획 · 실패 관점 20개

| # | 독립 계획 실패 경로 | 판정 |
|---:|---|---|
| 1 | PR #756의 pure Rust 결과를 앱 runtime 결과로 간주 | 통과 — 계획은 새 cascade부터 draw/present까지 별도 검증한다. |
| 2 | 기본 Grid track-sizing profile의 authored overflow allowlist를 무심코 확장 | 통과 — fixture-only profile/adapter 경계를 요구한다. |
| 3 | computed `overflow-x`가 layout adapter에서 항상 Visible로 바뀌는 현재 제한을 놓침 | 통과 — 현재 projection gate를 명시하고 새 경로에서 값을 보존한다. |
| 4 | authored overflow를 computed overflow와 같은 것으로 기록 | 통과 — x/y authored와 computed 값을 각각 검사한다. |
| 5 | 축 간 CSS computed-value 전환을 고려하지 않음 | 통과 — horizontal scrollable case에서 y가 auto가 되는지 검증한다. |
| 6 | item overflow 대신 Grid root overflow를 입력 | 통과 — item의 target-axis automatic minimum을 직접 고정한다. |
| 7 | min-content contribution과 automatic minimum을 혼합 | 통과 — 100px contribution과 source/value를 별도로 확인한다. |
| 8 | scrollable case에서 max-content contribution을 0으로 축소 | 통과 — 모든 case에서 100px max-content contribution을 요구한다. |
| 9 | visible/clip track 결과가 비대칭으로 승인 | 통과 — 각 track을 `[50,50]px`로 exact 비교한다. |
| 10 | hidden/auto/scroll 결과가 서로 다르게 허용됨 | 통과 — 각 track을 `[40,40]px`로 exact 비교한다. |
| 11 | root frame이나 item frame 누락 | 통과 — root `80×20px`와 예상 item frame을 모두 고정한다. |
| 12 | 1/64px tolerance가 0px 비교 조건을 약화 | 통과 — frame과 track은 zero CSS-pixel tolerance다. |
| 13 | Rows 입력까지 추가되어 scope가 커지거나 무관한 결과가 됨 | 통과 — five-case fixed Columns slice로 한정한다. |
| 14 | feature-off build에 fixture dispatch/marker가 포함 | 통과 — 기본 feature-off와 fixture-test positive control을 별도로 확인한다. |
| 15 | Android만 실행하고 두 플랫폼 지원으로 기록 | 통과 — API 37 ARM64 AVD와 iOS 26.2 Simulator 각각 5/5가 필요하다. |
| 16 | Simulator 결과를 실기기 결과로 서술 | 통과 — 실제 기기 검증을 주장하지 않는다. |
| 17 | draw/present를 하지 않은 trace-only 실행을 화면 검증으로 간주 | 통과 — draw/present와 geometry trace를 별도 요구한다. |
| 18 | clipping/scrollbar pixel 동작을 sizing fixture의 합격 조건으로 오인 | 통과 — paint, scroll, scrollbar는 명시적 비목표다. |
| 19 | 이 slice를 전체 CSS admission 또는 C11.4 완료로 승격 | 통과 — C11.4 미완료와 `72/213`을 유지한다. |
| 20 | 구현/runtime 검토와 계획 검토를 같은 artifact로 재사용 | 통과 — 계획 20, 구현/runtime 20, PR diff 30, post-merge 30을 분리한다. |

이 계획 검토는 범위·oracle·계산 경계·feature gate·platform evidence만 평가한다. 제품 동작 완료 판정은 하지 않는다.
