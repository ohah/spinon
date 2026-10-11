# C12.4 iOS Metal surface 픽셀 검증 계획

**상위 계획:** [C12.4 쌓임 맥락](c12-4-stacking-context.md)  
**비교 기준:** [고정 Chromium 154 fixture](../tests/fixtures/css/references/c12-4-stacking-context-v1.json)  
**Android 대조:** [AVD PixelCopy 비교](../plan/c12-4-android-surface-pixels.md)  
**계획 실패 관점 검토:** [계획 검토 기록](../spec/internal/evidence/c12-4-ios-metal-surface-pixels-plan-review-2026-10-11.md)  
**상태:** 구현·iPhone 17 Pro / iOS 26.2 Simulator 실행 완료 · [픽셀 비교 결과](../spec/internal/evidence/c12-4-ios-metal-surface-pixels-2026-10-11/run-c124-ios-20261011T105945959Z-ce5b401c/comparison.json)

## 목적과 범위

고정 Chromium 154에서 확인한 Flex `row`·`row-reverse`·`column`·`column-reverse` 및 두 `order` 조합 8개를 iOS Simulator의 실제 WGPU Metal surface texture에서 직접 읽어 RGBA 기대 표본과 비교한다. 전체 CSS나 전체 C12.4 지원을 선언하지 않는다.

- iPhone 17 Pro / iOS 26.2 Simulator를 사용한다. Android 실기기는 사용하지 않는다.
- 각 조합을 새 앱 프로세스와 새 runtime host에서 실행해 이전 scene, log, surface frame을 재사용하지 않는다.
- 기존 C12.4 JavaScript fixture의 Flex container와 두 item의 inline style을 V8에서 변경한다. HostDocument → Stylo → Taffy → paint snapshot → WGPU Metal surface 경로를 그대로 통과한다.
- 읽기 대상은 headless texture나 별도 offscreen target이 아니라 `CAMetalLayer`가 제공한 WGPU surface texture다. WGPU `SurfaceCapabilities.usages`가 `COPY_SRC`를 지원하지 않으면 실패로 기록한다. screenshot, UIKit snapshot 또는 offscreen rendering으로 대체하지 않는다.
- 이 경로는 내부 진단 전용이며 제품 API·기본 렌더 경로에 노출하지 않는다. 읽기 대기는 render 전용 queue에서만 수행하고 UIKit main thread에서는 기다리지 않는다.

## 입력과 고정 기대값

8개 case의 방향, order, sample 위치와 색상은 `crates/spinon-render-wgpu/tests/runtime_stacking.rs`의 `reverse_flex_paint_matches_all_pinned_chrome_overlap_pixels`와 고정 Chrome reference에서 가져온다.

| 방향 | item order (파랑, 빨강) | surface local CSS sample | 기대 RGBA |
| --- | --- | --- | --- |
| `row` | `[0, 0]` | `(70, 155)` | `[229, 72, 77, 255]` |
| `row` | `[1, 0]` | `(70, 155)` | `[40, 102, 246, 255]` |
| `row-reverse` | `[0, 0]` | `(70, 155)` | `[40, 102, 246, 255]` |
| `row-reverse` | `[1, 0]` | `(70, 155)` | `[229, 72, 77, 255]` |
| `column` | `[0, 0]` | `(70, 175)` | `[229, 72, 77, 255]` |
| `column` | `[1, 0]` | `(70, 175)` | `[40, 102, 246, 255]` |
| `column-reverse` | `[0, 0]` | `(70, 175)` | `[40, 102, 246, 255]` |
| `column-reverse` | `[1, 0]` | `(70, 175)` | `[229, 72, 77, 255]` |

원본 fixture canvas는 `320×240 CSS px`, Flex parent origin은 `(20,125) CSS px`다. 로컬 microfixture sample `(50,30)` 또는 `(50,50)`을 합쳐 표의 surface CSS 좌표를 만든다. 실제 drawable 크기와 scale은 앱 실행 metadata에서 읽는다. 실행 기기의 scale로 physical coordinate를 계산하며 좌표는 `(css + 0.5) × scale`의 내림 규칙을 쓴다.

