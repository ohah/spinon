# 0077 · C04.12 루트 HTML document 진입점

- **계약 버전:** `0.1.0-draft` · 출시 전 숫자 버전 고정
- **상태:** 제한된 초기 문서 트리 연결 중 · C04.12 전체 미완료
- **상위 계획:** [앱 루트 HTML과 문서 환경 metadata](https://macstudio.tailed42f2.ts.net/spinon/plan/c04-root-html-document-environment.html)
- **Chromium 사전 비교:** [viewport·color-scheme 행렬](./evidence/c04-root-html-document-environment-2026-10-17/README.md)

## 목적과 현재 범위

기존 `RuntimeSession::new()`의 빈 `HostDocument` 동작을 유지하면서, 호스트가 전달한 UTF-8 HTML document를 표준 HTML5 tree construction으로 파싱해 V8 `document`와 HostDocument cascade 입력으로 사용한다. HTML source 안의 `<script>`는 실행하지 않는다. Android/iOS 기본 앱은 bundle의 `index.html`을 새 root-HTML session에 전달하고 별도 `root-app.js`를 같은 session에서 평가한다. 이 계약은 HTML document tree와 `documentElement`·`head`·`body` 접근만 정의하며 `meta viewport`와 `meta color-scheme`의 효과는 정의하지 않는다. viewport·color-scheme metadata 적용, OS 설정 수집, page-scale transform, UA Canvas 색상과 HTML paint는 미지원이다.

## 입력과 parser 제한

- Rust 진입점은 `RuntimeSession::new_with_root_html(&str)`이며 HTML5 document mode를 사용한다. C ABI 진입점은 `spinon_runtime_session_new_with_root_html(const char *, size_t, char *, size_t)`다. 기존 생성 함수와 JavaScript-only 동작은 바뀌지 않는다.
- 기본 앱 asset contract는 UTF-8 `index.html`과 별도 `root-app.js`다. Android assets와 iOS app bundle에 둘 다 포함하고, root HTML을 먼저 session document로 파싱한 뒤 JS source를 같은 session에서 평가한다. HTML의 `<script>`를 자동 실행하지 않는다. 기존 `app.js` native smoke는 별도 JavaScript-only 실행으로 남긴다.
- HTML parser는 pinned `html5ever = 0.40.1`이다. DOM adapter는 제품용 `TreeSink`를 구현한다. test-only·비제품 DOM 구현을 선언한 `markup5ever_rcdom`을 사용하지 않는다.
- source는 유효한 UTF-8이어야 하고 최대 크기는 1,048,576 bytes다. HTML5 tokenizer/tree builder의 정상 복구 경로를 적용하며 원본 parse error 횟수는 최대 128까지 계수한다.
- import 가능한 tree는 최대 65,536 node, 최대 depth 256, 전체 16,384 attribute다. source 자체도 1,048,576 bytes로 제한한다. `NoQuirks` document만 받으며, quirks/limited-quirks document는 세션 생성 오류로 거부한다.
- `HostDocument`가 표현하지 못하는 doctype·comment·processing instruction은 보존하지 않는다. element와 text 순서는 보존한다. namespace가 있는 attribute는 현재 batch ABI가 namespace를 표현하지 못하므로 전체 생성 오류로 거부한다. element namespace는 보존한다.
- `<template>`의 별도 contents fragment는 일반 child tree로 연결하지 않는다. `<script>`는 inert element/text로만 문서에 남고 실행하지 않는다. 외부 stylesheet fetch·script 실행·dynamic document mutation과 DOM parser API는 범위 밖이다.
- parse/import 실패는 V8 isolate를 공개하기 전에 session startup 실패로 반환한다. 부분 HostDocument는 해당 worker 초기화와 함께 버린다.

## DOM 관찰 계약

- V8 `document.documentElement`는 문서의 첫 element child를 반환하고, 없으면 `null`을 반환한다. `document.head`와 `document.body`는 첫 root가 HTML `html` element인 경우 그 직접 자식 중 각각 `head`·`body`인 첫 element를 반환하고, 없으면 `null`이다.
- HTML parser가 생성한 `<html>`, `<head>`, `<body>`와 나머지 element/text는 같은 문서 generation과 순서를 갖는 HostDocument 입력이다. HTML `meta`, title 및 기타 비표시 노드는 UA/CSS 처리 대상이며 임의로 visible layout root로 바꾸지 않는다.
- 기본 app `root-app.js`는 루트 `lang`, head/body 부모 관계, viewport와 color-scheme metadata node/content, root style text, body content를 실제 V8에서 읽는다. 이후 `data-runtime` attribute를 설정하고 같은 session에서 다시 읽는다. 이 assertion은 parser가 metadata element를 DOM으로 보존하는지만 확인하며 viewport·OS scheme·computed CSS·Canvas·화면에 적용되는 효과를 확인하지 않는다.
- C04.11에 포함된 연결 `<style>` 수집은 같은 HostDocument tree를 사용한다. `documentElement`의 CSS `:root` 의미, UA root blockification과 body/root layout은 별도 검증 전까지 보장하지 않는다.

## 오류·검증 경계

- 잘못된 UTF-8, source/node/depth budget 초과, quirks mode, unsupported namespaced attribute, HostDocument quota 및 parser/tree 변환 오류는 세션 생성 실패다. source 일부만 화면에 게시하지 않는다.
- 이 제한 계약은 앱이 받은 임의 HTML 전체의 WHATWG conformance, HTML metadata 의미, CSS layout·paint 동등성을 뜻하지 않는다.
- parser 제한의 기존 검증은 tree-construction fixtures, C API validation, JavaScript-only session 경계와 opt-in 실제 V8 document getter를 포함한다. [고정 HTML 직접 전달 Android 캡처](https://macstudio.tailed42f2.ts.net/spinon/internal/evidence/c04-root-html-document-environment-2026-10-17/android-api37-root-html-accessors.png) · [iOS 캡처](https://macstudio.tailed42f2.ts.net/spinon/internal/evidence/c04-root-html-document-environment-2026-10-17/ios-iphone17pro-root-html-accessors.png). 이전 opt-in bundle asset 경로는 [Android 캡처](https://macstudio.tailed42f2.ts.net/spinon/internal/evidence/c04-root-html-document-environment-2026-10-17/android-api37-root-html-asset.png)와 [iOS 캡처](https://macstudio.tailed42f2.ts.net/spinon/internal/evidence/c04-root-html-document-environment-2026-10-17/ios-iphone17pro-root-html-asset.png)에 있다. 기본 앱 경로는 [Android API 37 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 실행 근거](./evidence/c04-12-root-html-default-app-2026-10-17/README.md)를 참조한다. 이 실행은 bundled `index.html`을 V8 document로 초기화하고 별도 JS에서 DOM getter·metadata 보존·attribute mutation을 확인한다. Simulator 화면은 빈 네이티브 host이며 HTML paint가 아니다. viewport/page-scale, OS color preference, `prefers-color-scheme`, meta/CSS color-scheme 효과, UA Canvas와 HTML rendering은 계속 미지원이므로 C04.12 완료 기준에 미달한다.
