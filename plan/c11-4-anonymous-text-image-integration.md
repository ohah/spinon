# C11.4 anonymous Grid text와 직접 HTML image 결합 계획

**상위:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 구현을 추가하고 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 다섯 장면을 각각 통과시켰다. 집중 Rust 테스트, 기본/opt-in FFI artifact 경계, 전체 workspace Clippy와 양 플랫폼 fixture 앱 빌드도 통과했다. 구현 실패 관점 20개는 [별도 검토](../spec/internal/evidence/c11-4-anonymous-grid-text-image-runtime-2026-10-21/implementation-review-20.md)에, PR #425의 독립 diff 실패 관점 30개는 [PR 검토](../spec/internal/evidence/c11-4-anonymous-grid-text-image-runtime-2026-10-21/pr-diff-review-30.md)에 기록했다. C11.4와 공식 완료 수는 바꾸지 않는다.

## 목적과 경계

Spinon은 익명 Grid text item과 direct HTML `img` item을 각각 제한 fixture에서 측정한다. direct-leaf text/image multi-span도 함께 있는 별도 경로가 있지만, Grid의 직접 text node가 만든 익명 item과 replaced image를 한 layout revision에서 함께 실행하는 다섯 장면은 없다. 현재 `runtime_layout.rs`는 익명 text fixture 입력과 image fixture 입력이 동시에 감지되면 `unsupported_c114_mixed_intrinsic_fixture`로 중단한다. HostDocument의 anonymous text collector는 direct blockified item에서 run을 flush하고 image intrinsic input은 별도 snapshot으로 제공하므로, 좁은 결합 fixture adapter가 필요하다.

이번 작업은 기존 `RuntimeBlockFlexGridTrackSizingV1`의 **opt-in fixture 경로**에 한정한다. 새로운 공개 API, CSS 지원 선언, 내부 계약 버전 변경은 없다. 검증 결과 기존 layout 경로가 다섯 장면을 이미 처리하면 production layout 변경 없이 fixture adapter·회귀 테스트만 추가한다. 구체적인 실패가 나오면 이 다섯 장면에서 드러난 결함만 수정하고, 알 수 없는 입력은 기존처럼 전체 거부한다.

## Chrome oracle와 규범 기대

Agent Browser에서 Chrome for Testing `155.0.8059.85`, macOS ARM, DPR 1로 고정 fixture를 실행했다. Ahem과 PNG decode 준비 상태, 독립 track 계산, 실제 computed tracks 및 위치는 [사전 비교 기록](../spec/internal/evidence/c11-4-anonymous-grid-text-image-precomparison-2026-10-21/README.md), [JSON](../spec/internal/evidence/c11-4-anonymous-grid-text-image-precomparison-2026-10-21/chrome155-observation.json), [화면](../spec/internal/evidence/c11-4-anonymous-grid-text-image-precomparison-2026-10-21/chrome155-agent-browser.png)에 있다.

