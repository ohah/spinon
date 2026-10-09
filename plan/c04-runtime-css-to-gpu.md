# C04.10 · 런타임 CSS 장면을 GPU 화면에 연결

- **문서 유형:** 구현 계획 · 구현 상태는 [`spec/STATUS.md`](../spec/STATUS.md)에서 관리
- **상위 항목:** [C04 stylesheet·selector·cascade](../spec/STATUS.md#css-구현-체크리스트) · [S04 Android·iOS GPU 적용기](../spec/STATUS.md#2-세-플랫폼-수직-구현)
- **선행 구현:** C04.9 Runtime CSS→Taffy layout snapshot, S04 고정 GPU surface, S02.2 revision tuple
- **내부 계약 버전:** 출시 전 `0.1.0` 고정. 계획·구현만으로 숫자 버전을 올리지 않는다.

## 현재 진행 상태

- 런타임 CSS 장면·WGPU Android/iOS 수직 경로, surface resize 왕복, latest-only lane, pending draw 종료, 내부 draw 오류 주입을 구현하고 시뮬레이터에서 확인했다.
- Android·Swift lane 100,000건 단일 생산자 및 8개 동시 생산자 시험, 양 플랫폼 10,000개 pending 요청·종료·오류 복구 실행 근거를 [queue 시뮬레이터 기록](../spec/internal/evidence/c04-runtime-css-to-gpu-queue-simulators-2026-10-09.md)에 연결했다.
- 전체 JS/CSS·Rust workspace·Clippy 회귀와 Android API 37 및 iOS 26.2 Simulator 앱 빌드를 통과했다. 구현 변경에 대한 별도 실패 관점 검토도 [검토 기록](../spec/internal/evidence/c04-runtime-css-to-gpu-final-implementation-review-2026-10-09.md)에 남겼다. PR #96 변경 검토를 마쳤으며 병합을 기다린다. 실기기·하드웨어 성능 근거는 주장하지 않는다.

## 확인된 수명 경계

Android demo의 현재 `dispose`는 Activity UI thread에서 runtime·render executor drain을 동기 대기한다. C04.10은 저장소의 고정 JavaScript fixture만 실행하며 V8 실행 취소·시간 제한을 제공하지 않는다. 임의 앱 JavaScript를 연결하기 전에는 Activity UI를 막지 않는 비동기 teardown 또는 별도 취소·종료 계약을 추가한다.

## 목적

같은 JavaScript DOM 작업에서 만든 HostDocument를 V8 → Stylo cascade → Taffy → 플랫폼 중립 렌더 장면 → Android·iOS WGPU surface까지 연결해, 실제 앱 화면에 CSS가 계산한 상자와 배경색을 그린다. C04.9의 JSON 진단 결과나 S04의 고정 `StaticRenderSnapshot`을 제품 장면으로 가장하지 않는다.

단일 V8 입력 [`runtime-css-to-gpu.js`](../tests/fixtures/css/c04/runtime-css-to-gpu.js)가 `<div>` 요소를 만들고 제한된 inline CSS를 적용한다. Chromium oracle은 대응하는 [`runtime-css-to-gpu.html`](../tests/fixtures/css/c04/runtime-css-to-gpu.html)을 사용한다. 두 입력의 요소 순서·style·viewport가 맞는지 고정 fixture 검증으로 확인한다. Stylo와 Taffy의 완료 key가 같은 경우에만 런타임 렌더 장면을 만들며, Android·iOS 개발 시뮬레이터 화면의 geometry와 색을 고정 Chromium 기준에 대조한다. HTML 요소를 OS별 native view로 만들지 않는다. native view는 WGPU surface를 소유하는 경계에만 둔다.

이 내부 수직 슬라이스는 전체 CSS, React/Vue/Svelte 어댑터, 텍스트·접근성·DOM 이벤트, 연속 프레임 구동기 또는 공개 앱 API 완료를 뜻하지 않는다.

## 작업 경계와 계약

### 런타임 계산과 장면

- 새 `RuntimeFlexPaintV1` profile에서 기존 레이아웃 속성과 `background-color`를 한 번의 Stylo cascade로 계산한다. C04.8의 7개 UA JSON 속성·schema·오류 의미와 C04.9의 layout JSON schema·기존 허용 목록·`unsupported_inline_property` 동작은 바꾸지 않는다. C04.9 layout JSON은 `background-color`를 계속 거부하고, 새 render state는 layout과 paint를 함께 처리한다.
- `spinon-render::StaticRenderSnapshot`은 fixture·Chromium provenance 전용으로 유지한다. 동적 장면에는 fixture 해시를 요구하지 않는 별도 `RuntimeRenderSnapshot`을 둔다.
- 장면 key는 generation, document revision, render-tree revision, style revision, environment revision을 모두 담는다. CSS 계산 결과와 layout 결과가 같은 key가 아니거나 현재 요청 key와 다르면 장면 생성을 거부한다. `SpinonRuntimeGpuHost`는 runtime session과 renderer 핸들을 묶는 C ABI 조정 객체다. 기존 Rust `RuntimeSession`의 전용 V8 actor와 cascade/layout worker를 유지하며, GPU renderer의 호출은 Android 전용 serial render executor와 iOS serial render queue에 한정한다. 스레드 사이에는 revision이 붙은 불변 장면만 전달한다. 플랫폼 큐는 현재 surface generation을 확인하고 WGPU renderer는 present 직전에 host presentation sequence를 다시 검사한다. C04.9 raw session API의 포인터 의미는 바꾸지 않는다.
- 각 노드에는 `NodeId`, DOM preorder paint 순서, CSS px frame, paint 값(불투명 sRGB 또는 `none`)을 보존한다. 중복·누락 ID, 잘못된 순서, 비유한 frame, 음수 크기, `x+width` 또는 `y+height` 산술 overflow는 장면 전체 실패다. 사각형이 viewport 바깥으로 나가는 것 자체는 오류가 아니며 GPU surface가 자른다.
- `display:none` 및 면적이 0인 노드는 그리지 않는다. 기본 투명 배경은 paint 없음으로 처리한다. 첫 oracle fixture의 root는 명시적인 불투명 배경색을 갖게 한다. surface clear color는 호스트가 정하는 값이며 CSS `background-color` 지원으로 보고하지 않는다.
- 허용 paint 입력은 Stylo 계산 결과가 불투명한 sRGB로 변환되는 `background-color` 값과 초기 투명값으로 한정한다. 부분 alpha, gradient, image, border, shadow, transform, clipping, stacking context는 실패 또는 미지원이다. CSS 문자열을 다시 파싱해 색상을 만들지 않는다.
- paint 순서는 요소 DOM preorder이며 첫 fixture는 일반 flow의 겹치지 않는 상자만 쓴다. 위치 지정·z-index·복잡한 stacking을 지원한다고 주장하지 않는다.

### 렌더러와 플랫폼

- 제품 WGPU surface·geometry 코드는 `crates/spinon-render-wgpu`에 둔다. 기존 `spikes/wgpu-backend`는 고정 S04/R13 fixture 진입점으로 축소하고 제품 렌더링 코드를 추가하지 않는다. fixture 출력은 기존과 동일해야 한다.
- Surface format은 `SRGB` color space를 제공하는 RGBA8/BGRA8 형식만 고른다. 하드웨어 sRGB 텍스처 형식이 있으면 우선하고, 없으면 UNORM 형식에 encoded sRGB shader 값을 사용한다. 두 경로 모두 sRGB 인코딩을 정확히 한 번만 적용하며, 지원되는 8-bit 조합이 없으면 명시적으로 실패한다.
- 플랫폼 FFI의 런타임 draw 요청은 Rust 안에서 revision 일치 장면을 얻어 WGPU renderer로 전달한다. Node 배열이나 JSON을 Java/Kotlin/Swift로 복사해 다시 Rust에 넣지 않는다. 기존 `spinon_runtime_session_*` C ABI 포인터 의미는 보존하고, 새 `SpinonRuntimeGpuHost`가 session·renderer 소유권을 묶는다.
- V8은 기존 Rust `RuntimeSession` actor thread에서 실행하고 cascade/layout은 기존 CSS worker에서 계산한다. 플랫폼은 eval과 environment update 호출을 직렬 background executor에서 수행하고, 계산 완료를 기다리는 일도 그 executor에서만 한다. Android WGPU renderer 생성·draw·destroy와 iOS adapter/device 초기화·draw·destroy는 전용 serial render queue에서 직렬 실행한다. Android surface 크기가 바뀌면 이전 renderer를 해당 queue에서 파괴하고 같은 runtime host와 최신 `SurfaceView` native window로 새 renderer를 만든다. renderer 재생성 성공 뒤에 새 generation의 CSS environment를 적용하고 draw한다. Android emulator에서 `surface.configure`가 성공하고 획득 texture 크기까지 새 값인데도 확대 영역이 검게 남는 것을 재현해 이 경계를 택했다. UIKit은 `UIView.layer` 접근이 main thread로 제한되므로 iOS의 WGPU surface handle 준비와 `CAMetalLayer` configure/resize는 main thread에서 수행한다. iOS resize는 presentation sequence를 먼저 무효화하고 draw admission을 닫은 뒤 render queue barrier를 거친다. main thread의 surface configure가 끝나고 surface generation이 다시 확인된 뒤에만 draw를 재개한다. 완성된 scene snapshot은 immutable publication 경계로 renderer에 전달한다. 플랫폼 큐는 target surface generation이 현재 값인지 확인하고, renderer는 GPU submit 전과 present 직전에 current presentation sequence를 검사한다. 마지막 검사 후 이미 시작한 present 한 프레임은 완료될 수 있다. `layout_timeout_millis`는 CSS 결과 대기만 제한하며 동기 JavaScript 실행 제한은 아니다. 렌더 큐는 V8이나 CSS 결과를 동기 대기하지 않는다. Android `SurfaceHolder.Callback.surfaceDestroyed`는 renderer 사용을 차단하고 render queue의 기존 작업과 destroy가 끝날 때까지 반환하지 않는다. 이는 Android [SurfaceView 공식 계약](https://developer.android.com/reference/kotlin/android/view/SurfaceView.html)의 surface 유효 수명 경계를 따른다. 이 콜백 동안 UI thread가 렌더 큐 drain을 기다릴 수 있다. surface 해제는 대기 중 렌더 작업이 끝난 뒤 수행한다. 이 구조는 JavaScript·CSS·GPU가 같은 OS thread를 공유한다고 주장하지 않는다.
- CSS viewport는 native surface의 콘텐츠 bounds를 CSS px로 나타낸다. Android dp·iOS point와 CSS px 대응 및 backing density 적용은 각 경계에서 한 번만 한다. OS system bar/safe-area 영역은 콘텐츠 surface에서 제외한다. 초기 media 입력은 OS color scheme와 mobile coarse/no-hover capability에서 얻는다.
- resize 또는 color scheme 변화는 단조 증가하는 host presentation sequence를 즉시 바꾸고 새 `EnvironmentRevision`을 요청한다. surface 생성·크기 변경·해제는 별도의 platform surface generation을 증가시킨다. background executor가 새 환경 revision을 발행하고 renderer queue가 현재 surface generation을 대상으로 구성한 뒤에만 해당 presentation sequence 장면을 제출할 수 있다. Android는 크기 변경에서 renderer를 파괴·재생성하고 새 environment를 적용한다. iOS는 기존 renderer를 resize한다. 화면 검증은 요청 크기와 함께 Android native window pixel 크기, WGPU가 획득한 surface texture 크기, CSS viewport/frame, draw 결과를 기록한다. 성공 status만으로 화면 전체가 새 크기에 맞춰 그려졌다고 판정하지 않는다. GPU submit 전과 present 직전 모두 sequence를 확인해 각 검사보다 먼저 무효화된 장면은 거부한다. 마지막 검사 뒤 이미 시작된 present 한 프레임은 완료될 수 있다. 변경 전의 미완료 장면은 제출하지 않고 최신 key로 다시 계산한다.

### 대기열 제한·종료·draw 오류

- V8 JavaScript 평가와 `HostDocument` 변경은 순서 보존 작업이다. 이 lane은 병합·대체·폐기하지 않는다. 환경·viewport 재계산 요청과 표시 요청만 latest-wins 대상으로 삼는다.
- platform runtime queue의 presentation lane은 요청마다 closure나 입력 snapshot을 queue에 넣지 않는다. host가 소유한 최신 environment/presentation state 하나와 dirty marker 하나만 사용한다. 현재 작업이 끝나면 executor queue tail에 drain 작업을 최대 하나 예약한다. 새 요청이 계속 들어오면 중간 revision 계산을 생략할 수 있고 처리 지연의 상한은 보장하지 않지만, 요청이 멈춘 뒤 마지막 revision과 입력은 반드시 적용한다.
- platform render queue의 surface lane은 callback별 생성·resize·destroy closure를 보관하지 않는다. 현재 surface generation, surface 수명·format·크기, renderer generation, 최신 draw 요청을 canonical state로 유지하고 idempotent reconciler 하나가 이를 따라잡는다. renderer 생성 도중 더 최신 generation이 오면 생성 결과를 파괴하고 최신 generation을 재조정한다. 각 lane은 실행 하나와 대기 중 drain 작업 최대 하나만 허용한다. renderer 생성/configure가 현재 generation에서 실패하면 오류를 남기고 자동 재시도 loop를 만들지 않는다. 새 surface 입력 또는 검증 화면의 명시적 retry가 있을 때 다시 시도한다.
- lane의 복구 가능한 작업 예외나 underlying executor 예약 실패는 scheduled/dirty 상태를 복구하고 오류를 기록한다. `OutOfMemoryError` 등 프로세스를 안전하게 계속할 수 없는 fatal 오류를 삼키지 않는다. 조정기는 복구 가능한 실패 뒤 고착되지 않으며, 실패를 main thread 실행으로 우회하지 않는다.
- Android `surfaceDestroyed`는 surface generation을 먼저 무효화하고 새 draw를 닫은 뒤 reconciler가 해당 surface를 더 이상 쓰지 않고 renderer를 파괴할 때까지 반환하지 않는다. 올바른 surface 수명 보장을 위해 이 대기에는 timeout 후 surface를 사용 중인 채 callback을 반환하는 경로를 두지 않는다. 대기 시간은 기록한다. 따라서 GPU/driver 호출 자체가 멈추면 callback 대기도 길어질 수 있다는 한계를 명시한다. Activity 종료는 runtime 작업 drain을 먼저 끝내고 render queue의 종료 조정이 renderer 파괴와 host 해제를 마친 뒤 반환한다. 이미 GPU 호출에 들어간 작업은 강제 취소하지 않는다.
- iOS는 UIKit 및 `CAMetalLayer` 접근을 main thread로 유지한다. runtime presentation 및 render draw 요청은 각각 latest-only lane을 거친다. UIKit surface configure callback은 동시에 하나만 대기시키며, 대기 중 layout 변경은 canonical state에 합치고 callback이 돌아온 뒤 최신 generation만 configure한다. 종료는 새 lane 요청을 닫고 실행 중 작업이 끝난 뒤 renderer/host를 해제한다.
- 두 platform 조정기는 상태 잠금을 잡은 채 V8/FFI/WGPU 호출, executor wait, main-thread callback 대기를 하지 않는다. Android의 destroy latch는 renderer 조정 완료 경계에서 항상 `finally`로 깨운다. iOS 메인 스레드는 render queue drain을 동기 대기하지 않는다.
- executor의 종료/예약 실패는 화면 상태에 오류로 남기며 main thread 동기 fallback을 하지 않는다. renderer 실패 뒤 이전 generation을 유효한 것으로 가장하지 않는다.
- draw 실패 주입은 내부 검증 빌드에서만 켠다. WGPU renderer의 draw 경계에서 다음 한 번의 draw를 명시적으로 실패시켜 nonzero 오류가 FFI와 플랫폼 상태까지 전달되는지 확인한다. 실패는 JS·DOM·CSS snapshot을 바꾸지 않는다. 실패 주입을 소비한 뒤 다음 일반 draw가 성공하는지 확인한다. 이 검증은 실제 GPU/OS 고장을 복구한다는 주장이 아니라 오류 전파와 renderer 재사용 경로를 확인한다.
- 포화 검증은 각 플랫폼 lane의 실행을 latch로 멈춘 동안 최소 10,000개 요청을 제출하고, 대기 중인 drain 수가 1을 넘지 않으며 재개 뒤 가장 최신 viewport/scheme/generation만 최종 적용되는지 확인한다. consumer 예외·executor 거부 뒤 재사용, pending draw 중 종료, surface generation 폭주, 현재 generation의 생성 실패 뒤 자동 retry loop 부재, draw 실패 직후 정상 draw, 종료 뒤 callback 부재, JS 작업 순서 보존을 별도로 검증한다.

## 구현 전 비교 모델

- surface 크기가 바뀌는 시나리오는 기존 고정 `301×100 CSS px` 기준과 분리해 비교한다. 새 resize fixture의 root는 `width:100vw;height:100vh`를 사용하며 Chromium `154.0.8037.98`에서 `301×100`과 `341×128 CSS px` 두 viewport를 각각 관찰한다. 각 크기의 computed width/height와 모든 node frame을 별도 저장하고, 색상·display 결과는 기존 기준과 동일해야 한다. 예상 root frame은 각각 `(0,0,301,100)`과 `(0,0,341,128)`이며 자식의 고정 크기와 Flex 위치는 두 크기에서 유지된다. reference는 기존 C04.10 기준 파일을 덮어쓰지 않고 별도 파일로 보존한다.
- 플랫폼 경계에서 Android `SurfaceHolder` pixel 크기는 현재 display density로 나눠 CSS viewport를 만들고, iOS `UIView.bounds` point 크기를 CSS viewport로 사용한다. WGPU drawable 크기는 Android pixel 크기·iOS point×scale로 별도 전달한다. 크기·scale·scheme 입력은 runtime executor에 불변 snapshot으로 전달하며, 뒤늦게 도착한 이전 surface generation의 resize/environment 작업은 무시한다. 0 또는 음수 크기는 기존 유효 입력과 revision을 유지한다.
- C04.10 개발 화면에 `301×100`과 `341×128` 표면 크기를 전환하는 검증 제어를 둔다. Android는 에뮬레이터 입력으로 버튼을 누르고, iOS는 `--spinon-c0410-auto-resize` 실행 인자로 `301→341→301→341` 3회 전환해 재현한다. 두 플랫폼에서 surface 크기, CSS viewport, environment revision, layout frame, render scene frame, draw 완료를 함께 기록한다. 왕복 전환 뒤 기준 화면과 revision 증가를 확인한다. 제어와 실행 인자는 내부 검증 화면에만 두며 사용자 앱 API로 취급하지 않는다.
- 기준 브라우저는 고정 Chromium `154.0.8037.98`이다. 새 HTML/inline CSS 입력, 노드 ID 매핑, viewport·device scale·scheme·pointer 환경과 Chromium 실행 파일/fixture hash를 코드 구현 전에 기존 `spec/internal/evidence/c04-runtime-css-to-gpu-precomparison-2026-10-09.md`와 추가 [resize 비교 기준](../spec/internal/evidence/c04-runtime-css-to-gpu-resize-precomparison-2026-10-09.md) 및 고정 JSON에 기록한다.
- Android·iOS 앱은 HTML을 읽지 않고 `tests/fixtures/css/c04/runtime-css-to-gpu-resize.js` 하나를 실제 V8에 전달한다. Chromium HTML은 그 JS 입력과 동일한 노드 순서와 인라인 스타일을 담으며, 정적 검증이 native JNI/Swift 호출 경로가 이 단일 스크립트를 쓰는지 확인한다. 기존 `runtime-css-to-gpu.js`와 V1 reference는 초기 고정 크기 기준으로 보존한다.
- 최소 fixture는 한 root와 자식 3개, 명시 inline `display`·크기·Flex 배치·opaque `background-color`, transparent 자식, `display:none` 자식을 포함한다. 텍스트 노드·positioning·overlap은 두지 않는다.
- 비교는 세 가지를 따로 판정한다: Stylo computed `background-color` 문자열은 Chromium 문자열과 정확 비교, Taffy 각 표시 요소의 `x/y/width/height`는 각 항목별 최대 절대 오차 0.5 CSS px, 같은 draw pass용 offscreen readback의 요소 내부 중심 표본은 기대 sRGB RGB 값과 채널별 정확 비교한다. surface screenshot은 실제 GPU 화면 제출을 증명하는 별도 시각 근거이며 pixel-exact color oracle로 사용하지 않는다. 가장자리 샘플은 반올림·안티앨리어싱 때문에 색 판정에서 제외한다.
- 동일 JS 입력은 Android API 37 ARM64 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 실제 V8 세션을 거친다. 실제 제품 GPU 화면 screenshot이 있어야 하고, Rust unit test·offscreen readback만으로 화면 동작을 완료 처리하지 않는다.
- 시뮬레이터 결과는 실기기·하드웨어 GPU·성능 증거가 아니다. 이 작업에서 성능 우위나 전체 CSS 호환을 주장하지 않는다.

## 완료 조건

1. `RuntimeRenderSnapshot`이 전체 revision tuple, DOM preorder, frame와 paint를 동일한 current key로 보존하고 잘못되거나 stale한 입력은 장면 전체를 거부한다.
2. 이전 C04.8/C04.9 JSON과 S04 static fixture의 출력·profile·미지원 범위가 회귀하지 않는다. `background-color` 허용은 새 runtime paint profile에만 적용한다.
3. WGPU product crate가 runtime 장면을 CPU geometry와 실제 surface draw로 처리한다. 노드별 Android/iOS native view는 만들지 않는다.
4. Android와 iOS 시뮬레이터에서 같은 고정 JavaScript 입력이 실제 V8 DOM/CSS를 거쳐 WGPU 화면에 보인다. Chromium computed style/geometry와 offscreen pixel readback을 각각 대조하고, surface screenshot은 실제 제출 확인에 사용한다.
5. 실제 surface 생성·수명·format·크기 변경·해제에서 platform surface generation이 증가하고, OS color scheme 변경에서 environment revision이 증가한다. 크기 변경 뒤 native surface, WGPU 획득 texture, runtime scene의 CSS viewport/frame이 모두 현재 값인지 캡처와 로그로 맞춘다. 각 sequence 검사 전에 무효화된 장면은 제출되지 않는지 확인하고, 이미 present에 들어간 프레임 한 장이 완료될 수 있는 경계도 확인한다. 각 latest-only lane은 실행 중 1개와 대기 drain 1개를 넘지 않으며 최소 10,000개 입력 뒤 가장 최신 요청을 적용한다. JavaScript 평가·DOM 변경은 입력 순서를 보존한다. Android `surfaceDestroyed` callback은 renderer destroy/drain 전 반환하지 않으며 실제 대기 시간을 기록한다. pending draw 종료, surface generation 폭주, consumer/executor 실패 후 조정기 회복, 현재 generation 생성 실패의 자동 재시도 loop 부재, draw 실패 전달과 다음 draw 성공, 종료 뒤 callback 부재, 빈 root도 검증한다.
6. 내부 계약 `0.1.0`을 유지하고, C04.10 및 S04 연결 상태·API/내부 계약·실행 근거·Tailscale CSS 계획과 roadmap을 동기화한다.

## 구현 순서

1. 계획에 대한 서로 다른 실패 관점 20개 검토를 기록하고 지적을 반영한다.
2. 고정 Chromium precomparison HTML·CSS·computed style·geometry·색 표본과 두 surface 크기의 viewport reference를 저장하고 hash 및 오차 기준을 구현 전에 잠근다.
3. `spinon-render`에 runtime scene 자료형을 추가하고 `spinon-style-to-render`에서 같은 style/layout key의 HostDocument snapshot을 검증해 장면으로 변환한다.
4. 새 `RuntimeFlexPaintV1` 결과에서 style과 layout의 완료 snapshot을 단일 key로 함께 읽는 내부 경로를 추가한다. 기존 C04.9 strict layout state와 readback은 보존한다. public mutable reference나 FFI 수명 연장은 만들지 않는다.
5. WGPU surface·geometry·draw 코드를 `spinon-render-wgpu` 제품 crate로 분리한다. 기존 정적 fixture 경로를 새 crate 소비자로 바꾸고 기존 fixture 증거를 유지한다.
6. `spinon-ffi`에 별도 renderer handle 및 runtime 장면 제출 함수를 연결한다. 기존 RuntimeSession pointer와 JSON readback 계약은 유지한다.
7. Android·iOS에 `SpinonRuntimeGpuHost` 조정 객체를 연결한다. eval·환경 갱신은 platform background executor를 통해 기존 RuntimeSession worker로 전달한다. 환경·표시 요청은 latest-only admission을, JavaScript 평가는 무손실 순서 보존 queue를 쓴다. Android renderer 생성·표시·surface 수명 조정은 canonical surface state를 따라가는 하나의 coalescing reconciler에서 처리하고 크기 변경 때 같은 host에서 renderer를 파괴·재생성한다. `surfaceDestroyed` callback은 renderer drain 뒤 반환한다. iOS는 UIKit surface 준비·`CAMetalLayer` configure·resize configure를 메인 스레드에서 실행하며, surface configure 전후 render queue barrier를 둬 draw와 직렬화한다. adapter·device·pipeline 초기화와 draw·파괴는 전용 render queue에서 처리한다. iOS의 runtime update와 draw도 각 latest-only lane을 통과한다. 불변 snapshot revision과 surface generation admission을 검증하고 surface screenshot과 별도 offscreen readback을 수집한다.
8. 내부 전용 draw failure hook을 빌드 격리해 nonzero 오류 전달, snapshot 불변, 다음 일반 draw 성공을 검증한다. Android·iOS lane stress test는 latch가 실행을 막은 동안 최소 10,000개 요청을 넣고 queue bound, 최신값 반영, close/drop, 종료 동작을 확인한다.
9. Rust/Bun 검증과 Android·iOS 시뮬레이터의 실화면 실행을 수행한다. 기능 구현 뒤 새 실패 관점 20개로 별도 검토하고, PR 전체를 다시 20개 관점으로 검토한다.
10. 내부 계약·상태 대장·Tailnet preview를 동기화하고 GitHub PR에는 두 플랫폼 화면 캡처를 첨부한다. PR은 rebase merge를 사용한다.

## 범위 밖

- 텍스트 shaping/측정/그리기, 이미지, SVG, alpha blending, gradients, borders, shadows, clipping, transform, scroll
- CSSOM·author stylesheet resource graph·외부 CSS URL loader
- React/Vue/Svelte host adapter, DOM event/onClick, accessibility tree, public rAF
- continuous animation frame queue, JS event queue와 displayed-frame hit-test
- OTA/HMR·전체 앱 번들·성능 benchmark·실기기 검증·전체 HTML/CSS 지원
