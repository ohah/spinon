# C11.1 parsed Root HTML `grid-auto-rows` fixed-px list runtime 후보

## 구현 범위

기본 `RuntimeSession::new_runtime_gpu_with_root_html()` 경로에서 `grid-auto-rows`의 비어 있지 않은 finite·non-negative px track list를 implicit rows에 순환 적용한다. 첫 implicit row는 목록의 첫 값부터 시작하며 explicit rows에는 목록을 적용하지 않는다. `grid-auto-flow: row`, `grid-auto-columns: auto`, 기존 fixed-size item·placement·resource 제한을 유지한다. C11.4 fixture profile이나 fixture 전용 앱 ABI는 추가하지 않았다.

입력은 [Chrome 156 사전 비교 fixture](../../../../tests/fixtures/css/c11/root-html-grid-auto-row-list-precomparison.html)와 바이트 단위로 같다(SHA-256 `963d5b05c6c64c8f64638e3cc6d4268cf364e9c52c8d595d0ca973814e77d857`). 독립 기대값은 explicit row 20px, gap 5px, implicit pattern `[15,25]px`에서 첫 세 implicit row `[15,25,15]px` 및 시작점 `25,45,75px`다. 여덟 item frame은 Rust runtime test에서 exact 비교한다. 1·2·3 implicit-row matrix, 0px track, invalid/mixed list 거부, row expansion과 auto-row list 각각의 10,000-track cap, allocation failure 및 same-worker recovery도 확인한다.

## Rust 검증

| 검사 | 결과 | 원본 |
| --- | ---: | --- |
| Root HTML cascade positive·non-px rejection | 3/3 | [최신 로그](./cascade-tests-final-clean.log) |
| cascade track-list reservation failure·복구 | 1/1 | [최신 로그](./cascade-allocation-recovery-test-final-clean.log) |
| Root HTML projection | 11/11 | [최신 로그](./projection-tests-final-clean.log) |
| layout pattern·zero·invalid·no-implicit-row·list/expansion limit | 6/6 | [최신 로그](./layout-list-tests-final-clean.log) |
| layout expanded-row allocation failure·복구 | 1/1 | [최신 로그](./layout-allocation-recovery-test-final-clean.log) |
| parsed Root HTML runtime matrix | 20/20 | [최신 로그](./runtime-tests-final-clean.log) |
| 변경 네 크레이트 all-target Clippy `-D warnings` | 통과 | [최신 로그](./clippy-final.log) |
| `cargo fmt --all -- --check`, `git diff --check` | 통과 | [포맷](./format-check-final.log) · [diff](./diff-check-final.log) |

로그 본문은 테스트 결과와 재현 정보를 유지하고, 저장소 diff 검사에 잡히는 불필요한 EOF 빈 줄과 Xcode 출력의 줄 끝 공백만 정규화했다. iOS build command output은 [정규화 로그](./ios-simulator-build-pass03-clean.log)로 제공하고 빌드는 첨부 스크립트로 다시 실행할 수 있다.

검토 과정의 음성 assertion stage mismatch log는 [별도 보존](./runtime-tests-align-negative-pass01-failure.log)했다. 제품은 unsupported alignment를 cascade에서 거부했고, 테스트 기대 오류 코드를 실제 경계에 맞췄다. 초기 HostDocument probe 실패는 이 아래 Simulator 절에 기록했다.

## Android 및 iOS Simulator

최신 layout resource-cap 변경이 포함된 앱을 재빌드하고 동일한 [HTML 입력](./root-html-grid-auto-row-list-runtime.html), 기본 `index.html` route, corrected [V8 DOM probe](./root-app-runtime-probe.js)로 실행했다. C04 runtime GPU 외 C04/C11/C15/C18/C19 fixture flags는 모두 꺼져 있었다. 양 플랫폼에서 `layout=ready boxes=11`, root CSS height 100px, `presented boxes=11`을 확인했다.

| 대상 | 최신 실행 결과 | 최신 근거 |
| --- | --- | --- |
| Android API 37 ARM64 AVD `spinon_api37_0_applied_txn`, ANGLE/Vulkan/SwiftShader | viewport `411.42856×841.52374`, root `411.42856×100`; 11 boxes draw/present | [화면](./android-api37-avd-pass03.png) · [실행 요약](./android-api37-avd-runtime-pass03-summary.log) · [빌드](./android-build-pass04.log) |
| iPhone 17 Pro, iOS 26.2 Simulator, Metal Simulator GPU | viewport `402×778`, root `402×100`; 11 boxes draw/present | [화면](./ios-26_2-simulator-pass03.png) · [실행 요약](./ios-simulator-runtime-pass03-summary.log) · [빌드](./ios-simulator-build-pass03-clean.log) |

최신 Android APK SHA-256은 `1abecc713c812165f79f4b36e4d8f1d4febd4a2188d6616c5db69352fb4394d2`, iOS executable SHA-256은 `4c94f1b806ba770d791b97d30ca7794ff3b86386de3ea041856b0fe1bfe45e61`이다. Pass01/pass02 산출물과 실패 로그를 덮어쓰지 않고 보존했다. 기기 UDID, fixture flags, screenshot/input/app hashes는 [관측 JSON](./platform-runtime-observation.json)에 있다. Simulator build 중 임시로 바꾼 example/generated bootstrap assets는 원본 hash로 복구했고, pass02 복구 hash는 [이 파일](./restored-bootstrap-sha256.txt)과 `original-bootstrap-pass02/`·`original-bootstrap-pass03/`에 있다.

초기 Android probe는 HostDocument body에 없는 `querySelector`를 호출해 probe만 실패했다. 실패 log/screenshot을 보존하고 `firstChild` 순회 probe로 고친 뒤 pass01 이후 성공했다. 이는 제품 runtime 실패가 아니다. iOS build logs에는 기존 dSYM duplicate-object와 AppIntents metadata 경고도 보존했다.

스크린샷은 화면 표시 확인이며 PixelCopy·Metal drawable readback이나 픽셀 동등성 근거가 아니다. 최신 Simulator marker는 layout-ready와 draw/present를 증명하고, item별 정확한 geometry는 Rust runtime test가 증명한다. 실기기, hardware GPU, iOS compositor 이후 출력, WPT 적합성 또는 일반 CSS Grid 지원은 검증하지 않았다. 두 앱은 캡처 뒤 종료했다.

## 실패 관점 검토와 범위 판정

[구현/runtime 실패 관점 20개](./implementation-runtime-review-20.md)와 별도 [구현 PR diff 실패 관점 30개](./pr-diff-review-30.md)를 작성했다. 검토 중 발견한 두 경계 누락은 invalid auto-row list를 implicit expansion 전에 검사하도록 수정하고, list 자체에 공통 10,000-track cap을 적용해 회귀 테스트로 확인했다.

C11.1/C11 parent checklist와 공식 완료 `72/213`은 유지한다. 이 후보는 기본 parsed Root HTML 경로의 fixed-px implicit-row list만 연결한다. Arbitrary track values, named-area 조합, `grid-auto-columns` lists, implicit columns, 일반 CSS admission, WPT 또는 C11 전체 완료를 의미하지 않는다.
