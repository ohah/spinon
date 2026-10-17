# C13.1 복합 stacking descendant clip runtime 근거

**날짜:** 2026-10-18 · **범위:** 고정 fixture runtime 전용 · **공식 C13.1/C13 상태:** 미완료 · **Tailnet 읽기 전용 집계:** `74/207` (Agent Browser 확인, 상위 78 + 세부 ID 127 + 무ID 하위 2)

## 검증 대상

[계획과 Chrome 155 oracle](../c13-1-stacking-descendant-clip-plan-2026-10-18/README.md)의 `visible`, `clip`, `hidden` 3개 case를 기존 C13.1 제한 profile fixture host에 연결했다. 13개 고정 frame과 case당 여섯 표본(총 18개)의 topmost hit identity·opaque RGBA를 각각 비교한다. 핵심 경계는 clip을 벗어나는 내부 높은 `z-index` child와 외부 sibling overlay다. 색 비교가 hit identity를 대신하지 않는다.

구현은 [runtime fixture adapter](../../../../crates/spinon-ffi/src/runtime_gpu/fixtures/c13_1_stacking_descendant_clip.rs), [FFI adapter](../../../../crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c13_1.rs), Android JNI/Java app route 및 iOS Swift/Metal capture route에 있다. 고정 입력, viewport/DPR, screenshot·observation·sample hash를 Rust adapter가 검증한다.

## Android API 37 ARM64 AVD

실행 대상은 `emulator-5554` 한 대, API 37 ARM64 AVD와 ANGLE/Vulkan/SwiftShader software renderer다. 각 행은 다른 app process와 run ID다. Android SurfaceView `PixelCopy`는 `1418×336` drawable에서 고정 18점을 읽었다.

| run | PID | run ID | frame/hit | PixelCopy |
| --- | ---: | --- | ---: | ---: |
| 01 | 11022 | `20c6306b5dd6` | 13/13 · 18/18 | 18/18 exact |
| 02 | 11108 | `20c7faf7f6a5` | 13/13 · 18/18 | 18/18 exact |
| 03 | 11165 | `20c9eddfb0a9` | 13/13 · 18/18 | 18/18 exact |

세 run의 evaluation/draw identity는 `(generation=1, document_revision=65, render_tree_revision=13, style_revision=0)`으로 같고, Android surface generation `2`, presentation sequence `5`를 draw report와 PixelCopy에 연결했다. 세 PNG의 SHA-256은 `ab406960a56890e9bf0cc71ff2dfdd84d77323bce218fc52ab7c163234c44566`이다. [세 화면](android-guard-run-01-screen.png) · [PixelCopy PNG](android-guard-run-03-pixelcopy.png) · [세 run 원본 로그](android-guard-run-01.log) · [세 run 전체 logcat](android-guard-run-03-logcat-all.log).

## iOS Simulator

