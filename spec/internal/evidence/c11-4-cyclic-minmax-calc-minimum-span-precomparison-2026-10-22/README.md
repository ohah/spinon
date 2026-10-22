# C11.4 cyclic `minmax(calc())` minimum + spanning item 사전 비교

이 사전 비교는 `minmax(calc(…%), max-content)` 두 track을 하나의 spanning item이 가로지를 때 cyclic minimum의 intrinsic basis와 span contribution이 분리되는지 고정한다. PR #493이 사용한 한 track/direct text 입력과 달리 두 explicit track 및 span-2 item을 사용한다. 계획·관찰 단계이며 Spinon runtime 성공을 뜻하지 않는다.

## 입력과 환경

- Fixture: [`grid-cyclic-minmax-calc-minimum-span-c114-precomparison.html`](../../../../tests/fixtures/css/c11/grid-cyclic-minmax-calc-minimum-span-c114-precomparison.html)
- Font: 저장소 Ahem, `10px/10px`; Chrome에서 `document.fonts.status=loaded`, `document.fonts.check("10px Ahem")=true`를 확인했다.
- 브라우저: Agent Browser 0.31.1, `HeadlessChrome/156.0.0.0`, viewport `1280×633` CSS px, DPR 1.
- Input SHA-256: fixture `05a674fffb38b9fff186a48755b9d2e96e3878d41680f9a3614271e0503933f8`; Ahem font `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`.
- Oracle: [CSS Grid Layout Module Level 1, 2025 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes), [§11.4](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing), [§11.5.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items).
- Chrome 관찰 JSON: [chrome156-observation.json](./chrome156-observation.json). 화면: [chrome-agent-browser-pass45.png](./chrome-agent-browser-pass45.png).
- Input·font·observation·screenshot SHA-256: [SHA256SUMS.txt](./SHA256SUMS.txt).
- Browser session `s45r/p45r`, PID `44443`를 확인 뒤 닫았다. 전역 세션은 0개이고 PID는 종료됐다. Tailnet preview server PID 1191은 유지했다. terminal-browser는 사용하지 않았다.

## 규범에서 독립적으로 계산한 기대값

열 fixture는 Ahem `aa aa` 하나가 두 `minmax(calc(50% + 4px), max-content)` 열을 span한다. Ahem max-content inline contribution은 50px이다. 두 max-content track은 그 span 기여에서 4px gutter를 뺀 46px을 대칭 분배해 intrinsic sizing 단계에서 각각 23px을 받으므로 max-content grid content width는 `23 + 4 + 23 = 50px`이다. cyclic percentage는 grid의 intrinsic size 계산에서 `auto`처럼 취급한 다음 결과 grid size를 기준으로 resolve한다. 따라서 각 최종 minimum은 `50% × 50 + 4 = 29px`이다. max-content limit 23px이 minimum보다 작으므로 minmax range의 최댓값도 29px으로 올라간다. 최종 columns는 `[29,29]px`, content root는 `50×10px`, 2px border box는 `54×14px`, spanning item은 `62×10px`이다.

행 fixture는 definite `20px` inline size의 Ahem `a a`를 두 행에 span시킨다. 폭 20px에서 text block max-content contribution은 20px이다. 4px row gap을 뺀 16px을 두 대칭 max-content row가 나눠 intrinsic height `20px`을 만든다. cyclic minimum은 `50% × 20 + 2 = 12px`씩 resolve한다. 최종 rows는 `[12,12]px`, content root는 `20×20px`, border box는 `24×24px`, spanning item은 `20×28px`이다.

Definite-size control은 각 cyclic scene과 같은 `50px` column / `20px` row basis를 직접 지정해 같은 최종 track과 overflow를 확인한다. 네 scene 모두 Chrome 156에서 위 독립 기대값과 일치했다. Chrome은 동작 관찰자이며 규범 기대를 만들어내는 근거로 쓰지 않았다.

| Scene | Intrinsic / definite content size | Final used tracks | Root border box | Spanning item frame |
|---|---:|---|---:|---:|
| cyclic columns | `50×10px` | Columns `[29,29]`, Rows `[10]` | `54×14px` | `(2,2) 62×10px` |
| definite columns | `50×10px` | Columns `[29,29]`, Rows `[10]` | `54×14px` | `(2,28) 62×10px` |
| cyclic rows | `20×20px` | Columns `[20]`, Rows `[12,12]` | `24×24px` | `(2,54) 20×28px` |
| definite rows | `20×20px` | Columns `[20]`, Rows `[12,12]` | `24×24px` | `(2,90) 20×28px` |

## Chrome 관찰값

Chrome은 네 장면 모두 기대 track과 frame을 직렬화했다. column item의 오른쪽 끝은 root border 오른쪽보다 10px 더 나가며, row item의 아래 끝은 root border 아래보다 6px 더 나간다. 후속 Simulator gate는 root-relative column `(58,6)`/`(66,6)`과 row `(8,27)`/`(8,31)`을 쌍으로 표본화한다. 앞 좌표는 border 밖 item 배경 `#e5484d`, 뒤 좌표는 item 밖 page 배경 `#ffffff`여야 한다. 이는 CSS border stroke 자체를 검증하지 않는다.

## 후속 구현 경계

예정된 제한 slice는 이 네 ID, 두 identical `minmax(calc(50% + length), max-content)` track, 한 in-flow span-2 Ahem text item, 한 축별 4px gap, 2px root border만 허용한다. 일반 CSS admission, 다중 span group/source order, nested/anonymous/replaced item, 다른 track grammar·writing mode, WPT 및 C11.4 전체는 포함하지 않는다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator를 사용하고 실기기 검증은 gate에 넣지 않는다.
