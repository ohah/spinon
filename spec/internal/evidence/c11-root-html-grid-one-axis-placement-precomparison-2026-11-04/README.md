# C11 Root HTML 한 축 고정 배치 사전 비교

## 비교 입력과 기준

- Fixture: [`root-html-grid-one-axis-placement-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-one-axis-placement-precomparison.html)
- 독립 기대값: [`expected.json`](./expected.json)
- Chrome 관측값: [`chrome156-observation.json`](./chrome156-observation.json)
- 실행 화면: [`chrome156.png`](./chrome156.png)
- 검증기: [`verify_precomparison.py`](./verify_precomparison.py)
- 동작 기준: [CSS Grid Layout Level 1 §8.5, Automatic Placement Algorithm](https://www.w3.org/TR/css-grid-1/#auto-placement-algo)

Agent Browser 0.31.1의 HeadlessChrome 156, macOS 26.5.1 arm64에서 viewport 1280×633 CSS px, DPR 1로 관측했다. Fixture SHA-256은 `14ebfe57fe3baeef908158ef566e606f8276ae8d963f8aac4b6f1c1980a1203e`, Chrome full-page 화면은 1280×13104 px이며 SHA-256은 `e940466bec89356d5ad2641890bff6f2578cdc6123cdbc814cfc48841d9bc608`다.

## 결과

14 scenes에서 41개 item node의 computed placement를 기록했다. display-none node에는 layout frame이 없으므로 geometry는 40 frames다. 독립 기대 geometry는 authored fixed tracks와 source order, sparse row auto-placement 규칙으로 계산했다. verifier는 Chrome의 computed placement와 track 크기, root-relative item frame을 대조해 `PASS scenes=14 items=41 frames=40 accepted=8 expected_rejection=5 max_abs_error_css_px=0.0`를 출력했다. Accepted는 기존 all-auto control을 제외한 8개 scene, expected rejection은 5개 scene이며 all-auto는 기존 경로의 별도 control이다.

| Scene 종류 | 기준 결과 |
| --- | --- |
| all-auto 기존 경로 | 기존 auto placement 유지 |
| column definite / row auto | source order에 따라 빈 row를 찾음 |
| row definite / column auto | row 안에서 빈 column을 찾음 |
| definite-axis span | span 크기만큼 연속된 빈 track을 찾음 |
| definite-axis negative line | explicit grid 끝 기준으로 해석 |
| 서로 다른 고정 축 혼합 | 이번 제한 profile에서는 거부 |
| fully definite와 one-axis 혼합 | 이번 제한 profile에서는 거부 |
| implicit row/column 필요 | 이번 제한 profile에서는 거부 |
| `display:none` child | occupancy에서 제외하며 frame 없음 |
| `visibility:hidden` child | 보이지 않지만 occupancy에 참여 |
| 같은 부모의 sibling Grid | row/column owner별 배치 독립 |
| nested Grid item | browser geometry는 관측하고 제품 제한에서는 거부 |

Chrome 결과는 browser oracle 관측이다. 이 사전 비교는 Spinon 구현, WPT 적합성, 앱 runtime 또는 surface pixel 결과를 증명하지 않는다. `expected.json`의 거부 장면은 이후 제품 후보의 제한 계약을 정하기 위한 fixture이며 Chrome에서 해당 장면이 실패했다는 뜻이 아니다.
