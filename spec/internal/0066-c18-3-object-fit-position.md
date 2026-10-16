# 0066 · C18.3 정적 PNG `object-fit` / `object-position`

**계약 버전:** `0.1.0` · **구현:** `spinon-render`, `spinon-render-wgpu`, `spinon-runtime`, `spinon-style`, `spinon-ffi`, Android/iOS fixture adapters · **앱 작성자용 공개 API 아님**

## 책임과 profile

이 계약은 live V8 document의 HTML `img` computed style을 `RuntimeRenderImage`의 concrete object rect와 texture sampling에 연결하는 제한 profile을 정의한다. Taffy가 만든 element frame과 paint order는 유지한다. URL/network loader, 비동기 decode/replacement, SVG·다른 포맷, intrinsic/auto used size, border/padding/transform, alpha-bearing image surface, 공개 `HTMLImageElement`, 전체 CSS Images 적합성은 제공하지 않는다. C18 상위 항목 및 공개 image 지원은 미완료다.

구현 전 고정 oracle은 Agent Browser 0.31.1이 제어한 Chrome `155.0.8059.85`다. viewport는 `224×72 CSS px`, HTML fixture의 image는 12개, source는 `24×12` opaque RGBA8 PNG이며 encoded SHA-256은 `cc621c7a4be9345a8124b91ebcd5a6ff9bc3a3206ec59fe1228b24ba09cae454`다. DPR 1 사전 비교의 frame·computed value·19개 sample은 [기존 oracle](../evidence/c18-3-object-fit-precomparison/README.md)에, Android DPR 2.625 및 iOS DPR 3 screenshot은 [실행 근거](./evidence/c18-3-object-fit-position-implementation-2026-10-16/README.md)에 둔다.

profile은 `object-fit: fill | contain | cover | none | scale-down`, 물리 x/y축의 두 `<percentage>` `object-position`, `image-rendering: auto`만 허용한다. percentage는 finite이면 음수·100 초과를 허용하고 clamp하지 않는다. 키워드·길이·`calc()`·1/3/4항 문법과 다른 image-rendering 값은 오류로 거부한다.

## 계산과 clipping

입력 frame `(fx, fy, fw, fh)`와 natural image `(nw, nh)`에서 scale은 다음과 같다.

| `object-fit` | concrete object 크기 |
| --- | --- |
| `fill` | `(fw, fh)` |
| `contain` | `(nw·s, nh·s)`, `s=min(fw/nw, fh/nh)` |
| `cover` | `(nw·s, nh·s)`, `s=max(fw/nw, fh/nh)` |
| `none` | `(nw, nh)` |
| `scale-down` | contain 크기가 natural보다 작으면 contain, 아니면 natural |

각 축의 원점은 `frameOrigin + (frameSize - objectSize) × percentage / 100`이다. layout frame은 이 계산으로 바뀌지 않는다. draw geometry는 frame, concrete object 및 누적 ancestor clip의 교집합을 만든 뒤 그 교집합에서 object UV를 계산한다. 교집합이 비면 정점을 만들지 않는다. 0·비유한·overflow geometry, 잘못된 UV, 지원하지 않는 style 및 border 조합은 draw 제출 전에 거부한다.

## 이미지 자원·revision 소유권

`RuntimeImageResourceKey`는 document generation, document revision, render-tree revision, intrinsic-size revision을 묶는다. `RuntimeRenderImage`는 NodeId, 자연 크기, caller-owned `Arc<[u8]>` RGBA8, optional object style과 style revision을 보유한다. 생성은 양의 dimensions와 checked `width × height × 4` byte length를 요구한다. snapshot 결합은 image node·source key·style revision을 현재 render key와 대조한다. GPU cache identity에는 source key, node, 크기, RGBA hash 및 sampling mode가 포함된다.

C18.3의 `auto` 경로는 `Rgba8Unorm` texture에서 원본 8-bit encoded channel을 읽고, 네 texel의 bilinear weight를 적용한 뒤 8-bit 결과를 ties-to-even으로 양자화한다. 결과는 Chrome 155의 DPR별 고정 screenshot sample과 Android/iOS Simulator 표면에서 비교한다. 기존 C18.1/2 natural-size 경로는 object style이 없을 때 기존 sampling mode를 유지한다. 이 exact profile은 Android `Rgba8Unorm` PixelCopy와 iOS `Bgra8Unorm` Metal drawable에서 확인했으며 모든 GPU/surface format 또는 hardware GPU 색상 일치를 뜻하지 않는다.

## Fixture-only C ABI

