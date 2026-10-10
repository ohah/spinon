# C10.3.2 구현 실패 관점 검토

**대상:** C10.3.2 변경 diff, Chrome fixture/reference, Stylo→Taffy projection, layout tree, render list, FFI 및 플랫폼 연결
**검토일:** 2026-10-10 · **상태:** 코드·fixture·Android/iOS Simulator runtime 검증 완료 · 실기기/하드웨어 GPU·pixel diff 미검증

계획 검토 표를 재사용하지 않고 구현의 서로 다른 실패 경계를 확인했다. 아래 결과는 실행 근거와 정적 검토의 경계를 구분한다.

| # | 공격 관점 | 확인과 결과 |
| --- | --- | --- |
| 1 | `order` 초기값이 0이 아니거나 자식에게 상속되는가 | Chrome reference와 여섯 profile 검사에서 default child는 0, 부모의 `order:-7`은 child frame 정렬에 전파되지 않는다. |
| 2 | 음수값이 기본 순서보다 뒤로 밀리는가 | `-1, 0, 2` 혼합 case에서 item frame을 Chrome과 대조했다. |
| 3 | 동일 값 정렬이 불안정해 매 실행마다 바뀌는가 | layout/paint에서 안정 정렬하고 source tie 순서를 fixture와 scene test로 확인했다. |
| 4 | 큰 양수 값이 signed int32 상한에서 overflow하는가 | Chrome Typed OM, Stylo profile, layout reference의 `2147483647` 경계를 비교했다. |
| 5 | 작은 음수 값에서 `-2147483648`을 양수로 뒤집는가 | lower clamp Typed OM과 Rust layout 전체 frame을 비교했다. |
| 6 | authored `order:1.5`가 유효 선언처럼 수용되는가 | invalid declaration은 무시되고 기존 유효값/default가 남는 Stylo 결과를 검사했다. |
| 7 | `calc()`의 양수 절반값을 잘못 반올림하는가 | `calc(1.5)`가 2, `calc(-1.5)`가 -1인 computed integer와 frame을 비교했다. |
| 8 | `var()`로 해석한 order가 typed layout에 도달하지 않는가 | custom property가 서로 다른 순서를 만드는 cascade·layout test와 고정 reference를 대조했다. |
| 9 | author stylesheet order가 inline-only 경로에서 누락되는가 | stylesheet selector cascade가 Taffy frame을 바꾸는 test를 실행했다. |
| 10 | 여섯 runtime Flex profile 중 일부에서만 값이 파싱되는가 | 각 profile의 computed `order`와 frame을 parameterized test로 확인했다. |
| 11 | Block 부모 자식도 `order`에 따라 바뀌는가 | non-Flex layout과 render scene은 원본 순서인 reference·회귀 test를 확인했다. |
| 12 | Flex 부모가 아닌 조상/이웃의 order를 잘못 합치는가 | sort는 Flex 부모의 직계 child 목록에만 적용하며 non-Flex render test가 통과했다. |
| 13 | 줄 수집 전이 아닌 뒤에 정렬해 줄 배치가 달라지는가 | wrap fixture에서 order가 line 수집 순서를 바꾼 전체 node geometry를 Chrome과 비교했다. |
| 14 | 정렬 뒤 grow 분배가 기존 item을 잘못 연결하는가 | order 변경 case의 모든 node frame과 grow 분배 geometry를 대조했다. |
| 15 | 중첩 컨테이너 item이 바깥 형제에 끼어드는가 | layout 좌표, scene preorder, 연속 paint rank에서 subtree scope를 확인했다. |
| 16 | Taffy postorder ID를 CSS child 순서와 혼동하는가 | reference node ID별 모든 frame field를 비교하고 source child ID 벡터가 그대로인지 검사했다. |
| 17 | CSS 변경 후 오래된 order/layout frame이 남는가 | inline order만 바꿔 full cascade·incremental cascade의 layout 결과를 비교하는 회귀 test를 추가했고 통과했다. |
| 18 | 겹치는 item의 paint list가 layout과 다른 정렬을 사용하는가 | scene node 순서·연속 paint rank test, 양 플랫폼의 실제 9-box WGPU 제출 로그와 겹침 screenshot을 확인했다. Chrome과 screenshot pixel-by-pixel 비교는 하지 않았다. |
| 19 | adapter/scene에 order 누락 또는 손상 값이 오면 조용히 0 처리하는가 | adapter 직접 fractional 값과 render list의 누락/비정수 computed order에 문맥 있는 오류를 기대하는 test를 추가했고 통과했다. |
| 20 | FFI·Android·iOS selector에서 잘못된 fixture나 호출 경계로 연결되는가 | Android/iOS 앱을 pinned V8로 빌드·실행했고 두 플랫폼 모두 status 0, 동일 9개 node frame, WGPU 9-box 제출을 남겼다. iOS linker의 `RawWindowMetalLayer` duplicate-symbol warning은 앱 실행 중 표면화되진 않았지만 별도 미해결 사항으로 기록했다. |

