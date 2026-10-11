# C12.4 · Android·iOS 런타임 근거

**입력:** [C12.4 런타임 fixture](../../../../tests/fixtures/css/c12/runtime-stacking-context.js)  
**계약:** [내부 계약 0058](../../0058-c12-4-stacking-context.md) · [구현 계획](../../../../plan/c12-4-stacking-context.md)  
**기준:** [고정 Chromium 사전 비교](../c12-4-stacking-precomparison-2026-10-11.md)

## 실행 환경

| 대상 | 기기·OS | 그래픽 경로 | CSS viewport / surface |
| --- | --- | --- | --- |
| Android 실기기 | Samsung SM-S731N · Android 16/API 36 | Vulkan · Samsung Xclipse 940 · `Rgba8UnormSrgb` | `320×240` CSS px, scale `2.8125` / `900×675` |
| Android Simulator | `sdk_gphone16k_arm64` · Android 17/API 37.1 · 16 KB page size | WGPU `Gl` · ANGLE/SwiftShader software renderer | `320×240` CSS px, scale `2.625` / `840×630` |
| iOS Simulator | iPhone 17 Pro · iOS 26.2 · Xcode 26.2 | Metal · Apple iOS simulator GPU · `Bgra8UnormSrgb` | `320×240` CSS px, scale `3.0` / `960×720` |

Android 실기기·Android Simulator·iOS Simulator는 새 Rust 모듈 분리 뒤 앱을 다시 빌드하고 실행한 결과다. 시뮬레이터의 그래픽 경로는 실기기 GPU 증거로 간주하지 않는다. iOS 결과도 Simulator이며 iOS 실기기 결과가 아니다.

## 확인한 동작

