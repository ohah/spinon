# C13.1 clip hit-test identity와 fallback · 2026-10-14

## 사전 판정 기준

기준은 고정 C13.1 원본 fixture의 Chrome `elementFromPoint()`다. 구현 전에 다음 조건을 정했다.

1. Chrome 155 revision, UA, 1280×1100 CSS px viewport, DPR 1 및 원본 fixture와 선행 observation SHA-256을 기록한다.
2. 기존 Rust matrix의 13개 유효 target 행 각각에 대해 target frame의 모든 정수 x/y 좌표를 기존 orthogonal fixed point에서 조회한다.
3. Chrome topmost element가 `target`이면 같은 target node, `.box`이면 Rust clip parent, `.nested-inner`이면 해당 nested node, `.case`/`main`/`body`/`html`이면 테스트 scene의 root node가 반환되어야 한다. 구간은 빈 점 없이 target frame 전체를 덮어야 한다.
4. Chrome이 target을 반환하지 않는 점을 단순 `target이 아님`으로 합치지 않는다. Rust가 clip parent 또는 root 중 올바른 fallback을 반환하는지 비교한다.
5. 관찰 JSON의 invalid `clip-margin-invalid`는 hit matrix 실행 대상이 아니다. 해당 CSS는 별도 cascade diagnostic으로 layout 전에 reject한다. float 세 행은 target이 없는 Chrome rows이며 C13.1 runtime test가 layout에서 fail-closed한다. 이 둘을 성공 layout으로 꾸미지 않는다.
6. 이 검증은 축별 1D sweep이며 전체 2D surface scan, DOM event dispatch 또는 Android surface 검증으로 확대하지 않는다.

## Oracle 관찰

agent-browser `0.31.1`로 원본 [`overflow-clip-c13-1.html`](../../../../tests/fixtures/css/c13/overflow-clip-c13-1.html)을 열어 [`capture-element-hit-identity.js`](capture-element-hit-identity.js)를 실행했다. Chrome for Testing `155.0.8059.40`, revision `cfaadc5a132d78e1828635aa8405a499f3e14864-refs/branch-heads/8059@{#936}`, HeadlessChrome/155 UA, 1280×1100 viewport와 DPR 1을 기록했다. fixture SHA-256은 `b7acd7e63f79355b28cd480511b9be82e0f2bef478f1aadf685eb3b0bf4ef16a`, 선행 observation SHA-256은 `fd6d9e66825bc453609245950635f4fc56054ae9011b14d2f8901d490ecde4b5`다.

[`chrome155-element-hit-identity.json`](chrome155-element-hit-identity.json)은 target이 있는 14개 Chrome row의 축별 topmost element runs를 보존한다. 유효 runtime profile의 13개 case만 Rust test에서 사용하고, invalid margin 행은 parser diagnostic 경계를 따로 확인한다. 세 float 행은 원본 fixture에 target element가 없어 이 capture inventory에 없다.

정규화된 observation SHA-256은 `55682451470f701400ad29f0d86494a0811207fc589ec71b1e62d1a921186db5`다. capture script의 SHA-256은 `61a07f0e7a9362495f262ad8e7099824a3dac0ea718aa62f6bf8ae3e9dad42ce`다. 실행은 agent-browser에서 원본 HTML을 열고 1280×1100 viewport를 설정한 뒤 `agent-browser eval "$(cat capture-element-hit-identity.js)"`로 했다. 분리한 content-box absolute-child control은 viewport를 200×160으로 설정하고 x=119, 120, 122, 123, 124에서 `elementFromPoint(x,55)`의 ID를 새로 관찰해 [runtime matrix observation](../c13-1-overflow-clip-runtime-matrix-2026-10-14/chrome155-content-box-positioned-hit-observation.json)에 추가했다.

몇 edge fallback은 다음과 같다.

| case | 축/좌표 구간 | Chrome topmost element |
| --- | --- | --- |
| `positioned-contained` | x 504–531 | target |
|  | x 532–535 | clip `.box` |
|  | x 536–545 | case root |
| `nested-clip` | x 324–337 | outer clip `.box` |
|  | y 1016–1019 | outer clip `.box` |

이 결과는 positioned absolute child가 visual clip edge에서 사라진 뒤에도 부모 box가 hit target으로 남는지 검사한다. 기존 hit-range test는 target 반환 여부만 확인했으므로, fallback identity는 별도의 회귀 검사다.

## 실행 결과

- [`cargo-hit-identity-matrix.log`](cargo-hit-identity-matrix.log): `cargo test --locked -p spinon-style-to-render c13_1_clip_matrix`, 8 passed. 기존 13 valid rows의 frame, target range 및 전체 target-frame x/y integer identity runs가 통과했다.
- [`cargo-contentbox-fallback.log`](cargo-contentbox-fallback.log): positioned absolute-child control, 1 passed. x=119 target, x=120/122/123 clip parent, x=124 root fallback이 정확히 일치했다.
- [`cargo-render-test.log`](cargo-render-test.log): render crate 전체 52 passed, 0 failed, 1 ignored.
- [`cargo-clippy.log`](cargo-clippy.log), [`cargo-fmt.log`](cargo-fmt.log), [`git-diff-check.log`](git-diff-check.log): Clippy, rustfmt, diff check 통과.

[구현 실패 관점 20개](implementation-review-20.md)와 [PR 변경 관점 30개](pr-delta-review-30.md)는 이 exact fallback identity delta를 각각 별도로 검토했다.

## 남은 한계

Rust test는 target frame 내부의 축별 정수 표본을 검증한다. 전체 2D 좌표, frame 바깥의 모든 fallback 영역, pointer event dispatch, paint/surface, WPT, invalid/float row의 시뮬레이터 결과는 포함하지 않는다. 이 delta만으로 C13.1 또는 C13 완료를 선언하지 않는다.
