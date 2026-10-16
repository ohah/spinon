# 0065 · C18.2 static RGBA8 PNG alpha source-over

**계약 버전:** `0.1.0` · **구현:** `spinon-image`, `spinon-render`, `spinon-render-wgpu`, `spinon-ffi`, Android/iOS fixture adapters · **앱 작성자용 공개 API 아님

## 책임과 지원 범위

이 내부 계약은 fixture-owned 정적 PNG bytes 하나를 live V8 문서의 `<img>`에 연결하고, 불투명 CSS 배경 위에서 기본 source-over alpha 합성을 검증한다. C18.1의 opaque-only 검사를 대체하지 않으며, 고정된 C18.2 fixture route에서만 alpha 0·128·255 표본을 비교한다. 공개 `HTMLImageElement`, URL resolver, file/network loader, async decode/replacement, image opacity, group isolation, blend mode, `object-fit`/`object-position`, scaled sampling, 비-RGBA 포맷은 포함하지 않는다.

재현 입력·Chrome oracle·결과는 [C18.2 계획](../../plan/c18-2-static-alpha-image-compositing.md), [Chrome 155 사전 비교](./evidence/c18-2-alpha-precomparison-2026-10-16/README.md), [Android API 37 AVD](./evidence/c18-2-alpha-image-android-api37-2026-10-16/README.md), [iPhone 17 Pro Simulator](./evidence/c18-2-alpha-image-ios-simulator-2026-10-16/README.md)에 둔다. 고정 fixture PNG의 encoded SHA-256은 `89852885917d6f22c6be237035a10447bfb85caa8c58d4ec29205ee6f652624e`, decoded RGBA SHA-256은 `782a6e59a5a87aa2faa90a6e04039ae5d3e23e7d46c29f525115e428f59a25c9`다.

## 이미지·compositing profile

입력은 `24×16` RGBA8 natural-size PNG 한 장이다. fixture의 CSS viewport는 `80×40`; `<img>` frame은 `(8,12) 24×16`이고 backdrop은 `#112233`이다. PNG는 세 수직 띠를 가진다.

| PNG 구간 | 원본 RGBA | backdrop 위 기대 결과 |
| --- | --- | --- |
| alpha 0 | `(250,20,210,0)` | `(17,34,51,255)` |
| alpha 128 | `(220,40,80,128)` | `(118,37,65,255)` |
| alpha 255 | `(32,200,64,255)` | `(32,200,64,255)` |

이 profile의 source-over는 straight-alpha 입력에서 `Cout = Cs × αs + Cd × (1 − αs)`, `αout = αs + αd × (1 − αs)`를 사용한다. 테스트 backdrop은 불투명하므로 표면 출력 alpha는 255다. C18.2는 sRGB PNG sample을 linear sample로 읽은 뒤 unorm color target에 기록할 때 sRGB 값으로 다시 인코딩한다. 8-bit source-over 경로는 premultiplied source contribution을 8-bit로 양자화한 뒤 WGPU `ALPHA_BLENDING`으로 destination과 합성해 Chrome 155 RGBA8 oracle과 비교한다. 허용 오차는 0이다.

Android API 37 ARM64 AVD에서 관측한 surface는 `Rgba8Unorm`, iOS Simulator는 `Bgra8Unorm`이며 둘 다 sRGB color space를 제공했다. renderer는 지원 capability에 UNorm surface가 있으면 sRGB texture 형식보다 우선 선택한다. sRGB surface만 선택된 첫 iOS 실행은 alpha-128 결과가 Chrome과 일치하지 않았다. 그 fallback 형식은 이 계약의 exact support 범위로 선언하지 않는다. 제품 surface format 전체의 보편적인 색상 일치를 뜻하지 않는다.

## Live document와 resource 소유권

검증 경로는 다음 순서를 유지한다.

```text
live V8 HostDocumentSnapshot
  → 고정 C18.2 CSS fixture 평가
  → `<img id="c18-alpha-image" src="spinon-fixture:c18-2/alpha-source-over">` 확인
  → caller-owned PNG decode / ImageResourceSnapshot
  → generation·document·render-tree·intrinsic-size key에 묶인 RuntimeRenderImage
  → image frame과 clip을 보존한 RuntimeRenderSnapshot
  → WGPU source-over draw
  → Android PixelCopy 또는 iOS Metal drawable 표본
```

Fixture attachment는 HTML namespace의 `img`가 정확히 하나이고 id와 source가 고정값과 같아야 한다. image resource는 같은 immutable document/render revision과 intrinsic-size revision에 묶인다. render snapshot은 NodeId, frame, RGBA byte length, positive dimensions, revision 및 image clip 검증을 유지한다. C18.2는 C18.1의 shared runtime/readback feature를 opt-in dependency로 사용하지만 PNG bytes, JS route, report와 ABI는 별도 C18.2 feature/module에서 공급한다. 기본 feature set에는 두 fixture가 포함되지 않는다.

## Fixture-only C ABI

선언·구현은 `c18-2-alpha-image-compositing-fixture-test` Cargo feature와 `SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST` native define 아래에서만 노출한다.

