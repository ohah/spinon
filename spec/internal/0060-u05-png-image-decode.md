# 0060 · U05.1 제한 PNG 디코드와 이미지 측정 입력

**계약 버전:** `0.1.0-draft` · **구현:** `spinon-image` · **앱 작성자용 공개 API 아님

## 책임 경계

`spinon-image`는 고정 버전 PNG decoder를 호출해 정적 PNG byte slice를 제한 RGBA8로 변환하고, decoder가 제공한 픽셀 크기를 문서 snapshot에 묶어 layout intrinsic-size 입력으로 옮긴다. 호출자는 byte slice를 이미 소유한다. 플랫폼 호스트는 resource request 발행, 로컬/원격 URI 해석·읽기, 비동기 스케줄·취소, 캐시, GPU texture 생성과 해제를 계속 소유한다. 런타임이 이 인터페이스를 호출하거나 플랫폼에서 이미지를 표시한다는 계약은 아니다.

이 계약은 [U05.1 계획](../../plan/u05-1-png-decode.md)과 [구현 전 Chrome 비교](./evidence/u05-1-png-precomparison-2026-10-14/README.md)를 함께 따른다. decoder는 [고정 `png` 0.18.1 공식 API](https://docs.rs/png/0.18.1/png/)의 `normalize_to_color8`, `Limits`, frame metadata를 사용한다. decoder의 memory limit은 best-effort이며 호출자 소유 allocation을 제한에 포함하지 않는다([Decoder](https://docs.rs/png/0.18.1/png/struct.Decoder.html), [Reader](https://docs.rs/png/0.18.1/png/struct.Reader.html)). 따라서 구현은 dimensions·output bytes를 별도로 확인한다.

## 호출 흐름

```text
HostDocumentSnapshot + connected HTML img handle
                │
                ├─ ImageRequest::from_document ── exact src + source stamp
                │
caller-owned PNG bytes ─ PngDecoder::decode ─ DecodedImage
                │
                └─ ImageResourceSnapshot::new(request, decoded)
                         │
                         └─ to_layout_intrinsic_size_snapshot()
                                  │
                                  └─ C14.1 HostDocument → Stylo → Taffy input
```

### PNG decoder

```rust
let decoded = PngDecoder.decode(png_bytes)?;
let dimensions = decoded.dimensions();
let rgba8 = decoded.rgba8();
```

`DecodedImage`는 positive width·height와 tightly packed straight-alpha RGBA8를 소유한다. `rgba8().len()`은 `width * height * 4`이며, row padding은 없다. gray·gray-alpha 입력은 RGB 채널로 복제하고, alpha가 없는 형식은 255를 사용한다. 입력 sRGB sample은 변환 없이 복사된다. 이 결과를 GPU 표시·ICC 색 변환 완료로 해석하지 않는다.

| 제한 | 값 | 초과 동작 |
|---|---:|---|
| encoded PNG slice | 16 MiB | `EncodedInputTooLarge` |
| width·height 각 축 | 8192 pixels | `DimensionLimitExceeded` |
| 총 pixel | 8,388,608 | `PixelLimitExceeded` |
| RGBA8 output | 32 MiB | `DecodedImageTooLarge` |
| png decoder 내부 제한 | 32 MiB best-effort | decoder error; 하드 상한으로 간주하지 않음 |
| `img src` 길이 | 1,048,576 UTF-16 code unit | `SourceTooLong` |
| snapshot entry 수 | 4096 | `TooManyImages` |
| snapshot RGBA byte 합 | 64 MiB, entry별 `byte_len()` 합산 | `SnapshotImageBytesExceeded` |

pixel/byte 크기 산술은 checked 연산으로 계산한다. decoder output과 RGBA output vector는 `try_reserve_exact`를 사용한다. 입력 상한은 함수 호출 전에 호출자가 이미 읽은 메모리를 회수하거나 network/file I/O를 제한하지 않는다.

정적 PNG, 비-interlaced 또는 Adam7, palette·grayscale·RGB/RGBA, tRNS와 16-bit sample 변환을 허용한다. APNG 제어 chunk, `pHYs`, eXIf metadata, ICC profile, 명시적 `gAMA`/`cHRM`, HDR metadata는 `UnsupportedMetadata`로 거부한다. `sRGB` chunk 단독과 color metadata 없는 일반 PNG는 허용하지만, `sRGB`와 같이 기록된 explicit `gAMA`/`cHRM`도 현재 색 관리 profile에서 거부한다. text chunk는 보관하지 않는다. malformed chunk/checksum, 잘림, unsupported output, 할당 실패는 구체 decode 오류로 반환한다.

현재 HTML natural-size 변환은 PNG pixel width/height와 CSS px를 1:1로 놓는다. `pHYs`가 있으면 거부하므로 별도 density correction은 하지 않는다. EXIF orientation도 적용하지 않는다.

### 문서 요청과 snapshot

`ImageRequest::from_document(document, handle)`는 다음 조건을 모두 요구한다.

- handle이 입력 document generation에 속하는 존재 요소다.
- 요소는 연결되어 있고 namespace가 `http://www.w3.org/1999/xhtml`, local name이 `img`다.
- unqualified `src` 속성이 존재하고 비어 있지 않다.
- `src`는 1,048,576 UTF-16 code unit 이하여야 한다. 길이 검사는 `DomString` 복제 전에 수행한다.

요청은 generation, document revision, render-tree revision, exact element handle 및 UTF-16 `src`를 보존한다. `src`를 URL로 parse·resolve·normalize하지 않는다.

`ImageResourceSnapshot::new(document, revision, entries)`는 각 완료 요청의 세대·node·src를 다시 검사한다. 현재 `src`를 재복제하지 않고 비교한다. 다른 source stamp, src 변경, 제거·분리·종류 변경, 중복 NodeId는 실패한다. 최대 4096 entry와 entry별 RGBA byte 합 64 MiB를 허용하며, Arc로 같은 이미지를 공유해도 entry별 `byte_len()`을 합산한다. 초과와 checked addition overflow는 실패한다. 동일 snapshot의 성공 디코드 결과만 포함한다. 완료되지 않았거나 실패한 이미지는 entry에서 생략한다. 이후 C14.1 adapter가 보이는 auto/auto image 누락과 CSS 경계를 판단한다.

`to_layout_intrinsic_size_snapshot()`은 동일 source와 `IntrinsicSizeRevision`을 유지하며 각 decoded image의 픽셀 크기를 `LayoutIntrinsicSize` CSS px로 변환한다. C14.1 레이아웃 adapter는 일반적인 layout source/node/style/dimension 검사를 계속 수행한다.

## 오류 종류

| API | 오류 계열 | 대표 이유 |
|---|---|---|
| `PngDecoder::decode` | `ImageDecodeError` | byte·dimension·pixel cap, allocation, malformed PNG/checksum, unsupported metadata/output |
| `ImageRequest::from_document` | `ImageRequestError` | stale/unknown handle, detached node, non-HTML-image, missing/empty/oversized src |
| `ImageResourceSnapshot::new` | `ImageSnapshotError` | stale source, stale generation, changed src, invalid node, duplicate node, entry/byte cap |
| resource → layout 변환 | `spinon_layout::LayoutError` | layout snapshot 내부 입력 구성 실패 |

## 예시와 판정

`tests/fixtures/resources/c14-1-png-basic.png`는 24×16 RGBA8, 97 bytes, SHA-256 `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2`다. Chrome agent-browser `HeadlessChrome/155`, DPR 1에서 `naturalWidth=24`, `naturalHeight=16`, CSS rect `24×16`을 관찰했다. 같은 bytes를 디코드해 C14.1 snapshot으로 전달한 HostDocument→Stylo→Taffy fixture는 `x=0,y=0,width=24,height=16`을 반환한다. 세부 관찰과 캡처는 [사전 비교 근거](./evidence/u05-1-png-precomparison-2026-10-14/README.md)를 따른다.

Android API 37 arm64 AVD에서 `spinon-image` test executable의 10개 Rust decode/request/snapshot test도 통과했다([실행 로그와 환경](./evidence/u05-1-png-android-avd-2026-10-14/README.md)). workspace 전체 테스트 729 passed·4 ignored, workspace Clippy, rustfmt 및 diff 검사는 [현재 소스 회귀 근거](./evidence/u05-1-png-implementation-2026-10-14/README.md)에 기록했다. 별도 [구현 실패 관점 검토](./evidence/u05-1-png-implementation-review-2026-10-14.md) 20개를 수행했다. AVD 실행은 APK/runtime adapter나 AVD 화면 표시를 호출하지 않는다.

이 근거는 고정 PNG와 host-document fixture, 순수 Rust AVD test executable만 다룬다. U05 전체, HTML image loading, app runtime adapter, Android/iOS 앱 표시, GPU texture/pixels, 색 관리, PNG 전체 적합성 또는 공개 HTML/CSS 지원 판정이 아니다.
