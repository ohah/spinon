# C04.10 구현 적대 검토

> 이 문서는 resize 실행 검증 전의 구현 snapshot이다. 당시 #19의 resize 미검증은 이후 [resize 왕복 실행](c04-runtime-css-to-gpu-resize-simulators-2026-10-09.md)으로 일부 해소됐고, 현재 판정은 [resize 구현 재검토](c04-runtime-css-to-gpu-resize-implementation-review-2026-10-09.md)를 따른다. shutdown pending draw와 WGPU 오류 주입은 여전히 남아 있다.

- 검토 대상: C04.10 현재 working tree, branch `feat/c04-runtime-css-to-gpu` (`HEAD e1ba2a0` 기반)
- 계획: [`C04.10 구현 계획`](../../../plan/c04-runtime-css-to-gpu.md) · SHA-256 `aef3566cbddca2a4037b06ffaf292ea1867248a678663d2dbbede9a146929205`
- 근거: [Android surface 수명주기](c04-runtime-css-to-gpu-android-lifecycle-2026-10-09.md), [Android 화면](c04-runtime-css-to-gpu-android-lifecycle-2026-10-09.png), [iOS 화면](c04-runtime-css-to-gpu-ios-26.2-2026-10-09.png), [iOS 실행 로그](c04-runtime-css-to-gpu-ios-26.2-2026-10-09.log), 고정 Chromium fixture 비교와 Rust/Bun 결과

## 서로 다른 실패 관점 20개

| # | 공격 관점 | 검토 결과 |
|---:|---|---|
| 1 | 동적 runtime scene을 fixture hash가 필수인 static snapshot과 섞는가 | 통과. `RuntimeRenderSnapshot`은 `StaticRenderSnapshot`과 별도 자료형이다. |
| 2 | document generation 또는 document revision이 다른 장면을 허용하는가 | 통과. `runtime_scene_rejects_stale_document_style_or_environment_revisions` 및 전체 key 비교가 거부한다. |
| 3 | render-tree·style·environment revision을 빠뜨리는가 | 통과. 장면 key와 adapter가 다섯 축 전체를 보존한다. |
| 4 | 스타일 계산과 Taffy 계산이 서로 다른 key여도 합치는가 | 통과. 완료 snapshot key와 현재 요청 key가 같아야 publish한다. |
| 5 | 중복·누락 node mapping이나 DOM preorder 불일치가 부분 장면이 되는가 | 통과. adapter는 node 집합·mapping·순서를 검증하고 실패 시 전체를 거부한다. |
| 6 | 노드 ID 숫자 순서를 DOM paint 순서로 사용하는가 | 통과. DOM preorder를 별도 paint order로 전달하고 관련 Chromium fixture가 확인한다. |
| 7 | Stylo 계산값 대신 CSS 문자열을 다시 파싱해 색을 만드는가 | 통과. typed computed paint만 받아들이며 테스트가 재파싱 경로 부재를 확인한다. |
| 8 | C04.9 layout JSON이 새 paint property 때문에 느슨해지는가 | 통과. paint는 `RuntimeFlexPaintV1`에만 허용하고 기존 strict profile을 유지한다. |
| 9 | 부분 alpha, gradient, image를 조용히 불투명 색으로 바꾸는가 | 통과. alpha·미지원 paint 값은 실패하며 배경의 transparent 상태를 보존한다. |
| 10 | `display:none`, 0 크기 또는 transparent 노드가 잘못 그려지는가 | 통과. 숨김·0 면적은 geometry에서 빠지고 transparent는 paint 없음이다. |
| 11 | NaN·무한대·음수 크기·viewport 산술 overflow가 GPU buffer에 들어가는가 | 통과. typed frame 및 geometry 생성 경계가 실패 폐쇄한다. |
| 12 | surface format에 따라 sRGB 인코딩이 빠지거나 두 번 적용되는가 | 통과. RGBA/BGRA sRGB 선택, UNORM 대체와 색 인코딩 단위 테스트가 있다. |
| 13 | Chromium 색상 기준이 화면 screenshot만으로 판정되는가 | 통과. computed color와 offscreen 내부 픽셀 표본을 별도 판정하고 screenshot은 표시 확인에만 쓴다. |
| 14 | 빈 root가 stale scene을 남기거나 geometry가 없는 상태에서 실패하는가 | 통과. 빈 장면 offscreen readback이 clear RGBA `[0, 0, 0, 255]`를 반환한다. |
| 15 | 새 요청 중 이전 sequence의 scene이 publish/표시되는가 | 통과. publish 경쟁 테스트가 stale sequence를 거부하고 renderer는 submit/present 경계에서 sequence를 확인한다. |
| 16 | Android surface callback 반환 후 render queue가 이전 `Surface`를 계속 쓰는가 | 결함 수정 후 통과. `surfaceDestroyed`에서 draw admission을 끊고 renderer 파괴 latch까지 기다린다. API 37 회전 재생성 로그에서 destroy 완료가 callback 반환보다 먼저 기록됐다. |
| 17 | Android Activity 종료가 renderer 정리 전에 callback이나 owner를 해제하는가 | 결함 수정 후 통과. dispose가 runtime/render executor 종료를 기다린 뒤 callback을 제거한다. 실제 emulator 종료 로그에서 양 queue drain을 확인했다. |
| 18 | iOS에서 UIKit view/layer를 background queue에서 접근하거나 먼저 해제하는가 | 코드·실화면 통과. surface 구성은 main thread이고 renderer 종료가 끝날 때까지 canvas view를 강하게 유지한다. iOS 종료 중 pending draw는 별도 주입하지 않았다. |
| 19 | 실제 surface 크기가 바뀌는 중 이전 size의 장면이 제출되는가 | 코드 경로는 sequence invalidation·draw 차단·render queue barrier·main-thread configure·generation 재확인 순이다. **Android/iOS simulator에서 drawable 크기 변경을 실제로 만들지는 못해 실행 검증은 미완료다.** |
| 20 | GPU draw 실패·pending task shutdown이 복구 상태를 성공으로 오인하는가 | 오류는 별도 오류 상태로 전달하도록 되어 있다. **WGPU draw 실패를 주입한 실행과 종료 직전 pending draw를 강제한 실행은 아직 없다.** |

