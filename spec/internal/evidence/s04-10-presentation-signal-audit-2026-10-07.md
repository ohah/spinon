# S04.10 · 플랫폼 표시 신호와 FrameId 연결 조사

**조사일:** 2026-10-07 · **범위:** 고정 `wgpu 30.0.1` 코드, Apple·Android·Vulkan 공개 API, Android API 36 에뮬레이터와 Android 16 실기기 검증 · **결과:** API 조사·두 장치 capability 확인·실기기 화면 제출과 오프스크린 색상 readback 완료, 실제 표시 시각 수집은 미검증

## 판정

`Queue::present`가 반환한 사실만으로 표시 frame을 알 수는 없습니다. 다만 Android Vulkan에는 wgpu의 HAL 경계에서 frame별 `presentID`를 부여하고, 이후 해당 `presentID`의 실제 표시 시각을 비동기로 조회할 수 있는 후보 경로가 있습니다. iOS Metal에도 drawable별 확인 callback이 있지만 `wgpu 30.0.1`의 공개 `SurfaceTexture`에서는 그 drawable을 꺼내거나 callback을 등록할 수 없습니다.

따라서 S04.10은 아직 미완료입니다. Android API 36 에뮬레이터는 `VK_GOOGLE_display_timing`을 제공하지 않지만, Android 16 실기기의 Samsung Xclipse 940은 이를 열거했고 실기기에서 실행한 wgpu adapter도 해당 feature를 지원한다고 보고했습니다. 잠금 해제 뒤 실기기에서 S04 화면 제출과 36개 오프스크린 RGBA 표본을 두 surface generation에 걸쳐 확인했습니다. 이 실행은 `Queue::present` 요청과 GPU 색상 계산을 검증하지만 실제 표시 시각·present ID·제품 `FrameId` 상관관계를 검증하지 않습니다. 현재 `DeviceDescriptor`는 해당 feature를 요청하지 않으며 timing 조회도 구현되지 않았습니다. iOS callback과 두 플랫폼 입력 순서도 미실행입니다.

## 조사 기준과 현재 연결

- `spikes/wgpu-backend/Cargo.lock`은 `wgpu`, `wgpu-core`, `wgpu-hal`을 `30.0.1`로 고정합니다. Android fixture는 `SurfaceView`를 사용하고 현재 프로젝트 `compileSdk`는 36입니다. iOS fixture view는 `CAMetalLayer`를 backing layer로 사용합니다.
- 현재 fixture는 `queue.submit()` 뒤 `queue.present(frame)`을 호출하고 `present=requested`와 자체 frame sequence를 기록합니다. 이 로그는 OS가 어느 frame을 표시했는지 증명하지 않습니다.
- 공개 `wgpu::Queue::present`는 `()`를 반환하며, 공개 `SurfaceTexture`의 backend detail은 비공개 필드입니다. `wgpu::Adapter::get_presentation_timestamp()`는 presentation engine 시계의 기준값을 주지만 특정 frame의 표시 결과가 아닙니다.

### Android API 36 에뮬레이터 capability 조회

실기기가 아닌 `emulator-5554`에서 읽기 전용 `adb -s emulator-5554 shell cmd gpu vkjson`을 실행했습니다. JSON의 `devices[0].extensions[].extensionName`을 대상으로 확인했으며, 다른 위치의 feature/property 키 개수는 extension 수로 세지 않았습니다. 첫 조회 코드는 extension 이름을 `devices[0]` 바로 아래에서 찾는 잘못된 경로를 사용해 실기기를 미지원으로 잘못 분류했습니다. 올바른 배열 경로로 고쳐 두 장치를 다시 조사했으며 아래 결과는 교정 이후 값입니다. 대상은 Android 16 / API 36 `sdk_gphone64_arm64`이며, `llvmpipe (LLVM 21.1.4, 128 bits)` CPU Vulkan 장치와 Mesa `llvmpipe` 드라이버가 열거한 49개 device extension 중 `VK_GOOGLE_display_timing`은 없었습니다.

