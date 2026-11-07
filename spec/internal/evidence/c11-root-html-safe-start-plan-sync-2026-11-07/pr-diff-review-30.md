# C11.4 safe-end 병합 상태·safe-start 계획 동기화 PR diff 실패 관점 검토 30개

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | C11 상위 체크가 완료로 승격되는가 | 통과 — 체크를 그대로 미완료로 기록한다. |
| 2 | C11.4 체크가 safe-end slice 병합으로 완료 처리되는가 | 통과 — C11.4도 미완료다. |
| 3 | 공식 완료 counter가 잘못 변하는가 | 통과 — 공식 `78/214`를 유지한다. |
| 4 | PR #1252 상태가 여전히 open으로 표시되는가 | 통과 — merged 상태와 merge timestamp를 반영한다. |
| 5 | merge 방식과 SHA가 실제 GitHub 결과와 다른가 | 통과 — Rebase and merge 및 `40571673ca9e812683f4fc4410c6a2e0891d54d1`을 기록한다. |
| 6 | C11.4 safe-end의 좁은 profile 경계가 넓어지는가 | 통과 — fixed `RootHtmlAutoRowStretchV1` 한정이다. |
| 7 | simulator 검증이 hardware device로 과장되는가 | 통과 — Android API 37 AVD/iPhone 17 Pro Simulator만 구분해 적는다. |
| 8 | screenshot이 PixelCopy/Metal readback으로 과장되는가 | 통과 — OS screenshot visual bounds 한계를 유지한다. |
| 9 | CI가 실행되지 않았는데 green으로 기록되는가 | 통과 — PR check/workflow rollup이 비어 있음을 유지한다. |
| 10 | `safe|unsafe start`가 이미 구현된 것으로 표시되는가 | 통과 — 다음 계획 후보이며 구현 미시작임을 명시한다. |
| 11 | Agent Browser 관측을 Spinon 동작 증명으로 오해하는가 | 통과 — Chrome oracle precomparison으로 구분한다. |
| 12 | computed alignment serialization을 보존하지 않는가 | 통과 — `start`, `safe start`, `unsafe start`를 각각 기록한다. |
| 13 | positive-space 예상 y 좌표가 잘못되는가 | 통과 — `[0,10,20]px`를 기대값과 대조한다. |
| 14 | overflow 예상 y 좌표가 잘못되는가 | 통과 — `[0,56,112]px`를 기대값과 대조한다. |
| 15 | used auto-row sizes가 빠지는가 | 통과 — `[10,10,10]px`를 모든 장면에서 확인한다. |
| 16 | percentage `row-gap` basis가 불확정한가 | 통과 — definite 40px block-size와 `calc(120% - 2px)`를 고정한다. |
| 17 | item count, order 또는 size가 변하는가 | 통과 — 3개 item의 40×10px frame 전체를 비교한다. |
| 18 | 비교 tolerance가 관측 결과에 맞춰 사후 완화되는가 | 통과 — 1/64 CSS px를 사전에 고정하고 실제 최대 오차는 0이다. |
| 19 | scene 수와 좌표 수가 서로 모순되는가 | 통과 — 6 scene, 3 item × 4 coordinate로 72 값을 비교한다. |
| 20 | negative control인 plain start가 빠지는가 | 통과 — positive/overflow에서 start를 baseline으로 둔다. |
| 21 | safe/unsafe end와 start semantics를 혼동하는가 | 통과 — safe-start 계획의 start geometry와 기존 unsafe-end overflow는 별개다. |
| 22 | scroll-container safety/default behavior가 포함된 것으로 보이는가 | 통과 — 그 동작은 계획 범위 밖이다. |
| 23 | writing mode/direction 범위를 확대하는가 | 통과 — fixed horizontal-tb/LTR shape로 제한한다. |
| 24 | 일반 author CSS admission이나 WPT 결과를 주장하는가 | 통과 — 둘 다 미포함이다. |
| 25 | 내부 contract version이 불필요하게 올라가는가 | 통과 — `0.1.0` 유지다. |
| 26 | stale roadmap 카드가 safe-end를 아직 다음 후보로 가리키는가 | 통과 — 다음 계획 링크는 safe-start로 갱신한다. |
| 27 | merged safe-end evidence 링크가 사라지는가 | 통과 — plan, contract, STATUS와 public roadmap에서 보존한다. |
| 28 | precomparison fixture 또는 원본 hash를 잃는가 | 통과 — fixture, observation, expected, comparison과 PNG를 유지한다. |
| 29 | Agent Browser 프로세스가 종료되지 않은 채 남는가 | 통과 — session close 뒤 관련 process 조회 결과가 비어 있다. |
| 30 | post-merge 동기화 PR을 구현 PR 또는 C11 완료로 오해하는가 | 통과 — 이 PR은 상태·계획 문서만 바꾼다. |

검토 범위는 이 status/roadmap/next-plan PR diff다. safe-start의 Rust 구현·RuntimeSession·Android/iOS Simulator는 수행하지 않았고 통과로 주장하지 않는다.
