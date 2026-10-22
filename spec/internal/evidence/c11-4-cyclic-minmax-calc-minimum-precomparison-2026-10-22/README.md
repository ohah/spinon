# C11.4 cyclic `minmax(calc())` minimum 사전 비교

**상태:** 계획 단계 입력, 규범에서 도출한 기대값, Chrome 156 Agent Browser 관측과 CSS border/overflow 표본이다. Spinon runtime이나 Android/iOS 실행 결과는 아니다.

## 재현 입력과 환경

- Fixture: [HTML 입력](https://github.com/ohah/spinon/blob/main/tests/fixtures/css/c11/grid-cyclic-minmax-calc-minimum-c114-precomparison.html)
- Browser: Agent Browser `0.31.1`, macOS arm64, `HeadlessChrome/156.0.8078.48`, viewport `1280×633` CSS px, DPR 1.
- Ahem `10px/10px`이 loaded 상태고 `document.fonts.check("10px Ahem")`가 true인 것을 확인했다.
- [Chrome DOM 관측 JSON](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-precomparison-2026-10-22/chrome156-observation.json), [화면](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-precomparison-2026-10-22/chrome156-minmax-calc-minimum.png), [화면 pixel 표본](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-precomparison-2026-10-22/chrome156-pixel-samples.json), [W3C CSS Grid 관측](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-precomparison-2026-10-22/w3c-css-grid-observation.json), [독립 계산](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-precomparison-2026-10-22/spec-derived-expectations.json)을 보존했다.
- 계획 PR의 별도 [적대적 diff 검토 30개](./plan-pr-diff-review-30.md)를 기록했다.
- Ahem SHA-256: `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`.
- 입력 fixture, Ahem, browser observation, screenshot, pixel sample 및 기대값 digest는 [manifest](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-precomparison-2026-10-22/SHA256SUMS.txt)에 있다.
- 규범 기준은 [CSS Grid Layout Module Level 1, 2025 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes)과 [§10.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-grid-sizing)이다. Grid container size를 계산할 때 cyclic track percentage를 auto처럼 다루고, 그 크기를 얻은 뒤 percentages를 resolve한다. `minmax()` 최댓값이 최솟값보다 낮으면 최댓값을 최솟값까지 올린다.

## 독립 기대값과 Chrome 관측

각 root content basis는 10px이다. Ahem 한 글자 `a`의 max-content contribution은 10px이고, 2px border 양쪽을 포함한 root border box는 14px이다. cyclic percentage는 intrinsic root sizing 단계에서 auto처럼 처리되므로 root content size는 10px로 정해진다. final track pass에서는 `200% × 10 + 4 = 24px` 열 minimum과 `200% × 10 + 2 = 22px` 행 minimum을 계산한다. 각 `max-content` maximum 10px은 minimum보다 작으므로 최종 track은 각각 24px/22px이다.

| Scene | Independent expected | Chrome 156 observation |
| --- | --- | --- |
| `cyclic-minmax-calc-min-column` | Columns `[24]`, Rows `[10]`; root content `10×10`, border box `14×14`; item `24×10` | 동일 |
| `cyclic-minmax-calc-min-row` | Columns `[10]`, Rows `[22]`; root content `10×10`, border box `14×14`; item `10×22` | 동일 |
| `definite-minmax-calc-min-column` | Columns `[24]`, Rows `[10]`; root content `10×10`, border box `14×14`; item `24×10` | 동일 |
| `definite-minmax-calc-min-row` | Columns `[10]`, Rows `[22]`; root content `10×10`, border box `14×14`; item `10×22` | 동일 |

Chrome computed border color는 `rgb(240, 198, 116)`이다. PNG의 left border 표본은 `rgb(239, 191, 114)`로 캡처되어 CSS computed color와 차이가 있다. 따라서 screenshot pixel은 border와 item이 덮는 위치를 판별하는 공간 oracle이며, simulator 표본의 exact sRGB 기대값으로 재사용하지 않는다.

| Pixel | Chrome PNG observation | 의미 |
| --- | --- | --- |
| column `(1,5)` | `(239,191,114)` | item이 덮지 않은 left border |
| column `(5,5)` | `(229,72,77)` | item 내부 |
| column `(13,5)` | `(229,72,77)` | overflow item이 right border 위로 그림 |
| column `(16,5)` | `(229,72,77)` | 14px root border box 밖, 24px item 안 |
| column `(27,5)` | `(255,255,255)` | item 바깥 |
| row `(1,30)` | `(239,191,114)` | item이 덮지 않은 left border |
| row `(5,35)` | `(229,72,77)` | overflow item이 bottom border 위로 그림 |
| row `(5,40)` | `(229,72,77)` | 14px root border box 아래, 22px item 안 |

## Agent Browser 종료

전용 namespace `spinon-cyclic-minmax-minimum-plan-20261022`의 `plan` session을 종료했다. 직후 `session info`는 `active=false`, `pid=null`; namespace doctor는 `No active daemons`였다. 종료 후 process scan에서 Agent Browser 프로세스는 찾지 못했다. [종료 기록](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-cyclic-minmax-calc-minimum-precomparison-2026-10-22/browser-close.json).

## 범위

이는 단일 Chrome build의 관측이다. Spinon layout, WPT, 일반 CSS admission, Android/iOS, 실기기, hardware GPU와 iOS compositor-after pixels를 검증하지 않는다. Chrome PNG의 color-managed RGB를 앱의 CSS sRGB surface 기대값이라고 주장하지 않는다.
