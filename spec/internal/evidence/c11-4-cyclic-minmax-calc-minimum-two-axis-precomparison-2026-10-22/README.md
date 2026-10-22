# C11.4 양축 cyclic `minmax(calc())` minimum 사전 비교

**상태:** 계획 단계의 네 고정 입력, CSS Grid 2025 CRD에서 도출한 기대값, Chrome 156 Agent Browser 관측과 화면 표본이다. Spinon runtime 및 Android/iOS 실행 결과가 아니다.

## 실행 환경과 입력

- Fixture: [`grid-cyclic-minmax-calc-minimum-two-axis-c114-precomparison.html`](../../../../tests/fixtures/css/c11/grid-cyclic-minmax-calc-minimum-two-axis-c114-precomparison.html)
- Browser: Agent Browser `0.31.1`, Chrome `156.0.8078.48`, macOS arm64, viewport `1280×633` CSS px, DPR 1.
- `document.fonts.ready` 완료, `document.fonts.check("10px Ahem") === true`.
- Ahem SHA-256: `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`.
- [독립 기대값](./spec-derived-expectations.json), [Chrome DOM 관측](./chrome156-observation.json), [W3C CRD 관측](./w3c-css-grid-observation.json), [화면](./chrome156-cyclic-minmax-calc-minimum-two-axis.png), [화면 표본](./chrome156-pixel-samples.json), [digest](./SHA256SUMS.txt).

## 규범 기준과 독립 계산

[CSS Grid Layout Module Level 1, 2025 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes)는 column percentage를 grid container의 inner inline size, row percentage를 inner block size에 상대적으로 정의한다. Intrinsic container size가 해당 track에 의존하면 percentage는 intrinsic 계산 중 `auto`처럼 처리되고 결과 container size로 layout 때 다시 resolve된다. `minmax()` maximum이 minimum보다 낮으면 minimum까지 올린다. [Grid Sizing Algorithm](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-grid-sizing)은 columns sizing 다음 rows sizing 순서를 정의한다.

Ahem `aa aa`는 한 줄의 max-content width가 50px이다. 양축 cyclic scene의 intrinsic root content box는 50×10px이므로 최종 minimum은 column `200% × 50 + 4 = 104px`, row `200% × 10 + 2 = 22px`이다. Row-only control은 10px column에서 두 text line으로 20px block size를 만들며, cyclic row minimum은 `200% × 20 + 2 = 42px`이다. 2px border를 percentage basis에 더하지 않는다.

## 관측 결과

| Scene | 독립 기대 | Chrome 156 관측 |
| --- | --- | --- |
| `cyclic-minmax-calc-min-both-axes` | columns `[104]`, rows `[22]`; root content `50×10`, border box `54×14`; item `104×22` | 동일 |
| `cyclic-minmax-calc-min-column-definite-row` | `[104]` / `[10]`; root `50×10`; item `104×10` | 동일 |
| `definite-column-cyclic-minmax-calc-min-row` | `[10]` / `[42]`; root `10×20`, border box `14×24`; item `10×42` | 동일 |
| `definite-minmax-calc-min-both-axes` | `[24]` / `[22]`; root `10×10`; item `24×22` | 동일 |

Chrome screenshot 표본은 combined scene의 오른쪽과 아래에서 root border box 바깥까지 item 배경이 이어지고 item 끝 바깥은 흰색임을 보인다. 화면의 왼쪽 border sample `[239,191,114]`은 computed CSS border `rgb(240,198,116)`과 다르므로 pixel capture를 CSS 색상 exact 기준으로 쓰지 않는다.

## Agent Browser 종료

전용 namespace의 `plan` session을 닫았다. 직후 session info는 `active=false`, `pid=null`, session list는 비어 있었고 Agent Browser/Chromium 프로세스가 남지 않은 것을 확인했다. [종료 상태](./browser-close.json).

## 범위

이는 고정 입력에 대한 한 Chrome build의 DOM observation이다. WPT, Spinon 구현, 일반 CSS admission, Android/iOS, 실기기, hardware GPU, iOS compositor-after pixel 및 C11.4 완료를 검증하지 않는다.
