# C13.1 target frame 2D hit identity 사전 판정 · 2026-10-14

## 구현 전 고정

이 단위는 기존 13개 유효 Chrome row의 target frame 안에서만 수행하는 정수 좌표 2D `elementFromPoint()` 비교다. 구현 변경 전에 다음 oracle·fixture·합격 조건을 고정한다.

- Oracle: agent-browser `0.31.1`, Chrome for Testing `155.0.8059.40`, revision `cfaadc5a132d78e1828635aa8405a499f3e14864-refs/branch-heads/8059@{#936}`, 기존 HeadlessChrome/155 UA.
- Viewport: `1280×1100` CSS px, DPR 1.
- Fixture: `tests/fixtures/css/c13/overflow-clip-c13-1.html`, SHA-256 `b7acd7e63f79355b28cd480511b9be82e0f2bef478f1aadf685eb3b0bf4ef16a`.
- Parent observation: [1D hit identity observation](../c13-1-overflow-clip-hit-identity-2026-10-14/chrome155-element-hit-identity.json), SHA-256 `55682451470f701400ad29f0d86494a0811207fc589ec71b1e62d1a921186db5`.
- Valid rows: `visible`, `clip-x`, `clip-y`, `clip-both`, `clip-margin-padding`, `clip-margin-content`, `clip-margin-border`, `clip-margin-hidden`, `mixed-visible-hidden`, `mixed-clip-auto`, `positioned-contained`, `positioned-escape`, `nested-clip`. Invalid margin은 runtime reject이고 float 세 변형은 별도 fail-closed이므로 성공 layout scan에서 제외한다.

## 합격 조건

1. Browser·fixture metadata가 위 값과 일치하고 target이 있는 유효 row가 정확히 13개다.
2. 각 row의 target frame 경계는 기존 Chrome observation과 일치한다. 정수 표본은 `x=floor(left)..ceil(right)-1`, `y=floor(top)..ceil(bottom)-1`의 모든 조합이다.
3. 각 y row는 x 방향 RLE runs로 저장한다. run이 행의 시작부터 끝까지 겹침이나 누락 없이 덮으며, 모든 y 행도 연속이어야 한다.
4. Chrome identity `target`, `.box`, `.nested-inner`, `.case`/`main`/`body`/`html`은 각각 해당 runtime target, clip parent, nested inner, synthetic root NodeId로 접어 exact 비교한다. 알 수 없는 element identity는 capture와 Rust test 모두 실패한다.
5. Rust는 13개 row의 모든 정수 x/y 조합에서 Chrome과 같은 NodeId를 반환해야 한다. 프레임이나 RLE가 달라지면 hit 비교 전 실패한다.

이 검증은 target frame 내부의 정수 grid만 다룬다. frame 바깥 전체 viewport, 전체 2D draw surface, DOM event dispatch, AVD/iOS surface pixel 또는 WPT 적합성 결과가 아니다. 이 항목은 C13.1 완료를 선언하지 않는다.

이 기준을 기록하기 전에는 capture script, observation 또는 runtime test를 추가하지 않았다.
## 관찰 및 구현 결과

- agent-browser에서 원본 fixture를 열고 Chrome `155.0.8059.40` revision `cfaadc5a...@{#936}`, HeadlessChrome UA, viewport `1280×1100`, DPR 1을 다시 확인했다. 13개 유효행 target frame 전체의 정수 좌표 `(x,y)`를 조회했고 총 187,320점을 RLE로 보존했다.
- 캡처 명령은 `AGENT_BROWSER_ALLOW_FILE_ACCESS=1 agent-browser --session spinon-c13-hit-2d eval --stdin < capture-element-hit-identity-2d.js`다. [관찰 JSON](chrome155-target-frame-hit-identity-2d.json)의 SHA-256은 `248c9d15a56ef332428b0c1e341bac1b1ebe5ce6ea3ac7d01c95a95bd787fe5b`다. 관찰된 topmost identity는 `target`, `box`, `root`, `main`, `body`, `html`이며 wrapper identity는 synthetic root node로 접었다.
- Rust test는 기존 13 valid row call site에서 각 target frame의 모든 x/y 정수 조합을 조회해 exact NodeId를 비교한다. row·run의 연속 범위, target frame과 관찰 metadata도 비교한다. [`validate-capture-integrity.mjs`](validate-capture-integrity.mjs)는 browser/fixture/baseline hash, 13개 ID, frame floor/ceil, 모든 RLE 행·run 연속성, 187,320점 합계와 identity inventory를 별도로 확인했다([실행 결과](capture-integrity-validation.log)).
- 첫 compile에서 `include_str!` 경로가 하위 모듈 기준으로 한 단계 짧아 실패했다. 경로를 수정한 [실패 로그](cargo-hit-identity-2d-include-path-failure.log)를 남겼고, 동일 focused test는 [재실행 로그](cargo-hit-identity-2d-test.log)에서 8/8 통과했다.
- render crate 전체는 [52 passed, 1 ignored](cargo-render-test.log), layout/render Clippy `-D warnings`는 [통과](cargo-clippy.log), rustfmt check는 [통과](cargo-fmt.log)했다.
- 이 변경은 host Rust test와 observation evidence만 추가해 Android/iOS 앱 runtime을 다시 빌드하지 않았다. 기존 API 37 ARM64 AVD/PixelCopy 결과는 다른 fixture/scene에 대한 별도 근거다.

## 남은 범위

이 2D scan은 각 target frame 내부만 덮는다. 모든 viewport 좌표, target frame 밖의 넓은 2D fallback 영역, PixelCopy/Metal surface matrix, DOM event dispatch와 WPT 실행은 미완료다. C13.1 및 C13 완료를 뜻하지 않는다.

관련 Chrome 155 fixture 화면은 [이전 runtime oracle capture](../c13-1-overflow-clip-android-avd-2026-10-14/chrome155-runtime-overflow-clip.png)다. 이 그림은 동작 비교 화면이며 187,320개 identity 표본 자체는 위 JSON과 Rust test log에 보존한다.

## 적대적 검토

[구현 실패 관점 20개](implementation-review-20.md)와 [PR 변경 관점 30개](pr-delta-review-30.md)는 서로 다른 검토 기록이다.

## PR 기록

[PR #159](https://github.com/ohah/spinon/pull/159)은 `main` 기준 ready 상태로 열었다. 본문의 Chrome fixture 화면은 agent-browser에서 실제 표시와 320×120 이미지 로드를 확인했다. PR에는 GitHub status checks가 보고되지 않았고, main branch protection은 required status checks 0개, required approvals 0개, linear history 필수로 확인했다.
