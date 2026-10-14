# 0060 · C13.1 overflow clip runtime 내부 계약

**계약:** `0.1.0-draft` · **상태:** 제한 fixture profile 구현 중 · **제품 CSS 지원 선언:** 아님
**기준:** [C13 overflow·scroll 계획](../../plan/c13-overflow-scroll-clipping.md) · [C13.1 사전 비교](evidence/c13-1-overflow-clip-precomparison-2026-10-14/README.md)

## 범위

이 계약은 `RuntimeBlockFlexOverflowClipV1` 계산 profile에서 Stylo computed overflow 값을 layout adapter, immutable runtime render snapshot, WGPU 사각 geometry와 hit-test clip 입력으로 전달하는 내부 경계를 정한다. `CssClip` 같은 Rust 전달 타입이 crate 사이에서 공개되어 있어도 제품 CSS 지원 선언이나 C13 상위 기능을 뜻하지 않는다.

기존 13-node fixture는 HTML block·flow-root·position subset, 물리 horizontal-tb/LTR, 불투명 배경, 사각 clip만 포함한다. 별도 5-node fixture는 같은 제한 profile에서 불투명 단색 `solid` border-strip paint를 대조한다. 두 fixture는 다음 관찰을 분리한다.

- `overflow-x`와 `overflow-y`의 typed computed 값 및 mixed-axis 정규화
- `visible`의 자식 paint 연장과 `clip`의 축별 padding edge
- `overflow-clip-margin`의 `content-box`·`padding-box`·`border-box` visual box와 음수 값 fail-closed 경계
- 중첩 ancestor clip의 축별 교집합
- positioned containing block이 clip ancestor 바깥인 absolute 자식의 ancestor 예외
- paint clip과 hit-test clip의 서로 다른 결과
- static/absolute clip의 border-strip 색상과 paint/hit target 차이

## 내부 자료 흐름

1. `RuntimeBlockFlexOverflowClipV1` cascade는 Stylo의 `Overflow` 축과 `overflow-clip-margin`을 typed 값으로 snapshot에 복사한다. profile 밖의 CSS 문법은 이 입력 계약에 포함하지 않으며, 지원하지 않는 author declaration이나 stylesheet 진단은 runtime scene 생성 전에 실패한다.
2. `StyleLayoutOutput.layout.used_box_edges`는 Taffy가 계산한 네 면 padding/border used edge를 frame에 연결한다. clip edge는 이 used edge와 CSS px frame으로 구한다.
3. 각 render box의 DOM lineage를 자식부터 root까지 순회한다. ancestor clip ancestry는 bounded `try_reserve` 경계를 통과해야 한다. absolute descendant는 그 positioned containing block이 해당 clip ancestor 바깥이면 그 ancestor clip에서 빠진다. 현재 C12 profile의 fixed node도 element clip 대상이 아니다.
4. `RuntimeRenderBox`는 `paint_order`와 `hit_test_order`, `paint_clip_css_px`와 `hit_test_clip_css_px`를 각각 별도로 보존한다. C13.1 overflow profile만 DOM parent/child target 순서를 계산하며 다른 profile은 `hit_test_order == paint_order`를 유지한다. 기존 `.with_clip()`은 두 clip을 함께 설정하며 `.with_hit_test_clip()`은 hit-test 전용 값을 덮어쓴다. WGPU 배경 및 제한 border side quad는 paint 순서·clip을 사용하고 runtime hit query는 hit-test 순서·clip을 사용한다. `RuntimeRenderSnapshot`은 두 순서가 각각 유효한 순열인지 확인한다. 별도 border fixture는 opaque solid border만 표현하며 비단색 style과 부분 alpha 색은 거부한다.

## clip 계산

