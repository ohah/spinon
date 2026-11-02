# C11.1 Root HTML explicit `fit-content()` 사전 비교

## 기준 입력

- [고정 HTML fixture](root-html-grid-fit-content-precomparison.html)와 [규범 기반 기대값](expected.json)은 parsed Root HTML의 explicit columns/rows, 고정 grid 크기, Ahem `AA AA AA AA` 텍스트를 사용한다.
- 여섯 장면은 cap이 intrinsic minimum 아래인 열, min/max 사이인 열, max-content보다 큰 열, fixed integer `repeat()`, 기존 고정 named-area map 조합, 그리고 row의 min-content floor를 각각 확인한다.
- Ahem font는 `tests/fixtures/resources/c15/Ahem.ttf`, SHA-256 `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`이다. `document.fonts.ready` 뒤 loaded 상태를 확인했다.
- 독립 계산은 CSS Grid Level 1 §7.2.1의 `max(minimum, min(limit, max-content))` formula를 따른다. `minimum`은 auto minimum이며 다른 입력에서는 항상 min-content와 같지는 않다. 여기서는 non-scrollable Ahem text의 별도 min-content/max-content width probes가 20px/110px이고 `width:20px;height:min-content` probe가 row minimum 40px이다. 세 cap은 각각 `max(20,min(10,110))=20`, `max(20,min(40,110))=40`, `max(20,min(120,110))=110`이다. `fit-content(20px)` row는 40px floor를 유지한다.

## Chrome 관찰

- Google Chrome 156.0.8078.80, Agent Browser CLI 0.31.1, headless, 1280×633 CSS px, DPR 1을 사용했다. Terminal Browser는 사용하지 않았다.
- computed columns는 cap 장면 순서대로 `20px 40px`, `20px`, `110px`, repeat 장면 `40px 40px`, area 장면 `20px 40px`이다. Row 장면은 `20px` columns와 `40px` rows를 반환했다. 세 intrinsic probes와 여섯 장면 item frame은 [`chrome-observation.json`](chrome-observation.json)에 각각 보존했다.
- [화면 캡처](chrome-156-agent-browser.png)에서 `(10,30)`은 fixed item 빨강 `[229,72,77,255]`, `(59,30)`은 fit-content item 초록 `[48,164,108,255]`, `(60,30)`은 grid 바탕색 `[16,24,32,255]`이다. PNG SHA-256은 `4e7e3cc9d32e1a75c258e6fca5bb52c448fd1c156783d3d7b5eeb0d615577e54`이다.
- fixture SHA-256은 `809fdc36815e4ce0386f75d3fbf6ec82e9a55b1676070febd0605744566f8dfc`이다.

## 범위와 한계

이는 Chrome 기준 관찰과 규범 기반 독립 계산이다. Spinon 구현, Android/iOS runtime, simulator surface, 일반 CSS admission 또는 WPT 적합성을 검증하지 않는다. 후보 지원은 explicit row/column track의 finite non-negative px argument 및 이미 지원되는 fixed integer `repeat()` 확장에 한정한다. Named-area 장면은 track 수와 맞는 기존 고정 map 한 개로 제한한다. Percentage/calc argument, auto tracks, named lines, auto-repeat, area-created unsized tracks, implicit tracks, overflow와 aspect-ratio 영향을 받는 automatic minimum은 범위 밖이다.