## 구현과 판정 단계

1. 내부 fixture에서만 Metal surface `COPY_SRC` usage를 요청하고 런타임 capability를 검사한다. 지원되지 않으면 명확한 실패 보고를 남긴다.
2. 선택한 WGPU surface frame을 그린 같은 command encoder에서 texture → padded readback buffer 복사를 제출한다. 포맷은 현재 C12.4 SDR profile의 `Rgba8Unorm[Srgb]` 또는 `Bgra8Unorm[Srgb]`만 허용하고 채널 순서를 명시적으로 해석한다.
3. sample 요청은 surface width/height 안의 좌표여야 하며 case당 1개, 전체 8개를 제한한다. surface generation 또는 presentation sequence가 바뀌면 결과를 버린다.
4. readback mapping은 비동기 callback을 사용하고 render queue에서 최대 5초만 대기한다. timeout, map failure, capability 부재, 잘못된 포맷은 성공 표본으로 대체하지 않는다.
5. 각 프로세스는 고유 run ID, case ID, simulator UDID, iOS build, Metal adapter/device, surface format·크기·scale, input hash와 RGBA 표본을 기록한다. 기존 run marker나 중복 case는 실패다.
6. 독립 비교기는 고정 8-case 집합, run ID, 좌표, 포맷 해석 및 RGBA를 검사한다. 누락·중복·낡은 log·한 채널 변이·좌표 뒤바꿈·잘못된 BGRA 변환 negative control은 반드시 거부한다.
7. 기본 C12.4 Simulator smoke와 전체 workspace, Clippy, rustfmt, CSS reference 검사를 다시 실행한다. 실제 실행 결과와 한계는 계획·상태 대장·내부 계약·Tailnet preview에 반영한다.

## 완료 기준과 경계

- [x] iOS Simulator 8개 case 모두 실제 WGPU Metal surface texture에서 읽었다. 캡처 시점의 renderer 로그가 `metal-surface`임을 비교기가 확인했다.
- [x] 방향/order/sample 좌표와 case별 RGBA가 8/8 Chrome 고정 표본에 일치했다. 색상 허용 오차는 채널당 0이다.
- [x] 비교기 negative control 8개가 누락·중복·잘못된 run ID·색상 변이·좌표 뒤바꿈·offscreen 결과·실패 상태를 거부했다. WGPU capability 부족, map timeout, surface generation 경합의 플랫폼 fault injection은 실행하지 않았다.
- [x] surface pixel readback과 변경된 reverse fixture의 앱 실행을 case별 새 앱 프로세스, 원본 로그·화면 캡처 및 단일 비교 JSON으로 재현했다.
- [ ] iOS Simulator 결과를 실제 iPhone, 하드웨어 성능, WPT 적합성 또는 전체 C12.4 지원으로 확대하지 않는다.

reverse Flex의 Chrome 호환 동작은 W3C Flexbox paint 설명과 다른 제한 예외로 이미 분류되어 있다. 이 실행도 Chrome 154 compatibility observation만 확인하며 규격 적합성을 주장하지 않는다. Android 실기기 검증, Android hardware GPU pixel, iOS 실기기, 모든 21개 C12.4 지원 scene의 iOS pixel matrix와 전체 CSS 지원은 이 계획의 완료 기준이 아니다.

실행 과정에서 첫 `row-reverse` 표본 위치가 240×90 parent 안의 실제 겹침 영역을 벗어나 배경색을 읽었다. 샘플러 오류로 숨기지 않고 실패 run을 보존한 뒤 Chrome·WGPU microfixture와 동일한 100×100 parent로 fixture 초기화를 고쳐 새 프로세스로 8개를 모두 다시 실행했다. 비교 결과의 batch/case run ID와 Chromium 버전도 통합 비교기로 재생성했다. [첫 실패와 수정 후 run evidence](../spec/internal/evidence/c12-4-ios-metal-surface-pixels-2026-10-11/).
