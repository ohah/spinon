# C15.3 glyph raster 구현 적대 검토 20개

**날짜:** 2026-10-17
**대상:** 고정 font asset에서 immutable Swash glyph raster snapshot으로 가는 Rust 코드
**구분:** C15.3 shaping-plan 검토와 별도인 구현 검토. PR 변경 전체 검토는 별도 문서다.

| # | 실패 관점과 공격 입력 | 확인 결과 |
| --- | --- | --- |
| 1 | 다른 engine이 같은 `FontRegistryRevision` 번호를 쓰는 snapshot을 넘긴다. | `FontRegistryToken` identity를 먼저 확인해 `ForeignFontRegistrySnapshot`으로 거부한다. |
| 2 | engine clone이 변경 전 같은 registry와 같은 font bytes를 공유한다. | clone은 token 및 asset map을 공유하고 같은 layout을 rasterize하는 assertion을 통과한다. |
| 3 | 두 clone에 서로 다른 family를 등록해 revision 숫자를 같게 만든 뒤 한쪽 snapshot을 다른 쪽에 넘긴다. | registration 성공 시 token을 회전한다. divergent clone case는 foreign registry 오류를 확인한다. |
| 4 | font 추가 전 만든 layout을 같은 engine에서 재사용한다. | revision 불일치를 `StaleFontRegistryRevision`으로 반환한다. |
| 5 | font 등록 호출자가 원본 `Vec`를 즉시 해제한다. | engine이 bytes를 소유하며 drop 이후 raster 성공을 확인한다. |
| 6 | run이 engine에 없는 family를 요청한다. | system fallback을 탐색하지 않고 `MissingRasterFont`으로 거부한다. |
| 7 | 등록 asset map이 손상된 SFNT bytes로 변조된다. | `FontRef::from_index` 실패를 `InvalidRasterFont`으로 반환한다. |
| 8 | TTC/OTC처럼 두 번째 face가 있는 asset을 전달한다. | 두 번째 face probe가 존재하면 거부한다. 저장소에 고정 collection fixture는 없어 해당 분기의 실행 검증은 남아 있다. |
| 9 | glyph ID를 `u32::MAX`로 바꾸거나 `u16` 변환 한계를 넘긴다. | Swash ID 변환 실패를 `GlyphRasterizationFailed`로 반환한다. |
| 10 | glyph ID를 0 또는 font `glyph_count` 이상으로 바꾼다. | `.notdef`/out-of-face glyph를 render하지 않는다. ID 0 및 상한 밖 assertion을 확인한다. |
| 11 | cluster 범위를 `0..usize::MAX` 또는 UTF-8 경계 밖으로 바꾼다. | `valid_text_range`에서 `InvalidGlyphLayout`으로 거부한다. |
| 12 | DOM source range를 비우거나 실제 paragraph source와 무관하게 조작한다. | source 없음과 fabricated UTF-8 range 모두 `InvalidGlyphLayout`을 반환한다. |
| 13 | Arabic/Bidi shaped glyph가 연속된 두 원본 UTF-16 source 조각을 하나의 mapping으로 갖는다. | layout source를 연속 구간으로 확인해 유효 mapping을 보존하고, 비연속 mapping은 통과시키지 않는다. |
| 14 | x/y/advance에 NaN/무한 값을 넣거나 physical x anchor를 정수 범위 밖으로 보낸다. | finite/range guard가 raster 이전에 실패한다. x overflow negative control은 DPR 1에서 확인한다. |
| 15 | DPR에 0, 음수, NaN, infinity 또는 64 초과 값을 준다. | `InvalidRasterScale`로 빠르게 거부한다. |
| 16 | `font_size × DPR`을 0/비유한/512 초과 값으로 만든다. | `RasterFontSizeOutOfRange`로 거부하고 실제 run family를 오류에 보존한다. |
| 17 | DPR 1/2/3에서 bitmap이 CSS 크기로만 생성되거나 CSS origin이 배율 변경 때 움직인다. | Ahem fixture가 각각 10/20/30 physical px와 고정 CSS origin/baseline을 확인한다. |
| 18 | 음수 또는 fractional baseline이 fractional sample phase를 잃고 한 pixel 이동한다. | 양의 fractional x/y를 정수 anchor와 Swash phase로 분리하는 fixture가 있다. 음수 fractional phase의 독립 pixel fixture는 남아 있다. |
| 19 | space의 빈 raster가 `None` 대신 잘못된 0-size byte view가 되거나 truncated/oversized image가 허용된다. | Swash의 실제 `1×0` empty image와 synthetic `0×0`을 빈 glyph로 처리한다. 길이 불일치, 곱 overflow, 과대 크기 및 top negation overflow는 거부한다. |
| 20 | paragraph 내 glyph image 합계가 무제한 커지거나 이를 색/clip/paint 완료로 오해한다. | 누적 raster data를 64 MiB로 제한하고 한계/usize overflow를 독립 검사한다. 결과는 Rust data snapshot뿐이며 실제 WGPU paint, color glyph, compositing, overflow clip/hit-test는 미검증이라고 문서화했다. |

## 판정

위 검토에서 발견한 clone provenance, Swash zero-height blank image, 누적 snapshot 크기 결함을 수정하고 관련 negative control을 추가했다. collection-font fixture, 음수 fractional phase, 악성 font의 Swash 내부 temporary allocation, color bitmap과 실제 platform paint는 이 변경에서 확인하지 않았으며 README에 남은 범위로 기록했다. 테스트와 로그는 [구현 실행 근거](README.md)에 연결되어 있다.
