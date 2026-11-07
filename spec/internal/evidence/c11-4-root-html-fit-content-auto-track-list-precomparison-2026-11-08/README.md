# C11.4 Root HTML fit-content list `auto` track · 사전 비교

## 목적과 경계

현재 기본 parsed Root HTML profile은 fit-content implicit-row 목록에 finite non-negative `fit-content(px|%)`, fixed-px, direct percentage를 허용한다. 유효한 `auto` member를 한 목록에서 거부하는 결손을 고정한다. 이 evidence는 Chrome reference와 독립 계산만 기록하며 Spinon 구현 통과, WPT conformance 또는 C11.4 완료 근거가 아니다.

## 입력

- Fixture: [`root-html-grid-fit-content-auto-track-list-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-fit-content-auto-track-list-precomparison.html)
- Fixture SHA-256: `737e6e85256d3a67e3f0fdf1ce82848573754b40bc5c7983dc9c3f7257531eeb`
- Browser: Agent Browser 0.31.1, HeadlessChrome 157.0.0.0, macOS arm64
- Viewport: 1280×633, DPR 1
- Raw CSSOM, used tracks, item/payload frames: [`chrome-observation.json`](./chrome-observation.json)
- Independent expectations and sizing basis: [`expected-values.json`](./expected-values.json)
- Screenshot: [`chrome-precomparison.png`](./chrome-precomparison.png), 1280×633, SHA-256 `fc6a5e54d0b47eda43913be5d9aa614c816d9ccede400e2e55df527be0d815c2`

Each grid is 40×100px with a 40px explicit column, a 10px explicit row, zero edges/gaps, `align-content:start`, and visible overflow. Its four implicitly placed auto-height items have `min-height:0`; each wraps one nested block with a fixed height. The definite percentage basis is the 100px content-box block size.

## 관측과 독립 기대값

| Scene | Computed `grid-auto-rows` | Chrome used rows | 독립 used rows |
| --- | --- | --- | --- |
| fit, auto, fixed | `fit-content(20%) auto 8px` | `10px 20px 30px 8px 20px` | `[10, 20, 30, 8, 20]px` |
| auto, fit, fixed | `auto fit-content(20%) 8px` | `10px 30px 20px 8px 25px` | `[10, 30, 20, 8, 25]px` |
| zero fit, auto, fixed | `fit-content(0%) auto 8px` | `10px 0px 30px 8px 0px` | `[10, 0, 30, 8, 0]px` |

Independent fit-content calculation uses `max(auto minimum, min(limit, max-content))`: the `20%` limit resolves to 20px and `0%` to 0px, while the item's explicit `min-height:0` fixes the automatic minimum at 0px for this profile. The auto-track max sizing function uses the largest max-content contribution of its items; the fixed payloads provide 30px or 25px. Available height is sufficient for all track growth limits in each scene. `align-content:start` does not invoke the additional stretch step for auto-max tracks, so leftover space remains unused.

Chrome and the independently computed expected tracks and root-relative item/payload frames agree exactly. See the machine-readable evidence for every node frame. The screenshot is a visual fixture view, not a pixel oracle.

The initial exploratory browser pass exposed a fixture omission: without an authored `grid-auto-rows` declaration Chrome correctly observed the initial `auto` value in all scenes. That run was discarded; each scene was corrected to declare its exact pattern. The final capture verifies the three computed patterns above before treating its used values as evidence.

## 다음 관문

The implementation plan and its separate 20-point review are in [`plan/c11-4-root-html-fit-content-auto-track-list.md`](../../../../plan/c11-4-root-html-fit-content-auto-track-list.md) and [`plan/c11-4-root-html-fit-content-auto-track-list-plan-review-20.md`](../../../../plan/c11-4-root-html-fit-content-auto-track-list-plan-review-20.md). This precomparison is not implementation/runtime verification. Android/iOS execution, implementation review, PR diff review and merge-state checks remain separate.
