# C11.4 · three-fit repeated-text row-span `min-height:auto` 사전 비교

## 비교 범위

- 규범 근거는 [CSS Grid Layout Module Level 1 · 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto), [§7.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizing-functions), [§11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items)다. §6.6의 automatic minimum 조건은 non-scrollable item이 해당 축에서 auto 최소 track을 span하고, 여러 track을 span하면 flexible track이 없어야 한다. §7.2에서 `fit-content()` track의 minimum sizing function은 `auto`다. §11.5 spanning contribution은 track 순서·cap별 사용 크기·item span frame을 비교하는 기준으로 삼았다.
- 기준 구현은 Agent Browser가 실행한 HeadlessChrome `157.0.0.0`이다. Terminal Browser는 사용하지 않았다. Chrome 결과는 사전 비교 모델이며 Spinon 구현 결과가 아니다.
- [fixture](../../../../tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-three-fit-auto-minimum-precomparison.html), [capture](./capture-chrome.mjs), [독립 verifier](./verify-chrome-matrix.mjs), [검증 요약](./verification.json), [전체 관측 NDJSON](./chrome157-observation.ndjson), [SHA-256 manifest](./SHA256SUMS)을 보존한다. Fixture SHA-256은 `1ad7b120960c55fc9844b55bdac770817e8604d84f54bbade17aa1260494ef47`, 관측 NDJSON SHA-256은 `74102b7dc44d898655f1309b9d1dc0e478714bcf7842e1144cfd24571099a17d`다.

## 고정 입력과 독립 관계

- Grid는 definite `40×220px`, explicit column `40px`, explicit row `10px`, gap 0, LTR, visible overflow, sparse row flow다. Text payload는 기존 10px monospace / 10px line-height ASCII profile을 쓴다.
- 여섯-member `grid-auto-rows` multiset은 `fit-content()` 3개, `min-content` 1개, `max-content` 1개, `auto` 1개다. 20개 fit-position triple, 나머지 세 track의 여섯 순서, 세 cap(`0px`, `10px`, `20%`)의 여섯 위치 배치를 비교했다.
- Row topology는 ① span-2 item 다섯 개로 여섯 implicit row를 만드는 경우, ② span-2 두 개와 span-3 두 개로 같은 여섯 row를 만드는 경우다. 각 topology는 DOM 정방향/역방향으로 실행한다. `align-content:start|stretch`, `min-height:0|auto`를 교차해 총 `20×6×6×2×4×2 = 11,520` scene, 5,760 paired keys, 720개 고유 track list를 관측했다.
- 독립 판정은 일곱 used row의 유한·비음수 크기, authored track 순서 보존, item frame과 점유 track 합 일치, source-order reverse 불변성, stretch가 auto row에만 남는 공간을 배분하는지, candidate automatic minimum의 payload content floor 준수다. `min-height:auto` 대상은 visible/non-scrollable이며 span에 `fit-content()` 또는 `auto` minimum track이 있고 flexible track이 없는 item으로만 계산했다.

## 관측 결과

- 자동 최소 크기 대상 item은 candidate 24,576회였고 payload floor 아래로 내려간 경우는 0이었다. Baseline과 비교해 text item frame 22,320개가 변했고, 최대 item frame 차이는 70 CSS px, 최대 row-track 차이는 51.656 CSS px였다.
- Span frame과 track 합의 최대 차이는 0.00025 CSS px다. 5,760 source-order pair의 최대 geometry 차이는 0 CSS px다. `align-content:stretch`의 non-auto row 최대 변화는 0 CSS px이며 auto row에는 최대 130 CSS px가 추가됐다.
- Cap permutation 1,920개 비교 그룹 중 896개에서 used geometry가 달랐다. cap 변경이 같은 방식으로 모든 입력에 영향을 준다고 일반화하지 않는다.
- 4× scale로 보여 주는 [대표 Chrome 화면](./chrome157-three-fit-auto-minimum-sample.png)은 읽기 편하게 확대된 단일 fixture 장면이다. 배율 조정은 DOM layout 측정 뒤 screenshot 단계에만 적용했다.

## 수명과 한계

- Agent Browser session `spinon-c114-three-fit-row-span-auto-minimum-20261110`은 capture 종료 후 닫혔다. 뒤이은 `session list --json`은 빈 목록을 반환했다. 이 세션의 Chrome/Agent Browser 작업 프로세스는 남아 있지 않다.
- 본 비교는 Chrome CSS Grid 기준 관측이다. Root HTML RuntimeSession, Android/iOS app, PixelCopy/Metal surface readback, compositor 이후 pixels, 실기기, WPT 또는 일반 CSS admission을 검증하지 않았다. 해당 항목은 구현 단계에서 별도로 확인해야 한다.
