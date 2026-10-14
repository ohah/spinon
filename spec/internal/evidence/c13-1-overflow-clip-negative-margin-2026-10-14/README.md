# C13.1 음수 clip margin cascade 진단 경계 · 2026-10-14

## 사전 판정

이 변경은 `overflow-clip-margin:-1px`가 Spinon에서 어느 경계에서 거부되는지 고정한다. 2025-10-07 CSS Overflow Working Draft는 clip margin 길이를 0 이상으로 제한한다. 고정 Chrome 155 관찰에서는 `CSS.supports()`와 computed style이 `-1px`를 받아들이므로, 브라우저 수용 사실과 명세상 유효성을 같은 것으로 취급하지 않는다.

통과 조건은 다음과 같다.

1. 관찰 JSON이 Chrome 제품 버전·revision·UA와 원본 fixture 및 선행 observation hash를 고정한다.
2. 음수 inline declaration이 child node에 연결된 CSS cascade diagnostic을 남기고, layout projection이 `CascadeDiagnostic`으로 거부한다.
3. `0px` 대조군은 진단 없이 layout까지 통과한다. 값의 음수성 때문에 실패하는지 확인한다.
4. 테스트 실패를 renderer의 `UnsupportedComputedValue`라고 잘못 분류하지 않는다. 현재 구현의 관찰된 경계는 CSS parser diagnostic을 layout 입력 전에 차단하는 지점이다.
5. 이 host 단위 검증을 Android/iOS surface, pixel, DOM event 또는 WPT 실행으로 확대하지 않는다.

## 고정 관찰

- Browser: Google Chrome for Testing `155.0.8059.40`, arm64, revision `cfaadc5a132d78e1828635aa8405a499f3e14864-refs/branch-heads/8059@{#936}`, agent-browser `0.31.1`.
- User agent: `Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) HeadlessChrome/155.0.0.0 Safari/537.36`.
- [원본 fixture](../../../../tests/fixtures/css/c13/overflow-clip-c13-1.html) SHA-256: `b7acd7e63f79355b28cd480511b9be82e0f2bef478f1aadf685eb3b0bf4ef16a`.
- [선행 전체 Chrome observation](../c13-1-overflow-clip-precomparison-2026-10-14/chrome155-observation.json) SHA-256: `fd6d9e66825bc453609245950635f4fc56054ae9011b14d2f8901d490ecde4b5`.
- [선택 음수 값 observation](chrome155-negative-margin-observation.json) SHA-256: `983c0e434f96c8fbaceb8dda8a963121930992a5b4808437cdec3f834c99f7fd`.
- [CSS Overflow Working Draft의 clip margin 문단](https://www.w3.org/TR/2025/WD-css-overflow-3-20251007/#overflow-clip-margin)은 음수 길이를 유효하지 않은 값으로 정의한다. observation에는 Chrome의 `CSS.supports("overflow-clip-margin", "-1px") == true`와 computed value `-1px`가 담겨 있다.

| 검증 항목 | 결과 |
| --- | --- |
| Chrome의 `-1px` 파싱 수용 | `CSS.supports` true, computed `-1px` |
| Spinon cascade 결과 | child node의 diagnostic에 `overflow-clip-margin:-1px`와 `OtherInvalidValue` 포함 |
| Spinon layout 경계 | `StyleLayoutError::CascadeDiagnostic` 반환, layout 결과 없음 |
| `0px` 대조군 | diagnostic 없음, runtime layout 성공 |

## 실패 재현과 수정

초기 시도는 renderer snapshot 빌더에서 `UnsupportedComputedValue`가 나올 것으로 예상했다. 실제로는 `overflow_runtime_inputs` helper가 cascade diagnostic을 담은 스타일을 layout에 전달하고, layout 경계가 이를 거부한 결과를 helper가 unwrap하면서 테스트가 실패했다. 테스트를 이 실제 경계에 맞춰 style-to-layout의 C13.1 테스트로 옮겼다. 직접 cascade 결과의 node·property·value diagnostic을 확인한 다음 layout 거부를 assert하고, 유효한 `0px`를 negative control로 추가했다. 제품 코드는 변경하지 않았다.

## 실행 결과

- `cargo test --locked -p spinon-style-to-layout negative_clip_margin_is_reported_at_cascade_and_rejected_before_layout -- --nocapture`: 1 passed.
- `cargo test --locked -p spinon-style-to-layout`: 171 passed, 0 failed.
- `cargo test --locked -p spinon-style-to-render c13_1_clip_matrix`: 3 passed, 0 failed.
- 이 증거 slice에서는 ADB, Android AVD 앱 실행, 실기기, iOS Simulator, GPU surface, WPT를 실행하지 않았다. Chrome 관찰은 agent-browser로 얻었다.

이 결과는 invalid CSS 값의 parser diagnostic 및 layout fail-closed 경계만 검증한다. Chrome이 파싱한 음수 값의 전체 paint·event 의미를 명세상 정답으로 선언하지 않으며, C13.1 17-row runtime/surface matrix와 C13 완료 조건은 남아 있다.

## 적대 검토

- [구현 실패 관점 20개](implementation-review-20.md)
- [PR delta 변경 경계 30개](pr-delta-review-30.md)
