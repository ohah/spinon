# C13.2 구현 PR 변경 관점 30개 검토

- **대상:** C13.2 구현 PR 전체 diff: Rust runtime/layout/render, opt-in FFI fixture, Android/iOS Simulator route, build scripts, fixture, status/spec/evidence
- **계획 PR 검토와의 관계:** C13.2 계획 PR의 기존 30개 표와 분리했다. 이 표는 구현 변경 조합에서 생길 수 있는 계약·통합 실패를 검토한다.
- **검증:** workspace 797 passed / 0 failed / 4 ignored, workspace Clippy, rustfmt, `git diff --check`, Chrome reference 5/5, fixture-boundary 5/5, API 37 ARM64 AVD build/run, iOS 26.2 Simulator build/run
- **상태:** 미해결 P0/P1 없음. Simulator 및 CSS coverage의 제한은 PR 설명에 그대로 명시한다.

| # | PR 변경 조합에서 볼 실패 경로 | 확인 결과 |
| --- | --- | --- |
| 1 | C13.2 내부 계약 ID가 C13.1/U05 계약을 덮어쓰거나 공개 완료 계약처럼 올라간다. | `0062`의 draft 버전과 C13.2 내부 fixture 경계를 유지하고 상위 C13 완료로 표시하지 않는다. |
| 2 | plan·internal contract·STATUS가 서로 다른 구현 상태를 말한다. | 세 문서 모두 제한 구현/부분 Simulator 확인으로 맞추고 남은 앱 경계를 명시했다. |
| 3 | scroll container state가 기존 `RuntimeRenderKey` 필드를 대체해 document/style/environment revision을 잃는다. | 새 `ScrollStateRevision` builder를 연결하되 이전 key 축을 그대로 보존한다. Rust key 독립성 test가 있다. |
| 4 | static/다른 runtime profile에서도 새 scroll traversal이 켜진다. | scroll geometry는 지원 profile에서만 만들고 다른 profile의 offset request는 `UnsupportedRuntimeProfile`로 거부한다. |
| 5 | scroll request 타입이 좌표 단위나 내부 용도를 밝히지 않아 public CSSOM 계약으로 오해된다. | `RuntimeScrollOffsetRequest`는 CSS px와 내부 fixture request로 문서화되어 있고 public DOM API 완료를 주장하지 않는다. |
| 6 | `RuntimeScrollContainer` 생성이 range, offset, 크기 비유한 값을 받아들인다. | 생성자에서 양 축의 scrollable/client 관계와 유한 offset을 검증하며 duplicate owner snapshot도 거부한다. |
| 7 | 기존 `RuntimeRenderSnapshot::new` 호출부가 새로운 metadata 요구로 깨진다. | 기존 생성자와 `new_with_scroll_containers`를 호환 wrapper로 유지하고 기존 render tests를 포함한 workspace가 통과했다. |
| 8 | scroll 이동 때 paint frame은 바뀌고 hit frame은 이전 위치에 남는다. | 하나의 `apply_scroll_state` 결과가 paint/hit에 쓰인다. moved marker와 border escape NodeId를 양 플랫폼 로그에서 확인했다. |
| 9 | clip이 scroll content와 같이 움직여 scrollport 외부를 새로 드러낸다. | clip마다 바깥 scroll ancestor만 기록하고 자기 owner offset에서는 고정한다. nested snapshot test와 simulator marker assertion이 통과했다. |
| 10 | nested offset 순서를 뒤집거나 두 owner를 하나로 합친다. | ancestor 목록을 노드 lineage에서 만들고 각 owner state를 독립 유지한다. 단위 test가 outer/inner clip을 분리한다. |
| 11 | renderer module 분리 뒤 public export 또는 private helper 가시성이 깨진다. | `runtime/scroll.rs`, `runtime/snapshot.rs`가 runtime module 아래 역할별로 분리됐고 FFI/app builds 및 workspace tests가 통과했다. |
| 12 | `stacking.rs` 분리 중 기존 clip/staking traversal 순서를 바꾼다. | 기존 builder 경로를 유지하고 scroll geometry만 `stacking/scroll/geometry.rs`로 추출했다. 전체 workspace tests와 C13.1 boundary regression이 통과했다. |
| 13 | `overflow` 값이 Taffy로 전달되면서 `scroll`/`auto`가 `hidden`으로 합쳐진다. | layout projection이 다섯 typed value를 보존하고 projection test에서 각각 대조한다. |
| 14 | 새 error variant가 호출자·diagnostic에서 구분되지 않는다. | stale, duplicate, invalid request, revision exhaustion을 별도 variant와 한국어 설명으로 추가했다. |
| 15 | FFI C symbol이 fixture feature 없이 export된다. | 모듈·host method·header declaration이 `c13-2-scroll-fixture-test`로 보호되고 default archive 검사에서 symbol이 없다. |
| 16 | fixture JS payload가 runtime 기본 binary에 문자열로 들어간다. | 두 JS source는 전용 fixture module의 `include_str!`로만 가져오며 default archive 문자열 검사에 `c132-root`가 없다. |
| 17 | Cargo feature가 필수 runtime dependency 없이 활성화되거나 다른 fixture feature를 실수로 끈다. | C13.2 feature는 기존 C04 runtime GPU fixture에만 연결하고 optional core dependency를 선언한다. |
| 18 | Android build env가 `2`, typo, C04 미설정이어도 성공한다. | build.gradle.kts와 shell wrapper가 0/1 및 C04 prerequisite를 검사하고 실패 메시지를 출력한다. |
| 19 | Android native library가 Rust feature와 JNI compile define이 다르게 빌드된다. | 같은 env flag를 Cargo feature와 C++ macro에 전달하고 AVD APK build/runtime route가 통과했다. |
| 20 | Android feature-off 실행이 null host를 조용히 성공으로 보고한다. | `MainActivity`가 disabled route를 fail-closed하고 JNI는 명시적인 C13.2 disabled log를 낸다. |
| 21 | Android fixture가 surface 크기 0에서 먼저 평가돼 새 surface presentation과 경쟁한다. | 실패 `-12` 로그를 surface callback 순서와 대조해 초기화를 positive-size `surfaceChanged` 뒤로 옮겼다. 수정 뒤 AVD에서 연속 통과했다. |
| 22 | Android intent extra가 여러 fixture route를 동시 선택하거나 C13.2 route를 못 고른다. | route selection/conflict guard에서 하나의 C13 route만 허용하며 `spinon_c132_scroll_fixture`로 재실행해 확인했다. |
| 23 | iOS Simulator launch argument와 Rust/C bridge function이 다른 이름을 쓴다. | `--spinon-c132-scroll`, `SpinonRunner` header/Objective-C++ wrapper/Swift callsite가 일치하고 app 실행 로그가 통과했다. |
| 24 | iOS native build가 feature env를 simulator target에 전달하지 않아 빈 FFI symbol을 연결한다. | `build-ios-sim.sh`가 Xcode 환경과 cargo feature를 함께 전달하고 link/build 및 Simulator route가 통과했다. |
| 25 | iOS device route를 Simulator 실행 근거로 잘못 기록한다. | 지정 iPhone 17 Pro / iOS 26.2 Simulator UDID, simulator drawable, app hash를 기록하고 실기기 실행은 사용하지 않았다. |
| 26 | Android 실기기 serial이 AVD 대신 선택된다. | 모든 adb 명령을 `-s emulator-5554`로 지정했다. 연결돼 있던 물리 기기는 설치·실행에 사용하지 않았다. |
| 27 | 두 simulator screenshot이 assertion log와 무관한 이전 binary에서 온다. | 최종 module fix 뒤 앱을 양쪽에서 재빌드·재설치·재실행하고 screenshot/log/app hash를 갱신했다. 화면 하단 initial summary와 post-lifecycle draw 차이를 문서에 분리했다. |
| 28 | screenshot을 픽셀 자동 대조나 실제 GPU/display 증거라고 과장한다. | PNG는 육안으로 border escape를 보여주며 pixel readback은 미실행이라고 명시한다. AVD는 SwiftShader다. |
| 29 | C13.2 task source가 허용된 fixture 디렉터리를 벗어나거나 feature-boundary check가 무력화된다. | 새 Rust/FFI modules와 fixture source 경계를 기존 JS test가 검사하며 5/5 통과한다. |
| 30 | PR 설명이 성공 지표만 남기고 unsupported/missing matrix를 감춘다. | PR 본문과 evidence에서 root/body app, `box-shadow`, pixel readback, app-level stale/fault injection 및 public CSSOM 범위를 분리해 기록한다. |

## 판정

이 변경은 내부 scroll state의 제한 구현으로 리뷰할 수 있다. C13.2의 전체 완료 표시는 아직 부적절하다. 병합 전에는 최신 PR checks, branch protection, mergeability를 다시 확인하고 저장소 규칙에 따라 `Rebase and merge`만 사용한다.
