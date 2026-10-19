# C11.4 Ahem text 앱 후속 PR diff 적대 검토 30개

**대상:** `origin/main` 대비 C11.4 Ahem text scene 후속 diff 전체. 위 [구현 검토 20개](implementation-review-20.md)와 별도로 저장소 계약, 플랫폼 조합, 근거 재현성, 문서 상태와 PR 전달 경계를 점검했다.

| # | PR 변경에서 확인한 실패 경로 | 확인 내용과 판정 |
|---:|---|---|
| 1 | Chrome oracle가 변동 채널/브라우저 버전으로 기록되는가 | Agent Browser 0.31.1과 HeadlessChrome 155.0.8059.85 user agent를 JSON에 기록했다. |
| 2 | 저장 fixture와 앱 scene의 CSS가 다르게 비교되는가 | 같은 Ahem 선언과 문자열을 Chrome oracle와 app scene에서 실행했고 두 관찰을 JSON에 분리했다. |
| 3 | precomparison row 20px와 app row 40px이 불일치로 오인되는가 | definite-height app root의 `align-content:stretch`가 auto row를 40px로 늘리고 `align-items:start`가 item을 20px로 두는 이유를 관찰 자료와 README에 썼다. |
| 4 | min/max 값만 같고 app layout frame은 잘못되는가 | Android, iPhone, iPad 실행에서 app assertion이 root와 item의 width·height를 모두 검사했다. |
| 5 | JSON trace에만 값이 있고 실제 runtime scene이 실행되지 않는가 | Android/iOS 앱을 simulator에서 scene ID와 고유 run ID로 실행하고 `status=0`, 2 boxes, frame, diagnostics를 로그로 보존했다. |
| 6 | 화면 이미지가 glyph를 보여준다고 과장되는가 | README는 renderer의 glyph skip을 명시하고 각 screenshot은 box geometry 증거로 제한했다. |
| 7 | Android 실기기 실행으로 잘못 보고되는가 | Android 증거를 API 37 ARM64 AVD/SwiftShader로 특정했고 real device와 hardware GPU를 배제했다. |
| 8 | iOS simulator 기종/버전을 식별할 수 없는가 | iPhone 17 Pro 및 iPad Pro 13-inch M5, iOS 26.2 Simulator와 run ID를 각각 기록했다. |
| 9 | screenshot이 존재하지만 어느 플랫폼인지 모르는가 | 화면 파일명과 README 표에 플랫폼/기종/해상도를 대응시켰다. |
| 10 | screenshot이 검증 후 바뀌었는지 판별할 수 없는가 | PNG별 SHA-256을 저장하고 실제 로컬 파일 hash를 대조했다. |
| 11 | 삭제된 APK/JNI library hash가 retained file처럼 오해되는가 | hash 목록과 README는 APK/JNI 산출물이 디스크 정리 때 삭제됐고 hash만 남았다고 명시한다. |
| 12 | Android build가 실패했는데 성공으로 보고되는가 | 첫 Gradle copy의 `No space left on device` 로그와 재개 패키징의 `BUILD SUCCESSFUL` 로그를 함께 보존했다. |
| 13 | Android 재시도가 다른 Rust 산출물을 사용했는지 알 수 없는가 | 재시도는 앞선 성공한 JNI library를 사용했으며 첫 실패 위치가 JNI merge copy 단계임을 기록했다. |
| 14 | iOS build 오류를 숨기고 최종 성공만 남기는가 | C11.2.3 FFI undeclared symbol의 최초 실패 로그와 feature guard 수정 뒤 성공 로그를 둘 다 보존했다. |
| 15 | iOS 성공 로그가 feature 조합을 혼동하는가 | `c11_2_3_grid_fixture=0`의 의도된 build 조합을 README에 명시했다. |
| 16 | 새 C11.2.3 preprocessor guard가 compile test에 빠지는가 | 실제 iOS Simulator build를 수행했고 기존 build-flag JS 검사 13개를 통과시켰다. |
| 17 | 해당 JS test가 feature 값 대신 문자열 배열 순서에 의존하는가 | 고정 배열 전체 equality를 제거하고 필요한 core, layout dep, C11.4 forwarding을 독립 확인하도록 바꿨다. |
| 18 | fixture helper가 프로젝트의 fixtures module 경계를 벗어나거나 opt-in compile/link 경로를 우회하는가 | 후속 검토에서 찾은 helper를 `runtime_layout/fixtures/c11_4_text.rs` 아래로 이동했고 module과 entry point 모두 C11.4 feature-gated 상태인지 확인했다. |
| 19 | feature-test artifact에 실수로 C11.4 scene이 빠지는가 | Android positive-control library 검색 결과에 scene ID, item ID, text payload와 Ahem marker가 있다. |
| 20 | default artifact에 fixture payload가 남는가 | 65MB default `spinon-ffi` static library를 scan해 네 종류 marker가 모두 없음을 저장했다. |
| 21 | C04 fixture build와 새 하위 feature가 Cargo dependency에서 끊기는가 | runtime feature가 하위 layout·render feature를 켜고 app build flags test가 FFI manifest forwarding도 확인한다. |
| 22 | projection helper가 unrelated profile에 재사용되는가 | entry point는 `RuntimeBlockFlexGridTrackSizingV1`와 동일 viewport를 검사하고 function은 C11.4 feature 아래에서만 export한다. |
| 23 | 앱 report JSON이 잘려 contribution proof가 사라지는가 | Android는 5개, iPhone/iPad는 각 10개 trace chunks 전체를 run ID별로 보존했다. |
| 24 | 앱 assertion에서 먼저 실패해 trace가 남지 않는가 | Android/Swift path가 frame assertion 전에 diagnostics chunks를 기록하며 각 log에 trace와 summary가 함께 있다. |
| 25 | locale/환경 변화가 font selection에 개입하는가 | runtime engine은 system fonts를 비활성화하고 repository Ahem binary를 사용한다. |
| 26 | PR이 기능 상태를 C11.4 완료로 올리는가 | `spec/STATUS.md`는 C11.4와 C11을 미완료로 유지하고 이번 고정 scene의 범위만 하위 기록으로 추가한다. |
| 27 | 내부 contract가 Rust-only output을 FFI/public API로 잘못 약속하는가 | `spec/internal/0059-c11-grid-layout.md`는 layout values의 Rust 경계와 검증용 opt-in report를 제품 API와 구분한다. |
| 28 | 계획이 single scene을 일반 contribution matrix로 확장 해석하는가 | 계획에 고정 Ahem 입력, 제한 범위, box-only capture, 남은 matrix/WPT/glyph 범위를 명시했다. |
| 29 | simulator 캡처가 Agent Browser evidence와 혼동되는가 | Chrome oracle JSON, Android/iOS 앱 로그 및 platform screenshots를 파일별로 분리했다. |
| 30 | browser process나 사용자 tmux session이 QA 뒤 종료되는가 | Agent Browser session/process 종료를 evidence에 적고 tmux 유지와 명시적으로 구분했다. |

**결과:** 검토 과정에서 발견된 iOS feature-off symbol 호출, 오래된 build-flag test expectation, helper의 fixtures module 경계 위반은 diff에서 수정하고 재검증했다. 나머지 범위/해석 위험은 README·contract·STATUS에 제한으로 기록했으며 미해결 blocker는 없다. 이 결과는 GitHub PR의 live required checks나 merge 상태를 대체하지 않는다.
