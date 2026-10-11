# C12.4 실제 runtime 트리 깊이 · 시뮬레이터 실행 근거

## 실행 범위

Android 실기기를 연결하지 않은 상태에서 Android AVD와 iOS Simulator만 실행했다. 제품 앱의 실제 V8 JavaScript → HostDocument → Stylo → Taffy → WGPU runtime 경로에 C12.4 진단을 주입했다. 이 결과는 깊이 한계와 초과 후 worker 복구를 확인하며, 전체 C12.4 완료나 pixel 동등성을 뜻하지 않는다.

| 플랫폼 | 환경 | 앱 결과 |
| --- | --- | --- |
| Android | `emulator-5554`, API 37 / Android 17, `arm64-v8a`; renderer는 ANGLE/SwiftShader | HostDocument depth 1,024 계산 성공(2,715ms), 1,025 거부(`tree_depth_exceeded`, 12ms), 같은 worker에서 새 얕은 revision 복구(4ms). 마지막 장면 10 boxes WGPU 제출. 앱 PID 5943. |
| iOS | iPhone 17 Pro Simulator, iOS 26.2 | depth 1,024 계산 성공(924ms), 1,025 거부(`tree_depth_exceeded`, 9ms), 같은 worker에서 새 얕은 revision 복구(2ms). 앱 PID 4264. |

각 실행은 기존 10-node C12.4 HostDocument에서 `c124-root` 아래 1,023개 요소를 붙여 해당 root 기준 총 depth 1,024를 만들었다. 응답은 `layout=ready`, 1,033개의 전체 scene box, 같은 수의 cascade 재계산 스타일을 확인했다. 이어 한 요소를 추가해 depth 1,025로 만들었고, 첫 초과 node `1034`와 허용값 `1024`를 포함한 `tree_depth_exceeded`를 받았다. 마지막 단계에서 깊은 subtree를 기존 HostDocument에서 제거하고 같은 runtime host에 새 revision을 제출해 원래 10 boxes를 복구했다.

같은 Android Debug APK에서 C12.4 26-scene matrix도 다시 실행했다. 77 source node, 21 expected-ready, 5 expected-rejection의 전체 26 case가 일치했다. 이는 AVD의 runtime geometry·진단 회귀이며 scene별 surface pixel 비교는 아니다.

## 산출물

- Android 깊이 실행: [원시 log](c12-4-layout-depth-safety-android-avd-2026-10-11.log) · [화면 캡처](../../public/c12-4-android-api37-depth-safety-2026-10-11.png)
- iOS 깊이 실행: [원시 log](c12-4-layout-depth-safety-ios-simulator-2026-10-11.log) · [화면 캡처](../../public/c12-4-ios-iphone-17-pro-depth-safety-2026-10-11.png)
- 최종 Android AVD 회귀 matrix: [원시 log](c12-4-layout-depth-safety-android-avd-matrix-2026-10-11.log) · [화면 캡처](../../public/c12-4-android-api37-depth-safety-matrix-2026-10-11.png)
- 구현 경계 검토: [별도 실패 관점 검토](c12-4-layout-depth-safety-implementation-review-2026-10-11.md)

## 해석 한계

- Android는 AVD 소프트웨어 그래픽 경로다. 실기기 CPU·GPU·thread stack 특성으로 일반화하지 않는다.
- iOS도 시뮬레이터 결과이며 실제 iPhone의 thread stack과 메모리 사용량을 확인하지 않았다.
- 두 앱 모두 CSS cascade worker stack을 8 MiB로 설정했다. 앱 안의 단일 session 실행만 검증했다. 여러 session이 동시에 유지될 때의 예약/commit 메모리 및 실제 RSS 증분은 측정하지 않았다.
- 화면 캡처는 색상 장면이 다시 제출된 시각 확인이다. Android surface pixel 대조, iOS Metal drawable readback 및 Chrome과의 pixel 동등성 검증은 하지 않았다.
- detached subtree의 HostDocument node reclamation과 V8 wrapper/GC는 이 실행 범위에 포함하지 않는다.
- 깊이 `1024`는 현 구현의 임시 resource cap이다. 전체 CSS 호환 보장 또는 모든 기기에서의 성능 약속이 아니다.

## 회귀 명령 결과

- `mise exec -- cargo test --locked --workspace`: 통과. 새로 보강한 같은 HostDocument 새 revision 복구 테스트 포함.
- `mise exec -- cargo clippy --locked --workspace --all-targets -- -D warnings`: 통과.
- `mise exec -- cargo fmt --all -- --check`: 통과.
- `mise exec -- bun run test:css-reference`: 170/170 통과.
- Android Debug APK와 iOS Simulator 앱 빌드: 모두 통과. Android Gradle은 compile SDK 37.2가 현 AGP 검증 상한 36.1보다 높다는 경고를 냈지만 APK 빌드와 설치는 성공했다.
- 최신 Android AVD C12.4 scene matrix: 26/26 일치(21 ready, 5 expected rejection).

## 후속 동시 session 메모리 관측

이 문서의 단일 session 측정 뒤 별도 [동시 session 비용 측정](c12-4-concurrent-session-stack-cost-2026-10-11.md)에서 Android API 37 AVD와 iPhone 17 Pro iOS 26.2 Simulator에 0·1·2·4·8개 C12.4 host를 누적 생성하고, 각 단계 3회 샘플 뒤 8개를 해제해 10초 후 다시 측정했다. iOS `footprint`는 process physical footprint와 전체 stack dirty category를, Android는 `dumpsys meminfo` PSS/RSS와 `/proc/status` VmSize를 제공했다. CSS worker별 stack commitment는 분리되지 않았다. Android `/proc/maps`·`smaps_rollup` 접근은 permission denied였고 iOS `vmmap`은 Simulator DYLD 정보를 읽지 못했다. 따라서 실기기 RSS·thread별 stack 사용량·누수 판정은 미확정이다. 이 후속 측정은 앞 절의 단일 session 결과를 대체하지 않는다.

## 후속 iOS Metal 표면 pixel 관측

별도 [Metal surface 비교 계획](../../../plan/c12-4-ios-metal-surface-pixels.md)과 [8-case 결과](c12-4-ios-metal-surface-pixels-2026-10-11/)에서 iPhone 17 Pro / iOS 26.2 Simulator의 실제 WGPU Metal surface texture를 읽었다. 변경된 reverse Flex fixture를 포함한 방향·order 8개 case가 고정 Chromium 154 표본과 8/8 exact match했다. 이 결과는 본 문서의 깊이 fixture를 pixel 비교한 것이 아니며 iOS 전체 26-scene matrix·실제 iPhone·하드웨어 GPU 검증도 아니다.
