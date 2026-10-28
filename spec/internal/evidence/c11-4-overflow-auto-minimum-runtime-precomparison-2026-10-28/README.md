# C11.4 overflow automatic minimum both-axis runtime · 사전 비교

## 범위

기존 [14-case Rust oracle](../c11-4-overflow-scrollable-auto-minimum-precomparison-2026-10-28/README.md)의 geometry와 contribution 기대값을 유지하면서, 대상 축 한쪽에만 overflow를 authored하고 반대 축에는 `visible`을 둔 CSS 입력을 새로 고정했다. 목적은 Columns와 Rows의 automatic-minimum 분기, CSS computed overflow의 축간 변환, 두 control의 layout 차이를 구현 전에 관찰하는 것이다.

이 자료는 Chrome 브라우저 기준이다. Spinon 구현, WPT runner 결과 또는 모바일 runtime 근거가 아니다.

## 독립 규범 기대

- CSS Grid Layout Module Level 1, 2025 CRD §6.6: [automatic minimum](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto).
- CSS Overflow Module Level 3 §3.1: [overflow properties](https://www.w3.org/TR/2025/WD-css-overflow-3-20251007/#overflow-properties). `hidden`, `auto`, `scroll`은 scrollable 값이다. 한 축이 scrollable이면 반대 축의 `visible`은 computed `auto`로 바뀌며 `clip`은 computed `hidden`으로 바뀐다. 이 fixture는 반대 축을 `visible`로 두므로 clip case는 clip을 유지하고, scrollable target의 반대 축은 auto가 된다.
- 기존 수치 기대값과 source attribution은 [expected.json](../c11-4-overflow-scrollable-auto-minimum-precomparison-2026-10-28/expected.json)에 고정되어 있다. 이번 axis-specific computed overflow 기대값은 별도 [expected.json](expected.json)에 독립 규칙으로 추가했다.

## 재현 입력과 관측

- Fixture: [grid-auto-min-scrollable-overflow-c114-axis-precomparison.html](../../../../tests/fixtures/css/c11/grid-auto-min-scrollable-overflow-c114-axis-precomparison.html)
- Replaced resource: [10×10 RGBA PNG](../../../../tests/fixtures/resources/c11-4-overflow-auto-minimum-10x10.png)
- URL: `http://127.0.0.1:4187/tests/fixtures/css/c11/grid-auto-min-scrollable-overflow-c114-axis-precomparison.html`
- Browser: Agent Browser 0.31.1, HeadlessChrome 156.0.8078.49, macOS `MacIntel` user-agent, DPR 1, viewport `1280×633`.
- Ahem `10px` font loaded; seven local PNG references 모두 `decode()` 완료 후 natural size `10×10`을 관찰했다.
- 모든 Grid root에 `align-items:stretch`를 명시했다. 이 고정은 replaced image의 row item frame을 결정하며, 5px row control에서 `normal`은 item 10px, `stretch`는 item 5px이다. 차이는 [alignment contrast 관측](chrome156-align-items-contrast.json)에 기록했다.
- 전체 관측: [Chrome JSON](chrome156-observation.json) · [기대값 대조](comparison.json) · [전체 화면](chrome156-axis-precomparison.png)

14개 scene 모두 computed Columns/Rows, grid/item size, image natural size와 computed `overflow-x/y`가 독립 기대값과 일치했다. **14/14 PASS, 0 mismatch.** root의 절대 viewport 좌표는 두 열짜리 browser gallery 배치에 좌우되므로 대조에서 제외하고, 크기와 track 값만 비교했다.

초기 관측에서 fixture 오류를 구현 전에 찾았다. `img` UA 기본 `overflow-y:clip`이 visible control을 오염시켰고, 직접 만든 SVG data URL 형식과 decode 순서에도 문제가 있었다. 재현 가능한 10×10 RGBA PNG를 고정 fixture resource로 추가하고, 두 overflow 축의 visible baseline, row-clip 방향과 모든 image의 `decode()` 대기를 바로잡았다. 마지막 재실행은 Ahem/image readiness를 확인한 뒤 14개 모두 통과했다.

## 입력 hash

| 파일 | SHA-256 |
| --- | --- |
| fixture HTML | `e156b6030455546d1595971aba467f5b23535faa7bbea9d356a4e99246f99eef` |
| 10×10 PNG resource | `d9d0901f98fe6bbfe3291dad85c5b32a42d71d940f1f51642f2f3a0e4ddfe7b9` |
| independent expected JSON | `fbe3d413fc55b8abfe1e565af60fe05190cf637558a95452b3506c477d8498fa` |
| Chrome observation JSON | `9cc849363cc438479e7e4d302cd04692c25c19a380edbc5021481a714f24069a` |
| alignment contrast JSON | `6058943058eb7c9d9d1df4a84ddf68cfc7203a803ffa219f1cd77e7e42f7109f` |
| comparison JSON | `5902603f5c959779a862000678c1d7c287d4006ebb586b38c8c55d87ad093d53` |
| Chrome screenshot | `42cf1fa96c095d409ff86a434a2088c67f6403552d4273db065c05f9835b138c` |

Screenshot and browser JSON belong to Chrome. They are not Spinon app screenshots, platform surface equality or post-compositor evidence.
