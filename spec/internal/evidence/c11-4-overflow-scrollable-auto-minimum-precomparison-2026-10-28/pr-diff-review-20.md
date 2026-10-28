# C11.4 overflow automatic minimum 구현 PR diff 검토 · 20개 관점

**대상:** 같은 체크아웃에서 만든 구현 branch 전체 diff: `spinon-layout` test module, C11.4 plan 보정, Chrome fixture·expected·observation·screenshot·manifest, 계획 검토 후속 addendum 및 이 검토 기록.

**검토 방식:** 구현/runtime 20개 관점과 다른 PR 문서·범위·근거의 실패 경로를 확인했다. PR 본문은 구현 변경의 좁은 범위와 기준 화면 링크를 담고, GitHub check 상태가 조회되지 않으면 통과로 표현하지 않는다.

| # | PR diff에서 확인한 실패 경로 | 검토 결과와 증거 | 판정 |
| ---: | --- | --- | --- |
| 1 | 계획이 `minmax(0,1fr)`에서 존재하지 않는 Minimum trace를 기대 | plan, expected JSON, test가 query 없음으로 맞춰졌고 과거 기대값 실패를 addendum으로 보존했다. | 통과 |
| 2 | 계획의 Rows text 입력이 C15 guard를 실패시킴 | README와 plan이 replaced-image 경로 및 변경 이유를 명시한다. | 통과 |
| 3 | image auto width aspect ratio가 row 결과를 바꿈 | HTML/test의 10px width 입력과 구현 첫 실행의 발견을 기록했다. | 통과 |
| 4 | `<img>` 기본 `clip`이 `visible` 행 baseline에 숨어 있음 | HTML에서 `overflow:visible`을 명시하고 최종 Chrome computed observation에서 visible/clip을 구분한다. | 통과 |
| 5 | fixture, expected, observation의 case 수나 ID가 다름 | 검증 script가 14 IDs와 grid/track/item/overflow를 대조했다. | 통과 |
| 6 | fixture 입력이 expected CSS dimensions와 다름 | Rows는 30×5 Grid, 10px column, natural 10×10 image와 fixed width 10px이다. | 통과 |
| 7 | Chrome 결과를 독립 기대값으로 사용 | expected JSON은 W3C 조건과 고정 산술 입력을 담고 browser observation은 별도 파일이다. | 통과 |
| 8 | browser version, viewport, DPR 또는 Ahem 상태 누락 | Chrome 156, Agent Browser 0.31.1, `1280×633`, DPR 1, loaded Ahem metadata를 보존했다. | 통과 |
| 9 | evidence screenshot이 과거 fixture를 보여줌 | 최종 CSS 수정 뒤 screenshot을 다시 캡처하고 화면에서 14개 case를 확인했다. | 통과 |
| 10 | SHA-256 manifest가 이전 screenshot/관측값에 고정 | HTML, Ahem, observation, metadata, screenshot, expected, close record를 다시 해시했다. | 통과 |
| 11 | Agent Browser 프로세스가 열린 채 남음 | dedicated session 종료 후 session list empty, session `active:false`/`pid:null`, process scan empty를 저장했다. | 통과 |
| 12 | Terminal Browser evidence가 요청된 Agent Browser를 대신함 | 브라우저 기준은 Agent Browser만 사용했다. | 통과 |
| 13 | Rust test가 기본 product artifact에 fixture logic을 추가 | 새 module 선언은 `#[cfg(test)]` 아래에만 있다. runtime source와 FFI를 변경하지 않았다. | 통과 |
| 14 | 동일 stem child module이 다른 디렉터리 구조로 흩어짐 | parent `grid_auto_min_scrollable_overflow_tests.rs`와 child `.../row_axis_replaced.rs`를 같은 stem directory로 둔다. | 통과 |
| 15 | 테스트가 넘치거나 모듈 이름으로 책임 경계를 찾기 어려움 | 두 Rust 파일은 각각 430줄과 179줄이며 Columns text / Rows replaced 입력으로 나뉜다. | 통과 |
| 16 | 검토에서 simulator를 실행하지 않고 통과했다고 기록 | 변경이 unit-test only임을 plan, README, 본문 scope에 적고 simulator 미실행을 명시한다. | 통과 |
| 17 | 좁은 결과가 C11.4 공식 완료 체크/개수에 반영됨 | `spec/STATUS.md`와 API 명세는 수정하지 않았고 parent checkbox는 미완료다. | 통과 |
| 18 | PR 설명이 check rollup 부재를 CI pass로 바꿈 | PR 검토 시 latest `gh pr checks --required`, branch protection, mergeability를 확인하고 실제 상태만 보고한다. | 통과 |
| 19 | 사용자 파일·기존 untracked evidence가 PR에 포함됨 | stage할 경로는 task code/docs/evidence로 제한하고, 기존 untracked artifact는 그대로 둔다. | 통과 |
| 20 | merge 완료 전에 roadmap sync/상태 완료를 주장 | Rebase merge 결과(`state`, `mergedAt`, `mergeCommit`) 확인 후 main sync를 하고, C11.4 completion은 올리지 않는다. | 통과 |

**결론:** 이 diff에서 blocker 0개다. merge 전에는 GitHub 최신 check/merge state를 다시 읽으며, 빈 check rollup은 pass로 부르지 않는다. PR 외부 동작 여부는 이 test-only 변경으로 주장하지 않는다.
