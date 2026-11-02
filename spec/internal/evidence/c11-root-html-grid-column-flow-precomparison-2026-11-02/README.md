# C11 Root HTML column auto-flow 사전 비교

## 목적

기본 parsed Root HTML runtime에서 `grid-auto-flow: column`을 추가로 연결할 수 있는지 제한된 명시 격자 입력으로 확인한다. 이 사전 비교는 C11 구현 완료나 전체 CSS Grid 지원을 뜻하지 않는다.

## 독립 기준

- 입력은 [고정 HTML fixture](https://github.com/ohah/spinon/blob/main/tests/fixtures/css/c11/root-html-grid-column-flow-precomparison.html)이며 viewport는 `1280×633 CSS px`, DPR 1이다.
- 규칙 기준은 [CSS Grid Layout Module Level 2 §7.7](https://www.w3.org/TR/css-grid-2/#grid-auto-flow-property) 및 [§8.5](https://www.w3.org/TR/css-grid-2/#auto-placement-algo)다. 참조 시점 문서는 2025-03-26 W3C Candidate Recommendation Draft이며, 최종 Recommendation으로 표현하지 않는다. 이 fixture는 sparse `column` flow에서 source-order item을 첫 명시 열의 행부터 채우고 다음 열로 이동하는 규칙만 사용한다.
- 독립 계산은 explicit columns `[30,70]`, rows `[20,40]`, gap 0, item 수 4로 한다. 각 item의 고정 크기를 대응 cell과 같게 지정했으므로 예상 frame은 A `(0,0,30,20)`, B `(0,20,30,40)`, C `(30,0,70,20)`, D `(30,20,70,40)`이며 implicit track은 없다. 기계 판정값은 `expected.json`에 있다.
- Agent Browser `0.31.1`의 HeadlessChrome `156.0.0.0`에서 같은 fixture를 열어 `getBoundingClientRect()`와 computed style을 기록했다. 관측 metadata는 `chrome-observation.json`에 있다. 아래 화면은 이 Chrome fixture 캡처다.

![Chrome 156 column-flow fixture](./chrome-156-column-flow.png)

## 코드 사전 조사

- Stylo typed cascade는 `ComputedCssGridAutoFlow::Column`을 보존하고 style-to-layout은 이를 `GridAutoFlow::Column`으로 매핑한다.
- vendored Taffy adapter와 dense-column placement normalizer는 column flow를 표현하지만, 현재 C11.1 기본 layout mode는 이를 거부한다.
- Root HTML Grid projection은 현재 기본 row flow만 허용하고 `grid-auto-columns`가 `auto`가 아니면 거부한다. 따라서 author CSS가 이 경로에서 같은 column placement를 만드는지는 아직 입증되지 않았다.
- 변경 전 계획은 명시 track 내 sparse column flow로만 제한한다. implicit row/column, `dense`, named areas, authored item placement, intrinsic contribution, 방향·writing-mode 변형, 일반 CSS admission 및 WPT 완료는 범위 밖에 둔다.

## 해시와 한계

Fixture와 관측/기대값의 SHA-256은 `SHA256SUMS.txt`에 고정한다. Chrome은 독립 CSSOM oracle 확인에 사용했으며 Spinon runtime·앱 화면·GPU surface 픽셀은 이번 사전 비교에서 실행하지 않았다.
