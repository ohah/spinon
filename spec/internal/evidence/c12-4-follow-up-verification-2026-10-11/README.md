# C12.4 후속 검증 결과 · 2026-10-11

**판정:** 앞서 미실행으로 남긴 검사 중 시뮬레이터에서 수행 가능한 iOS Metal 표면 픽셀, Android/iOS 동시 session 측정, worker별 stack bounds/mincore 표본을 완료했다. 당시 누락은 기술적으로 불가능해서가 아니라 Android 중심 fixture 단계에서 검증을 멈춘 진행 누락이었다. 기존 process-level Android `/proc` 직접 읽기는 권한 거부였지만, 후속 AVD 실행에서 package UID `run-as`로 TID별 `maps`·`smaps`를 읽었다. iOS `vmmap` 요약 실패와 별도로 앱 내부 worker별 pthread/mincore 계측은 성공했다. 공식 WPT의 Spinon 적합성, 실기기별 stack/RSS, 전역 allocator OOM recovery는 별도 경계다. 후속으로 iOS 전체 26-scene runtime matrix도 완료했으며 별도 [실행 계획](../../../../plan/c12-4-ios-runtime-matrix.md)과 [비교 근거](../c12-4-ios-runtime-matrix-implementation-review-2026-10-11.md)에 기록했다. C12.4 완료로 표시하지 않는다.

## 새로 끝낸 항목

- iPhone 17 Pro / iOS 26.2 Simulator에서 변경된 reverse Flex fixture를 새 앱 프로세스별로 실행하고 실제 WGPU Metal surface texture에서 8개 방향/order pixel을 직접 읽었다. 고정 Chromium 154 표본과 8/8 exact match, channel tolerance 0이다. 최초 row-reverse 표본 위치 실패는 실패 run으로 보존했고, parent geometry를 맞춘 후 전체 8개를 재실행했다. [계획·판정 기준](../../../../plan/c12-4-ios-metal-surface-pixels.md) · [비교 JSON](../c12-4-ios-metal-surface-pixels-2026-10-11/run-c124-ios-20261011T105945959Z-ce5b401c/comparison.json).
- iOS Simulator와 Android API 37 AVD에서 C12.4 session 0·1·2·4·8개를 생성해 각 단계 세 번 샘플링하고 모두 해제한 뒤 10초 후 재측정했다. 별도 후속 probe는 두 플랫폼에서 worker별 3회 pthread/mincore 표본을 수집했고, Android는 `run-as` TID별 `maps`·`smaps` 및 해제 뒤 VMA 부재를 확인했다. process `footprint`/PSS/RSS/VmSize만으로 worker stack을 귀속하지 않았다. 초기 Android process-level `/proc` 직접 읽기 거부와 iOS `vmmap` DYLD query failure는 그 실행의 도구 경계로 보존한다. [동시 session 계획](../../../../plan/c12-4-concurrent-session-stack-cost.md) · [process-level 결과](../c12-4-concurrent-session-stack-cost-2026-10-11.md) · [worker별 결과와 metric 한계](../c12-4-worker-stack-residency-2026-10-11/README.md).
- 실제 HostDocument→Stylo→Taffy→paint 경로의 8 MiB worker에서 Android AVD와 iOS Simulator 각각 depth 1,024 성공, 1,025 거부, 같은 worker의 얕은 새 revision 복구를 확인했다. 일반 test thread depth 256 stack overflow는 유지한다. 깊이 결과는 [플랫폼 실행 기록](../c12-4-layout-depth-safety-platforms-2026-10-11.md)에 있다.

## iOS 전체 runtime matrix 후속 실행

- 고정 iPhone 17 Pro / iOS 26.2 Simulator에서 같은 26-scene·77-node 입력을 실행했다. 21개 지원·5개 기대 거부가 모두 일치했고, 지원 fixture node 64개의 x/y/width/height 최대 오차는 0 CSS px였다(허용 0.5 CSS px). 이 비교는 geometry 및 거부 진단만 다루며 Metal surface pixel이나 WPT 적합성은 아니다. [계획](../../../../plan/c12-4-ios-runtime-matrix.md) · [20개 구현 실패 관점](../c12-4-ios-runtime-matrix-implementation-review-2026-10-11.md) · [실행 원본](../c12-4-ios-runtime-matrix-2026-10-11/run-c124-ios-matrix-20261011T114910232Z-7f5abd20/README.md) · [비교 JSON](../c12-4-ios-runtime-matrix-2026-10-11/run-c124-ios-matrix-20261011T114910232Z-7f5abd20/comparison.json).

## 이 세션에서 다시 실행한 회귀

