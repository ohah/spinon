# U05.1 · 제한 PNG 디코드와 이미지 측정 snapshot

## 목표

U05의 첫 구현 단위로 정적 PNG 바이트를 제한된 RGBA8 이미지로 디코드하고, 문서 세대·revision·현재 `<img src>`에 묶인 immutable image-resource snapshot을 C14.1 `LayoutIntrinsicSizeSnapshot`으로 연결한다. 이 작업은 U05 전체나 공개 이미지 지원을 완료하지 않는다.

공통 Rust 모듈은 포맷 디코드와 값 검증을 소유한다. 플랫폼 호스트가 요청·비동기 작업·캐시·URI/네트워크 로딩·GPU 자원 수명을 소유한다. 이는 현재 [코어 아키텍처](../docs/architecture.md)의 자원 소유 경계를 바꾸지 않는다.

## 비교 기준과 사전 판정

- 고정 입력: [`c14-1-png-basic.png`](../tests/fixtures/resources/c14-1-png-basic.png), 97 bytes, SHA-256 `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2`.
- 독립 oracle: 로컬 HTML fixture를 Chrome agent-browser로 열고 `naturalWidth`, `naturalHeight`, `getBoundingClientRect()` 및 `devicePixelRatio`를 읽었다. 관찰 UA는 `HeadlessChrome/155`, DPR `1`; 값은 natural size `24×16`, rect `x=0,y=0,width=24,height=16`이다. 캡처와 실행 환경 한계는 [사전 비교 근거](../spec/internal/evidence/u05-1-png-precomparison-2026-10-14/README.md)에 둔다.
- 고정 판정: 디코드 차원 `24×16`, RGBA 길이 `24*16*4=1536`, 요청된 같은 HTML `<img>`의 CSS natural size `24×16`, C14.1 결과 frame의 네 필드 차이 각각 `≤0.5 CSS px`.
- Oracle은 Chrome의 HTML image sizing 관찰이다. PNG 일반 적합성·색 정확도·Android/iOS 제품 표시를 증명하지 않는다.

## 범위와 계약

### 디코더

- workspace crate `spinon-image`, interface contract `0.1.0`, 고정 의존성 `png = "=0.18.1"`.
- 입력은 호출자가 소유한 PNG byte slice이며, 동기 `PngDecoder::decode`가 decoded RGBA8 이미지 또는 구체 오류를 반환한다. 파일·URL·Base64·MIME 탐색은 이 함수의 책임이 아니다.
- encoded input은 16 MiB 이하, 각 축은 8192 pixel 이하, 총 pixel은 8,388,608 이하, RGBA8 출력은 32 MiB 이하로 제한한다. 모든 곱셈·변환은 checked 연산으로 계산한다. 입력은 이미 호출자 메모리에 있을 수 있으므로 encoded-byte 상한은 전달 이후의 사용량 상한이고 네트워크·파일 read를 제한하지 않는다.
- decoder 내부 메모리는 32 MiB best-effort limit을 설정한다. 구현은 이 한도만 신뢰하지 않고 header dimensions와 output buffer를 별도로 검증한다. decoder가 요구하는 caller-owned buffer는 직접 `try_reserve_exact`로 확보한다.
- 8-bit RGBA 출력으로 정규화한다. palette, grayscale, tRNS, 16-bit 입력은 pinned decoder transformations 후 처리하며 결과 color type, bit depth, row stride, buffer length를 재검증한다. `png`가 돌려준 encoded sample을 sRGB color conversion 완료로 표시하지 않는다.
- 정적 PNG만 허용한다. APNG, `pHYs`, EXIF metadata, 미지원 ICC/gamma/chromaticity/HDR color metadata는 현재 명시 오류로 거부한다. 디코드 오류·체크섬 오류도 그대로 성공으로 바꾸지 않는다. text metadata는 디코더가 보관하지 않도록 한다.
- 현재 density policy는 pHYs가 없는 PNG의 pixel dimensions를 1:1 CSS px로 노출한다. 색 관리가 없는 RGBA8은 색상 정확성·GPU 최종 표시의 증거가 아니다.

### 문서 결합과 측정 snapshot

