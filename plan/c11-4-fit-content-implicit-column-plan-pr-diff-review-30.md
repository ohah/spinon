# C11.4 fit-content implicit column 계획 PR diff 적대 검토 30개

대상 변경: `plan/c11-4-fit-content-implicit-column-wpt.md`, 이 계획 검토, pinned Chrome 사전 비교 artifacts.

| # | 변경에서 확인한 실패 경로 | 판정 |
|---:|---|---|
| 1 | WPT repository가 CSS Grid oracle의 primary source인지 | PASS — official WPT source path와 revision을 기록했다. |
| 2 | source pin이 target assertion을 정확히 가리키는지 | PASS — markup 64–66행, assertion 122–124행이다. |
| 3 | pinned WPT SHA-256이 fixture를 식별하는지 | PASS — source pin과 README의 hash가 일치한다. |
| 4 | live WPT 응답이 같은 revision 내용인지 | PASS — Agent Browser fetch hash가 pinned file과 같다. |
| 5 | browser metadata가 실제 실행 정보인지 | PASS — HeadlessChrome 156, viewport, DPR 및 13/13 harness 결과를 보존했다. |
| 6 | Chrome WPT와 Spinon 결과를 혼동하는 표현이 있는지 | PASS — evidence는 Chrome baseline이며 Spinon WPT pass가 아니라고 명시했다. |
| 7 | CSS input이 target WPT declaration과 어긋나는지 | PASS — size/placement/track declarations를 정리해 명시했다. |
| 8 | existing generic implicit-fit-content fixture를 새 결과로 중복 집계하는지 | PASS — 기존 root/flow/text/cap 차이를 구체적으로 구분했다. |
| 9 | expected JSON이 root dimensions를 잘못 분류하는지 | PASS — border/padding 없는 fixed 300×300 content size로 고정했다. |
| 10 | explicit track과 implicit track 순서가 뒤집히는지 | PASS — `[60,40]` order 및 source expectation을 비교했다. |
| 11 | computed `grid-auto-columns`와 resolved columns를 혼동하는지 | PASS — authored property와 resolved CSSOM string을 별도 필드로 기록했다. |
| 12 | item minimum contribution과 max-content contribution이 섞이는지 | PASS — 0px과 400px을 separate normative inputs로 썼다. |
| 13 | fit-content argument가 content minimum을 clamp한다고 잘못 주장하는지 | PASS — §7.2.1과 §11.5에 따라 minimum과 growth cap을 분리했다. |
| 14 | max-content growth limit이 argument보다 커지는지 | PASS — 40px clamp를 explicit expected field로 기록했다. |
| 15 | free-space distribution이 auto-track stretch 대상과 혼동되는지 | PASS — §11.8 auto-track eligibility를 계획에 인용했다. |
| 16 | implicit row의 expected size가 missing인지 | PASS — cross-axis row `10px` 및 item/child height 10px을 고정했다. |
| 17 | item placement의 auto end가 arbitrary span으로 바뀌는지 | PASS — `grid-column:2` 그대로 두고 second-column area 검사를 요구했다. |
| 18 | row auto-placement가 extra columns를 만드는지 | PASS — single item, row flow, exact 3-node tree를 요구한다. |
| 19 | child width overflow가 root boundary 밖에 있는지 | PASS — 400px child endpoint 460px, root endpoint 300px로 산술 검증했다. |
| 20 | overflow sample 위치가 host viewport 밖으로 나가는지 | PASS — 480×320 CSS viewport와 x=350.5 target sample을 요구한다. |
| 21 | outside-root pixel이 surface clear와 구분되는지 | PASS — child sample 및 x=470.5 surface-clear sample을 각각 지정했다. |
| 22 | screenshot overlay가 raw WPT input으로 오해되는지 | PASS — overlay stage와 geometry evidence boundary를 README에 썼다. |
| 23 | pixel sample color가 diagnostic CSS와 일치하는지 | PASS — root/item/child 색과 expected sample roles를 plan에 고정했다. |
| 24 | renderer trace from stale revisions can pass | PASS — same source/style/environment revision requirement is explicit. |
| 25 | malformed or extra source tree publishes partial output | PASS — negative controls and no-partial-output criterion are required. |
| 26 | product artifact fixture exclusion is assumed from source paths | PASS — default negative and opt-in positive archive checks are explicit. |
| 27 | test-only fixture is mistaken for general CSS author admission | PASS — CSS grammar, runtime profile, public API remain fixed. |
| 28 | internal contract version is bumped without need | PASS — `0.1.0` stays unchanged. |
| 29 | C11.4 completion count is incremented after a single WPT assertion | PASS — checkbox and `72/213` are explicitly retained. |
| 30 | evidence links, paths, and file names refer to distinct artifacts | PASS — source pins, expected JSON, browser capture, README, plan, and reviews cross-link by relative paths. |

**판정:** 계획 변경 diff 30/30 PASS. PR merge 전 Markdown link validation and docs build remain required.
