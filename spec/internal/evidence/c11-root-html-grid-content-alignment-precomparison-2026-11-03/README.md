# Root HTML Grid content alignment 사전 비교

## 환경과 입력

- 기준 구현: Agent Browser에서 실행한 `HeadlessChrome/156.0.0.0`.
- DPR 1, viewport 1280×633 CSS px.
- 고정 fixture: [`root-html-grid-content-alignment.html`](../../../../tests/fixtures/css/c11/root-html-grid-content-alignment.html). 초기 축별 Chrome matrix는 속성을 하나씩 바꿔 관찰했다. 최종 동시 정렬 fixture 사후 관찰은 [별도 기록](chrome-post-fixture-validation/README.md)에 둔다.
- 공통 고정 track은 content box 240×180px, columns 40px 60px, rows 30px 50px, 양 축 gap 10px, 네 direct cell item 10×10px이다. 양의 free space는 inline 130px, block 90px이다.
- 음수 free-space 대조는 같은 track·gap을 80×70px container에 놓아 inline −30px, block −20px를 만든다.
- auto-track 관측은 Agent Browser에서 fifth item과 `grid-auto-flow`/computed alignment를 바꿔 본 진단이다. 고정 fixture source에는 포함하지 않았고 이번 고정-px slice의 판정 대상도 아니다.

## 독립 기대값과 관찰값

- spec-derived formulas와 concise expected frames: [`expected-values.json`](expected-values.json).
- 원본 Chrome computed styles, used track serialization, element frames 및 viewport: [`chrome/observation.json`](chrome/observation.json).
- 화면 캡처: [`positive-space-evenly-center.png`](chrome/positive-space-evenly-center.png), [`negative-safe-unsafe-center.png`](chrome/negative-safe-unsafe-center.png).
- 고정 track들의 initial extent는 inline `40 + 10 + 60 = 110px`, block `30 + 10 + 50 = 90px`이다. fixed track 시작점은 computed track origin에 item의 0 offset을 더해 비교한다.
- Chrome의 `space-evenly` inline 위치는 `[43.328125, 136.65625]px`; 수식상 `[130/3, 410/3]px`와의 최대 차이는 1/64 CSS px 이내다.
- unsafe center/end는 negative free space에서도 넘침을 보존한다. safe center/end는 각 축의 start origin으로 돌아간다.
- 최종 fixture는 두 속성을 함께 지정해 `justify-content: space-evenly`와 `align-content: space-around`를 확인했다. Chrome 사후 캡처와 그때의 fixture hash는 [`chrome-post-fixture-validation`](chrome-post-fixture-validation/README.md)에 기록했다.

## SHA-256

```text
initial fixture 7897aefdbe087f3d509ba3f1e43b4b9e4f9dc81a76707478cbe7ffaecfdf4ba3
final combined fixture 95d57100a0f24d353911934922ba7832f6461cd39586e76b8bb92fc1a7f2e132
observation 94452d25de886113dce48a3bf98e79927cf2f5c5df936cfed8ce8b83320d18c7
positive screenshot 3b3086deb76fcb448f1d8e37a67deb80551fa8662399972485e9a8e7a31a8f5d
negative screenshot 511fe4a60334276207ac906d14be196707880f61f180d80f76e36a7535316795
```

## 한계

이는 Chrome Agent Browser 관찰이며 Spinon 동작 근거가 아니다. Chrome build ID, operating system rendering path, layout-frame-only result, WPT pass 또는 platform pixel equality를 뜻하지 않는다. Browser process 종료는 implementation run evidence에 기록한다.
