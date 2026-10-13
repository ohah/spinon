# 0059 · C11 Grid 레이아웃 내부 계약

**문서 ID:** `0059` · **내부 숫자 버전:** `0.1.0` 고정 · **상태:** C11.1·C11.2.1은 `origin/main`에 통합; C11.2.2와 C11.2.3 제한 profile 및 지정 검증 완료 · **Android:** C11.2.2·C11.2.3 SM-S731N 실기기 geometry/PixelCopy 행렬 통과 · **iOS:** C11.2.2·C11.2.3 iPhone 17 Pro/iOS 26.2 Simulator geometry/Metal drawable 행렬 통과(compositor 이전) · **미완료:** Spinon WPT 적합성, 전역 OOM 복구, 전체 CSS Grid 지원, iOS 실기기·compositor 이후 픽셀 · **공개 API:** 아님 · **제품 CSS 지원:** 미선언

이 계약은 Stylo의 computed Grid CSS를 `spinon-layout` 입력으로 변환하고 Taffy Grid 계산 후보에 전달하는 Rust 내부 경계를 정의한다. `compute_runtime_block_flex_grid_style_layout` 경로는 HostDocument snapshot, 같은 mount-root의 Stylo view, typed cascade, style projection과 Taffy layout을 연결한다. Android/iOS 앱의 C11.1 고정 fixture도 이 공통 Rust 경로를 V8 runtime host·FFI와 연결한다. C11.1 Android 실기기와 iOS Simulator geometry는 각각 10 scene·40 node에서 통과했고, Android 고정 장면 표본은 7/7 exact였다. C11.2.1의 정확한 구현 후보 tree `fb9411cb7cc7f767699447d29a202a8fadca1ece`는 Android 16/API 36 SM-S731N과 iPhone 17 Pro/iOS 26.2 Simulator의 10 scene을 각각 실행했다. 양쪽 모두 성공 6/6·기대 거부 4/4, 21-node frame, geometry 최대 오차 0 CSS px, 성공 scene surface 표본 20/20 exact였다. Android 값은 실기기 PixelCopy, iOS 값은 compositor 제출 전 Metal drawable readback이다. Android API 37 AVD의 별도 표면 행렬은 ANGLE/SwiftShader 소프트웨어 경로이며 실기기 GPU 결과가 아니다. C11.2.1은 현재 main에 통합되어 있다. 공식 Spinon WPT runner·적합성, iOS 실기기와 compositor 이후 pixel parity는 미완료다. 이 계약은 CSS parser, 전체 CSS Grid 지원 또는 공개 `@spinon/*` API를 제공하지 않는다. 제품 상태는 [공식 상태 대장](../STATUS.md), 작업 분해는 [C11 계획](../../plan/c11-css-grid.md)을 따른다.

## 책임 경계

- `spinon-style`은 동일 문서·style·environment revision에서 computed CSS와 `RuntimeBlockFlexGridV1` profile diagnostic을 소유한다. 현재 단계는 explicit track list의 definite `px`, percentage, 양의 유한 `fr`, row auto-flow, row/column gap을 typed snapshot에 담는다.
- `spinon-style-to-layout`은 supported computed values만 engine-neutral `GridContainerInput`으로 변환하고 existing `LayoutInput`에 붙인다. HostDocument snapshot과 Stylo view의 document/root identity를 확인한다. 알 수 없는 track sizing, placement, definite basis 부재는 node/property/value가 식별되는 오류로 반환한다.
- `spinon-layout`은 Block·Flex·Grid 알고리즘 선택, track sizing candidate, item placement, frame 생성, allocation/index 오류와 layout revision을 소유한다. Taffy `0.14.0`은 잠긴 구현 의존성이지 호환성 판정 권위가 아니다.
- `spinon-style-to-render`/V8 runtime/FFI 및 Android/iOS app path에는 제한된 C11.1 고정 fixture 진입점이 추가됐다. Rust FFI는 `spinon_runtime_gpu_host_new_c11_1_grid_fixture`와 `spinon_runtime_gpu_host_eval_c11_1_grid_fixture`를 제공한다. Android는 `nativeCreateC11GridHost`/`nativeEvalC11GridFixture`, iOS는 `createRuntimeGpuHostWithC11_1GridFixture`/`evalRuntimeGpuC11_1GridFixture`로 호출한다. 이 fixture를 GPU scene에 전달하는 것은 확인했지만 일반 사용자 API나 전체 Grid 지원은 아니다.
- Android/iOS platform layer는 별도 native layout engine을 실행하지 않고 공통 Rust snapshot을 WGPU surface로 제출한다. 현재 검증은 위의 고정 fixture에 한정된다.