```text
deviceName=llvmpipe (LLVM 21.1.4, 128 bits)
driverName=llvmpipe
VK_GOOGLE_display_timing=false
deviceExtensions=49
```

이 명령의 출력은 `vkjson is deprecated`라고 알립니다. 이 검사는 Android 시스템이 열거한 현재 에뮬레이터 Vulkan 장치의 읽기 전용 capability 조회입니다. wgpu 앱을 빌드·설치하거나 wgpu의 adapter feature 집합을 직접 관찰한 것은 아니며, 실기기·다른 드라이버·다른 Android 버전의 지원 여부도 말하지 않습니다. 현재 에뮬레이터에서는 제안한 `VK_GOOGLE_display_timing` 경로를 실행할 수 없으므로 present-ID 통합 실험을 통과한 것으로 간주하지 않습니다.

### Android 16 실기기 Vulkan·wgpu capability 확인

사용자가 실기기 검증을 요청한 뒤 연결된 Samsung `SM-S731N`에서 확인했습니다. 기기 속성은 Android 16 / API 36, `arm64-v8a`였고, `cmd gpu vkjson`의 `devices[0].extensions[].extensionName`에는 Samsung Proprietary driver 24.0.534가 열거한 154개 device extension 중 `VK_GOOGLE_display_timing`이 있었습니다. 따라서 에뮬레이터의 미지원 결과를 Android 전체로 일반화할 수 없습니다.

기존 V8 Android build 산출물을 읽기 전용으로 지정해 S04 Android fixture를 빌드하고, 이번 worktree에서 만든 APK를 실기기에 설치해 Vulkan backend selector `1`로 실행했습니다. 앱이 실제 선택한 `Samsung Xclipse 940` adapter의 `adapter.features()`도 `VULKAN_GOOGLE_DISPLAY_TIMING` 지원을 `true`로 기록했습니다.

```text
device=SM-S731N Android=16 api=36 abi=arm64-v8a
driver=Samsung Proprietary driver 24.0.534
deviceExtensions=154 VK_GOOGLE_display_timing=true
SPINON_S04_RENDERER=ready surface_generation=1 size=1080x2340 density=2.8125 backend=Vulkan device=IntegratedGpu name=Samsung Xclipse 940 google_display_timing_supported=true format=Rgba8UnormSrgb color_space=Srgb
```

첫 실기기 실행은 잠금 화면/dozing 상태여서 renderer 초기화까지만 남고 프레임은 제출되지 않았습니다. 잠금 해제 후 다시 실행하자 실제 화면에 색상 띠가 나타났고, 앱의 36개 표본 readback 표시도 통과로 바뀌었습니다. 앱을 cold start한 뒤와 홈 화면으로 나갔다 다시 전면에 가져온 뒤 각각 새 surface generation을 만들고 같은 색상 readback을 통과했습니다.

```text
10-07 22:28:32.971 SPINON_S04_RENDERER=ready surface_generation=1 size=1080x2340 density=2.8125 backend=Vulkan device=IntegratedGpu name=Samsung Xclipse 940 google_display_timing_supported=true format=Rgba8UnormSrgb color_space=Srgb
10-07 22:28:32.973 SPINON_S04_FRAME status=0 acquire=Success surface_generation=1 frame_sequence=1 submission_index=SubmissionIndex { index: 1 } present=requested fixture=S04-asymmetric-y-v1 document_revision=1 render_tree_revision=1 diagnostics=deferred_to_readback
10-07 22:28:32.990 SPINON_S04_READBACK status=1 surface_generation=1 frame_sequence=1 fixture=S04-asymmetric-y-v1 target=Rgba8UnormSrgb size=301x65 bytes_per_row=1280 sample_rows=12 samples=36 rgba=exact diagnostics=none
10-07 22:28:44.175 SPINON_S04_RENDERER=ready surface_generation=2 size=1080x2340 density=2.8125 backend=Vulkan device=IntegratedGpu name=Samsung Xclipse 940 google_display_timing_supported=true format=Rgba8UnormSrgb color_space=Srgb
10-07 22:28:44.177 SPINON_S04_FRAME status=0 acquire=Success surface_generation=2 frame_sequence=1 submission_index=SubmissionIndex { index: 1 } present=requested fixture=S04-asymmetric-y-v1 document_revision=1 render_tree_revision=1 diagnostics=deferred_to_readback
10-07 22:28:44.193 SPINON_S04_READBACK status=1 surface_generation=2 frame_sequence=1 fixture=S04-asymmetric-y-v1 target=Rgba8UnormSrgb size=301x65 bytes_per_row=1280 sample_rows=12 samples=36 rgba=exact diagnostics=none
```

