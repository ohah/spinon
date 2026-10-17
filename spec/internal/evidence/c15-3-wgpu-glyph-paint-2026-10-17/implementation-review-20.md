# C15.3 WGPU glyph paint 구현 실패 관점 검토 20개

**대상:** C15.3 WGPU glyph paint 변경 작업 트리, 2026-10-17
**범위:** `RuntimeRenderText` snapshot 계약, mask atlas, WGPU paint 경로, fixture FFI 및 Android/iOS simulator route. 공개 CSS text 지원이나 실제 기기 동작을 판정하지 않는다.
**판정 규칙:** 각 항목은 서로 다른 실패 원인을 따로 확인했다. `확인`은 코드와 관련 focused test 또는 보존한 runtime evidence가 해당 범위에서 통과했다는 뜻이며, 모든 GPU·OS·사용자 입력 조합을 입증한다는 뜻은 아니다.

## 검토

1. **잘못된 bitmap dimension과 짧거나 긴 payload** — `RuntimeGlyphBitmap::new`가 zero dimension, `width × height` overflow, 길이 불일치와 16 MiB 초과 mask를 거부한다. `text_snapshot_checks_bitmap_geometry_source_owner_and_unique_node`의 길이 불일치 control이 오류를 확인했다. **확인.**
2. **여러 glyph가 합쳐질 때 메모리 한도 우회** — `RuntimeRenderText::new`는 총 mask bytes 64 MiB와 glyph 수 1,000,000을 snapshot admission에서 다시 제한한다. 한도 초과 payload 전체를 할당하는 대형 단위 시험은 추가하지 않았으며, 개별 glyph 한도와 누적 합산 코드를 정적으로 대조했다. **코드 확인, 대형 payload 실행은 미검증.**
3. **NaN·무한대·0 또는 음수 scale로 인한 잘못된 좌표** — 위치와 scale 유효성을 생성자에서 검사하고 `append_text_vertices`가 CSS 좌표 변환 뒤 유한성과 양의 크기를 확인한다. 무한대 위치 거부는 `text_snapshot_checks_bitmap_geometry_source_owner_and_unique_node`가 확인한다. **확인.**
4. **유효 범위 밖의 sRGB 채널·alpha** — `RuntimeSrgbColor::new`가 네 성분 각각의 유한성 및 `[0,1]` 범위를 검사한다. NaN 채널 negative control이 같은 text snapshot 시험에서 통과했다. **확인.**
5. **오래된 document/style/environment/intrinsic-size source를 현재 scene에 연결** — `matches_text_source`와 `validate_texts`가 revision tuple을 대조하고 `InvalidTextSource`로 거부한다. intrinsic-size revision mutation은 text snapshot 시험에서 확인했다. scroll revision은 text 위치 입력이 아니며 scroll owner는 별도로 거부한다. **확인.**
6. **없는 owner 또는 한 owner에 두 text snapshot 연결** — scene admission이 box membership과 owner uniqueness를 검사한다. missing owner와 duplicate owner negative control이 통과했다. **확인.**
7. **scroll 이동 후 glyph만 이전 위치에 남는 상태** — text owner 자체가 scroll container이거나 metadata에 scroll ancestor가 있으면 `UnsupportedTextScroll`로 거부한다. 새 owner-scroll negative control이 통과했다. ancestor metadata 분기도 코드에서 확인했지만 별도 mutation은 두지 않았다. **확인, scroll text는 지원 밖.**
8. **paint command가 누락·중복·역순이거나 group이 닫히지 않은 상태** — `validate_paint_commands`가 box/text index를 각자 연속 순서로 소비하고 group depth를 균형 검사한다. 잘못된 text index와 누락 command 시험 및 기존 group validation을 대조했다. **확인.**
9. **box와 glyph의 겹침 순서가 배열 순서와 달라지는 상태** — `DrawText`는 별도 command로 box command 사이에 놓이며 WGPU pass가 command stream을 순서대로 소비한다. `scene_paint_order_can_interleave_box_and_glyph_commands`가 순서 보존을 검사한다. **구조 확인; 색상 겹침의 별도 앱 matrix는 범위 밖.**
10. **clip 경계에서 quad만 자르고 mask UV를 자르지 않아 인접 coverage를 새는 상태** — `append_text_vertices`가 geometry와 UV를 같은 visible interval로 자른다. WGPU readback의 clip 안/반열린 바깥 sample이 각각 기대 coverage와 불투명 검정으로 일치했다. **확인.**
11. **clip이 없을 때 glyph가 target viewport 바깥에 제출되는 상태** — visible rect를 CSS viewport와 교차한 뒤 physical surface로 변환한다. readback은 CSS clip 바깥 sample이 검정인 것을 확인했고 viewport 교차는 vertex 코드에서 확인했다. surface 크기 0은 준비 단계에서 거부한다. **코드와 clip 경계 확인.**
12. **glyph bearing·baseline·origin 또는 device scale을 누락해 위치가 달라지는 상태** — bitmap의 raster origin/baseline/left/top, CSS origin과 양수 scale을 snapshot이 보존하고 vertex 생성에서 결합한다. fixture는 scale 1을 고정하고 두 simulator 로그에서 viewport와 scale을 대조했다. 다른 scale 조합은 이 slice에서 행렬화하지 않았다. **제한 범위 확인.**
13. **mask atlas 행이 겹치거나 row 전환 뒤 coverage가 섞이는 상태** — CPU packer는 glyph별 위치를 명시하고 mask row를 복사한다. `atlas_rows_keep_each_glyph_coverage_and_cache_dimensions_are_part_of_key`가 배치와 byte 배열을 검사했다. **확인.**
14. **device의 texture 너비·높이를 넘는 atlas 제출** — packer가 device max dimension을 사용하고 각 glyph와 추가 row를 거부한다. 이번 검토에서 너비 초과와 누적 행 높이 초과 negative controls 두 개를 추가했고 둘 다 통과했다. **확인.**
15. **허용된 64 MiB atlas byte 한도 초과** — `width × height`를 checked 계산하고 할당 전에 byte 한도를 적용한다. 64 MiB 경계 자체를 대량 할당하는 시험은 실행하지 않았다. **정적 코드 확인, 경계 부하 실행은 미검증.**
16. **동일 mask bytes지만 atlas 차원이 다른 key 충돌** — cache digest에 너비·높이와 pixel bytes 모두 포함한다. 동일 bytes/다른 shape negative control이 key 불일치를 확인했다. 암호학적 SHA-256 충돌 공격은 이 로컬 cache 계약의 현실적인 oracle가 아니므로 별도 시험하지 않았다. **확인.**
17. **cache 제한을 넘길 때 현재 draw가 참조 중인 texture 해제** — cache는 `Arc<GpuGlyphAtlas>`를 쓰고 `PreparedGlyphs`도 texture 소유 Arc를 유지한다. 제한 초과 시 cache entry를 지워도 해당 frame의 bind group/texture가 pass 수명까지 살아 있다. 코드 소유권을 추적했다. **코드 확인; 64 MiB eviction pressure 실행은 미검증.**
18. **shader, texture 또는 vertex buffer 생성의 WGPU validation/OOM 오류가 숨겨짐** — pipeline·atlas·확장 vertex buffer 생성에 validation 및 OOM error scope를 두고 실패를 반환한다. 의도적 GPU OOM injection은 수행하지 않았다. **오류 경로 코드 확인, OOM 재현은 미검증.**
19. **vertex 개수·byte 수·draw range 변환 overflow 또는 reserve 실패 뒤 잘린 draw** — 정점 index와 byte 크기를 checked 변환하고 host `try_reserve` 실패를 반환하며 GPU create validation을 확인한다. atlas max dimension control과 실제 36 glyph batch가 확인됐다. million-glyph 메모리 압력은 실행하지 않았다. **일반 경계 확인, 대형 압력은 미검증.**
20. **alpha mask, text alpha 또는 opacity group에서 source-over 의미가 바뀌는 상태** — 기본 2×2 WGPU readback은 partial coverage byte를 exact output으로 확인한다. 추가한 opacity group readback은 `RuntimeSrgbColor` alpha 0.5와 group opacity 0.5를 겹쳐 검사한다. text scene을 precise compositor fast path에서 제외해 처리 불가능한 `DrawText`가 해당 경로에 들어가지 않도록 했다. iOS Simulator route도 실행했으며 링크 경고는 README에 성공 상태와 분리해 기록했다. **확인.**

## 발견과 잔여 경계

검토 중 `InvalidPaintCommands` 사용자 진단이 box 명령만 언급해 text command 오류를 가리키지 못하는 문구 결함을 고쳤다. message에 text와 group 경계를 포함했다. 추가한 alpha group readback, scroll-owner rejection, atlas dimension rejection 시험은 모두 통과했다. 전체 render crate는 87+38+3+4개 시험, FFI fixture는 44개 시험이 통과했다.

fixture는 고정 font bytes와 app route로만 text를 연결한다. DOM text extraction, computed font/style projection, layout intrinsic consumer, 조상 overflow·scroll transform, hit-test와 font fallback은 이 검토의 통과 항목이 아니다. simulator 화면 capture는 OS screen screenshot이며 PixelCopy 또는 Metal drawable pixel readback이 아니다. WGPU offscreen readback은 macOS에서 실행한 작은 mask 표본 시험이다.
