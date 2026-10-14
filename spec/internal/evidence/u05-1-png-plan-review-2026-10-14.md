# U05.1 계획 적대 검토 · 2026-10-14

대상: [`u05-1-png-decode.md`](../../../plan/u05-1-png-decode.md). 각 행은 서로 다른 실패 축을 계획에 대조했다. 검토 중 반영된 변경은 각 판정에 기록했다.

| # | 실패 관점 | 판정과 계획 반영 |
|---:|---|---|
| 1 | encoded byte limit이 파일/network read 이전에 적용된다고 오해할 수 있다 | 전달된 slice 이후의 상한이라고 한정했다. loader I/O는 제외했다. |
| 2 | PNG signature가 없거나 잘린 입력이 generic panic/성공으로 처리될 수 있다 | 디코더 오류를 그대로 실패로 전달하고 잘린 입력을 negative control에 넣었다. |
| 3 | IHDR dimensions가 0일 때 빈 RGBA 이미지가 반환될 수 있다 | 축별 양수 검증과 경계 오류 시험을 검증 목록에 명시했다. |
| 4 | 매우 큰 단일 축에서 pixel cap만으로 Taffy/플랫폼 경계를 보호하지 못할 수 있다 | 각 축 8192 cap을 pixel 수와 별도로 두었다. |
| 5 | width×height 또는 pixel×4가 overflow해 작은 할당값으로 돌아갈 수 있다 | checked 연산 및 각 산술 실패 case를 구현 리뷰에 포함했다. |
| 6 | `png::Limits`가 모든 할당을 hard cap한다고 잘못 신뢰할 수 있다 | best-effort 한도와 caller-owned buffer 제외를 계획에 명시하고 별도 dimension/output cap을 둔다. |
| 7 | 압축 해제 후 buffer를 `vec![0; n]`로 직접 만들며 OOM abort 가능성을 숨길 수 있다 | `try_reserve_exact`로 caller buffer를 확보하고 결과를 오류 처리한다. |
| 8 | text chunk 압축 폭탄이 image decode 시 불필요하게 metadata로 누적될 수 있다 | text metadata 무시 설정을 계약에 포함했다. |
| 9 | decoder 변환 후 color type이 입력과 같다고 가정할 수 있다 | 출력 color type·bit depth를 확인해 지원 변형만 RGBA8로 변환한다. |
| 10 | row `line_size` padding을 픽셀 데이터로 복사할 수 있다 | stride·buffer length를 검증한 뒤 row별 유효 pixel 영역만 복사한다. |
| 11 | palette index와 palette bounds 오류가 잘못된 색으로 변환될 수 있다 | pinned decoder expansion 사용 및 corrupt/truncated palette failure fixture를 검증한다. |
| 12 | tRNS가 alpha로 확장되지 않아 투명색이 불투명해질 수 있다 | tRNS 포함 indexed/grayscale 입력을 시험하고 RGBA alpha 결과를 대조한다. |
| 13 | 16-bit 또는 packed grayscale 결과가 8-bit 샘플 배열로 오인될 수 있다 | transformations와 postcondition을 명시하고 bit-depth 분기 실패를 확인한다. |
| 14 | interlace 처리 후 결과 길이·방향을 잘못 계산할 수 있다 | Adam7 fixture 포함을 검증 매트릭스와 구현 리뷰에 추가해야 한다. plan was incomplete: **FIX**. |
| 15 | APNG 첫 frame만 정적 PNG로 성공 처리되어 animation 의미가 사라질 수 있다 | animated control chunk를 확인하고 APNG를 decode 전 명시 거부한다. |
| 16 | pHYs density를 무시해 natural CSS size가 browser와 달라질 수 있다 | 이번 profile에서 pHYs가 있는 모든 PNG를 명시 거부한다. |
| 17 | PNG EXIF orientation을 무시해 layout/decoded pixels가 다른 방향일 수 있다 | EXIF metadata가 있는 입력을 거부하고 orientation matrix는 후속 범위로 남긴다. |
| 18 | gamma/chromaticity/ICC/HDR 태그를 변환하지 않은 채 sRGB RGBA라고 부를 수 있다 | 색공간 변환을 주장하지 않고 미지원 metadata를 거부한다고 계약에 썼다. metadata 조합별 negative control 필요. |
| 19 | CRC/Adler checksum 실패가 lenient 옵션으로 무시될 수 있다 | checksum bypass를 켜지 않고 손상 fixture에서 실패를 확인한다. |
| 20 | APNG/pHYs/EXIF/color metadata 오류를 같은 generic decode 오류로 뭉치면 caller가 재시도 여부를 판단할 수 없다 | format unsupported와 malformed decode 오류를 구분하고 API error cases를 명세한다. |
| 21 | 다른 문서 generation의 동일 NodeId가 resource snapshot을 오염할 수 있다 | request handle generation과 snapshot source 전체를 대조하도록 했다. |
| 22 | 같은 문서에서 src가 바뀐 뒤 늦은 완료가 새 src로 채택될 수 있다 | exact `DomString` src와 document/render-tree revision을 함께 비교한다. |
| 23 | SVG namespace의 `img` 유사 노드를 HTML image로 잘못 취급할 수 있다 | HTML namespace와 local name을 함께 확인한다. |
| 24 | detached node의 decode 완료를 연결된 scene 자원으로 승격할 수 있다 | 계획 수정: request 생성은 connected HTML img만 허용한다. fixture는 연결/분리 경계를 분리한다. **FIX**. |
| 25 | duplicate asynchronous completion이 같은 NodeId를 덮어써 source/dimensions 혼선을 만들 수 있다 | duplicate NodeId 완료를 오류로 거부하도록 했다. |
| 26 | `src` 누락/빈 문자열을 URL처럼 decode 작업에 등록할 수 있다 | request 생성 시 누락·빈 src를 거부한다. |
| 27 | IntrinsicSizeRevision overflow 때 예전 snapshot revision이 재사용될 수 있다 | revision 생성은 호출자에게 맡기고 `checked_next`에서 소진되면 실패하도록 명시했다. |
| 28 | HTML width/height 속성을 natural size에 먼저 적용해 C14.1이 두 번 clamp할 수 있다 | decoder 치수는 원본 natural size이며 HTML/CSS 사용 크기는 C14 경계에 남긴다. |
| 29 | 공개 API/전체 U05/C14 지원 완료처럼 문서가 확대 해석될 수 있다 | 숫자 버전 0.1.0 내부 계약, 공개 지원·로드맵 전체 완료 금지를 명시했다. |
| 30 | 순수 fixture 성공을 Android/iOS runtime 및 GPU 표시 proof로 과장할 수 있다 | runtime owner가 없다는 한계와 simulator 표시 근거 제외를 문서에 기록했다. |

### 조치

30 관점을 검토했다. 두 계획 결함(Adam7 fixture 제외, detached node 수용 가능성)을 계획에 반영했다. 구현 완료 전 해당 관점은 코드와 별도 fixture로 다시 판정한다.
