# C11.4 익명 Grid text와 복수 spanning image 결합 계획

**상태:** Chrome 155 Agent Browser 사전 비교를 고정했고 계획 실패 관점 20개 검토를 마쳤다. 구현과 Android/iOS 시뮬레이터 검증은 시작 전이다. 이 제한 slice는 C11.4 또는 공식 완료 수를 승격하지 않는다.

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [anonymous Grid text](./c11-4-anonymous-grid-text-items.md) · [image multi-span](./c11-4-image-multispan-contributions.md) · [Grid 내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

## 목적과 범위

익명 Grid text와 직접 HTML image의 단일 조합 및 직접 image 두세 개의 multi-span sizing은 각각 별도 제한 fixture에서 확인했다. 두 종류가 한 HostDocument revision에서 함께 수집되고, 같은 Grid의 익명 text item과 모든 replaced image가 하나의 intrinsic-size snapshot 및 track sizing pass에 들어가는 경로는 아직 없다.

이번 fixture는 하나의 definite `220×180px` Grid, 네 개 `min-content` column, 세 개 `60px` row, gap 0을 사용한다. 직접 text sequence는 Ahem `XX` 하나이며 CSS placement를 지정할 수 없는 anonymous block Grid item으로 자동 배치된다. 이미지는 고정 24×16 PNG와 `width:auto`, `height:16px|32px|48px`를 사용한다. 각각의 inline min-content contribution은 aspect ratio 3:2에 따라 24px, 48px, 72px이다. 이미지들은 span-2를 `1 / span 2`와 `2 / span 2`에 두고, 세 번째 이미지는 `1 / span 3`에 둔다. 행은 각 60px로 고정해 item끼리 수직으로 겹치지 않는다.

고정 장면은 네 개다.

| 장면 | 직접 자식 DOM 순서 | 익명 item 배치 | 규범에서 독립 도출한 열 track |
| --- | --- | --- | --- |
| `anonymous-text-two-images-forward` | `XX`, span-2 24px image, span-2 48px image | row 1, column 3 | `[12, 28, 20, 0]` |
| `two-images-anonymous-text-reversed` | span-2 48px image, `XX`, span-2 24px image | row 1, column 3 | `[12, 28, 20, 0]` |
| `anonymous-text-three-images-forward` | `XX`, span-2 24px image, span-2 48px image, span-3 72px image | row 1, column 3 | `[16, 32, 24, 0]` |
| `three-images-anonymous-text-reversed` | span-3 72px, span-2 48px, `XX`, span-2 24px image | row 1, column 3 | `[16, 32, 24, 0]` |

고정 Chrome observation 및 각 root-relative frame은 [사전 비교 근거](../spec/internal/evidence/c11-4-anonymous-text-multimage-precomparison-2026-10-21/README.md)에 따로 보존한다. Chrome 결과는 규범 기대값을 만들거나 수정하는 입력으로 사용하지 않는다.

## 규범 계산과 판정

기준은 [CSS Grid Layout Module Level 1, 26 March 2025 CRD §6, §8.5, §11.3–§11.5.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)이다. §6은 직접 text sequence를 anonymous block-container Grid item으로 만들며 placement 속성이 없어 자동 배치한다고 규정한다. 먼저 명시 배치된 이미지의 점유 칸을 만든 뒤 row-sparse auto-placement가 익명 item을 첫 빈 칸인 row 1, column 3에 둔다. §11.4에서 span-1 `XX`의 20px min-content가 세 번째 track base에 들어간다. 네 track의 min sizing function은 `min-content`이므로 Grid item의 automatic minimum size 조건인 `auto` min track은 없다.

§11.5.1의 track별 planned increase와 growth-limit 분배로 독립 계산한다. 첫 span-2 image의 24px 기여는 첫 두 열에 12px씩 계획한다. 두 번째 image의 48px 기여는 열 2와 열 3을 합친 현재 크기에서 남은 공간을 채우며, 열 3은 span-1 text가 만든 20px growth limit에 도달하므로 나머지는 아직 확장 가능한 열 2로 간다. 따라서 두-image 결과는 `[12, 28, 20, 0]`이다. 세 번째 image의 72px span-3 기여는 첫 세 열의 현재 합 60px에서 12px이 남는다. 앞선 span-2 sizing으로 해당 열들의 growth limit이 정해진 뒤 §11.5.1의 beyond-limit 분배가 12px을 세 열에 4px씩 나누므로 `[16, 32, 24, 0]`이 된다. 역순 DOM은 동일한 planned increase를 내야 한다.

각 Chrome computed track과 root-relative image/text geometry는 독립 rational 기대값에서 `1/64 CSS px` 이내여야 한다. DOM Range rect는 glyph fragment 관찰이며 anonymous item box frame의 대체값이 아니다. 실제 런타임 통과 판정은 text anchor/source-node provenance, image별 node ID·natural size·revision, intrinsic contribution trace, 최종 track, item frame과 root frame을 별도 확인한다.

## 구현 및 검증 관문

1. 기존 opt-in `RuntimeBlockFlexGridTrackSizingV1` fixture 경로에서 네 고정 장면만 허용한다. 하나의 text sequence와 2~3개의 직접 HTML `img`가 동일한 root/document/style/layout revision으로 모여야 한다.
2. 각 이미지 `ImageRequest`를 고정 PNG bytes로 decode하고 모든 completion을 하나의 `ImageResourceSnapshot`으로 구성한다. image ID가 중복되거나 하나라도 빠지거나 decode가 실패하면 부분 snapshot/layout을 게시하지 않는다. 각 image의 natural size와 source revision을 확인한다.
3. Rust 회귀는 anonymous anchor와 source text, child order, image node ID/order, intrinsic snapshot의 node-keyed entry, Columns/Rows stage 및 contribution trace, rational track과 root-relative frame을 검사한다. unknown scene/topology/style/source는 fail closed한다.
4. 같은 opt-in FFI/app fixture에서 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator로 각 네 장면을 실행한다. 사용자가 선택한 범위는 시뮬레이터이므로 Android 실기기 근거는 이 계획의 완료 조건이 아니다. 모든 scene의 `drawPresented`, trace/revision, finite track/frame 수치와 전체 ID 집합을 검사한다.
5. 기본 제품 artifact에서 새 fixture symbol·scene string·payload가 빠지고 opt-in positive control에만 포함되는지 대조한다. 기존 anonymous-text/image 단일 scene, image multi-span, mixed text/image multi-span의 focused regressions와 workspace Clippy/rustfmt를 확인한다.
6. 구현 이후 기존 계획 검토와 다른 20개 구현 실패 관점으로 코드·runtime 결과를 검토한다. PR 변경분은 별도 20개 관점으로 적대 검증한다. PR 본문에 Chrome 및 Android/iOS Simulator 화면을 첨부하고 live required checks가 있으면 모두 통과한 뒤 Rebase and merge한다.

## 범위 밖

이 계획은 고정 Ahem `XX`, 고정 PNG, direct leaf image 2~3개와 네 source order에 한정한다. 다른 text/image, 여러 anonymous text run, comments/whitespace/hidden child 조합, nested/out-of-flow items, dynamic image replacement, 다른 track function/gap/writing mode, implicit image placement, WPT와 일반 CSS Grid 호환성은 주장하지 않는다. PNG texel paint·CSS border paint·OS compositor 이후 pixels는 확인 전까지 검증하지 않는다. C11.4 checkbox와 공식 완료 수는 유지한다.

## 근거

- [Chrome 155 observation, screenshot, asset/input hashes](../spec/internal/evidence/c11-4-anonymous-text-multimage-precomparison-2026-10-21/README.md)
- [비교 입력 fixture](../tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-multimage-precomparison.html)
- [계획 실패 관점 검토 20개](./c11-4-anonymous-text-multispan-images-plan-review-20.md)
- [계획 PR 변경분 실패 관점 검토 20개](../spec/internal/evidence/c11-4-anonymous-text-multimage-precomparison-2026-10-21/plan-pr-diff-review-20.md)
