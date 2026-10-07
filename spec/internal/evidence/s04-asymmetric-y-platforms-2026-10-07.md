# S04.8 비대칭 y 좌표와 GPU 표면 실행 근거

## 판정 범위

별도 `S04-asymmetric-y-v1` fixture의 computed style과 모든 노드 frame을 고정 Chromium reference에 비교했습니다. 서로 다른 세로 위치·높이·간격을 가진 frame의 CSS→NDC 변환을 독립 CPU oracle로 확인하고, 같은 fixture를 Android Vulkan 및 iOS Metal 시뮬레이터 표면에 제출했습니다. 플랫폼별 offscreen RGBA readback은 고정 36개 표본을 정확히 통과했습니다.

이는 정적 내부 fixture의 좌표·색상 경로를 검증합니다. 연속 렌더링, 전체 화면 pixel 비교, 터치·hit-test·DOM 이벤트, 제품 CSS 지원, GPU 표시 완료 callback, 성능, 실기기 또는 하드웨어 GPU 결과는 아닙니다.

## 사전 고정 기준

| 항목 | 기준 |
| --- | --- |
| Fixture | `S04-asymmetric-y-v1`, viewport `301×65 CSS px`, device scale factor 1 |
| CSS 배치 | 부모 column Flex, `row-gap: 3px`; 자식 높이 `12/18/24px`, 시작 y `0/15/36px` |
| Chromium | `154.0.8037.98`, headless offline capture |
| Reference | `s04-asymmetric-y-v1-chromium-154.0.8037.98-f7ffacb8763c-06ff4aab2ac9-ccffd5c5fe77` |
| Fixture / stylesheet SHA-256 | `f7ffacb8763c1d86367c86749659b45bc34960615f742842c06abad6b841dfad` / `06ff4aab2ac9c112d1b2dc1c81a71df183b4cdc2ead8e909cfeb020548cc3fc5` |
| Reference SHA-256 | `8cd484cf7022d3da12eeab317b9dfb10f05f010e6ddd0d9e9ee4273c295f8a3b` |
| Geometry 판정 | 모든 노드의 x/y/width/height 각각 Chromium 대비 최대 절대 오차 `0.5 CSS px` 이하. 평균으로 개별 좌표 오류를 상쇄하지 않음 |
| CPU→NDC oracle | viewport `100×80`, surface `240×180`, density `1.5`, 비대칭 x/y와 높이 3개; 각 vertex 좌표 오차 `< 1e-5` |
| GPU 표본 | x `[0,150,300]`, y `[0,11,12,14,15,32,33,35,36,59,60,64]`; 총 36개 RGBA8 표본을 정확 비교 |

Chromium reference는 [fixture JSON](../../../tests/fixtures/css/s04/asymmetric-y.v1.json), [CSS 입력](../../../tests/fixtures/css/s04/asymmetric-y.v1.css)과 함께 저장했습니다. capture 도구는 기존 `S04-flex-paint-v1` 출력 schema를 보존하며 새 비대칭 fixture에만 새 reference schema를 씁니다. 이미 존재하는 reference 경로는 덮어쓰지 않습니다.

## 실행 환경과 결과

| 플랫폼 | 시뮬레이터·표면 | 관찰 결과 |
| --- | --- | --- |
| Android | Android 16 / API 36, ARM64 `sdk_gphone64_arm64`; surface `1080×2400`, density `2.625` | wgpu Vulkan, `Cpu` adapter `llvmpipe (LLVM 21.1.4, 128 bits)`, `Rgba8UnormSrgb`; generation 1, acquire `Success`, present 요청, frame 1의 36/36 RGBA 표본 정확 일치 |
| iOS | iPhone 17 Pro / iOS 26.2 Simulator, ARM64; surface `1206×2622`, density `3.0` | wgpu Metal, `Bgra8UnormSrgb` 표면; generation 1, acquire `Success`, present 요청, frame 1의 36/36 RGBA 표본 정확 일치. 실행 로그에는 adapter 이름이 기록되지 않음 |

두 화면 캡처에서 동일한 빨강·파랑·초록 띠와 세로 간격이 보이며, 화면 바깥은 부모 배경색으로 채워집니다. 캡처의 제목과 상태 문구는 개발용 native label이고 색상 띠는 wgpu 표면입니다. 로그의 `present=requested`는 제출 요청일 뿐 실제 표시 완료 callback을 뜻하지 않습니다. Android의 같은 frame 보고서가 JNI와 Java 양쪽에서 각각 기록되어 Logcat에 두 줄로 보입니다. 이는 두 frame이 아니라 한 번의 응답을 두 경계에서 기록한 것입니다.

- Android 화면: [캡처 PNG](s04-asymmetric-y-android-2026-10-07.png)
- Android Logcat: [원본 로그](s04-asymmetric-y-android-2026-10-07.log)
- iOS 화면: [캡처 PNG](s04-asymmetric-y-ios-2026-10-07.png)
- iOS unified log: [원본 로그](s04-asymmetric-y-ios-2026-10-07.log)

## 재현 명령

고정 Chromium 기준을 별도 디렉터리에 다시 캡처합니다.

```sh
SPINON_REFERENCE_OUTPUT_DIR=/tmp/s04-asymmetric-y-reference mise exec -- bun run css:reference:s04-y
```

Rust·JavaScript 검증 및 플랫폼 fixture 빌드는 다음과 같습니다.

```sh
mise exec -- bun run test
mise exec -- cargo test --locked --manifest-path spikes/wgpu-backend/Cargo.toml --features s04-fixture
mise exec -- cargo clippy --locked --manifest-path spikes/wgpu-backend/Cargo.toml --features s04-fixture --all-targets -- -D warnings
mise exec -- cargo fmt --all -- --check
mise exec -- cargo fmt --manifest-path spikes/wgpu-backend/Cargo.toml --all -- --check
mise exec -- env SPINON_ENABLE_S04_ANDROID_FIXTURE=1 bun run build:android
SPINON_ENABLE_S04_IOS_FIXTURE=1 mise exec -- bun run build:ios-sim
```

두 플랫폼의 opt-in 빌드와 시뮬레이터 실행에서 `S04-asymmetric-y-v1` fixture를 확인했습니다. 실기기는 연결하거나 사용하지 않았습니다.

## 한계

- offscreen readback은 `301×65 Rgba8UnormSrgb`, 1280-byte padded row를 사용하며 전체 frame이 아닌 36개 고정 표본을 확인합니다.
- CSS geometry 비교와 CPU NDC oracle은 각각 별도 검사입니다. GPU surface 캡처는 색 띠의 실제 가시성을 보여 주지만 캡처 전체를 Chromium 이미지와 pixel 단위로 대조하지 않았습니다.
- Android 에뮬레이터의 adapter는 CPU `llvmpipe`입니다. Android 하드웨어 GPU, 기기별 backend 차이와 성능은 검증하지 않았습니다.
- iOS 결과는 시뮬레이터 Metal surface에 한정됩니다. 실기기 GPU, 회전·재부착 수명, 성능, 전체 화면 색 정확도는 미검증입니다.
- 이 fixture는 앱 runtime에 연결되지 않았고 일반 Flexbox·CSS 지원 범위를 넓히지 않습니다.
