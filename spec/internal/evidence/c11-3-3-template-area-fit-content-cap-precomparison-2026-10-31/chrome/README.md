# Chrome 156 `fit-content()` cap 사전 비교

## 실행 조건

- Google Chrome `156.0.8078.76`, Agent Browser CLI `0.31.1` headless session.
- 1280×633 CSS px, DPR 1. Terminal Browser는 사용하지 않았다.
- 고정 입력은 [fixture](../grid-template-area-fit-content-cap.html)와 [독립 기대값](../expected-values.json)에 기록했다. `document.fonts.ready` 뒤에 측정했고 Ahem 폰트가 loaded 상태임을 확인했다.
- screenshot은 [chrome-156-agent-browser.png](chrome-156-agent-browser.png)이며 SHA-256은 `dfe52f261926d2961693e3822d9bf50fcad0be89711721ad807c4a0046dd8d52`이다. 빨간 explicit area 20px, 초록 cap area 40px, 뒤쪽 빈 트랙이 보인다. 기계 판독 결과는 [observation.json](observation.json)에 있다.

## 관찰

`grid-template-areas: "fixed fit"`이 2-column explicit grid를 만든다. `grid-template-columns`는 첫 column 20px만 sizing하므로 두 번째 area-created explicit track은 `grid-auto-columns: fit-content(40px)`를 사용한다. 이 장면은 grid 밖 placement를 만들지 않아 positive implicit track이 없다. Ahem 10px의 `AA AA AA AA`는 independent min-content probe에서 20px, max-content probe에서 110px이다. 따라서 Grid fit-content sizing의 기대 폭은 `max(20, min(40, 110)) = 40 CSS px`다.

Chrome computed columns는 `20px 40px`, computed rows는 `60px`이다. area item의 root-relative frame은 `[20, 0, 40, 60]`이다. PNG pixel을 직접 읽어 `(10,30)`은 fixed-area 빨강 `[229,72,77,255]`, `(59,30)`은 cap 안쪽 초록 `[48,164,108,255]`, `(60,30)`은 cap 바깥 root 배경 `[16,24,32,255]`임을 확인했다. 이 입력은 min-content가 cap 이하이고 max-content가 cap을 넘으므로 40px cap을 실제로 구분한다.

## 범위 한계

이는 Chrome의 독립 oracle만 확인한다. Spinon 구현·런타임, Android/iOS Simulator, 표면 pixel, WPT 적합성 또는 일반 CSS Grid 지원을 증명하지 않는다. 이전의 fixed 60px child가 있던 `fit-content(40px)` scene은 min-content 자체가 40px를 초과할 수 있어 cap 증거로 사용하지 않았다.
