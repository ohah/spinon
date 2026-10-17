# C04.12 후속 · 루트 HTML Canvas 색상 scheme

**상위 계획:** [C04.12 루트 HTML과 문서 환경](c04-root-html-document-environment.md) · [내부 계약 0077](../spec/internal/0077-c04-12-root-html-document.md)
**상태:** 제한 구현 및 Android/iOS Simulator 확인 완료 · C04.12 전체 미완료
**목표:** 루트 HTML의 유효한 color-scheme 선언을 현재 CSS cascade 결과와 결합해 root canvas의 sRGB 배경색으로 전달한다.

## 범위

- `prefers-color-scheme`은 native OS 선호도를 계속 나타낸다. HTML meta나 CSS `color-scheme` 값으로 바꾸지 않는다.
- `meta name="color-scheme"`의 단일 declaration만 지원한다. declaration이 없거나 단일 값이 invalid이면 page-supported scheme을 제공하지 않는다. 중복 선언은 HTML 계약 밖이므로 Canvas 변경을 fail-closed한다.
- root의 Stylo computed `color-scheme`이 `normal`이면 metadata scheme 목록을 사용한다. 명시적인 root CSS scheme은 metadata보다 우선한다. CSS 원문을 자체 토큰화하지 않고 revision이 맞는 computed-style snapshot을 읽는다.
- 제한 값은 `normal`, `light`, `dark`, `light dark`, `dark light`, `only light`, `only dark`다. 그 밖의 computed 값은 canvas 색을 갱신하지 않고 식별 가능한 오류로 끝낸다.
- Chromium `155.0.8059.85`에서 관찰한 sRGB `Canvas` 색을 사용한다: light `#ffffff`, dark `#121212`. 이 바이트 값은 고정 Chrome 구현 기준이며 W3C가 정한 universal palette가 아니다.
- Android 기본 root view와 iOS 기본 root host view에 현재 document/environment/style revision의 Canvas 색을 적용한다. OS scheme 전환 뒤 새 cascade 결과가 확인된 경우에만 배경을 바꾼다.
- 기존 `page_scale`, CSS viewport, DPR과 surface physical size는 서로 분리한다. `prefers-color-scheme` media query는 계속 OS 선호를 따른다.

## oracle와 재현

- 주 oracle은 Chromium `155.0.8059.85`, revision `0e66b94549a8bd85f03fbdf189b752d6e7b63c32-refs/branch-heads/8059@{#1304}`다. 작업 전 계획에서 지정한 Agent Browser만 사용한다.
- 기존 [C04.12 scheme reference](../spec/internal/evidence/c04-root-html-document-environment-2026-10-17/README.md)의 light/dark media emulation과 scheme fixture를 재사용한다.
- 관측값은 root computed `color-scheme`, explicit `Canvas` probe background, `prefers-color-scheme` media 결과, OS emulation을 별도 기록한다. screenshot은 oracle 시각 대조이며 app 화면으로 표시하지 않는다.
- 비교행은 metadata 없음, `dark`, `light dark`, `only dark`, CSS root `dark`, metadata와 root CSS 충돌, `normal`, 단일 invalid metadata, invalid 뒤 valid인 duplicate metadata, OS light/dark다.
- canvas 색상은 각 scheme의 정확한 RGB byte 비교, media preference는 OS 값과 정확 비교, environment/style/document revision은 key equality로 판정한다.

## 구현과 실패 경계

1. Root HTML cascade가 끝나기 전에 색을 반환하지 않는다. 반환한 environment revision 또는 style/document revision이 현재 완료 snapshot과 다르면 stale 색을 거부한다.
2. root CSS의 computed `color-scheme`은 실제 HTML document root node에서 읽는다. body/첫 자식의 scheme을 root scheme으로 사용하지 않는다.
3. CSS root 값이 `normal`이면 metadata supported-scheme 목록으로 돌아가고, 둘 다 없으면 고정 Chrome oracle과 같이 light Canvas를 선택한다.
4. explicit CSS root scheme이 유효하면 metadata보다 우선한다. 다만 `prefers-color-scheme`의 media 결과는 OS 값을 유지한다.
5. 서로 다른 두 개 이상의 `meta name=color-scheme` 요소가 있으면 전체 canvas 갱신을 거부한다. HTML 규격상 중복 metadata는 invalid 입력이다. 고정 Chrome 155는 이 입력에서 첫 candidate 뒤의 scheme을 일관되게 선택하지 않았으므로 그 결과를 product rule로 모사하지 않는다.
6. 지원하지 않는 CSS scheme token, cascade worker 오류·timeout, 누락 root style 또는 revision 불일치가 있으면 이전 색을 유지하고 새 성공으로 보고하지 않는다.
7. native content root만 바꾼다. status/navigation bar, system controls, scrollbars, `CanvasText`, body background propagation, HTML element layout·paint, text, hit testing, WebView behavior는 구현하지 않는다.

## 검증 기준

- Rust tests는 metadata scheme parsing/선택, root CSS precedence, `normal` fallback, invalid value fail-closed, light/dark Canvas bytes와 OS media preference 보존을 검사한다. cascade coordinator의 별도 timeout test는 pending 상태에서 이전 completion을 재사용하지 않음을 확인하고, Canvas resolver는 environment 및 document/style key가 반환 snapshot과 다르면 실패하도록 검사한다.
- Android API 37 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 기본 bundle `index.html`에서 light→dark→light를 실행한다. 화면 capture의 앱 콘텐츠 영역은 각 환경에서 Chrome Canvas 기준값과 exact해야 한다.
- root HTML parser, environment cascade, JavaScript-only session 경계의 기존 unit tests와 Android/iOS 기본 앱의 초기 실행 및 theme 전환을 재검증한다. C04.10 runtime GPU fixture와 resize geometry는 이번 Canvas 배경 slice의 동작 검증으로 간주하지 않는다.
- 이 slice는 root canvas surface background만 다룬다. HTML layout·paint와 기본 문서의 text/DOM rendering을 완료로 표시하지 않으며 C04.12 체크 상태를 바꾸지 않는다.

## 계획 적대 검토

[구현 전 계획 검토 20개](../spec/internal/evidence/c04-12-root-canvas-color-scheme-plan-review-2026-10-17.md)와 [구현 후 실패 관점 검토 20개](../spec/internal/evidence/c04-12-root-canvas-color-scheme-2026-10-17/implementation-review-20.md)를 분리해 기록했다. 최종 변경 검토는 [PR diff 검토 30개](../spec/internal/evidence/c04-12-root-canvas-color-scheme-2026-10-17/pr-delta-review-30.md)에서 다룬다.
