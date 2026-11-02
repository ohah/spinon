# C11.1 parsed Root HTML Grid percentage gap 사전 비교

## 판정 기준과 출처

- 규범 기준: [CSS Gaps Level 1 §2.3 Percentages In gap Properties](https://www.w3.org/TR/css-gaps-1/#gap-percent). Grid gap percentage는 해당 축 content box 크기를 기준으로 하고, cyclic percentage는 intrinsic contribution 계산 중 0, contents layout에서는 content box로 해석한다. [§2.4 legacy aliases](https://www.w3.org/TR/css-gaps-1/#gap-legacy)는 `grid-gap`을 `gap`의 호환 별칭으로 정의한다.
- 상호 확인: [CSS Grid Layout Module Level 2 §11.1 Gutters](https://www.w3.org/TR/css-grid-2/#gutters). Gap은 track 사이의 고정 크기 gutter로 sizing과 item span에 참여한다.
- 브라우저 oracle: Agent Browser `HeadlessChrome/156.0.0.0`, user agent `Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) HeadlessChrome/156.0.0.0 Safari/537.36`.
- 입력: [`grid-percentage-gap-v1.html`](../../../../tests/fixtures/css/c11/grid-percentage-gap-v1.html), SHA-256 `c79a97f3d5fa55c838e8e1ab5386cec4e3cbb73adee682c614bad0e5cce28a36`.
- Agent Browser 관측 JSON SHA-256: `12de0fa47eb4ae66473c334618e3f1de0d9dc32fb1d77380ca9d39ef1648bb12`.
- Chrome 장면 캡처 SHA-256: `f415a6f8922b7e4e1c3829241a35bd6d5588007f316d6055f259c6b94961a1c4`.
- 화면: [Chrome 156 precomparison](./chrome156-precomparison.png). 원 관측: [JSON](./chrome156-observation.json).

## 독립 기대값과 브라우저 관측

기대값은 CSS Gaps §2.3의 축별 content-box percentage 규칙과 definite Grid sizing으로 계산했다. Chrome은 computed percentage 문자열과 최종 used track/frame을 별도로 반환했다.

| 장면 | 독립 기대값 | Chrome 156 관측 | 결과 |
|---|---|---|---|
| `basis` | 10% of 200px = 20px column gap; 10% of 100px = 10px row gap. Tracks `[40,140]` / `[20,70]`; item origins `(0,0)`, `(60,0)`, `(0,30)`, `(60,30)` | `gridTemplateColumns=40px 140px`; rows `20px 70px`; row/column gaps `10%`; item frames match | 일치 |
| `zero` | 0% resolves to 0px on both axes | `0%`; tracks and item frames unchanged | 일치 |
| `zero-basis` | A definite 0px content width makes a 75% column gap 0px; both zero-width item origins are x=0 | computed `75%`; two `0px` columns; both item origins x=0; root scroll width 0 | 일치 |
| `shorthand` | `gap:5% 10%` means 5px row gap and 20px column gap; tracks `[30,150]` / `[10,85]` | tracks `30px 150px` / `10px 85px`; origins `(0,0)`, `(50,0)`, `(0,15)`, `(50,15)` | 일치 |
| `legacy-alias` | `grid-gap:5% 10%` has the same longhand values and geometry as `gap:5% 10%` | computed row/column `5%`/`10%`; tracks and item origins exactly match `shorthand` | 일치 |
| `over-100` | 120% of 100px = 120px; second item x=130px | computed `120%`; second item x=130px; root scroll width 140px | 일치, clamp 없음 |
| `implicit-row` | 25% of 100px = 25px implicit row; 10px row gap; second row y=30px | implicit rows `20px 25px`; second row y=30px | 일치 |
| `column-flow` | 50% of 200px = 100px implicit column; 20px gap; next column x=60px | columns `40px 100px`; next item x=60px | 일치 |
| `named-area` | 12px column/7px row gap; lower row y=27px; second column x=52px; spanning header width 132px | `scrollWidth=132`, `scrollHeight=77`; header width 132px; side/content positions `(0,27)` / `(52,27)` | 일치 |
| `named-area-px` | Existing px gap route uses 6px columns / 4px rows; lower row y=24px; second column x=46px; header width 126px | same used track sizes and frames; scroll size 126×74px | 일치 |
| `invalid-negative` | Invalid `gap:-10px` declaration is discarded; preceding 5px remains | computed row/column gap `5px`; second item x=15px | 일치 |
| `math-control` | Chrome computes the valid `calc()` value for this size; Spinon Root HTML profile remains out of scope and must reject `column-gap` without partial layout | computed `calc(5% + 2px)`; second item x=22px | 경계 고정 |

이 고정 비교는 12 scene만 다룬다. cyclic/indefinite gap resolution, general author CSS, arbitrary track grammar, WPT runner 적합성, app surface pixel equality를 입증하지 않는다.

## 현행 구현 기준선

현 `main` `227aa285`에서 `root_html_grid_gap_admits_only_finite_nonnegative_px_or_normal`를 실행해 1/1 통과했다. 이 테스트는 `Percentage(0)`도 거부한다. `project_styles`의 Root HTML Grid gate 역시 현재 `Normal`/finite non-negative px만 허용하고, `root_html_named_area` gate는 gap 0만 허용한다. 이는 현재 제한을 확인한 기준선이며 새 구현의 통과 증거가 아니다.

## 후속 구현 계획

세부 수용 범위, 거부 경계, finite extent 검사, 구현·실행 판정은 [C11.1 percentage gap 계획](../../../../plan/c11-root-html-grid-percentage-gap.md)을 따른다. 구현 뒤에는 이 아홉 scene과 Chrome 관측을 재사용하되 implementation runtime/PR 검토는 별도로 한다.
