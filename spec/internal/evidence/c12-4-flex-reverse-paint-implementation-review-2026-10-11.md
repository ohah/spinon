# C12.4 · reverse Flex paint 구현 후 검토와 실행 근거

**범위:** pinned Chrome 154에서 관측한 `row-reverse`·`column-reverse` paint 순서를 제한 profile에 반영한 변경과 Android AVD surface 검증. 계획 검토 표를 재사용하지 않고 실제 코드·테스트·플랫폼 근거를 다시 확인했다.

## 구현 후 실패 관점 20개

| # | 확인한 실패 가능성 | 확인 결과와 근거 |
| ---: | --- | --- |
| 1 | raw author CSS를 다시 읽어 cascade 결과와 다른 `flex-direction`을 쓰는가 | painter는 computed style snapshot의 `flex-direction`을 읽는다. |
| 2 | 누락 또는 알 수 없는 computed direction을 `row`로 조용히 대체하는가 | `row`, `row-reverse`, `column`, `column-reverse`만 받고 누락·미지원 값은 오류로 반환한다. |
| 3 | 부모가 Flex가 아닌데도 자식 순서를 뒤집는가 | Flex 부모만 `ordered_children` 경로를 실행한다. 기존 Block 형제 순서는 유지된다. |
| 4 | `display:none` 또는 absolute/fixed 자식을 in-flow item reverse에 섞는가 | 표시되는 in-flow 요소만 임시 item 순서에 넣고 기존 필터·슬롯 규칙을 보존한다. |
| 5 | `order` 정렬과 reverse 적용 순서가 뒤바뀌어 결과가 다른가 | `(order, source_order)`로 안정 정렬한 뒤 reverse 주축에서만 item sequence를 뒤집는다. |
| 6 | 동일 `order` tie가 source order를 잃는가 | 0/0 조합이 Rust paint-rank 및 Chrome/WGPU 8-case 회귀에 포함되어 통과한다. |
| 7 | non-reverse `row`·`column` 결과가 회귀하는가 | 두 방향의 order 0/0 및 1/0 pixel 표본이 pinned Chrome과 일치한다. |
| 8 | reverse 분기가 `row-reverse`에만 적용되고 `column-reverse`는 빠지는가 | 두 reverse 방향 모두 0/0·1/0 rank와 pixel 조합을 실행했다. |
| 9 | 역순 처리 중 HostDocument child vector가 바뀌는가 | 변경은 painter용 `ordered_children` 반환값에만 적용된다. source tree와 NodeId는 그대로다. |
| 10 | Taffy geometry를 paint 순서 조정이 바꾸는가 | 26-scene Android matrix에서 지원 64 node frame의 Chrome 대비 최대 오차는 0 CSS px였다. |
| 11 | 중첩 Flex의 부모 order가 손자에게 전파되는가 | 정렬은 각 Flex 부모의 direct children 범위이며 기존 nested-flex local-order 회귀가 workspace test에서 통과했다. |
| 12 | reverse가 z-index 단계나 context atomics 전체를 뒤집는가 | reverse는 direct in-flow item sequence에만 적용한다. 기존 context phase·nested atomicity 회귀를 workspace test로 재실행했다. |
| 13 | 추가 allocation이나 새 실패 stage가 reverse 분기에서 생기는가 | reverse는 이미 만든 vector의 iterator 방향만 바꾼다. 기존 painter reservation failure-injection 테스트가 통과한다. |
| 14 | WGPU가 snapshot의 실제 paint 순서를 그리지 않는가 | 독립 Chrome overlap fixture 8개를 macOS WGPU offscreen texture에서 읽었고 모든 RGBA 표본이 일치했다. |
| 15 | 색상·표본 좌표 실수로 pixel regression이 거짓 통과하는가 | 표본은 겹침 내부이며 방향별 geometry를 사용한다. 첫 column-reverse 시도에서 y=30이 겹침 밖임을 frame으로 확인하고 y=50 표본으로 고쳤다. 최종 8개 표본은 통과한다. |
| 16 | Android AVD가 실제 `SurfaceView` 대신 다른 texture를 읽는가 | Android 경로는 `PixelCopy.request(SurfaceView, ...)`를 사용했고 metadata의 serial은 `emulator-5554`, API 37이다. |
| 17 | 일부 scene/sample 누락을 전체 성공으로 세는가 | comparator는 고정 case 집합과 count를 확인한다. 21/21 지원 scene·22/22 sample이 모두 일치했다. |
| 18 | 오래된 PNG나 다른 fixture/reference가 현재 결과에 섞이는가 | 고유 run ID와 inventory/reference/APK hash를 비교 JSON에 남겼다. `flex-row-reverse-paint` pixel은 `[40,102,246,255]`, 최대 채널 오차는 0이다. |
| 19 | simulator software 결과를 기기 GPU 결과로 과장하는가 | AVD metadata는 ANGLE/Vulkan/SwiftShader software renderer라고 기록한다. 하드웨어 GPU나 실기기 검증으로 취급하지 않는다. |
| 20 | Chrome 호환 예외를 표준 적합성으로 잘못 표시하는가 | pinned Chrome 154 관측과 W3C Flexbox Level 1 §4.3/§5.1 설명의 충돌을 계약·계획에 공개했다. 결과는 제한된 Chrome 호환 동작이며 iOS Metal pixel 검증은 미완료다. |

