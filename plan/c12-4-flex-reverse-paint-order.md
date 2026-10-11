# C12.4 reverse Flex paint 순서 계획

**상위 계획:** [C12.4 쌓임 맥락](c12-4-stacking-context.md)  
**사전 비교:** [pinned Chrome reverse Flex 관측](../spec/internal/evidence/c12-4-flex-reverse-paint-precomparison-2026-10-11.md)  
**구현 전 실패 관점 검토:** [계획 검토](../spec/internal/evidence/c12-4-flex-reverse-paint-plan-review-2026-10-11.md)  
**상태:** 구현 완료 · Android API 37 AVD의 21-scene PixelCopy 22/22 exact · iOS 26.2 Simulator Metal surface의 reverse Flex 8/8 exact · iOS 26-scene geometry matrix 26/26 · 실기기와 전체 scene별 iOS surface pixel은 별도 미확정

## 목표

고정 Chrome 154가 실제로 그리는 reverse Flex item 겹침을 WGPU paint 목록에서 재현한다. 기존 C12.4 Android AVD surface pixel run에서 지원 scene 21개 중 `flex-row-reverse-paint` 하나가 불일치했다. geometry는 고정 Chrome과 같았고, 겹침 위치의 Chrome pixel은 파랑, Android `PixelCopy` pixel은 빨강이었다.

## 계약과 의도적 경계

- 비교 모델은 [사전 비교 기록](../spec/internal/evidence/c12-4-flex-reverse-paint-precomparison-2026-10-11.md)의 Chrome `154.0.8037.98` / 바이너리 SHA 고정 microfixture와 기존 C12.4 screenshot이다.
- 해당 oracle의 경험적 규칙은 computed `order`와 source order로 만든 direct in-flow flex item paint sequence를 `row-reverse` / `column-reverse`에서 역순으로 소비하는 것이다. non-reverse `row` / `column`은 현재 순서를 유지한다.
- 이는 W3C Flexbox Level 1의 reverse 값은 paint order에 영향을 주지 않는다는 문구와 충돌한다. 이 변경은 pinned Chrome 154 호환을 선택하는 제한된 예외다. 표준 적합성이나 다른 브라우저 동등성을 주장하지 않고, 미래 Chrome 기준 변경 시 다시 비교한다.
- 문서 트리, layout frame, NodeId, computed `order`, source/accessibility traversal은 변경하지 않는다. 오직 GPU에 제출하는 render box 순서만 조정한다.
- `display:none`과 absolute/fixed direct child의 기존 제외·순서 규칙, nested stacking context의 원자성, 같은 order의 stable source-order tie 정책을 보존한다.

## 구현 항목

1. `ordered_children`에서 부모의 computed `flex-direction`을 검증하고 reverse 여부를 paint ordering 단계에 전달한다. CSS 값을 재파싱하거나 raw author declaration을 사용하지 않는다.
2. 기존 `(order, source_order)` stable ordering 결과를 만든 뒤 in-flow Flex item만 reverse direction에서 역순으로 배치한다. out-of-flow child와 숨겨진 subtree는 기존 슬롯·필터 정책을 유지한다.
3. 실패 주입 가능한 reserve 경로 및 invalid/missing computed property 오류 전파를 그대로 유지한다. 추가 allocation 없이 reverse iterator로 적용 가능한지 확인한다.
4. 8개 Chrome oracle 조합의 Rust paint-rank와 WGPU overlap pixel 회귀를 추가한다. 기존 `flex-order-and-tie` 회귀도 유지한다.
5. Android API 37 AVD에서 APK를 빌드하고 26-case runtime matrix(21 ready / 5 expected rejection), 21-scene `PixelCopy`(22 samples)를 실행한다. 연결된 실기기는 사용하지 않는다.
6. 실제 diff와 실행 근거를 기준으로 별개의 20 관점 구현 검토를 작성한다. 계획 검토를 재사용하지 않는다.
7. 결과를 C12.4 계획, 내부 계약/evidence index, `spec/STATUS.md`, public preview 문서에 동기화한다. [iOS Metal surface 8-case 비교](../plan/c12-4-ios-metal-surface-pixels.md)는 별도 계획에서 완료했으며, 이 결과를 전체 iOS 26-scene matrix로 확대하지 않는다.

