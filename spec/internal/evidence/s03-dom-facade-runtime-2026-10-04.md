# S03.2 · 제한 DOM façade 실행 근거

**실행일:** 2026-10-04 · **대상:** Android·iOS 시뮬레이터 · **판정:** 두 대상에서 실제 V8 실행 통과

## 고정 입력

`examples/bootstrap/app.js`가 V8에서 S03.2 façade를 초기화하고 Rust `HostDocument`에 요소·Text를 생성·이동·삽입·분리한다. 같은 평가 흐름에서 자식 순서, wrapper 정체성, 부모·형제·텍스트·속성 조회, 오류 이름과 실패 전후 revision을 확인한다. 추가로 Text 노드 빈 자식, `nodeValue`, 필수 인자, nullable `insertBefore(undefined)`, 잘못된 receiver, DOMException 기본 인자·읽기 전용 속성, 앱 루트 Text 거부, `nodeType` 인스턴스 재정의, `Node.prototype.parentNode` getter 재정의를 점검한다.

V8 소스 revision은 `7b50b62cb18f28617959e8452e2cd18195b38bcf`다. 앱은 V8 전역에 JS façade를 설치하고 Rust callback으로 조회·변경한다. 이 결과는 브라우저 DOM 전체 구현이나 JIT·성능 검증이 아니다.

## 플랫폼 실행

| 플랫폼 | 빌드 | 기기·OS | 결과 |
| --- | --- | --- | --- |
| Android | `mise exec -- bun run build:android` · Gradle debug APK, Rust release archive | Android emulator `sdk_gphone64_arm64`, API 36 / Android 16 / `arm64-v8a` | 앱 시작, `is_main_thread=false`, `document_revision=24`, `render_tree_revision=15`, `document_nodes=10` |
| iOS | `mise exec -- bun run build:ios-sim` · Xcode 26.2 (`17C52`) | iPhone 17 Pro simulator, iOS 26.2, arm64 simulator | 앱 시작, `is_main_thread=false`, `document_revision=24`, `render_tree_revision=15`, `document_nodes=10` |

두 로그의 실제 `SPINON_BOOTSTRAP_RESULT`는 `nodes=2 last_node=8 tag=text text=이벤트:7`도 반환했다. 원본 실행 행은 [Android 로그](s03-dom-facade-android-2026-10-04.log), [iOS 로그](s03-dom-facade-ios-2026-10-04.log)에 있다.

## 단위·fixture 검증

- `mise exec -- cargo test --locked --workspace` 통과.
- `mise exec -- cargo clippy --locked --workspace --all-targets -- -D warnings` 통과.
- `mise exec -- cargo fmt --all -- --check` 통과.
- `cd examples/bootstrap && mise exec -- bun test app.test.ts` 통과. 같은 façade 원문과 모의 HostDocument를 실행한다.
- `mise exec -- bun run bundle:bootstrap` 통과.
- `clang++ -std=c++20 -fsyntax-only ... native/v8/src/spinon_v8.cc` 통과; 이후 Android·iOS 앱 빌드도 해당 C++ 경로를 링크했다.
- `git diff --check` 통과.

Chromium `154.0.8037.95`의 이전 비교 실행은 [사전 비교 기록](s03-dom-facade-precomparison-2026-10-04.md)에 있다. 이번에 추가한 nullable 인자·DOMException 생성 기본값은 Web IDL 규칙과 Bun fixture로 확인했다. 이 추가 사례의 재실행 Chrome 프로세스는 제한 시간 내 종료되지 않아 새 Chromium 실행 결과로 주장하지 않는다.

## 경계

실행은 에뮬레이터·시뮬레이터다. Android 실기기, iOS 실기기, JIT 모드별 속도, 장기 실행 메모리, 최대 quota의 순간 메모리, GPU 화면 픽셀, CSS cascade·레이아웃 반영, 접근성, React·Vue·Svelte renderer 통합은 검증하지 않았다. 이 근거는 내부 façade 코드 경로의 동작 확인이며 공개 DOM API 지원 완료를 뜻하지 않는다.
