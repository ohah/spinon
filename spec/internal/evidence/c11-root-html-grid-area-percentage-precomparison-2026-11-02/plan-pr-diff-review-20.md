# C11.1 named areas × percentage plan PR diff · 20개 적대 검토

범위는 계획 PR의 parent plan index, 신규 Root HTML feature plan, 그 계획의 독립 20-point review, Chrome 156 precomparison inputs/outputs/screenshots/hashes다. 구현 코드·public CSS admission·공식 완료 상태는 변경하지 않는다.

| # | 별도 diff 실패 경로 | 대조 결과 |
|---:|---|---|
| 1 | parent plan link가 없는 plan만 추가됨 | `plan/c11-css-grid.md`에서 계획과 사전 비교를 연결했다. PASS. |
| 2 | 계획이 구현됐다고 오해될 수 있음 | plan summary에 Chrome precomparison 완료, implementation 미착수라고 분리했다. PASS. |
| 3 | status/official count가 계획 PR에서 승격될 수 있음 | `spec/STATUS.md`를 수정하지 않고 C11 status와 `72/213` 유지 문구를 명시했다. PASS. |
| 4 | 사전 비교 fixture가 구현 code와 혼합됨 | Chrome inputs/evidence는 별도 evidence directory에만 있다. PASS. |
| 5 | 실제 input과 independent calculation source hash가 불명확함 | 각 source, observation, expected JSON과 screenshot SHA-256 manifest를 추가했다. PASS. |
| 6 | Chrome 값으로 독립 기대값을 역산함 | `expected.json`에 간단한 axis multiplication을 따로 기록하고 README가 source 분리를 설명한다. PASS. |
| 7 | screenshot이 Spinon runtime으로 오인될 수 있음 | README는 Chrome을 independent oracle로 분류한다. PASS. |
| 8 | browser observation이 screenshot만 있고 DOM geometry가 빠짐 | observation JSON에 computed tracks, root/item DOMRects, area names를 넣었다. PASS. |
| 9 | viewport/DPR가 capture 재현에서 빠짐 | observation에 1280×633 CSS px와 DPR 1을 고정했다. PASS. |
| 10 | source order 때문에 semantic item mapping이 우연히 맞을 수 있음 | 모든 page가 `content/header/side` 순서로 DOM을 선언한다. PASS. |
| 11 | columns와 rows percentage가 동일 basis로 기록됨 | expected/observation이 columns와 rows를 별도 lists와 axis별 계산으로 기록한다. PASS. |
| 12 | overflow screenshot/geometry가 빠짐 | over-100 case는 dedicated source, screenshot, item frame, total scroll width를 제공한다. PASS. |
| 13 | 0% control이 source-only 주장에 그침 | Chrome CSSOM 및 zero-width side frame을 관찰 JSON에 포함했다. PASS. |
| 14 | existing regression links가 누락됨 | plan은 fixed-px and fr area paths를 regression controls로 연결한다. PASS. |
| 15 | 계획에 신규 `% + fr` 지원이 몰래 포함됨 | plan은 named-area `% + fr` combination을 out-of-scope로 고정했다. PASS. |
| 16 | negative input criteria가 없는 happy-path plan임 | indefinite, range, computed sizing override, grammar, area topology와 stale revision rejection을 명시했다. PASS. |
| 17 | simulator capture가 exact pixel pass로 과장될 수 있음 | plan과 PR body는 screenshots를 visual smoke로만 취급한다. PASS. |
| 18 | implementation validation gate가 빠짐 | focused tests, regressions, Clippy, formatting, both simulators, archive boundary가 별도 관문이다. PASS. |
| 19 | plan review가 PR diff review로 재사용됨 | 별도 plan review 20개와 이 PR diff review 20개를 분리했다. PASS. |
| 20 | local links나 SHA manifest가 깨졌을 수 있음 | PR 작성 전 local link validator와 `shasum -c` 실행을 통과해야 하며 최종 diff에서 다시 확인한다. PASS. |

결과: 20/20 PASS. 이 표는 plan-only diff를 대상으로 하며 implementation review가 아니다.