## 실행한 확인

- `cargo fmt --all -- --check`와 `git diff --check` 통과.
- `cargo test --locked --workspace` 통과. release 전용 benchmark 2개만 명시적으로 ignored 상태다. 새 inline mutation·malformed paint-order 검사도 전체 suite에 포함됐다.
- `cargo clippy --locked --workspace --all-targets -- -D warnings` 통과.
- `mise exec -- bun run test:css-reference` 통과: CSS reference 검사 117개, 실패 0.
- `mise exec -- node --test tools/css-reference/c10-3-flex-order-alignment.test.mjs` 통과: 7개. Android/iOS runtime log의 9개 frame 일치, 성공 marker, 9-box WGPU 제출과 Android SwiftShader backend를 검사한다.
- Rust 파일 구조 규칙에 맞춰 631줄이던 `style_values.rs`에서 Flex 값 해석과 spacing/math 해석을 각각 `projection/style_values/flex_values.rs`, `projection/style_values/spacing_values.rs`로 분리했다. 이 이동 뒤 포맷, 전체 workspace test, Clippy, CSS reference 117개와 C10.3.2 전용 7개 검사를 다시 실행해 모두 통과했다.
- `mise exec -- env ANDROID_HOME="$HOME/Library/Android/sdk" ANDROID_SDK_ROOT="$HOME/Library/Android/sdk" platforms/android/gradlew -p platforms/android -PspinonC04RuntimeGpu=1 :app:compileDebugJavaWithJavac -x :app:prepareSpinonBootstrap` 통과.
- `xcrun swiftc -frontend -parse platforms/ios/Sources/AppDelegate.swift platforms/ios/Sources/C0410RuntimeGpuDemo.swift` 및 iOS Simulator target Objective-C++ syntax-only 검사 통과.
- Android JNI source도 Android NDK `clang++ -fsyntax-only`로 검사했다.
- Android `tools/build-android-app.sh`와 iOS `tools/build-ios-sim.sh`가 모두 성공했다. 실행 로그는 Android API 37 emulator와 iPhone 17 Pro / iOS 26.2 Simulator의 V8 evaluation status 0, 동일 9개 frame 및 WGPU 9-box 제출을 보여 준다. 실제 build/runtime 로그와 캡처는 이 문서의 `c10-3-2-flex-order/`에 저장했다.
- 실제 앱 화면은 [Android screenshot](c10-3-2-flex-order/android-api37-emulator.png) 및 [iOS screenshot](c10-3-2-flex-order/ios-26.2-iphone-17-pro.png), 원시 실행 기록은 [Android log](c10-3-2-flex-order/android-api37-emulator.log) 및 [iOS log](c10-3-2-flex-order/ios-26.2-iphone-17-pro.log)에서 확인할 수 있다.
- 위 Android·iOS build log, runtime log와 screenshot은 parser 모듈 분리 뒤 다시 생성했다. 두 플랫폼의 V8 fixture가 status 0으로 끝나고 동일한 node frame 및 9개 WGPU box를 제출하는 것을 확인했다.
- Android renderer는 ANGLE을 통한 SwiftShader software backend다. 실기기와 hardware GPU 동작·성능으로 확대하지 않는다.
- iOS link에서 `RawWindowMetalLayer` static symbol이 `libspinon_wgpu_r08_spike.a`와 `libspinon_ffi.a` 양쪽에 들어 있다는 경고가 난다. 앱 빌드와 화면 제출은 성공했지만 이 링크 경고는 이 구현에서 수정하지 않았으며 후속 조사 대상이다. V8 debug-map timestamp 경고와 AppIntents metadata 생략 경고도 build log에 보존했다.
- 두 simulator screenshot은 사람이 확인했지만 Chrome과의 pixel-by-pixel raster equality는 수행하지 않았다.

## 발견하여 보완한 부분

초기 구현 검토에서 inline style 변경 후의 order 재계산과 render scene의 누락·손상 computed order에 대한 개별 실패 검사가 빠져 있었다. 해당 두 경계를 테스트로 추가했다. 새 runtime evidence 검사를 처음 실행했을 때 iOS와 Android draw marker의 field 순서가 다름을 발견해 parser를 수정했고 두 로그를 각각 다시 검사했다. 현재 남은 경계는 위에 적은 link warning과 pixel-by-pixel/실기기 검증이다.