| 명령 | 결과 | 원본 |
| --- | ---: | --- |
| `mise exec -- cargo test --locked --workspace` | 568 passed, 0 failed, 3 ignored | [이번 실행 로그](../c12-4-ios-runtime-matrix-2026-10-11/run-c124-ios-matrix-20261011T114910232Z-7f5abd20/cargo-test-workspace.log) |
| `mise exec -- cargo clippy --locked --workspace --all-targets -- -D warnings` | 통과 | [이번 실행 로그](../c12-4-ios-runtime-matrix-2026-10-11/run-c124-ios-matrix-20261011T114910232Z-7f5abd20/clippy.log) |
| `mise exec -- cargo fmt --all -- --check` | 통과 | [이번 실행 로그](../c12-4-ios-runtime-matrix-2026-10-11/run-c124-ios-matrix-20261011T114910232Z-7f5abd20/rustfmt.log) |
| `mise exec -- bun run test:css-reference` | 185 passed, 0 failed | [이번 실행 로그](../c12-4-ios-runtime-matrix-2026-10-11/run-c124-ios-matrix-20261011T114910232Z-7f5abd20/css-reference.log) |
| C12.4 runtime matrix 비교기 실패 제어 | 4 passed, 0 failed | [로그](../c12-4-ios-runtime-matrix-2026-10-11/run-c124-ios-matrix-20261011T114910232Z-7f5abd20/comparator-negative-controls.log) |
| 수정 문서의 상대 Markdown link 검사 | 833개 확인, 0개 누락 | 이번 실행에서 재검사 |

## 후속 재실행 · 2026-10-11

worker별 stack 표본과 iOS matrix 후속 검증을 반영한 뒤 전체 Rust workspace, CSS reference, JS suite, 정적 검사들을 다시 실행했다. 이전 표의 185개 CSS reference는 당시 실행 기록으로 보존하고, 가장 최근 실행 결과를 아래에 기록한다.

| 명령 | 결과 | 실행 로그 |
| --- | ---: | --- |
| `mise exec -- cargo test --locked --workspace` | 568 passed, 0 failed, 3 ignored | [workspace 로그](validation-final-20261011/cargo-test-workspace.log) |
| `mise exec -- bun run test:css-reference` | 189 passed, 0 failed | [CSS reference 로그](validation-final-20261011/css-reference.log) |
| `mise exec -- bun run test:js` | 2 passed, 0 failed | [JS suite 로그](validation-final-20261011/js-tests.log) |
| `mise exec -- cargo clippy --locked --workspace --all-targets -- -D warnings` | 통과 | [Clippy 로그](validation-final-20261011/clippy.log) |
| `mise exec -- cargo fmt --all -- --check` | 통과 | [rustfmt 로그](validation-final-20261011/rustfmt.log) |
| `mise exec -- cargo test --locked -p spinon-runtime --features c12-4-worker-stack-residency-test` | 157 passed, 2 ignored | [worker 계측 feature 로그](validation-refresh-20261011/worker-stack-feature.log) |
| 변경된 Markdown 문서 상대 링크 검사 | 1,056개 확인, 0개 누락 | [링크 검사 로그](validation-final-20261011/markdown-links.log) |

Android APK와 iOS Simulator 앱은 각 실제 캡처 실행에서 다시 빌드·설치했고, 상세 빌드 및 플랫폼 원본은 위 플랫폼 evidence 폴더에 보관했다. Tailnet 공개 미리보기에서 결과 HTML과 JSON의 HTTP 상태 200을 확인했다. 공개 배포나 GitHub Pages 갱신은 하지 않았다.

## 아직 닫히지 않은 경계

- **공식 WPT on Spinon:** 기존 4개 자동 후보와 1개 수동 후보 실행은 Chrome 154 기준 결과다. Spinon에는 공식 WPT `testharness.js`와 결과 보고를 수행하는 브라우저 runner가 연결되어 있지 않다. 현재 Spinon 쪽 Rust/WGPU-derived regression은 별도 실행했지만 공식 WPT 적합성이 아니다. WPT runner 통합은 구현이 필요한 후속 작업이다.
- **실기기 memory·GPU:** 이번 대상은 iOS Simulator와 Android AVD다. AVD renderer는 ANGLE/SwiftShader라 hardware GPU 근거가 아니다. 실제 기기에서 worker별 committed stack 또는 RSS를 분리할 권한·계측은 확보되지 않았다.
- **할당 실패:** 기존 실패 주입은 painter의 명시적 `Vec::try_reserve` growth point에 한정된다. 전역 allocator OOM을 강제해 앱 복구를 보장하는 검증은 하지 않았다. Rust process OOM 경로 전체를 이 fixture로 증명할 수 없다.

## 결론

첫 보고 뒤 시뮬레이터에서 재현 가능한 공백을 모두 후속 실행해 iOS 표면 readback·reverse fixture·Simulator/AVD 동시 session과 worker stack 표본·전체 runtime scene matrix를 확인했다. 당시 확인을 멈춘 이유는 기술적 차단이 아니라 진행 누락이었다. 남은 Spinon 공식 WPT runner 통합은 별도 기능 구현이고, 실기기 GPU·메모리 측정은 실제 기기 접근이 필요하며, 전역 allocator OOM 복구는 명시적 `Vec::try_reserve` 실패 주입과 다른 계약이다. 이 경계를 완료로 가장하지 않는다. C12.4, 전체 CSS 지원, Spinon WPT 적합성은 계속 미완료다.