## C11.1 앱 경로와 검증 경계

- Android 실기기 Samsung SM-S731N / Android 16·API 36 / Xclipse 940·Vulkan에서 pinned Chrome 154 기준 10 scene·40 node geometry를 검증했고 `fixed-tracks-gap` 장면의 실제 `SurfaceView` PixelCopy 표본은 7/7 exact다. [최신 전체 행렬·PixelCopy 결과](./evidence/c11-grid-android-physical-device-2026-10-12/README.md).
- Android API 37 AVD에서는 10 scene·36 sample PixelCopy 행렬이 exact였다. renderer는 ANGLE/SwiftShader 소프트웨어 경로이므로 실기기 GPU 결과로 일반화하지 않는다. [AVD/iOS 10-scene 표면 비교](./evidence/c11-1-mobile-surface-pixel-matrix-2026-10-12/README.md).
- iPhone 17 Pro / iOS 26.2 Simulator에서 고정 Chrome 154 reference 10 scene·40 node geometry와 별도 10-scene Metal drawable 행렬의 36/36 RGBA exact를 확인했다. 이는 simulator surface 결과이며 iPhone 실기기나 compositor 최종 출력의 color-management 결과가 아니다. [10-scene geometry](./evidence/c11-grid-ios-simulator-2026-10-12/c11-grid-ios-matrix-20261011T205619778Z-2032fc2e/README.md) · [10-scene 표면 비교](./evidence/c11-1-mobile-surface-pixel-matrix-2026-10-12/README.md) · [초기 Metal 표본](./evidence/c11-1-ios-metal-surface-pixels-2026-10-12/run-c11-ios-metal-20261011T205937099Z-a996f7ef/README.md).
- 두 플랫폼의 geometry 행렬은 각각 10 scene·40 node다. 표면 행렬은 Android API 37 AVD와 iOS Simulator에서 각각 10 scene·36 sample이며, Android 실기기 표면은 fixed scene 7 sample만 확인했다. 공식 Spinon WPT runner, Android 실기기 전체 pixel 행렬, iOS 실기기, 다른 기기 성능을 뜻하지 않는다. 전체 경계와 다음 관문은 [C11 계획](../../plan/c11-css-grid.md)을 따른다.

## C11.2.1 numeric explicit placement 구현 상태

C11.1의 all-auto row placement와 구분되는 `RuntimeBlockFlexGridPlacementV1` profile을 추가했다. Stylo computed longhand를 typed line/span 값으로 보존하고, `spinon-layout`에서 explicit track 경계와 line 변환을 검증한다. `RuntimeBlockFlexGridV1`의 기존 성공 의미는 유지한다. 상세 지원·거부 계약은 [C11.2.1 계획](../../plan/c11-2-1-explicit-grid-placement.md)을 따른다.

