# 0064 · C18.1 static PNG image paint와 overflow pixel

**계약 버전:** `0.1.0-draft` · **구현:** `spinon-image`, `spinon-render`, `spinon-render-wgpu`, `spinon-ffi` · **앱 작성자용 공개 API 아님

## 책임 경계

이 내부 계약은 고정 fixture PNG의 caller-owned decode bytes를 revision-bound render snapshot에 연결하고 WGPU에 정적으로 그리는 제한 profile을 정의한다. 플랫폼 앱에서 URL을 해석하거나 network/file bytes를 읽는 image resource loader는 제공하지 않는다. 제품 API나 전체 C18 CSS 지원을 약속하지 않는다.

기준 입력은 [C18.1 계획](../../plan/c18-1-static-image-paint-overflow.md), 고정 Chrome pixel oracle은 [사전 비교](./evidence/c18-1-image-paint-precomparison/README.md), 앱 실행 근거는 [Android API 37 AVD](./evidence/c18-1-image-paint-android-api37-2026-10-16/README.md)와 [iPhone 17 Pro Simulator](./evidence/c18-1-image-paint-ios-simulator-2026-10-16/README.md)다. 기본/opt-in artifact 비교는 [artifact boundary](./evidence/c18-1-image-paint-artifact-boundary-2026-10-16/README.md)에 기록한다.

## 이미지 입력과 snapshot

```text
live V8 HostDocumentSnapshot + connected <img src>
              │ ImageRequest::from_document
fixed caller-owned PNG bytes ─ decode ─ ImageResourceCompletion
              │ ImageResourceSnapshot(document, intrinsic revision)
              ├─ C14 image layout input
              └─ C18.1 fixture: RuntimeRenderImage(node, source key, size, RGBA8)
                         │ RuntimeRenderSnapshot::with_images
                         └─ WGPU image pipeline → same frame command stream
```

`RuntimeImageResourceKey`는 문서 generation, document revision, render-tree revision 및 intrinsic-size revision을 함께 묶는다. `RuntimeRenderImage`는 NodeId, 이 key, 양의 natural width/height와 caller-owned RGBA8를 보존한다. 생성 시 checked `width × height × 4`가 실제 byte 길이와 같은지 검사한다. `RuntimeRenderSnapshot::with_images`는 중복 NodeId, 현재 render key와 다른 source key, snapshot에 없는 node 및 layout frame과 다른 natural size를 거부한다. image node의 칠해지는 border side가 있으면 draw 순서에서 image가 border를 덮을 수 있으므로 `UnsupportedImageBorder`로 연결을 거부한다. 이미지가 없는 snapshot의 동작은 기존 경로와 같다.

이미지 frame이 붙은 snapshot은 다른 document revision으로 재발행하지 않는다. 바뀐 document에는 새 image request, decode completion과 resource snapshot이 필요하다. GPU cache는 renderer/device 인스턴스 안에 있으며 source key, NodeId, size와 RGBA hash로 항목을 식별한다. renderer를 교체하면 이전 device의 GPU image를 재사용하지 않는다.

## Paint profile

현재 profile은 C18.1 고정 fixture의 자연 크기 `24×16` PNG 두 장에 한정한다. fixture는 같은 PNG bytes를 서로 다른 `img[src]` 요청 identity로 연결한다. 첫 이미지는 visible parent 오른쪽 border를 덮고 border box 바깥까지 그린다. 둘째 이미지는 hidden parent의 padding edge에서 clip된다. background, image, border와 descendant command는 같은 source-order stream에 놓인다.

WGPU는 RGBA8 sRGB texture와 nearest/clamp sampler를 사용한다. 이 profile은 alpha channel의 모든 값이 255인 image만 허용하며, viewport sampling을 exact 하게 유지하기 위해 image frame은 decoded natural dimensions와 일치해야 한다. 고정 fixture는 image 자체의 border/padding/transform을 주지 않는다. runtime render box는 padding과 transform을 표현하지 않으므로 일반 호출자의 이 style 조합을 검사한다고 보장하지 않는다. scaled filtering, `object-fit`/`object-position`, alpha blend, SVG·다른 format, background image는 이 계약에 포함되지 않는다.

| 항목 | 내부 계약 |
| --- | --- |
| source stamp | generation + document + render-tree + intrinsic-size revision이 현재 key와 exact 일치 |
| image identity | 각 live `img` NodeId는 한 번만 나타나고 resource 집합과 document의 예상 set이 같음 |
| pixel input | tightly packed RGBA8, positive dimensions, checked `width × height × 4` |
| frame | CSS px natural-size frame; fixture는 `24×16` |
| sampling | nearest, clamp-to-edge |
| opacity | RGBA alpha가 모든 pixel에서 255 |
| image border | 칠해지는 border side가 있으면 snapshot attachment 단계에서 거부 |
| clipping | ancestor clip chain을 image geometry에 적용; `hidden` clip edge는 fixture에서 padding edge |
| lifetime | immutable runtime snapshot과 renderer/device 소유 GPU texture/cache |

## Opt-in fixture FFI

