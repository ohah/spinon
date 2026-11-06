# C11.4 Root HTML ASCII letter-word intrinsic sizing

이 후보는 기본 parsed Root HTML 앱 경로의 white-space: normal direct-text allowlist를 [A-Za-z]+ token으로 넓힌다. Token 구분자는 CSS collapsible ASCII SPACE, TAB, LF, CR이고 원문 text·Text node identity는 보존한다. 범위와 독립 기대값은 [계획](../../../../plan/c11-root-html-intrinsic-letter-words.md), [Chrome 사전 비교](../c11-root-html-intrinsic-letter-words-precomparison-2026-11-06/README.md), [Agent Browser Chrome 표본 위치](./chrome/README.md)에 고정했다.

## Rust 검증

- cargo fmt --all -- --check — 통과.
- cargo test --locked -p spinon-style-to-layout --lib c11_1_root_html_intrinsic_auto_rows_text_tests — 10/10 통과.
- cargo test --locked -p spinon-runtime --lib root_html_grid_intrinsic_auto_rows:: — 14/14 통과.
- cargo clippy --locked -p spinon-style-to-layout --all-targets -- -D warnings — 통과.
- cargo clippy --locked -p spinon-runtime --all-targets -- -D warnings — 통과.

Focused 10-scene matrix는 ASCII 단어와 공백 형태의 min-content·auto column geometry, DOM source 순서 및 identity를 검사한다. 별도 fixture feature 검사는 track trace와 원문 source 보존도 확인한다. [구현/runtime 실패 관점 검토](./implementation-runtime-review-20.md)에서 이 동작과 제한을 검토했고, [실제 구현 PR diff 검토](./pr-diff-review-30.md)는 별도로 기록했다. 패키지 전체 테스트 결과를 이 기록에서 주장하지 않는다.

## Android와 iOS Simulator

최종 실행은 [pass07](./pass07/environment.txt)다. 두 플랫폼은 같은 HTML fixture SHA-256 377f53c7569bda6a5bd79f44366bcd79a759d102416bfa13ff43af2c6c2ce72a를 사용했다. 정확한 소스 입력 목록과 manifest hash는 [source-inputs.sha256](./pass07/source-inputs.sha256) 및 environment.txt에 기록했다.

| 플랫폼 | 실행·surface | Root HTML app 확인 | 표본 |
| --- | --- | --- | --- |
| Android | API 37 ARM64 AVD emulator-5554, density 2.625, ANGLE/Vulkan/SwiftShader, surface 1080×2209 | layout ready, boxes=8, app eval error=none, draw presented, runtime_text_count=2 | CSS (42,19) → surface (111,51), [91,168,245,255], PixelCopy 1/1 exact |
| iOS | iPhone 17 Pro Simulator iOS 26.2 (9DCCE836-7659-4914-B6C1-13F54BFFA9DB), viewport 402×778 CSS px, Metal drawable 1206×2334 | root app marker ready, layout ready, boxes=8, draw presented, runtime_text_count=2 | CSS (42,19) → drawable (127,58), [91,168,245,255], Metal 1/1 exact |

해당 점은 40px Grid 오른쪽 경계 밖이면서 긴 text item 배경 안이다. 화면 캡처와 직접 surface readback은 [Android](./pass07/android/) 및 [iOS](./pass07/ios/) evidence에 있다. iOS 표본은 compositor 제출 전 drawable이고 Android는 software renderer AVD다. 모바일 로그는 모든 item frame을 덤프하지 않으므로 개별 mobile log만으로 전체 geometry matrix 통과를 주장하지 않는다. 앱 실행기는 fixture source·원문 text와 자식 순서를 확인한 다음 data-runtime=ready를 설정한다.

## 기본/fixture 산출물 경계