- Android와 iOS에서 V8 fixture가 `status=0`, `document_nodes=10`으로 끝났다.
- 두 플랫폼에서 Stylo/Taffy 결과가 `layout=ready`, `boxes=10`이었다.
- 두 플랫폼의 10개 CSS frame이 같았다. 로그는 viewport가 준비되기 전 `layout=empty` 이벤트도 남기므로, 실제 결과는 Android `SPINON_C0410_EVAL` 및 iOS의 `SPINON_C0410_ENVIRONMENT` 준비 이벤트에서 확인했다.
- paint rank는 양쪽 모두 `[0:1,1:2,2:8,3:4,4:5,5:6,6:9,7:3,8:7,9:10]`이었다. 이는 positioned `z-index:auto`, 중첩 context 격리, static Flex item의 정수 `z-index`를 fixture에서 확인한다.
- Android Vulkan과 iOS Metal surface에서 각각 10개 box가 제출됐다. 두 화면 캡처에서 오른쪽 경계 잘림이나 canvas 밖으로 나간 상자는 보이지 않았다. 이 시각 확인은 픽셀별 자동 비교를 뜻하지 않는다.
- C12.4 이전 실행에서 발생한 Android demo 진입 누락을 `MainActivity` 분기와 Surface 준비 뒤 초기화 분기에 연결했다. 이번 재빌드·실기기 실행에서 실제 `layout=ready`와 WGPU 제출까지 확인했다.
- iOS는 AppDelegate의 C12.4 진입 인자로 실행했다. 이번 빌드·Simulator 실행에서도 V8 호스트 준비, 10개 노드, paint rank와 Metal 제출을 확인했다.
- 2026-10-11 15:09 KST에 현재 Rust/C12.4 소스로 iOS Simulator 앱을 다시 빌드·설치·실행했다. `SPINON_C124_EVAL status=0`, 화면 상태 `layout=ready`, `boxes=10`을 확인했다. 새 화면과 원본 로그는 [재실행 캡처](./ios-simulator-rerun-2026-10-11.png), [재실행 로그](./ios-simulator-rerun-2026-10-11.log)에 있다. 이 실행도 Simulator 결과이며 iOS 실기기 결과나 표면 픽셀 readback은 아니다.
- 2026-10-11 15:24 KST에 API 37.1 / 16 KB page size Android AVD에서 현재 APK를 다시 설치하고 C12.4 fixture를 실행했다. V8 평가는 `status=0`, `document_nodes=10`; layout은 `ready`, `boxes=10`; paint order는 기존 두 플랫폼과 같은 `[0:1,1:2,2:8,3:4,4:5,5:6,6:9,7:3,8:7,9:10]`이었다. WGPU가 선택한 adapter는 `Gl`이며 ANGLE/SwiftShader software renderer를 사용했다. 화면에서 root와 겹침 색상 장면이 보이고 10개 box 제출이 성공했다. [시뮬레이터 캡처](./android-api37-emulator-2026-10-11.png)와 [원본 logcat](./android-api37-emulator-2026-10-11.log)을 보관했다. 이 결과는 하드웨어 GPU 성능 또는 surface pixel readback 측정이 아니다.
- 2026-10-11 16:08 KST에 Android 실기기가 아닌 `emulator-5554` AVD만 연결된 것을 확인한 뒤, 수정 APK를 설치해 26-scene matrix를 실행했다. 각 장면은 새 host로 V8 → HostDocument → cascade/layout → RuntimeRenderSnapshot을 실행했다. 21개 지원 장면은 `ready`, 5개 미지원 장면은 기대한 속성/API 진단으로 거부됐고 요약은 `cases=26 nodes=77 expected_ready=21 expected_rejected=5 matched=26`이다. 거부 진단은 `visibility` 두 건과 `opacity`, `showModal`, `content` 각 한 건이다. 지원 장면의 fixture node 64개 frame을 고정 Chrome 기준과 비교한 최대 절대 오차는 `0 CSS px`이며 허용치는 `0.5 CSS px`다. 비교기는 inventory의 예상 분류와 진단 문자열, frame 개수 및 중복 node ID도 확인한다.
- 이 **초기 matrix 실행**은 각 scene을 Android surface에 제출하거나 장면별 GPU pixel을 읽지 않았다. 함께 저장한 화면은 기본 10-node GPU scene과 matrix의 `26/26` 상태를 보여준다. Android AVD adapter는 `Gl`이며 ANGLE/SwiftShader software renderer다. 이는 하드웨어 GPU 성능 증거가 아니다. 당시 `row-reverse` 차이는 scene pixel oracle이 없어 미해결이었다. 이후 [Android `PixelCopy` 후속 실행](../c12-4-android-surface-pixels-2026-10-11/run-c124-20261011T100934321Z-68d7a5c4/comparison.json)에서 21개 지원 scene·22개 sample을 다시 비교했고, reverse scene을 포함해 모두 통과했다.
- 2026-10-11 16:56 KST에 painter 예약 실패 주입·깊이 스트레스와 Android matrix 진입 extra 수정 뒤 APK를 다시 빌드해 API 37.1 AVD에서 matrix를 재실행했다. `SPINON_C124_MATRIX_SUMMARY status=matched cases=26 nodes=77 expected_ready=21 expected_rejected=5 matched=26`으로 끝났다. [재실행 화면](./android-api37-emulator-matrix-reserve-rerun-2026-10-11.png)과 [장면별 원본 로그](./android-api37-emulator-matrix-reserve-rerun-2026-10-11.log)를 보관했다. matrix는 계속 runtime snapshot 검증이며 Android surface pixel 검증은 아니다.
- 2026-10-11 17:15 KST에 Android 실기기 없이 `emulator-5554` API 37 AVD에서 matrix를 다시 실행했다. Android 17, arm64-v8a, 16 KiB page size였으며 연결 device 목록에는 이 AVD만 있었다. `status=matched cases=26 nodes=77 expected_ready=21 expected_rejected=5 matched=26`을 확인했다. [최신 재실행 화면](./android-api37-emulator-matrix-deep-graph-rerun-2026-10-11.png)과 [장면별 로그](./android-api37-emulator-matrix-deep-graph-rerun-2026-10-11.log)를 보관했다. 이 matrix는 실제 scene별 surface pixel 또는 deep graph 실행이 아니다.
- [실제 문서 그래프 깊이 stress](../c12-4-document-graph-stress-2026-10-11.md)에서 HostDocument→Stylo cascade→Taffy→paint snapshot 전체 경로를 128 nodes/default test stack과 1,024 nodes/32 MiB test stack에서 통과했다. 일반 test thread에서 depth 256을 실행했을 때 cascade 뒤 layout 호출 중 process stack overflow가 발생했다. Android/iOS runtime thread 안전 깊이는 확인하지 않았고 전체 graph stress는 미완료다.

## 독립 GPU 픽셀 확인

별도 macOS headless WGPU 통합 테스트가 Stylo cascade → Taffy layout → C12.4 paint list → WGPU readback 경로를 실행했다. 네 표본이 정확히 다음 색과 일치했다.

