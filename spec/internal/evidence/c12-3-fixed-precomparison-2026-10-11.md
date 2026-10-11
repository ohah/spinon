# C12.3 viewport fixed 사전 비교

이 기록은 Spinon의 `position: fixed` 구현 전에 Chrome의 geometry와 containing-block owner를 기준값으로 고정한다. 여기 적힌 관찰은 브라우저 oracle이며 Spinon 구현·모바일 런타임·스크롤 중 동작을 검증했다는 뜻이 아니다.

## 실행 조건

| 항목 | 고정값 |
| --- | --- |
| Chromium | Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`, macOS arm64 |
| Chromium binary SHA-256 | `ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954` |
| CSS 기준 | [CSS Positioned Layout 3 · 2025-10-07 Working Draft](https://www.w3.org/TR/2025/WD-css-position-3-20251007/) |
| viewport | `360×800`, `390×844` CSS px 각각 DPR `1`, `2`, `2.625`, `3` |
| 환경 | `en-US`, UTC, light, forced colors 없음, coarse pointer, hover 없음, LTR, `horizontal-tb` |
| fixture | 22개 case, 식별 DOM node 79개, node·case ID 고유성 및 source parent 고정 |
| 판정 | 각 node `x/y/width/height`와 owner를 독립 비교, 최대 오차 `0.5 CSS px`; 평균으로 실패를 감추지 않음 |
| WPT | commit `9ec154ff43db468923997c08bb08f905ceab62a5`, [3개 대응 경로](https://github.com/web-platform-tests/wpt/tree/9ec154ff43db468923997c08bb08f905ceab62a5/css/css-position); 실행하지 않음 |

실행 명령은 `mise exec -- bun run css:reference:c12-3-fixed-positioning`, 기준 잠금 검사는 `mise exec -- node --test tools/css-reference/c12-3-fixed-positioning.test.mjs`다. 기본 capture는 기존 기준 파일을 덮어쓰지 않으며 재생성에는 명시적으로 `--replace-reference`를 요구한다. HTML은 `overflow:hidden`으로 데스크톱 scrollbar gutter가 viewport client size를 줄여 percentage 기준을 바꾸는 일을 막는다. capture는 `innerWidth/Height`와 `documentElement.clientWidth/Height`가 요청 viewport와 모두 같은지 검사한다. 이것은 scroll 동작을 검증하는 설정이 아니다.

| 고정 입력 파일 | SHA-256 |
| --- | --- |
| `tests/fixtures/css/c12/position-fixed-inventory.json` | `e4fe9296b7658d707dc398f2850d8087117bde1a3b042606018b16d642f8304c` |
| `tests/fixtures/css/c12/position-fixed.html` | `affa32b7b1ff298cf331a06dbf04209fa57e2bcf30ae3a79c88d5f436843e00e` |
| `tools/css-reference/capture-c12-3-fixed-positioning.mjs` | `3cc69945ac98ad0c22f91b221c489cb59f0c8e68339fdd61ae1cab0c9b66a1f1` |
| `tools/css-reference/chromium-session.mjs` | `b010ada41ed75dc2d67ee25abd4266b17930f1de95fc683eebc5cdab6ad2cdc1` |
| `tests/fixtures/css/references/c12-3-position-fixed-v1.json` | `db934028bb39c90ba8c5506a1e25243f5815265e672f6c70c7c9fd7327fe14ac` |

## 고정한 관찰

- 각 viewport에서 DPR 1·2·2.625·3의 CSS frame과 computed style이 모두 동일하다. 실제 화면 pixel이나 GPU paint를 측정한 것은 아니다.
- `360×800`에서 top-left fixed는 `(17, 11, 30, 20)`, right/bottom fixed는 `(319, 770, 33, 21)`, 네 inset percentage box는 `(90, 80, 40, 18)`, `calc()` inset은 `(21, 21, 25, 14)`다.
- 같은 fixture를 `390×844`로 바꾸면 right/bottom은 `(349, 814, 33, 21)`, percentage box는 `(97.5, 84.390625, 40, 18)`, `calc()` inset은 `(22.5, 21.875, 25, 14)`다. 분수 값의 비교 허용치는 기존 계약처럼 node별 `0.5 CSS px`다.
- definite inset의 auto 크기 stretch는 각각 `(20, 120, 315, 530)` 및 `(20, 120, 345, 574)`이며 box-sizing, padding, border를 포함해 outer frame을 기록한다. 실제 테두리 선 paint는 포함하지 않는다.
- fixed ancestor 내부 absolute child는 그 fixed box를 owner로 사용하고, fixed descendant는 조상 fixed box를 owner로 삼지 않고 viewport 기준을 쓴다. `display:none` 아래 fixed node는 box와 owner가 없다.
- fixed-CB 경계에는 author stylesheet와 inline의 transform/effect, 단독 `transform-style:preserve-3d`, 개별 `rotate`·`scale`·`translate`, `perspective`, `filter`, `backdrop-filter`, layout/paint/content containment, `content-visibility:auto`, 관련 `will-change`가 있다. Chrome에서 해당 owner가 쓰인 frame을 관찰한다. 이는 Spinon 지원 허용 목록이 아니라 구현 시 거부할 후보 입력의 oracle이다. `will-change:opacity`는 viewport owner를 유지하는 음성 대조군이다.
- 각 absolute/fixed frame은 Chrome이 계산한 owner 원점과 computed used inset·margin을 사용해 좌표 차이도 남긴다. 전체 observation의 최대 owner-coordinate 차이는 `0.005 CSS px`다.
- `360×800 → 390×844 → 390×844 재적용 → 360×800`에서 동일 크기 재적용과 왕복 결과가 node별로 결정적이다. 이는 browser oracle의 geometry만 비교하며 Spinon의 environment revision 게시 순서를 증명하지 않는다.

## 명시적 미측정

- WPT 파일은 subset 선정 근거만 확인했다. WPT 실행 결과로 보고하지 않는다.
- Android 실기기, iOS Simulator, V8→Stylo→Taffy→WGPU runtime, resize revision 경쟁, stale completion, 실패 원자성, fixed UI의 paint order·stacking·hit-test·scroll 불변, visual viewport/keyboard/safe-area·pinch zoom은 구현 뒤 별도 검증한다.
- root 자체가 fixed인 입력, 두 축의 static-position이 필요한 입력, invalid viewport는 Chrome 성공 fixture로 취급하지 않는다. runtime 거부 경계에서만 검증한다.
- 이 baseline만으로 C12.3 또는 상위 C12를 완료 처리하지 않는다.
