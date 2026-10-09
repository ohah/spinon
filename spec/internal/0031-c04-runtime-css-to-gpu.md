# 0031 · C04.10 Runtime CSS → WGPU 장면

**계약 버전:** 미출시 내부 계약 `0.1.0` 고정 · **상태:** 구현·시뮬레이터 검증 및 구현 검토 완료, PR 검토 전 · **공개 API:** 아님 · **제품 CSS 지원 완료:** 아님

## 범위

같은 `RuntimeSession`의 HostDocument를 `RuntimeFlexPaintV1` 스타일 계산, Taffy frame 계산, 불변 Rust 렌더 장면, Android·iOS WGPU surface로 연결한다. C04.10 앱은 viewport resize를 관찰하는 [`runtime-css-to-gpu-resize.js`](../../tests/fixtures/css/c04/runtime-css-to-gpu-resize.js)를 실제 V8에 전달한다. Chromium 비교 oracle은 대응하는 HTML fixture를 사용하며 입력 대응은 [기존 고정 기준](evidence/c04-runtime-css-to-gpu-precomparison-2026-10-09.md)과 [resize 비교 기준](evidence/c04-runtime-css-to-gpu-resize-precomparison-2026-10-09.md)에 기록한다.

이 계약은 제한된 `display`, Flex 배치, 크기 및 불투명 `background-color` 수직 슬라이스만 다룬다. 앱 작성자용 API, React/Vue/Svelte 연동, 일반 CSS 지원, 텍스트·접근성·DOM 이벤트, 성능 보장은 제공하지 않는다. 내부 계약 숫자 버전은 출시 전까지 `0.1.0`을 유지한다.

## Rust 장면 자료형

- `RuntimeRenderKey`는 `generation`, `document_revision`, `render_tree_revision`, `style_revision`, `environment_revision` 전체를 값으로 보존한다. 일부 revision만 비교해 현재성을 판정하지 않는다.
- `RuntimeRenderSnapshot`은 CSS px viewport와 DOM preorder로 정렬된 표시 가능 render box 목록을 갖는다. `display:none` 또는 폭·높이 0인 항목은 생성 단계에서 제외한다. paint order는 포함된 box에 대해 0부터 연속이다.
- 각 `RuntimeRenderBox`는 NodeId, 유한한 CSS px 사각형, paint, 순서를 갖는다. NodeId 중복과 paint 순서 누락·중복을 거부한다. `CssRect`는 비유한 값, 음수 크기와 오른쪽/아래쪽 산술 overflow를 거부한다.
- `RuntimePaint::None`은 투명 배경이라 그리지 않는다. `OpaqueCssSrgb`는 alpha 255의 8-bit sRGB 색만 표현한다. 부분 alpha와 다른 장식은 모델에 포함하지 않는다.
- DOM preorder box 목록 순서로 사각형을 제출한다. 첫 비교 입력은 겹치는 요소나 stacking context를 쓰지 않으므로 stacking·z-index 호환을 주장하지 않는다.

## 내부 C ABI와 호출자

