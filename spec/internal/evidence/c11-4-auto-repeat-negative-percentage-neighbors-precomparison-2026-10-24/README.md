# C11.4 음수 percentage `calc()` 이웃 사전 비교

## 목적과 판정 기준

이 자료는 definite content axis에서 auto-repeat 앞·뒤에 놓인 CSS-valid `calc(length - percentage)` 이웃 track의 Chrome 156 결과를 고정한다. Spinon 구현 결과가 아니며 C11.4 완료 판정도 아니다.

- [CSS Grid Layout Module Level 1, 2025 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes)는 track breadth에 `<length-percentage [0,∞]>`를 사용하고 column percentage는 grid container의 inner inline size, row percentage는 inner block size를 기준으로 한다.
- [같은 명세 §7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat)는 definite axis에서 fixed track sizing function과 gap으로 auto-repeat 횟수를 선택하고, auto-fit은 item 배치 후 빈 반복 track과 인접 gutter를 collapse한다.
- [CSS Values and Units Level 4 §10.12](https://www.w3.org/TR/css-values-4/#calc-range)는 math function 안에서는 parse-time range rejection을 하지 않고 top-level 계산 결과를 target range로 clamp하도록 규정한다. 그러므로 이웃의 계산 결과가 음수면 `<length-percentage [0,∞]>`의 하한인 0px을 쓴다.
- 기준 fixture는 [고정 9-scene HTML](../../../../tests/fixtures/css/c11/grid-auto-repeat-negative-percentage-neighbors-c114-precomparison.html)이다. Chrome CSSOM·frame 관측은 [chrome156-observation.json](./chrome156-observation.json), 화면은 [chrome156-full.png](./chrome156-full.png)에 둔다.
- 화면은 1280×646 PNG이며 SHA-256은 `bb8d80977b0911ff0a4d33b6264374cb4cbcd952f5cbe74afe448ef89545ed89`다.
- 독립 계산은 각 scene의 definite inner axis, signed neighbor 결과의 0px clamp, 40px repeated minimum, 4px gap을 입력으로 가장 큰 양의 repeat count를 구한다. CSSOM track 폭 허용 오차는 1/64 CSS px이며 독립 계산을 Chrome 출력에서 역산하지 않는다. 재현 검사는 `python3 spec/internal/evidence/c11-4-auto-repeat-negative-percentage-neighbors-precomparison-2026-10-24/validate-precomparison.py`다.

## 브라우저 관측

Agent Browser 0.31.1에서 macOS ARM64 Chrome 156.0.8078.48, HeadlessChrome UA, 1280×633 CSS viewport, DPR 1을 사용했다. Chrome `CSS.supports()`는 `calc(20px - 5%)`와 `calc(8px - 5%)` auto-repeat declarations를 모두 지원한다고 반환했다.

| Scene | 기준 축·이웃 used size | Chrome track CSSOM | 경계 |
|---|---|---|---|
| `col-prefix-positive` | inline 240px, 8px | `8px 42.3906px × 5` | column prefix |
| `col-suffix-positive` | inline 240px, 8px | `42.3906px × 5 8px` | column suffix |
| `row-prefix-positive` | block 240px, 8px | `8px 42.3906px × 5` | row prefix |
| `row-suffix-positive` | block 240px, 8px | `42.3906px × 5 8px` | row suffix |
| `col-prefix-negative-final` | inline 240px, `8px - 12px → 0px` | `0px 44px × 5` | 음수 최종값 clamp |
| `row-suffix-negative-final` | block 240px, `8px - 12px → 0px` | `44px × 5 0px` | 행 축 음수 최종값 clamp |
| `col-border-box` | outer 136px, content 116px, 14.2px | `14.1875px 46.9062px × 2` | padding·border 제외 basis |
| `col-fit-prefix-item` | inline 240px, 8px | `8px 228px 0px × 4` | auto-fit의 occupied/collapsed tracks |
| `col-both-negative-final` | inline 240px, `-2px - 2.4px → 0px` | `0px 44px × 5` | signed length와 percentage |

Chrome의 소수 CSSOM 값은 1/64px 양자화로 독립 기대값과 다를 수 있다. 9개 scene 모두 40px repeated minimum과 4px gap을 사용한다. `col-fit-prefix-item`은 첫 반복 track 하나만 item이 차지하므로 나머지 네 반복 track의 collapse를 대조한다.

## 결론과 제한

음수 percentage를 포함하는 calc는 Chrome에서 유효한 grid track 선언이다. 최종 used size가 양수면 그 값을 neighbor로 사용하고, 최종값이 음수면 0px로 clamp한 뒤 auto-repeat 횟수를 계산한다. Column·row와 prefix·suffix 위치에서 같은 규칙이 관찰됐다.

이 자료는 단일 Chrome/macOS ARM64 고정 fixture 관측이다. Spinon cascade AST, layout, Android/iOS 앱, PixelCopy/Metal pixels, 실기기·hardware GPU, WPT 적합성을 검증하지 않는다. 구현은 정확히 두 term인 Stylo typed AST의 signed length + signed percentage profile에 한정하며 다른 연산 tree는 계속 fail-closed한다. 공식 C11.4 상태와 `72/213`은 미완료로 유지한다. 계획 단계의 적대 검토는 [계획 검토 20개](../../../../plan/c11-4-auto-repeat-negative-percentage-neighbors-plan-review-20.md)에 둔다.
