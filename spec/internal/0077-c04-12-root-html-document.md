# 0077 · C04.12 루트 HTML document 진입점

- **계약 버전:** `0.1.0-draft` · 출시 전 숫자 버전 고정
- **상태:** 제한된 초기 문서 트리 연결 중 · C04.12 전체 미완료
- **상위 계획:** [앱 루트 HTML과 문서 환경 metadata](https://macstudio.tailed42f2.ts.net/spinon/plan/c04-root-html-document-environment.html)
- **Chromium 사전 비교:** [viewport·color-scheme 행렬](./evidence/c04-root-html-document-environment-2026-10-17/README.md)

## 목적과 현재 범위

기존 `RuntimeSession::new()`의 빈 `HostDocument` 동작을 유지하면서, 호스트가 전달한 UTF-8 HTML document를 pinned `html5ever` tree construction으로 파싱해 V8 `document`와 HostDocument cascade 입력으로 사용한다. HTML source 안의 `<script>`는 실행하지 않는다. Android/iOS 기본 앱은 bundle의 `index.html`을 새 root-HTML session에 전달하고 별도 `root-app.js`를 같은 session에서 평가한다.

현재 후보 구현은 `<meta name="viewport">`에서 CSS layout viewport와 분리된 `page_scale`을 계산하고, OS의 light/dark 선호를 CSS `prefers-color-scheme` 입력으로 전달한다. 루트 HTML 전용 author cascade profile에서 viewport width/height 및 기존 pointer·scheme media query allowlist를 계산한다. `<meta name="color-scheme">`의 초기 document scheme과 native 선호도는 별도 typed 값으로 반환한다. `:root { color-scheme: ... }`는 이 profile에서 computed-style snapshot에 보존하지만 meta와 CSS 선언의 우선순위, UA Canvas 색상, native 배경·form control 테마에는 연결하지 않는다. page scale은 snapshot에만 기록하고 renderer transform에는 적용하지 않는다. 이 변경은 CSS cascade까지의 환경 연결이며 HTML layout·paint·화면 표시를 지원하지 않으므로 C04.12는 미완료다.

## 입력과 parser 제한

- Rust 진입점은 `RuntimeSession::new_with_root_html(&str)`이며 HTML5 document mode를 사용한다. C ABI 진입점은 `spinon_runtime_session_new_with_root_html(const char *, size_t, char *, size_t)`다. 기존 생성 함수와 JavaScript-only 동작은 바뀌지 않는다.
- 기본 앱 asset contract는 UTF-8 `index.html`과 별도 `root-app.js`다. Android assets와 iOS app bundle에 둘 다 포함하고, root HTML을 먼저 session document로 파싱한 뒤 JS source를 같은 session에서 평가한다. HTML의 `<script>`를 자동 실행하지 않는다. 기존 `app.js` native smoke는 별도 JavaScript-only 실행으로 남긴다.
- HTML parser는 pinned `html5ever = 0.40.1`이다. DOM adapter는 제품용 `TreeSink`를 구현한다. test-only·비제품 DOM 구현을 선언한 `markup5ever_rcdom`을 사용하지 않는다.
- source는 유효한 UTF-8이어야 하고 최대 크기는 1,048,576 bytes다. HTML5 tokenizer/tree builder의 정상 복구 경로를 적용하며 원본 parse error 횟수는 최대 128까지 계수한다.
- import 가능한 tree는 최대 65,536 node, 최대 depth 256, 전체 16,384 attribute다. source 자체도 1,048,576 bytes로 제한한다. `NoQuirks` document만 받으며, quirks/limited-quirks document는 세션 생성 오류로 거부한다.
- `HostDocument`가 표현하지 못하는 doctype·comment·processing instruction은 보존하지 않는다. element와 text 순서는 보존한다. namespace가 있는 attribute는 현재 batch ABI가 namespace를 표현하지 못하므로 전체 생성 오류로 거부한다. element namespace는 보존한다.
- `<template>`의 별도 contents fragment는 일반 child tree로 연결하지 않는다. `<script>`는 inert element/text로만 문서에 남고 실행하지 않는다. 외부 stylesheet fetch·script 실행·dynamic document mutation과 DOM parser API는 범위 밖이다.
- parse/import 실패는 V8 isolate를 공개하기 전에 session startup 실패로 반환한다. 부분 HostDocument는 해당 worker 초기화와 함께 버린다.

## 환경 API와 cascade 범위

- Rust API는 `RuntimeSession::set_root_html_environment(surface_width_px, surface_height_px, device_scale_factor, media_environment)`이다. 앞의 두 값은 surface physical pixel, device scale은 physical pixel per CSS pixel이다. JavaScript-only session에는 root HTML metadata가 없으므로 `InvalidEnvironment`로 실패한다.
- 내부 C ABI `spinon_runtime_session_set_root_html_environment`도 같은 입력을 받는다. `prefers_color_scheme` 입력은 `0=light`, `1=dark`이고, output struct `SpinonRuntimeRootHtmlEnvironment`에는 CSS viewport width/height, device scale, 별도 page scale, prefers/document scheme(`0=light`, `1=dark`)과 environment revision이 들어간다. 성공은 `0`, 인자·세션 유형·환경 오류는 `-1`, 종료 중은 `-6`, CSS worker 불능은 `-7`이다. 호출자는 세션 해제와 병행하지 않는다. 이는 앱 native bridge용 내부 ABI이며 공개 제품 API가 아니다.
- Root HTML CSS cascade는 `width`, `height`, `min-width`, `max-width`, `min-height`, `max-height`의 px 길이 또는 0 media query와 `prefers-color-scheme`, `pointer`/`any-pointer`, `hover`/`any-hover`의 기존 제한 surface를 받는다. 허용한 query list와 `and`·`not`·`only`·`screen`·`all` 조합만 통과하며 다른 feature, 단위, 속성, malformed condition은 전체 cascade 오류로 거부한다. 이 허용 범위는 `new_with_root_html` profile에만 추가됐고 JavaScript-only/default profile은 viewport dimension query를 계속 거부한다.
- `color-scheme` author property는 Root HTML computed-style output에만 추가된다. layout adapter는 이 값으로 OS scheme을 바꾸거나 Canvas/system color를 그리지 않는다. `meta` 초기 document scheme도 `prefers-color-scheme` 입력과 합치지 않는다.
- Environment revision은 CSS viewport, DPR 또는 media input이 바뀔 때 증가하고 같은 입력을 다시 전달하면 유지된다. cascade snapshot은 비동기 latest-wins worker에서 갱신된다. `page_scale` 변화는 snapshot 수치이며 surface transform·pinch zoom·visual viewport와 연결하지 않는다.

## DOM 관찰 계약

- V8 `document.documentElement`는 문서의 첫 element child를 반환하고, 없으면 `null`을 반환한다. `document.head`와 `document.body`는 첫 root가 HTML `html` element인 경우 그 직접 자식 중 각각 `head`·`body`인 첫 element를 반환하고, 없으면 `null`이다.
- HTML parser가 생성한 `<html>`, `<head>`, `<body>`와 나머지 element/text는 같은 문서 generation과 순서를 갖는 HostDocument 입력이다. HTML `meta`, title 및 기타 비표시 노드는 UA/CSS 처리 대상이며 임의로 visible layout root로 바꾸지 않는다.
- 기본 app `root-app.js`는 루트 `lang`, head/body 부모 관계, viewport와 color-scheme metadata node/content, root style text, body content를 실제 V8에서 읽는다. 이후 `data-runtime` attribute를 설정하고 같은 session에서 다시 읽는다. 이 assertion은 parser가 metadata element를 DOM으로 보존하는지만 확인하며 viewport·OS scheme·computed CSS·Canvas·화면에 적용되는 효과를 확인하지 않는다.
- C04.11에 포함된 연결 `<style>` 수집은 같은 HostDocument tree를 사용한다. `documentElement`의 CSS `:root` 의미, UA root blockification과 body/root layout은 별도 검증 전까지 보장하지 않는다.

## 초기 viewport·color-scheme metadata 환경

- Metadata는 parser가 만든 동일 HTML document의 HTML namespace `<head>` 안 `<meta>` 요소에서 수집한다. 태그와 속성 이름 및 지원 키워드는 ASCII 대소문자를 구분하지 않는다. `content` 값의 원문 attribute는 DOM에 계속 보존한다.
- 호스트 입력은 surface pixel width/height, 양수 유한 `deviceScaleFactor`, OS의 light/dark 선호와 pointer capability다. surface pixel 크기를 device scale로 나눈 값을 metadata 적용 전의 모바일 screen CSS 크기로 사용한다. CSS layout viewport, device scale과 page scale은 별도 값이다.
- 이 slice의 viewport descriptor allowlist는 `width=device-width`, 양수 CSS px `width`, 양수 `initial-scale`이다. `content`는 comma/semicolon 구분을 허용하며, 같은 element 안에 descriptor가 반복되면 마지막 유효 값이 앞선 값을 대체한다. 여러 viewport `<meta>`가 있으면 마지막 요소를 사용한다. 이 duplicate element 규칙은 [고정 Chrome 155 사전 비교 fixture](./evidence/c04-root-html-document-environment-2026-10-17/mobile-duplicate-width.html)의 관찰에 한정한다. 지원하지 않는 descriptor는 무시하고, 유효하지 않은 width/scale 값은 그 descriptor만 생략해 기본 계산을 유지한다.
- viewport metadata가 없으면 모바일 기본 layout width는 `980 CSS px`이고 page scale은 `min(screenWidth / 980, 1)`이다. `width=device-width`는 screen CSS width를 요청한다. 고정 width는 initial scale 적용 후 screen CSS 폭보다 좁은 layout viewport를 만들지 않도록 확장한다. layout height는 `screenHeight / pageScale`이다. 이 계산은 [사전 비교 행렬](./evidence/c04-root-html-document-environment-2026-10-17/README.md)에 기록한 no-meta, device width, `width=320, initial-scale=1`, `initial-scale=0.5` 표본에 대해 Chrome 결과와 정확히 비교한다. metadata가 desktop surface에 미치는 동작과 pinch zoom은 이 slice에서 정의하지 않는다.
- `meta name="color-scheme"`은 `<head>`에서 첫 요소만 선택한다. `content`에서 지원하는 목록은 `light`, `dark`, `only light`, `only dark`다. 목록에 OS 선호가 있으면 그 scheme을 initial document scheme으로 선택하고, `only`는 지정된 scheme을 선택한다. 값이 비어 있거나 지원되지 않으면 initial document scheme은 light다. 이는 UA 초기 scheme 선택 결과이지 OS 선호가 아니다.
- `CssMediaEnvironment.color_scheme`과 `prefers-color-scheme`는 native OS/user preference만 반영한다. 문서 metadata가 `dark`여도 OS가 light면 `(prefers-color-scheme: light)`가 참이고, `document_color_scheme`은 별도 initial scheme 결과다. Root CSS `color-scheme` computed value는 snapshot에 있지만 meta 선언과의 UA 초기값 우선순위, Canvas 색 선택은 미정인 후속 범위다.
- Root HTML session은 viewport metadata를 적용한 CSS viewport와 호스트 `deviceScaleFactor`·media environment로 cascade를 재요청한다. `pageScale`은 environment snapshot에 별도로 남기며 surface transform을 변경하지 않는다. Android UI mode 변화와 iOS trait 변화는 native preference를 다시 전달한다. 현재 Simulator 확인은 이 환경 입력·revision과 cascade result를 관찰하며 root HTML layout·paint는 통과 기준이 아니다.
- 이 단계의 실행 근거는 metadata typed snapshot, `@media` viewport width/height·`prefers-color-scheme` computed result, root CSS `color-scheme` value 및 environment revision을 구분한다. layout viewport가 적용됐다는 사실만으로 page zoom, Canvas color, HTML rendering, screenshot equivalence를 주장하지 않는다.

## 오류·검증 경계

- 잘못된 UTF-8, source/node/depth budget 초과, quirks mode, unsupported namespaced attribute, HostDocument quota 및 parser/tree 변환 오류는 세션 생성 실패다. source 일부만 화면에 게시하지 않는다.
- 이 제한 계약은 앱이 받은 임의 HTML 전체의 WHATWG conformance, 위 allowlist 밖 metadata 의미, page zoom, CSS layout·paint 동등성을 뜻하지 않는다.
- parser 제한의 기존 검증은 tree-construction fixtures, C API validation, JavaScript-only session 경계와 opt-in 실제 V8 document getter를 포함한다. [고정 HTML 직접 전달 Android 캡처](https://macstudio.tailed42f2.ts.net/spinon/internal/evidence/c04-root-html-document-environment-2026-10-17/android-api37-root-html-accessors.png) · [iOS 캡처](https://macstudio.tailed42f2.ts.net/spinon/internal/evidence/c04-root-html-document-environment-2026-10-17/ios-iphone17pro-root-html-accessors.png). 이전 opt-in bundle asset 경로는 [Android 캡처](https://macstudio.tailed42f2.ts.net/spinon/internal/evidence/c04-root-html-document-environment-2026-10-17/android-api37-root-html-asset.png)와 [iOS 캡처](https://macstudio.tailed42f2.ts.net/spinon/internal/evidence/c04-root-html-document-environment-2026-10-17/ios-iphone17pro-root-html-asset.png)에 있다. 기본 앱 경로는 [Android API 37 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 실행 근거](./evidence/c04-12-root-html-default-app-2026-10-17/README.md), 환경 cascade는 [viewport·OS scheme runtime 실행 근거](./evidence/c04-12-root-html-runtime-environment-2026-10-17/README.md)를 참조한다. 두 Simulator 화면은 빈 네이티브 host이며 HTML paint가 아니다. CSS viewport와 제한 media cascade가 계산되더라도 page-scale transform, UA Canvas 색상, HTML layout/paint는 미지원이므로 C04.12 완료 기준에 미달한다.