| computed 축 값 | paint clip | overflow-clip-margin | 내부 hit-test clip |
| --- | --- | --- | --- |
| `visible` | 해당 축 제한 없음 | 사용하지 않음 | 해당 축 제한 없음 |
| `clip`, static parent | 선택 visual box edge에서 offset만큼 바깥으로 확장한 사각 interval | 적용 | paint interval |
| `clip`, positioned parent의 in-flow descendant | 선택 visual box edge에서 offset만큼 바깥으로 확장한 사각 interval | 적용 | paint interval과 border-box의 합집합 interval |
| `clip`, positioned parent의 containing block에 속한 absolute descendant | 선택 visual box edge에서 offset만큼 바깥으로 확장한 사각 interval | 적용 | paint interval |
| `clip`, absolute descendant의 containing block이 해당 ancestor 바깥 | 해당 ancestor에서 clip하지 않음 | 해당 descendant 경로에 효과 없음 | 해당 ancestor에서 clip하지 않음 |
| `hidden`, `scroll`, `auto` | padding-box edge | 적용 효과 없음 | border-box interval |

이 표의 hit-test 행은 고정 Chromium 155 fixture 관찰을 이 내부 profile이 표현하는 방법이다. static/absolute border-strip 차이는 [controlled six-point fixture](evidence/c13-1-overflow-clip-paint-hit-border-2026-10-14/README.md)에서 확인했다. 해당 Chrome fixture에서는 positioned clip parent 배경이 단순 정적 in-flow descendant보다 먼저 paint되고 hit target은 별도 계산됐다. runtime은 auto `z-index` absolute/fixed clip parent의 descendant 전체가 static인 경우에 한해 부모를 그 subtree 앞에 배치한다. positioned 또는 z-index 후손이 있는 조합에는 이 재배치를 적용하지 않으며 별도 대조가 남아 있다. 이 제한 동작은 CSS painting 표준이나 전체 hit-test 표준 적합성을 뜻하지 않는다. `hidden`·`scroll`·`auto`는 이 단계에서 scroll container, scroll range, offset, scrollbar 또는 사용자 입력을 제공하지 않는다. 이 값의 clip paint 부분만 계산한다.

추가 Chrome 관찰에서는 positioned `clip` ancestor의 in-flow descendant가 border strip hit 범위를 유지했지만, 같은 ancestor가 containing block인 absolute descendant는 visual clip edge까지만 hit됐다. 따라서 규칙은 sibling 존재 여부가 아니라 현재 target의 ancestor lineage에 포함된 positioned containing block으로 선택한다. outer containing block을 쓰는 absolute descendant는 그 ancestor clip에서 제외한다. [10개 추가 matrix 행과 별도 content-box control](evidence/c13-1-overflow-clip-runtime-matrix-2026-10-14/README.md)은 범위가 다른 이전 matrix 세 행과 함께 Rust에서 축별 target hit interval까지 대조한다. 후속 [hit identity 비교](evidence/c13-1-overflow-clip-hit-identity-2026-10-14/README.md)는 13개 유효행의 target frame 안에서 Chrome topmost element를 exact NodeId로 확인한다.

축별 interval은 반열린 `[min, max)`이며, ancestor interval은 교집합으로 누적한다. `max < min`, 비유한 좌표, 누락된 frame/edge/style, 누락된 typed overflow 값은 partial snapshot 대신 구체 `StyleRenderError`로 거부한다. `overflow-clip-margin` offset은 유한한 0 이상 CSS px만 허용한다.

## fixture와 build 경계

Android/iOS의 고정 V8 fixture는 `c13-1-overflow-clip-fixture-test` opt-in Cargo feature에서만 제공한다. Android Gradle/JNI/Java route와 iOS build script/Xcode ObjC++ macro/Swift route가 같은 feature 상태를 사용해야 한다. C13.1 fixture flag는 C04 runtime GPU fixture를 요구하고 기본값은 off다. Cargo 기본 iOS FFI archive에는 C13.1 fixture symbol과 source payload가 없고 opt-in archive에는 들어가는 것을 matching LLVM 22 `llvm-nm` 및 `strings`로 확인했다. 이 검사는 정적 FFI archive를 대상으로 한다.

