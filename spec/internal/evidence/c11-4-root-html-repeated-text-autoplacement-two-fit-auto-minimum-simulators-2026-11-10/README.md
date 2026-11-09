# C11.4 두 fit-content auto-placement 자동 최소 크기 구현 근거

이 디렉터리는 C11.4 계획에서 정한 `min-height:auto` bounded profile의 RuntimeSession, iOS Simulator, Android AVD 실행 결과를 보존한다. Chrome 157은 기대값 oracle이며, 이 자료는 전체 CSS Grid 지원·WPT·실기기·pixel parity를 주장하지 않는다.

## Rust 실행

- `runtime-tests.log`: two-fit auto-placement 관련 RuntimeSession 테스트 5개. 기존 `min-height:0` 2,880 scene, 새 `min-height:auto` 2,880 scene, controls, 거부·atomic recovery가 모두 통과했다.
- 실행 명령: `mise exec -- cargo test --locked -p spinon-runtime --lib parsed_root_html_two_fit_auto_placement -- --nocapture`.
- feature-gated trace 명령: `mise exec -- cargo test --locked -p spinon-runtime --lib parsed_root_html_two_fit_auto_minimum_trace_preserves_floor_above_zero_caps --features c11-4-grid-track-sizing-fixture-test -- --nocapture`.
- feature-gated automatic-minimum trace 검사도 통과했다. 별도 실행 로그는 보존하지 않았다.
- `clippy.log`: `cargo clippy --locked --workspace --all-targets -- -D warnings` 통과.
- `mise exec -- cargo fmt --all -- --check`와 `git diff --check`도 통과했다.
- 저장소 diff 검사에 걸리는 줄 끝 공백과 마지막 빈 줄만 로그 사본에서 정규화했다. 나머지 출력은 보존했다.
- `implementation-runtime-review-20.md`와 `pr-diff-review-30.md`는 서로 다른 실패 관점의 검토 기록이다.

## 앱 실행

공통 fixture는 `root-html-auto-minimum-runtime.html`이며, app marker는 `root-app-auto-minimum-runtime.js`다. 각 로그에서 기본 Root HTML route boot pass, `layout=ready`, `draw ... presented`를 확인했다.

최종 code commit `10c36104fdce8c6c99ae2dafc6df68a795524acd`에서 app을 다시 빌드한 뒤 네 가지 scheme/platform을 실행했다. 아래 화면과 실행 로그는 이 최종 code commit의 결과다.

처음 실행에서는 inactive `display:none` color-scheme Grid의 alignment 값이 현재 scheme 계산을 막았다. style projection에서 숨겨진 subtree의 alignment 허용 범위를 현재 visible tree 검사에서 제외하고, 숨겨진 node는 layout 전용 기본 `justify-content`를 사용하도록 수정했다. 같은 RuntimeSession에서 light/dark를 전환하는 회귀가 둘 다 Ready와 draw까지 확인한다.

| 대상 | 실행 로그 | 화면 |
| --- | --- | --- |
| iPhone 17 Pro · iOS 26.2 · light | `ios-light.log` | `ios-iphone17pro-ios26_2-light.png` |
| iPhone 17 Pro · iOS 26.2 · dark | `ios-dark.log` | `ios-iphone17pro-ios26_2-dark.png` |
| Android API 37.2 ARM64 AVD · light | `android-light.log` | `android-api37_2-light.png` |
| Android API 37.2 ARM64 AVD · dark | `android-dark.log` | `android-api37_2-dark.png` |

Android graphics device는 AVD의 Goldfish GFXStream/llvmpipe (LLVM 21.1.4)를 통한 software Vulkan이다. 시뮬레이터 이미지는 layout과 draw/present의 시각 확인용이다. 네이티브 compositor 후 픽셀 동일성이나 하드웨어 GPU 결과는 측정하지 않았다.

## 빌드와 자원 정리

- `ios-build.log`와 `android-build.log`는 task fixture app의 각 플랫폼 build 결과다. 복구 후 기본 bootstrap 입력으로 다시 빌드했으며, 저장소 bootstrap 소스는 작업 전 SHA-256과 일치한다.
- Android 재실행 AVD의 renderer는 Goldfish GFXStream/llvmpipe의 software Vulkan이다. hardware GPU 결과가 아니다.
- 앱 프로세스를 종료했다. 작업 전용 iOS simulator profile과 실패한 task 전용 Android AVD를 삭제했다. 검증에 빌린 공유 iPhone profile 및 Android AVD는 shutdown 상태로 보존했다.
- 마지막 fixture 실행 후 `examples/bootstrap/index.html` 및 `root-app.js`를 복원하고 기본 bundle/iOS/Android app을 다시 빌드했다. 원본 SHA-256은 각각 `e7ecee32a5c5f645cc90ea7bd0c524caaf9da93fd4c780ff9dbb29d0894dd3b9` 및 `b87b53883ce838df86412657e1353f8dc90435049eb8ead6ba8fa478ca8a1c32`와 일치한다. 임시 Android `local.properties`와 fixture backup도 제거했다.
- 현재 실행 중인 iOS/Android 기기는 없고, S03 Pilot 이름의 iOS profile은 현재 Simulator 목록에 없다.
