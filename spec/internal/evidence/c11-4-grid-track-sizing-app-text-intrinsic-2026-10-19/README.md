# C11.4 앱 경로의 고정 text intrinsic 장면

## 범위와 판정 기준

이번 slice는 고정 direct-leaf text 장면 하나를 C11.4 앱 fixture 경로에 연결한다. 장면은 Ahem 10px `XX XXXXXXX`, `grid-template-columns:min-content`, 120×40 CSS px Grid root 및 `align-items:start`를 사용한다. 구현 전에 고정한 기대값은 min-content 70px, max-content 100px, item frame 70×20px다. fixture adapter는 `runtime_layout/fixtures/c11_4_text.rs`에 있고 opt-in feature에서만 포함된다. 기준은 저장소의 C11.4 Chrome fixture와 Chrome 155 Agent Browser 관찰이며, 계산은 CSS Grid 전체 적합성 판정으로 확대하지 않는다.

Chrome 155.0.8059.85 / Agent Browser 0.31.1에서 `tests/fixtures/css/c11/grid-track-sizing-c114-precomparison.html`와 앱 scene의 정확한 선언을 각각 열었다. Ahem font가 준비된 뒤 두 장면 모두 70px column과 70×20px item을 보였다. 저장한 관찰 자료는 [Chrome 관찰 JSON](chrome155-agent-browser-observation.json)이다. 앱 선언의 40px definite grid height와 기본 `align-content:stretch` 때문에 Chrome의 auto row는 40px이며, `align-items:start`로 item은 20px 높이를 유지한다. 사전 비교 fixture에서는 row가 20px이다.

## 앱 실행 결과

Text input은 C11.4 fixture-test feature에서만 활성화한다. Rust runtime은 scene의 text, direct-leaf item owner, root Grid, Ahem 관련 computed text longhand를 검증하고, 시스템 폰트 없이 bundled Ahem을 등록한다. Android/iOS fixture report에서 direct-leaf source, font revision, min/max intrinsic values, measure callback, Columns/Rows stage trace 및 최종 frame을 복원했다.

| 실행 경로 | 환경 | 결과 | 화면 |
| --- | --- | --- | --- |
| Android 앱 | Android API 37 ARM64 AVD, SwiftShader | run `c114-text-intrinsic-review-android-20261019`; status 0, 2 boxes, root 120×40, item 70×20, min/max 70/100; assertion error 없음 | [1080×2400 PNG](android-simulator.png) |
| iPhone 앱 | iPhone 17 Pro, iOS 26.2 Simulator | run `c114-text-intrinsic-post-review-ios-iphone-20261019`; status 0, 2 boxes, 같은 min/max와 frame; assertion error 없음 | [1206×2622 PNG](post-review-ios-iphone-simulator.png) · [실행 로그](post-review-ios-iphone-simulator.log) |
| iPad 앱 | iPad Pro 13-inch M5, iOS 26.2 Simulator | run `c114-text-intrinsic-post-review-ios-ipad-20261019`; status 0, 2 boxes, 같은 min/max와 frame; assertion error 없음 | [2064×2752 PNG](post-review-ios-ipad-simulator.png) · [실행 로그](post-review-ios-ipad-simulator.log) |

세 화면은 실제 Spinon 앱 캡처지만 이 feature의 renderer가 text glyph를 그리지 않는다. 따라서 캡처는 root/item box의 위치와 색상만 보여주며 글꼴 모양이나 glyph paint parity를 증명하지 않는다. Android는 simulator AVD 경로이고 iOS 결과도 Simulator다. 실기기와 hardware GPU는 사용하지 않았다.

## 빌드·검증

