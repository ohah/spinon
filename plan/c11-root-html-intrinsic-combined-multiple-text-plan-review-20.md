# C11.4 combined intrinsic columns + multiple text rows plan review · 20 failure perspectives

대상: `c11-root-html-intrinsic-combined-multiple-text.md`, fixture, independent expected values, Chrome observation. 각 항목은 계획 구현 시 놓칠 수 있는 서로 다른 실패 경로다.

1. **열 계산 순서:** 코드가 row contribution 전에 intrinsic columns를 resolve하는지 확인한다. 순서를 바꾸면 long text line count가 바뀐다.
2. **min-content 열과 단어 폭:** 모든 word가 6.03125px에 맞는 Chrome 기준은 oracle 전용이며 runtime은 자체 font metric을 재야 한다.
3. **max-content overflow:** 열이 Grid보다 넓어져도 마지막 item과 text frame의 visible overflow를 자르거나 clamp하지 않아야 한다.
4. **auto track stretch:** 40px definite Grid에서 auto 열이 40px가 되는 조건과 기본 `justify-content:start`를 계획에 고정한다.
5. **flex expansion:** `minmax(min-content,1fr)`가 같은 폭으로 보여도 min-content floor와 1fr resolution을 별도 scene으로 남긴다.
6. **다중 item maximum:** long/short contribution을 max가 아닌 합이나 첫 item으로 고정해선 안 된다. 다른 줄 높이가 각각의 row에 적용된다.
7. **source reversal:** DOM 순서를 바꿨을 때 row sizes는 text node identity를 따라야 하며 paint order는 그대로여야 한다.
8. **empty controls:** fixed 1px controls가 text min-content보다 작고 intrinsic column contribution을 오염시키지 않는지 fixture와 oracle에서 확인한다.
9. **explicit row boundary:** 첫 fixed row와 뒤의 implicit rows가 섞이지 않는지 final track trace에서 검증한다.
10. **explicit columns 범위:** 새 validator가 모든 intrinsic grammar를 새로 허용한다고 오해하지 않게 기존 combined-route validator의 제약을 재사용한다.
11. **intrinsic row rejection:** `grid-template-rows`에도 intrinsic sizing을 추가로 허용하는 경로가 생기지 않아야 한다.
12. **fit-content rejection:** 기존 route에서 거부하는 `fit-content()`가 multi-text candidate 조건 때문에 우회되지 않아야 한다.
13. **topology rejection:** nested Grid, inline descendants, 여러 text node, anonymous direct text와 두 번째 visible Grid owner를 계속 거부한다.
14. **hidden nodes:** `display:none` 제외와 `visibility:hidden` occupancy 처리의 기존 계약을 바꾸지 않는다.
15. **source grammar:** ASCII A token grammar가 잘못된 whitespace, non-ASCII, edge separator, empty string을 계속 닫는다.
16. **item style owner:** 각 text item's computed style와 grid owner/source node identity를 개별 대조해야 한다.
17. **revision atomicity:** source/document/render/style revision 불일치가 있으면 일부 item만 frame/paint에 게시하지 않아야 한다.
18. **track limit arithmetic:** columns×rows 및 implicit row 수를 checked arithmetic으로 계산하고 10,001번째 row를 사전 거부한다.
19. **trace budget:** optional trace budget을 넘겨도 정상 frame은 유지하고, trace allocation failure는 layout 오류로 남기는 기존 구분을 유지한다.
20. **evidence overclaim:** simulator screenshot은 visual smoke로 제한하고 PixelCopy/Metal/WPT/physical-device claims는 실제 실행이 없으면 제외한다.

## 사전 검토 결과

Chrome 156 observation은 네 column mode와 source-order reverse를 포함한다. Independent expected rows와 browser used metrics는 README에 분리했다. 이 리뷰는 계획 검토이며 구현 검토 또는 PR actual-diff review를 대신하지 않는다.

## Fixture boundary correction

초기 precomparison draft의 빈 controls는 auto width였지만 계획의 item contract는 finite fixed-px size만 허용한다. 구현 검증에서 이 불일치를 발견해 controls를 1px fixed width로 수정하고, source hash·expected frames·Chrome observation·screenshots를 다시 생성했다. 1px control은 Chrome의 6.03125px text min-content보다 작으며 track 결과는 바뀌지 않았다. 이 수정은 구현 PR에 들어가기 전의 fixture/planning correction이다.
