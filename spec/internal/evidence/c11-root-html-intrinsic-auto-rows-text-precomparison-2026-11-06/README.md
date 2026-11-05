# Root HTML text가 있는 `min-content` implicit row: 구현 전 비교

## Fixture와 기준

- Fixture: [root-html-grid-intrinsic-auto-rows-text-precomparison.html](https://github.com/ohah/spinon/blob/main/tests/fixtures/css/c11/root-html-grid-intrinsic-auto-rows-text-precomparison.html)
- SHA-256: `f0759d21c51af46497327686a854b52cd5e004134c35fd562e3e0170c8df49e6`
- 기준: Agent Browser의 HeadlessChrome `156.0.0.0`, DPR 1, viewport 1280×633.
- Browser DOM/CSSOM 관측: [chrome156-observation.json](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-root-html-intrinsic-auto-rows-text-precomparison-2026-11-06/chrome156-observation.json)
- 독립적으로 계산한 기하 기대값: [expected.json](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-root-html-intrinsic-auto-rows-text-precomparison-2026-11-06/expected.json)
- Browser screenshot: [chrome156.png](https://raw.githubusercontent.com/ohah/spinon/main/spec/internal/evidence/c11-root-html-intrinsic-auto-rows-text-precomparison-2026-11-06/chrome156.png)
- 후속 계획: [C11.4 min-content implicit row text](https://github.com/ohah/spinon/blob/main/plan/c11-root-html-intrinsic-auto-rows-text.md)
- 계획 검토 20개: [plan failure review](https://github.com/ohah/spinon/blob/main/plan/c11-root-html-intrinsic-auto-rows-text-plan-review-20.md)
- width 정정 검토 20개: [계획 revision review](https://github.com/ohah/spinon/blob/main/plan/c11-root-html-intrinsic-auto-rows-text-width-revision-review-20.md)
- 계획 PR diff 검토 30개: [plan PR diff review](./plan-pr-diff-review-30.md)
- 규범 기준: [CSS Grid Layout 2025 CRD §11.5, Resolve Intrinsic Track Sizes](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing).

## 고정 입력과 결과

Fixture는 40×40px Grid 하나, explicit 40px 열 하나, explicit 10px 행 하나, row 방향 sparse auto-placement, gap 0, start content alignment, `grid-auto-rows:min-content`를 사용한다. 두 번째 direct `div`는 `width:auto`를 선언하고 기본 `justify-items:normal` stretch에 따라 40px로 resolve된다. 이 item에는 `A A A A A A`만 있고 monospace text 10px와 line-height 10px을 적용한다. Chrome에서는 두 줄로 줄바꿈된다. 첫 번째와 마지막 direct `div`의 높이는 각각 10px과 34px이다.

Chrome 관측 row는 `[10, 20, 34]px`이며 frame은 `first=[0,0,40,10]`, `wrapped-text=[0,10,40,20]`, `last=[0,30,40,34]`이다. Text item은 authored width `auto`, resolved width `40px`를 보고한다. 마지막 row로 Grid의 visible content는 y=64px까지 이어져 40px container보다 24px 더 내려간다. 임시로 `grid-auto-rows:10px`를 적용한 control은 `[10,10,10]px`를 보고하고 마지막 item을 y=20px에 둔다. 두 줄의 text line box가 있어도 text item frame은 10px이다. 이 고정 크기 control로 content-derived row sizing과 고정 추정값을 구별한다.

## 해석과 한계

CSS Grid track-size 계산은 고정 fixture와 관측한 text line layout으로 도출했다. 일반 monospace 측정치가 규범값이라는 주장은 아니다. Android와 iOS 시스템 글꼴 지표는 다를 수 있으므로 runtime 근거에는 플랫폼별 실제 줄 수와 frame을 기록하고 text와 무관한 fixed track은 별도로 비교해야 한다. 이 캡처가 입증하는 것은 Chrome 기하뿐이다. Spinon 동작, WPT 적합성, glyph pixel 일치, PixelCopy/Metal readback 또는 compositor 출력을 증명하지 않는다.
