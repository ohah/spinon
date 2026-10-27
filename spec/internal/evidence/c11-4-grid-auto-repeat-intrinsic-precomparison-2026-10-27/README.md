# C11.4 `grid-auto-repeat-intrinsic-001.html` 사전 비교

## 고정 입력과 브라우저 관찰

기준은 Web Platform Tests revision `c999c58338ee1d223df5ad62e48be1202ced0d37`의 `css/css-grid/grid-definition/grid-auto-repeat-intrinsic-001.html`이다. WPT source와 5개 direct helper의 SHA-256은 [source-manifest.json](source-manifest.json)에 고정했다. WPT HTML 자체의 SHA-256은 `e6b83286b5f1cf4a46de70cd0dec292eff88d3bd24639570ca812a9ff13eb7c1`이다.

Agent Browser 0.31.1 HeadlessChrome 156.0.0.0에서 viewport 1280×633, DPR 1로 원본을 열었다. WPT testharness는 `Harness status: OK`, 8 tests found, 8 Pass, 0 Fail, 0 Skip을 표시했다. 각 root의 CSS used columns와 child rect를 별도 기록했다. [Chrome 관찰 JSON](chrome-observation.json)에는 root/item/probe의 padding-edge-relative geometry가 있고, [화면](chrome-wpt-results.png)은 overflow와 전체 8/8 보고를 함께 보여 준다.

## 독립 기대값

[expected.json](expected.json)은 CSS Grid Level 1 2025 CRD §5.2 intrinsic container sizing, §7.2.3.2 auto-repeat count, §9.1 absolute item line fallback, §11.5 intrinsic track sizing에서 별도로 유도한 8개 결과를 보관한다. W3C 섹션 텍스트는 [w3c-css-grid-sections.json](w3c-css-grid-sections.json)에 저장했다.

| 조건 | 반복 수 | columns | root border box |
|---|---:|---|---:|
| `minmax(min-content,40px)`, 20px repeated track, max/min-content, auto-fill/fit | 3 | `[20,20,20,40]` | 104×34 |
| `minmax(60px,auto)`, 10px repeated track, max-content, auto-fill/fit | 4 | `[10,10,10,10,60]` | 104×34 |
| `minmax(60px,auto)`, 10px repeated track, min-content, auto-fill/fit | 1 | `[10,60]` | 74×34 |

한 in-flow item은 `grid-column:1 / -1`이라 모든 repeated track을 span한다. 따라서 auto-fit도 이 8개 장면에서는 empty repeated track을 collapse하지 않아 auto-fill과 같은 track 결과를 낸다. Absolute probe는 track contribution에 들어가지 않는다. min-content/auto-tail 두 장면은 root content width 70px 안에 100px item이 있어 border-box right edge보다 28px 더 그려진다. 세 번째 probe는 width 0이므로 geometry에는 남지만 paint 표본으로 세지 않는다.

## 범위

이 자료는 pinned WPT source의 Chrome testharness 결과와 CSS 규범 기대값이다. Spinon 코드, Android/iOS Simulator, Android 실기기, compositor 이후 pixel, 전체 WPT 통과를 검증하지 않았다. 구현 비교는 [집중 계획](../../../../plan/c11-4-grid-auto-repeat-intrinsic-wpt.md)의 8개 fixed scene, layout trace와 surface 표본을 사용한다.
