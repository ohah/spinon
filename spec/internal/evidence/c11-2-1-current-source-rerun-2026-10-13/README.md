# C11.2.1 현재 소스 재검증 · 2026-10-13

## 결론

incremental cascade에서 재사용되는 `display:none` 조상 문맥이 dirty descendant에 전달되지 않는 오류를 확인하고 수정했다. 회귀 테스트는 수정 전 실패하고 수정 후 통과했다. 수정 소스를 포함한 현재 작업 트리에서 저장소 테스트, Android AVD, Android 실기기, iOS Simulator 실행이 모두 기준에 맞았다.

C11.2.1 구현은 아직 별도 변경으로 통합되지 않았으므로 완료 처리하지 않는다. Android 실기기 10-scene runtime/PixelCopy 검증은 이번에 완료했다. Spinon WPT runner와 전체 WPT 적합성은 X16에서 별도 관리하며 C11.2.1 완료 관문으로 취급하지 않는다. C11 전체와 공개 CSS 지원도 미완료다.

## 발견한 회귀와 수정

incremental restyle의 context ancestor 재사용 분기에서 기존 computed style을 복사하면서 `display:none` 상태를 후손 순회 문맥에 되살리지 않았다. 그 상태에서 숨겨진 nested Grid 아래의 item 하나만 dirty가 되면, 숨김 조상 아래에서도 Grid placement projection이 실행될 수 있었다.

수정 위치: [`compute.rs`](../../../../crates/spinon-style/src/stylo_dom/cascade/incremental/compute.rs). 재사용된 조상 style의 computed `display`에서 `subtree_hidden`을 다시 계산해 자식 작업 항목에 전달한다. 새 회귀 테스트 [`runtime_grid_tests.rs`](../../../../crates/spinon-style/src/stylo_dom/cascade/runtime_grid_tests.rs)의 `incremental_restyle_preserves_hidden_context_for_unsupported_nested_grid`는 hidden wrapper 아래의 unsupported `repeat(auto-fit, 80px)` Grid와 dirty item을 조합한다. [`c11_2_1_grid_placement_tests.rs`](../../../../crates/spinon-style-to-layout/src/c11_2_1_grid_placement_tests.rs)의 `hidden_grid_item_ignores_unsupported_constraints_and_alignment`도 hidden item에서 범위 밖 placement·min/max·alignment 값이 보이는 Grid 계산을 실패시키지 않는지 확인한다.

Negative control에서 수정 분기를 제거하자 새 incremental test가 hidden item의 `grid_item_placement.is_none()` assertion에서 실패했다. 수정 후에는 같은 테스트가 통과했다. [수정 전 실패 로그](./incremental-hidden-context-negative-control.log)

## 저장소 검증

실행 로그는 현재 변경이 섞여 있는 dirty working tree에서 생성했으며, 특정 커밋이나 병합된 PR의 증거가 아니다. 기준 HEAD는 `975d3356c9e8837834d4dd633fb3699084fad71a`였다.

| 검사 | 결과 |
| --- | --- |
| Bun example test | 2/2 통과 |
| CSS reference | 341/341 통과 |
| Android 계측 도구 | 29/29 통과 |
| Rust workspace | 625 통과 · 4 ignored · 0 실패 |
| Release 최대 layout depth | 1/1 통과 |
| Clippy `--locked --workspace --all-targets -- -D warnings` | 통과 |
| `cargo fmt --all -- --check` | 통과 |
| `git diff --check` | 통과 |

전체 실행 로그: [full-test.log](./full-test.log) · [Clippy](./clippy.log) · [rustfmt](./rustfmt.log) · [diff check](./diff-check.log)

## 앱 runtime과 표면 픽셀

