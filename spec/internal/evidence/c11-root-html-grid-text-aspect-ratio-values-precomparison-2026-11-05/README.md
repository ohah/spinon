# C11.1 Root HTML direct-text `aspect-ratio` 값 · Chrome 사전 비교 · 2026-11-05

이 자료는 기본 parsed Root HTML Grid direct-text 구현이 `2 / 1` 하나만 허용하는 값을 여러 finite positive ratio로 넓힐 계획을 위해 만든 기준 관찰이다. Chrome 관찰은 독립 oracle이며 Spinon의 현재 또는 후보 동작이 아니다.

## 실행 입력과 환경

- Agent Browser 0.31.1 / HeadlessChrome 156.0.0.0, macOS ARM64, viewport 1280×633, DPR 1.
- 입력: [HTML fixture](../../../../tests/fixtures/css/c11/root-html-grid-direct-text-aspect-ratio-values-precomparison.html).
- fixture SHA-256: `f4a528205c3e64cb78267a1c9c9c63778dc117807a348395fb0ef7aa23daa8e8`.
- 모든 장면은 70px Grid, `[10px, 60px]` fixed columns, 한 `auto` row, zero gap, start alignment, visible overflow와 `10px/10px monospace`를 쓴다. target text는 `AAAA`, sibling control은 `A`다.
- [독립 기대값](./expected.json)과 [Chrome DOM/CSSOM 관찰](./chrome-observation.json)은 14 scene 전체를 담는다. [전체 화면](./chrome156-full.png)은 1280×853이다.

## 관찰 결과

Width-locked target은 width 60px에서 `1/2`, `4/3`, `3/2`(pair 및 단일값 `1.5`), `5/2` 비율일 때 높이 각각 120, 45, 40, 24px로 계산된다. `10/1`은 단순히 6px로 줄어들지 않는다. 6px ratio transfer보다 10px text automatic minimum이 커서 target과 auto row는 10px이다. `aspect-ratio:auto` 대조 장면도 10px이다.

Height-locked scene은 첫 `1/2` 장면을 height 80px로 설정해 width 40px이 Ahem의 앱 간 intrinsic 차이에 좌우되지 않도록 했다. height 30px scene은 `4/3`, `3/2`, `5/2`, 기존 회귀 `2/1` 순으로 target width 40, 45, 75, 60px이다. `5/2` target은 70px Grid의 오른쪽 끝을 15px 넘어가며, `overflow:visible`이라 잘리지 않는다. no-ratio height control의 Chrome monospace intrinsic width 24.09375px은 관찰값일 뿐 플랫폼 간 기대값이 아니다.

CSS ratio는 width/height 순서로 적용한다. 이 제한 입력의 결과만 계획 비교 자료로 고정하며, 일반 CSS 적합성, 임의 Grid topology, `auto <ratio>`, min/max transfer, replaced item, Android/iOS runtime, 표면 pixel 또는 compositor 검증을 주장하지 않는다. 구현 전 계획과 검토 기록은 [C11 값 확장 계획](../../../../plan/c11-root-html-grid-direct-text-aspect-ratio-values.md)에 연결한다.