[CSS Grid §6](https://www.w3.org/TR/css-grid-1/#grid-items)은 직접 in-flow 자식을 item으로 만들고 text sequence를 익명 block-container item으로 묶으며 whitespace-only sequence를 렌더링하지 않는다. 기본 inline `img`는 grid item이어서 blockify되고 text sequence 사이에서 item 경계를 만든다. [§11.5.1](https://www.w3.org/TR/css-grid-1/#algo-spanning-items)의 span 기여분 분배를 적용한 독립 track 기대값은 다음과 같다.

| 장면 | 직접 자식 | 기대 columns | 기대 자동 배치 |
| --- | --- | --- | --- |
| `anonymous-text-image-text` | `XX`, `<img span 2>`, `XX` | `[20,36,36,20]` | text 0열, image 1–2열, text 3열 |
| `image-before-comment-group` | `<img span 2>`, `XX`, comment, `XX` | `[36,36,40,0]` | image 0–1열, 두 text node 한 item으로 2열 |
| `comment-group-before-image` | `XX`, comment, `XX`, `<img span 2>` | `[40,36,36,0]` | 두 text node 한 item으로 0열, image 1–2열 |
| `whitespace-around-image` | whitespace-only, `<img span 2>`, whitespace + `XX` | `[36,36,20,0]` | 빈 text sequence는 생략, image 0–1열, text 2열 |
| `hidden-child-around-image` | `XX`, `display:none` span, `<img span 2>`, `XX` | `[20,36,36,20]` | hidden child는 item과 run 경계를 만들지 않음 |

각 scene은 definite `220×60px` Grid, `repeat(4,min-content)`, gap 0, Ahem `10px/10px`, 24×16 PNG를 쓴다. image는 `height:48px;width:auto`라 72px inline contribution을 갖는다. 원본 CSS는 Chrome fixture에 고정한다.

## 구현과 검증 관문

1. 계획 PR을 Rebase and merge한 뒤 같은 checkout에서 최신 `main`으로 구현 branch를 만든다. 새 worktree는 만들지 않는다.
2. opt-in C11.4 fixture 전용 결합 입력 경로를 추가한다. text는 Grid root의 직접 text sequence로 입력하고, `<img>`는 기존 image request/decode/intrinsic snapshot으로 제공한다. 두 입력은 같은 style/document revision과 같은 Grid 계산에 들어가야 한다.
3. source-order scan, blockified image 경계, comment로 연결된 text sequence, whitespace-only 생략, `display:none` 통과를 검사한다. HostDocument 런타임 장면에는 comment node를 만들지 않고 인접 Text node 둘로 표현한다. Chrome의 literal comment oracle과 런타임 host-tree 동작을 같은 것으로 오인하지 않는다.
4. Rust 회귀에서는 다섯 장면의 Grid child 순서, text-source node 집합과 stable anchor, image node와 natural size, `LayoutGridItemIntrinsicInput` source/order/revision, Columns/Rows trace, item/root-relative frame을 검증한다. 알 수 없는 topology/scene은 지원하지 않는다.
5. 기존 opt-in GPU fixture 경로에서 각 장면을 실제 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator로 실행한다. 전체 scene 결과, finite track/frame, trace 완전성, layout revision, `drawPresented=true`를 확인한다. Android는 사용자가 고른 시뮬레이터로만 실행하고 이번 범위에 실기기 실행을 넣지 않는다.
6. `cargo fmt --all -- --check`, focused Rust/runtime/FFI tests, workspace all-target Clippy `-D warnings`, Android/iOS fixture app builds를 실행한다. 기본 product artifact에는 fixture symbol·scene 문자열·고정 fixture payload가 없고, opt-in artifact에는 있어야 한다.
7. 테스트가 통과해도 전체 C11.4, 일반 HTML, nested Grid, 모든 image/text 조합, WPT, 실제 이미지 texel paint, OS compositor 표면을 완료로 승격하지 않는다. 코드 변경 후 새 구현 실패 관점 20개를 검토하고, 구현 PR diff도 별도 20개 관점으로 검토한다.

## 완료 판정

- Chrome 155 관찰과 독립 규범 계산의 열 크기가 다섯 장면 모두 일치한다.
- 각 layout revision에서 익명 text와 replaced image intrinsic input이 함께 보존되고 자동 배치·track·frame·trace가 일치한다.
- Android AVD와 iOS Simulator 각각 다섯 장면 모두 layout/frame/trace/draw-presentation 관문을 통과한다.
- 기본/opt-in artifact 경계를 검사하고 계획·구현·PR diff 리뷰를 분리 기록한다.
- 완료 수와 C11.4 checkbox는 그대로 둔다.

## 근거

- [Chrome 사전 비교, screenshot, observation, asset hash](../spec/internal/evidence/c11-4-anonymous-grid-text-image-precomparison-2026-10-21/README.md)
- [구현·AVD/iOS Simulator 결과·정적 라이브러리 경계](../spec/internal/evidence/c11-4-anonymous-grid-text-image-runtime-2026-10-21/README.md)
- [구현 실패 관점 20개](../spec/internal/evidence/c11-4-anonymous-grid-text-image-runtime-2026-10-21/implementation-review-20.md)
- [Chrome 입력 fixture](../tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-image-precomparison.html)
- [계획 실패 관점 20개](./c11-4-anonymous-text-image-plan-review-20.md)
- [계획 PR diff 실패 관점 20개](../spec/internal/evidence/c11-4-anonymous-grid-text-image-precomparison-2026-10-21/plan-pr-diff-review-20.md)
