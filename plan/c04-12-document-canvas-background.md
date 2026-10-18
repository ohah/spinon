# C04.12 후속 · 루트 HTML 문서 배경

**상위 계획:** [앱 루트 HTML과 문서 환경 metadata](c04-root-html-document-environment.md) · [내부 계약 0077](../spec/internal/0077-c04-12-root-html-document.md)
**상태:** Chrome 사전 비교 완료 · 구현 중 · C04.12 전체 미완료

## 목표와 제한 범위

기본 앱이 HTML5 parser로 만든 같은 문서 tree에서 HTML namespace `html`과 `body`의 computed `background-color`를 읽고, HTML canvas background 선택 순서에 따라 앱 content root의 평탄한 배경색을 고른다. 두 배경이 모두 투명하면 현재 CSS `color-scheme`/metadata로 선택하는 Canvas 기본색(`light=#ffffff`, `dark=#121212`)을 사용한다.

Canvas 색상 입력은 Stylo가 계산한 sRGB color space의 8-bit 정수 채널만 받는다. 알파 0은 투명으로 처리하고, 알파 1은 세 채널이 RGBA8 경계에 정확히 맞을 때만 받는다. 선택되는 배경 후보가 다른 색 공간, system color, 정수 바이트로 표현할 수 없는 색, 부분 alpha 또는 CSS image/gradient이면 새 색을 게시하지 않는다. 불투명 HTML 배경이 선택되면 표현 가능한 body 색은 Canvas 선택에 영향을 주지 않으며, body box 자체의 paint는 구현하지 않는다. 단, root HTML profile의 cascade 검증은 문서 전체에 적용되므로 선택되지 않는 노드의 profile 밖 계산 색이나 선언도 cascade 전체를 실패시킬 수 있다. Stylo 0.22가 system color 키워드를 computed absolute color에서 지우므로, 사용자 지정 속성 값을 포함한 parsed author declaration에서 해당 토큰을 먼저 검출한다. 선택되는 후보의 부분 alpha는 revision이 맞아도 resolver가 `unsupported_computed_background_color`로 실패하고 native host는 이전 색을 유지한다. 나머지 profile 밖 CSS 선언은 cascade를 실패시킨다.

선택 순서는 다음과 같다.

1. HTML root의 computed sRGB `background-color`가 불투명하면 그 값을 canvas로 선택한다.
2. root가 투명이면 연결된 HTML body의 computed sRGB `background-color`를 선택한다.
3. body도 투명이거나 body node가 없으면 computed root `color-scheme` 및 metadata에서 얻은 scheme fallback을 선택한다.
4. 지원하지 않는 계산값이나 색 입력은 조용히 대체하지 않고 실패시킨다.

이것은 Canvas의 단색 선택만 다루며 body box 자체의 레이아웃·paint는 구현하지 않는다. Chrome pixel fixture는 body가 비어 있어 root/body의 canvas 배경 선택과 body box paint를 분리한다. 앱 화면도 native content root 색 확인만 의미한다. 일반 HTML element, text, image, form control, system UI, CSSOM 변경 뒤 자동 native 재게시 및 전체 CSS 색상 적합성은 범위 밖이다. C04.12 완료 체크는 유지한다.

## Oracle와 fixture

- 기준은 Chromium `155.0.8059.85`의 CSS/HTML canvas 동작이다. 페이지 관찰은 Agent Browser로 수행하고 Terminal Browser는 사용하지 않는다.
- 고정 fixture [`canvas-background.html`](../spec/internal/evidence/c04-12-document-canvas-background-2026-10-18/canvas-background.html)은 390×844 CSS px viewport, DPR 3, 빈 body, root `color-scheme: light dark`를 사용한다. browser revision, OS, viewport/DPR, fixture SHA-256, html/body computed color와 1170×2532 screenshot의 pixel sample을 보존한다.
- 행렬: transparent light/dark fallback, root opaque, transparent root + body opaque, 둘 다 opaque, inline/stylesheet/`var()`/media cascade 결과, partial alpha, HSL/HWB, fractional sRGB channel. Chrome CSSOM 값과 final pixel을 모두 기록한다. CSSOM 문자열이 pixel byte와 다르면 pixel 결과를 우선해 지원을 좁힌다.
- Chrome에서 computed `html`·`body`는 별도로 기록한다. Canvas sample은 빈 body 문서 screenshot에서 읽는다. 실제 body box가 canvas 위에 별도로 칠해지는 효과는 이 단계가 표현하지 않는다.
- exact pass: 지원 행의 구현 RGBA8가 Chrome canvas sample과 바이트 단위로 같다. 지원하지 않는 색 입력은 출력 buffer/native 색을 바꾸지 않고 명시된 실패를 반환한다. OS scheme 변화는 투명 fallback에서만 기본색에 반영되며 불투명 문서 배경을 덮지 않는다.
- Rust/C API 기준은 선택 결과, transparent·opaque·unsupported 분류, root/body node 분리, 최신 `DocumentRevision`·`EnvironmentRevision`·`StyleRevision` 일치다. `RuntimeSession::new()` JavaScript-only 동작과 기존 scheme resolver 결과를 유지한다.
- 실제 앱 적용은 Android API 37 ARM64 AVD 및 iPhone 17 Pro / iOS 26.2 Simulator에서 기본 HTML asset을 실행해 content root pixel을 확인한다. 이를 body box paint·전체 HTML rendering·compositor 출력·실기기 결과로 확대 해석하지 않는다.

## 변경 순서

1. 이 계획에 대한 별도 20개 계획 공격 검토를 기록한다.
2. Agent Browser로 Chrome matrix를 실행하고 precomparison evidence 및 fixture hashes를 기록한다.
3. C04.12 root HTML cascade에 typed sRGB background color를 추가하고 정확히 RGBA8로 표현 가능한 색만 허용한다. inline/style-sheet origin, `var()`, media query는 기존 allowlist와 revision을 따른다.
4. computed style snapshot에서 HTML root/body node를 구분하고 canvas 선택기를 순수 함수로 둔다. 결과는 기존 revision key로 보호한다.
5. 내부 명세 0077과 root Canvas 실행 근거를 갱신한다. C04.12 완료 체크는 하지 않는다.
6. 구현 뒤 별도의 20개 실패 관점 검토를 기록한다. Android/iOS Simulator 증거와 JavaScript-only 경계를 확인한다.
7. PR 전에 최종 변경분 전체에 서로 다른 30개 적대 관점 검토를 새로 수행한다.

## 사전 비교 결과와 검토

Chrome oracle 결과와 screenshot hashes는 [precomparison 실행 기록](../spec/internal/evidence/c04-12-document-canvas-background-2026-10-18/README.md)에 있다. 초기, oracle 반영 뒤, 정확한 sRGB byte 경계로 범위를 다시 고정한 검토 등 계획 공격 검토 60개는 [별도 검토 기록](../spec/internal/evidence/c04-12-document-canvas-background-plan-review-20-2026-10-18.md)에 있다. `color(srgb 0.2 0.2 0.9)`의 CSSOM은 `color(srgb 0.2 0.2 0.9)`이나 Chrome pixel은 `(51,51,229)`였다. 따라서 byte 경계에 정확히 맞지 않는 computed sRGB 색은 이번 구현에서 거부하도록 범위를 좁혔다.
