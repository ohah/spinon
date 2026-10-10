# 0050 · C10.3.1 Flex 축 역방향·줄 역방향

**문서 ID:** `0050` · **내부 계약 숫자 버전:** `0.1.0` 고정 · **상태:** 구현 브랜치 검증 완료 · **공개 CSS/API 완료:** 아님

`0050`은 제한 runtime Flex profile 안에서 `row-reverse`, `column-reverse`, `wrap-reverse`와 `flex-flow`를 처리하는 내부 계약이다. C10.3.1 부분 범위만 다루며 C10.3 또는 전체 Flexbox 완료를 뜻하지 않는다.

## 입력과 실행 경로

Stylo가 계산한 `flex-direction`·`flex-wrap` longhand를 여섯 runtime Flex profile에서 typed `FlexDirection`·`FlexWrap`으로 투영한다. Taffy 0.14.0에 주축 reverse와 교차축 line reverse를 전달한다. 계산 좌표는 바뀌어도 `HostDocument` 자식 벡터, DOM/source preorder와 NodeId는 원래 순서를 보존한다.

검증한 경로는 V8 fixture → Stylo cascade → typed style/layout snapshot → Taffy → RuntimeRenderSnapshot → WGPU다. 대상 runtime profile은 `RuntimeFlexLayoutV1`, `RuntimeFlexPaintV1`, `RuntimeFlexCustomPropertiesV1`, `RuntimeFlexCustomPropertiesPaintV1`, `RuntimeFlexRegisteredPropertiesV1`, `RuntimeFlexRegisteredPropertiesPaintV1`이다. 기존 `FlexAlignmentV1`에서는 역방향 값을 `flex-direction` 속성·NodeId를 포함해 거부한다.

## 지원 범위와 실패 경계

| 조건 | 계약 |
| --- | --- |
| `row-reverse`, `column-reverse` | runtime Flex profile에서 주축 시작 위치를 반전하며 원본 자식 순서는 바꾸지 않는다. |
| `wrap-reverse` | 줄의 교차축 쌓임 방향만 반전한다. 줄 안 item 수집·배치 규칙은 기존 C10.1/C10.2를 따른다. |
| `flex-flow` | 두 토큰의 순서를 바꾼 shorthand를 동일한 longhand로 계산하고, 이후 longhand 선언의 cascade 우선순위를 유지한다. |
| 방향 조합 | 고정 환경 `writing-mode:horizontal-tb`, `direction:ltr`만 검증한다. reverse Flex와 computed `direction:rtl` 조합은 지원하지 않고 `flex-direction` 오류로 거부한다. |
| 상위 Flex profile | 여섯 runtime profile 밖에서는 reverse `flex-direction`을 계산 성공으로 바꾸지 않는다. |
| 측정 정밀도 | 각 node의 `x`, `y`, `width`, `height`별 최대 절대 오차는 `0.5 CSS px`다. DPR 1·2 결과를 각각 비교한다. |

`order`, `align-self`, 비기본 `align-content`, baseline, writing mode 전환, RTL reverse 동작, text/anonymous flex item, positioned child paint, hit-test, 실기기 동작과 하드웨어 GPU 성능은 이 단계에서 지원 완료로 주장하지 않는다. positioned child 교차는 C12 뒤 C10.3.5에서, 실제 텍스트 baseline은 C15 연계 후 다룬다. Unsupported CSS computed value나 layout 오류를 기본값으로 성공 처리하지 않는다.

## Chromium 비교와 결과

- Oracle은 macOS arm64 Chrome `154.0.8037.98`, revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`다. 고정 reference ID는 `chromium-darwin-arm64-Chrome-154.0.8037.98-c0e49361e122-da53d5b7e9bf`다.
- [기준 JSON](../../tests/fixtures/css/references/c10-3-1-flex-reverse-v1.json)은 17개 case·64개 node를 DPR 1과 2에서 보존한다. Taffy core 비교는 모든 case의 모든 node frame field를 검사했다. 계산 기준은 Chromium이며 Taffy 출력은 기대값으로 사용하지 않는다.
- [reference 회귀 검사](../../tools/css-reference/c10-3-1-flex-reverse.test.mjs)는 fixture·도구 digest, shorthand override, 줄 경계와 두 플랫폼 실행 로그를 검사한다. `node --test tools/css-reference/c10-3-1-flex-reverse.test.mjs` 결과는 5/5 통과다.
- Android API 37 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 실제 V8 fixture의 7개 CSS frame이 reference와 각각 일치했고 WGPU가 7개 box를 제출했다. Simulator 결과는 실기기 또는 하드웨어 GPU 성능 근거가 아니다.
- Android evidence는 ANGLE/SwiftShader의 `Gl` 경로를 사용하며, 표면 texture는 `840×630`, CSS viewport는 `320×240`이었다. iOS Simulator는 Metal 표면 texture `960×720`, CSS viewport `320×240`이었다. 로그 출력은 플랫폼별 NodeId와 CSS frame을 분리해 보존한다.

실행 근거: [사전 Chromium 기준](./evidence/c10-3-1-flex-reverse-precomparison-2026-10-10.md), [구현 실패 관점 검토](./evidence/c10-3-1-flex-reverse-implementation-review-2026-10-10.md), [Android 로그](./evidence/c10-3-1-flex-reverse/android-api37-emulator-logcat.txt)·[화면](./evidence/c10-3-1-flex-reverse/android-api37-emulator.png), [iOS 로그](./evidence/c10-3-1-flex-reverse/ios-26.2-simulator-log.txt)·[화면](./evidence/c10-3-1-flex-reverse/ios-26.2-iphone-17-pro-simulator.png).

## C10.3 진행 상태

C10.3.1 구현 브랜치의 Chromium·Rust·Android·iOS Simulator 검증은 완료했다. 이 계약의 상태는 PR 병합 전까지 구현 브랜치 기준이며 공식 상태 대장의 C10.3 parent는 계속 미완료다. C10.3.2 `order`, C10.3.3 item/line alignment, C10.3.4 baseline, C10.3.5 positioned child 교차와 C15 text baseline 연결은 남아 있다. 계약 숫자 버전 `0.1.0`은 유지한다.

## 참고

- [C10 Flexbox 계획](../../plan/c10-flexbox.md) · [C10.3 순서·정렬 계획](../../plan/c10-3-flex-order-alignment.md)
- [C10.3.1 fixture inventory](../../tests/fixtures/css/c10/flex-reverse-inventory.json) · [runtime fixture](../../tests/fixtures/css/c10/runtime-flex-reverse.js)
- [Chromium reference 검사](../../tools/css-reference/c10-3-1-flex-reverse.test.mjs)