## 통과 조건

- 8개 Chrome oracle 조합의 direct Flex paint rank와 WGPU overlap pixel이 모두 일치한다.
- 기존 C12.4의 모든 scene이 예상 runtime 상태를 유지하고, 지원 scene 21개 / 표본 22개가 Android AVD surface에서 모두 channel error 0으로 통과한다.
- AVD는 `emulator-5554`, API 37이며 renderer가 ANGLE/SwiftShader인지 기록한다. 결과를 hardware GPU·실기기 성능으로 일반화하지 않는다.
- focused Rust tests, WGPU readback test, 해당 Node fixture tests, Android build/runtime/pixel scripts, formatting과 diff check가 통과한다.
- metadata/hash 불일치, case/sample 누락, reversed 아닌 direction, out-of-flow item, order tie, nested contexts, unsupported input의 negative controls가 기대대로 실패하거나 기존 동작을 보존한다.
- CSS 표준 문구와 pinned Chrome 관측의 차이는 계약/evidence에 드러나며 C12.4 전체 CSS 지원으로 확대 표시하지 않는다.

## 실행 결과

- Chrome `154.0.8037.98`의 여덟 방향/order 조합을 Rust paint-rank와 macOS WGPU offscreen overlap pixel로 확인했다. 기준 Chrome은 W3C Flexbox 설명과 다른 reverse paint 관측을 보이며, 이 구현은 저장소의 pinned Chrome 호환 기준에만 맞춘다.
- Android `emulator-5554`, API 37 AVD의 26-scene runtime matrix는 26/26 일치했다. 21개 지원 scene의 64개 node frame 오차는 최대 0 CSS px였고, 5개 미지원 scene은 기대한 진단으로 거부됐다.
- Android `PixelCopy`는 지원 scene 21/21, 표본 22/22, 최대 채널 오차 0으로 통과했다. `flex-row-reverse-paint` 표본은 파랑 `[40,102,246,255]`였다. 실행 자료는 [pixel 비교 JSON과 원본 캡처](../spec/internal/evidence/c12-4-android-surface-pixels-2026-10-11/run-c124-20261011T100934321Z-68d7a5c4/comparison.json)에 있다.
- AVD는 ANGLE/SwiftShader 소프트웨어 그래픽 경로다. 실기기 GPU 검증으로 일반화하지 않는다. iOS 26.2 Simulator의 별도 실행은 변경된 reverse fixture 8개를 실제 Metal surface에서 읽었고 모두 pinned Chrome 표본과 일치했다. 전체 26-scene geometry matrix는 별도 계획에서 26/26 통과했으며, 모든 scene의 iOS surface pixel readback은 이 계획에서 수행하지 않았다.
- 별도 [구현 후 실패 관점 검토](../spec/internal/evidence/c12-4-flex-reverse-paint-implementation-review-2026-10-11.md)는 구현과 실행 결과의 범위·실패 경계를 대조한다.

## 중단 조건

- 고정 Chrome 버전/해시 또는 microfixture 결과가 예상과 다르면 구현 전에 oracle을 다시 고정한다.
- 현재 실행 AVD가 아닌 serial, 다른 API/density, 다른 fixture/reference hash이면 결과를 성공으로 합산하지 않는다.
- frame 변화, HostDocument 재정렬, 실패 시 부분 paint snapshot, sample 색상 불일치가 있으면 gate를 닫지 않는다.
- Android surface gate 통과 뒤 iOS reverse fixture 표면 pixel도 확인했다. 실기기 GPU·실기기 메모리와 전체 scene의 iOS surface pixel 검증은 별도 미완료다.