| 겹침 표본 | RGBA |
| --- | --- |
| cyan positioned child | `[56, 189, 248, 255]` |
| 바깥 `z-index:2` green | `[34, 170, 102, 255]` |
| static Flex item `z-index:2` red | `[229, 72, 77, 255]` |
| root background | `[16, 24, 39, 255]` |

실행 명령은 `cargo test --locked -p spinon-render-wgpu --test runtime_stacking -- --nocapture`이다. 이 결과는 macOS headless WGPU 경로의 offscreen 표본이다. Android/iOS surface의 픽셀 readback 결과로 확대하지 않는다.

## 검증 결과

- 변경 후 `cargo test --locked --workspace` 통과.
- 변경 후 `cargo clippy --locked --workspace --all-targets -- -D warnings` 통과.
- `cargo fmt --all -- --check`, `git diff --check` 통과.
- `mise exec -- bun run test:css-reference`: 170개 통과.
- Android Debug APK 빌드와 이전 Android 실기기 실행 통과. 빌드에서 AGP 8.13.2와 compile SDK 37.2 조합이 검증되지 않았다는 경고가 있었지만 APK assembly는 성공했다.
- Android Debug APK 재빌드와 API 37.1 Android Simulator 실행 통과. 시뮬레이터의 WGPU backend는 `Gl`, adapter는 ANGLE/SwiftShader 소프트웨어 경로였다.
- 수정 APK의 Android API 37.1 AVD matrix 및 comparator 통과: 26개 outcome, 77개 source node, 5개 진단 거부, 지원 fixture node 64개 frame을 비교했다. 26개 scene별 surface pixel은 비교하지 않았다.
- painter의 명시적 `Vec::try_reserve` capacity-growth 지점마다 실패를 주입해 `AllocationFailed { stage }` 전파와 stage 식별을 검증했다. 0개 요청·남은 capacity 요청은 주입 대상에서 제외하는 대조도 통과했다.
- 합성 paint-context 그래프 50,000단계 깊이와 50,000개 형제 폭을 flatten하고 전체 NodeId 순서를 비교했다. 이는 renderer context flatten 단계만 검증한다.
- 실제 HostDocument→Stylo→Taffy→paint snapshot은 128-node/default stack 및 1,024-node/32 MiB stack에서 count·최심부 frame·paint order를 확인했다. depth 256의 default test thread overflow가 나와 safe product-depth 경계는 미확정이다. 자세한 조건은 [깊은 그래프 기록](../c12-4-document-graph-stress-2026-10-11.md)을 따른다.
- 새 APK 설치 후 Android 17/API 37.1 AVD에서 matrix 26/26을 다시 확인했다. AVD 그래픽 adapter는 ANGLE/SwiftShader software renderer다.
- 진단 매핑 수정 뒤 전체 `cargo test --locked --workspace` 및 `cargo clippy --locked --workspace --all-targets -- -D warnings`가 통과했다. 집중 런타임 test는 unit 133 passed / 2 ignored, priority integration 3 passed, lifecycle integration 16 passed였다. CSS reference 검사는 170 passed.
- iOS Simulator 빌드와 실행 통과. Xcode 링커가 기존 Rust 정적 archive 사이 `raw_window_metal` 중복 심볼 경고를 출력했으나 빌드와 실행은 성공했다.
- 15:09 재빌드에서도 같은 `raw_window_metal` 중복 심볼 경고가 있었지만 `BUILD SUCCEEDED` 후 앱 실행 및 fixture 평가가 성공했다.

## 남은 검증 경계