```c
SpinonRuntimeGpuHost *spinon_runtime_gpu_host_new_c18_2_alpha_image_fixture(
    char *output, size_t output_capacity);
int32_t spinon_runtime_gpu_host_eval_c18_2_alpha_image_fixture(
    SpinonRuntimeGpuHost *host, uint64_t layout_timeout_millis,
    char *output, size_t output_capacity);
int32_t spinon_runtime_gpu_host_request_c18_2_surface_pixels_for_test(
    SpinonRuntimeGpuHost *host, const uint32_t *sample_xy, size_t sample_count,
    char *output, size_t output_capacity);
void spinon_runtime_gpu_host_free(SpinonRuntimeGpuHost *host);
```

Host 생성은 caller-owned output buffer에 초기 report를 쓰고 불투명 host handle을 반환한다. `eval`은 고정 JavaScript fixture를 live V8 document에 적용하고 layout/render report를 반환한다. pixel 요청은 정확히 10개의 서로 다른 `(x,y)`를 interleaved `u32` 20개로 전달해 다음 draw의 표본으로 예약한다. 각 output buffer는 호출 중 `output_capacity` bytes만큼 writable해야 한다. host와 좌표 배열은 유효해야 하고, 같은 host의 호출은 직렬화하며 호출 중 host를 해제하지 않는다. 작업 종료 시 호출자가 `spinon_runtime_gpu_host_free`로 host를 해제한다. Android/iOS native wrapper와 앱 adapter는 이 수명·순서를 따르는 검증 전용 bridge다.

Android route는 `spinon_c182_alpha_image=true`, iOS route는 `--spinon-c182-alpha-image` opt-in argument를 사용한다. route와 C ABI는 기본 앱에서는 비활성화되고 fixture 검증 빌드에서만 연결한다. 실제 기본/opt-in Cargo staticlib, Android APK native library와 iOS app executable의 심볼·원본 JS·PNG byte 대조는 [artifact evidence](./evidence/c18-2-alpha-image-android-api37-2026-10-16/artifact-boundary/scan-results.txt), [Android APK 대조](./evidence/c18-2-alpha-image-android-api37-2026-10-16/artifact-boundary/android-app-boundary.txt), [iOS app 대조](./evidence/c18-2-alpha-image-ios-simulator-2026-10-16/app-artifact-boundary.txt)에 둔다.

## 오류 동작

| 조건 | 동작 |
| --- | --- |
| output이 null이거나 capacity가 0, null host/좌표, 표본 개수가 10이 아님 | `ERR_ARGUMENT` 계열 상태로 거부하고 drawable readback을 예약하지 않는다. host 생성은 null을 반환한다. |
| 좌표 중복 또는 drawable 범위 밖 좌표 | FFI 또는 renderer readback validation에서 거부하고 그 표본 집합을 현재 draw 결과로 승인하지 않는다. |
| image가 없거나 두 개 이상, 예상 id가 없거나 중복, source 변경 | document attachment에서 오류 report를 반환하고 image scene을 제출하지 않는다. |
| PNG decode, resource snapshot, layout frame 또는 revision이 유효하지 않음 | scene attachment를 실패시켜 이전 revision의 image를 새 scene에 재사용하지 않는다. |
| WGPU surface acquire, pipeline validation, allocation 또는 readback 실패 | 성공 표본을 현재 run으로 재사용하지 않으며 host/report가 실패 상태를 반환한다. |
| sample pixel이 Chrome 기대값과 다름 | 앱 화면에 failure 상태를 표시하고 exact comparator를 실패시킨다. |

임의의 잘못된 주소가 가리키는 C 포인터의 유효성은 FFI에서 판별할 수 없다. 호출자는 host, 좌표와 출력 메모리의 실제 범위·수명을 보장해야 한다. 이 fixture는 OS OOM·GPU device loss를 강제로 유도했다는 보증을 하지 않는다.

## 관찰 기준과 제한

Chrome `155.0.8059.85`, DPR 1의 고정 10개 CSS pixel 좌표를 Android AVD PixelCopy `10/10`, iOS Metal drawable `10/10`과 각각 대조했다. 동일 좌표를 전체 화면 screenshot에서도 `10/10` exact로 확인했고 fixture 바깥 제목·부제·상태·배경 anchor 네 점도 각 화면에서 확인했다. Android 표면은 `210×105`, scale `2.625`; iOS drawable은 `240×120`, scale `3`이다. iOS Metal readback은 OS compositor 제출 전이며 전체 화면 캡처는 별도 Simulator screenshot이다.

C18.1 opaque image·border overpaint·overflow·hidden clip 회귀는 두 플랫폼에서 drawable/surface `9/9`, 전체 화면 `9/9`, anchor `4/4`로 통과했다. Android 근거는 [C18.1 regression](./evidence/c18-2-alpha-image-android-api37-2026-10-16/c181-regression/README.md), iOS 근거는 [C18.1 regression](./evidence/c18-2-alpha-image-ios-simulator-2026-10-16/c181-regression/README.md)다.

이 evidence는 지정된 ANGLE SwiftShader Android AVD와 iOS Simulator GPU에 한정한다. Android 실기기·물리 display scanout, iOS 실기기·hardware GPU, surface UNorm을 지원하지 않는 기기, 다른 PNG sampling/color profile, 실제 앱 URL loader, 제품 image lifecycle, C14/C18/U05 전체 또는 CSS/WPT 적합성을 증명하지 않는다. 구현·계획을 포함해 C18.2는 내부 fixture slice이며 상위 C18과 공개 image 지원은 미완료다.
