# C04.12 기본 앱 HTML session 구현 실패 관점 20개

검토 대상은 기본 HTML asset bundle, Android JNI/Activity 경로, iOS Objective-C++/AppDelegate 경로와 같은 V8 session에서 실행하는 `root-app.js`다. opt-in fixture 통과를 기본 app 통과로 바꾸어 읽지 않도록 구현 경로와 Simulator 실행 로그를 대조했다.

| # | 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | Android 기본 경로 대신 fixture 전용 분기에서만 HTML을 읽는가 | GPU·C04.12 fixture 조기 반환 뒤 정상 기본 경로에서 `index.html`과 `root-app.js`를 읽는다. 기본 Simulator 로그에 loader marker가 있다. 통과. |
| 2 | root asset 누락 뒤 JavaScript-only 경로가 성공처럼 계속되는가 | Android는 root asset 읽기 실패 시 오류를 기록하고 Activity를 끝낸다. root session을 생략한 정상 시작으로 위장하지 않는다. 통과. |
| 3 | iOS root asset 누락을 성공 로그로 처리하는가 | 둘 중 하나가 빠지면 `SPINON_ROOT_HTML_ASSET_ERROR`로 종료하고 success marker를 남기지 않는다. 통과. |
| 4 | iOS의 기존 `app.js` smoke asset 누락이 root HTML 시작까지 차단하는가 | smoke 누락은 별도로 기록하고 root document loader를 계속 호출한다. 두 경로의 책임을 분리했다. 통과. |
| 5 | HTML 안의 `<script>`가 parser와 기본 app loader 양쪽에서 실행되는가 | parser는 inert script 계약을 유지하고 host는 외부 `root-app.js`만 명시적으로 평가한다. 통과. |
| 6 | 비 ASCII root HTML이 플랫폼 기본 문자셋으로 바뀌는가 | Java에서 UTF-8 bytes, iOS에서 UTF-8 `NSData`를 만든다. 한국어 fixture가 두 Simulator에서 실제 V8 검사를 통과했다. 통과. |
| 7 | Android의 문자열 문자 수 검사로 UTF-8 byte budget을 우회하는가 | JNI byte array 길이를 기준으로 최대 1 MiB를 확인하고 Rust C ABI도 같은 한계를 재확인한다. 통과. |
| 8 | iOS에서 잘못된/과대 source를 FFI로 그대로 넘기는가 | Objective-C++ UTF-8 conversion 실패와 1 MiB 초과를 먼저 거부하며 Rust 경계도 재검증한다. 통과. |
| 9 | 빈 source에서 null pointer와 zero length 조합이 UB를 만드는가 | Rust C ABI는 길이 0에서 빈 slice를 만들고 null pointer dereference를 하지 않는다. 통과. |
| 10 | root/head/body getter가 실제 HTML5 parser가 만든 tree를 읽지 않는가 | `root-app.js`가 getter와 direct-parent 관계를 assertion하고 양쪽 `status=0` report로 통과했다. 통과. |
| 11 | viewport/color-scheme node가 누락되거나 content가 바뀌어도 통과하는가 | 두 meta의 name/content를 각각 비교한다. 현재 범위가 DOM 보존임을 함께 고정했다. 통과. |
| 12 | CSS 선언의 text가 parser 이후 달라져도 assertion이 통과하는가 | root style의 `textContent`를 정확 비교한다. 통과. |
| 13 | body text나 main id가 사라져도 성공하는가 | main element의 id 및 한국어 text를 확인한다. 통과. |
| 14 | JS가 문서를 읽기만 하고 HostDocument에 mutation을 반영하지 않는가 | `data-runtime`을 설정한 뒤 같은 session의 getter로 되읽는다. 성공 report에서 document revision 2를 확인했다. 통과. |
| 15 | JS assertion 예외가 실패인데도 Activity가 세션을 보관하는가 | eval status가 `status=0`으로 시작하지 않으면 native session을 즉시 해제하고 error marker를 남긴다. 통과. |
| 16 | session 생성 성공 뒤 eval 실패 경로에서 worker/session이 누수되는가 | 양 플랫폼의 eval 실패 분기에서 `freeRuntimeSession`/`nativeSessionFree`를 호출한다. 통과. |
| 17 | 종료와 초기화 완료가 경합해 닫힌 Activity에 handle을 게시하는가 | Android의 volatile closing flag와 lock, iOS의 lock 아래 termination flag 확인 뒤 닫힌 경우 handle을 저장하지 않고 해제한다. 통과. |
| 18 | Android Activity 종료에서 root session이 해제되지 않거나 두 번 해제되는가 | handle을 lock 아래 0으로 교환하고 기존 runtime-control queue에서 해제한다. Simulator에서 뒤로 가기 뒤 FREE start/done을 확인했다. 통과. |
| 19 | HTML parse와 V8 초기화가 UI thread를 막는가 | Android는 bootstrap executor, iOS는 global user-initiated queue에서 실행한다. 두 로그 모두 `is_main_thread=false`다. 통과. |
| 20 | fixture flag가 켜진 제품 기본 artifact만으로 결과가 재현되는가 | 두 앱 기본 빌드에서 C04.12 fixture flag는 0이며 기본 asset 경로가 독립 실행됐다. 별도 test suite는 실행하지 않았다고 근거에 구분한다. 통과. |

## 수정한 발견

- 첫 iOS 빌드에서 Objective-C selector의 Swift importer 이름과 nullable runtime report를 잘못 가정한 컴파일 오류가 발견됐다. 실제 Swift 진단에 맞게 `createRuntimeSession(withRootHTML:)` 호출 및 report fallback을 수정하고 iOS Simulator 빌드를 다시 통과시켰다.
- 검토 중 Android가 root asset 누락 뒤 기존 smoke만 실행할 수 있는 분기를 발견해 fail-closed 처리했다. iOS는 기존 smoke asset 누락이 root document loader를 막지 않도록 경로를 분리했다.
- Android Activity 종료 시 session free를 호출하도록 구현했고 API 37 AVD의 back/close 경로에서 해제 로그를 확인했다.

이 검토는 정적 HTML의 document session 전달을 다룬다. viewport/page scale, OS 색상 preference, CSS `color-scheme` 효과와 HTML GPU rendering의 검토·구현 근거로 확대하지 않는다.
