# C11.1 Stylo→layout 연결 실행 검증 · 2026-10-12

**상태:** Stylo typed cascade와 HostDocument→Stylo→layout Rust adapter를 확인했다. C11 Grid는 V8/FFI 앱 runtime 및 WGPU surface에 연결되지 않았으므로 Android/iOS Grid 화면 검증은 아직 할 수 없다.

## Rust·JS 검증

| 명령 | 결과 | 원본 |
| --- | --- | --- |
| `mise exec -- cargo test --locked --workspace` | 592 passed, 0 failed, 3 ignored | [Rust 전체 로그](cargo-test-workspace.log) |
| `mise exec -- cargo clippy --locked --workspace --all-targets -- -D warnings` | 통과 | [Clippy 로그](cargo-clippy-workspace.log) |
| `mise exec -- cargo fmt --all -- --check` | 통과 | [rustfmt 로그](rustfmt.log) |
| `mise exec -- bun run test:css-reference` | 197 passed, 0 failed | [CSS reference 로그](css-reference-tests.log) |
| `mise exec -- bun run test:js` | 2 passed, 0 failed | [JS 로그](js-tests.log) |
| `mise exec -- bun run test:css-c11-grid-precomparison` | 4 passed, 0 failed | [고정 Chrome reference 로그](c11-precomparison-tests.log) |

Rust 전체 시험에는 다음 C11 경계가 포함된다.

- layout-core의 typed Grid input, fixed·percentage·`fr`·gap, row auto-placement, source order, unsupported flow/placement/order/position, implicit-cell 부재와 10,000-track 거부.
- Stylo cascade의 typed `px`·percentage·`fr` tracks, author stylesheet cascade, unsupported track/placement 진단 및 track reservation 실패 문맥.
- HostDocument→Stylo→typed Grid→Taffy 연결, pinned Chrome `fixed-tracks-gap` scene의 5 node geometry, definite percentage basis, revision 보존, hidden subtree, root identity mismatch 거부와 allocation 실패 경계.

Chrome 고정 reference는 10 scene·40 node다. 하지만 Spinon Rust adapter가 직접 geometry로 대조한 장면은 `fixed-tracks-gap` 한 개·5 node뿐이다. 10 scene 전체가 Spinon에서 실행됐다는 의미가 아니다. pinned reference inventory 검사는 [비교 전 기준 로그](../c11-grid-precomparison-2026-10-11/README.md)를 참조한다.

## Android 실기기 확인과 C11의 현재 경계

실기기 연결을 사용해 현재 APK의 C12.4 기존 runtime 경로를 다시 확인했다. 이는 기기에서 실측을 수행했다는 증거이며 C11 Grid 실행 결과는 아니다.

| 실기기 실행 | 결과 | 기록 |
| --- | --- | --- |
| C12.4 runtime matrix | 26 scene·77 node, 21 지원·5 기대 거부; 상태 `matched` | [run `c124-android-20261011T165616037Z`](../c12-4-android-physical-device-2026-10-12/c124-android-20261011T165616037Z/README.md) |
| SurfaceView PixelCopy | 21/21 scene·22/22 sample, channel 오차 0 | [캡처·요약](../c12-4-android-surface-pixels-2026-10-12/run-c124-20261011T165625558Z-5c5a9db9/README.md) · [비교 JSON](../c12-4-android-surface-pixels-2026-10-12/run-c124-20261011T165625558Z-5c5a9db9/comparison.json) |
| 실제 worker 깊이·복구 | 1,024 성공·1,025 거부·새 revision 복구 | [run `c124-depth-android-20261011T165654410Z`](../c12-4-android-physical-device-2026-10-12/c124-depth-android-20261011T165654410Z/README.md) |

세 실행은 Samsung SM-S731N, Android 16/API 36에서 수행했다. 런타임 로그는 Vulkan `Samsung Xclipse 940`, APK SHA-256 `d3b87f2cf67389e27386f316b3b84a9775a458c9c85f84fa1e855fc139803090`를 기록한다. 이 재검증은 해당 기기·빌드의 관측이며 다른 기기·OS 성능을 일반화하지 않는다.

**C11 Grid를 실기기에서 아직 실행하지 않은 이유는 기기 부재가 아니다.** 현재 Android/iOS FFI 및 runtime session에는 `RuntimeBlockFlexGridV1` 호출 경로가 없고 기존 mobile runtime은 `display:grid`를 지원하지 않는 값으로 거부한다. 앱 launch fixture에서 Grid를 성공으로 강제하면 검증 대상 경로를 건너뛰게 된다. 다음 선행 구현은 runtime/FFI Grid profile entry와 그 입력·revision·실패 진단을 연결하는 것이다. 그 후 동일한 Chrome scene을 Android 실기기와 iOS Simulator에서 실행해 geometry와 surface pixels를 비교해야 한다.

## 남은 경계

- C11.1 mobile V8→Stylo→Taffy→WGPU scene 및 Android/iOS pixel test.
- Rust adapter의 직접 Chrome comparison을 나머지 pinned scene으로 넓히는 fixture/comparator.
- 전체 Spinon WPT runner. 현재 Chrome 154에서 통과한 WPT candidates는 browser oracle 결과이며 Spinon 적합성 판정이 아니다.
- C11.2 이후 explicit placement, implicit track, repeat/named line/area, intrinsic track sizing, alignment와 writing-mode 상호작용.

상세 코드 실패 경계는 [20개 관점 adapter 검토](../c11-style-grid-projection-implementation-review-2026-10-12.md), 로드맵은 [C11 전용 계획](../../../../plan/c11-css-grid.md), 내부 연결 계약은 [0059](../../0059-c11-grid-layout.md)를 참조한다.
