# C13.2 visible overflow 표면 픽셀 구현 실패 관점 검토 20개

- **대상:** C13.2 고정 다섯 표본의 C ABI, WGPU surface readback, Android PixelCopy, iOS Metal drawable 경로.
- **검토 시점:** 구현과 run5 재검증 뒤.
- **대조 근거:** C13.2 FFI·renderer·Android/iOS adapter 코드, 고정 Chrome 155 observation, run5 로그/PNG, opt-in Rust 검사와 Android/iOS Simulator 실행.
- **판정:** 아래 20개 실패 경로를 확인했다. stale draw가 제출 전 끝날 때 예약 표본이 다음 화면으로 넘어갈 수 있어 요청 정리와 회귀 검사를 추가했다. 남은 OS OOM·실기기·compositor 한계는 완료 근거로 확대하지 않는다.

| # | 구현을 깨뜨릴 독립 실패 관점 | 확인 근거와 판정 |
|---|---|---|
| 1 | C13.2 FFI 선언·구현이 기본 제품 artifact로 새어 나온다. | Cargo `c13-2-scroll-fixture-test`, Rust fixture module, C header/JNI/Objective-C++ define gate를 대조했다. default archive에는 신규 C13.2 surface-pixel symbol과 전용 문자열이 없고 opt-in positive control에는 두 symbol과 문구가 있다([feature boundary](./c13-2-visible-overflow-surface-pixels-2026-10-15/feature-boundary.md)). **PASS** |
| 2 | host 생성 시 null 또는 용량 0의 출력 버퍼를 전달해 쓰기 오류나 잘못된 host를 만든다. | 생성 FFI는 null/0을 즉시 거부하고, report 복사가 실패하면 로컬 host를 반환하지 않는다. FFI 단위 검사에서 null/0을 확인했다. **PASS** |
| 3 | 호출자가 0·4·6·`usize::MAX` 표본을 보내 고정 fixture의 길이 계약을 깬다. | 정확히 5개인지 포인터 처리보다 먼저 검사하고 좌표 수 곱셈에 `checked_mul`을 쓴다. 해당 네 count의 거부를 opt-in FFI test에서 확인했다. **PASS** |
| 4 | 표본 count는 5지만 좌표 포인터가 null인데 slice로 읽는다. | null sample pointer는 `from_raw_parts`보다 먼저 거부한다. `from_raw_parts` 유효 범위는 한글 `# Safety` 계약에 호출자 책임으로 명시했다. **PASS — 코드 순서 검토** |
| 5 | 같은 좌표를 여러 표본 ID에 재사용해 표본 누락을 숨긴다. | FFI가 앞선 좌표쌍과 중복을 비교해 거부하며, 중복 좌표 호출이 host 접근 전 실패하는 단위 검사가 있다. **PASS** |
| 6 | null host 또는 pixel-readback 전용이 아닌 일반 host에서 표본을 예약한다. | 인자 검증 뒤 null host를 거부하고, runtime host는 `surface_pixel_readback_test` 플래그가 켜진 경우에만 readback을 허용한다. 잘못된 FFI 입력 단위 검사와 전용 host constructor를 대조했다. **PASS** |
| 7 | pixel-test host가 C13.2 scroll matrix와 다른 session/document를 캡처한다. | readback 전용 constructor는 `new_c13_2_scroll_fixture()` 결과를 재사용하고 그 host에 readback flag만 설정한다. run5 두 플랫폼에서 `boxes=26`, document `583`, tree `124`, scroll `8`을 기록했다. **PASS** |
| 8 | `COPY_SRC` 사용이 없는 surface를 읽거나 일반 renderer 초기화 동작을 바꾼다. | UIKit copy-src surface 초기화는 C13.2 opt-in cfg 분기 안에 있고, 동일 cfg의 반대 분기는 원래 초기화를 유지한다. run5 iOS drawable readback이 통과했다. **PASS** |
| 9 | 이전 표본이 아직 소비되지 않았는데 새 요청이 덮어써 frame identity를 잃는다. | renderer는 outstanding `pending_surface_pixel_samples`가 있으면 새 요청을 거부한다. 요청 직후 draw report에서만 결과를 꺼내며 iOS adapter가 동일 presentation sequence인지 재검증한다. **PASS — 코드와 run5 로그 대조** |
| 10 | 표본 좌표가 surface 경계를 넘거나 지나치게 많은 GPU copy를 만든다. | WGPU readback은 실제 configured width/height로 각 point를 검사하고 1–64개로 제한한다. C13.2 adapter는 고정 다섯 표본이며 run5 좌표 모두 실 drawable 범위 안이다. **PASS** |
| 11 | 요청 뒤 presentation이 바뀌어 제출 전 draw가 stale가 되어도 좌표가 다음 frame으로 유출된다. | 검토 중 발견한 경로다. 첫 `is_current()` stale 반환에서 pending/completed 표본을 지우고, 같은 정리 helper의 회귀 검사를 추가했다. `stale_draw_discards_pending_and_completed_pixel_samples` 통과. **PASS — 수정 후** |
| 12 | GPU 제출 뒤 current-check가 실패했는데 이미 예약한 표본이 다음 draw로 이어진다. | post-submit stale 경로도 readback을 포함한 경우 pending/completed 상태를 폐기한다. 추가한 정리 helper로 같은 invariant를 강제한다. **PASS — 코드 검토** |
| 13 | texture copy 생성 또는 GPU map 대기 실패 뒤 요청이 renderer에 남는다. | `append_copy`와 `map_samples` 실패도 표본 상태를 지우고 오류를 반환하도록 처리했다. 동일 host에서 다음 요청을 막는 stale state가 남지 않는다. **PASS — 코드 검토** |
| 14 | 표본별 GPU buffer row가 WGPU 정렬 요구와 맞지 않아 다른 픽셀을 읽는다. | 한 점당 `COPY_BYTES_PER_ROW_ALIGNMENT` stride를 쓰고 크기·offset 곱셈에 checked arithmetic을 적용한다. 기존 renderer 단위 테스트가 stride 관련 decode 경계를 포함한다. **PASS** |
| 15 | BGRA surface를 RGBA로 오인하거나 alpha를 바꾼다. | 지원 포맷을 8-bit RGBA/BGRA로 제한하고 BGRA의 R/B만 교환한다. format/decode 단위 검사와 Android RGBA, iOS `Bgra8UnormSrgb` run5 결과가 일치했다. **PASS** |
| 16 | iOS 표본 좌표가 1× 가정 또는 screen scale을 사용해 drawable 좌표와 어긋난다. | CSS pixel cell center에 실제 drawable/400×240 ratio를 곱하고 좌표·scale/bounds를 확인한다. run5 `1200×720`, scale `3×3`, 다섯 고정 좌표가 exact다. **PASS** |
| 17 | Metal readback을 다른 process, surface generation, presentation, 크기 또는 format에 귀속한다. | request와 draw 양쪽에서 PID/run ID, generation, sequence, drawable size/format, 26-box identity와 revisions를 연결한다. run5 begin/summary와 simulator 로그가 일치한다. **PASS** |
| 18 | iOS draw report에 표본 JSON이 누락·중복·잘못된 형태인데 성공으로 보고한다. | JSON 파싱, 정확히 5개 개수, 고정 순서 좌표, RGBA, escape 위치를 검사하고 하나라도 틀리면 mismatch를 반환한다. run5 log는 다섯 sample exact와 `status=matched`다. **PASS** |
| 19 | Android가 앱 window screenshot 또는 오래된 SurfaceView 세대에서 표본을 읽는다. | `PixelCopy.request(surfaceView, ...)`에 실제 surface dimensions를 주고 callback 전후의 run/PID/generation/presentation/size를 재확인한다. run5 AVD log의 PixelCopy source는 `1050×630` SurfaceView다. **PASS** |
| 20 | 저장 PNG가 손상·덮어쓰기됐는데 callback의 메모리 bitmap만 보고 통과한다. | run ID 기반 파일은 `createNewFile()`로 충돌을 거부하고, 디스크 PNG를 다시 decode해 크기·표본을 읽으며 SHA-256을 기록한다. run5 PNG는 `1050×630`, `e7f61a…b88192`이고 재비교에서 5/5 exact다. **PASS** |

## 남은 검증 경계

PixelCopy OS timeout/OOM과 scheduler 경합을 강제로 재현하지 않았다. Android 결과는 API 37 ARM64 AVD/SwiftShader, iOS 결과는 iOS 26.2 Simulator의 compositor 제출 전 Metal drawable이다. 실기기·hardware GPU·compositor 이후 display scanout은 이 구현 검토의 통과 범위가 아니다.