각 `SPINON_S04_FRAME` 줄은 같은 `frame_sequence`를 JNI wrapper와 Java host가 각각 기록한 중복 로그이며 실제 제출은 한 번입니다. readback 성공도 native 상세 줄과 Java 상태 줄로 각각 기록됩니다. 오프스크린 readback은 fixture render target의 색상 36점을 검사하며 Android display surface 픽셀을 읽지 않습니다. 실기기 캡처에는 색상 띠가 보이지만 캡처 픽셀은 픽셀 단위 oracle로 사용하지 않았습니다. `present=requested`는 `Queue::present` 호출만 뜻합니다. `google_display_timing_supported=true`는 adapter capability이며 `DeviceDescriptor`에서 feature를 요청하지 않았고, `presentID`를 붙이거나 `actualPresentTime` record를 조회하지 않았습니다. `frame_sequence=1`은 renderer마다 초기화되는 fixture-local counter라 production `FrameId`가 아닙니다. 실기기에서 입력 순서와 오래된 generation 거부는 시험하지 않았습니다.

## Android · Vulkan 표시 시각 조회 후보

`wgpu-hal 30.0.1`의 Vulkan `Surface`는 다음 present에 사용할 `VkPresentTimeGOOGLE`을 설정하는 `set_next_present_time()`과 현재 swapchain handle을 얻는 `raw_native_swapchain()`을 공개합니다. HAL 문서는 `presentID`를 이후 `vk::PastPresentationTimingGOOGLE` 결과에서 다시 찾을 수 있다고 설명하고, 조회는 `ash`로 직접 해야 한다고 명시합니다. `wgpu::Surface::as_hal()`과 `wgpu::Device::as_hal()`은 해당 HAL surface/device 접근 지점입니다. 기기와 Vulkan adapter가 `VULKAN_GOOGLE_DISPLAY_TIMING`을 제공하고 이를 Device 생성 때 요청해야 이 경로를 사용할 수 있습니다.

Khronos 확장 명세에서 `presentID`는 `vkQueuePresentKHR` 요청을 식별하는 값입니다. `desiredPresentTime = 0`은 표시 시각을 강제하지 않고 ID만 부여하는 용도입니다. 이후 `vkGetPastPresentationTimingGOOGLE`가 비동기로 반환하는 `actualPresentTime`은 해당 swapchain image가 실제 표시된 시각이며, Android에서는 `CLOCK_MONOTONIC`과 같은 단조 시계 기준 나노초입니다. 결과는 한 번만 반환되고, 구현이 보관하는 이력에는 제한이 있습니다. swapchain 접근은 외부 동기화가 필요합니다.

따라서 안전한 실험 흐름은 다음과 같습니다.

1. 단일 serialized present owner가 새 내부 `presentID`를 할당하고 `(runtime, surface generation, FrameId, presentID)`를 bounded pending map에 등록합니다.
2. 같은 owner가 HAL surface에 해당 `presentID`와 `desiredPresentTime = 0`을 설정한 뒤, 그 frame만 `Queue::present`합니다. 등록 또는 HAL 설정이 실패하면 표시 확인으로 취급하지 않습니다.
3. 이후 같은 swapchain의 과거 timing 결과를 `ash`로 조회하고 결과 배열 순서가 아닌 `presentID`로 pending map을 찾습니다. 반환 record가 가리키는 generation·FrameId가 일치할 때만 확인 frame으로 전이합니다.
4. 조회에서 아직 나오지 않은 frame은 pending으로 둡니다. map 만료·surface 교체·장치 손실·ID 충돌·지원하지 않는 extension·조회 실패는 확인 성공으로 추정하지 않고 unknown/drop 진단으로 끝냅니다. 입력이 결과보다 먼저 도착하는 경우와 bounded 대기 한계는 실행 전 계약에서 따로 고정해야 합니다.

