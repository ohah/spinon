# C15.3 owner text clip PR 변경 실패 관점 검토 30개

**대상:** `073c74f7bd3140f9cdefbac70a344dd2db3d240c` 이후 현재 변경 전체. 2026-10-17
**구분:** PR diff·ABI·플랫폼 경로·상태 문서의 회귀 질문을 각각 검토했다. [구현 실패 관점 20개](./implementation-review-20.md)와 별도 기록이다.

## 검토

1. **owner clip 변경이 C15.3 전체 완료로 표시됨** — `spec/STATUS.md`, C15 계획과 공개 로드맵에서 parent 항목은 계속 미완료다. 이 diff는 고정 fixture의 제한된 owner clip slice만 추가한다. **허위 완료 없음.**
2. **text가 없는 기존 snapshot에서 새 검증이 box-only 동작을 바꿈** — text 목록이 비어 있으면 owner 검증은 추가 명령을 만들지 않는다. 기존 box/image 경로에 대한 별도 변경은 없다. renderer 회귀 테스트 결과는 [Rust 출력](./rust-render-tests.txt)에 보존했다. **기존 경로 보존.**
3. **owner NodeId가 scene box에 없는데 배열 인덱스로 잘못 접근** — owner 매핑과 index 범위 확인 후 clip을 적용한다. `text_snapshot_checks_bitmap_geometry_source_owner_and_unique_node`에서 없는 owner를 거부한다. **오류 반환.**
4. **한 NodeId에 여러 text가 붙어 paint 소유권이 모호해짐** — 중복 owner는 clip 적용 전에 거부한다. 같은 snapshot 테스트가 중복을 검사한다. **오류 반환.**
5. **text source revision과 snapshot revision이 달라도 owner clip 계산을 통과** — source revision 검증이 clip 적용보다 앞선다. stale source case가 snapshot 검증에서 실패한다. **오류 반환.**
6. **scroll metadata 배열 길이가 boxes보다 짧아 panic** — 길이 및 owner index 유효성을 text iteration 전에 검사하고 `InvalidScrollState`를 반환한다. **panic 경로 없음.**
7. **같은 index의 scroll metadata가 다른 box frame을 가리킴** — metadata frame이 box와 다르면 owner clip으로 쓰지 않고 거부한다. **잘못된 owner 상속 없음.**
8. **동일 owner에 scroll container가 중복 등록되어 순서에 따라 선택** — snapshot 생성 시 중복 scroll owner를 거부한다. **모호한 선택 없음.**
9. **scroll ancestor metadata는 있으나 실제 container가 없어서 clip을 건너뜀** — 누락 container를 `InvalidScrollState`로 처리한다. `text_rejects_scroll_ancestor_metadata_without_a_matching_container`가 검사한다. **조용한 fallback 없음.**
10. **text 자체가 scroll container인데 transform 없이 clip만 적용** — text owner의 scroll container 소유를 `UnsupportedTextScroll`로 명시 거부한다. `text_owner_that_is_itself_a_scroll_container_is_rejected`가 검사한다. **지원 과장 없음.**
11. **가로 scroll offset을 무시해 잘못된 위치의 글리프를 그림** — x offset 1 CSS px는 `UnsupportedTextScroll`로 거부된다. **잘못된 transform 없음.**
12. **세로 scroll offset만 검증에서 빠짐** — 별도 y offset 1 CSS px 테스트가 같은 거부를 검사한다. **축별 불일치 없음.**
13. **offset 0인 scroll ancestor가 정적 clip까지 제거** — 0 offset ancestor에서는 유효 paint clip을 유지한다. `text_inside_an_unscrolled_overflow_owner_keeps_its_paint_clip`이 검사한다. **정적 clip 보존.**
14. **text에 이미 더 좁은 clip이 있는데 owner clip이 text clip을 확장** — text clip은 owner box의 유효 clip과 교차할 뿐이다. x/y 교집합 테스트가 양 축 경계를 검사한다. **clip 확장 없음.**
15. **조상 overflow clip을 놓쳐 hidden glyph가 parent 밖에 표시** — owner box의 effective `clip_css_px()`를 text clip에 반영한다. paired visible/hidden fixture에서 hidden 바깥 표본을 별도로 확인한다. **고정 fixture 통과.**
16. **overflow visible text가 owner border에 임의로 잘림** — constrained clip이 없는 owner에 box frame을 강제로 대입하지 않는다. visible-owner 테스트와 Android/iOS 표면 표본이 parent 밖 ink를 확인한다. **고정 fixture 통과.**
17. **새 clip 검증이 DrawText index/order를 바꿈** — 기존 paint command index 전체성·순서 검증은 유지되며 관련 renderer/WGPU suite가 통과했다. **순서 회귀 없음.**
18. **owner 경계 표본이 glyph가 아니라 임의 배경 좌표에 놓임** — fixture는 Ahem raster mask에서 coverage 255를 찾아 표본을 고른다. 좌표는 [실행 README](./README.md)와 원본 플랫폼 로그에 기록했다. **실제 glyph 내부 표본.**
19. **hidden 내부 표본이 parent 배경색만으로 합격** — 판정은 non-background 조건이 아니라 exact ink `[9,33,112,255]`를 요구한다. 이전 false-positive 입력을 고치고 두 simulator에서 재실행했다. **배경 오인 방지.**
20. **hidden parent 밖에 남은 glyph가 stage 배경으로 가려져도 합격** — 바깥 표본은 exact stage background `[247,249,252,255]`를 요구한다. PixelCopy와 Metal drawable이 각각 일치한다. **잔류 잉크 검출.**
21. **CSS px 표본을 surface pixel로 환산하며 밀도 값에 의존** — Android와 iOS 모두 고정 viewport 대 실제 surface 크기의 좌표 변환을 검증하고 sample 범위를 먼저 확인한다. 실제 크기와 표본은 README 및 로그에 남아 있다. **유효 범위 내 변환.**
22. **WGPU readback test가 owner frame 안에서만 픽셀을 확인** — 회귀 테스트는 owner를 1×1로 줄이고 unclipped ink가 바깥 표본에 남는지 읽는다. **경계 밖 paint 검사.**
23. **C ABI 표면 요청에서 sample 개수·null 포인터·byte 크기 오류를 허용** — opt-in entry point는 고정 count와 포인터, checked multiplication, caller buffer 길이를 검증한다. [FFI 테스트](./ffi-tests.txt)가 통과했다. **잘못된 입력 거부.**
24. **GPU readback을 일반 renderer 경로에서 호출 가능** — renderer readback 진입점은 fixture-only 상태와 유효 renderer를 확인하고 feature cfg로 격리한다. **일반 런타임 API 확장 없음.**
25. **Android PixelCopy가 이전 surface generation이나 다른 크기를 읽음** — 요청은 현재 surface/renderer/size와 연결하고 PixelCopy 응답을 기다린다. Android 원본 log의 run ID·표본·PixelCopy `3/3`을 확인했다. **현재 AVD surface 일치.**
26. **Android report JSON의 중복·미지·좌표 밖 표본을 허용** — Java parser는 이름 집합, 중복, coordinate range, paired coordinate 및 RGBA 조건을 검사한다. **잘못된 report 거부.**
27. **Android 화면 캡처를 PixelCopy readback 증거로 혼용** — PixelCopy PNG와 OS screenshot을 별도 파일·역할로 기록했다. 색 합격은 PixelCopy report에서만 판정한다. **증거 역할 분리.**
28. **iOS readback 결과가 stale Metal drawable 또는 다른 format에 속함** — Swift 경로는 generation, drawable size 및 `Bgra8Unorm`을 대조하고 CSS 좌표를 surface 좌표로 환산한다. iOS simulator log에서 세 조건과 `3/3`을 확인했다. **현재 drawable 일치.**
29. **기본 FFI/플랫폼 artifact에 테스트 ABI나 fixture font bytes가 포함** — default와 opt-in artifact를 비교해 기본 symbol·payload 부재 및 opt-in 존재를 확인했다. [artifact 경계 기록](./artifact-boundary.txt)에 검사 방식과 Apple `nm` 한계를 적었다. **기본 artifact 격리.**
30. **기기·화면·문서 근거가 구현 범위를 넘어 support 완료를 주장** — API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator만 기록하고 실기기·hardware GPU·iOS compositor 이후·일반 DOM text/style admission은 미검증으로 남겼다. 공식 상태와 계획도 C15.3 미완료를 유지한다. **범위 한정.**

## 결과

30개의 서로 다른 PR 변경 실패 질문을 코드, 저장된 실행 결과, 기본/opt-in artifact, 상태 문서에 대조했다. 고정 fixture의 owner overflow clip은 두 simulator에서 확인했지만 일반 DOM/style 연결이나 C15.3 전체 완료로 확대하지 않는다.
