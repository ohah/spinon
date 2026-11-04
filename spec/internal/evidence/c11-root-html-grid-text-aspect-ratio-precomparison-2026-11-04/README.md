# 기본 parsed Root HTML Grid direct-text aspect-ratio 사전 비교

**계획:** [기본 parsed Root HTML Grid direct-text aspect-ratio](../../../../plan/c11-root-html-grid-text-aspect-ratio.md)

**계획 PR diff 검토:** [독립 20개](./plan-pr-diff-review-20.md)

## 입력과 환경

- HTML 입력: [`root-html-grid-direct-text-aspect-ratio-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-direct-text-aspect-ratio-precomparison.html)
- Browser: HeadlessChrome `156.0.0.0`, Agent Browser CLI
- viewport: `1280×633 CSS px`
- Ahem: 10px font check `true`; `A` advance 10px, `AAAA` content width 40px, line-height 10px
- HTML에는 네 개의 Grid root가 body의 direct child로 있다. 각각의 Grid는 direct HTML block item `A`와 `AAAA`를 가진다. Ratio 장면마다 owner 한 개에 ratio item 한 개만 둔다.
- Root와 item에는 margin/padding/border가 없다. Grid-item background-color만 scene 식별용으로 다르며 sizing에는 영향을 주지 않는다.
- 입력 fixture SHA-256: `f43743af899f73083b66df6612f98d440f4cfdd9c013945b3234a61ee8416bec`
- Ahem SHA-256: `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448`

## 독립 기대값과 관찰

Grid columns `[10px,60px]`, width `70px`, gap `0`, single auto row, start alignment를 먼저 고정했다. Numeric ratio는 width/height `2/1`이다. CSS Grid Layout 2025 CRD §6.2의 Grid-item auto sizing과 CSS Sizing 4 2026 WD §4.2의 preferred-ratio automatic-size 계산을 구분해 참고했다. Sizing 문서는 Working Draft이므로 그 조항만으로 안정된 normative support를 주장하지 않는다.

`width-ratio`에서 `AAAA`의 definite width `60px`는 ratio로 height `30px`에 대응하며 auto row는 30px다. 같은 width의 ratio 없는 대조군은 Ahem line-height 10px로 row 10px다. `height-ratio`에서 definite height `30px`는 width 60px가 되고, ratio 없는 대조군은 Ahem `AAAA` content width 40px다. 이 계산은 Chrome 결과를 복사하지 않고 입력 크기·ratio·glyph metrics에서 산술로 도출했다.

Chrome 156은 font-ready 상태에서 네 scene 전부 track 및 root/item frame이 `expected.json`과 `0 CSS px` 차이로 일치했다. 전체 DOM/CSSOM 결과는 [`chrome-observation.json`](./chrome-observation.json), 시각 기록은 [Agent Browser screenshot](./chrome-156-agent-browser.png)에 있다. Screenshot은 화면 구성을 확인하는 자료이며 pixel-diff/WPT 결과가 아니다.

![Chrome 156 Agent Browser precomparison](./chrome-156-agent-browser.png)

`SHA256SUMS`는 이 입력·관찰·기대값·화면과 Ahem resource를 고정한다. Browser capture 뒤 해당 Agent Browser session을 닫았고, 종료 확인 때 agent-browser/전용 HeadlessChrome 프로세스는 남아 있지 않았다.