- Rust/Chrome style-to-layout fixture 9/9, runtime profile 3/3, layout-core placement 정규화 4개와 nested/hidden 경계 2개, FFI 인수 경계 1/1 및 full `bun run test`가 통과했다. 겹치는 item의 DOM paint snapshot 순서도 runtime test로 고정했다. 별도 profile을 쓰므로 C11.1의 auto-placement 동작을 C11.2.1 지원으로 확장해 세지 않는다. [구현 실패 관점 검토](./evidence/c11-2-1-grid-placement-implementation-review-2026-10-12.md).
- Android API 37 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 고정 `numeric-lines` V8 fixture를 각각 실행했다. root 포함 4개 CSS px frame이 Chrome 154와 최대 차이 0이며 WGPU surface present marker가 나왔다. Android AVD는 ANGLE/SwiftShader 소프트웨어 backend다.
- [이전 Android AVD/iOS Simulator 10-scene runtime 행렬](./evidence/c11-2-1-mobile-runtime-matrix-2026-10-12/README.md)은 성공 6/6·기대 거부 4/4, 21-node frame, geometry 오차 0 CSS px를 기록했다. AVD는 ANGLE/Vulkan/SwiftShader 소프트웨어 renderer다. [정확한 구현 후보의 전체 실행 요약](./evidence/c11-2-1-index-candidate-2026-10-13/README.md)은 Android SM-S731N 실기기와 iPhone 17 Pro/iOS 26.2 Simulator의 각각 10 scene, 성공 6/6·기대 거부 4/4, 20/20 surface sample exact 결과를 기록한다. 상세는 [Android 실기기 PixelCopy](./evidence/c11-2-1-grid-placement-android-runtime-matrix-2026-10-13/c1121-android-20261012T203140900Z-69e4ca98/README.md)와 [iOS Simulator Metal drawable](./evidence/c11-2-1-grid-placement-ios-simulator-runtime-matrix-2026-10-13/c1121-iossim-20261012T203042604Z-f7983af0/README.md)에 있다. WPT runner·Spinon 적합성, iOS 실기기와 compositor 이후 parity는 미완료다. C11.2.1 PR 통합과 C11 상위 상태는 미완료다.

## C11.2.3 dense·column auto-placement 제한 profile

`RuntimeBlockFlexGridAutoPlacementDenseV1`은 C11.2.2의 sparse row profile을 바꾸지 않고 `grid-auto-flow: row dense`, `column`, `column dense` 및 회귀용 `row`를 별도 내부 runtime 경로로 평가한다. 양의 implicit track 확장과 계획에서 고정한 auto-track sizing subset만 처리한다. typed computed `grid-auto-flow`, `grid-auto-rows`, `grid-auto-columns`를 cascade에서 받아 Rust placement 단계와 Taffy projection에 전달한다. 상세 허용값·거부값·계산 경계는 [C11.2.3 계획](../../plan/c11-2-3-dense-column-placement.md)에 고정한다.

- FFI entry point는 `spinon_runtime_gpu_host_new_c11_2_3_grid_auto_placement_fixture`와 `spinon_runtime_gpu_host_eval_c11_2_3_grid_auto_placement_fixture`다. scene source는 고정 25-scene runtime matrix에서 ID로만 선택한다.
- 고정 Chromium 154 기준 25 scene·107 node 중 구현 성공 19, 기대 거부 6이다. Android SM-S731N 실기기와 iPhone 17 Pro/iOS 26.2 Simulator 각각에서 같은 행렬이 통과했고 geometry 최대 오차는 0 CSS px, 표면 sample은 각 99/99 exact였다. Android 실행은 actual PixelCopy 크기와 runtime CSS viewport의 비율을 사용한다. [최신 Android](./evidence/c11-2-3-dense-column-placement-android-physical-runtime-matrix-2026-10-13/c1123-a-mv5yld3e-4ded3cc6/README.md) · [최신 iOS Simulator](./evidence/c11-2-3-dense-column-placement-ios-simulator-runtime-matrix-2026-10-13/c1123-i-mv5ymb0x-10eed1fb/README.md) · [표면 구현 검토](./evidence/c11-2-3-mobile-surface-pixels-implementation-review-2026-10-13.md).
- 실제 Vec 예약 지점 여섯 곳(`VisibleItemList`, `RowLockedCursor`, `ExpandedRows`, `ExpandedColumns`, `OccupancyRows`, `OccupancyIntervals`)에서 allocator 실패를 주입했다. 정확한 `GridAllocationFailure`와 같은 worker의 후속 계산 복구를 확인했다. 이는 전역 OOM 복구나 다른 collection의 할당 실패를 보장하지 않는다. [실행 근거](./evidence/c11-2-3-grid-allocation-failure-2026-10-13.md).
- 이 profile은 public API나 제품 CSS 지원 선언이 아니다. 제한 profile의 geometry·표면 pixel gate는 지정 Android 실기기/iOS Simulator에서 통과했다. Spinon WPT 적합성, 전체 Grid/CSS 지원, iOS 실기기와 compositor 이후 픽셀은 완료하지 않았다.