## 수정한 결함

Android `SurfaceHolder.Callback.surfaceDestroyed`가 renderer destroy를 render queue에 예약하고 곧바로 반환해, OS가 surface를 무효화하는 시점과 이전 draw가 겹칠 수 있었다. callback에서 surface 사용을 무효화하고 render queue의 대기 작업과 renderer 파괴가 끝날 때까지 기다리도록 바꿨다. Activity dispose도 runtime/render queue drain 후 callback을 해제한다.

## 실행 검증

- `cargo test --locked --workspace --features spinon-ffi/c04-runtime-gpu` 통과. macOS WGPU readback에서 Chromium 내부 색 표본과 빈 장면 clear 색을 대조했다.
- `cargo clippy --locked --workspace --all-targets --features spinon-ffi/c04-runtime-gpu -- -D warnings` 통과.
- `mise exec -- bun run test:js` 2개 통과, `mise exec -- bun run test:css-reference` 21개 통과.
- Android API 37 ARM64 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 같은 V8 fixture를 실제 화면에 표시했다. light/dark 환경 revision 갱신도 두 플랫폼에서 확인했다.
- Android emulator는 ANGLE/Vulkan SwiftShader 소프트웨어 장치로 표시했다. 실기기·하드웨어 성능 근거가 아니다.
- manifest/package/contract 출시 버전은 올리지 않았고 내부 계약은 `0.1.0`이다.

## 완료 판정

계획의 필수 실행 검증 중 크기가 달라지는 surface resize, WGPU draw 실패 주입, 종료 중 pending draw 재현이 남아 있다. 따라서 구현은 두 simulator에서 화면에 표시되지만 C04.10 전체 완료나 PR merge 준비로 표시하지 않는다.