| 함수 | 호출자·동작 | 수명과 직렬화 |
| --- | --- | --- |
| `spinon_runtime_gpu_host_new` | runtime session과 C04.10 profile을 소유하는 host를 만들고 초기화 보고를 쓴다. 실패 시 null | output은 writable이며 다른 호출과 host free를 경합시키지 않는다 |
| `spinon_runtime_gpu_host_begin_presentation_update` | UI 변경 handler가 환경·surface 변경을 큐에 넣기 전에 이전 scene을 무효화한다. lock·대기 없이 증가한 내부 sequence를 반환하며 실패 시 0 | 성공 sequence는 내부 값이다. 다른 ABI 함수 인자로 전달하거나 revision처럼 해석하지 않는다 |
| `spinon_runtime_gpu_host_set_environment` | 양수의 유한한 CSS viewport·device scale·`dark` 0/1을 전달하고 `layout_timeout_millis` 안에 CSS 재계산 완료를 기다린다 | 기다릴 수 있는 runtime background executor에서만 호출한다. surface resize와 renderer 호출을 같은 sequence로 묶는 API는 아니다 |
| `spinon_runtime_gpu_host_eval` | NUL 종료 UTF-8 JavaScript를 동기 평가하고 같은 runtime revision의 최신 장면을 발행한다. `layout_timeout_millis`는 평가 이후 CSS 재계산 대기에만 적용한다 | runtime background executor에서 호출한다. JavaScript 실행 자체는 시간 제한·취소 대상이 아니며 source는 호출 중 읽을 수 있어야 한다 |
| `spinon_runtime_gpu_host_eval_fixture` | 저장소의 단일 `runtime-css-to-gpu-resize.js`를 V8에서 동기 평가한다. `layout_timeout_millis`는 평가 이후 CSS 재계산 대기에만 적용한다 | runtime background executor에서 호출한다. JavaScript 실행 자체는 시간 제한·취소 대상이 아니다 |
| `spinon_runtime_gpu_host_create_android` | Android native window에서 WGPU surface와 renderer를 만들고 선택된 adapter/device·format 보고를 쓴다. width·height는 backing surface pixel이다. backend 0은 Vulkan 초기화가 표면 configure 전에 실패하면 surface를 버리고 GL을 순차 재시도한다. 1=Vulkan 강제, 2=GL 강제이며 iOS는 3=Metal을 사용한다 | 같은 host에 renderer 하나만 허용한다. 전용 serial render executor에서 호출하고 native window owner는 renderer보다 오래 살아야 한다 |
| `spinon_runtime_gpu_host_prepare_uikit_surface` | UIKit `UIView`에서 WGPU surface만 준비한다. `UIView.layer` 접근은 메인 스레드에서만 허용된다 | 반드시 iOS 메인 스레드에서 호출한다. view는 호출 동안 유효해야 하며 성공한 surface는 같은 host의 대기 자원이다 |
| `spinon_runtime_gpu_host_create_uikit` | 메인 스레드에서 준비한 UIKit surface에 adapter·device·pipeline을 초기화한다. CAMetalLayer configure는 미룬다. width·height는 backing surface pixel이다 | 전용 serial render queue에서 호출한다. 성공 보고 문자열은 별도 해제 handle이 아니다 |
| `spinon_runtime_gpu_host_configure_uikit_surface` | 준비된 renderer의 `CAMetalLayer` surface 속성을 구성한다 | UIKit layer 속성 변경이므로 메인 스레드에서 호출한다. draw보다 먼저 호출해야 한다 |
| `spinon_runtime_gpu_host_resize` | surface pixel 크기를 바꾸고 기존 장면을 무효화한다. 성공 시 0 | Android는 serial render executor에서, iOS는 메인 스레드에서 호출한다. iOS는 호출 전에 serial render queue barrier를 통과해 draw와 직렬화한다 |
| `spinon_runtime_gpu_host_draw` | GPU submit 전과 present 직전에 publication sequence를 비교한다. 장면이 없으면 clear color만 그린다. 각 검사보다 먼저 무효화된 작업은 제출·표시하지 않으며, 마지막 검사 뒤 이미 시작한 present 한 프레임은 완료될 수 있다 | renderer 생성·resize·destroy와 같은 serial render executor/thread에서만 호출한다. 계산 완료를 기다리지 않는다 |
| `spinon_runtime_gpu_host_inject_next_draw_failure_for_test` | 내부 `c04-runtime-gpu-test-hooks` 시험 빌드에서 다음 draw 한 번에 합성 surface 오류를 넣는다. 기본 앱 빌드에는 함수 심볼과 호출 경로가 없다 | 시험 빌드에서만 호출하며 draw·resize·destroy와 같은 serial render executor/thread에서 직렬화한다. 제품 API가 아니다 |
| `spinon_runtime_gpu_host_destroy_renderer` | 현재 surface generation을 무효화하고 대기 중인 surface 접근이 끝난 뒤 renderer를 파괴한다 | 같은 platform render queue에서 draw·resize와 직렬화한다. Android `surfaceDestroyed`는 drain 완료 뒤 반환한다 |
| `spinon_runtime_gpu_host_free` | V8 session과 host를 해제한다 | 새 요청 차단, runtime 작업 drain, renderer queue drain 및 renderer 파괴 후에만 호출한다. 동시에 다른 host 호출이 없어야 한다 |