iPhone 17 Pro / iOS 26.2 Simulator UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`의 서로 다른 세 app process에서 확인했다. 각 실행은 13/13 frame, 18/18 hit identity, 18개 Metal drawable 표본을 통과했다. surface는 `1620×384`, `Bgra8Unorm`, generation 1이며 표본 파일에는 RGBA8 순서로 기록했다. readback은 OS compositor 제출 전 drawable에서 수행했다.

| run | PID | run ID | eval/draw revision |
| --- | ---: | --- | --- |
| 01 | 80211 | `d7ccb239f2864ec4999058863f11a0ff` | `(1,65,13,0)` |
| 02 | 80392 | `44a26937455d46468551ff5dc939608c` | `(1,65,13,0)` |
| 03 | 80797 | `a440653c88be4fb1a8aaf0d586ad965e` | `(1,65,13,0)` |

run 03의 [Metal sample JSONL](ios-guard-run-03-metal-samples.jsonl) SHA-256은 `aa53f276a215929a6f35a1ea6273bd626ccc051d8ee9171212ccb513adb6229e`이고 [manifest](ios-guard-run-03-metal-manifest.json)은 `status=complete`, `sampleCount=18`, process ID `80797`을 기록한다. [iOS 화면](ios-guard-run-03-screen.png)과 [세 원본 로그](ios-guard-run-01.log), [세 화면 로그](ios-guard-run-03.log)을 보존했다.

## negative controls와 수정 이력

- Android API 37 AVD에서 별도 PID `11542`, run ID `21538e187781`로 eval snapshot의 document revision `65` 대신 `66`을 기대하게 했다. Android app은 `stage=stale-revision`으로 거부하고 `surface_readback=not-requested`를 기록했다. [실행 로그](android-stale-revision.log) · [실행 결과](android-stale-revision-launch.txt).
- iPhone 17 Pro Simulator의 별도 PID `18795`도 같은 문서 revision 변이를 거부하고 Metal readback을 요청하지 않았다. [실행 로그](ios-stale-revision.log) · [실행 결과](ios-stale-revision-launch.txt).
- 양 플랫폼에서 여러 C13.1 route를 함께 선택한 기존 route-conflict negative control은 host 생성 전에 거부됐다. [Android](android-route-conflict.log) · [iOS](ios-route-conflict.log).
- 첫 adapter 부정 검사는 고정 sample row의 `case`/`name` 필드를 확인하지 않던 validator 결함을 찾았다. validator를 정확한 18행 순서 검증으로 고친 뒤 focused C13.1 test를 다시 통과시켰다.
- stale-revision probe를 처음 연결했을 때 플랫폼의 fail-closed route 검사에서 modifier extra가 거부됐다. stacking route와 함께 있을 때만 허용하고 route가 없으면 거부하도록 제한해 negative run을 재검증했다. ADB `-e`는 Boolean이 아니라 문자열 extra를 만들어 기본 화면이 실행됐으므로 final invocation은 `--ez`로 수정했다. 이는 ADB 전송 실패가 아니었다.

## 빌드와 Rust 검사

- Rust focused C13.1: `mise exec -- cargo test -p spinon-ffi --features c13-1-overflow-clip-fixture-test c13_1` — 14/14 통과.
- Rust workspace: `mise exec -- cargo test --workspace` — 종료 코드 0. [원본 로그](cargo-test-workspace.log).
- `mise exec -- cargo clippy --locked --workspace --all-targets -- -D warnings` 및 `mise exec -- cargo clippy --locked -p spinon-ffi --all-targets --features c13-1-overflow-clip-fixture-test -- -D warnings` — 둘 다 경고 없이 통과. [workspace](cargo-clippy-workspace.log) · [fixture feature](cargo-clippy-c13-1-fixture.log).
- Android fixture build: `ANDROID_HOME=/Users/yoonhb/Library/Android/sdk SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST=1 mise exec -- ./gradlew -PspinonC04RuntimeGpu=1 :app:assembleDebug` — 성공. [로그](android-build-stale-guard.log). C04는 environment variable이 아닌 Gradle property `-PspinonC04RuntimeGpu=1`로 지정한다.
- iOS fixture build: `SPINON_ENABLE_C04_RUNTIME_GPU=1 SPINON_ENABLE_C13_1_OVERFLOW_CLIP_FIXTURE_TEST=1 mise exec -- bun run build:ios-sim` — iOS Simulator arm64 성공. [로그](ios-build-stale-guard.log).
- [default/opt-in archive 검사](feature-boundary/README.md)는 fixture FFI symbol·payload가 기본 archive에서 빠지고 opt-in archive에 포함되는지 각각 확인한다.

## 완료 범위와 제한

이 근거는 지정 fixture와 고정 Chrome 155 oracle의 hit identity·18개 표면 색 표본만 다룬다. Android는 API 37 ARM64 AVD software renderer이고 iOS는 iOS 26.2 Simulator drawable이다. 실기기·hardware GPU·OS compositor/scanout, WPT, DOM event dispatch, CSSOM, 일반 CSS stacking·overflow 조합, C13.1/C13 제품 지원 완료는 검증하지 않았다. C13.1과 C13 상위 체크는 미완료다. Agent Browser에서 확인한 `74/207`은 상태 대장 항목의 읽기 전용 합계이며 제품 지원 API 완료 수가 아니다. 이 근거는 공식 상태 체크를 변경하지 않는다.
