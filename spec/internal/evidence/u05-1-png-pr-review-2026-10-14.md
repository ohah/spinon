# U05.1 PNG PR 적대 검토 · 2026-10-14

## 검토 기준

검토 대상은 `origin/main`의 PR 후보 전체와 request-bound decode 후속 변경이다. 각 행은 서로 다른 실패 경로 또는 저장소 계약 경계를 확인한다. 작업 단위 구현 검토 20개는 별도 문서에 둔다. 이번 PR 검토에서는 새로운 수정 필요 사항을 발견하지 않았다. URI 바이트 provenance와 앱 runtime 연결은 명세에 남은 범위로 기록했다.

## 실패 관점

| # | 확인한 실패 경계 | 확인 결과와 근거 |
|---:|---|---|
| 1 | PNG 외 임의 입력이 유효 이미지로 채택된다 | `read_info`와 `next_frame`의 `png` 오류를 실패로 반환한다. 잘못된 signature fixture가 거부된다. |
| 2 | 파일 크기 제한 검사 전에 decoder가 입력을 복제한다 | 16 MiB encoded cap을 `Cursor`와 decoder 생성 전에 확인한다. cap 초과 fixture가 별도 오류로 종료된다. |
| 3 | 0 크기 또는 축 제한을 넘는 이미지가 큰 버퍼 할당을 유발한다 | IHDR dimension 검사와 축·pixel cap이 decoded buffer 예약보다 먼저 수행된다. zero·8193축·pixel 초과 fixture가 거부된다. |
| 4 | pixel count 또는 RGBA 길이 산술이 overflow한다 | `usize` 변환과 곱셈·4배 산술에 checked 연산을 사용하고 실패 폐쇄한다. |
| 5 | RGBA 결과 크기가 전역 byte cap을 넘는다 | pixel cap에 더해 `MAX_RGBA_BYTES` 및 decoder output size를 각각 확인한다. |
| 6 | 메모리 압박에서 infallible Vec 성장이 발생한다 | decoder 및 RGBA buffer를 `try_reserve_exact`로 예약하고 allocation 오류를 반환한다. |
| 7 | `png::Limits`가 전체 프로세스 메모리를 강제한다고 오해한다 | 계약 0061은 decoder limit이 best-effort이며 caller-owned 입력/output을 별도 검사한다고 구분한다. |
| 8 | interlaced PNG에서 패스 순서가 최종 이미지 행으로 노출된다 | Adam7 fixture의 전체 row-major RGBA가 비interlaced fixture와 byte 단위로 일치한다. |
| 9 | grayscale·alpha·RGB 입력이 잘못된 channel 수나 alpha로 변환된다 | grayscale/RGB의 opaque alpha와 grayscale-alpha/RGBA의 alpha를 각각 fixture로 확인한다. |
| 10 | indexed palette와 `tRNS` 투명도가 소실된다 | palette 및 투명도 fixture에서 두 pixel의 RGBA sample을 확인한다. |
| 11 | packed 1-bit 및 16-bit sample 변환이 행 경계를 넘는다 | 1-bit grayscale unpack과 16-bit→8-bit 정규화를 작은 fixture로 검사한다. |
| 12 | PNG row stride를 무시해 padding이나 잘린 행이 결과에 들어간다 | color type별 최소 row bytes, decoder stride, 전체 출력 길이를 검사한 뒤 행별로 RGBA에 추가한다. |
| 13 | decoder가 헤더와 다른 출력 크기나 bit depth를 반환한다 | frame 출력 dimensions 및 8-bit 정규화 결과를 명시적으로 확인한다. |
| 14 | 손상되거나 잘린 chunk/checksum이 성공 처리된다 | truncation과 손상된 IDAT CRC negative fixture가 `Decoder` 오류로 거부된다. |
| 15 | 여러 frame의 APNG가 정적 PNG로 일부만 표시된다 | animation/frame control metadata를 디코드 전후 확인하고 APNG fixture를 거부한다. |
| 16 | `pHYs`가 CSS px 자연 크기를 바꾼다 | pixel-density metadata를 명시적으로 거부하고 계약에 CSS px 자연 크기와 한계를 기재한다. |
| 17 | EXIF orientation metadata가 실제 픽셀 크기/방향과 어긋난다 | EXIF chunk를 앞·뒤 위치 두 fixture로 거부한다. |
| 18 | sRGB와 gamma/chromaticity/HDR/ICC 설정을 임의로 섞는다 | 변환을 구현하지 않은 metadata를 거부하고 sRGB 단독 허용 및 sRGB+gAMA 거부 control을 확인한다. |
| 19 | 비BMP `src` 길이를 UTF-8 바이트 수로 잘못 세거나 검사 전에 복제한다 | `DomString::code_units()` 한도를 clone 전에 검사하며 UTF-16 경계의 supplementary 문자 허용/거부 fixture가 있다. |
| 20 | detached, foreign namespace, 비-`img`, namespaced `src`가 요청으로 생성된다 | HTML namespace·정확한 local name·연결성·unqualified `src`를 검사하고 각 negative case를 요청 시험에 둔다. |
| 21 | 알 수 없는 handle이 detached 오류와 혼동되거나 잘못 허용된다 | node lookup을 connectivity 검사보다 먼저 해 `UnknownElement`과 `DetachedElement`을 구분한다. |
| 22 | `ImageRequest`의 document generation/document/render-tree stamp가 빠진다 | 요청에 전체 `LayoutSourceRevision::HostDocument`를 보존하고 snapshot에서 현재 stamp와 재비교한다. |
| 23 | src 변경 뒤 이전 비동기 결과가 새 요소 상태를 덮는다 | snapshot 생성 때 현재 unqualified src를 재확인하며 변경 src fixture는 `SourceChanged`를 기대한다. |
| 24 | 다른 문서 세대나 제거·변경된 요소 결과가 snapshot에 섞인다 | generation·전체 revision·요소 유효성을 검사하고 다른 generation/revision fixture를 거부한다. |
| 25 | 같은 NodeId의 두 decode 결과가 순서에 따라 임의로 덮인다 | `BTreeMap` 삽입 전 중복 node를 거부하며 duplicate fixture가 있다. |
| 26 | request와 별개로 전달한 `DecodedImage`가 다른 요청 payload와 짝지어진다 | 공개 snapshot 입력이 `ImageResourceCompletion`만 받도록 바꿨다. `ImageRequest::decode`가 같은 request를 보존하며 외부에서 field/constructor에 접근할 수 없다. caller가 해당 src의 bytes를 넘기는 책임은 계약에 남는다. |
| 27 | 수천 개 entry 또는 Arc 공유가 snapshot byte budget을 우회한다 | 4096 entry 및 entry별 decoded byte 합을 checked 검증하고 Arc 공유도 각 entry 크기로 계산한다. 경계/overflow 검사는 구현 검토 근거에 있다. |
| 28 | pixel dimensions가 image density나 처리된 화면 크기로 잘못 표기된다 | pixel dimensions만 CSS px 측정 snapshot으로 전달한다. density 변환·GPU upload·색 관리 주장을 계약과 구현에서 제외한다. |
| 29 | CSS 지정 크기, auto/auto 누락, stale layout 입력이 PNG adapter에서 우회된다 | PNG resource snapshot은 intrinsic 입력만 제공하고 C14.1 style/layout adapter가 별도 revision·CSS preflight를 수행한다. HostDocument→Stylo→Taffy fixture가 24×16 frame을 비교한다. |
| 30 | ADB/브라우저 근거가 실기기·제품 화면 표시로 과장되거나 다른 작업 범위가 완료 처리된다 | AVD 기록은 API 37 arm64 Rust test executable 10/10으로 한정하고 ADB 설정/linker 실패 원인을 분리한다. Chrome 비교는 agent-browser 자료이며 terminal-browser를 쓰지 않았다. U05/C14.1은 STATUS에서 미완료다. PR은 `enhancement`와 `area: layout` 기존 라벨을 사용한다. |

## 판정

서로 다른 30개 실패 경계와 저장소 계약을 검토했다. 이번 PR 변경에서 추가 수정은 필요하지 않았다. 아직 GitHub CI check는 보고되지 않았으므로 검토 통과를 CI 통과나 병합 가능 판정으로 확대하지 않는다. 상위 U05/C14.1, URI loader, 앱 runtime, GPU/화면 표시와 플랫폼 제품 동작은 미완료다.