Android backend 기본값은 Vulkan을 먼저 초기화한다. adapter/device 초기화가 surface configure 전에 실패하면 해당 surface를 버리고 GL backend로 새 surface를 순차 생성하며, 두 backend를 한 인스턴스에서 동시에 configure하지 않는다. 지원 backend 한 개만 강제하는 값도 개발 검증용으로 제공한다. Android는 `SurfaceView` callback이 바뀐 platform surface generation을 기록하고, 크기 변경 시 현재 generation의 이전 renderer를 render queue에서 파괴한 다음 같은 host의 최신 native window로 새 renderer를 만든다. Android emulator에서 기존 renderer의 resize status와 WGPU texture size가 새 크기였는데도 색상 영역이 구 surface 크기에 머무는 동작을 확인해 현재 개발 경로는 재생성을 택했다. 새 renderer가 준비된 뒤 CSS environment를 다시 계산하고 draw한다. `surfaceDestroyed`는 presentation sequence를 무효화한 뒤 render queue의 이전 작업과 renderer 파괴가 끝날 때까지 반환하지 않는다. Activity 종료도 runtime 작업과 render queue가 모두 끝난 뒤 `SurfaceView` callback을 제거한다. iOS는 `CAMetalLayer` drawable 크기 변경 때 generation을 갱신하며 생성·resize·draw 작업이 현재 값을 대상으로 하는지 확인한다. UIKit view는 surface 파괴 작업이 끝날 때까지 강하게 유지한다. 고정한 `wgpu 30.0.1` UIKit 경로는 `CAMetalLayer` 속성을 바꾸므로 iOS는 UIView surface 준비와 `surface.configure`를 메인 스레드에서 수행한다. adapter·device·pipeline 초기화와 draw는 전용 serial render queue에서 수행한다. resize 때는 렌더 큐 barrier로 이미 제출된 draw를 비운 뒤 메인 스레드에서 surface를 다시 구성하고, 새 generation 장면만 다시 제출한다. Android pixel size·native window·WGPU texture, iOS point/drawable size, CSS viewport/frame을 함께 기록한다. 두 플랫폼은 색상 체계·surface 변경 시 host sequence를 UI 이벤트에서 먼저 무효화한다. Rust renderer는 GPU submit 전과 present 직전에 sequence를 검사한다. 각 검사 시점보다 먼저 무효화된 장면은 제출·표시하지 않지만, 마지막 검사 후 present가 시작된 프레임 한 장은 완료될 수 있다. platform surface generation은 `RuntimeRenderKey`의 필드가 아니며 host ABI에도 전달하지 않는다.

### platform 작업 admission 계약

- JS 평가와 DOM 변경은 순서 보존하며 절대 병합·대체하지 않는다. 환경·viewport 재계산과 표시 요청만 latest-wins다.
- runtime presentation lane은 최신 environment state와 dirty marker 하나만 보관한다. render surface lane은 최신 surface generation·수명·format·크기와 draw 요청을 canonical state로 유지하고 하나의 reconciler가 따라잡는다. 각 lane은 실행 하나와 pending drain 하나를 넘지 않는다. 유입이 멈춘 뒤 마지막 입력을 적용하며 지연 상한은 보장하지 않는다.
- stale renderer 생성물은 파괴한다. 현재 generation에서 surface 생성/configure가 실패하면 오류를 보고하고 새 surface 입력 또는 명시 retry 전까지 자동 재시도하지 않는다. lane의 복구 가능한 작업 실패나 executor 예약 실패는 lane 상태를 되살려 이후 입력을 받을 수 있게 한다. fatal process error는 삼키지 않는다.
- Android `surfaceDestroyed`는 새 draw를 닫고 renderer queue가 surface 사용을 끝낼 때까지 기다린다. 안전한 해제보다 timeout 반환을 택하지 않으며 대기 시간을 기록한다. 상태 잠금을 잡고 native 호출이나 대기를 하지 않는다. Android Activity 종료는 runtime drain 뒤 render 해제와 host free를 직렬화한다.
- iOS UIKit/`CAMetalLayer` 접근은 main thread에서만 한다. UIKit configure callback은 하나만 pending으로 두고 그동안의 크기 변경은 최신 상태에 합친다. main thread는 render queue를 동기 대기하지 않는다.
- draw 오류 주입은 내부 검증 빌드에만 포함한다. 오류는 nonzero로 전달되고 JS/DOM/CSS snapshot을 바꾸지 않으며, hook을 소비한 후 다음 정상 draw를 확인한다. 합성 오류는 실제 GPU/driver 고장 복구 증거로 보지 않는다.

platform runtime/render lane은 실행 중 하나와 대기 drain 하나만 유지한다. 환경·표시·surface 재조정 입력만 latest-wins로 합치며 JavaScript 평가와 DOM 변경은 순서를 보존한다. Android Java·iOS Swift 단위 시험은 100,000건 단일 생산자 burst와 8개 동시 생산자×10,000건에서 최신 canonical state와 queue 상한을 확인한다. Android API 37·iOS 26.2 시뮬레이터에서는 각 render lane을 멈춘 뒤 10,000건을 제출하고 종료 drain을 확인했다. Android pending draw는 종료 때 버리고 renderer/host를 해제했으며 iOS는 surface generation 무효화 뒤 renderer/host를 비동기 drain했다. 두 플랫폼 모두 내부 draw 오류 `-11` 뒤 같은 `environment_revision`의 일반 draw가 성공했다. 상세 실행 원본은 [queue 시뮬레이터 기록](evidence/c04-runtime-css-to-gpu-queue-simulators-2026-10-09.md)과 연결한다.

