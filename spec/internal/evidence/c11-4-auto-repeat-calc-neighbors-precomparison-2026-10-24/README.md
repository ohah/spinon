# C11.4 auto-repeat `calc()` 이웃 track 사전 비교

## 범위와 실행 기준

이 자료는 auto-repeat fragment 앞뒤에 제한 `calc(length + percentage)` 이웃 track을 둔 고정 CSS scene의 규범 기대값과 Chrome 관측을 보존한다. Spinon 구현 결과나 지원 완료 판정이 아니다.

- 규범 기준: [CSS Grid Layout Module Level 1, 2025 CRD §7.2.1, §7.2.3.1–§7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat). Auto-repeat은 definite axis에서 content box, non-repeating track과 gutter를 포함해 횟수를 계산한다. Auto-fit은 배치 뒤 비어 있는 반복 track과 인접 gutter를 collapse한다.
- calc 범위 기준: [CSS Values and Units Level 4 §10.12](https://www.w3.org/TR/css-values/#range-checking). `calc()` 최종 값은 property의 닫힌 허용 범위로 clamp된다. 따라서 `<length-percentage [0,∞]>` 이웃의 계산 결과가 음수인 scene은 0px track으로 처리한다.
- 기준 브라우저: Agent Browser 0.31.1, Chrome 156.0.8078.48, `HeadlessChrome/156.0.0.0`, 1280×633 viewport, DPR 1.
- fixture: [14개 고정 scene](../../../../tests/fixtures/css/c11/grid-auto-repeat-calc-neighbors-c114-precomparison.html). 모든 기본 repeat 최소 track은 40px이고 gap은 4px이다. 일반 content-box scene은 240px, border-box scene은 outer 136px에서 8px padding과 2px border를 빼 content basis 116px이다.
- Chrome CSSOM의 `gridTemplateColumns`/`gridTemplateRows`, grid frame과 첫 item frame을 [원본 observation](./chrome156-observation.json)에 저장하고 [Agent Browser/Chrome metadata](./chrome156-browser-metadata.json)를 고정했다. 화면은 [viewport 캡처](./chrome156-viewport.png)와 [전체 scene 캡처](./chrome156-full.png)에 있다.
- 각 grid의 1px outline/background는 캡처에서 frame을 알아보기 위한 시각 표식이며 layout input을 바꾸지 않는다. `col-fill-overflow` item은 suffix track의 x=252px에 있고 grid border-box 오른쪽 x=248px보다 4px 밖이다. 예정 runtime은 item 중심과 root 경계 안·바깥을 surface sample으로 따로 검사해야 한다.
- `validate-precomparison.py`는 fixture 입력에서 repeat 횟수와 track flex share를 독립 계산하고 Chrome CSSOM과 1/64 CSS px 안에서 비교한다. 실행 결과: **14/14 scene 통과**.

## 독립 계산과 Chrome CSSOM

확정 크기 axis에서 최소 track 40px, 4px gap과 이웃의 계산된 used size를 합산해 content box를 넘지 않는 가장 큰 양의 repeat count를 선택한다. 어떤 양의 count도 맞지 않으면 normative 최소 1회를 쓴다. `1fr` used size는 남은 definite free space를 살아 있는 flex track 수로 나눠 계산했다.

| Scene | 규범 기반 기대값 | Chrome 156 CSSOM | 범위 판정 |
|---|---|---|---|
| `col-fill-both` | count 4, `[22,47,47,47,47,10]` | 동일 | 대상 |
| `row-fill-both` | count 4, `[22,47,47,47,47,10]` | 동일 | 대상 |
| `col-fill-prefix` | count 4, `[22,50.5,50.5,50.5,50.5]` | 동일 | 대상 |
| `row-fill-suffix` | count 5, `[42,42,42,42,42,10]` | 동일 | 대상 |
| `col-fit-first` | count 4, `[22,200,0,0,0,10]` | 동일 | 대상; 빈 track과 gutter collapse |
| `col-fit-empty` | count 4, `[22,0,0,0,0,10]` | 동일 | 대상; 모든 빈 반복 track collapse |
| `col-fill-overflow` | count 1, `[196,40,74]`, grid overflow 78px | 동일 | 대상; 최소 1회 overflow |
| `col-fill-border-box` | basis 116, count 2, `[9.8,44.15,44.15,5.9]` | `[9.796875,44.15625,44.15625,5.890625]` | 대상; 1/64px 이내 quantization |
| `col-fill-extra-term` | count 4, `[23,50.25,50.25,50.25,50.25]` | 동일 | CSS-valid negative control; 3-term sum은 계획된 typed subset 밖 |
| `col-fill-negative-term` | count 5, `[10,42,42,42,42,42]` | 동일 | 대상; 음수 length term이 있어도 최종 used size 10px |
| `col-fill-negative-final` | count 5, `[0,44,44,44,44,44]` | 동일 | 대상; 음수 calc 최종값을 0px로 clamp |
| `col-fill-negative-percentage-term` | count 5, `[8,42.4,42.4,42.4,42.4,42.4]` | `[8,42.390625,42.390625,42.390625,42.390625,42.390625]` | CSS-valid negative control; 음수 percentage AST 항은 계획 profile 밖 |
| `col-max-content` | indefinite axis이므로 auto-repeat count 1; cyclic used size는 이 slice에서 계산하지 않음 | `[12.296875,40]` | CSS-valid negative control; indefinite/cyclic basis는 계획 밖 |
| `col-fill-fixed-control` | count 4, `[22,47,47,47,47,10]` | 동일 | 기존 fixed-px 회귀 control |

`col-fill-extra-term`은 CSS-valid 3-term math control, `col-fill-negative-percentage-term`은 CSS-valid 음수 percentage term control, `col-max-content`는 indefinite/cyclic axis control이다. Chrome 156은 각각 `[23,50.25,50.25,50.25,50.25]`, `[8,42.390625,42.390625,42.390625,42.390625,42.390625]`, `[12.296875,40]`을 관찰했다. Chrome track 값은 브라우저 관측이며 규범 계산을 Chrome 값에서 역산하지 않았다. 이 negative control들은 browser CSS 지원 오류가 아니라 Spinon 계획의 좁은 profile 경계다. 실제 runtime이 이 선언을 받으면 현재 계약상 fail-closed해야 한다.

## 제한과 남은 검증

- 이 비교는 Chrome 156의 단일 macOS ARM64 관측이며 WPT나 Android/iOS 앱 runtime을 포함하지 않는다.
- 일반 CSS admission, arbitrary `ComputedCssMath` tree, 3-term·곱셈·함수형 math, negative percentage term, intrinsic/flex neighbor, multiple auto-repeat, indefinite/cyclic percentage basis와 전체 Grid 적합성은 구현 범위 밖이다.
- 양수 definite-axis의 2-term typed sum, 음수 length term, 최종 0px range clamp, 합산·Taffy 표현 overflow gate가 구현 계획이다. CSS cascade AST가 예상 shape와 다르면 해당 값은 조용히 단순화하지 않고 fail-closed하며 범위를 재검토한다.
- 14 scene의 Chrome output은 plan의 acceptance oracle다. C11.4 전체 체크와 공식 완료 집계 `72/213`은 유지한다.
