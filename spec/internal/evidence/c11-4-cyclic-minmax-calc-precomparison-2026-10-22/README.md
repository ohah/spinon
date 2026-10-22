# C11.4 cyclic percentage `minmax(calc())` 사전 비교

**상태:** 계획 단계의 fixed input, 독립 CSS Grid 기대값과 Agent Browser Chrome 156 관측이다. Spinon runtime이나 Android/iOS 앱 검증 결과가 아니다.

## 재현 입력과 환경

- fixture: [`grid-cyclic-minmax-calc-c114-precomparison.html`](../../../../tests/fixtures/css/c11/grid-cyclic-minmax-calc-c114-precomparison.html)
- browser: Agent Browser `0.31.1`, macOS arm64, `HeadlessChrome/156.0.8078.48`, viewport `1280×633` CSS px, DPR 1.
- Ahem 10px/10px line-height가 loaded 상태이고 `document.fonts.check('10px Ahem')`가 true일 때 computed tracks, root/item DOMRect와 scroll extent를 수집했다.
- [Chrome observation JSON](./chrome156-observation.json)과 [Chrome screenshot](./chrome156-minmax-calc.png)은 같은 fixture를 사용한다.
- Ahem font SHA-256은 `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`이다. fixture·관측·화면·독립 기대값의 현재 digest는 [SHA-256 manifest](./SHA256SUMS.txt)에 둔다.
- 규범 근거: [CSS Grid Layout Module Level 1, 2025 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes)과 [§11.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-grid-sizing). W3C 페이지를 Agent Browser에서 열어 cyclic percentage의 intrinsic-size 처리와 columns-then-rows sizing 순서를 확인했다.

## 독립 계산과 관측

기대값은 [독립 계산 JSON](./spec-derived-expectations.json)에 저장했다. CSS Grid §7.2.1에 따라 cyclic percentage를 intrinsic container-size 계산에서 auto처럼 취급하고, 해당 크기를 얻은 뒤 `calc()` maximum을 resolve한다. Ahem column `aa aa`의 intrinsic width 20/50px에서 root max-content width는 50px이며 final maximum은 29px이다. row fixture는 20px inline size에서 네 줄·40px block size를 만든 뒤 final maximum 22px을 사용한다. definite controls는 40px와 10px basis에서 24px·9px이다.

| scene | independent expected | Chrome 156 observation |
| --- | --- | --- |
| `cyclic-minmax-calc-column` | Columns `[29]`, Rows `[20]`, root `50×20`, item `29×20` | 동일 |
| `cyclic-minmax-calc-row` | Columns `[20]`, Rows `[22]`, root `20×40`, item `20×22` | 동일 |
| `definite-minmax-calc-column` | Columns `[24]`, Rows `[20]`, root `40×20`, item `24×20` | 동일 |
| `definite-minmax-calc-small-limit` | Columns `[9]`, Rows `[20]`, root `10×20`, item `9×20` | 동일 |

Screenshot의 small-limit root y는 110 CSS px다. root-relative `(5,5)` inside item, `(15,5)` outside 9px item과 10px root, `(25,5)` past text를 sample했다. 각각 RGB `(229,72,77)`, `(229,72,77)`, `(255,255,255)`이다. 이는 Chrome screenshot의 text overflow 관측이며 border stroke 비교가 아니다.

전용 Agent Browser session `spinon-cyclic-minmax-calc-20261022`는 닫았고 session info에서 inactive를 확인했다. 종료 뒤 남은 Agent Browser runtime PID `636`은 별도 `hwpjs-curve-page-35c54947528a` session에 연결되고 pageCount 1로 확인되어 종료하지 않았다. [브라우저 종료와 보존 기록](./browser-close.json).

## 범위

단일 Chrome build의 관측이다. Spinon layout, WPT, Android/iOS, 일반 CSS admission, glyph equality, CSS border stroke, hardware GPU, physical device, iOS compositor-after pixels 및 전체 C11.4를 검증하지 않는다.
