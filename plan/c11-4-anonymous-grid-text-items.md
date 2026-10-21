# C11.4 anonymous Grid text item 제한 계획

**상태:** Chrome 155 사전 관찰과 계획 실패 관점 20개 검토를 마쳤다. 구현은 아직 시작하지 않았다. C11.4 checkbox와 완료 수는 그대로 유지한다.

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [CSS Grid 내부 계약](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md)

## 목적과 범위

현재 C11.4 intrinsic input은 `<div>`의 direct-leaf text와 direct `img`를 보존하지만, Grid container의 직접 텍스트는 `HostDocument` layout 변환에서 거부한다. CSS Grid는 in-flow direct child를 Grid item으로 만들고, 각 direct text sequence를 anonymous block Grid item으로 감싼다. collapsible whitespace만 있는 sequence는 item이 되지 않으며, 익명 item은 placement 속성을 지정할 요소가 없어 자동 배치된다. 익명 box 자체에는 선택자를 적용할 수 없지만 텍스트는 Grid container의 상속 font를 사용한다. 기준은 [CSS Grid Layout Module Level 1 §6](https://www.w3.org/TR/css-grid-1/#grid-items)이다.

이 계획은 기존 `RuntimeBlockFlexGridTrackSizingV1`의 opt-in fixture 경로에서 직접 text sequence를 C11.4 text intrinsic input으로 연결한다. 이번 범위에서 익명 item 하나는 연속된 직접 text node 한 묶음이며, 안정적인 내부 anchor는 그 묶음의 첫 비공백 text `NodeId`다. 묶음 안의 원본 text node ID와 정확한 문자열, Grid owner, child-list 순서를 따로 보존한다. HostDocument 원본 노드를 합치거나 공개 `@spinon/*` API를 추가하지 않는다.

## 고정 Chrome 155 oracle

Agent Browser의 Chrome `155.0.8059.85`, macOS ARM, DPR 1에서 Ahem `10px/10px`가 로드된 것을 확인했다. [고정 입력 fixture](../tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-precomparison.html), [관찰 JSON](../spec/internal/evidence/c11-4-anonymous-grid-text-precomparison-2026-10-21/chrome155-observation.json), [화면](../spec/internal/evidence/c11-4-anonymous-grid-text-precomparison-2026-10-21/chrome155-anonymous-grid-text.png), [hash·판정 기록](../spec/internal/evidence/c11-4-anonymous-grid-text-precomparison-2026-10-21/README.md)을 사용한다. Chrome 결과는 Spinon 결과와 분리된 사전 oracle이다.

판정은 grid-local CSS 좌표로 한다. track 및 box frame은 1/64 CSS px 이내여야 한다. Range rect는 text fragment 관찰값이며 anonymous item box 또는 glyph ink의 대체값으로 쓰지 않는다.

| 장면 | Chrome 결과 | 검증 대상 |
| --- | --- | --- |
| `anonymous-min-content-wrap` | columns `20px 100px`, row `20px`; `XX XX` Range 조각 2개가 각각 `20×10px`, 같은 x에서 y 10px 차이 | non-whitespace direct text가 하나의 min-content Grid item이 되고 두 줄로 배치됨 |
| `anonymous-comment-continuation` | columns `40px 80px`, row `10px`; comment 양쪽 text 조각은 `(0,0)`·`(20,0)`, 각각 `20×10px` | DOM text node 경계·comment가 anonymous item 경계가 아님 |
| `anonymous-runs-around-block` | columns `20px 30px 40px`, row `10px`; 첫 text `(0,0) 20×10`, block `(20,0) 30×10`, 뒤 text `(50,0) 40×10` | direct block child가 text run 두 개를 나누고 source-order auto-placement를 유지함 |
| `whitespace-does-not-create-item` | columns `20px 30px 40px`, row `10px`; block item 두 개는 `(0,0)`과 `(20,0)`에서 `10×10px`; whitespace Range rect 없음 | normal white-space 아래 collapsible whitespace sequence가 auto-placement cell을 소비하지 않음 |

## 구현 및 검증 관문

1. 같은 checkout에서 계획 PR을 먼저 Rebase and merge하고 `origin/main`의 동일 tree에서 구현 branch를 만든다. 별도 worktree를 만들지 않는다.
2. Grid root의 직접 자식 순서를 스캔해 연속 text node를 한 익명 item으로 묶는다. element child는 run을 끊고 기존 item 경로로 처리한다. comment는 HostDocument text-run을 끊지 않는다. 빈 node와 `white-space:normal`에서 collapsible-only인 run은 Grid item 목록과 auto-placement에서 제외한다.
3. 익명 item은 blockified Grid item 기본값과 자동 placement를 사용하고, text의 `ParagraphStyle`은 Grid owner에서 상속한다. `LayoutGridItemIntrinsicInput`에는 source kind `Text`, 정확한 원본 text node 집합, anchor ID, owner, grouped source order, layout/font revision을 기록한다. intrinsic 값은 callback available-space 표본에서 추론하지 않고 Parley min/max-content 측정에서 얻는다.
4. 기존 block item 사이에 direct text run이 놓인 case에서 Grid item/node/frame 순서와 `Columns`/`Rows` contribution trace를 함께 확인한다. 실행 결과에는 `layout=ready`, input revision, intrinsic source·크기, track/frame, trace completeness와 `drawPresented=true`가 있어야 한다.
5. `white-space:pre`, direct inline element/span, nested Grid, nested block flow, anonymous text와 replaced element 조합, `display:contents`, `order`, dense flow, explicit placement/span, writing mode 변경, 다른 font/script는 이번 지원 입력이 아니다. 구현 경계에서 명시 오류로 끝내고 부분 layout 또는 부분 trace를 게시하지 않는 negative control을 둔다.
6. Rust focused tests, C11.4 FFI/layout fixture tests, `cargo fmt`, 관련 Clippy와 fixture artifact boundary를 검사한다. 기본 artifact에는 C11.4 fixture symbol·scene 문자열·고정 입력 payload가 빠지고 opt-in positive control에는 포함되어야 한다.
7. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 네 장면을 비교한다. Android 실기기 serial `R5KYB06SMPE`(model SM-S731N)가 검증 시점에 ADB에 보이면 같은 APK identity를 추가 기록하고 실기기에서도 실행한다. 장치가 연결되지 않으면 AVD 결과를 실기기 결과로 표기하지 않는다.
8. 구현/runtime 변경 뒤 계획 검토와 별도로 새로운 실패 관점 20개를 작성한다. 구현 PR diff에는 프로젝트 규칙의 서로 다른 30개 관점을 추가 검토하고, 발견한 결함은 수정 후 영향 관점을 재실행한다. required check이 있으면 통과한 뒤 Rebase and merge한다.

## 완료 판정과 남은 범위

- 네 fixture case의 Chrome observation과 Ahem asset hash가 고정되어야 한다.
- direct non-whitespace text run, comment로 나뉜 인접 text node, block child가 분리하는 두 run, collapsible-only whitespace의 item 생성·제외 결과가 Chrome 판정과 맞아야 한다.
- 익명 text item의 원본 text-node provenance, stable anchor, owner, source order, font/layout revision, min/max intrinsic width·height가 한 layout revision에서 함께 검증되어야 한다.
- Android AVD와 iOS Simulator의 track·frame·trace·draw/present가 네 case 모두 통과해야 한다. 실기기 근거는 serial 및 설치 APK hash를 확인했을 때만 별도로 기록한다.
- Range fragment 비교는 glyph paint, anonymous box의 전체 frame, Surface pixel 또는 OS compositor 표면을 증명하지 않는다. text glyph paint는 별도 C15/C18 slice에 남긴다.
- 이 opt-in subset은 일반 Grid direct-text 지원, inline span item, nested/non-leaf aggregate, 일반 font/script, `white-space:pre`, WPT, 전체 intrinsic contribution matrix 또는 C11.4 완료를 뜻하지 않는다.

## 근거

- [Chrome 사전 비교와 asset hash](../spec/internal/evidence/c11-4-anonymous-grid-text-precomparison-2026-10-21/README.md)
- [Chrome 입력 fixture](../tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-precomparison.html)
- [계획 적대 검토 20개](./c11-4-anonymous-grid-text-plan-review-20.md)
- [계획 PR diff 적대 검토 30개](../spec/internal/evidence/c11-4-anonymous-grid-text-precomparison-2026-10-21/plan-pr-diff-review-30.md)
