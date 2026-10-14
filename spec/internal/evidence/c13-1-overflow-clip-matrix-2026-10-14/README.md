# C13.1 clip 축·margin 행렬 확장 · 2026-10-14

## 사전 판정 기준

기준 fixture와 Chrome 관찰은 [C13.1 전체 사전 비교](../c13-1-overflow-clip-precomparison-2026-10-14/README.md)에 먼저 고정했다. 이번 테스트는 그 17개 행 가운데 빠져 있던 `clip-y`, `clip-margin-border`, `clip-margin-hidden`의 rectangle runtime geometry와 hit range를 채운다.

1. Chrome computed overflow 값과 대상 frame을 고정 observation과 대조한다. frame 각 값은 축별 `0.5 CSS px` 안이어야 한다.
2. clip paint bound와 hit-test bound를 별도 비교한다. y-only clip의 x는 무제한이며, `overflow-clip-margin` border-box 확장은 양 끝에 적용한다.
3. 각 case의 Chrome 고정 축 좌표에서 대상 frame의 모든 정수 CSS 좌표를 조회하고 `elementFromPoint()`의 target hit range와 일치시킨다. 경계는 Chrome이 기록한 정수 범위를 그대로 사용한다.
4. `overflow:hidden`에서 border-box clip margin 선언은 computed value로 남아도 clip 효과를 만들지 않는다. paint clip은 padding edge, hit clip은 border edge로 각각 확인한다.
5. 선택한 세 행의 통과를 전체 17행, GPU surface pixel, DOM event dispatch 또는 C13.1 완료로 확대하지 않는다.

## 고정 입력

- Chrome for Testing `155.0.8059.40` arm64, revision `cfaadc5a132d78e1828635aa8405a499f3e14864-refs/branch-heads/8059@{#936}`, V8 `15.5.35.20`.
- [원본 fixture](../../../../tests/fixtures/css/c13/overflow-clip-c13-1.html) SHA-256 `b7acd7e63f79355b28cd480511b9be82e0f2bef478f1aadf685eb3b0bf4ef16a`.
- [전체 Chrome observation](../c13-1-overflow-clip-precomparison-2026-10-14/chrome155-observation.json) SHA-256 `fd6d9e66825bc453609245950635f4fc56054ae9011b14d2f8901d490ecde4b5`.
- 이번 세 행은 agent-browser `0.31.1`의 `window.collectC131ClipOracle()`로 다시 읽었다. [선택 observation](chrome155-selected-observation.json) SHA-256 `5df65f92a0b00564529454be38aeb024f0d45e6533c7f605c697a332c07983b0`.

| Chrome case | computed x/y · clip margin | 대상 frame x/y/w/h | Chrome target hit 범위 x / y | runtime clip paint x/y · hit x/y |
| --- | --- | --- | --- | --- |
| `clip-y` | `visible / clip` · `0px` | `430 / 98 / 140 / 120` | `430..569 / 98..179` | `unbounded / 88..176` · `unbounded / 84..180` |
| `clip-margin-border` | `clip / clip` · `border-box 8px` | `34 / 434 / 140 / 120` | `34..147 / 434..523` | `12..148 / 412..524` · `12..148 / 412..524` |
| `clip-margin-hidden` | `hidden / hidden` · `border-box 8px` | `430 / 434 / 140 / 120` | `430..535 / 434..515` | `420..532 / 424..512` · `416..536 / 420..516` |

## 실행 결과와 제한

`cargo test --locked -p spinon-style-to-render c13_1_clip_matrix`는 3/3, 같은 크레이트 전체는 47 passed·0 failed·1 ignored로 통과했다. 크레이트 Clippy(`-D warnings`), `cargo fmt --all -- --check`, `git diff --check`도 통과했다. 각 행에서 runtime target frame을 Chrome observation과 대조하고, x/y 방향으로 전체 정수 query를 순회해 대상 node의 hit 여부를 비교한다. test는 고정 Chrome 값·fixture hash·UA도 확인한다.

이는 Rust `RuntimeRenderSnapshot`의 clip geometry와 내부 hit query 검사다. 이번 변경에서 Android/iOS 앱, WGPU surface sample, screenshot pixel, DOM event dispatch 또는 WPT를 실행하지 않았다. hit-range scan은 대상 node가 맞는지를 확인하며, 대상이 아닌 좌표에서 다른 node의 정확한 fallback ID까지 검사하지 않는다.

전체 C13.1 17-case runtime/surface matrix와 clip 밖 draw/hit/event negative-control breadth는 계속 남아 있다. WPT source triage는 별도 [분류 근거](../c13-1-overflow-clip-wpt-classification-2026-10-14/README.md)에 기록했고 WPT 테스트 자체는 실행하지 않았다. C13.1 및 상위 C13은 미완료다.

## 적대 검토

[구현 실패 관점 20개](implementation-review-20.md)와 [PR 변경 관점 30개](pr-delta-review-30.md)는 이번 세 행의 구현 검증과 변경 경계를 각각 다룬다.
