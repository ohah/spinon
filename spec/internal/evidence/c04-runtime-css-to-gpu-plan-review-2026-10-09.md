# C04.10 계획 적대 검토

**대상:** [`plan/c04-runtime-css-to-gpu.md`](../../../plan/c04-runtime-css-to-gpu.md) · **대상 SHA-256:** `ebbae63903a8c32589758642108a85fb4b1a44e5217350ea58ad7c1105de4859` · **검토 관점:** 구현 전 계획의 서로 다른 실패 경로 20개

| # | 공격 관점 | 판정 및 반영 |
|---:|---|---|
| 1 | 이 계획이 JSON 진단 결과를 GPU 제품 장면으로 오인하는가 | 거부했다. C04.10에서 새 typed `RuntimeRenderSnapshot`을 생성하고 실제 WGPU surface 화면까지 완료 조건에 둔다. |
| 2 | 고정 `StaticRenderSnapshot`의 fixture hash 요구를 runtime에 억지로 적용하는가 | 거부했다. static snapshot은 fixture provenance 전용으로 남기고 runtime scene 자료형을 분리한다. |
| 3 | 새 background-color를 허용하며 C04.9 오류 의미를 조용히 바꾸는가 | 초기 계획에는 충돌이 있었다. 새 `RuntimeFlexPaintV1` profile/state로 격리하고 C04.9 layout JSON의 `unsupported_inline_property` 동작을 유지하도록 수정했다. |
| 4 | UA cascade와 layout input이 서로 다른 문서·세대에서 올 수 있는가 | generation, document, render-tree, style, environment 전체 tuple이 같을 때만 장면을 구성하도록 고정했다. |
| 5 | JS commit 직후 오래된 layout이 늦게 GPU에 제출될 수 있는가 | RuntimeGpuHost가 eval·환경 변경·장면 작성·draw를 하나의 직렬 host queue에서 처리하고 제출 직전에 current key를 다시 검사한다. |
| 6 | surface resize 뒤 이전 surface용 장면이 새 surface에 제출되는가 | CSS revision과 별도로 surface generation을 제출 key에 포함하고 불일치 장면을 거부한다. |
| 7 | surface 종료 중 native handle이 renderer에서 해제될 수 있는가 | queue drain 뒤 renderer destroy, 그 뒤 native surface release 순서를 완료 조건과 소유 계약에 넣었다. |
| 8 | V8·Stylo·Taffy·WGPU 초기화가 UI thread를 막는가 | 전용 Android HandlerThread와 iOS 직렬 background queue를 정하고 UI thread는 view/layer와 변경 알림만 처리하도록 했다. |
| 9 | `RuntimeSession` raw pointer를 새 GPU 코드가 재해석해 ABI를 깨는가 | 기존 C ABI 포인터 의미를 유지하고 별도 `SpinonRuntimeGpuHost` opaque owner를 두도록 했다. |
| 10 | Java/Swift 사이에서 JSON·가변 Node 배열을 반복 복사하는가 | Rust 내부에서 같은 revision의 scene을 만들고 renderer에 직접 전달하도록 계획했다. |
| 11 | unsupported CSS가 기본값으로 조용히 그려지는가 | 지원 밖 property·partial alpha·gradient·image 등은 scene 전체 실패 또는 명시 미지원으로 두고 fallback을 금지했다. |
| 12 | transparent 기본 배경이 불투명한 박스로 잘못 그려지는가 | `none` paint와 host surface clear color를 분리하고, fixture root에만 명시 불투명 색을 둔다. |
| 13 | `display:none` 또는 크기 0 노드가 GPU 삼각형으로 남는가 | 해당 노드는 장면의 identity를 보존하되 draw primitive를 만들지 않도록 경계를 고정했다. |
| 14 | DOM 생성 ID와 그리기 순서가 뒤섞이는가 | paint order는 ID 수치가 아닌 DOM preorder로 생성하고 중복·누락·비연속 mapping은 전체 실패한다. |
| 15 | DOM preorder를 CSS stacking·z-index 적합성으로 과장하는가 | fixture를 겹치지 않는 일반 flow로 한정하고 positioning·z-index·복잡한 stacking을 미지원으로 명시했다. |
| 16 | CSS px, Android dp/iOS point, physical backing scale이 두 번 적용되는가 | viewport는 native content bounds로 정의하고 logical CSS scale과 GPU backing density를 각 경계에서 한 번 적용한다. |
| 17 | safe-area·system bar가 좌표 원점을 플랫폼별로 다르게 만드는가 | surface를 콘텐츠 bounds에 제한하고 system bar/safe-area를 뺀 bounds를 viewport로 삼도록 계획했다. |
| 18 | offscreen readback만으로 실제 화면 표시를 주장하는가 | 색 정확 비교는 동일 draw pass offscreen readback에 한정하고, 양 플랫폼 surface screenshot은 실제 화면 제출의 별도 근거로 요구한다. |
| 19 | Chromium 전체 screenshot 색 오차가 GPU·색 관리 차이를 숨기거나 과장하는가 | computed color 정확 비교, frame 좌표 0.5 CSS px, 내부 readback pixel 정확 비교를 분리하고 캡처 가장자리를 색 oracle에서 제외한다. |
| 20 | spike/backend 복제, 버전 상승 또는 제품 지원 과장이 생기는가 | renderer core는 `crates/spinon-render-wgpu`로 승격하고 spike는 기존 fixture entry로 축소한다. WGPU 구현 중복·C ABI fixture 회귀를 확인하며 내부 계약 0.1.0, 시뮬레이터 전용 증거, 미지원 범위를 고정했다. |

## 계획 수정 후 재확인

초안에서 확인한 C04.9 오류 정책 충돌, runtime·GPU 소유권 경합, 화면 캡처와 pixel oracle의 혼동을 계획에 반영했다. 수정된 문서의 SHA-256은 표제와 같으며, 20개 관점이 반영된 완료 조건·비교 경계·범위 밖 항목을 대조했다. 구현 전에 새 Chromium fixture와 precomparison 근거를 먼저 고정한다.
