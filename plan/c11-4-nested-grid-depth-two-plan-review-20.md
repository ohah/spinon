# C11.4 두 단계 nested Grid 계획 실패 관점 검토 20개

| # | 독립 실패 관점 | 확인과 판정 |
| --- | --- | --- |
| 1 | test input이 두 단계 nesting이 아니라 단지 child 하나를 더 둔 평면 구조인가? | Root → middle Grid → inner Grid → text의 owner topology와 각 Grid ID를 fixture/expected JSON에 분리했다. 계획은 depth-one control과 depth-two scene을 비교한다. 통과. |
| 2 | depth-one control이 없어 추가된 middle Grid의 효과를 분리하지 못하는가? | 동일 text·track function·constraint의 depth-one scene을 min/max 각각 둔다. 통과. |
| 3 | min-content 전달만 확인하고 max-content 경로를 놓치는가? | 네 scene은 min/max constraint 각각 depth 1/2를 모두 고정한다. 통과. |
| 4 | CSS width를 definite px로 고정해 intrinsic propagation을 우회하는가? | Root와 모든 child Grid의 width는 대응하는 `min-content` 또는 `max-content`다. 고정 px width는 없다. 통과. |
| 5 | Grid track이 content와 무관한 고정 폭이라 contribution을 관찰하지 못하는가? | 각 Grid는 해당 constraint와 같은 단일 intrinsic column track을 사용한다. 통과. |
| 6 | 독립 기대값이 Chrome 결과를 복사해 oracle과 implementation을 순환시키는가? | Ahem 문자열 계산과 CRD §5.2/§11.1로 70/100px을 먼저 도출하고 observation은 별도 JSON으로 둔다. 통과. |
| 7 | font load 실패 뒤 sans-serif fallback이 Ahem처럼 오인되는가? | Fixture는 `document.fonts.ready` 후 `document.fonts.check('10px Ahem')`을 저장하며 false면 gate가 실패한다. 통과. |
| 8 | min-content 계산에서 공백을 포함해 80px으로 잘못 세는가? | `XX XXXXXXX`의 min-content는 unbreakable longest word `XXXXXXX`만 사용해 70px로 계산했다. 통과. |
| 9 | max-content 계산에서 collapsible space를 누락하는가? | `20 + 10 + 70 = 100px`으로 계산하고 Chrome intrinsic probe도 따로 저장한다. 통과. |
| 10 | min-content text line count를 한 줄로 가정해 block-size를 잘못 판정하는가? | 70px width에서 2줄×10px=`20px`, 100px width에서 1줄=`10px`로 독립 계산했다. 통과. |
| 11 | text line-height가 user-agent 기본값에 따라 달라지는가? | Leaf text는 `font:10px/10px Ahem`을 명시한다. 통과. |
| 12 | one-track auto row나 default alignment가 예상 Grid frame을 늘리는가? | 모든 parent/leaf box의 margin/padding/border/gap은 0이고 start alignment를 명시했다. auto row는 text의 측정 높이만 사용한다. 통과. |
| 13 | gap이나 border가 intrinsic width에 섞여 중첩 수준별 합산을 모호하게 하는가? | Inline axis는 한 track, column gap 0, border/padding 0으로 고정한다. 통과. |
| 14 | fixture의 viewport/page label absolute offset을 runtime 비교 기준으로 쓰는가? | Chrome page coordinate는 관찰 JSON에 남기되 gate는 root-relative box geometry만 비교하도록 계획했다. 통과. |
| 15 | 중간 Grid의 child ownership이나 nesting order가 바뀌어도 test가 통과하는가? | ID, owner, child order, exact node count를 runtime allowlist로 확인하고 미등록 tree는 거부한다. 통과. |
| 16 | anonymous text/inline wrapper/auto-placement 같은 별도 규칙이 결과를 오염하는가? | Leaf는 direct-leaf text 한 개이며 inline descendant/anonymous item은 제외했다. Grid마다 child가 한 개뿐이다. 통과. |
| 17 | test가 실제 parent intrinsic contribution 대신 각 Grid의 독립 width만 확인하는가? | Runtime gate는 각 Grid Columns trace와 상위 item contribution source, text intrinsic source/revision을 함께 요구한다. 통과. |
| 18 | depth 3 이상이나 임의 CSS 지원까지 근거를 과장하는가? | 계획의 positive gate는 depth 1/2의 fixed scene 네 개로 한정하고 depth 3+, subgrid, arbitrary CSS를 제외했다. 통과. |
| 19 | simulator draw/present를 text glyph painting 증거로 오해하는가? | 앱 캡처는 box/runtime 상태만 증명하고 glyph paint와 compositor-after pixel은 검증하지 않는다고 명시했다. 통과. |
| 20 | 이 slice 통과를 C11.4 완료 또는 공식 수 증가로 계산하는가? | 계획과 README에서 C11.4 미완료, 공식 `72/213` 유지, WPT 미실행을 명시했다. 통과. |

## 판정

Chrome oracle, CSS Grid 규범에서 도출한 70/100px 기댓값, 두 단계 owner topology, Simulator/API 범위와 미완료 상태를 서로 다른 실패 관점으로 검토했다. 구현 전 계획 입력과 판정 경계가 고정됐다.