이는 현재 구현이 아닙니다. Android 실기기는 extension을 지원하지만, `DeviceDescriptor`에서 feature를 요청한 뒤 `wgpu` HAL surface/device raw guard와 `ash` 호출을 동시에 안전하게 직렬화할 수 있는지, 요청한 각 `presentID`에 유효한 timing record가 오는지는 아직 검증하지 않았습니다. API 36 emulator는 이 경로를 지원하지 않습니다. Rust에서 `ash` API를 직접 부르려면 기존 `wgpu-hal`의 transitive 의존성만으로는 부족하므로 `ash`를 직접 의존성으로 선언해야 합니다. lockfile에는 이미 같은 `ash 0.38.0+1.3.281`이 있습니다.

## Android · 다른 backend와 OS timing API의 경계

- `Choreographer.FrameTimeline`의 VSync ID와 예상 시각은 frame scheduling 입력입니다. `SurfaceControl.Transaction.setFrameTimeline(vsyncId)`는 그 transaction의 목표 timeline을 지정합니다. 현재 wgpu Vulkan `Queue::present`는 `vkQueuePresentKHR`를 직접 호출하므로 이 transaction을 자동으로 붙이지 않습니다. callback 시점에 얻은 VSync ID를 제출 frame의 표시 결과라고 간주하면 안 됩니다.
- `SurfaceControl.JankData`는 API 36부터 존재하고 VSync ID 및 jank 분류를 제공하지만 callback은 frame 처리 경로 밖에서 batch로 늦게 도착합니다. `presentTime`은 API 37.2부터이고 unknown/unset 값도 정의되어 있습니다. `SurfaceView.registerOnJankDataListener`의 surface별 등록 역시 문서상 37.2부터입니다. 현재 프로젝트는 API 36 emulator/`compileSdk 36` 기준이라 이 경로는 현재 fixture에서 실행되지 않았습니다. 이 API는 Vulkan 결과가 없을 때의 대체 경로로 간주하지 않습니다.
- `SurfaceControl.Transaction.addTransactionCompletedListener`도 transaction 완료 결과입니다. 현재 프레임을 wgpu swapchain present 대신 그 transaction에 연결하지 않으므로 현재 렌더 경로의 완료 신호는 아닙니다.
- `VK_GOOGLE_display_timing`은 Vulkan backend와 기기 extension 지원에 한정됩니다. GLES backend 또는 extension 미지원 기기에는 이 경로가 없습니다. 별도 confirmed-frame 신호가 없는 backend에서 S05 click 의미를 유지하는 방법은 아직 구현·검증하지 않았습니다.

## iOS · Metal drawable callback 경계

Apple의 `MTLDrawable.addPresentedHandler(_:)`는 drawable 단위 callback을 제공하고 `presentedTime`은 화면에 표시된 host time입니다. `presentedTime == 0`이면 표시되지 않았거나 dropped frame입니다. callback에서 해당 drawable의 `drawableID`/`presentedTime`을 읽어 FrameId와 묶는 방식은 표준 Metal surface를 직접 소유할 때 사용할 수 있습니다.

현재 `wgpu-hal 30.0.1` Metal `SurfaceTexture`는 drawable을 private field로 보관하고, HAL Queue가 내부 command buffer에서 그 drawable을 present합니다. 공개 `wgpu::SurfaceTexture`는 drawable이나 `MTLDrawable` callback을 노출하지 않습니다. `Surface::as_hal()`은 `CAMetalLayer`를 볼 수 있게 하지만 이미 획득한 특정 drawable을 돌려주지 않습니다. 정확한 연결에는 wgpu API/HAL callback 추가(upstream 협의 또는 좁은 pinned patch)나 wgpu presentation을 우회하는 별도 Metal 경로가 필요합니다. `CADisplayLink`, command buffer 완료, `Queue::on_submitted_work_done`만으로 특정 drawable의 표시 완료를 대체하지 않습니다.

