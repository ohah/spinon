# C11.4 · 두 단계 nested Grid intrinsic contribution 계획

**상태:** 계획 PR 병합 뒤 fixed runtime fixture와 Rust 검증을 구현했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 네 scene이 각각 4/4 통과했고, 기본/opt-in static archive 경계를 확인했다. 구현 실패 관점 20개와 별도 PR diff 실패 관점 30개 검토를 마쳤으며 PR [#450](https://github.com/ohah/spinon/pull/450)은 Rebase and merge됐고 SHA는 `80837bbe03542eb55ebc983964c72fd925454d28`다. 병합 당시 check rollup은 비었고 main required status checks는 설정되어 있지 않았다. C11.4 checkbox와 공식 완료 수 `72/213`은 유지한다.

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [일반 nested Grid 제한 계획](./c11-4-nested-grid-intrinsic.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

## 목적과 현재 결손

현재 nested Grid fixture는 root Grid 아래 nested Grid 한 단계의 intrinsic contribution을 고정한다. CSS Grid Level 1은 nested Grid container의 min-content/max-content size를 그 Grid의 track sizing 결과로 정의하므로, 중간 Grid를 추가해도 leaf text의 intrinsic contribution이 각 parent sizing context를 거쳐 전달되어야 한다. 단계 B의 현재 근거는 depth 1에 머물러 있다.

이번 slice는 **root Grid → middle Grid → inner Grid → direct-leaf text**의 고정 두 단계 nesting에서 `min-content`와 `max-content` inline-axis contribution이 전달되는지 확인한다. depth-one control과 나란히 두어 추가된 중간 Grid가 크기나 frame을 바꾸지 않는지 검증한다. 구현에는 이 fixture-test 전용 profile에서 Grid의 `min-content`/`max-content` width를, intrinsic height를 함께 쓰지 않는 경우에 한해 허용하는 projection 경계가 필요하다. 일반 CSS admission, 일반 nested-tree traversal, 임의 nesting depth는 추가하지 않는다.

## 고정 입력과 규범 기대값

각 Grid에는 단일 track 하나만 둔다. `min-content` 장면은 모든 Grid의 width와 column track을 `min-content`로, `max-content` 장면은 모두 `max-content`로 설정한다. 가장 안쪽 direct-leaf item은 동일 Ahem `10px/10px` text `XX XXXXXXX`다. margin, padding, border, gap은 0이며, writing mode와 alignment는 기본 horizontal/LTR 및 start다.

CSS Grid 2025 CRD [§5.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-container-sizing)에서 Grid container의 min-content/max-content size는 해당 constraint에서 계산한 track sizes와 gutters의 합이다. [§11.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-overview)에 따라 각 nested Grid는 자기 track을 계산하고, parent의 child Grid item intrinsic contribution은 그 child Grid의 size에서 온다. 고정된 한 column과 gap 0에서는 각 단계에서 한 track의 기대값이 leaf text contribution과 같다.

Ahem `X` glyph advance는 10px, collapsible space advance는 10px이다. 따라서 `XX XXXXXXX`의 min-content width는 가장 긴 unbreakable word인 `XXXXXXX`의 `7×10 = 70px`, max-content width는 `2×10 + 10 + 7×10 = 100px`이다. `min-content`에서 text는 두 줄 `70×20px`, `max-content`에서는 한 줄 `100×10px`이다. Root·middle·inner Grid는 각 장면에서 모두 같은 one-track width를 받는다.

| Scene | Grid nesting | Independent expected Columns at each Grid | Root / intermediate / leaf text frame sizes |
| --- | --- | --- | --- |
| `nested-depth-one-min-content` | root → inner | `[70px]` | all Grid widths `70px`; text `70×20px` |
| `nested-depth-two-min-content` | root → middle → inner | `[70px]` | all Grid widths `70px`; text `70×20px` |
| `nested-depth-one-max-content` | root → inner | `[100px]` | all Grid widths `100px`; text `100×10px` |
| `nested-depth-two-max-content` | root → middle → inner | `[100px]` | all Grid widths `100px`; text `100×10px` |

이 수치는 Chrome 관측을 기준으로 역산하지 않았다. 규범 정의와 고정 font metrics로 계산한 `spec-derived-expectations.json`이 기준이며, Chrome `HeadlessChrome/156.0.0.0`의 used track·frame 관측은 별도 비교자료다. browser 관측과 독립 계산이 일치한다.

## Chrome oracle와 재현

[HTML fixture](../tests/fixtures/css/c11/grid-track-sizing-c114-nested-grid-depth-two-precomparison.html)는 저장소 Ahem font를 load한 뒤 네 scene의 used columns, 각 Grid frame, leaf text frame과 환경을 기록한다. Agent Browser 0.31.1의 Chrome `156.0.0.0`, macOS ARM, DPR 1, viewport `1280×633`에서 확인했다.

- [Chrome observation](../spec/internal/evidence/c11-4-nested-grid-depth-two-precomparison-2026-10-21/chrome156-observation.json)
- [Chrome screenshot](../spec/internal/evidence/c11-4-nested-grid-depth-two-precomparison-2026-10-21/chrome156-nested-grid-depth-two.png)
- [독립 기대값](../spec/internal/evidence/c11-4-nested-grid-depth-two-precomparison-2026-10-21/spec-derived-expectations.json)
- [입력·관찰·asset SHA-256](../spec/internal/evidence/c11-4-nested-grid-depth-two-precomparison-2026-10-21/SHA256SUMS.txt)

Scene geometry는 root-relative 좌표로 비교한다. Root absolute page offsets는 oracle 재현값일 뿐 판정값이 아니다. Width/height와 used tracks는 0.05 CSS px 이내로 비교하고, fixture topology와 Ahem readiness는 exact해야 한다.

## 구현·검증 절차

1. 계획 PR을 `Rebase and merge`하고 최신 `origin/main`에서 구현 branch를 만든다. 별도 worktree는 만들지 않는다.
2. C11.4 opt-in fixture adapter에 fixed scene ID 네 개를 추가한다. root/middle/inner owner, child order, one-track function, Ahem string, font revision, text node identity를 exact allowlist로 검증하고 미등록 input은 partial layout 없이 거부한다.
3. Rust direct-layout/runtime gates에서 각 Grid의 Columns/Rows track trace, 각 Grid frame, text intrinsic min/max source·revision, leaf text frame과 depth-one control을 확인한다. `MinContent`·`MaxContent` contribution source를 표시하고 root-relative 0px offsets를 비교한다.
4. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 네 scene의 node IDs/order, frame, used track, trace completeness, draw/present를 확인한다. Simulator의 화면은 box/runtime 상태만 증명하며 glyph paint 또는 OS compositor 이후 pixel을 주장하지 않는다.
5. 기본 product artifact에서 fixture symbol·scene strings·Ahem payload가 제외되고 opt-in fixture artifact에는 들어오는지 각각 확인한다. Focused Rust tests, `cargo fmt --all -- --check`, workspace Clippy와 양 플랫폼 fixture app builds를 실행한다.
6. 구현/runtime 코드를 계획 검토와 독립된 실패 관점 20개로 검토하고 수정한 관점은 재검토한다. 구현 PR diff는 앞 두 검토와 독립된 실패 관점 30개로 검토한다. PR에 Android/iOS Simulator 화면을 올리고 Agent Browser에서 실제 로드되는지 확인한 뒤 전용 browser session을 닫는다.
7. Required checks 설정과 현재 결과를 확인하고, 요구 gate가 있으면 통과 후 `Rebase and merge`한다. check가 설정되지 않았거나 비어 있으면 CI 통과로 표현하지 않는다.

## 구현 결과와 남은 검토

고정 scene adapter, runtime Ahem 입력 연결, root부터 inner Grid까지의 frame·Columns/Rows trace assertion을 추가했다. 각 Grid는 직계 자식 Grid 또는 마지막 leaf text item에서 `MinContent`/`MaxContent` one-track contribution을 받아야 한다. layout projection 변경은 C11.4 fixture-test profile에만 적용되며 non-Grid intrinsic width, intrinsic height 조합과 다른 profile은 계속 거부한다. Android/iOS 실행 결과, build/test logs, screenshots, APK hash와 static archive 경계는 [runtime evidence](../spec/internal/evidence/c11-4-nested-grid-depth-two-runtime-2026-10-21/README.md)에 연결했다.

화면 검증은 box frames, track/contribution trace, intrinsic text metrics, draw/present까지만 다룬다. glyph pixel equality와 iOS compositor-after pixels는 증명하지 않는다. [구현/runtime 실패 검토 20개](../spec/internal/evidence/c11-4-nested-grid-depth-two-runtime-2026-10-21/implementation-review-20.md)를 마쳤다. 구현 PR diff 실패 검토 30개를 마쳤고 PR [#450](https://github.com/ohah/spinon/pull/450)의 Rebase and merge 결과 `80837bbe03542eb55ebc983964c72fd925454d28`를 상태 대장과 공개 로드맵에 기록했다. 병합 당시 check rollup은 비었고 required status checks는 설정되어 있지 않았다. 이 focused slice의 검증은 C11.4 전체 완료 판정이 아니다.

## 제외와 완료 판정

- arbitrary HTML/CSS, CSS cascade/parser, anonymous Grid item, inline descendants, nested depth 3+, subgrid와 auto-placement는 포함하지 않는다.
- Percentage/calc, gutters, multiple/spanning Grid items, image/replaced element, vertical writing, author min/max constraints와 dynamic font changes는 포함하지 않는다.
- WPT, Android 실기기, Android hardware GPU, text glyph pixels, iOS compositor-after pixels, 전체 C11.4/CSS Grid 지원은 검증하지 않는다.
- 완료 판정은 네 scene에서 text intrinsic contribution이 각 nested Grid를 통과하고, 각 Grid의 used track·frame이 독립 기대값과 맞으며 Android/iOS Simulator, artifact boundary와 새 code/PR review가 통과하는 것이다. 이 slice는 depth 3 이상과 C11.4 전체를 완료하지 않는다.

## 계획 review와 근거

- [계획 실패 관점 검토 20개](./c11-4-nested-grid-depth-two-plan-review-20.md)
- [계획 PR diff 실패 관점 검토 20개](../spec/internal/evidence/c11-4-nested-grid-depth-two-precomparison-2026-10-21/plan-pr-diff-review-20.md)
- [사전 비교 README](../spec/internal/evidence/c11-4-nested-grid-depth-two-precomparison-2026-10-21/README.md)
- [구현/runtime 실패 관점 검토 20개](../spec/internal/evidence/c11-4-nested-grid-depth-two-runtime-2026-10-21/implementation-review-20.md)
- [구현 PR diff 실패 관점 검토 30개](../spec/internal/evidence/c11-4-nested-grid-depth-two-runtime-2026-10-21/pr-diff-review-30.md)
- PR 병합 근거는 PR 진행 후 연결한다.