V8 평가·환경 변경과 그 완료 대기는 platform runtime executor에서 수행한다. Android의 WGPU 생성·resize·draw·destroy는 전용 serial render 실행기에서 수행한다. iOS의 surface 준비·surface configure·resize configure는 메인 스레드에서 수행하고 adapter/device/pipeline 초기화·draw·destroy는 전용 serial render 실행기에서 수행한다. iOS 메인 스레드는 CSS 계산이나 adapter/device 대기를 하지 않는다. 종료 시 UI는 host sequence를 즉시 무효화하고, runtime 작업을 끝낸 후 renderer queue를 drain해 파괴한다.

## 상태 코드와 보고

| 코드 | 의미 |
| ---: | --- |
| `0` | 호출 성공 |
| `-1` | 잘못된 인자·포인터·UTF-8 또는 0 surface 크기 |
| `-3` | 출력 버퍼 부족 또는 보고 문자열 복사 실패 |
| `-10` | 제한 시간 내 CSS 계산 완료 없음 |
| `-11` | runtime layout 또는 WGPU renderer 오류 |
| `-12` | 오래된 revision·presentation sequence 거부 또는 sequence 소진 |

`eval`은 내부 `RuntimeSession`의 JavaScript·취소·종료 오류 코드를 그대로 전달할 수도 있다. surface 생성은 포인터 대신 null/nonnull 및 보고 문자열을 사용한다. output 포인터가 null이거나 capacity가 0이면 보고를 쓰지 않고 `-1`을 돌려준다. 보고 복사가 실패하면 원래 결과보다 `-3`이 우선한다. WGPU surface 실패는 로그/상태 보고이며 JS 또는 CSS snapshot 성공으로 바꾸지 않는다.

`layout_timeout_millis`는 Stylo·Taffy 계산 결과를 기다리는 시간만 제한한다. 동기 JavaScript 평가 자체는 C04.10에서 취소하거나 시간 제한하지 않으며, 긴 실행을 안전하게 중단하는 별도 계약은 아직 없다.

## 색상·표면 규칙

- WGPU surface는 RGBA8 또는 BGRA8 형식 중 `SRGB` color space가 가능한 것을 사용한다. 하드웨어 sRGB 텍스처 형식을 우선하며, 없으면 일반 UNORM 형식에 shader 입력을 sRGB encoded 값으로 전달한다. 지원 조합이 없으면 초기화를 실패시킨다. present mode는 FIFO다.
- 하드웨어 sRGB 형식에는 CSS encoded sRGB 채널을 선형 RGB로 바꿔 shader에 전달하고, UNORM 대체 형식에는 encoded 값을 그대로 전달해 이중 변환을 막는다. 색 표본은 같은 WGPU draw path의 offscreen readback에서 채널 단위 정확 비교한다. 화면 캡처는 실제 surface 표시 증거이며 색상 oracle로 쓰지 않는다.
- 사각형은 CSS px frame과 viewport 크기로 정규화해 GPU clip space에 제출한다. Android dp·iOS point를 CSS px로 바꾸고 backing scale을 surface pixel에 반영하는 책임은 각 platform 경계에 있다.
- 투명 장면은 paint 없음으로 처리한다. surface clear color는 CSS `background-color`가 아니다.

## 미지원 경계 및 검증 상태

partial alpha, gradient, image, border, shadow, transform, clipping, stacking, text, scroll, accessibility, hit testing 및 continuous frame loop는 지원하지 않는다. 기존 C04.8 JSON·C04.9 layout JSON과 S04 static fixture 계약은 바꾸지 않는다. 상세 완료 조건과 Chromium 오차 기준은 [C04.10 계획](../../plan/c04-runtime-css-to-gpu.md)을 따른다. Rust readback·단위 테스트만으로 Android·iOS 앱 화면 통과를 표시하지 않는다.

구현 실패 경로 검토와 실행 범위는 [최종 구현 검토](evidence/c04-runtime-css-to-gpu-final-implementation-review-2026-10-09.md) 및 [대기열·종료·draw 실패 시뮬레이터 기록](evidence/c04-runtime-css-to-gpu-queue-simulators-2026-10-09.md)에 있다. 실기기와 하드웨어 GPU 검증은 포함하지 않는다.
