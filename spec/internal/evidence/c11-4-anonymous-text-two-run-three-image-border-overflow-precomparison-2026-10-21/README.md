# C11.4 two anonymous text run과 세 image bordered Grid 사전 비교

## 실행 조건

- Agent Browser `0.31.1`, `HeadlessChrome/156.0.0.0`, macOS ARM, viewport `1280×720`
- Ahem `10px/10px` font load 완료; 고정 PNG 6개 모두 decode 완료, natural size `24×16`
- 고정 입력: [fixture](../../../../tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-two-run-three-image-border-overflow.html)
- CSSOM 관측: [JSON](./chrome156-observation.json)
- 화면: [Agent Browser screenshot](./chrome156-agent-browser.png)

## 표준 기대값과 브라우저 관측

독립 기대값은 [CSS Grid 2025 CRD §6, §8.5, §11.4–§11.5.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)에서 계산한다. 두 직접 text sequence는 각각 anonymous Grid item이며, 명시 배치된 세 image가 차지한 칸 뒤에 row 1 column 3과 4로 자동 배치된다. Ahem `XX`의 min-content 기여는 각각 20px이다. PNG natural ratio `24:16`과 image별 definite height는 inline min-content contribution `24px`, `48px`, `72px`를 준다.

첫 span-2 image는 columns 1–2를 `[12,12]`로 만든다. 둘째 span-2 image는 columns 2–3의 base 합 32px에 16px을 더해 `[12,28,20]`을 만든다. 앞선 image 기여가 growth-limit pass를 넘기지 않아 columns 1–2의 growth limit은 무한으로 유지된다. 세 번째 span-3 image의 72px 기여에서 첫 세 열의 60px base 합을 빼면 12px이다. §11.5.1 up-to-limit 분배는 무한 growth limit인 columns 1–2에 6px씩 더하고, text가 있는 column 3은 20px에서 멈춘다. 따라서 독립 기대 track은 `[18,34,20,20]`이다. 네 번째 track은 두 번째 anonymous text item의 20px min-content다.

Chrome 156은 forward/reversed 두 장면에서 CSSOM track `[16,32,24,20]`을 관측했다. 이는 2026-10-21의 Chrome 155 세-image 관측 `[16,32,24,0]`과 같은 첫 세 열 차이를 보인다. CSSOM은 내부 intermediate growth limit을 노출하지 않으므로 Blink의 원인을 확정하거나 전체 CSS Grid 적합성을 판정하지 않는다.

| Scene | 독립 기대 Columns | HeadlessChrome 156 CSSOM | Chrome text range x | Grid border box | 세 번째 image |
| --- | --- | --- | --- | --- | --- |
| `two-anonymous-runs-three-images-forward` | `[18,34,20,20]` | `[16,32,24,20]` | `54`, `78` | `(4,22) 40×184` | `(6,144) 72×48` |
| `three-images-two-anonymous-runs-reversed` | `[18,34,20,20]` | `[16,32,24,20]` | `54`, `78` | `(4,282) 40×184` | `(6,404) 72×48` |

두 장면은 같은 2px `#f0c674` border와 `40×184px` border-box를 사용한다. 첫 장면의 grid-local border box 기준 third image frame은 `(2,122) 72×48px`이고 오른쪽 끝은 x=74다. border box의 오른쪽 끝 x=40을 34px 넘는다. root-local `(62,128)` 표본은 오른쪽 border edge에서 22px 바깥이며 PNG의 source pixel `(20,2)`에 대응한다. 해당 source texel은 `#00ff40`이다. viewport 좌표는 root 원점 `(4,22)`를 더한 `(66,150)`이다.

## 해시

| 입력/결과 | SHA-256 |
| --- | --- |
| `grid-track-sizing-c114-anonymous-text-two-run-three-image-border-overflow.html` | `81badda027413c1778568db999c365ea8a72382735c71b39eb49fb7f59209694` |
| `chrome156-observation.json` | `f6754459b2e1dd3fadd0446e5849872a59d15435e2385a2181033ab79ad5091a` |
| `chrome156-agent-browser.png` | `d04395eb0f75615a5ee766515ac4cc9dcd40c757c2130d0961d6d639b5765fd6` |
| `c14-1-png-basic.png` | `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2` |
| `Ahem.ttf` | `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448` |

이 사전 비교는 Chromium CSSOM 관측과 독립 규범 계산만 다룬다. Spinon 앱 runtime, Android/iOS surface sample, PNG texel paint, WPT 적합성 또는 전체 C11.4 완료의 근거가 아니다.
