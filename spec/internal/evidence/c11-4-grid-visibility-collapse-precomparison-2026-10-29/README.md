# C11.4 Grid visibility:collapse 사전 비교

이 자료는 CSS Grid item에 지정한 visibility:collapse가 Grid track sizing, frame, hit testing, paint 및 visible descendant에 미치는 동작을 구현 전에 고정한다. 입력은 [fixture](../../../../tests/fixtures/css/c11/grid-visibility-collapse-c114-precomparison.html), Chrome 관측은 [JSON](./chrome-agent-browser-observation.json), 화면은 [PNG](./chrome-agent-browser-precomparison.png), 규범 계산은 [expected.json](./expected.json)에 있다.

## 규범 계산

[CSS Display Level 3 §4](https://www.w3.org/TR/css-display-3/#visibility)는 visibility:collapse에 formatting-context-specific 동작이 정의된 경우를 제외하면 hidden과 같이 invisible 처리하며, visibility:visible을 설정한 descendant는 표시될 수 있다고 규정한다. [CSS Grid Level 1 2025 CRD](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)는 Grid item에 대해 별도 visibility:collapse sizing 절차를 두지 않는다. 따라서 Grid item에서는 computed value를 collapse로 보존하되 hidden과 같은 layout/paint 효과를 내야 한다는 것이 이 비교의 표준 기반 inference다. Flexbox의 strut/re-layout 동작은 [CSS Flexbox §4.4](https://www.w3.org/TR/css-flexbox-1/#visibility-collapse)의 별도 규칙이므로 이번 Grid 기대값으로 일반화하지 않는다.

각 축의 minmax(auto,max-content) 두 track에 100px max-content child를 span시킨다. collapse와 hidden은 item의 automatic minimum contribution을 유지해 tracks [50,50]px 및 100px item을 만든다. display:none 대조군은 item을 grid formatting context에서 제거해 해당 intrinsic contribution을 없애고 tracks [0,0]px로 만든다. collapse visible-descendant scene은 visible 8px child를 유지한다. Row-axis visible-descendant scene의 뒤따르는 flow content를 92px로 둬 총 contribution을 100px로 맞춘다. 양축 visible positive control은 같은 100px item을 실제로 paint해 root 밖 표본의 red 색을 확인한다.

## Agent Browser 관측

- Browser: Agent Browser 0.31.1, HeadlessChrome/156.0.0.0, macOS host.
- Viewport: 1280×633, DPR 1.
- 실행 대상: repository fixture. 관측 JSON에서는 workspace별 절대 경로를 file://<workspace>/tests/fixtures/...로 정규화했으며 scene 값은 바꾸지 않았다.
- 양축 각각 collapse, hidden, display-none, collapse-visible-descendant, visible positive control을 같은 document revision에서 읽었다. 총 10 scenes의 used tracks, root/item/child frames, computed visibility/display, root overflow 및 item/root-boundary hit targets는 validate_precomparison.py로 expected JSON과 대조한다.
- root-local surface sample 14개를 PNG에서 직접 읽었다. collapse/hidden red content 내부는 root background #f0f0f0, visible descendant는 #0000ff, invisible item의 root 경계 바깥 1px 표본은 body white #ffffff, visible positive control의 같은 위치는 red #ff0000이다.
- Fixture SHA-256: 376826646d970b61f8e1a1e68f2ecd6fd70faf114c59627390abfa951dbd3ac7.
- Screenshot SHA-256: 23b1c2013be0842fcb8c43b952bc91ae9241c137e6a03b1729872f6902635782.
- validate_precomparison.py 결과: 10/10 scene, 14/14 exact screenshot samples 통과. Agent Browser session을 닫았고 session list는 비어 있으며 session info는 active=false, pid=null을 반환했다.

Chrome 관측은 기준 구현 결과이며 Spinon/WPT pass가 아니다. 이 사전 비교는 지정한 CSS, 양 축 및 고정 dimensions만 다루고 clipping, scrolling, arbitrary Grid tree, Flex collapse 또는 전체 C11.4 적합성을 입증하지 않는다.
