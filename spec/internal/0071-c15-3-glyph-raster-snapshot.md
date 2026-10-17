# 0071 · C15.3 고정 글꼴 glyph raster snapshot

**버전:** `0.1.0` · 내부 초안<br>
**상태:** 제한된 Rust raster 단계 · 앱 작성자 공개 API 아님<br>
**연결:** [C15 inline text plan](../../plan/c15-inline-text-layout.md) · [공식 상태 대장](../STATUS.md#css-구현-체크리스트) · [구현 근거](./evidence/c15-3-glyph-raster-2026-10-17/README.md)

## 범위

이 계약은 Parley가 만든 `TextLayoutSnapshot`의 glyph ID, 위치, cluster range와 원본 DOM source mapping을 Swash `=0.2.10`으로 rasterize해 immutable Rust snapshot에 보존하는 내부 단계다. glyph image를 `spinon-render` command, WGPU frame, HostDocument text admission, Block/Flex/Grid consumer에 연결하지 않는다. 이 문서와 raster pass는 C15.3 또는 공개 CSS text 지원 완료를 뜻하지 않는다.

## 입력과 사전 조건

- 호출자는 glyph를 만든 `TextLayoutEngine` 또는 그 clone 계열에서 snapshot을 rasterize한다. 서로 다른 engine은 등록 횟수가 같아도 snapshot을 공유할 수 없다. engine clone은 immutable font bytes와 registry identity를 공유한다.
- snapshot의 `FontRegistryRevision`은 현재 engine revision과 같아야 한다. 뒤에 font를 등록해 revision이 달라지면 이전 layout snapshot을 거부한다.
- `device_scale_factor`는 유한한 `(0, 64]`이다. 각 run의 `font_size × device_scale_factor`는 유한한 `(0, 512]` physical px 범위여야 한다.
- 각 run의 family에 대응하는 등록 font asset이 있어야 한다. 현재 raster adapter는 단일-face SFNT만 허용하고 face index 0을 사용한다. 두 번째 face가 있는 font collection은 거부한다. system font fallback을 조회하지 않는다.
- glyph ID는 0이 아니며 선택한 font의 `glyph_count`보다 작아야 한다. glyph origin·baseline·advance, cluster UTF-8 range와 DOM source range가 snapshot 안에서 유효해야 한다.

## 결과·좌표·소유권

- 결과 glyph 순서는 input run 순서와 각 run glyph 순서를 따른다. 각 항목은 run/glyph index, 원래 glyph ID, paragraph 기준 CSS px origin/baseline, physical pixel baseline anchor, cluster range, source mapping과 선택적 bitmap을 보존한다.
- bitmap 좌표는 baseline anchor에 상대적 physical pixel이다. `left_px`와 화면의 위쪽을 가리키는 음수 `top_px`, `width_px`/`height_px`가 placement를 정의한다. rasterizer는 physical origin의 fractional phase를 Swash에 전달하고 정수 anchor를 따로 보존한다.
- Swash source 순서는 palette 0 color outline, best-fit color bitmap, scalable outline이다. raster format은 Alpha이며 현재 결과는 8-bit alpha mask 또는 RGBA8 color content다. 공백과 정상적인 빈/무잉크 glyph(너비 또는 높이가 0이고 payload가 비어 있음)는 `bitmap: None`이다.
- alpha mask는 픽셀당 1 byte, subpixel/color RGBA8은 픽셀당 4 bytes다. dimension 곱은 overflow 검사하고 bitmap 하나는 최대 16 MiB, snapshot 전체는 최대 64 MiB다. raster 결과는 `Arc<[u8]>`로 소유하며 engine이나 Swash scratch buffer를 빌리지 않는다. 16 MiB 단일 bitmap 검사는 Swash가 결과를 만든 뒤 적용되므로 임의의 악성 font가 유도하는 내부 scratch allocation의 상한을 증명하지 않는다.
- rasterizer 호출마다 `ScaleContext`를 생성한다. 영속 glyph cache, cross-thread cache coherence와 renderer atlas 수명은 이 계약에 없다.

## 오류 경계

foreign registry, stale revision, invalid DPR, 허용 범위 밖 physical font size, 누락/손상된 font asset, 범위 밖 glyph ID, 무효 source mapping, 좌표 overflow, 손상/과대 bitmap과 결과 vector allocation 실패를 구별해 `TextLayoutError`로 반환한다. font asset 부재나 glyph 이미지 부재를 다른 font/system fallback으로 대체하지 않는다.

## 플랫폼·검증 한계

Pure Rust Swash raster 결과를 고정 font input으로 검사한다. OS text renderer, Skia/CoreText parity, color-management output, final WGPU sampling, alpha compositing, parent clip, overflow/scroll, hit-test, app runtime, Android AVD PixelCopy와 iOS Metal drawable은 이 단계에서 검증하지 않는다. antialiasing output을 Chrome screenshot 전체 유사도로 판정하지 않는다.

다음 연결 단계는 layout/runtime text admission, WGPU glyph draw command, paint order와 clip/hit-test consumer다. 이 단계에서 실제 frame ink를 만들고 Android/iOS Simulator의 text ink 및 border 바깥 overflow/clip을 검사할 때까지 C15.3은 미완료다.
