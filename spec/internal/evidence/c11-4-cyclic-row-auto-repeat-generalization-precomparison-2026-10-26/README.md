# C11.4 행 축 cyclic auto-repeat 일반화 사전 비교

이 자료는 병합된 C11.4 행 축 cyclic percentage auto-repeat slice 뒤의 제한 일반화 후보를 고정한다. Chrome 관측과 CSS Grid 규범에서 계산한 기대값을 별도 파일에 보존한다. 아직 Spinon runtime 결과는 없다.

## 입력과 원본

- Fixture: [`grid-cyclic-row-percentage-auto-repeat-generalization-c114-precomparison.html`](../../../../tests/fixtures/css/c11/grid-cyclic-row-percentage-auto-repeat-generalization-c114-precomparison.html)
- 독립 기대값: [`spec-derived-expectations.json`](./spec-derived-expectations.json)
- Chrome 156 raw DOM/CSSOM 관측: [`chrome156-generalized-row-observation.json`](./chrome156-generalized-row-observation.json)
- Chrome 156 matrix screenshot: [`chrome156-generalized-row-matrix.png`](./chrome156-generalized-row-matrix.png)
- 거부한 row re-expansion 가설의 raw 관측: [`chrome156-observation.json`](./chrome156-observation.json)
- 거부 가설 fixture 및 screenshot: [`grid-cyclic-row-percentage-auto-repeat-reexpansion-c114-precomparison.html`](../../../../tests/fixtures/css/c11/grid-cyclic-row-percentage-auto-repeat-reexpansion-c114-precomparison.html), [`chrome156-row-axis-auto-repeat-reexpansion.png`](./chrome156-row-axis-auto-repeat-reexpansion.png)

Chrome은 Agent Browser의 HeadlessChrome 156.0.0.0, viewport `1280×633` CSS px, DPR 1에서 관찰했다. `CSS.supports()`는 `auto-fill`과 `auto-fit` 모두 true다. Browser output은 구현 oracle 관측이며 규범 기대값의 근거로 쓰지 않았다.

## 규범 기반 계산

