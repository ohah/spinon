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

Span-1 text는 column 3의 base size와 growth limit을 20px로 만든다. span-2 group에서는 첫 image contribution 24px이 columns 1–2에 12px씩 기여한다. 두 번째 image contribution 48px은 columns 2–3의 현재 base 합 32px에서 남은 16px을, growth limit이 무한한 column 2에 배분한다. 따라서 두-image track은 `[12,28,20,0]`이다. §11.4는 intrinsic max track sizing function의 초기 growth limit을 무한으로 둔다. span-2 image들의 §11.5 intrinsic-maximum pass에서 무한 limit을 base size로 치환하면 각각 `12+12=24`와 `28+20=48`이므로 추가 분배가 없어 columns 1–2의 growth limit은 무한으로 남는다. 세 번째 span-3 image의 contribution 72px은 첫 세 열의 base 합 60px에서 12px을 더 요구한다. §11.5.1의 up-to-limit 단계에서 columns 1–2는 무한 growth limit이고 column 3은 20px limit에 도달했으므로 12px을 두 무한 track에 6px씩 나눈다. 남은 공간이 없어 beyond-limit 단계는 실행되지 않으며, 규범 기대값은 `[18,34,20,0]`이다. 이 계산 수정은 고정 입력이나 Chrome 관측을 바꾸지 않는다.

| Scene | 독립 기대 Columns | Chrome 155 CSSOM | root-local text range x | root-local image frame |
| --- | --- | --- | ---: | --- |
| `anonymous-text-two-images-forward` | `[12,28,20,0]` | `12px 28px 20px 0px` | 40 | A `(0,0 24×16)`, B `(12,60 48×32)` |
| `two-images-anonymous-text-reversed` | `[12,28,20,0]` | `12px 28px 20px 0px` | 40 | A `(0,0 24×16)`, B `(12,60 48×32)` |
| `anonymous-text-three-images-forward` | `[18,34,20,0]` | `16px 32px 24px 0px` | 48 | A `(0,0 24×16)`, B `(16,60 48×32)`, C `(0,120 72×48)` |
| `three-images-anonymous-text-reversed` | `[18,34,20,0]` | `16px 32px 24px 0px` | 48 | A `(0,0 24×16)`, B `(16,60 48×32)`, C `(0,120 72×48)` |

각 표의 좌표는 scene마다 달라지는 page y와 label inset을 제거한 Grid-local CSS px다. 원본 JSON에는 page 좌표, DOM child order, image natural size, computed placement와 root frame을 보존한다. reverse-order control은 동일한 final track와 item frame을 냈다.

## 해시

| 입력/결과 | SHA-256 |
| --- | --- |
| `grid-track-sizing-c114-anonymous-text-multimage-precomparison.html` | `5710b2726af62529944b21f03b35212d38d4caa90a0774a24f694e837a435c5b` |
| `chrome155-observation.json` | `664f1c2852932e4d4b53ca56ccf427365f97c11c187dd3f224ba4975aea013c6` |
| `chrome155-agent-browser.png` | `3a097777a4a97ee2216f3f5f442ae29cb9f3fef5dc738af289faee5db54b01bd` |
| `chrome156-agent-browser-observation.json` | `edd9548bfeeac0cbef28a419d6f158e55fc5f6c5b7fbee1faa53f7a9763027ec` |
| `c14-1-png-basic.png` | `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2` |
| `Ahem.ttf` | `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448` |

## 2026-10-21 표준 계산 재검토

이 README의 최초 계산은 세-image 장면에서 `[16,32,24,0]`을 규범 기대값으로 적었으나, §11.5.1의 up-to-limit 단계를 건너뛰고 beyond-limit 단계를 먼저 적용한 오류였다. 수정한 계산은 위 표와 같다. CSS Grid 2025 CRD §11.4–§11.5.1의 성장 한도 처리와 분배 순서를 기준으로 하며, 같은 입력의 Chrome 155 값 `[16,32,24,0]`은 관측값으로 그대로 보존한다.

2026-10-21 Agent Browser 0.31.1로 같은 로컬 fixture를 다시 열었다. 실행된 `HeadlessChrome/156.0.0.0`에서 Ahem이 로드됐고 10개 PNG 모두 natural size `24×16`으로 decode됐다. 네 장면의 CSSOM track은 두-image 장면에서 `[12,28,20,0]`, 세-image 장면에서 `[16,32,24,0]`이었다. 관측 JSON은 [chrome156-agent-browser-observation.json](./chrome156-agent-browser-observation.json)에 있다. 이는 Chrome 155 고정 관측의 교차 버전 재현이며, 156 실행으로 155 실행 환경을 대체하지 않는다.

재검토는 이 고정 fixture에서 규범 계산과 Chromium 관측이 다름을 확인한다. Blink의 현재 공개 [Grid track sizing implementation](https://chromium.googlesource.com/chromium/src/+/refs/heads/main/third_party/blink/renderer/core/layout/grid/grid_track_sizing_algorithm.cc)은 up-to-limit 및 beyond-limit 분배 코드를 공개하지만, 이 링크는 고정 Chrome 155 바이너리의 정확한 revision이나 내부 중간 track state를 증명하지 않는다. 따라서 이 한 사례의 관측 차이를 전체 Chrome/CSS Grid 적합성 판정이나 원인 단정으로 일반화하지 않는다.

이 근거는 브라우저의 CSSOM track 관찰이다. Spinon 구현, Android/iOS runtime, WPT 적합성, PNG texel paint, border paint 또는 OS compositor 이후의 pixel 근거가 아니다.