- `ImageRequest::from_document`는 동일 세대의 연결된 HTML namespace `img`와 비어 있지 않은 unqualified `src`를 받아 request를 만든다. `src`는 해석·정규화하지 않은 DOM UTF-16 값 그대로 저장한다. detached subtree의 결과는 layout resource snapshot에 들어가지 않는다.
- `ImageRequest::decode(bytes)`는 요청 객체를 디코드 결과와 함께 소유하는 `ImageResourceCompletion`을 만든다. snapshot API는 별도 request와 decoded image tuple을 받지 않아 호출자가 request와 image 값을 서로 바꿔 끼우는 실수를 줄인다. loader가 `bytes`를 해당 src에서 가져왔는지는 URI/network loader를 소유하는 caller 책임이며 이 slice에서 독립 검증하지 않는다.
- 요청의 `src`는 UTF-16 code unit 1,048,576개 이하로 제한해 DOM 값을 복제할 때의 할당을 제한한다. 한도를 넘으면 request 생성이 실패한다.
- `ImageResourceSnapshot::new`는 각 `ImageResourceCompletion`의 request source generation/document/render-tree revision, node handle, 현재 요소 종류와 정확한 `src`를 입력 `HostDocumentSnapshot`과 대조한다. 검증은 현재 `src`를 다시 복제하지 않고 비교한다. 하나라도 다르면 stale/invalid request 오류를 반환한다. 같은 `NodeId` 중복 완료를 거부한다.
- 한 resource snapshot은 최대 4096개 entry, entry별 `DecodedImage::byte_len()`의 합 64 MiB 이하로 제한한다. Arc를 공유해도 각 entry 크기를 보수적으로 합산한다. count와 byte 합산은 checked 연산으로 실패 폐쇄한다.
- resource snapshot은 `IntrinsicSizeRevision`과 문서 `LayoutSourceRevision`을 보존한다. 변환한 `LayoutIntrinsicSizeSnapshot`에는 디코드 완료된 항목만 들어가며 C14.1 adapter가 CSS 적용 상태와 누락 auto/auto image를 계속 판정한다.
- 이미지 크기는 `<img>`의 HTML width/height 반영 전 자연 CSS px 크기다. C14.2의 intrinsic ratio sizing, 한 축만 지정된 크기, `srcset`/`picture`, density correction은 이번 범위가 아니다.

## 명시적 제외

비동기 작업자·취소·캐시·동일 URL 공유·로컬/원격 resolver·HTTP·data URI decoder·파일/네트워크 I/O·GPU texture upload·render invalidation·오류 UI·animated PNG·다른 이미지 형식·ICC 색 변환·전체 PNG 적합성·실기기 또는 시뮬레이터 앱 표시 검증은 포함하지 않는다. 현재 runtime은 image request나 C14 snapshot을 만들지 않으므로 host-document fixture를 넘는 simulator runtime 결과를 만들 수 없다.

## 구현과 검증 순서

1. 이 계획을 구현 전에 별도 20개 적대 실패 관점으로 검토하고 발견을 반영한다.
2. Chrome PNG precomparison과 fixture hash를 고정한다.
3. `spinon-image` 계약·오류·정적 PNG 디코더·문서 요청·resource snapshot→layout snapshot 어댑터를 구현한다.
4. 정상 색상형/RGBA 정규화, Adam7 interlace, 각 resource cap 직전·초과, 잘린/CRC 손상 입력, APNG·density·orientation·색 metadata, stale generation/document/src, detached·비-HTML/non-img, duplicate mapping을 independent fixture와 negative control로 확인한다.
5. 동일 24×16 바이트를 HostDocument→request→decode→resource snapshot→C14.1 layout까지 통과시켜 Chrome 관찰값과 대조한다.
6. formatter, crate test, 관련 layout/style test, Android ARM64 target compile을 기록한다. 허용된 API 37 AVD에서 `spinon-image` Rust test executable을 직접 실행해 순수 디코더/요청 시험만 확인하며 이를 APK/runtime 통합 근거로 확대하지 않는다.
7. 구현 결과를 새 20개 실패 관점으로 검토하고 결함을 고친 뒤 영향 관점을 다시 확인한다.
8. 내부 계약 `0061`, 아키텍처, U05/C14 상태와 증거 인덱스를 같은 작업에 갱신한다. 전체 U05/C14를 완료로 체크하지 않는다.
9. PR은 프로젝트 PR 기준에 따라 독립 실패 관점으로 검토하고, 필수 CI·mergeability를 현재 GitHub 상태에서 확인한다.
