# C11.4 · `fit-content()` intrinsic limit 열 축 후속 계획

**상위:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [기존 growth-limit row slice](./c11-4-fit-content-limit-boundaries.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태 · 2026-10-30:** 행 축 `fit-content()` limit slice는 PR #787로 병합됐다. 이 후속은 같은 §11.2/§11.5.1 계산의 열 축을 고정한다. Chrome 사전 비교와 독립 기대값은 [evidence](../spec/internal/evidence/c11-4-fit-content-columns-precomparison-2026-10-30/README.md)에 있다. 계획 검토와 계획 PR diff 검토가 끝나기 전 구현은 시작하지 않는다. C11.4 checkbox와 공식 완료 `72/213`은 유지한다.

## 목표

기존 `fit-content-limit-40-wpt`, `fit-content-limit-30-wpt`, `fit-content-limit-200-wpt` 행 축 장면을 columns 방향의 고정 opt-in fixture로 보완한다. Track sizing algorithm이나 일반 CSS admission은 이 계획에서 바꾸지 않는다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 computed track, item/child frame, trace, draw/present와 viewport 안의 fixed surface samples를 확인한다.

## 고정 입력과 독립 기대값

원본 HTML fixture는 [`grid-fit-content-columns-c114-precomparison.html`](../tests/fixtures/css/c11/grid-fit-content-columns-c114-precomparison.html)이다. [Chrome 156 관측](../spec/internal/evidence/c11-4-fit-content-columns-precomparison-2026-10-30/expected.json)은 upstream reference이며 독립 기대값은 CSS Grid 2025 CRD §11.2 공식에서 계산한다.

각 장면은 고정 `grid-template-rows:60px`, 단일 implicit column, `grid-auto-flow:column`, zero gap, 100×10px child, definite Grid width를 사용한다. 30px/40px limit scene의 root width는 60px, 200px limit scene은 300px이다. Main scene은 item `min-width:0`으로 minimum contribution 0px을 고정한다. 따라서 100px max-content contribution과 limit 30/40/200px에서 used columns는 각각 30/40/100px이다. 별도 `min-width:auto` control은 100px automatic minimum이 40px argument보다 큰 floor를 확인한다. Target/root edge 바깥으로 보이는 child는 clipping이 아니라 `overflow:visible`로 남는다.

| scene | limit | item minimum | used column | item / child width |
| --- | ---: | --- | ---: | ---: |
| `column-fit-content-30` | 30px | 0px | 30px | 30px / 100px |
| `column-fit-content-40` | 40px | 0px | 40px | 40px / 100px |
| `column-fit-content-200` | 200px | 0px | 100px | 100px / 100px |
| `column-fit-content-auto-minimum-control` | 40px | auto → 100px | 100px | 100px / 100px |

## 구현 및 검증 경계

1. 계획 PR을 병합한 뒤 같은 checkout에서 최신 `main`을 받아 구현한다. 새 worktree를 만들지 않는다.
2. 기존 exact scene router를 보존하고 네 column scene을 fixture-test feature에만 연결한다. root/item identity, direct child width, `min-width`, `grid-auto-columns`, row definition, source/style/layout revision이 고정 fixture 계약과 다르면 계산 전에 거부한다.
3. Rust direct-layout와 style projection 검사는 min-content minimum 0, max-content 100px, `MaximizeTracks` raw finite growth limit 100px, used base 30/40/100px, auto-minimum floor control, all node frames와 unsupported scene 거부를 확인한다. Raw growth limit과 used base size를 혼동하지 않는다.
4. Android/iOS fixture route의 scene allowlist, strict result matcher, capture labels를 함께 갱신한다. 각 플랫폼에서 네 scene 모두 frame, complete Rows/Columns trace, draw/present revision을 검사한다. PixelCopy와 iOS compositor 제출 전 Metal drawable samples는 root 내부·item overflow·root 바깥이지만 viewport 안인 좌표를 분리한다. 이 결과를 실기기/hardware GPU 또는 iOS compositor-after 근거로 확대하지 않는다.
5. 기본/product FFI archive에서 새 marker·symbol·payload가 없는지, fixture-test positive control에는 있는지 확인한다. Rust formatter, 관련 focused tests와 영향을 받는 Clippy, Android/iOS Simulator builds를 기록한다.
6. 새 구현 실패 관점 20개와 사용자가 요청한 별도 PR diff 관점 20개를 검토한다. 수정 후 영향을 받은 관점을 재실행한다. PR 화면은 Agent Browser에서 이미지 load를 확인하고 이번 브라우저 세션과 프로세스를 닫는다.
7. 병합 뒤 source SHA, 실제 PR merge state/merge SHA와 check rollup을 확인한다. `spec/STATUS.md`, 상위 계획과 공개/Tailnet roadmap에 제한 결과를 동기화하되 C11.4 unchecked와 `72/213`을 유지한다.

## 제외 범위

이 slice는 fit-content 전 grammar, 일반 author CSS 지원, text wrapping, multiple items/spans, nested Grid, all WPT execution, Android physical-device/hardware GPU, iOS compositor-after pixels, 전체 C11.4/C11 완료를 닫지 않는다.