## 다음 실행 단계와 통과 판정

1. 실기기에서 확인된 Xclipse 940 adapter에 opt-in으로 `VULKAN_GOOGLE_DISPLAY_TIMING`을 요청하고, HAL `set_next_present_time()`과 ash timing 조회를 실제 surface에 연결합니다. 현재 확인한 physical draw/readback과 emulator의 extension 미지원 경로를 각각 회귀 기준으로 유지합니다.
2. 연속 frame에 `FrameId`와 분리된 `presentID`를 넣고, `actualPresentTime` record의 `presentID`로만 귀속합니다. timestamp 미도착, frame 교체/drop, query batch, ID 재사용, surface resize/recreate, device/surface 오류를 포함해 bounded map과 unknown 처리까지 확인합니다.
3. iOS는 프레임 하나와 실제 `MTLDrawable` callback을 식별하는 최소 wgpu patch 실험을 별도로 수행합니다. `presentedTime == 0`, callback 중복/지연, surface 재생성 뒤 늦은 callback을 확인합니다.
4. 두 플랫폼 callback은 JS를 직접 호출하지 않고 같은 직렬 UI-host 전달 지점에 표식만 보냅니다. 로그의 `FrameId`, `presentID` 또는 drawable ID, surface generation, 입력 sequence로 순서를 확인합니다. 광학 scanout이나 사용자가 픽셀을 인지한 사실을 주장하지 않습니다.

**미실행:** wgpu Device에 feature 요청, Android 실제 표시 timing 수집, iOS drawable callback 연결, 입력 동시성 및 surface 교체 뒤의 늦은 timing callback 검증. 이번 실기기 실행과 문서 변경만으로 S04.10·S05.2를 완료 처리하지 않습니다.

## 주요 원문

- [wgpu 30.0.1 `Queue::present`와 `SurfaceTexture`](https://github.com/gfx-rs/wgpu/tree/v30.0.1/wgpu/src/api)
- [wgpu-hal 30.0.1 Vulkan `Surface::set_next_present_time`](https://docs.rs/wgpu-hal/30.0.1/wgpu_hal/vulkan/struct.Surface.html#method.set_next_present_time)
- [`VULKAN_GOOGLE_DISPLAY_TIMING` feature](https://docs.rs/wgpu-types/30.0.1/wgpu_types/struct.FeaturesWGPU.html#associatedconstant.VULKAN_GOOGLE_DISPLAY_TIMING)
- [Khronos `VK_GOOGLE_display_timing`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GOOGLE_display_timing.html), [`VkPresentTimeGOOGLE`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentTimeGOOGLE.html), [`VkPastPresentationTimingGOOGLE`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPastPresentationTimingGOOGLE.html), [`vkGetPastPresentationTimingGOOGLE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPastPresentationTimingGOOGLE.html)
- [Apple `MTLDrawable.addPresentedHandler`](https://developer.apple.com/documentation/metal/mtldrawable/addpresentedhandler%28_%3A%29), [`presentedTime`](https://developer.apple.com/documentation/metal/mtldrawable/presentedtime)
- [Android `SurfaceView.registerOnJankDataListener`](https://developer.android.com/reference/android/view/SurfaceView#registerOnJankDataListener(java.util.concurrent.Executor,android.view.SurfaceControl.OnJankDataListener)), [`SurfaceControl.JankData`](https://developer.android.com/reference/android/view/SurfaceControl.JankData), [`Choreographer.FrameTimeline`](https://developer.android.com/reference/android/view/Choreographer.FrameTimeline), [`SurfaceControl.Transaction.setFrameTimeline`](https://developer.android.com/reference/android/view/SurfaceControl.Transaction#setFrameTimeline(long))
