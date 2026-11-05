# C11.4 여러 direct text item의 implicit min-content row 사전 비교

**대상 source:** [`root-html-grid-intrinsic-auto-rows-multiple-text-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-intrinsic-auto-rows-multiple-text-precomparison.html) · **SHA-256:** `3232e75cc346718a1a7970ec71b7810b3d34a196caa0919610e5deb454657970` · **기준:** Agent Browser가 연 HeadlessChrome `156.0.0.0`, DPR 1
**관측:** [Chrome JSON](./chrome156-observation.json) · **독립 기대값:** [expected JSON](./expected.json) · **화면:** [Chrome capture](./chrome156.png)

**계획 검토:** [20개 실패 관점](../../../../plan/c11-root-html-intrinsic-auto-rows-multiple-text-plan-review-20.md) · [계획 PR diff 검토 30개](./plan-pr-diff-review-30.md)

고정 40×40px Grid에는 explicit `40px` column, `10px` row, `grid-auto-rows:min-content`를 둔다. Grid 직속 item은 고정 높이 10px의 첫 item, 10px/10px monospace normal-wrap의 `A A A A A A` text item, 같은 text style의 `A` item, 고정 높이 34px의 마지막 item 순서다. Chrome은 resolved text width를 둘 다 40px로 관측했다. 긴 문자열은 두 줄, 짧은 문자열은 한 줄이다.

Chrome의 계산된 행 크기는 `[10,20,10,34]px`, item frame은 `[0,0,40,10]`, `[0,10,40,20]`, `[0,30,40,10]`, `[0,40,40,34]`다. 마지막 item의 하단은 `74px`로 40px Grid 밖으로 34px 흐른다. 이 값은 source order와 서로 다른 두 text row contribution을 검증하기 위한 독립 기대값의 교차 확인이다.

`grid-auto-rows:10px` control은 computed rows `[10,10,10,10]px`를 만들고 마지막 item을 y=30px에 둔다. 따라서 min-content row 계산은 고정 track control과 다른 관측 가능한 layout을 만든다.

Agent Browser에서 두 text item의 DOM source order만 역전하면 computed rows도 `[10,10,20,34]px`로 바뀌고, 짧은 text row는 y=10px, 긴 text row는 y=20px에 놓인다. 마지막 item은 y=40px를 유지한다. 구현은 row 기여가 각 text item의 identity/source order를 따라가는지 이 대조와 비교한다.

브라우저 제어는 `agent-browser --session spinon-c11-multiple-text`를 사용했다. 최종 관측·screenshot viewport는 160×100 CSS px이며 DPR 1이다. root/item CSS geometry는 viewport 축소 전후 동일했다. 각 scene 관측 후 Agent Browser session을 닫았고 `pgrep`에서 Agent Browser/Chrome 프로세스가 남지 않은 것을 확인했다. Screenshot은 참고 화면이다. 이 사전 비교는 Spinon 실행, WPT 적합성, glyph pixels, Android/iOS surface readback 또는 compositor parity를 주장하지 않는다.