세 플랫폼 실행 모두 같은 fixture inventory SHA `b2801e708e4c1ad540e119dcdba04d7c215e4029d5ec5627683e15ede8ef368b`, HTML SHA `e9b080bbc65901b17f7dead60eb8071df8ecbefee2ea62f1318776848e75f78d`, Chrome 154 reference SHA `5bbbd041e2e59fda32c684f7290f61bff41ab6dda9ab9f77ce2fb36f0e0ee034`를 사용했다. 각 실행은 10 scene·21 node에서 성공 6/6, 기대 거부 4/4, geometry 최대 절대 오차 0 CSS px(허용 0.5)를 기록했다. 거부 장면은 예상 오류·partial frame 없음·non-empty WGPU 제출 없음으로 처리했다.

| 대상 | 결과 | 표면 표본 | 실행 근거 |
| --- | --- | --- | --- |
| Android API 37.1 AVD | 10/10 scene · geometry 오차 0 | PixelCopy 20/20 exact | [run `c1121-android-20261012T160420312Z-2114a1c3`](../c11-2-1-grid-placement-android-runtime-matrix-2026-10-13/c1121-android-20261012T160420312Z-2114a1c3/README.md) · ANGLE/Vulkan/SwiftShader 소프트웨어 renderer |
| Android 16/API 36 실기기 SM-S731N | 10/10 scene · geometry 오차 0 | PixelCopy 20/20 exact | [run `c1121-android-20261012T194810238Z-5c74d6b6`](../c11-2-1-grid-placement-android-runtime-matrix-2026-10-13/c1121-android-20261012T194810238Z-5c74d6b6/README.md) · Xclipse 940/Vulkan |
| iPhone 17 Pro/iOS 26.2 Simulator | 10/10 scene · geometry 오차 0 | Metal drawable 20/20 exact | [run `c1121-iossim-20261012T194925776Z-70f51679`](../c11-2-1-grid-placement-ios-simulator-runtime-matrix-2026-10-13/c1121-iossim-20261012T194925776Z-70f51679/README.md) · compositor 제출 전 readback |

Android 실기기 PixelCopy와 iOS Metal drawable은 각각 여섯 성공 scene의 표면 색상 표본을 비교했다. 이 결과는 WPT 적합성, 전체 CSS Grid 동작, 실제 표시 이후 compositor/scanout 일치 또는 성능 비교를 증명하지 않는다. iOS 실기기 실행도 포함하지 않는다.

## 2026-10-13 추가 플랫폼 재실행

Android 16/API 36 SM-S731N과 iPhone 17 Pro/iOS 26.2 Simulator에서 C11.2.1 10-scene matrix를 새로 빌드해 재실행했다. 각 대상은 성공 6/6·기대 거부 4/4, 21 node, geometry 최대 오차 0 CSS px, PixelCopy/Metal 표본 20/20 exact·최대 channel 오차 0이다. [Android 실행 `c1121-android-20261012T194810238Z-5c74d6b6`](../c11-2-1-grid-placement-android-runtime-matrix-2026-10-13/c1121-android-20261012T194810238Z-5c74d6b6/README.md) · [iOS 실행 `c1121-iossim-20261012T194925776Z-70f51679`](../c11-2-1-grid-placement-ios-simulator-runtime-matrix-2026-10-13/c1121-iossim-20261012T194925776Z-70f51679/README.md).

이 재실행은 C11.2.1 전용 matrix다. 빌드 당시 working tree에는 별도 미커밋 C12 변경도 함께 있었으므로, 특정 PR tree 단독 빌드나 C12 결과를 뜻하지 않는다. iOS readback은 compositor 제출 전 drawable이며 WPT 적합성·전체 Grid 지원·성능을 증명하지 않는다.

## 다음 상태

- Android 실기기 C11.2.1 전체 scene·PixelCopy 관문: 완료.
- iOS Simulator C11.2.1 전체 scene·Metal drawable 표본: 완료.
- C11.2.1 코드 통합/PR: 미완료.
- Spinon WPT runner와 WPT 적합성: X16 별도 추적, 미완료이며 C11.2.1 구현 선행 조건은 아님.
- C11.2.2 runtime 구현: C11.2.1 별도 통합 및 Rust·Chrome fixture 관문 뒤 시작.
- C11 전체 및 전체 CSS 지원: 미완료.
