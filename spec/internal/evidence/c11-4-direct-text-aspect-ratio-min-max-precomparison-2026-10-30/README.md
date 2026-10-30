# C11.4 direct-text aspect-ratio min/max 사전 비교

**기준:** 2026-10-30 · 구현 PR 전 · Agent Browser / HeadlessChrome 156 · CSS px

이 비교는 기존 direct-text Grid fixture가 거부하는 item `min-width`, `max-width`, `min-height`, `max-height` 입력을 구현 전에 고정한다. 대상은 Ahem 텍스트를 가진 non-replaced HTML Grid item이며, replaced-image의 transferred size suggestion이나 Spinon/WPT 적합성 결과가 아니다.

## 기준 문서와 계산 모델

[CSS Grid Layout Level 2 §6.6](https://drafts.csswg.org/css-grid/#min-size-auto)은 Grid item의 content-based minimum에서 non-replaced item의 `content size suggestion`을 relevant-axis min-content size로 정의하고, preferred aspect ratio가 있으면 반대 축 definite min/max를 ratio로 환산해 clamp하도록 한다. 이 흐름은 replaced element에 한정된 `transferred size suggestion`과 구별한다. [CSS Sizing Level 4](https://drafts.csswg.org/css-sizing-4/#aspect-ratio)는 ratio conversion에서 `auto <ratio>`가 content-box 크기를 사용하고 숫자 ratio가 `box-sizing`으로 정한 box를 사용하는 기준을 제공한다. 두 링크는 작업 시점 공개 초안이며 이 고정 fixture를 넘어서는 지원 선언이 아니다.

입력의 `AAAA`는 Ahem 10px에서 40px, `A`는 10px이다. Ratio는 2:1이다. 따라서 폭 방향 definite `min-width:60px`은 세로 기여 30px에 대응하고, `max-width:30px`은 15px에 대응한다. `min-height:30px`은 폭 60px에 대응하고 `max-height:15px`은 폭 30px에 대응한다. 기본 `min-width:auto`는 min-content floor를 유지하므로 `max-height:15px` 장면의 폭은 40px이며, 같은 장면에 `min-width:0`을 둔 control은 환산된 최대 폭 30px을 드러낸다.

Nonzero edge control은 `padding:2px 4px`와 1px border를 사용한다. 가로 edge 합은 10px, 세로 edge 합은 6px이다. `auto 2 / 1`에서 border-box width 60px은 content width 50px을 남겨 content height 25px, outer height 31px을 만든다. Numeric border-box ratio의 대응 row height는 30px이다. 마지막 max-height control은 auto ratio의 content-box basis 때문에 content `18×9px`와 edge를 합쳐 item `28×15px`이 된다.

## 고정 입력과 브라우저 관측

Canonical input은 [`grid-direct-text-aspect-ratio-min-max-precomparison.html`](../../../../tests/fixtures/css/c11/grid-direct-text-aspect-ratio-min-max-precomparison.html)이다. Root는 width `100px`, columns `max-content max-content`, rows `auto`, gap 0이다. 각 root에는 direct text `AAAA`와 `A`가 들어간다. Canonical fixture의 local Ahem font와 [embedded-font capture](./chrome-oracle.html)를 각각 실행했다. Browser 결과는 ID 순서·computed ratio·track list·root/item frame이 17/17 정확히 같았고 양쪽 `document.fonts.check("10px Ahem")`는 true였다.

| Case | columns | row | ratio item | 요점 |
|---|---:|---:|---:|---|
| `numeric-baseline` | `40px 10px` | `20px` | `40×20` | numeric 2:1 baseline |
| `min-width-60` | `60px 10px` | `30px` | `60×30` | opposite-axis minimum |
| `max-width-30-min-width-0` | `30px 10px` | `15px` | `30×15` | opposite-axis maximum |
| `min-height-30` | `60px 10px` | `30px` | `60×30` | vertical minimum converts to width |
| `max-height-15-min-height-0` | `40px 10px` | `15px` | `40×15` | auto min-content floor stays active |
| `max-height-15-min-height-0-min-width-0` | `30px 10px` | `15px` | `30×15` | explicit zero minimum exposes transfer |
| `min-width-60-clamped-by-max-height-20` | `60px 10px` | `20px` | `60×20` | opposite maximum caps converted minimum |
| `min-height-20-clamps-transferred-max-width-30` | `30px 10px` | `20px` | `30×20` | opposite minimum floors converted maximum |
| `source-min-width-60-over-max-width-40` | `60px 10px` | `30px` | `60×30` | source min exceeds source max |
| `source-min-height-30-over-max-height-20` | `60px 10px` | `30px` | `60×30` | vertical min exceeds vertical max |
| `auto-ratio-min-width-60` | `60px 10px` | `30px` | `60×30` | `auto <ratio>` without edges |
| `no-ratio-min-width-60` | `60px 10px` | `10px` | `60×10` | no ratio must not transfer |
| `no-ratio-min-height-30` | `40px 10px` | `30px` | `40×30` | no ratio keeps axes independent |
| `numeric-border-box-min-width-60` | `60px 10px` | `30px` | `60×30` | numeric border-box with edges |
| `auto-content-box-min-width-60` | `60px 10px` | `31px` | `60×31` | auto ratio uses content box |
| `numeric-border-box-max-height-15-min-controls` | `30px 10px` | `15px` | `30×15` | numeric ratio with explicit zero mins |
| `auto-content-box-max-height-15-min-controls` | `28px 10px` | `15px` | `28×15` | auto ratio plus content-box edges |

정확한 expected JSON과 관측 JSON은 [`expected-values.json`](./expected-values.json), [`chrome-observation-agent-browser-auto-track.json`](./chrome-observation-agent-browser-auto-track.json), [`chrome-observation-embedded-auto-track.json`](./chrome-observation-embedded-auto-track.json)에 있다. Agent Browser 원본 stdout도 각 `.raw.json`에 보존했다. [`capture-source-comparison.json`](./capture-source-comparison.json)은 font source 간 동일 결과 및 source hash를 담는다. 전체 실행 화면은 [`chrome-oracle-agent-browser-auto-track.png`](./chrome-oracle-agent-browser-auto-track.png)다.

## 재검증과 hash

다음 명령은 고정 expected 17개, canonical/embedded raw·normalized 결과 동등성, browser/font readiness, fixture/font hash를 검사한다. 마지막 1px height 변경 negative control은 verifier가 잘못된 결과를 거부하는지 확인한다.

```sh
node spec/internal/evidence/c11-4-direct-text-aspect-ratio-min-max-precomparison-2026-10-30/verify-precomparison.mjs
```

현재 source hash:

| Artifact | SHA-256 |
|---|---|
| Canonical HTML fixture | `79f2e9994ba2fcff01d521e6c228b2526da9b93fd2b973f139e761838f95fcfc` |
| Embedded-font HTML capture | `c7251ac14aea07cbf09bec9b0ddf16c97a6f0aac16d16ac8f5ea8c6b1669f7f4` |
| Ahem font | `b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448` |
| Canonical observation JSON | `4784a5fc2ca9fb37f84f6df24c00c56201c7b0b8ebe499281a02a4ca4a2569bd` |
| Embedded-font observation JSON | `76d415520fba6badb64acfc6d2e1da696174b8ce10c6534938b7626ce804e711` |
| Canonical raw Agent Browser output | `b3864a99ba728a97b15127accc9e342e7cb554330172dfe221657688580c7d7b` |
| Embedded-font raw Agent Browser output | `b3864a99ba728a97b15127accc9e342e7cb554330172dfe221657688580c7d7b` |
| Expected values JSON | `66a67b95a701b4e3456f875cace58550362709c6d8a34e72f29157ae01a3e695` |
| Full-page screenshot | `d57e4de66970e399b1edc113e09c5ceda095b42eb4e462c8659f1bacd3363539` |

Agent Browser session `spinon-c11-4-direct-text-ratio-mm-final-20261030` opened the canonical fixture and embedded-font copy sequentially, then was closed. The final session query returned `active=false` and `pid=null`. The browser oracle used no terminal-browser session.

## 연속 px 범위 추가 oracle

초기 17개 oracle이 30px·60px 등 일부 값만 사용해 `0..=60px` 전체를 연속적으로 허용하는 구현 경계를 충분히 고정하지 못했다. 구현 review 뒤 같은 Ahem 10px 조건의 두 입력을 Agent Browser HeadlessChrome 156에서 추가 관측했다. Canonical 17개 fixture와 expected JSON은 그대로 둔다.

| 추가 case | CSS 제약 | columns | row | ratio item |
|---|---|---|---|---|
| `min-width-45` | `min-width:45px` | `45px 10px` | `22.5px` | `45×22.5px` |
| `max-height-17-75` | `min-width:0; min-height:0; max-height:17.75px` | `35.5px 10px` | `17.75px` | `35.5×17.75px` |

두 case 모두 `fontReady:true`였고 전체 root/child frame은 [추가 관측 JSON](./extended-range-oracle-observation.json)에 있다. 입력과 browser-side 측정기는 [HTML](./extended-range-oracle.html), 화면은 [PNG](./extended-range-oracle.png)다. 실제 구현은 fixture contract의 finite `0..=60px` 범위에서 이처럼 정수가 아닌 CSS px도 받으며 percentage, `calc()`, 음수, 60px 초과 및 비유한 값은 계속 거부한다.

## 범위 한계

이 evidence는 Chrome 156의 고정 입력 관측이다. Spinon runtime, Android/iOS, replaced image, 임의 font/text, percentages, `calc()`, arbitrary Grid structure, 전체 WPT, iOS compositor 이후 pixels, hardware GPU 또는 C11/C11.4 전체 지원을 증명하지 않는다. 구현 acceptance는 Chrome geometry와 함께 CSS Grid §6.6 단계별 runtime trace 및 unsupported-input 거부를 확인해야 한다.
