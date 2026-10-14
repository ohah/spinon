# 0060 · C13.1 overflow clip runtime 내부 계약

**계약:** `0.1.0-draft` · **상태:** 제한 fixture profile 구현 중 · **제품 CSS 지원 선언:** 아님
**기준:** [C13 overflow·scroll 계획](../../plan/c13-overflow-scroll-clipping.md) · [C13.1 사전 비교](evidence/c13-1-overflow-clip-precomparison-2026-10-14/README.md)

## 범위

이 계약은 `RuntimeBlockFlexOverflowClipV1` 계산 profile에서 Stylo computed overflow 값을 layout adapter, immutable runtime render snapshot, WGPU 사각 geometry와 hit-test clip 입력으로 전달하는 내부 경계를 정한다. `CssClip` 같은 Rust 전달 타입이 crate 사이에서 공개되어 있어도 제품 CSS 지원 선언이나 C13 상위 기능을 뜻하지 않는다.

첫 fixture는 HTML block·flow-root·position subset, 물리 horizontal-tb/LTR, 불투명 배경, 사각 clip만 포함한다. runtime scene에는 13개 요소가 있고 다음 관찰을 분리한다.

- `overflow-x`와 `overflow-y`의 typed computed 값 및 mixed-axis 정규화
- `visible`의 자식 paint 연장과 `clip`의 축별 padding edge
- `overflow-clip-margin`의 `content-box`·`padding-box`·`border-box` visual box와 음수 값 fail-closed 경계
- 중첩 ancestor clip의 축별 교집합
- positioned containing block이 clip ancestor 바깥인 absolute 자식의 ancestor 예외
- paint clip과 hit-test clip의 서로 다른 결과

## 내부 자료 흐름

1. `RuntimeBlockFlexOverflowClipV1` cascade는 Stylo의 `Overflow` 축과 `overflow-clip-margin`을 typed 값으로 snapshot에 복사한다. profile 밖의 CSS 문법은 이 입력 계약에 포함하지 않으며, 지원하지 않는 author declaration이나 stylesheet 진단은 runtime scene 생성 전에 실패한다.
2. `StyleLayoutOutput.layout.used_box_edges`는 Taffy가 계산한 네 면 padding/border used edge를 frame에 연결한다. clip edge는 이 used edge와 CSS px frame으로 구한다.
3. 각 render box의 DOM lineage를 자식부터 root까지 순회한다. ancestor clip ancestry는 bounded `try_reserve` 경계를 통과해야 한다. absolute descendant는 그 positioned containing block이 해당 clip ancestor 바깥이면 그 ancestor clip에서 빠진다. 현재 C12 profile의 fixed node도 element clip 대상이 아니다.
4. `RuntimeRenderBox`는 `paint_clip_css_px`와 `hit_test_clip_css_px`를 별도로 보존한다. 기존 `.with_clip()`은 둘을 함께 설정하며 `.with_hit_test_clip()`은 hit-test 전용 값을 덮어쓴다. WGPU 사각형 clipping은 paint 값을 사용하고 runtime box hit query는 hit-test 값을 사용한다.

## clip 계산

| computed 축 값 | paint clip | overflow-clip-margin | 내부 hit-test clip |
| --- | --- | --- | --- |
| `visible` | 해당 축 제한 없음 | 사용하지 않음 | 해당 축 제한 없음 |
| `clip` | 선택 visual box edge에서 offset만큼 바깥으로 확장한 사각 interval | 적용 | paint interval과 border-box의 합집합 interval |
| `hidden`, `scroll`, `auto` | padding-box edge | 적용 효과 없음 | border-box interval |

이 표의 hit-test 행은 고정 Chromium 155 fixture 관찰을 이 내부 profile이 표현하는 방법이다. CSS painting 표준 요구를 뜻하지 않는다. `hidden`·`scroll`·`auto`는 이 단계에서 scroll container, scroll range, offset, scrollbar 또는 사용자 입력을 제공하지 않는다. 이 값의 clip paint 부분만 계산한다.

축별 interval은 반열린 `[min, max)`이며, ancestor interval은 교집합으로 누적한다. `max < min`, 비유한 좌표, 누락된 frame/edge/style, 누락된 typed overflow 값은 partial snapshot 대신 구체 `StyleRenderError`로 거부한다. `overflow-clip-margin` offset은 유한한 0 이상 CSS px만 허용한다.

## fixture와 build 경계

Android의 고정 V8 fixture는 `c13-1-overflow-clip-fixture-test` opt-in Cargo feature와 `SPINON_ENABLE_C131_OVERFLOW_CLIP_FIXTURE_TEST=1` build define에서만 제공한다. C header, JNI, BuildConfig, Java route가 같은 feature 상태를 사용해야 한다. 기본 FFI 정적 라이브러리에서 C13.1 fixture 심볼·source payload가 빠지는지와 feature artifact에 들어가는지를 각각 검사한다.

Chrome oracle은 agent-browser를 사용한다. 고정 11개 좌표의 `document.elementFromPoint()`와 runtime box hit query를 비교해 clip 안/밖, clip margin, nested axis, positioned escape 결과가 11/11 일치했다. 같은 11 query는 Android API 37 ARM64 AVD의 실제 앱 fixture snapshot에서도 11/11 일치했다. Android 캡처는 AVD SurfaceView `PixelCopy`로 수행하며, 선택한 CSS px 중심을 density로 surface texel에 변환한다. screenshot 픽셀은 paint 증거다. 고정 point 비교는 DOM event dispatch와 전체 hit-test matrix를 증명하지 않는다. [앱 실행 근거](evidence/c13-1-overflow-clip-hit-test-android-avd-2026-10-14/README.md)

## 현 검증과 남은 경계

[C13.1 Android AVD 근거](evidence/c13-1-overflow-clip-android-avd-2026-10-14/README.md)는 13개 node frame, Chrome 대조 13/13 geometry, 선택한 11개 표면 색 표본의 Android AVD/Chrome exact 일치와 agent-browser Chrome DOM query 대 host runtime query 11/11 일치를 기록한다. [후속 앱 실행 근거](evidence/c13-1-overflow-clip-hit-test-android-avd-2026-10-14/README.md)는 실제 Android fixture snapshot의 hit query 11/11과 PixelCopy 11/11을 기록한다. AVD는 ANGLE·SwiftShader software renderer다. C13.1 완료 판정이나 실기기/최종 compositor 표시 결과로 해석하지 않는다.

여전히 필요한 C13.1 관문은 전체 사전 비교 matrix 적용, 실제 paint/hit negative-control breadth, clip ancestry allocation 실패 후 동일 runtime worker 복구, `clip` 대 `hidden`/`flow-root` float formatting 비교, unsupported radius/transform 조합의 fail-closed 재검증, iOS Simulator 통합, WPT 후보 분류다. snapshot builder 단위에서는 예약 실패를 반환하고 같은 reserve의 다음 호출이 성공하는 회귀 테스트를 통과했다. 이는 앱 runtime worker 복구를 입증하지 않는다. [구현 실패 관점 검토 20개](evidence/c13-1-overflow-clip-implementation-review-2026-10-14.md), [hit-test query delta 검토 20개](evidence/c13-1-overflow-clip-hit-test-review-2026-10-14.md), [앱 hit-test delta 검토 20개](evidence/c13-1-overflow-clip-app-hit-test-review-2026-10-14.md)와 [PR 변경 관점 검토 30개](evidence/c13-1-overflow-clip-pr-review-2026-10-14.md)는 별도 기록이다. C13.2 이후 scroll/CSSOM/input 기능과 public CSS 지원 선언은 이 계약 바깥이다.