Chrome oracle은 agent-browser를 사용한다. 고정 15개 좌표의 document.elementFromPoint()와 runtime box hit query가 15/15 일치했다. 그중 viewport 밖 네 점은 Chrome, host 및 Android 앱 query에서 모두 null/None이었다. 같은 15 query는 Android API 37 ARM64 AVD의 실제 앱 fixture snapshot에서도 15/15 일치했다. Android 캡처는 AVD SurfaceView `PixelCopy`로 수행하며, 선택한 CSS px 중심을 density로 surface texel에 변환한다. screenshot 픽셀은 paint 증거다. 고정 point 비교는 DOM event dispatch와 전체 hit-test matrix를 증명하지 않는다. [앱 실행 근거](evidence/c13-1-overflow-clip-hit-test-android-avd-2026-10-14/README.md)와 [viewport negative-control 근거](evidence/c13-1-overflow-clip-hit-test-negative-controls-android-avd-2026-10-14/README.md).

## 현 검증과 남은 경계

[C13.1 Android AVD 근거](evidence/c13-1-overflow-clip-android-avd-2026-10-14/README.md)는 13개 node frame, Chrome 대조 13/13 geometry, 선택한 11개 표면 색 표본의 Android AVD/Chrome exact 일치와 agent-browser Chrome DOM query 대 host runtime query 당시 11/11 일치를 기록한다. [후속 앱 실행 근거](evidence/c13-1-overflow-clip-hit-test-android-avd-2026-10-14/README.md)는 실제 Android fixture snapshot의 원래 11 query와 PixelCopy 11/11을 기록한다. [negative-control 실행](evidence/c13-1-overflow-clip-hit-test-negative-controls-android-avd-2026-10-14/README.md)은 확장한 query 15/15와 null control 4/4를 기록한다. [iOS Simulator 통합](evidence/c13-1-overflow-clip-ios-simulator-2026-10-14/README.md)은 13/13 geometry와 15/15 hit-test를 기록한다. 이 캡처는 Metal surface 제출·Simulator 화면만 증거로 삼으며 drawable readback이나 compositor/scanout 결과를 뜻하지 않는다. Android AVD는 ANGLE·SwiftShader software renderer다. 어느 플랫폼 실행도 C13.1 완료 판정이나 제품 CSS 지원을 뜻하지 않는다.

