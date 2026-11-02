# C11 Root HTML sparse column-flow 계획 실패 관점 검토 20개

**대상:** [계획](./c11-root-html-grid-column-flow.md), 고정 HTML [fixture](../tests/fixtures/css/c11/root-html-grid-column-flow-precomparison.html), 독립 기대값·Chrome 156 관측. 이는 계획 검토이며 구현/runtime review나 PR diff review가 아니다.

| # | 독립 실패 관점 | 계획의 통제 기준 | 판정 |
|---:|---|---|---|
| 1 | `column`을 physical x축 순회로 단정해 writing mode 의미를 잘못 구현 | 지원을 horizontal-tb/LTR 조건으로 제한하고 RTL·vertical writing mode를 거부한다. | 통과 |
| 2 | row/column 좌표를 바꿀 때 source order가 뒤바뀐다 | 2×2 각 item의 row·column·frame을 독립 oracle 표에 고정하고 source-order 변형을 수락 기준에 둔다. | 통과 |
| 3 | `column dense`를 sparse `column`과 같은 동작으로 취급한다 | 정확히 `Column`만 허용하고 row/column dense는 rejection control로 둔다. | 통과 |
| 4 | 4셀을 채운 뒤 5번째 item이 implicit column을 만들지만 정상 layout으로 publish된다 | visible item capacity를 `columns × rows`로 제한하고 초과를 scene publication 전에 거부한다. | 통과 |
| 5 | item span이 capacity 계산을 우회한다 | 네 computed placement 값이 모두 `auto`인 item만 허용하고 span 입력을 거부한다. | 통과 |
| 6 | authored explicit numeric placement가 자동 배치로 무시된다 | 계산된 네 placement longhand를 검사하고 numeric line은 fail-closed로 둔다. | 통과 |
| 7 | named line/area placement가 빈 explicit grid에서 조용히 해석된다 | named line, areas 및 repeat 조합을 이 slice에서 금지하고 각각 negative case로 둔다. | 통과 |
| 8 | non-default `grid-auto-rows`가 column flow의 implicit row를 만들어낸다 | computed `auto`만 허용하며 implicit sizing 자체를 범위 밖에 둔다. | 통과 |
| 9 | non-default `grid-auto-columns`가 capacity 오류 뒤 implicit 열을 sizing한다 | computed `auto`만 허용하고 5번째 item rejection을 검증한다. | 통과 |
| 10 | 빈 explicit axis 또는 overflowed track count가 capacity 곱셈을 통과한다 | track list를 non-empty finite 값으로 제한하고 checked `columns × rows` 계산 후 검증하도록 요구한다. | 통과 |
| 11 | hidden subtree가 cell capacity를 소비하거나 숨은 item frame을 publish한다 | `display:none`은 기존 visible-tree 규칙대로 capacity와 scene에서 제외하고 unsupported display는 거부한다고 분리해 적었다. | 통과 |
| 12 | CSS `order`로 order-modified sequence가 source order와 달라진다 | 모든 item `order:0`을 요구하고 별도 CSS order를 허용 범위에서 제외한다. | 통과 |
| 13 | default `grid-template-areas` projection과 column mode가 충돌한다 | no-area/no-repeat mode를 별도 contract로 두고 area 조합은 거부한다. | 통과 |
| 14 | root별 named area 처리, id 또는 selector를 fixture 선택에 쓴다 | RootHtml default entry에서 selector·tag·HostNode ID가 바뀐 control을 요구하고 semantic paint로 frame을 찾는다. | 통과 |
| 15 | fixed gap이 column 흐름에서 한 축에 두 번 반영되거나 빠진다 | zero-gap oracle 외에 `row-gap:3px`, `column-gap:5px` control을 추가하고 네 item origin을 고정했다. | 통과 |
| 16 | intrinsic text/image/nested item이 고정-size empty item처럼 배치된다 | empty direct block `div`만 허용하고 text/image/nested content 거부를 각각 명시한다. | 통과 |
| 17 | computed style/layout이 서로 다른 document revision에서 섞인다 | 기존 Root HTML entry의 style/layout revision identity를 보존하고 runtime assertion에 포함한다. | 통과 |
| 18 | simulator screenshot을 CSSOM 또는 GPU pixel parity로 과장한다 | Chrome CSSOM oracle, Rust frame assertions, Android/iOS draw evidence를 분리하고 surface readback 한계를 기록한다. | 통과 |
| 19 | fixture-only code가 default 제품 artifact 또는 FFI에 새 marker를 넣는다 | 실제 Root HTML profile에서 구현하고 fixture-only constructor·feature를 사용하지 않으며 제품 artifact 경계를 확인한다. | 통과 |
| 20 | 이 제한 slice가 C11.1/C11 완료 수를 올린다 | 내부 계약 0059 `0.1.0` 유지, C11/C11.1 미완료 및 공식 `72/213` 유지 조건을 계획에 명시한다. | 통과 |

**판정:** 계획 실패 관점 20/20 통과. 검토 중 hidden item이 fail-closed된다는 초기 문구는 현재 Root HTML visible-tree 동작과 맞지 않아 수정했다. 관점 5·11은 새로 적은 visible item 경계와 hidden frame 제외 조건을 다시 확인했다. 구현 전에 이 계획을 변경하면 영향받는 관점만 이름을 바꿔 재사용하지 않고 실제 수정 경계를 재검토한다. 구현/runtime 검토와 계획 PR diff 30개 검토는 별도다.
