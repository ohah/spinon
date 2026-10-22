# C11.4 · cyclic `minmax(calc())` 겹치는 row span 기여 계획

**상위 계획:** [C11.4 CSS Grid track sizing](./c11-4-grid-track-sizing.md) · [열 축 겹치는 span 선행 계획](./c11-4-cyclic-minmax-calc-overlapping-spans.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** Chrome 156 사전 비교와 독립 계산을 마쳤고, 계획한 고정 runtime slice를 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 확인했다. 구현 PR [#503](https://github.com/ohah/spinon/pull/503)은 2026-10-22 06:31:18 UTC Rebase and merge됐고 SHA `36acdd4f48f23499835530704fa8aef20590a6e5`를 남겼다. 병합 당시 PR status-check rollup은 비어 있었고 main required status checks는 설정되지 않았다. C11.4 checkbox와 공식 완료 수 `77/213`은 그대로 둔다.

## 목적과 경계

병합된 [PR #500](https://github.com/ohah/spinon/pull/500)은 `minmax(calc(10% + 2px), max-content)` 열 세 개에서 서로 겹치는 span-2 text item 둘과 뒤이은 span-3 item의 planned increase를 확인했다. 그 계획은 row axis를 명시적으로 제외했다. 이 후속 계획은 같은 span 관계를 행 축에 배치하되, 너비 10px에서 글자가 세로로 감기는 block contribution을 별도로 oracle과 대조한다. 축 동작이 같다고 전제하지 않는다.

범위는 세 explicit row track, 세 direct text item, 고정 10px column, `row-gap:2px`, Ahem 10px/10px, forward/reverse DOM 순서와 세 root height condition인 정확히 여섯 scene이다. 일반 CSS admission, 임의 track/item 수, 다른 track function이나 calc 식, implicit track, nested/anonymous/replaced item, writing mode 변경, WPT 또는 전체 C11.4 지원은 추가하지 않는다.

## 기준 입력과 독립 기대값

고정 입력, Chrome 관측, 독립 계산과 화면은 [사전 비교 자료](../spec/internal/evidence/c11-4-cyclic-minmax-calc-overlapping-rows-precomparison-2026-10-22/README.md)에 둔다. HTML fixture는 [`grid-cyclic-minmax-calc-overlapping-rows-c114-precomparison.html`](../tests/fixtures/css/c11/grid-cyclic-minmax-calc-overlapping-rows-c114-precomparison.html)이다. 각 grid는 `grid-template-rows:repeat(3, minmax(calc(10% + 2px), max-content))`, 단일 10px column과 2px row gap을 쓴다. 세 item의 row placement는 `1/3`, `2/4`, `1/4`이며 DOM 순서만 forward/reverse로 바꾼다.

저장소 Ahem의 glyph advance와 10px line height, 10px item inline size, `word-break:break-all`에서 각 `a`가 한 줄을 차지한다. 이에 따라 span-2 left/right 및 span-3 item의 block contribution은 각각 70px, 80px, 140px이다. CSS Grid 2025 CRD §7.2.1의 cyclic percentage 처리와 §11.5.1의 span 길이별 그룹 및 per-track planned increase에서 독립 계산한 값은 다음과 같다.

1. 첫 span-2 item은 `70px − 2px gutter = 68px`을 두 행에 분배해 row 1/2에 34px씩 계획한다.
2. 두 번째 span-2 item은 `80px − 2px gutter = 78px`을 row 2/3에 분배해 39px씩 계획한다. 같은 span 그룹의 track별 최대 planned increase는 `[34,39,39]`이다.
3. span-3 item은 `140 − (34+39+39) − 4px gutters = 24px`를 남기고 각 행에 8px씩 더한다. intrinsic max-content upper sizes와 content height는 `[42,47,47]`, 140px이다.
4. cyclic `height:max-content`에서 calc minimum은 `10% × 140 + 2 = 16px`이며 각 upper size보다 작다. definite 140px control도 Rows `[42,47,47]`이어야 한다.
5. definite 100px control에서는 각 calc minimum이 12px이고 gap 뒤 남은 60px을 세 base에 균등 배분해 Rows `[32,32,32]`가 된다. span-3 frame은 100px이고 Ahem text는 140px이므로 content box를 40px 넘는다.

| Scene group | Content height | Final Rows | Root border box | span-2 left / right frame height | span-3 frame / text height |
|---|---:|---:|---:|---:|---:|
| cyclic forward/reverse | 140px | `[42,47,47]` | `14×144px` | `91px / 96px` | `140px / 140px` |
| definite 140px forward/reverse | 140px | `[42,47,47]` | `14×144px` | `91px / 96px` | `140px / 140px` |
| definite 100px forward/reverse | 100px | `[32,32,32]` | `14×104px` | `66px / 66px` | `100px / 140px` |

모든 line rect, root-relative item 위치, DOM 순서와 UA는 [Chrome observation JSON](../spec/internal/evidence/c11-4-cyclic-minmax-calc-overlapping-rows-precomparison-2026-10-22/chrome156-observation.json)에 있다. 기대값은 브라우저 결과를 복사하지 않고 별도 [expectation JSON](../spec/internal/evidence/c11-4-cyclic-minmax-calc-overlapping-rows-precomparison-2026-10-22/spec-derived-expectations.json)에 계산했다. CSS Grid의 규범 근거는 [2025 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes) 및 [§11.3–§11.5.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing)이다.

## 구현·판정 관문

1. 여섯 scene ID와 세 direct text child, 동일 calc-minimum row track 세 개, 고정 10px column, 2px row gap, 숫자로 지정한 row span만 기존 C11.4 opt-in fixture-test 경로에 연결한다. 다른 DOM shape, axis, span, expression 및 알 수 없는 ID는 거부한다.
2. adapter가 CSS source axis를 row axis로 보존하는지 확인한다. Typed input과 trace에는 세 block contribution 70/80/140px, row span group, planned increase, used Rows, root 및 item frame이 보여야 한다. Columns 결과나 border box를 row contribution으로 섞지 않는다.
3. cyclic 및 definite 140px 장면에서 Rows `[42,47,47]`, definite 100px에서는 `[32,32,32]`를 확인한다. forward/reverse의 kind별 frame이 일치해야 한다. 100px control의 frame height와 text line count/scroll height는 별도로 비교한다.
4. 작은 control의 root-relative `(7,7)` 내부 text, `(7,126)` root 바깥 Ahem text, `(7,150)` text 뒤 host clear를 Android PixelCopy와 iOS 제출 전 Metal drawable에서 판정한다. 기록된 DPR과 native viewport origin을 사용해 CSS 좌표를 backing-pixel 좌표로 변환한다. Browser screenshot의 흰 페이지 배경과 app host clear는 별개다. 이 검증은 CSS border stroke 또는 iOS compositor 이후 pixels를 증명하지 않는다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 여섯 장면의 track/frame/draw/present를 각각 확인한다. 실기기는 연결하지 않는다.
6. 기본 release FFI artifact에는 신규 scene ID/payload/fixture entry가 없어야 하며 opt-in fixture-test positive control에는 포함되어야 한다. 필요한 focused Rust 검사, 수정 crate lint/format과 Android/iOS Simulator build를 기록한다.
7. 구현/runtime 검토는 이 계획 리뷰와 겹치지 않는 20개 실패 경로로 수행하고, 정확한 구현 PR diff도 별도 20개 관점으로 검토한다. 문제를 고친 뒤 영향을 받는 장면과 표본을 재실행한다.
8. PR은 입력·관측·기대값, 실제 실행 근거와 제한을 연결한다. C11.4 checkbox와 `77/213`을 올리지 않는다. 필수 check와 merge state를 확인하고 `Rebase and merge` 정책을 따른다.

## 사전 비교 결과

Agent Browser `0.31.1`이 연 `HeadlessChrome/156.0.0.0`에서 Ahem 로드와 여섯 장면을 확인했다. cyclic/definite 140px 장면은 모두 Rows `42px 47px 47px`, definite 100px 장면은 `32px 32px 32px`였고 forward/reverse kind별 frame이 일치했다. 100px 장면의 text line rect 수는 7/8/14이며 frame height는 66/66/100px, scroll height는 70/80/140px다. 이는 Chrome 기준 관측이며 Spinon 구현 성공이나 전체 브라우저 호환성 주장이 아니다.

## 명시적 범위 밖

이 계획은 한 fixed profile의 row axis, 세 explicit track, 두 겹치는 span-2 Ahem text와 한 span-3 Ahem text, 두 DOM 순서 및 세 height 조건만 다룬다. 일반 CSS admission, 다른 writing mode/폰트/폭, 다른 track function, 임의 span group, implicit/nested/anonymous/replaced item, WPT, 실기기, Android hardware GPU, CSS border paint, iOS compositor 이후 pixels와 C11.4 전체 지원은 미검증이다.

## 계획 검토

- [서로 다른 계획 실패 관점 20개](./c11-4-cyclic-minmax-calc-overlapping-rows-plan-review-20.md)
- [Chrome 156 사전 비교와 독립 계산](../spec/internal/evidence/c11-4-cyclic-minmax-calc-overlapping-rows-precomparison-2026-10-22/README.md)

이 계획의 병합은 runtime 구현, 제품 기능 지원 또는 C11.4 완료를 뜻하지 않는다.
