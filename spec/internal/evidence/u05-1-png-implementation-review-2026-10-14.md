# U05.1 PNG 구현 적대 검토 · 2026-10-14

대상은 `spinon-image` 디코더·요청·snapshot, C14.1 layout 연결, 계약 0060 및 fixture다. 각 관점은 별도의 입력 실패·수명·자원 경계로 확인했다. 구현 중 찾은 결함은 수정 뒤 영향받는 관점을 다시 실행했다.

| # | 실패 관점 | 판정과 조치 |
|---:|---|---|
| 1 | 큰 caller-owned PNG slice를 decoder에 넘긴 뒤에야 입력 한도를 확인할 수 있다 | 디코더 시작에서 16 MiB 상한을 확인한다. 계획과 계약에서 호출 전 I/O에는 적용되지 않는다고 제한했다. oversized slice negative control 통과. |
| 2 | 0 크기·축 한도 초과 입력이 압축 해제 전에 거부되지 않는다 | 헤더에서 양수 크기, 축별 8192 및 checked pixel 수를 먼저 확인한다. 잘못된 크기·축 초과 negative control 통과. |
| 3 | `width*height*4` 또는 플랫폼 `usize` 변환 overflow가 작은 할당으로 돌아간다 | checked 변환·곱셈 후 pixel/output 상한을 적용한다. 8,388,608 pixel 초과 입력 거부. |
| 4 | decoder `Limits`가 caller-owned 출력과 ICC·palette 임시 메모리까지 hard cap한다고 오해한다 | 32 MiB best-effort 한도와 별도로 output 크기를 검사하고 직접 buffer는 `try_reserve_exact`로 확보한다. 내부 문서가 이를 하드 보장으로 표현하지 않음을 확인. |
| 5 | Adam7 pass를 평면 행처럼 읽어 pixel 순서가 달라진다 | Adam7 fixture decode 결과가 같은 RGBA의 비-interlaced fixture와 byte 단위로 같다. |
| 6 | packed grayscale, palette, tRNS 또는 16-bit input을 RGBA8 입력으로 오인한다 | 고정 png 변환 이후 color type·bit depth를 재확인하고 packed gray/RGB/gray-alpha/palette+tRNS/16-bit 사례의 RGBA 값을 비교했다. |
| 7 | alpha 없는 grayscale/RGB에서 쓰레기 alpha가 나오거나 회색 채널이 다르다 | 변환 규칙은 회색 채널 복제와 opaque alpha 255이며 gray, gray-alpha, RGB, RGBA 픽셀 control 통과. |
| 8 | row stride padding을 실제 pixel로 복사하거나 output이 짧아도 성공한다 | `line_size`, row bytes, 총 written length, 최종 RGBA 길이를 모두 검사하고 유효 row bytes만 복사한다. fixture 결과 길이 1536 bytes. |
| 9 | signature·압축 stream·CRC가 잘렸거나 손상되어도 lenient decode로 성공한다 | signature, truncation, chunk CRC negative control이 모두 decoder error로 실패한다. |
| 10 | `acTL`만 있는 파일을 첫 frame 정적 PNG로 받아들인다 | animation/frame control metadata를 decode 전과 finish 후 검사한다. `acTL` 단독과 `acTL+fcTL` 모두 Animation 오류. |
| 11 | IDAT 뒤 ancillary metadata를 header 검사에서 놓치고 resource snapshot으로 보낸다 | `reader.finish()` 뒤 metadata를 재검사한다. IDAT 뒤 `eXIf` negative control은 Exif 오류다. |
| 12 | `pHYs` density를 무시해 pixel 크기를 browser CSS natural size로 잘못 쓴다 | 현재 1:1 policy에서 모든 pHYs chunk를 거부하며, density fixture가 PixelDensity 오류다. |
| 13 | sRGB와 충돌하는 명시적 gAMA/cHRM을 변환 없이 허용한다 | 최초 구현은 sRGB 존재 시 명시 gAMA를 통과시켰다. 명시 gAMA/cHRM을 항상 거부하도록 수정했고 sRGB 단독 positive 및 sRGB+gAMA negative control을 확인했다. |
| 14 | ICC profile 또는 HDR/cICP sample을 RGBA 색상 변환 완료로 표시한다 | png encoder로 유효 ICC chunk를 만든 control과 유효 cICP chunk를 거부한다. `DecodedImage` 문서도 sample 복사와 색 변환을 구분한다. |
| 15 | 대용량 compressed text chunk를 요청 때마다 보관한다 | `set_ignore_text_chunk(true)`를 설정하고 text 보관을 디코더 책임에서 제외했다. |
| 16 | `DecodedImage::clone()`이 32 MiB pixel buffer를 호출자 의사 없이 복사한다 | `Clone` derive를 제거했다. 공유가 필요한 C14 fixture는 `Arc<DecodedImage>`를 사용한다. |
| 17 | 너무 긴 DOM src를 검사 전에 복제하거나 UTF-8 byte 길이로 잘못 센다 | `DomString` UTF-16 길이를 clone 전에 제한한다. non-BMP 문자 524,288개(정확히 1,048,576 code unit)는 허용하고 524,289개(1,048,578 code unit)는 거부하는 boundary control을 추가했다. |
| 18 | unknown/detached/SVG/non-img 또는 namespaced `src`를 HTML 이미지 요청으로 채택한다 | node 존재 여부를 연결성보다 먼저 확인하며 HTML namespace·`img`·unqualified nonempty `src`를 요구한다. 각 negative control을 확인했다. |
| 19 | 늦은 완료가 바뀐 src·다른 generation/document·중복 NodeId를 덮거나 다른 decoded image와 잘못 결합된다 | 최초 API는 request와 decoded image tuple을 받아 payload misassociation을 막지 못했다. `ImageRequest::decode`가 request-bound completion을 만들고 snapshot은 그 타입만 받도록 바꿨다. 현재 src는 복제 없이 다시 비교하며 generation·전체 source stamp·중복 키도 검증한다. URI loader byte provenance는 loader 범위로 명시했다. |
| 20 | 많은 작은 이미지나 큰 RGBA entry 합계가 resource snapshot 메모리를 무제한 늘린다 | 최초 구현에는 snapshot 전체 상한이 없었다. 최대 4096 entry·64 MiB 합계를 checked 검증하고, 공개 생성 경로의 4097번째 entry 거부와 byte-budget 경계/overflow helper 시험을 추가했다. |

## 수정 뒤 판정

20개 관점을 확인했다. 구현 발견(Clone 복사, snapshot 총량 미제한, sRGB 혼합 gamma 허용, 현재 src 재복제, request/payload misassociation)을 고치고 관련 관점을 다시 확인했다. 추가 요청 검사에서 unknown node가 detached로 분류되던 순서를 바로잡았다. 이 결과는 PNG fixture와 HostDocument/C14.1 Rust 경계의 구현 근거다. 앱 runtime image loader, 비동기 취소·cache, GPU texture/pixel, Android 앱 화면 및 iOS 동작을 증명하지 않는다.