고정 Chrome의 `hidden`, `clip`, `clip + flow-root` float 비교는 3-case 브라우저 관찰과 C13 runtime의 명시적 fail-closed 동작까지 확인했다. C13은 float geometry를 계산하지 않으며, Spinon의 float geometry parity는 C26 소유 구현 이후 비교한다. 이 경계 근거는 [float formatting 비교](evidence/c13-1-overflow-clip-float-boundary-2026-10-14/README.md)에 있다. C13.1의 radius/transform fail-closed는 inline clip ancestor·positioned descendant와 author stylesheet 경로 모두 runtime test로 확인했다 ([근거](evidence/c13-1-overflow-clip-unsupported-effects-2026-10-14/README.md); [구현 관점 20개](evidence/c13-1-overflow-clip-unsupported-effects-implementation-review-2026-10-14.md); [PR 변경 관점 30개](evidence/c13-1-overflow-clip-unsupported-effects-pr-review-2026-10-14.md)). [선택 Chrome matrix 근거](evidence/c13-1-overflow-clip-matrix-2026-10-14/README.md)는 clip-y·border-box clip margin·hidden margin-ignore 세 행의 Rust clip geometry와 정수 hit range를 대조한다. 13개 유효행 geometry/hit identity와 invalid margin·float의 fail-closed runtime 판정은 확인했다. 남은 C13.1 관문은 유효행 전체 surface 비교, target frame 바깥의 넓은 2D paint/hit controls, DOM event dispatch 및 WPT 실행이다. WPT 이름 범위 58개 source는 13개 Spinon fixture candidate, 17개 Chrome oracle-only, 28개 unsupported로 분류했으나 WPT를 실행하지 않았다([분류 기록](evidence/c13-1-overflow-clip-wpt-classification-2026-10-14/README.md)). 이번 [controlled border-strip 근거](evidence/c13-1-overflow-clip-paint-hit-border-2026-10-14/README.md)는 Chrome 여섯 점과 Rust workspace를 대조했다. 기존 iOS Simulator 앱 fixture는 13/13 geometry와 15/15 hit query를 유지했고, 별도 5-node route는 iOS Simulator와 Android API 37 ARM64 AVD에서 각각 5/5 frame·6/6 hit query·6/6 색 표본을 확인했다([iOS 근거](evidence/c13-1-overflow-clip-paint-hit-border-2026-10-14/ios-border-simulator-verification.txt), [Android 근거](evidence/c13-1-overflow-clip-border-android-avd-2026-10-14/README.md)). 같은 Android APK의 기존 route도 13 node, 15/15 hit, 11/11 PixelCopy로 유지했다. iOS Simulator 제한 fixture 실행은 [통합 근거](evidence/c13-1-overflow-clip-ios-simulator-2026-10-14/README.md)에 기록했으며 Metal drawable readback을 포함하지 않는다. clip ancestry 예약 실패 후 같은 CSS runtime worker가 layout 오류를 게시하고 다음 environment revision에서 fault-free 전체 snapshot을 게시하는 결정적 복구 검사는 [worker recovery 근거](evidence/c13-1-overflow-clip-worker-recovery-2026-10-14/README.md)에 있다. 기존 snapshot builder 단위의 같은 reserve 재사용 회귀도 유지한다. 이 검사는 OS allocator 전역 OOM이나 앱·플랫폼 allocator 동작을 입증하지 않는다. [기존 구현 실패 관점 검토 20개](evidence/c13-1-overflow-clip-implementation-review-2026-10-14.md), [worker recovery 구현 검토 20개](evidence/c13-1-overflow-clip-worker-recovery-implementation-review-2026-10-14.md), [hit-test query delta 검토 20개](evidence/c13-1-overflow-clip-hit-test-review-2026-10-14.md), [앱 hit-test delta 검토 20개](evidence/c13-1-overflow-clip-app-hit-test-review-2026-10-14.md), [viewport negative-control 검토 20개](evidence/c13-1-overflow-clip-hit-test-negative-controls-review-2026-10-14.md), [기존 PR 변경 관점 검토 30개](evidence/c13-1-overflow-clip-pr-review-2026-10-14.md), [worker recovery PR delta 검토 30개](evidence/c13-1-overflow-clip-worker-recovery-pr-review-2026-10-14.md)는 각각 별도 기록이다. C13.2 이후 scroll/CSSOM/input 기능과 public CSS 지원 선언은 이 계약 바깥이다.

float boundary 단계의 새 [구현 실패 관점 20개](evidence/c13-1-overflow-clip-float-boundary-implementation-review-2026-10-14.md)와 [PR 변경 관점 30개](evidence/c13-1-overflow-clip-float-boundary-pr-review-2026-10-14.md)는 기존 검토와 별도로 기록했다.

border-strip app 확장의 iOS [구현 실패 관점 20개](evidence/c13-1-overflow-clip-border-simulator-implementation-review-2026-10-14.md)와 [PR 변경 관점 30개](evidence/c13-1-overflow-clip-border-simulator-pr-review-2026-10-14.md)는 iOS Simulator route 및 archive controls를 대상으로 기록했다. 뒤이어 [Android AVD border paint 근거](evidence/c13-1-overflow-clip-border-android-avd-2026-10-14/README.md)는 별도 route에서 5/5 frame, 6/6 hit query와 6/6 PixelCopy 색을 고정 Chrome과 대조했다. Android 실행은 API 37 ARM64 AVD의 ANGLE/SwiftShader software renderer로 제한하며 실기기나 hardware GPU 결과로 확대하지 않는다. 해당 Android delta의 신규 [구현 실패 관점 20개](evidence/c13-1-overflow-clip-border-android-avd-2026-10-14/implementation-review-20.md)와 [PR 변경 관점 30개](evidence/c13-1-overflow-clip-border-android-avd-2026-10-14/pr-delta-review-30.md)를 별도로 기록했다. event dispatch, 복합 stacking descendants, radius/다른 면 색 corner join과 전체 C13.1 사전 비교 matrix는 남아 있다. WPT source 분류는 고정했지만 WPT 실행은 하지 않았다.
