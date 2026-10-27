# PR #673 공개 roadmap/status 동기화 변경 검토 · 30관점

**검토 대상:** PR #673의 병합 기록을 `spec/STATUS.md`와 `spec/public/roadmap.html`에 반영하고, local preview screenshot 및 provenance를 추가한 문서 diff. Reviewer는 코드를 수정하거나 test/browser/simulator를 실행하지 않았다. 검토 시점에는 문서 PR #674가 아직 생성되지 않아 PR 본문 image attachment만 생성 후 확인할 항목으로 남겼다.

## 집계

**30 PASS / 0 PARTIAL / 0 FAIL.** 초기 검토 시에는 PR #674 본문 이미지 확인이 대기였으나, PR 생성 뒤 Agent Browser에서 이미지가 정상 로드되는 것을 확인했다.

| # | 실패 관점 | 판정 | 근거 |
|---:|---|---|---|
| 1 | 기준 revision | PASS | 변경 base `83653cd02835d05bbdf455888dde7cc78f28dfa6`는 #673 merge SHA와 같다. |
| 2 | 변경 범위 격리 | PASS | STATUS, public roadmap, local preview evidence만 바꾼다. |
| 3 | staged diff 완전성 | PASS | PR 초기 7개 경로에 post-create verification JSON 경로 1개를 더해 전체 8개가 됐다. follow-up commit은 기존 3개 파일 수정과 새 JSON 추가로 구성되고, 4개 staged file의 cached diff check가 통과한 뒤 push됐다. |
| 4 | C11.4 공식 checkbox | PASS | `spec/STATUS.md`에서 미완료로 유지한다. |
| 5 | 공식 완료 집계 | PASS | STATUS와 public roadmap 모두 `72/213`을 유지한다. |
| 6 | PR #673 병합 상태 | PASS | GitHub PR 상태는 MERGED다. |
| 7 | PR #673 병합 시각 | PASS | `2026-10-27T00:34:44Z`가 GitHub merge metadata와 일치한다. |
| 8 | PR #673 merge SHA | PASS | `83653cd02835d05bbdf455888dde7cc78f28dfa6`이 GitHub merge SHA와 일치한다. |
| 9 | GitHub check rollup | PASS | PR rollup은 비어 있고 문서는 이를 CI pass라 하지 않는다. |
| 10 | main required checks | PASS | API가 required status checks not enabled(404)로 응답하며 문서 문구가 맞다. |
| 11 | test-only 변경 범위 | PASS | 구현은 Rust trace test-only 변경으로 설명한다. |
| 12 | runtime/simulator 과장 방지 | PASS | 앱 runtime 및 Android/iOS simulator 검증을 명시적으로 범위 밖으로 둔다. |
| 13 | 고정 scene 수 | PASS | 네 forward/reversed scene 설명이 plan 및 precomparison과 일치한다. |
| 14 | span-2 기대 row | PASS | `[40,60,60,0]px`가 기존 oracle과 일치한다. |
| 15 | span-2/3 기대 row | PASS | `[160/3,220/3,220/3,0]px`가 독립 기대값과 일치한다. |
| 16 | 규범 기대와 Chrome 관찰 구분 | PASS | Chrome은 final geometry/frame control로만 설명한다. |
| 17 | fixed Columns 대조군 | PASS | 220px early-return track 기록이 구현 evidence와 일치한다. |
| 18 | Rows stage order | PASS | 전체 stage sequence 검증을 과장 없이 요약한다. |
| 19 | source order controls | PASS | 네 고정 scene과 forward/reversed 검사가 일치한다. |
| 20 | focused Rust test 결과 | PASS | validation manifest는 3 passed, 0 failed를 기록한다. |
| 21 | fmt/Clippy 결과 | PASS | 명령 결과가 validation manifest에 있다. |
| 22 | 구현 검토 결과 | PASS | 19 PASS / 1 PARTIAL이 구현 review와 일치한다. |
| 23 | 구현 review PARTIAL | PASS | test constants와 checked-in JSON의 기계적 동기화 한계를 공개한다. |
| 24 | PR diff review 결과 | PASS | 27 PASS / 3 PARTIAL / 0 FAIL을 연결된 report에서 확인한다. |
| 25 | 잔여 PR diff 한계 | PASS | 공개 기록은 잔여 PARTIAL을 숨기거나 CI 성과로 바꾸지 않는다. |
| 26 | STATUS 내부 링크 | PASS | plan, evidence, review, validation 및 preview README 대상이 존재한다. |
| 27 | public roadmap 배치·내용 | PASS | #673 기록이 앞선 #671 계획 뒤에 추가되고 완료 수는 유지된다. |
| 28 | screenshot 내용 | PASS | 이미지에는 새 #673 roadmap 항목이 보이며 1280×633 local screenshot이다. |
| 29 | screenshot provenance/hash | PASS | Agent Browser, local URL, `live_tailnet:false`, source/image hashes가 기록되고 3 artifact checksum이 통과했다. |
| 30 | PR 본문 screenshot attachment | PASS | PR #674 본문에서 이미지가 `complete=true`, 1280×633으로 로드됐다. raw URL·alt text와 browser 종료 결과는 [pr-body-verification.json](./pr-body-verification.json)에 기록했다. |

## 생성 후 재검토

- 관점 30은 PR #674 생성 뒤 Agent Browser에서 본문 이미지 로드를 확인해 PASS로 갱신했다.
- 이 screenshot은 문서 PR worktree의 local preview다. Tailnet live route는 PR merge 후 별도 검증한다.
