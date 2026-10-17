# C15.3 glyph raster snapshot PR 변경 적대 검토 20개

**날짜:** 2026-10-17
**대상:** `feat/c15-3-text-glyph-paint-20261017`에서 고정 font bytes를 raster snapshot으로 만드는 전체 변경
**검토 기준:** 최종 변경 diff, Swash `0.2.10` 소스, 고정 font fixtures, 실행 로그와 내부 계약

| # | 독립 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | 다른 engine에서 만든 layout이 같은 숫자 revision을 가져 provenance 검사를 우회한다. | revision 비교 다음 `FontRegistryToken` identity를 비교해 foreign engine을 거부한다. 별도 engine negative control이 통과했다. |
| 2 | 동일 font registry를 복제한 engine clone이 자기 snapshot을 rasterize하지 못한다. | clone은 identity와 owned font asset map을 공유한다. clone과 원본의 raster 결과 동등 검사를 통과했다. |
| 3 | 한 registry clone에서 다른 font를 추가하고 다른 clone에도 다른 font를 추가하면 같은 숫자 revision이 된다. | 등록 성공 때마다 token을 회전해 divergent clone 결과를 거부한다. 같은 revision인 분기 fixture가 통과했다. |
| 4 | 등록 함수가 호출자 font buffer를 빌려서 호출 뒤 해제된 메모리를 읽는다. | engine이 `Arc<Vec<u8>>`를 소유한다. 호출자 buffer를 drop한 뒤 raster 성공을 확인했다. |
| 5 | font 등록 검증 실패 뒤 collection, asset map, revision 또는 identity 일부만 바뀐다. | 후보 collection의 검증이 끝난 뒤에만 collection·asset·token·revision을 갱신한다. 기존 transactional registration 회귀 검사를 확인했다. |
| 6 | font 추가 전 layout이 뒤의 registry 상태에서 재사용된다. | revision mismatch는 `StaleFontRegistryRevision`으로 실패한다. 회귀 입력이 이를 확인했다. |
| 7 | `u32::MAX`, `.notdef` 0, 실제 font glyph 수 바깥 ID가 renderer에 전달된다. | Swash ID 변환과 face glyph count를 모두 검사하고 실패시킨다. 세 경계 negative control이 통과했다. |
| 8 | 누락되거나 손상된 font asset에서 OS/system fallback이 조용히 선택된다. | 누락은 `MissingRasterFont`, 손상 asset은 `InvalidRasterFont`으로 종료한다. fallback을 호출하지 않는 오류 검사를 확인했다. |
| 9 | TTC/OTC의 예상 밖 face index에서 다른 face를 rasterize한다. | face 0만 쓰고 face 1이 있으면 거부한다. 코드 분기는 확인했으나 실제 font collection fixture가 없어 실행 결과는 미검증으로 남겼다. |
| 10 | NaN/무한 origin·baseline·advance, UTF-8 중간 offset, 범위 초과 cluster가 rasterizer에 도달한다. | finite 좌표와 UTF-8 boundary를 검사한다. NaN/범위 초과 glyph 변조가 `InvalidGlyphLayout`으로 끝났다. |
| 11 | 비어 있거나 layout의 원본 DOM source와 연결되지 않은 mapping이 결과에 보존된다. | source가 비어 있으면 거부하고, 실제 layout source 조각으로 연속 재구성되는지 확인한다. fabricated range와 누락 source negative control이 통과했다. |
| 12 | Arabic/Bidi/combining에서 glyph 순서나 DOM source map이 raster 단계에서 재정렬·손실된다. | run/glyph 순서, glyph ID, cluster range, source mapping을 비교하는 고정 Hangul/CJK/Arabic/ZWJ/combining/ligature fixture가 통과했다. |
| 13 | DPR 변경이 CSS 위치까지 배율화하거나 bitmap을 CSS 크기로 남긴다. | Ahem bitmap은 DPR 1/2/3에서 10/20/30 physical px이며 CSS origin과 baseline은 유지된다. |
| 14 | 소수점 glyph origin이 정수 anchor 반올림으로 바뀌거나 subpixel phase가 빠진다. | 정수 anchor와 Swash phase를 분리한다. 양의 fractional x/y를 검사했으며 음수 fractional phase의 독립 pixel fixture는 남은 범위로 기록했다. |
| 15 | 공백·정상 빈 glyph의 Swash `1×0` image가 잘못된 빈 byte view가 된다. | 비어 있는 zero-width 또는 zero-height image를 `None`으로 변환하고, 실제 whitespace 및 synthetic empty image를 검사했다. |
| 16 | truncated payload, dimension 곱 overflow, 잘못된 채널 수 또는 `i32::MIN` top이 bitmap 배치에 들어간다. | mask/RGBA channel 수로 기대 byte 길이를 계산하고 overflow와 top negation을 검증한다. malformed payload negative controls가 통과했다. |
| 17 | 단일 bitmap이 제한을 초과해 저장되거나 색 bitmap의 RGBA byte 수를 alpha mask 기준으로 계산한다. | alpha mask는 1 byte/pixel, color/subpixel RGBA는 4 byte/pixel로 구분하고 16 MiB 경계를 검사한다. 색 outline/bitmap 실제 font fixture는 없어 출력 parity는 주장하지 않는다. |
| 18 | 여러 glyph가 합쳐진 paragraph snapshot이 64 MiB를 넘어 누적되거나 `usize` 합산 overflow가 난다. | 각 결과 bitmap을 추가할 때 checked accumulator를 적용한다. limit 통과, limit 초과, 정수 overflow negative control을 확인했다. Swash 내부 생성 allocation은 결과 후 검사라 상한을 증명하지 않는다. |
| 19 | 새 hidden registry token 때문에 기존 `TextLayoutSnapshot` 값 비교가 engine identity에 따라 달라진다. | identity는 raster 입력 provenance 검사에만 사용하고 snapshot equality는 기존 공개 값 필드만 비교하도록 분리했다. 동일 font의 독립 engine 결과 equality와 foreign provenance 거부를 함께 확인했다. |
| 20 | 근거가 Rust bitmap 준비 단계를 실제 text paint 또는 C15 완료로 과장한다. | 계약·상태·근거에서 WGPU command, runtime admission, Android/iOS Simulator ink, color management, compositing, ancestor clip/hit-test를 미완료로 유지했다. 테스트 35/35·154/154, fmt, workspace Clippy 로그는 확인했고 UI/Simulator는 실행하지 않았다. |

## 판정

검토 중 test module이 500줄 권장 상한을 넘는 점과 registry identity가 `PartialEq`에 스며드는 점을 발견했다. shaping/raster test module을 나누고 snapshot equality를 내용 기반으로 유지하도록 수정한 뒤 관련 test를 다시 실행했다. TTC/OTC, 음수 fractional phase, 실제 color glyph, Swash 내부 temporary allocation, WGPU/app/Simulator paint는 이 변경에서 입증하지 않았으며 내부 계약과 실행 근거에 한계로 남겼다.
