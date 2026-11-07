# C11.4 `align-content:safe|unsafe center` 사전 비교

## 목적과 경계

C11.4의 다음 제한 후보를 정하기 위해 기존 fixed `RootHtmlAutoRowStretchV1` shape에서 Chrome 157의 `safe center`, `unsafe center`, `center` 동작을 비교했다. 이 자료는 계획 전 oracle evidence이며 Spinon 구현 결과가 아니다.

- Fixture SHA-256: `68ed05e0a40a39813f6e85808c338934311c6fa195495ee5278d083f58ef529a`
- 관측자: Agent Browser 0.31.1, HeadlessChrome 157.0.0.0, viewport 1280×633 CSS px, DPR 1
- Grid: content box 40×40px, 40px column, explicit 10px row + 두 implicit `auto` rows, column-gap 0, fixed empty item 하나와 10px line-height direct text `A`/`B`
- Positive scene은 row-gap 0px, overflow scene은 definite 40px basis의 `calc(120% - 2px)` (46px row-gap)다.

## 관측 및 독립 기대값

6 scene에서 computed alignment/gap, used row sizes, 세 item의 frame을 기록했다. positive space에서 세 값 모두 child y offsets `[5,15,25]px`다. overflow는 `[10,10,10]px` tracks와 두 46px gutters로 122px alignment subject를 만들어 40px container보다 82px 크다. `safe center`는 start fallback으로 `[0,56,112]px`, `unsafe center`와 plain `center`는 group을 -41px 이동해 `[-41,15,71]px`다. Chrome 관측은 독립 산출값과 모두 정확히 일치했다. 원점 포함 raw 좌표와 computed CSS serialization은 `chrome-observation.json`, 계산은 `expected-values.json`에 있다.

규범 출처는 [CSS Box Alignment Level 3 §4.4](https://www.w3.org/TR/css-align-3/#overflow-values)다. 이 W3C 문서는 2026-10-08 Working Draft이며 `<overflow-position>`이 at-risk로 표시되어 있으므로 현재 공개 CSS 지원 선언이나 안정된 제품 계약으로 해석하지 않는다. §4.4에서 explicit `safe` overflow는 flex-start처럼 동작하고 Grid에서 flex-start는 start와 같으며, `unsafe`는 지정 alignment를 유지한다. [§5.1.4 Grid Containers](https://www.w3.org/TR/css-align-3/#align-content-property)는 정렬 subject를 grid tracks와 gutters로 정의한다.

## 재현 자료

- [고정 HTML fixture](./fixture.html)
- [Chrome 관측 JSON](./chrome-observation.json)
- [독립 기대값 JSON](./expected-values.json)
- [Agent Browser 캡처](./chrome-safe-center.png)

이 결과는 C11.4 구현·런타임 검증, CSS conformance/WPT, mobile simulator/device, surface pixels를 입증하지 않는다. Agent Browser를 종료했으며 Chrome 브라우저 child process는 남지 않았다.
