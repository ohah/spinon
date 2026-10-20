# C11.4 · 직접 이미지 다중 span intrinsic 기여 계획

**상태:** Chrome 155 고정 사전 비교, 계획 실패 관점 20개, 구현/runtime 20개 리뷰를 마쳤다. 네 scene은 Android 실기기·AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 검증했다. 이번 제한 구현은 direct HTML `img` 세 개 이하, fixed PNG, explicit height·auto width, `min-content` column과 span 2/3 조합에 한정한다. C11.4 체크와 공식 완료 수는 변경하지 않는다. [PR #411](https://github.com/ohah/spinon/pull/411)에서 검토 중이다.

**상위 원본:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [공식 상태](../spec/STATUS.md#c11-css-grid) · [Grid 내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

## 목적과 범위

기존 image intrinsic fixture adapter는 root Grid의 직접 HTML `img` 하나만 받는다. `spanning-min-content-items`는 한 이미지가 두 `min-content` column을 span하는 경우만 다루며, 여러 replaced image의 최소 기여가 span 길이별로 모이고 DOM 순서가 바뀌어도 결과가 유지되는 경로는 앱에서 확인하지 않았다. C11.4의 직접 텍스트 multi-span slice와 같은 four-scene 구조를 고정 PNG image에 연결한다.

이번 제한 fixture는 단일 Grid root와 직접 자식 HTML `img` 2~3개만 받는다. 각 이미지 source는 저장소의 24×16 PNG이고 `width:auto`, `height:16px|32px|48px`를 사용한다. intrinsic ratio 3:2에서 각각 24×16, 48×32, 72×48 CSS px의 replaced size가 나온다. Grid는 40×60 CSS px border-box, 1px padding, 4개의 `min-content` column, 3개의 20px row, 0 gap, `align-items:start`, `justify-content:start`다. Padding은 이전 border 입력과 같은 38×58 content box와 image offset을 만든다. 첫 span-2 image는 column 1–2, 두 번째는 2–3, 선택적 span-3 image는 1–3에 명시 배치한다.

Scene allowlist:

| Scene | DOM 순서 | 독립 기대 track 크기 |
| --- | --- | --- |
| `image-span2-forward` | span-2 24px, span-2 48px | `[12, 24, 24, 0]` |
| `image-span2-reversed` | 위 두 item의 역순 | `[12, 24, 24, 0]` |
| `image-span2-before-span3` | span-2 24px, span-2 48px, span-3 72px | `[16, 28, 28, 0]` |
| `image-span3-before-span2` | 위 세 item의 역순 | `[16, 28, 28, 0]` |

CSS Grid §11.5.1의 planned-increase 절차가 span 안 모든 track 크기를 계산하고, 한 span 그룹에서 track별 최대 증가를 모은 뒤 다음 큰 span 그룹으로 넘어가는 것을 검사한다. Chrome 155의 고정 관찰은 위 배열과 일치한다. 기여 분배는 규범 알고리즘에서 별도 도출한 표에 적으며, Chrome 관찰값을 규범 기대값으로 대체하지 않는다.

### 입력 경계

- HTML namespace의 direct-leaf `img`만 허용한다. 각 장면의 root, image ID, child count와 DOM 순서는 상수 allowlist로 검사한다.
- source는 `https://spinon.invalid/c14-1-png-basic.png`와 동일한 고정 PNG bytes, intrinsic natural size는 24×16, image revision은 fixture 실행 중 고정한다.
- 지원 style은 root의 지정 크기·padding·track·row와 `align-items:start`를 포함하고, item은 지정 height·auto width·`justify-self:auto`·`align-self:auto` 및 numeric span만 받는다. 둘 다 replaced image의 intrinsic frame을 유지하는 computed/used alignment를 사용한다.
- 다른 image source/크기, percent/calc, image replacement, nested/anonymous item, text/image 혼합, 기타 track 함수, writing mode, WPT, 공개 CSS 지원은 fail-closed 또는 범위 밖이다.

## 사전 기준과 판정

- 고정 oracle: Chrome `155.0.8059.85`, Agent Browser, macOS ARM, DPR 1. 입력 HTML은 [fixture](../tests/fixtures/css/c11/grid-track-sizing-c114-image-multispan-precomparison.html), 관찰·화면·해시는 [사전 비교 근거](../spec/internal/evidence/c11-4-image-multispan-precomparison-2026-10-21/README.md)에서 보존한다.
- 규범 기준: [CSS Grid Layout Level 1 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto)와 [§11.3–§11.5.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items). 지정 높이로 얻는 opposite-axis transferred size suggestion, item별 min-content/minimum contribution, span grouping, planned increase를 서로 구분해 기록한다.
- Chrome 대조는 computed used column, grid border-box, 각 image border-box와 DOM 순서를 비교한다. 각 geometry 값 허용 오차는 `1/64 CSS px`다. 앱 재현 입력은 브라우저 fixture와 ID·style·source order가 동일해야 한다.
- Native PixelCopy에서 root 내부와 root 외부, red image-item background가 root 오른쪽·아래로 넘친 표본을 검사한다. 실행 중 이 runtime 경로에서 CSS border 크기는 layout에 반영됐지만 green border 색은 SurfaceView에 그려지지 않는 것을 확인했다. 이 slice는 border paint 지원을 주장하지 않고 같은 inner geometry를 padding으로 보존한다. PixelCopy는 선택 SurfaceView 좌표에 한정하며 PNG texel 또는 화면 compositor 뒤 색을 증명하지 않는다.

## 구현 및 검증 관문

1. image fixture adapter를 기존 단일-image 행렬에서 분리해 2~3개 direct image 입력을 검증한다. 부분 intrinsic snapshot이나 부분 layout을 반환하지 않도록 한다.
2. fixture FFI route는 네 scene만 allowlist하고 item 배치·스타일·PNG source·node 수를 고정한다. default/product artifact에서 fixture 전용 FFI symbol·scene 문자열이 빠지고 opt-in fixture-test build에는 들어가는지 대조한다.
3. Rust focused tests에서 이미지별 intrinsic source/revision과 contribution, span `2 → 3` stage trace, order reversal 후 track/frame 불변을 확인한다.
4. Chrome 155의 네 scene frame과 trace를 Android API 37 ARM64 AVD, Samsung SM-S731N Android 16/API 36 실기기, iPhone 17 Pro/iOS 26.2 Simulator에서 재현한다. 각 target에서 root/image 전체 frame, track 배열, draw/present를 검사하고 root 경계 밖 overflow의 native surface 색 sample을 보존한다.
5. 실제 PNG texel paint, image async replacement, WPT 및 전체 C11.4/CSS Grid 완료로 확대하지 않는다. 공식 checkbox와 완료 수는 그대로 둔다.
6. 계획은 구현 전 서로 다른 실패 관점 20개로 검토한다. 구현 뒤 코드와 runtime 동작은 새로운 20관점으로 검토하며, 최종 PR diff는 별도 30관점으로 검토한다.

## 남은 범위

이 slice는 고정 24×16 PNG와 직접 HTML `img`의 지정 height·auto width·numeric span 조합만 다룬다. dynamic image resource 교체, 일반 intrinsic image matrix, multiple anonymous/nested image items, text/image 혼합 contribution, complete Grid stage oracle, WPT 및 전체 C11.4는 계속 미완료다.
