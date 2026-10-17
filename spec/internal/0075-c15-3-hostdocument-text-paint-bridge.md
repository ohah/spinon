# 0075 · C15.3 HostDocument text layout → runtime glyph paint

**버전:** `0.1.0` · 내부 초안

**상태:** 고정 fixture의 HostDocument→Taffy text snapshot→runtime glyph paint 구현 및 Android/iOS Simulator surface 검증 통과 · 상위 C15.3/C15 미완료

**연결:** [C15 계획](../../plan/c15-inline-text-layout.md) · [공식 상태 대장](../STATUS.md#css-구현-체크리스트) · [고정 Chrome oracle와 판정](./evidence/c15-3-hostdocument-text-paint-precomparison-2026-10-17/README.md) · [C15.3 runtime glyph paint](./0072-c15-3-wgpu-glyph-paint.md)

## 범위

이 계약은 opt-in C15.3 fixture-test에서 fixture 소유 `HostDocumentSnapshot`의 직접 text child를 `LayoutInput::from_host_document_with_text_paragraphs`로 admission하고, Taffy가 만든 `LayoutOutput.text_layouts`의 glyph와 revision을 runtime text paint까지 보존하는 제한 bridge를 정의한다. live V8 DOM에서 일반 text node를 admission하거나 CSS font style을 계산하는 기능은 정의하지 않는다.

## 고정 입력과 layout 결과

- viewport와 root `div`는 `320×240 CSS px`다. root의 layout은 Block, 고정 크기, `overflow: visible`이다.
- paragraph `p`는 root의 직접 child이며 Block, `width: 15px`, `height: auto`, 양 축 `overflow: visible`이다. margin, padding, border는 0이고 paragraph 안에 직접 text node `XX` 하나가 있다.
- `ParagraphStyle`은 고정 Ahem font, `font-size: 10px`, `line-height: 10px`, normal white-space/word-break, LTR와 normal ligatures를 사용한다. system font fallback은 사용하지 않는다.
- 고정 입력의 layout 결과는 paragraph frame `0,0,15,10`, 한 줄 text advance `20px`, 두 glyph advance 각각 `10px`, owner 밖 horizontal overflow `5px`다. 단어 내부 break가 없으므로 text는 한 줄에 유지된다.
- `LayoutInput`, `LayoutOutput`, paragraph `LayoutTextSnapshot`은 같은 layout revision을 가져야 한다. `LayoutSourceRevision::HostDocument`는 fixture가 만든 snapshot의 generation·document revision·render-tree revision과 모두 일치해야 한다.
- 원 text node는 UTF-16 `[0,1]`, `[1,2]` source span과 paragraph UTF-8 `[0,1]`, `[1,2]` span으로 보존된다. 두 shaped glyph의 각 advance·x origin·source는 그 spans와 일치해야 한다.

## Raster와 runtime paint

- 같은 `TextLayoutEngine` 및 같은 Ahem font registry로 paragraph layout과 glyph raster를 계산한다. raster의 paragraph, font registry revision, glyph ID, source span은 입력 `LayoutTextSnapshot`과 일치해야 한다.
- 각 raster glyph는 alpha-mask bitmap을 가진다. runtime glyph bitmap은 해당 raster snapshot의 크기, origin, baseline, left/top bearing과 pixel buffer를 보존한다.
- runtime owner는 paragraph `NodeId`이며 origin은 같은 layout output의 paragraph `LayoutFrame`에서 가져온다. runtime text의 clip은 이 fixture에서 unconstrained이고 paint command 순서는 root box, paragraph box, text다.
- runtime scene과 text는 호출 runtime의 `RuntimeRenderKey`를 보존한다. HostDocument geometry/source revision은 fixture snapshot에 기록하며 이를 live runtime DOM revision으로 주장하지 않는다.
- report는 source revision, owner frame, glyph advances, UTF-16 span, overflow 폭과 glyph raster에서 계산한 여섯 개의 고유 sample 좌표를 보존한다. 처음 두 좌표는 `x < 15`, 뒤 네 좌표는 `x >= 15`이고 모두 `320×240` viewport 안이어야 한다.

## 실패와 fixture 경계

HostDocument 생성/commit, layout input 생성, Taffy layout, source·revision 불일치, 예상치 않은 line/glyph/source mapping, font registry mismatch, raster bitmap 누락 또는 runtime scene 조립 실패는 error로 반환하고 paint scene을 게시하지 않는다. FFI host constructor는 null/zero-length output을 거부하고, diagnostic report가 output buffer에 들어가지 않으면 host를 반환하지 않는다.

Rust module, C ABI, Android JNI adapter와 iOS Objective-C++ selector는 `c15-3-wgpu-glyph-paint-fixture-test` opt-in에 묶인다. Android adapter는 전용 fixture-test Java source set에만 둔다. 실제 default/opt-in Rust archive scan에서 기본 archive에는 C15.3 FFI symbol과 Ahem/Noto Sans KR bytes가 없고 opt-in positive control에는 둘 다 포함됨을 확인했다. [구현 근거와 전체 로그](./evidence/c15-3-hostdocument-text-paint-implementation-2026-10-17/README.md)

Android 검증은 Android 17/API 37 ARM64 AVD `PixelCopy`로 고정 `#092170` opaque glyph sample 6개를 읽었다. status는 `matched`; owner 안쪽 2/2와 오른쪽 경계 바깥 4/4가 정확한 잉크 RGBA였다. iOS는 iPhone 17 Pro/iOS 26.2 Simulator Metal drawable에서 동일 여섯 CSS 표본을 읽어 6/6을 확인했고 `renderer=metal-drawable-before-compositor`로 기록했다. 첫 iOS 실행은 Swift C13 route validator가 새 HostDocument argument를 거부하는 실제 결함을 노출했다. validator allowlist와 회귀 검사를 고친 후 iOS app을 다시 빌드하고 성공 화면·Metal log를 수집했다. 최종 Rust source 분리와 UI 수정 뒤 두 simulator를 새 바이너리로 다시 실행했다. screenshot은 UI 근거이고 각 표면의 별도 readback log가 pixel 판정 근거다. [실행·최종 화면·hash·artifact 경계](./evidence/c15-3-hostdocument-text-paint-implementation-2026-10-17/README.md)

## 제외 범위

live V8 DOM과 CSSOM, generic DOM text admission, style-to-font projection, 일반 line breaking/shaping, fallback·font loading, intrinsic sizing 상호작용, wrapping text, glyph별 hit target, DOM event dispatch, 실기기, Android hardware GPU와 iOS compositor 이후 surface는 별도 구현·계약 대상이다. 이 제한 fixture는 C15.3, C15, C16 또는 C19의 완료를 뜻하지 않는다.