기존 10-node fixture는 Android 실기기·Android/iOS Simulator에서 화면 제출까지 확인했다. Android API 37 AVD runtime matrix는 26 scene·77 source node 중 21 ready·5 expected rejection이었고, 지원 node 64개 frame의 pinned Chrome 대비 최대 오차는 0 CSS px였다. Android `PixelCopy`는 21개 지원 scene의 `SurfaceView`에서 22/22 표본, 최대 channel error 0으로 통과했다. iOS 26.2 Simulator에서는 실제 WGPU Metal surface texture에서 reverse Flex 방향·order 8개 표본을 읽어 8/8 exact match했다. 별도 iOS runtime matrix에서도 같은 26 scene·77 source node를 실행해 21 ready·5 expected rejection, 지원 node 64개 frame 최대 오차 0 CSS px를 확인했다. 따라서 Metal pixel 표본과 iOS 전체 geometry matrix는 각각 완료했지만, 전체 iOS scene별 surface pixel을 비교한 것은 아니다. reverse Flex는 pinned Chrome 154 호환 예외이며 W3C Flexbox 설명과 다르다. AVD는 ANGLE/SwiftShader software renderer다. Painter `Vec::try_reserve` capacity-growth 오류 주입과 합성 context flatten 50,000단계 깊이·50,000개 형제 검증은 완료했다. 실제 HostDocument→Stylo→Taffy→paint 경로는 Android AVD와 iOS Simulator worker에서 1,024 depth 성공·1,025 거부·새 revision 복구를 확인했다. 기본 test stack의 256 depth overflow 관측도 유지한다. iOS Simulator·Android AVD의 0·1·2·4·8 동시 session memory를 세 표본씩 측정했으나 process 전체 메모리이므로 CSS worker stack으로 귀속하지 않는다. Android `/proc/maps`·`smaps_rollup` 권한 거부, iOS `vmmap` DYLD 조회 실패는 보존했고 가능한 PSS/RSS/footprint 표본만 사용했다. 전역 allocator OOM, 실기기 stack/RSS, Spinon WPT 적합성은 미확정이다. 고정 WPT 후보 4개 자동·1개 수동 실행은 Chrome 154 기준 의미만 확인했으며 Spinon WPT 적합성 결과가 아니다. 이 근거만으로 CSS 전체 stacking 지원 또는 C12.4 완료를 표시하지 않는다.

## 원본 근거 파일

- Android 실기기 화면: [android-sm-s731n-android16-vulkan.png](./android-sm-s731n-android16-vulkan.png)
- Android logcat: [android-sm-s731n-android16-vulkan.log](./android-sm-s731n-android16-vulkan.log)
- Android Simulator 화면: [android-api37-emulator-2026-10-11.png](./android-api37-emulator-2026-10-11.png)
- Android Simulator logcat: [android-api37-emulator-2026-10-11.log](./android-api37-emulator-2026-10-11.log)
- Android API 37.1 AVD 26-scene matrix: [화면](./android-api37-emulator-matrix-2026-10-11.png) · [원본 logcat](./android-api37-emulator-matrix-2026-10-11.log) · [Chrome geometry 비교 JSON](./android-api37-emulator-matrix-2026-10-11.json)
- 예약 실패·스트레스 변경 뒤 Android AVD matrix 재실행: [화면](./android-api37-emulator-matrix-reserve-rerun-2026-10-11.png) · [장면별 로그](./android-api37-emulator-matrix-reserve-rerun-2026-10-11.log)
- 실기기 연결 해제 후 Android API 37 AVD matrix 재실행: [화면](./android-api37-emulator-matrix-deep-graph-rerun-2026-10-11.png) · [장면별 로그](./android-api37-emulator-matrix-deep-graph-rerun-2026-10-11.log)
- iOS Simulator 화면: [ios-26.2-iphone-17-pro-simulator.png](./ios-26.2-iphone-17-pro-simulator.png)
- iOS Simulator unified log: [ios-simulator.log](./ios-simulator.log)
- 최신 iOS Simulator 재실행 화면·로그: [PNG](./ios-simulator-rerun-2026-10-11.png) · [로그](./ios-simulator-rerun-2026-10-11.log)
- macOS WGPU readback log: [wgpu-readback.log](./wgpu-readback.log)

## 후속 iOS surface·session 측정

이 README의 앞선 날짜별 기록은 해당 시점의 사실로 유지한다. 후속으로 iOS Metal surface 8-case 비교, iOS 26-scene geometry matrix, Android/iOS Simulator 동시 session memory 표본을 완료했다. 상세 자료는 [iOS Metal 비교 계획](../../../../plan/c12-4-ios-metal-surface-pixels.md), [원본 iOS pixel JSON](../c12-4-ios-metal-surface-pixels-2026-10-11/run-c124-ios-20261011T105945959Z-ce5b401c/comparison.json), [iOS runtime matrix 비교 JSON](../c12-4-ios-runtime-matrix-2026-10-11/run-c124-ios-matrix-20261011T114910232Z-7f5abd20/comparison.json), [동시 session 측정](../c12-4-concurrent-session-stack-cost-2026-10-11.md)에 있다. 전체 iOS scene별 surface pixel, 실기기/thread별 memory 귀속, Spinon WPT 적합성은 여전히 미확정이다.
