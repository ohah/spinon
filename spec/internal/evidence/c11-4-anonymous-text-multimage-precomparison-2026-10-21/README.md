# C11.4 anonymous Grid text와 복수 spanning image 사전 비교

## 실행 조건

- Chrome for Testing `155.0.8059.85`, Agent Browser `0.31.1`, macOS ARM, DPR 1
- viewport `1280×577`; fixture는 viewport 크기에 의존하지 않는다.
- Ahem `10px/10px`: `document.fonts.check('10px Ahem') = true`
- PNG decode 완료: 각 image의 natural size `24×16`
- 입력: [비교 fixture](../../../../tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-multimage-precomparison.html)
- JSON: [chrome155-observation.json](./chrome155-observation.json)
- 화면: [Agent Browser screenshot](./chrome155-agent-browser.png), 전체 네 scene 표시

## 표준 기대값과 브라우저 관찰

기대 track은 CSS Grid 2025 CRD §6, §8.5, §11.4–§11.5.1 규칙에서 독립적으로 계산하고 Chrome 관찰은 별도 열에 대조했다. `XX`의 Ahem min-content는 20px이다. `<img>`의 natural ratio `24:16`과 definite height는 각각 24px·48px·72px의 inline min-content contribution을 준다. explicit image placement가 먼저 점유 칸을 정한 후 anonymous `XX`는 row 1, column 3으로 auto-place된다.

Span-1 text는 column 3을 20px로 만든다. span-2 group에서는 첫 image contribution 24px이 columns 1–2에 12px씩 기여한다. 두 번째 image contribution 48px은 column 3의 20px base 이후 남은 28px을 확장 가능한 column 2에 배분한다. 따라서 두-image track은 `[12,28,20,0]`이다. span-3 image는 첫 세 열의 60px 합에 12px을 더한다. 앞선 span-2 phase에서 growth limit을 채운 track에 대한 §11.5.1 beyond-limit 분배는 각 열에 4px씩 더해 `[16,32,24,0]`을 만든다.

| Scene | 독립 기대 Columns | Chrome 155 CSSOM | root-local text range x | root-local image frame |
| --- | --- | --- | ---: | --- |
| `anonymous-text-two-images-forward` | `[12,28,20,0]` | `12px 28px 20px 0px` | 40 | A `(0,0 24×16)`, B `(12,60 48×32)` |
| `two-images-anonymous-text-reversed` | `[12,28,20,0]` | `12px 28px 20px 0px` | 40 | A `(0,0 24×16)`, B `(12,60 48×32)` |
| `anonymous-text-three-images-forward` | `[16,32,24,0]` | `16px 32px 24px 0px` | 48 | A `(0,0 24×16)`, B `(16,60 48×32)`, C `(0,120 72×48)` |
| `three-images-anonymous-text-reversed` | `[16,32,24,0]` | `16px 32px 24px 0px` | 48 | A `(0,0 24×16)`, B `(16,60 48×32)`, C `(0,120 72×48)` |

각 표의 좌표는 scene마다 달라지는 page y와 label inset을 제거한 Grid-local CSS px다. 원본 JSON에는 page 좌표, DOM child order, image natural size, computed placement와 root frame을 보존한다. reverse-order control은 동일한 final track와 item frame을 냈다.

## 해시

| 입력/결과 | SHA-256 |
| --- | --- |
| `grid-track-sizing-c114-anonymous-text-multimage-precomparison.html` | `5710b2726af62529944b21f03b35212d38d4caa90a0774a24f694e837a435c5b` |
| `chrome155-observation.json` | `664f1c2852932e4d4b53ca56ccf427365f97c11c187dd3f224ba4975aea013c6` |
| `chrome155-agent-browser.png` | `3a097777a4a97ee2216f3f5f442ae29cb9f3fef5dc738af289faee5db54b01bd` |
| `c14-1-png-basic.png` | `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2` |
| `Ahem.ttf` | `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448` |

이 사전 비교는 고정 Chrome 관찰만 기록한다. Spinon 구현, Android/iOS runtime, WPT 적합성, PNG texel paint, border paint 또는 OS compositor 이후의 pixel 근거가 아니다.
