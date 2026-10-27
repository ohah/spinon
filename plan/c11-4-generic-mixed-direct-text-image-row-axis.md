# C11.4 · generic direct mixed text/image row-axis intrinsic input

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**선행 구현:** [generic direct mixed text/image column slice](./c11-4-generic-mixed-direct-text-image-intrinsic.md) · [PR #710 runtime 근거](../spec/internal/evidence/c11-4-generic-mixed-text-image-runtime-2026-10-28/README.md) · [fixed-ID row slice](./c11-4-mixed-direct-text-image-intrinsic-row-axis.md)

**사전 비교:** [Chrome 156 관측과 독립 기대값](../spec/internal/evidence/c11-4-generic-mixed-row-axis-precomparison-2026-10-28/README.md) · [현재 Agent Browser 관측](../spec/internal/evidence/c11-4-generic-mixed-row-axis-precomparison-2026-10-28/agent-browser-observation-2026-10-28.json) · [고정 기대값](../spec/internal/evidence/c11-4-generic-mixed-row-axis-precomparison-2026-10-28/expected.json)

**상태:** 구현 PR [#713](https://github.com/ohah/spinon/pull/713) 병합. 2026-10-27T21:00:41Z Rebase and merge, merge SHA `679b8d51bc3b4edc2cbd1c4f5921e3b73a12dabe`. Fixed-ID row slice 이후 새 HostNode ID를 쓰는 generic mixed row adapter를 추가했다. Android/iOS Simulator 각 16/16 scene과 feature-gated runtime/FFI 검증을 마쳤지만, 이 제한 slice는 C11.4 완료를 뜻하지 않는다. 상위 체크와 공식 `72/213`은 유지한다. GitHub check rollup은 비어 있어 CI 통과로 간주하지 않는다. [실행·check 근거](../spec/internal/evidence/c11-4-generic-mixed-row-axis-runtime-2026-10-28/README.md), [구현/runtime 검토 20개](../spec/internal/evidence/c11-4-generic-mixed-row-axis-runtime-2026-10-28/implementation-runtime-review-20.md), [PR diff 검토 30개](../spec/internal/evidence/c11-4-generic-mixed-row-axis-runtime-2026-10-28/pr-diff-review-30.md).

## 목적과 검증 범위

PR #710은 새 HostNode ID를 가진 direct Ahem text + leaf PNG image 입력 네 개를 `min-content`/`max-content` **column** mixed adapter에서 받도록 했다. 해당 구현은 intrinsic row를 fixed rows로 두고 row-axis contribution은 제외한다. 별도 fixed-ID row slice는 text와 natural-size image가 같은 intrinsic row에 기여하는 네 입력을 이미 실행했지만, 그 입력을 `NodeId`와 묶인 adapter로만 판정한다.

이 계획은 같은 row-axis input contract를 보존한 채, novel HostNode root/item IDs를 source, computed style, topology와 placement로 식별하는 별도 opt-in row adapter를 추가한다. 목표는 generic routing과 mixed row contribution/revision/trace 연결이다. min/max row algorithm의 차이를 입증하는 것이 아니다.

## 기준 입력과 독립 기대값

기준 HTML, embedded Ahem/PNG hashes, Chrome observation과 한계는 [2026-10-25 fixed-ID row precomparison](../spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/README.md)에 고정돼 있다. 2026-10-28 재관측에서 Agent Browser CLI의 HeadlessChrome 156, viewport 1280×633, Ahem loaded, PNG decode 완료 및 natural size 24×16px을 다시 확인했다. 관측한 네 입력 모두 columns `[50,24]px`, row `20px`, root `74×20px`, text `(0,0) 50×20px`, image `(50,0) 24×16px`이다. DOM source order만 앞뒤로 바뀐다. 새 관측은 [precomparison evidence](../spec/internal/evidence/c11-4-generic-mixed-row-axis-precomparison-2026-10-28/agent-browser-observation-2026-10-28.json)에 있다.

CSS Grid 2025 CRD §11.5의 non-spanning intrinsic track sizing은 해당 intrinsic function을 가진 track에 span 1인 각 item의 관련 contribution 최댓값을 계산한다. 50px column 안에서 Ahem `XX XXXXX`가 두 줄, line-height 10px로 놓이므로 block-axis contribution은 20px이다. PNG의 natural block size와 contribution은 16px이다. 따라서 CSS Grid 산술의 독립 기대값은 `max(20,16)=20px`이다. CSS Box Sizing 2026 Working Draft §§2.2–2.3은 contribution 용어를 확인하는 참고 자료로 사용한다. 상세 기대값은 Chrome output에서 역산하지 않고 [expected.json](../spec/internal/evidence/c11-4-generic-mixed-row-axis-precomparison-2026-10-28/expected.json)에 고정한다.

`min-content`와 `max-content` row의 이 네 결과는 모두 20px이다. 이 매트릭스만으로 두 algorithm의 차이, 다른 inline size에서의 line wrapping 또는 임의 text/image sizing을 주장할 수 없다. Chrome은 입력 baseline일 뿐 Spinon runtime 결과가 아니다.

## 허용되는 HostDocument 입력

별도 fixture adapter는 C11.4 fixture-test route 안에서 다음 조건만 허용한다.

- Root는 HTML `div` Grid이며 새로운 HostNode root ID를 사용한다. 선택이나 기대값에 root/item ID allowlist를 쓰지 않는다.
- Root width는 `74px`, height는 `auto`, computed `box-sizing`은 oracle처럼 초기값 `content-box`, margin/padding/border/gap은 0, `horizontal-tb`, LTR, `align-items:start`다. Root의 `align-content`, `justify-items`, `justify-content`는 oracle fixture처럼 authored declaration을 추가하지 않아 computed initial value `normal`을 유지한다. Runtime root는 normal flow에 둔다.
- Columns는 `50px 24px`; rows는 단일 `min-content` 또는 단일 `max-content` track이다.
- Direct element child는 정확히 둘이다. 하나는 text node `XX XXXXX` 하나만 가진 HTML `div`, 나머지는 저장소 PNG를 natural size `24×16px`로 decode한 leaf HTML `img`다. 둘 다 row 1에 놓이며 text는 column 1, image는 column 2에 놓인다.
- Text profile은 Ahem 10px/10px, weight 400, normal whitespace/wrapping, horizontal-tb LTR이다. Text와 image item의 computed `box-sizing`도 초기값 `content-box`다. Image의 width/height는 auto이고 aspect-ratio override가 없다. Item box edges는 0이며 computed self-alignment는 `auto`다.
- Forward/reversed input은 두 sizing child의 DOM 순서만 다르다. 동일한 item identity, placement 및 기대 frame을 유지한다.
- HTML oracle의 absolute `.track-probe`는 track rect를 읽는 측정 도구다. Runtime HostDocument에는 이를 넣지 않으며 out-of-flow child를 contribution 또는 scene item으로 세지 않는다.

기존 fixed-ID direct-row route는 그대로 유지하고 먼저 처리한다. Generic row matcher는 exact mixed topology를 알아보되 그 profile에서 style, source, placement, decode, revision이 변형된 입력은 구체적 error로 fail-closed한다. 혼합 형태가 아닌 text-only/image-only root는 기존 경로로 가도록 `Ok(None)`을 반환한다. `RuntimeRootHtmlTextPaintV1` 등 다른 profile도 현재 route의 계약을 유지한다.

## 거부·회귀 관문

- Text 변경·누락, 다른 또는 누락된 image source, decode/natural-size 불일치, nested/non-leaf text/image topology, direct child 추가·중복은 거부한다.
- 두 intrinsic children이 같은 row가 아니거나 columns/rows 함수, track count, column placement, direction, writing mode, root/item edges, font, whitespace, profile이 allowlist와 다르면 거부한다.
- 준비에 사용한 HostDocument snapshot, computed-style revision 또는 intrinsic image snapshot이 stale/mismatched면 거부한다. Invalid mixed input을 text-only/image-only partial result로 낮추지 않는다.
- Novel root/item IDs로 네 positive scene이 준비되어야 한다. Known fixed row IDs는 기존 static adapter로 처리되고, PR #710 generic column scenes는 이 후속 adapter에 잡히지 않아야 한다.

## 구현·실행 순서

1. 이 계획 PR을 `Rebase and merge`한 뒤 같은 checkout에서 구현한다. worktree나 parallel agent lane을 만들지 않는다.
2. 기존 mixed router의 고정 row adapter, PR #710 generic column adapter 및 generic direct-text collector 순서를 확인한다. Fixed-ID row route를 먼저 보존하고, novel row shape를 새 generic row adapter가 검사하도록 최소 연결한다. Common product CSS admission이나 Taffy sizing algorithm을 바꾸지 않는다.
3. Runtime positive tests 네 개는 source order, novel IDs, root/column/row tracks, text/image intrinsic source와 같은 layout revision, Columns→Rows trace, node frames를 고정 기대값과 대조한다. 기존 direct row tests, PR #710 generic column tests, text-only/image-only fallback tests를 보존한다.
4. 서로 다른 scene route IDs로 네 opt-in FFI runtime scenes를 연결한다. Android/iOS strict matchers는 scene ID와 별개로 novel HostNode IDs, root/item kind/order, computed style, intrinsic values/revisions, track traces, root/item frames, draw/present를 검사한다.
5. Android API 37 ARM64 16 KB AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 플랫폼마다 아래 16개 scenes를 같은 source commit으로 실행한다: 기존 fixed direct-column 4, fixed direct-row 4, generic direct-column 4, 신규 generic direct-row 4. 플랫폼별 결과를 분리해 기록한다. 실제 앱이 만든 process만 종료하고 Simulator는 종료하지 않는다.
6. Focused Rust/runtime/FFI 검증, `cargo fmt --all -- --check`, 관련 feature-specific Clippy, default/product artifact negative control과 fixture-test positive control을 확인한다. 실행하지 않은 검증은 통과로 보고하지 않는다. 이 고정 fixture 검증은 일반 CSS admission, 전체 WPT, 실기기, Android hardware GPU, iOS compositor 이후 pixels, glyph/PNG texel equality를 증명하지 않는다.
7. 새 구현/runtime 실패 관점 검토 20개와 별도의 PR diff 실패 관점 30개를 수행한다. 영향받은 관점은 fix 뒤 다시 확인한다. PR 본문에는 Android/iOS 실행 화면을 넣고 Agent Browser에서 각 이미지가 로드되는지 확인한 뒤 이번 세션과 프로세스를 닫는다. Terminal Browser는 사용하지 않는다.
8. 병합 뒤 실제 merge SHA·check rollup·required checks를 확인한다. `spec/STATUS.md`, 상위 계획과 공개 roadmap에 제한 slice와 남은 범위를 기록한다. C11.4 checkbox 및 공식 `72/213`은 유지한다.

## 명시적 제외

이 계획은 generic column adapter 재구현, multiple rows, spanning, anonymous Grid text, auto-placement, arbitrary HTML/CSS, new font/script, vertical writing mode, width-dependent line-wrap variation, automatic minimum, CSS border/overflow paint, general author-document route, product ABI/API, entire WPT, physical device, hardware GPU, iOS compositor-after pixels, glyph/PNG texel equality 또는 C11.4 completion을 다루지 않는다.

## 실행 전 기존 미해결 이력

Fixed-ID row runtime은 2026-10-25 계획에서 한 차례 수정됐다. 초기 author `align-self`/`justify-self` 선언은 app profile에서 거부돼 해당 값을 제거하고 Chrome precomparison을 다시 수집했다. 이번 generic row 계획은 수정된 `auto` self-alignment 관측을 사용하며 명시적 self-alignment author CSS 지원을 추가하지 않는다. 그 이력은 [기존 precomparison](../spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/README.md)에 보존돼 있다.
