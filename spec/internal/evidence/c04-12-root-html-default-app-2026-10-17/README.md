# C04.12 기본 앱 루트 HTML → V8 document session 실행 근거

**상태:** 기본 Android/iOS 앱의 정적 HTML 전달·DOM 조회 확인 · C04.12 전체 미완료
**실행일:** 2026-10-17 · **실기기:** 사용하지 않음

## 범위

기본 번들에 `index.html`과 별도 `root-app.js`를 포함했다. 일반 Android/iOS 앱 시작 경로에서 HTML을 `spinon_runtime_session_new_with_root_html`로 전달해 초기 HostDocument/V8 `document`를 만들고, `root-app.js`를 같은 session에서 평가한다. HTML 안의 `<script>`는 자동 실행하지 않는다. 기존 `app.js` smoke는 별도 JavaScript-only 호출로 유지한다.

`root-app.js`는 실제 V8에서 다음을 확인한다.

- `document.documentElement`, `head`, `body`와 부모 관계 및 루트 `lang`
- `meta viewport`·`meta color-scheme`의 이름과 content 보존
- root `<style>` text와 body `<main>`의 id/text
- `data-runtime` 속성을 설정한 뒤 같은 session에서 다시 읽기

이 assertion은 HTML document tree가 V8 DOM에 들어왔는지만 판정한다. viewport metadata의 계산, page scale, OS 색상 설정, `prefers-color-scheme`, CSS `color-scheme` 효과, UA Canvas 색 및 HTML paint는 검사하지 않는다.

## 빌드 및 시뮬레이터

- Android: `SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 mise exec -- bun run build:android` · **BUILD SUCCESSFUL**, 기본 fixture flag `c04_12_root_html_fixture_test=0`.
- iOS: `SPINON_V8_DIR=/Users/yoonhb/Documents/workspace/spinon/build/v8-source/v8 mise exec -- bun run build:ios-sim` · **BUILD SUCCEEDED**, iPhone Simulator arm64.
- 공유 V8 checkout은 고정 revision `7b50b62cb18f28617959e8452e2cd18195b38bcf`이며 수정 사항이 없었다. 앱 산출물은 분리 worktree에서 만들었다.
- Android 실행: `spinon_api37_1_compat16k`, Android 17 / API 37 / ARM64, serial `emulator-5554`. `SPINON_ROOT_HTML_BOOT status=pass source=index.html`, `status=0`, `document_revision=2`, `document_nodes=20`을 확인했다. 뒤로 가기로 Activity를 닫은 후 `SPINON_RUNTIME_SESSION_FREE start/done`도 기록했다. [Logcat 원본](https://macstudio.tailed42f2.ts.net/spinon/internal/evidence/c04-12-root-html-default-app-2026-10-17/android-api37-logcat.txt)
- iOS 실행: iPhone 17 Pro / iOS 26.2 Simulator, UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`. 동일한 boot marker와 `status=0`, `document_revision=2`, `document_nodes=20`을 확인했다. [로그](https://macstudio.tailed42f2.ts.net/spinon/internal/evidence/c04-12-root-html-default-app-2026-10-17/ios-iphone17pro-log.txt)

두 build는 V8·Rust·JNI/Objective-C++·Java/Swift·asset copy를 포함한다. 별도 단위·통합 test suite는 이번 slice에서 실행하지 않았다.

`mise exec -- bun run docs:build`도 실행했지만 전체 문서 트리의 여러 기존 dead link 때문에 종료 코드 1을 반환했다. 계약 0077과 이번 실행 근거의 상대 계획/정적 파일 링크를 Tailnet 경로로 정리한 뒤 재실행했을 때 두 변경 문서는 오류 목록에 나오지 않았다. 전체 문서 빌드가 성공했다고 주장하지 않는다.

## Bundle 자산과 화면

- `index.html`: source, Bun bundle, Android APK asset, iOS app bundle의 SHA-256 모두 `73c7d9a557a7f2c15f8caac332a258e288d97f522e980b6aebe0d77ddfde9586`.
- `root-app.js`: source `b87b53883ce838df86412657e1353f8dc90435049eb8ead6ba8fa478ca8a1c32`; Bun bundle, APK asset, iOS app bundle은 모두 `98f5a7c551d3f4d00ea4a1030506d211fa0e63379241a78ca2151c8923965a08`.
- Android debug APK SHA-256: `033b5a77eecd5e0519cef73fe234b389ff6155ed26c5b50374a5ee0c88351466`.
- [Android API 37 화면 캡처](https://macstudio.tailed42f2.ts.net/spinon/internal/evidence/c04-12-root-html-default-app-2026-10-17/android-api37-bootstrap.png) · [iPhone 17 Pro 화면 캡처](https://macstudio.tailed42f2.ts.net/spinon/internal/evidence/c04-12-root-html-default-app-2026-10-17/ios-iphone17pro-bootstrap.png)

두 캡처는 빈 네이티브 host 화면을 보여 준다. V8 document session 성공을 화면의 HTML 출력으로 해석하지 않는다.

## 판정 경계

기본 앱에서 HTML source가 실제 V8 초기 document가 되고 JavaScript가 그 DOM을 조회·변경하는 경로는 두 Simulator에서 확인했다. `meta viewport`·`meta color-scheme`는 현재 element/content 보존만 된다. CSS viewport·page scale, OS color preference·media-query 연결, CSS `color-scheme`의 Canvas·UA 기본색, 그리고 root HTML의 GPU 화면 렌더는 구현되지 않았다. 따라서 이 실행은 기본 앱 HTML loader slice의 근거이며 C04.12 완료, 모바일 웹 viewport 호환 또는 color-scheme 호환을 뜻하지 않는다.