## 현재 제안 입력과 출력

`LayoutStyle.display`에 `Grid`가 추가될 때도 기존 값을 재해석하지 않는다. 새 Grid container input은 definite content-axis basis, ordered row/column track lists, row/column gap, auto-flow mode를 전달한다. 각 grid item은 source NodeId, optional explicit line/span placement, used margin/size/alignment 값을 받는다. 첫 구현은 다음 subset만 허용한다.

- `display:grid` block-level container
- definite axis에서 fixed length, resolved percentage, positive `fr` 명시 track
- 기본 `grid-auto-flow:row` auto placement와 row/column gap
- fixed-size, non-text in-flow grid items, `order:0`
- Block·Flex·Grid 간 중첩은 각 child algorithm과 동일 revision을 보존할 때만 허용

출력은 입력 `LayoutInputRevision`을 그대로 echo하는 NodeId별 CSS px border-box frame이다. 값의 비정상, owner/child 불일치, unsupported CSS, revision mismatch, overflow 또는 allocation failure가 있으면 전체 계산을 실패 처리한다. 이전 scene·0 크기·기본 Block layout으로 대체하지 않는다.

## Taffy 0.14.0 경계와 현재 layout-core

- `spinon-layout`은 `grid` feature와 custom-tree `LayoutGridContainer`를 연결한다. 엔진 중립 입력은 `GridContainerInput`·`GridItemInput`이며 현재 C11.1 typed track은 길이(px)·percentage·positive `fr`이다. layout-core와 HostDocument→Stylo→Taffy unit path는 각각 source-order, gap, definite percentage, unsupported flow/placement, stale/revision identity, root mismatch를 검사한다. pinned Chromium 직접 geometry 비교는 현재 fixed-track scene 1개·5 node에 한정된다. Taffy `Style`은 Grid named-line·area 버퍼까지 각 노드에 포함해 8 MiB worker의 1,024-depth debug 실행을 실패시켰다. 계산 트리에는 현재 profile이 소비하는 필드만 가진 `CompactTaffyStyle`을 저장하며, style 크기 회귀와 동일한 worker depth 시험을 통과했다. 이름·area·implicit track을 실제 지원할 때는 projection 입력과 테스트를 명시적으로 확장한다.
- Taffy `0.14.0` 내부 `MAX_GRID_TRACKS`는 축마다 10,000이며 explicit grid를 이 값에서 clamp한다. C11.1 adapter는 더 큰 explicit track list를 `GridTrackLimitExceeded`로 Taffy 호출 전에 거부한다. 이 제한 profile은 Taffy의 clamp 동작을 CSS 지원으로 승격하지 않는다. CSS Grid Level 1 §5.3은 UA-defined size limit와 grid area clamp를 허용하지만, 그 호환 동작은 아직 구현·Chromium 비교하지 않았다.
- 현재 코드에는 Stylo typed cascade와 Rust style→layout projection, 제한된 JS/FFI runtime publication, Android/iOS fixture entry point가 있다. Android 실기기 fixed scene 7/7, 별도 Android AVD 전체 scene 36/36, iOS Simulator 전체 scene 36/36 표면 표본이 exact였다. 이 제한 fixture 통과는 일반 사용자 API 또는 공개 CSS 지원 완료가 아니다.
- 현재 통합 기준 실행: `bun run test`에서 Rust workspace 596 passed·3 ignored, CSS reference 302/302, JavaScript 2/2, Android touch tool 29/29이며 Clippy·rustfmt가 통과했다. 직접 pinned Chrome Rust geometry 비교 1 scene·5 node와 별도 모바일 10 scene·40 node 행렬은 서로 다른 경로다. 원본 로그와 지원 밖 입력 경계는 [10-scene 표면 비교](./evidence/c11-1-mobile-surface-pixel-matrix-2026-10-12/README.md), [Stylo→layout 실행 검증](./evidence/c11-style-grid-projection-2026-10-12/README.md), [adapter 구현 검토](./evidence/c11-style-grid-projection-implementation-review-2026-10-12.md)에 둔다.

