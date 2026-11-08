# C11.4 text-content plan PR · diff review 20

대상 diff: post-#1290 status 동기화, 새 bounded plan, plan review, Chrome precomparison fixture/810-scene evidence, public roadmap의 다음 slice 상태.

| # | 독립 검토 관점 | 확인 결과 | 판정 |
|---:|---|---|---|
| 1 | 이 PR에 제품 코드가 섞였는가 | Rust source/runtime 변경은 없고 계획·상태·fixture/oracle 자료만 있다. | 통과 |
| 2 | #1290 병합 SHA가 현 HEAD와 맞는가 | 계획과 공식 상태가 PR #1290의 `b9440dc…` merge SHA를 사용한다. | 통과 |
| 3 | C11.4를 공식 완료로 오인하게 하는가 | C11.4/C11 체크와 `78/214`를 그대로 둔다. | 통과 |
| 4 | 새 slice가 실제 제품 결손과 연결되는가 | #1290 exact payload allowlist와 `has_expected_text` 제한을 근거로 한다. | 통과 |
| 5 | text grammar의 byte/character 경계가 모호한가 | uppercase ASCII라 byte·scalar 수가 같고 전체 32 byte, 단어 1–6자를 따로 고정한다. | 통과 |
| 6 | 기존 동작이 새 oracle에서 누락됐는가 | `legacy-1290` corpus 270 scene을 회귀 기준으로 포함한다. | 통과 |
| 7 | 새로운 문자열이 exact allowlist 밖인가 | `varied-ascii`와 32자 경계 corpus가 기존 8문장과 다르다. | 통과 |
| 8 | 상한 입력이 실제로 32자인가 | expected JSON은 첫 `length-boundary-32` payload를 32자로 기록한다. | 통과 |
| 9 | 기대 JSON의 장면 수가 설명과 일치하는가 | 3 corpus × 270 scene = 810이며 각 그룹은 270이다. | 통과 |
| 10 | 30 track tail order가 빠졌는가 | 각 corpus의 220px cohort가 30 × 4 cap × 2 alignment, 100px cohort가 30이다. | 통과 |
| 11 | cap 또는 alignment 경계가 빠졌는가 | `20px`, `20%`, `0%`, `120%` 및 `start`, `stretch`를 전부 수집했다. | 통과 |
| 12 | overflow를 Grid frame만으로 판정하는가 | JSON의 payload frames로 Grid bottom 아래 범위를 검사한다. | 통과 |
| 13 | line count를 고정 문장별 높이로 오해하는가 | Range rect line counts를 관측하고 plan은 product의 높이 하드코딩을 금지한다. | 통과 |
| 14 | oracle이 fixture 변형과 섞였는가 | fixture SHA가 expected JSON과 README에 일치한다. 캡처 script의 세 corpus도 저장했다. | 통과 |
| 15 | Chrome 측정이 제품 또는 mobile 결과로 과장됐는가 | 문서가 Chrome oracle, 제품 matrix, OS visual smoke를 분리한다. | 통과 |
| 16 | 거부 검사가 바뀐 grammar 외 조건을 놓치는가 | source count, inline style, CSS style, topology, track/cap/alignment 거부와 recovery를 요구한다. | 통과 |
| 17 | plan 20과 PR diff 20을 같은 검토로 세는가 | 계획 검토와 이 diff 검토는 서로 다른 관점을 별도 파일로 둔다. | 통과 |
| 18 | 공개 칸반에 병합 완료 항목이 진행 중으로 남는가 | #1290은 과거 merge로 기록되고 새 grammar는 `detailNext`로 표시한다. | 통과 |
| 19 | roadmap 문구가 아직 없는 결과를 완료로 표시하는가 | 구현 전이며 plan PR merge 전인 점을 명시하고 product progress로 표시하지 않는다. | 통과 |
| 20 | PR 이후 Tailnet 상태를 이미 동기화했다고 오인시키는가 | 이 diff는 local source만 고치며 Tailnet sync는 merge 뒤 별도 확인한다. | 통과 |

**판정:** 계획 PR을 검토할 수 있다. 제품 구현·시뮬레이터 확인은 이 문서 diff의 증거로 간주하지 않는다.
