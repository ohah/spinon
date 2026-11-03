# 구현/runtime 실패 관점 검토 20개 · 2026-11-04

검토 대상은 현재 branch의 parsed Root HTML numeric placement 코드, Rust tests, 고정 HTML fixture, Android AVD/iOS Simulator 실행 자료다. 각 항목은 독립된 실패 경로를 확인하며 PR 전체 diff 검토를 대신하지 않는다.

1. **Grid owner 교차 오염** — 한 explicit Grid의 mode가 sibling all-auto Grid에 전파될 수 있다. owner별 선택과 mixed scene frame을 확인했다. `root_html_explicit_route_isolated_from_all_auto_sibling_grid`에서 두 Grid가 서로 다른 결과를 유지한다. 통과.
2. **dense/column-flow route 가로채기** — 새 row-flow route가 특수 flow의 placement 알고리즘을 덮을 수 있다. selector가 dense owner와 non-row flow를 각각 거르고, dense projection 4/4 및 column-flow runtime 2/2를 확인했다. 통과.
3. **named-area route 충돌** — `grid-template-areas` Grid가 새 no-area admission을 우회할 수 있다. selector는 areas가 있는 Grid를 제외하고 기존 named-area validator를 보존한다. 코드 경계 확인. 통과.
4. **owner의 visible item 누락** — owner의 item 하나만 numeric이면 전체 Grid가 선택될 위험이 있다. selector는 owner에 매핑된 모든 visible item을 검사한다. mixed explicit/auto negative runtime case가 전체 scene을 거부한다. 통과.
5. **mixed auto/explicit 허용** — auto-placement와 definite placement가 조용히 섞일 수 있다. Rust style/runtime rejection matrix에서 같은 owner의 mixed child scene 전체가 실패한다. 통과.
6. **partial-axis placement 수용** — row 또는 column 한 축이 auto일 때 다른 축만 배치할 수 있다. partial-axis runtime rejection과 projection negative case가 전체 scene 실패를 확인한다. 통과.
7. **불완전 endpoint numeric 판정** — 네 endpoint 중 하나가 auto/unsupported인데 route가 선택될 수 있다. `candidate_nodes`와 alignment eligibility 모두 네 endpoint가 `Line|Span`인지 확인하며 C11.2.1 normalizer가 재검증한다. 통과.
8. **line 0 허용** — line index 0이 track boundary처럼 취급될 수 있다. RuntimeSession `grid-row:0 / 1` negative case와 기존 C11.2.1 placement suite 15/15를 확인했다. 전체 scene 거부. 통과.
9. **음수 line 및 reversed endpoint 계산** — `-1` 기준이나 endpoint swap이 틀릴 수 있다. Chrome 156 observation과 Rust expected frames가 음수 마지막 track 및 reversed row/column placement를 대조한다. 통과.
10. **shorthand/cascade winner 무시** — `grid-area` shorthand 또는 뒤의 longhand가 projection에서 누락될 수 있다. computed CSSOM 사전 비교와 `grid-area` 뒤 `grid-row-start`를 덮어쓰는 Rust frame case를 확인했다. 통과.
11. **span track count 오류** — `span`을 absolute line이나 start line로 혼동할 수 있다. `grid-row:2 / span 1` 및 `grid-column:1 / span 1` frame과 기존 C11.2.1 normalizer를 대조했다. 통과.
12. **implicit track 생성/범위 초과** — explicit grid 밖 line이 implicit row/column을 만들 수 있다. `grid-row:3 / 4`에 `grid-auto-rows`가 있는 runtime case가 partial scene 없이 실패한다. 통과.
13. **hidden child 용량 점유** — `display:none` child가 visible item 집합에 들어갈 수 있다. Rust projection hidden control은 zero frame을 내고 visible sibling의 authored cell을 유지한다. 통과.
14. **anonymous text item 누락** — CSSOM element만 열거해 direct anonymous text의 auto-placement 요구를 놓칠 수 있다. HostDocument text child가 포함된 mixed scene이 runtime에서 거부된다. 통과.
15. **nested Grid item 허용** — Grid child 자체가 새 profile에서 의도치 않게 중첩 Grid로 layout될 수 있다. direct nested `display:grid` runtime scene은 block-item gate에서 거부된다. 통과.
16. **replaced image item 허용** — HTML image가 numeric placement만으로 일반 empty `div`처럼 처리될 수 있다. direct `<img>` runtime scene은 Root HTML image-item guard에서 거부된다. 통과.
17. **overlap과 source order** — 같은 cell을 쓰는 두 item을 금지하거나 paint order를 바꿀 수 있다. Rust에서는 두 frame이 겹치고 runtime render-box index는 red item 다음 green item을 유지한다. 통과.
18. **C11.5 self-alignment 유실** — explicit mode 지정이 item alignment typed values를 지울 수 있다. `align-self:end;justify-self:end`의 Rust expected frame 및 runtime fixture의 alignment scene을 확인했다. 통과.
19. **visible overflow 및 pixel 증거 과장** — Grid border 밖 item이 잘리거나 OS screenshot을 surface parity로 오인할 수 있다. Chrome/Android/iOS PNG의 경계 표본은 Grid 밖 red와 item 밖 white를 보인다. PixelCopy/Metal readback이 아님을 함께 기록했다. 통과.
20. **실패 원자성과 same-session 회복** — invalid layout이 partial/stale snapshot을 남기거나 worker를 멈출 수 있다. 같은 RuntimeSession에서 valid light scene 뒤 dark named-line failure는 `completed=None`이고, light로 복귀하면 ready render snapshot을 다시 publish한다. 단독 negative cases도 diagnostic과 `completed=None`을 확인한다. 통과.

## 기존 baseline 실패

`root_html_grid_item_alignment` 전체 filter는 7개 중 3개가 통과하고 4개가 `UnsupportedGridContainerStyle { property: "C11.5 fixed-track auto-size/stretch contract" }`로 실패했다. 같은 checkout에서 구현 변경 파일 세 개를 `HEAD` 내용으로 임시 복원해 동일 filter를 실행했을 때도 똑같은 4개가 실패했고, 작업 파일은 저장본과 hash 대조 뒤 원상 복구했다. 원본 [baseline test log](c11-5-alignment-baseline-test.log)를 보존했다. 이 네 항목은 현재 후보가 고친 결함이나 신규 회귀로 표시하지 않으며 전체 alignment suite green이라고도 하지 않는다.

## 관찰된 fixture 입력 수정

Same-session media recovery test의 첫 stylesheet는 `background` shorthand가 `background-position-x` computed declaration을 포함해 현재 제한 Root HTML CSS profile에서 cascade rejection을 일으켰다. 이를 지원되는 `background-color`로 좁힌 뒤 named-line invalid scene → numeric valid scene 복구 test가 통과한다. 생산 코드는 이 관찰 때문에 변경하지 않았다.