## 실행 근거

- `mise exec -- cargo test --locked --workspace` — 통과. WGPU runtime integration 3개를 포함하며, workspace 출력에는 기존 ignored test 1개가 있다. [전체 원본 출력](./c12-4-flex-reverse-paint-2026-10-11/cargo-test-workspace.log).
- `node --test tools/css-reference/c12-4-runtime-matrix.test.mjs tools/css-reference/c12-4-stacking-context.test.mjs tools/css-reference/c12-4-android-surface-pixels.test.mjs` — 10/10 통과.
- Android API 37 AVD APK build 및 runtime matrix — 26/26 통과, 21 ready·5 expected rejection, 지원 frame 64개 최대 오차 0 CSS px.
- Android `PixelCopy` — 21/21 supported scene, 22/22 samples, 최대 channel error 0. [원본 비교 JSON](./c12-4-android-surface-pixels-2026-10-11/run-c124-20261011T100934321Z-68d7a5c4/comparison.json) · [reverse scene PNG](./c12-4-android-surface-pixels-2026-10-11/run-c124-20261011T100934321Z-68d7a5c4/flex-row-reverse-paint.png) · [원본 log](./c12-4-android-surface-pixels-2026-10-11/run-c124-20261011T100934321Z-68d7a5c4/flex-row-reverse-paint.log).
- iPhone 17 Pro / iOS 26.2 Simulator는 Metal `boxes=10` submit smoke를 기록했다. 이 실행은 변경된 reverse Flex fixture를 실행하지 않았고 Metal surface pixel도 읽지 않았다.

## 남은 경계

- API 37 AVD는 ANGLE/SwiftShader software renderer다. Android 실기기 동작·성능을 증명하지 않는다.
- iOS reverse fixture 및 전체 C12.4 matrix의 Metal surface pixel readback은 미실행이다.
- 이 결과는 전체 CSS Flex paint 규칙, 전체 WPT 적합성, 다른 브라우저 동등성 또는 C12.4 전체 완료를 뜻하지 않는다.
- Chrome 관측과 W3C 설명이 다르므로 사용자에게는 Chrome 154 pinned compatibility 예외로 문서화한다. 엔진 기준을 바꾸면 재비교가 필요하다.

## 후속 iOS surface 확인

위 검토 당시에는 iOS reverse fixture를 실행하지 않았고 Metal surface readback도 없었다. 이후 별도 새-process 실행에서 iOS 26.2 Simulator의 실제 WGPU Metal surface를 읽어 Flex `row`, `row-reverse`, `column`, `column-reverse` 및 order 조합 8개를 pinned Chrome 154 고정 표본과 비교했고 8/8 exact match했다. 최초 `row-reverse` sample은 겹침 밖 배경을 읽어 실패 run으로 보존했고, fixture parent를 Chrome/WGPU microfixture와 같은 100×100으로 수정한 뒤 재실행했다. 원시 비교는 [8-case JSON](./c12-4-ios-metal-surface-pixels-2026-10-11/run-c124-ios-20261011T105945959Z-ce5b401c/comparison.json), 계획·제약은 [iOS Metal 계획](../../../plan/c12-4-ios-metal-surface-pixels.md)에 있다. 이 결과는 iOS 전체 26-scene matrix나 WPT 적합성이 아니다.
