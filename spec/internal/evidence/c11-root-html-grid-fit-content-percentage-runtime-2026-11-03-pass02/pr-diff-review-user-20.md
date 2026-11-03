# C11.1 `fit-content(percentage)` 사용자 요청 PR diff 검토 20개

대상 PR [#1106](https://github.com/ohah/spinon/pull/1106), 검토 head `b78af0e1e9f52b098a8fbd78d15dd037124fc494`. 실제 변경 코드와 기존 독립 비교·focused test·simulator evidence를 대조했다. 계획 diff의 20개 검토 및 구현/runtime 검토 20개와 별도다.

| # | 실패 관점 | 판정과 근거 |
|---:|---|---|
| 1 | Cascade가 `%`를 pixel 수치로 바꾸어 percentage identity를 잃는다. | PASS — cascade 검사에서 column/row `FitContent(Percentage(...))`가 그대로 남고 runtime snapshot에서도 20%·120% 값을 확인한다. |
| 2 | 열 cap이 row basis를, 행 cap이 column basis를 읽는다. | PASS — layout 행렬은 서로 다른 width/height를 지정하고 결과 track 및 runtime marker 위치를 각각 비교한다. |
| 3 | border-box width가 content-box percentage basis로 쓰인다. | PASS — 30px content width와 좌우 2px border에서 Chrome 156 및 Ahem shared-layout 결과가 15px이다. border-box 오해 값 17px과 구별한다. |
| 4 | padding이나 border 두께를 basis에서 빼거나 더한다. | PASS — Ahem content-box discriminator가 border 포함·제외 계산을 나누며 runtime overflow scene은 border box를 별도로 assert한다. |
| 5 | CSS Grid gap을 basis에서 차감한다. | PASS — width 200px, column gap 20px에서 50% cap track이 100px인 focused layout assertion이 있다. |
| 6 | 0% cap을 무효 처리하거나 auto minimum을 0으로 강제한다. | PASS — 0% column/row 및 definite zero basis에서 min-content floor를 유지하는 Ahem 테스트가 있다. |
| 7 | definite 0px basis를 indefinite basis와 혼동한다. | PASS — 0px width/height 입력은 사용 가능하고 별도 `height:auto` runtime fixture는 property-bound 오류로 실패한다. |
| 8 | 100% 초과 입력을 cascade에서 100%로 clamp한다. | PASS — `Percentage(1.2)`를 cascade/projection/runtime snapshot에서 보존한다. |
| 9 | 120% cap이 definite available grid 공간을 넘어 used track을 키운다. | PASS — 독립 Chrome 156과 Ahem 장면에서 cap 60px, definite available 50px, used track 50px을 구분해 검증한다. |
| 10 | auto minimum이 resolved cap보다 큰 경우 minimum을 잘라낸다. | PASS — 0%·20% row cap의 40px Ahem floor가 유지된다. |
| 11 | max-content contribution이 cap보다 작아도 track을 cap까지 늘린다. | PASS — max-content 110px과 120% cap 장면의 결과는 110px이다. |
| 12 | 고정 `repeat(2, fit-content(20%))` 확장 시 반복 track 사이즈나 metadata가 달라진다. | PASS — cascade repeat group count와 두 typed track을 확인하고 layout/runtime에서 `[40px, 40px]` 및 두 번째 item의 x 위치를 검사한다. |
| 13 | `fit-content(calc(...))`를 direct percentage로 받아들인다. | PASS — cascade는 해당 Grid property에서 거부하고 projection의 `Calc` variant도 explicit allowlist 밖이다. |
| 14 | 음수, NaN, 양·음 무한대 cap이 layout engine에 도달한다. | PASS — projection과 raw layout 입력에서 finite non-negative 조건을 각각 검사한다. |
| 15 | `grid-template-areas`와 percentage track을 섞어 named-area 제한을 우회한다. | PASS — projection에서 `grid-template-areas` 경계로 거부하며 runtime test는 partial scene이 없음을 확인한다. |
| 16 | implicit `grid-auto-*` track이 이번 explicit-track admission을 통해 열린다. | PASS — cascade rejection과 `RootHtmlFitContentV1` auto-track validation은 기존 범위를 유지한다. |
| 17 | percentage가 없는 반대 축 `auto`를 percentage axis basis만으로 허용한다. | PASS — 계획·명세·fixture는 양 content dimension definite 조건을 명시하고 indefinite-height runtime 사례는 실패한다. |
| 18 | 다른 style profile이나 일반 CSS 경로로 제한 admission이 퍼진다. | PASS — 새 projection은 `RuntimeRootHtmlTextPaintV1` profile에서만 호출되고 C11.4 fixture test도 별도 regression gate로 남아 있다. |
| 19 | border 예외가 비-Grid element나 percentage 외 Grid에 적용된다. | PASS — border projection 허용은 Grid container와 유효한 non-negative percentage fit-content track을 함께 요구한다. 기존 px-only overflow fixture의 border는 runtime에서 거부된다. |
| 20 | invalid 입력에서 부분 layout/render snapshot이 publish되거나 overflow가 border에서 잘린다. | PASS — negative runtime matrix는 completed scene 부재를 확인하고 Android PixelCopy/iOS Metal drawable은 root interior, 노출 border, child overflow, 바깥 배경을 4/4 exact로 비교한다. |

## 판정

검토 대상 diff에서 수정이 필요한 동작 결함은 발견하지 못했다. 이는 제한된 parsed Root HTML profile과 기록된 focused/simulator evidence에 대한 판정이다. 일반 CSS admission, WPT 적합성, 반대 축 `auto`, 실기기/hardware GPU 및 iOS compositor 이후 픽셀은 검증 범위에 없다. GitHub PR check rollup은 비어 있고 `main`에는 required status check가 설정되지 않았다.

## 확인한 자료

- [계획과 범위](../../../../plan/c11-root-html-grid-fit-content-percentage.md)
- [실행·화면·로그·hash](README.md) · [구현/runtime 검토 20개](implementation-runtime-review-20.md)
- [layout percentage tests](../../../../crates/spinon-layout/src/c15_text_layout_tests/grid_fit_content/percentage.rs)
- [cascade tests](../../../../crates/spinon-style/src/stylo_dom/cascade/runtime_layout_tests/c11_1_root_html_fit_content.rs) · [projection tests](../../../../crates/spinon-style-to-layout/src/projection/style_values/grid_values/root_html_grid_track_tests/fit_content.rs)
- [Root HTML runtime tests](../../../../crates/spinon-runtime/src/session/tests/root_html_grid_fit_content_percentage.rs)