- SPINON_ENABLE_C11_GRID_METAL_SURFACE_PIXELS_TEST=1 Android APK의 classes2.dex에는 C114RootHtmlLetterWordPixelCopyAdapter, run ID와 sample ID가 있다. iOS feature executable에도 해당 sample marker가 있다. [feature-positive-control-final.txt](./artifact-boundary/feature-positive-control-final.txt).
- 같은 runtime GPU build에서 C11 pixel-readback flag를 끈 Android APK의 모든 classes*.dex에는 adapter·run ID·sample ID가 없다. iOS default Simulator executable에서도 run ID·sample ID·scene marker가 없다. 각 산출물 SHA-256은 [Android](./artifact-boundary/android-default-artifact-final.txt) 및 [iOS](./artifact-boundary/ios-default-artifact-after-isolation.txt)에 기록했다.
- 첫 artifact 검토에서 Android 공통 app class에 넣은 run/sample ID가 기본 APK에 남는 결함을 발견했다. 코드를 C11 fixture-test source set의 ServiceLoader adapter로 옮긴 뒤 default 부재와 feature-on positive control을 모두 재검증했다. 이전 실패 관측은 [분리 전 artifact 결과](./artifact-boundary/android-default-artifact-before-isolation.txt) 및 [positive control](./artifact-boundary/feature-positive-control-before-isolation.txt)에 보존했다.
- Xcode build stdout는 원본 byte를 보존하는 `.log.gz`로 저장했다. 원문 재생은 `gzip -cd <file.log.gz>`로 할 수 있다.

## 실행기 시도 기록

- pass01: app layout/draw와 PixelCopy 표본은 통과했지만 검증 스크립트가 실제 ..._PIXEL marker 대신 존재하지 않는 ..._PIXELCOPY_SAMPLE을 찾았다. PNG를 보존한 뒤 matcher를 수정했다.
- pass02: 같은 정적 run ID의 PixelCopy PNG가 cache에 남아 capture helper가 재기록을 거부했다. 재시도 전 해당 PNG를 evidence에 보존하고 그 파일 하나만 제거했다.
- pass03: 양쪽 표본은 exact였지만 artifact 검사에서 Android 기본 APK에 작업별 문자열이 남는 것을 발견했다. 따라서 최종 구현 evidence로 취급하지 않는다.
- pass04: fixture-only Android adapter로 격리한 뒤 Android/iOS 실행과 default/feature artifact 경계를 통과했다. 이후 Android main app class의 무의미한 서식 변경을 되돌려 실행 입력 hash와 정확히 맞추기 위해 최종 pass를 다시 만들었다.
- pass05: source manifest에 존재하지 않는 Android capture 파일 경로를 넣어 시뮬레이터 실행 전에 중단됐다. 앱·bootstrap 자산은 변경되지 않았다.
- pass06: 당시 Android 전체 `logcat` buffer를 지우고 실행했다. 구현 자체와 두 시뮬레이터 표본은 통과했지만 PR review에서 이 로그 정리가 불필요함을 발견했다.
- pass07: 전체 `logcat`을 지우지 않도록 실행기를 수정하고 현재 앱 PID의 로그만 수집했다. Android/iOS 표본이 모두 exact였고 `environment.txt`는 bootstrap 및 iOS app bundle asset을 byte-for-byte 복원했다고 기록한다.

capture 후 Android/iOS 앱은 종료됐다. PR 화면과 두 이미지 링크는 별도 Agent Browser 세션에서 확인한 뒤 세션을 종료했다. 기존 Chrome oracle은 [Agent Browser 캡처](./chrome/runtime-360x800.png)다.

## 범위와 남은 한계

이 제한 fixture 후보는 일반 CSS admission, Unicode/임의 font shaping, WPT 적합성, hardware Android GPU, Android 실기기, iOS compositor 이후 pixels, 공개 CSS 지원 또는 전체 C11.4를 증명하지 않는다. C11.4/C11 checkbox와 공식 완료 78/214는 미변경이다. PR check/CI 결과도 이 로컬 검증에 포함하지 않는다.
