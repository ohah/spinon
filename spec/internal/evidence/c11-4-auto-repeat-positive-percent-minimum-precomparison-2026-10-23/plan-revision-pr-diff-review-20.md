# C11.4 positive percentage auto-repeat minimum 계획 revision PR 변경 실패 관점 20개

이 revision PR의 계획·사전 비교 파일을 검토했다. 수정 계획 검토 20개와 독립이며 구현/runtime 리뷰가 아니다.

| # | diff 실패 관점 | 판정 |
|---:|---|---|
| 1 | 첫 계획이 아직 active scope처럼 보이는가? | 이전 계획에 PR #559 후 computed normalization으로 superseded됐다고 표시했다. 통과. |
| 2 | 실패 원인이 추측이나 Chrome 결과에서 왔는가? | Stylo `0.22.0` cascade assertion log에서 actual `Percentage(0.1)`을 보존했다. 통과. |
| 3 | 실패한 probe를 제품 회귀로 과장하는가? | log 설명은 예상 AST assertion이 틀렸다고 분리하고 exploratory test는 삭제했다. 통과. |
| 4 | 새 계획이 authored calc만 지원한다고 여전히 주장하는가? | direct percentage와 calc-normalized value가 같은 typed input임을 명시했다. 통과. |
| 5 | direct 10% acceptance가 계획에 숨겨져 있는가? | new scope, Chrome scenes, controls와 test gate에 직접 기록했다. 통과. |
| 6 | CSS Grid source가 해당 문법과 basis를 지지하는가? | §7.2.1과 §7.2.3.1/2 원문 링크를 Agent Browser에서 대조했다. 통과. |
| 7 | 10% expected 12px가 border box 124px로 계산되는가? | content-box 120px, outer border 124px를 JSON에 분리했다. 통과. |
| 8 | columns와 rows basis를 혼동하는가? | 각 축 content basis를 별도 고정하고 Chrome 16 scenes에서 기록했다. 통과. |
| 9 | border에 의해 item frame offset이 사라지는가? | 2px border와 item content-origin offset 2px를 observation에 보존했다. 통과. |
| 10 | auto-fill/auto-fit repeat count와 collapse가 혼합되는가? | 10 fill tracks와 `[120,0×9]` fit tracks를 각각 JSON에 고정했다. 통과. |
| 11 | 150% minimum의 fallback repeat count를 놓치는가? | 180px minimum과 120px content에서 positive count 1을 기록했다. 통과. |
| 12 | overflow가 border 내부 또는 screenshot 밖에서 끝나는가? | item frame은 outer border보다 58px 돌출하고 viewport 안에 전부 보인다. 통과. |
| 13 | overflow calc control이 다른 geometry를 쓰는가? | columns/rows 150% calc와 180px control의 tracks, frames, overflow bool이 일치한다. 통과. |
| 14 | 비교표가 16개 scene보다 작거나 빠진 조합이 있는가? | 10%의 12 scenes와 150% overflow의 4 scenes를 JSON parse로 확인한다. 통과. |
| 15 | Chrome build와 viewport를 재현할 수 없는가? | Google Chrome 156.0.8078.48, macOS, DPR 1, viewport 700×800을 기록했다. 통과. |
| 16 | 화면 제목·border·overflow가 겹쳐 식별되지 않는가? | Agent Browser 캡처에서 16개 제목, 2px border와 돌출 item을 확인했다. 통과. |
| 17 | 이전 계획 산출물이 변경돼 이미 병합된 evidence가 흔들리는가? | #559의 original fixture/evidence/hash를 그대로 두고 새 디렉터리를 추가했다. 통과. |
| 18 | 새 plan/README의 relative links가 끊기는가? | plan, evidence README와 fixture targets를 로컬 경로로 검사한다. 통과. |
| 19 | diff가 공식 status 완료 수나 내부 계약 버전을 움직이는가? | C11.4/`72/213`과 0059 `0.1.0`을 그대로 유지한다. 통과. |
| 20 | PR에 실패 test 또는 Cargo target/unrelated untracked files가 들어가는가? | revision diff는 계획·HTML oracle·evidence만 포함하고 기존 untracked 항목은 stage하지 않는다. 통과. |

20개 판정은 구현 코드나 simulator 성공 주장을 포함하지 않는다.