아래 함수와 C 선언은 `c18-1-image-paint-fixture-test` 및 `SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST` 조건에서만 컴파일된다. Android/iOS 기본 app route와 adapter도 기본 archive에는 포함하지 않는다.

```c
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c18_1_image_paint_fixture(
    char *output, size_t output_capacity);
int32_t spinon_runtime_gpu_host_eval_c18_1_image_paint_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
int32_t spinon_runtime_gpu_host_request_c18_1_surface_pixels_for_test(
    SpinonRuntimeGpuHost *host, const uint32_t *sample_xy, size_t sample_count,
    char *output, size_t output_capacity);
```

Host 생성 결과는 호출자 소유의 불투명 handle이다. `eval`과 pixel-readback 요청은 살아 있는 해당 fixture host에 대해 background executor에서 순서대로 호출한다. 같은 handle의 동시 호출·호출 중 해제는 금지한다. 각 output은 호출자가 제공한 `output_capacity` bytes를 쓰기 가능하게 유지해야 한다. surface 표본 요청은 정확히 9개의 서로 다른 `(x,y)` 좌표, 즉 읽기 가능한 `u32` 18개를 요구한다. null pointer, output capacity 0, 표본 개수 오류, 좌표 중복은 GPU readback 전에 argument failure로 반환한다.

ABI는 fixture-test 도구 전용이며 C header 선언, 구현, 플랫폼 native wrapper가 같은 build define을 사용해야 한다. 고정 PNG byte와 JS source는 Cargo feature 아래에 둔다. 기본/제품 artifact에는 이를 노출하는 FFI symbol·route가 없어야 한다. 실제 기본·opt-in staticlib 및 앱 archive search 결과는 [artifact evidence](./evidence/c18-1-image-paint-artifact-boundary-2026-10-16/README.md)를 따른다.

## 오류와 실패 동작

| 실패 | 결과 |
| --- | --- |
| 잘못된 RGBA byte 길이, 0 dimensions, frame mismatch | image snapshot 또는 WGPU 준비를 거부하고 frame submit 전에 오류 반환 |
| 칠해지는 image element border | `UnsupportedImageBorder`로 snapshot 결합을 거부 |
| image NodeId 누락·중복, 현재 key와 source revision 불일치 | snapshot 결합을 거부; stale image를 새 document에 붙이지 않음 |
| 비불투명 RGBA 또는 texture dimension 한계 초과 | 현재 제한 paint profile 오류; 불완전 frame을 성공 발표하지 않음 |
| WGPU pipeline·texture·buffer validation 또는 allocation error | validation/OOM error scope를 회수하고 구체 실패 report 반환 |
| FFI null host/coordinates/output, 잘못된 sample count 또는 중복 좌표 | `ERR_ARGUMENT` 계열 status와 output 진단; readback을 예약하지 않음 |
| 표면 acquire, device/surface generation 또는 readback failure | 해당 frame 증거를 무효화; 이전 성공 표본을 현재 run에 재사용하지 않음 |

이 profile은 실제 OS OOM이나 GPU device loss를 유도했다는 뜻이 아니다. 고정 fixture의 보고 가능한 validation/입력 거부와 simulator readback 성공을 분리해 기록한다.

## 비교 기준과 플랫폼 관찰

Chrome 155.0.8059.40 DPR 1, viewport `80×40 CSS px`의 9개 고정 RGBA sample을 비교 기준으로 둔다. 표본은 visible content, parent border overpaint, border 바깥 첫·중간·마지막 image pixel, image 뒤 배경, hidden padding 안쪽, hidden border 및 clip 바깥이다. 자세한 좌표와 기대 RGBA는 [계획](../../plan/c18-1-static-image-paint-overflow.md#고정-입력과-독립-oracle)을 따른다.

Android API 37 ARM64 AVD에서는 SurfaceView `PixelCopy` surface PNG와 전체 화면 emulator capture를 별도로 비교했다. iOS iPhone 17 Pro/iOS 26.2 Simulator에서는 Metal drawable readback과 full-screen Simulator capture를 나눠 비교했다. 두 플랫폼 모두 drawable/surface 9개와 screenshot 9개가 exact했고 screenshot fixture 밖의 네 anchor도 일치했다. Android는 ANGLE SwiftShader virtual GPU이고 iOS는 Simulator GPU다. 이 결과는 실기기·제품 하드웨어 GPU·OS display controller의 물리 scanout pixel 검증으로 확대하지 않는다.

## 현재 한계

이 계약의 검증은 고정 opaque natural-size PNG, live V8 fixture document와 Android/iOS Simulator 표면에 한정한다. URL resolver, file/network loader, cache policy, asynchronous decode/replacement/re-layout, image request cancellation, animated image, alpha blending, resampling, `object-fit`/`object-position`, arbitrary DOM image set, 공개 `HTMLImageElement`, 전체 C14·C18·U05 지원과 전체 CSS conformance를 판정하지 않는다. 구현과 재현 방법은 [C18.1 계획](../../plan/c18-1-static-image-paint-overflow.md)에, 현재 프로젝트 완료 범위는 [상태 대장](../STATUS.md)에 둔다.
