# C11.4 safe flex-start 계획 PR diff 실패 관점 검토 30개

**대상:** `align-content:safe|unsafe flex-start` 계획, Chrome 사전 비교·독립 기대값, 계획의 20개 실패 관점 검토, `safe|unsafe start` 병합 상태 및 후속 계획을 갱신한 상태/roadmap 문서 변경.

| # | 검토 관점 | 판정 |
|---:|---|---|
| 1 | C11 상위 완료 체크가 실수로 바뀌었는가 | 통과: C11/C11.4 미완료를 유지한다. |
| 2 | 공식 완료 집계가 바뀌었는가 | 통과: `78/214`를 유지한다. |
| 3 | 내부 계약 숫자 버전을 바꿨는가 | 통과: `0.1.0`을 유지한다. |
| 4 | PR #1254 병합 SHA가 원격 메타데이터와 맞는가 | 통과: GitHub `mergeCommit.oid`와 `d8c320a5267c9d739497b3b7233f541d97985c10`이 일치한다. |
| 5 | 병합 시각이 원격 기록과 맞는가 | 통과: `2026-11-07T05:08:55Z`를 기록한다. |
| 6 | 빈 PR check rollup을 CI green으로 과장하는가 | 통과: 빈 rollup이며 required checks 부재로 CI 통과라 하지 않는다고 쓴다. |
| 7 | safe-start 계획이 열린 PR 상태로 남았는가 | 통과: plan header에 병합 상태를 반영했다. |
| 8 | 공식 STATUS의 직전 작업 상태가 stale한가 | 통과: PR #1254 병합과 결과를 최신 순차 항목에 기록했다. |
| 9 | 내부 contract에 PR open 표현이 남았는가 | 통과: 병합 SHA 및 empty checks 문구로 갱신했다. |
| 10 | 공개 roadmap이 PR #1254 open이라 표시하는가 | 통과: 병합 상태와 다음 후보 링크로 갱신했다. |
| 11 | C11.4 parent plan이 이미 끝난 safe-start를 다시 다음으로 가리키는가 | 통과: safe flex-start 계획으로 전환했다. |
| 12 | successor plan 파일이 존재하지 않는 링크인가 | 통과: 새 plan과 20-point review가 생성됐다. |
| 13 | precomparison artifacts가 실제 입력/관측과 연결되는가 | 통과: HTML·observation·expected·comparison·screenshot을 같이 보존한다. |
| 14 | Chrome 버전이 고정/기록됐는가 | 통과: Agent Browser 관측은 HeadlessChrome 157.0.0.0이다. |
| 15 | scene 수와 비교 좌표 수가 모호한가 | 통과: 6 scenes, 72 item-frame coordinates를 기계 판독 결과에 둔다. |
| 16 | positive input의 free-space 계산이 맞는가 | 통과: 세 10px tracks와 40px box에서 10px free space를 기록한다. |
| 17 | overflow row-gap 산식이 맞는가 | 통과: definite 40px basis에서 46px gutters와 122px subject를 쓴다. |
| 18 | expected geometry가 browser 수치와 연결되는가 | 통과: 1/64 CSS px comparator는 0px max error로 끝난다. |
| 19 | computed CSS value identity가 유지되는가 | 통과: 두 safety modifier serialization을 각각 관측한다. |
| 20 | flex-start의 non-flex semantics를 근거 없이 주장하는가 | 통과: W3C CSS Box Alignment §4.1을 연결했다. |
| 21 | safe overflow fallback을 잘못 기술하는가 | 통과: §4.4의 flex-start fallback과 start geometry를 분리해 적었다. |
| 22 | unsafe modifier 의미를 누락하는가 | 통과: specified alignment를 그대로 유지한다고 기록한다. |
| 23 | Working Draft의 at-risk 상태를 숨기는가 | 통과: overflow-position at-risk 한계를 쓴다. |
| 24 | 허용범위가 arbitrary profile로 퍼지는가 | 통과: exact RootHtmlAutoRowStretchV1 fixed profile로 제한한다. |
| 25 | safe flex-end와 baseline negative controls가 빠졌는가 | 통과: 계속 거부할 값을 계획에 고정했다. |
| 26 | projection/layout allowlist parity가 누락되는가 | 통과: exact same values와 tests를 요구한다. |
| 27 | runtime partial publish/회복 게이트가 빠졌는가 | 통과: atomic rejection과 same-session recovery를 요구한다. |
| 28 | Android/iOS 실행을 실제 완료로 과장하는가 | 통과: 새 plan은 구현 전이며 Simulator gates만 향후 acceptance로 기록한다. |
| 29 | OS screenshot을 pixel readback으로 과장하는가 | 통과: screenshot을 visual bounds로 제한한다. |
| 30 | 계획 PR 검토와 구현/runtime/후속 PR 검토를 합산하는가 | 통과: 각 리뷰가 별도 gate이며 계획 리뷰가 구현 검토를 대체하지 않는다고 명시한다. |

이 검토는 계획·문서 PR diff만 다룬다. 구현/runtime 20개 및 구현 PR diff 30개는 구현 이후 별도로 수행해야 한다.