- `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test`: 185 passed, 3 ignored; 추가 integration target 3개와 16개 테스트 통과.
- `cargo clippy --locked --all-targets -p spinon-runtime -p spinon-style-to-layout -p spinon-style-to-render -p spinon-style --features spinon-runtime/c11-4-grid-track-sizing-fixture-test -- -D warnings`: 통과.
- `cargo fmt --all -- --check`, `git diff --check`: 통과.
- `node --test tools/css-reference/c11-ios-build-flags.test.mjs`: 13/13 통과.
- helper를 `runtime_layout/fixtures/c11_4_text.rs`로 옮긴 뒤 focused runtime test, 전체 feature suite 및 변경 Rust package Clippy를 다시 통과했다. [focused test](post-review-runtime-focused-test-retry.log), [전체 suite](post-review-runtime-feature-suite.log), [Clippy](post-review-clippy-changed-rust.log).
- Fixture positive control의 Android `.so`에는 scene ID·item ID·text payload·Ahem 자료가 있다. helper 이동 뒤 iOS simulator용 기본 feature `spinon-ffi`도 재빌드해 scene ID, item ID, text payload, Ahem marker가 모두 없는 것을 확인했다. 디스크 부족으로 끝난 첫 시도와 opt-level 1/debug 정보 제외 재시도 모두 로그로 보존했다. [positive artifact 검색](android-fixture-artifact-positive.log), [post-review 기본 artifact 재빌드 실패](post-review-default-artifact-build.log), [post-review 재시도 성공](post-review-default-artifact-build-retry.log), [post-review marker 검색](post-review-default-artifact-symbol-check.log), [기존 기본 artifact 확인](default-artifact-symbol-check.log).
- 첫 Android build 재시도는 `SPINON_V8_DIR` 미지정으로 worktree의 V8 checkout을 찾지 못해 중단됐다. root checkout의 pinned V8을 읽기 전용으로 지정한 재시도에서는 JNI library가 빌드됐고, 174MB JNI 복사 단계가 저장 공간 부족으로 실패했다. task `target/debug` 정리 뒤 동일 JNI library로 packaging resume가 성공했다. [최초 fixture build](android-build.log), [초기 V8 경로 실패](post-review-android-build.log), [pinned V8 build와 no-space 실패](post-review-android-build-retry.log), [패키징 재개 성공](post-review-android-package-resume.log).
- iOS 첫 Simulator 빌드에서는 C11.2.3 feature가 꺼진 조합에서도 Objective-C++가 해당 FFI symbol을 참조해 실패했다. 호출부를 대응 feature flag에 맞춰 gate한 뒤 같은 의도된 조합(`c11_2_3_grid_fixture=0`)의 빌드와 두 Simulator 실행이 성공했다. fixture module 이동 뒤 iOS 빌드도 다시 성공했고 iPhone/iPad 양쪽에서 고유 run ID로 재실행했다. [첫 빌드 실패](ios-build.log), [feature guard 수정 후 빌드](ios-build-retry.log), [post-review 빌드](post-review-ios-build-retry.log).
- Android fixture native library는 저장 공간 제한으로 release profile `opt-level=1`, debug info off로 만들었다. 이는 동작 확인용 산출물이며 release 최적화나 성능 근거가 아니다.

diff 검사를 위해 공백을 정리한 9개 로그의 byte-exact 원본은 [raw-whitespace-logs.tar.gz](raw-whitespace-logs.tar.gz)에 보존했다. 읽기용 `.log` 사본에서는 stack trace의 공백 들여쓰기, 줄 끝 공백과 EOF의 여분 빈 줄만 정리했으며 실행 결과·명령·오류 내용은 바꾸지 않았다. 나머지 원본 로그는 이 디렉터리에 그대로 있다. Agent Browser session과 그 브라우저 프로세스는 확인 후 종료했다. Terminal Browser는 사용하지 않았다.

## 남은 범위

이 slice는 특정 Ahem normal-text 입력 하나만 다룬다. helper의 `ParagraphStyle`은 현재 `line-break:auto`를 `LineBreakMode::Strict`로 표현하지만, 이 ASCII 단어·공백 fixture에는 줄바꿈 결과 차이가 없다. 다른 punctuation/CJK line-break 입력에는 이 scene 결과를 일반화하지 않는다. `break-all`, `nowrap`, 다른 font·script·writing mode, 이미지, anonymous/nested item, padding/border·percentage/calc edge, contribution 선택 행렬, 모든 auto-minimum 조건, glyph paint, WPT 및 전체 C11.4는 미완료다. 이 feature-test fixture를 제품 CSS 지원으로 세지 않는다.

## SHA-256

캡처 PNG의 SHA-256은 [SHA256SUMS](SHA256SUMS)에 보존했다. APK와 JNI library는 제한된 디스크 공간을 회복하기 위해 task build output 정리 때 제거했으며, 최종 모듈 경로 재검증 때의 hash는 각각 `7aa9ecdaf3c705e7a2d0f924ff978abb0abcacec180f0c0f6dfa8f5da6363d0e` (Android debug APK), `bcdb9e2f7f3c9dd34651707469665b06fe0d9ffead95324370ed04e47dd91f8c` (Android fixture JNI library)였다. 새 iOS 시뮬레이터 캡처와 실행 로그는 별도 `post-review-ios-*` 파일로 보존했다.
