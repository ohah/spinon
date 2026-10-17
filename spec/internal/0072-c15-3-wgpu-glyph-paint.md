# 0072 · C15.3 runtime glyph mask WGPU paint

**버전:** `0.1.0` · 내부 초안<br>
**상태:** 고정 글꼴 fixture에서 제한된 renderer slice · 공개 CSS text 지원 아님<br>
**연결:** [C15 계획](../../plan/c15-inline-text-layout.md) · [공식 상태 대장](../STATUS.md#css-구현-체크리스트) · [실행 근거](./evidence/c15-3-wgpu-glyph-paint-2026-10-17/README.md) · [선행 glyph raster 계약 0071](./0071-c15-3-glyph-raster-snapshot.md)

## 범위

이 단계는 `spinon-text-layout`의 고정 글꼴 glyph alpha mask를 `RuntimeRenderText` paint snapshot에 보관하고, `DrawText` command 순서에 따라 WGPU renderer가 R8 coverage atlas로 실제 색을 그리는 내부 경로다. glyph mask의 알파 coverage와 단색 text color를 합성하며 layout·font registry·render revision을 snapshot에 묶는다. 이 fixture route는 C15.3 내부 검증용이며 product HostDocument가 DOM text를 admit하거나 style projection이 text를 소비하는 계약이 아니다.

## 입력과 paint 동작

- 앱 fixture는 고정 C19.2 runtime box scene에 등록된 Noto Sans KR font bytes와 `C15 glyph paint coverage 한글 overflow sample` 문장을 연결한다. system font fallback에 의존하지 않는다.
- fixture 환경은 360×240 CSS px, DPR 1, 고정 x/y origin과 sRGB foreground color를 사용한다. text glyph가 owner box 밖에서 그려지는 것을 막도록 snapshot에 반열린 CSS x clip을 전달한다.
- glyph bitmap은 한 byte-per-pixel alpha coverage다. atlas는 glyph dimensions·pixel bytes를 구별하는 cache key를 사용하며, draw shader가 coverage와 text alpha를 곱해 foreground를 background 위에 합성한다.
- 각 glyph의 physical placement, CSS origin, baseline, DPR, source revision, owner `NodeId`, paint order와 clip을 runtime snapshot이 소유한다. 범위를 넘는 bitmap·glyph 수·invalid geometry·stale source·없는 owner·duplicate owner·scroll owner/ancestor는 snapshot 생성 중 거부한다.
- clip된 quad는 geometry와 UV를 함께 잘라내므로 경계 바깥 sample을 만들지 않아야 한다. clipped geometry와 paint order, atlas key, 실제 WGPU readback 표본을 focused tests에서 검사한다.

## 검증과 한계

- renderer/readback 테스트는 고정 alpha mask가 WGPU 색 표본으로 변환되는지, clip의 안쪽/바깥쪽 반열린 경계가 맞는지, paint command 순서와 opacity group 동작이 보존되는지 확인한다.
- Android API 37 ARM64 AVD와 iOS Simulator app route는 opt-in feature가 켜진 빌드에서만 실행한다. simulator screenshot은 실제 앱 화면을 보여주는 자료로 보존한다. screenshot만으로 drawable readback 또는 physical-device 동작을 주장하지 않는다.
- C15.3 fixture는 text를 root box에 시험용으로 붙이고 CSS clip을 직접 지정한다. DOM text extraction, computed font/style cascade, ancestor CSS overflow mapping, Block/Flex/Grid intrinsic measurement consumer, scroll transform, text hit-test, color glyph, fallback/async `@font-face`는 연결되지 않는다. 해당 범위는 이 문서나 앱 화면으로 완료되지 않는다.
- C15/C15.3은 layout consumer, 실제 simulator ink 및 clip evidence가 관련 acceptance와 함께 확인되기 전까지 미완료다. 내부 interface version은 `0.1.0`으로 유지한다.