## 오류와 지원 확장

- 제안 오류 분류: `UnsupportedGridProfile`, `UnsupportedGridTrack`, `UnsupportedGridPlacement`, `IndefiniteGridBasis`, `GridTrackExpansionOverflow`, `GridAllocationFailure`, `GridRevisionMismatch`.
- 오류에는 해당 HostDocument `NodeId`, property, computed value와 실패 단계가 포함되어야 한다. parser diagnostic과 adapter/layout error는 서로 변환해 숨기지 않는다.
- 내부 Rust enum·필드의 정확한 공개 형태는 C11.1 구현 전 비교 검토에서 확정한다. 계획 전용 타입을 API로 게시했다고 해석하지 않는다.
- Stage 확장으로 named lines, repeat, explicit/implicit placement, intrinsic sizing, alignment, writing mode, positioned items가 추가될 때마다 별도 단계·fixture·review를 만든다. 미출시 숫자 버전은 사용자가 호환성 정책을 정하기 전까지 올리지 않는다.

## C11.2.2 sparse row auto-placement 구현 상태

C11.2.2는 `RuntimeBlockFlexGridAutoPlacementV1` 제한 profile로 CSS 기본 `grid-auto-flow: row` sparse 배치, definite·auto item 혼합, 양의 implicit row와 단일 px `grid-auto-rows`를 처리한다. C11.2.1 profile의 성공 의미를 바꾸지 않으며 dense/column flow, implicit column, 반복 auto-row sizing 및 전체 Grid 지원을 제공하지 않는다. 상세 허용·거부 입력은 [C11.2.2 계획](../../plan/c11-2-2-sparse-row-placement.md)에 고정한다.

- 다섯 명시적 `Vec::try_reserve[_exact]` 실패 지점에서 실제 test allocator 실패, 정확한 `GridAllocationFailure` node/property, 결과 원자성, 같은 worker·engine instance의 복구를 검사했다. 이는 전역 OOM, `BTreeMap`·`BTreeSet` allocation 또는 OS allocator failure 처리를 뜻하지 않는다. [실행 근거](./evidence/c11-2-2-grid-allocation-failure-2026-10-13/README.md) · [allocator 구현 검토](./evidence/c11-2-2-grid-allocation-failure-implementation-review-2026-10-13.md).
- 검증한 실행은 Android 17/API 37 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 16 scene·69 node, 11 성공·5 기대 거부, geometry 오차 0 CSS px로 통과했다. Android PixelCopy와 iOS Metal drawable에서 각 11 scene·51 sample exact다. AVD는 ANGLE/SwiftShader이며 iOS readback은 OS compositor 제출 전이다. [Android AVD 행렬](./evidence/c11-2-2-sparse-row-placement-android-runtime-matrix-2026-10-13/c1122-avd-mv61a1sd-32a80873/README.md) · [iOS Simulator 행렬](./evidence/c11-2-2-sparse-row-placement-ios-simulator-runtime-matrix-2026-10-13/c1122-i-mv61i5ir-95971d40/README.md).
- Rust workspace, targeted CSS fixture tests, Android/iOS simulator builds와 scene 행렬이 통과했다. Chrome 154는 고정 subset의 oracle이다. [전용 범위 구현 검토](./evidence/c11-2-2-pr-candidate-implementation-review-2026-10-13.md)를 대조했다. Spinon WPT runner·적합성, 전체 CSS Grid, iOS 실기기와 compositor 이후 parity는 미완료다.

## 기준 자료

- [CSS Grid Layout Module Level 1 · 2025-03-26 CRD](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)
- [Taffy `0.14.0` · `compute_grid_layout`](https://docs.rs/taffy/0.14.0/taffy/fn.compute_grid_layout.html)
- [Taffy `0.14.0` · `LayoutGridContainer`](https://docs.rs/taffy/0.14.0/taffy/tree/trait.LayoutGridContainer.html)
- [C11 구현 계획](../../plan/c11-css-grid.md)