기준 문서는 [CSS Grid Layout Module Level 1, 2025 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes), [§7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat), [§11](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-overview), [§11.1 gutter 처리](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#gutters), [CSS Sizing Level 3 §2](https://www.w3.org/TR/css-sizing-3/#definite)다.

`height:max-content`인 대상은 auto-repeat 계산 시점에 block size가 definite하지 않으므로 반복은 한 번이다. 반복 행의 cyclic `25%` maximum은 intrinsic sizing 중 `auto`처럼 처리된다. 고정 prefix/suffix는 각각 8px, row-gap은 2px 두 개다. 반복 행에 겹쳐 놓은 두 direct item의 block contribution은 24px과 16px이며 한 track의 최대 기여는 24px이다. 따라서 intrinsic content height는 `8 + 24 + 8 + 2×2 = 44px`이다. 최종 definite percentage resolution에서 repeated row는 `44×25% = 11px`이지만, 이 행 sizing rerun에서 반복 수는 다시 계산하지 않는다. 두 cyclic 대상 모두 `[8px, 11px, 8px]`이다. `auto-fit`의 반복 행은 사용 중이므로 collapse하지 않는다.

Definite 44px control은 같은 percentage를 `11px`로 계산한다. `16 + 2(n+1) + 11n ≤ 44`이므로 반복 두 개가 들어간다. `auto-fill` rows는 `[8px,11px,11px,8px]`이며 suffix는 root 기준 `y=36px`에서 시작한다. `auto-fit` CSSOM track list는 `[8px,11px,0px,8px]`다. 비어 있는 두 번째 반복 행은 0px로 collapse한다. §11.1의 gutter overlap 처리 때문에 collapsed run 경계에는 하나의 2px shared gutter가 남고, 그 뒤 gutter는 0px이다. 고정 suffix 행은 root 기준 `y=23px`에서 시작한다. Browser-only in-flow suffix probe가 이 위치를 직접 측정한다. Runtime trace에서는 repeated row와 각 gutter의 순서·base size·collapse flag를 따로 확인한다.

## 관측 요약

| Scene | CSSOM rows | Root content box | 기대와 관측 |
|---|---|---|---|
| `cyclic-max-content-auto-fill-prefix-suffix` | `[8,11,8]px` | `24×44px` | 일치 |
| `cyclic-max-content-auto-fit-prefix-suffix` | `[8,11,8]px` | `24×44px` | 일치; 반복 행 점유 |
| `definite-44px-auto-fill-prefix-suffix-control` | `[8,11,11,8]px` | `24×44px` | 일치 |
| `definite-44px-auto-fit-prefix-suffix-control` | `[8,11,0,8]px` | `24×44px` | 일치; 빈 반복 행 collapse |

각 Chrome 장면의 direct in-flow child는 4개다. 이 가운데 3개가 runtime fixture의 sizing child이고, 나머지 24×1px suffix probe는 browser-only 관측 item이다. 24×8px prefix item은 고정 첫 행을 차지한다. 24×24px와 16×16px item은 같은 반복 행을 차지해 첫 cyclic 행을 점유하고, 반복 sizing 기여는 합계 40px이 아니라 최댓값 24px이다. 독립 기대 frame은 root-content-box-relative CSS px이고 raw `DOMRect`는 page-relative CSS px다. 예를 들어 repeated-row item의 root-relative 시작은 `itemRect.y - rootRect.y = 10px`이다. 파란 1px boundary는 absolute browser-only marker다. Chrome CSSOM의 `minHeight:0px`는 직렬화된 관측값으로 보존하며, layout input은 CSS에서 미선언된 초기값을 `LayoutDimension::Auto`로 투영해야 한다. 세부 좌표·computed style·원본 DOMRect는 raw JSON과 screenshot에 있다.

별도 거부 가설은 40px item, `50%` repeated row, prefix/suffix와 gap이 없는 `height:max-content` fixture에서 두 row `[20px,20px]`를 예상했다. Chrome 156은 대상 rows `[20px]`, definite 40px control은 `[20px,20px]`로 관측했다. 이는 별도 입력인 PR #632의 16px item fixture에서 rows `[8px]`였던 관측과 구별한다. 두 fixture 모두 행 축 재확장을 지지하지 않는다. 기존 열 축 #623 fixture는 축 외에 percentage, fixed neighbors, gap, item input도 다르므로 이 행 관측만으로 열 축 결과를 판단하지 않는다. 축 간 sizing 단계 차이는 별도 규범 비교가 필요하다.

## Hashes

각 사전 비교 입력과 원본에 `shasum -a 256`을 적용했다.

| File | SHA-256 |
|---|---|
| [HTML fixture](../../../../tests/fixtures/css/c11/grid-cyclic-row-percentage-auto-repeat-generalization-c114-precomparison.html) | `04d6f537a4c5868883477caedbc4afb3750170bff2f4d9d6cba1e8f8099a5028` |
| [Rejected-hypothesis HTML fixture](../../../../tests/fixtures/css/c11/grid-cyclic-row-percentage-auto-repeat-reexpansion-c114-precomparison.html) | `a6d9057170eac1c40a25c59b46e9ebc746f8dd2f11a1056f0bd81ab6046f1282` |
| [Independent expectations](./spec-derived-expectations.json) | `a4167f31f158f89cb6c03ae2782275664d306a237068fe1369479278c5714191` |
| [Chrome raw observation](./chrome156-generalized-row-observation.json) | `29202dc8b63e1d648b43b640943a7b496fb4155dd5af7aa5feda804e6560ca25` |
| [Chrome matrix screenshot](./chrome156-generalized-row-matrix.png) | `c108bc2ac634a20bf44fd9ebc2ae6693a04edec79812362ce840b0b1bc6899db` |
| [Rejected-hypothesis raw observation](./chrome156-observation.json) | `de75f45f05c962a821357f61445d283f3fe5f78f4632c5ce8f193177c80cee17` |
| [Rejected-hypothesis screenshot](./chrome156-row-axis-auto-repeat-reexpansion.png) | `294715e5f3f03e2ffbe3618f837d5b80cd639c854c24d2095400103280ac4058` |