선언·구현·플랫폼 연결은 `c18-3-object-fit-position-fixture-test`와 `SPINON_ENABLE_C18_3_OBJECT_FIT_POSITION_FIXTURE_TEST` opt-in에서만 노출한다. 이 feature는 필요한 C04.10 runtime GPU fixture 경로와 함께 빌드해야 한다.

```c
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c18_3_object_fit_position_fixture(
    char *output, size_t output_capacity);
int32_t spinon_runtime_gpu_host_eval_c18_3_object_fit_position_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
int32_t spinon_runtime_gpu_host_request_c18_3_surface_pixels_for_test(
    SpinonRuntimeGpuHost *host, const uint32_t *sample_xy, size_t sample_count,
    char *output, size_t output_capacity);
void spinon_runtime_gpu_host_free(SpinonRuntimeGpuHost *host);
```

생성은 고정 12-image live document 및 C18.3 host를 만들고 초기 report를 caller buffer에 쓴다. `eval`은 같은 host에서 fixture JS를 실행해 computed style, frame, object rect와 revision report를 만든다. 표면 표본 요청은 정확히 19개의 고유 `(x,y)` 쌍을 interleaved `u32` 38개로 받아 다음 draw에서 읽도록 예약한다. output은 caller가 `output_capacity`만큼 쓰기 가능하게 유지한다. host는 살아 있어야 하며 같은 host의 호출은 직렬화한다. 종료 후 호출자가 공통 `spinon_runtime_gpu_host_free`로 해제한다. 포인터의 실제 메모리 범위와 수명은 호출자가 보장한다.

Android app route는 `spinon_c183_object_fit_position=true`, iOS route는 `--spinon-c183-object-fit-position` 인자다. platform adapter와 고정 PNG/JavaScript payload도 fixture-test source set·compile define에 묶는다. 기본 및 제품 feature artifact에는 fixture ABI·문자열·PNG bytes가 없어야 한다. 실제 Cargo staticlib, Android APK native library와 iOS app executable의 default/opt-in 대조는 [artifact report](./evidence/c18-3-object-fit-position-implementation-2026-10-16/artifacts/README.md)에 있다.

## 실패 동작

| 조건 | 동작 |
| --- | --- |
| output/host/좌표 null, output capacity 0, 표본 수가 19가 아님, 좌표 중복 | argument error를 반환하고 표면 readback을 예약하지 않는다. |
| image 누락·중복, 다른 ID/source, computed value 차이, natural dimensions 차이 | fixture attachment를 실패시키고 scene을 제출하지 않는다. |
| stale document/style/intrinsic revision 또는 frame mismatch | revision-bound scene 결합을 거부한다. |
| unsupported value, invalid size, non-finite/overflow geometry 또는 UV | 구체 오류로 거부하고 성공 표본을 갱신하지 않는다. |
| WGPU texture/pipeline validation, allocation, surface/readback 실패 | 해당 실행 표본을 실패 처리하고 이전 run 값을 재사용하지 않는다. |
| Android PixelCopy 또는 iOS Metal sample 불일치 | 앱 fixture가 failure 상태를 표시하고 exact comparator를 실패시킨다. |

## 검증 결과와 한계

Android API 37 ARM64 AVD는 density `2.625`, surface `588×189`, PixelCopy `19/19` exact다. iPhone 17 Pro / iOS 26.2 Simulator는 scale `3`, Metal drawable `672×216`, `19/19` exact다. 같은 빌드에서 Android C18.1 `9/9`, C18.2 `10/10`, iOS C18.1 `9/9`, C18.2 `10/10`도 통과했다. C18.1의 parent border 바깥 overflow pixel과 clip·hidden control 표본도 양 플랫폼에서 확인했다. 각 실행의 raw log, PNG screenshot 및 oracle screenshot hash는 implementation evidence에 기록한다.

focused render/FFI tests, workspace tests, Clippy, fixture module-boundary 검사, 기본/opt-in artifact 검사와 한 개 geometry assertion 변경 음성 대조가 통과했다. 음성 대조에서는 기대 y 좌표를 14에서 15로 바꾼 테스트가 실패한 뒤 원래 oracle로 복구되어 통과했다. 전체 `test:css-reference`는 C11/C13 기존 정적 assertion 13개가 현재 AppDelegate 경로와 맞지 않아 실패했으며, C18 fixture module-boundary 파일은 별도 실행에서 5/5 통과했다. 미실행 또는 실패 범위를 C18.3 완료 증거로 확대하지 않는다.

검증은 지정된 Android AVD와 iOS Simulator에 한정한다. 실기기, hardware GPU, iOS OS compositor 이후 pixel, 모든 density/surface color space, 다른 브라우저·이미지 filter profile, CSS Images 전체, C14/C18/U05 전체 및 공개 image 지원은 이 계약이 보장하지 않는다.
