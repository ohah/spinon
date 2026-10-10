# C06.4 · 글꼴 상대 길이 단위

- **문서 유형:** 구현 계획 · 공식 상태는 spec/STATUS.md에서 관리
- **상위 계획:** C06 값·단위 변환
- **상태:** 구현·검증 완료 · 현재 작업 브랜치 미병합 · cascade 전용 합성 HTML 문서 루트 사용
- **내부 계약 숫자 버전:** 출시 전 0.1.0 고정
- **내부 문서 ID:** 0040 · 버전 번호가 아니다
- **선행:** C06.1–C06.3 typed percentage·absolute CSS px 값, C04 runtime cascade, C05 custom-property/stylesheet profile
- **비교 모델:** Chromium 154.0.8037.98, CSS viewport 320×800, CSS px 좌표, DPR 1·2 쌍 fixture
- **검토 근거:** 계획 실패 경로 20개와 별도 구현 실패 경로 20개를 각각 기록했다.

## 문서 루트 결정

CSS cascade가 사용하는 가상 Document 아래에 합성 HTML `<html>` 요소를 둔다. 이 요소가 `html` 및 `:root` 선택자와 `rem`의 문서 루트다. 기존 HostRoot 직속 요소는 CSS 문서 루트가 아니라 앱 mount subtree로 유지한다. 합성 요소는 Stylo cascade에만 존재하며 HostDocument의 NodeId, Taffy 입력/출력, layout frame, GPU paint node를 만들지 않는다. `HostDocument`와 공개 JS `document.documentElement`는 아직 구현되지 않았으므로 이 결정은 CSS cascade의 내부 입력 모델이며, 공개 DOM 경계를 완료했다는 뜻이 아니다. 공개 DOM facade 작업에서는 같은 문서 루트를 HostDocument가 소유하도록 계약을 확정하고 이 합성 계층을 대체해야 한다.

합성 HTML 요소의 기본 computed `font-size`는 우선 초기값으로 cascade한다. 그 computed CSS px를 Stylo `Device`의 root font-size에 설정한 후 mount subtree를 cascade한다. 따라서 `html { font-size: 1.25rem }`의 자체 `rem`은 initial medium(16px)을 기준으로 20px이 되고, descendants의 `rem`은 갱신된 20px root size를 사용한다. Device pixel ratio는 root font-size나 CSS length 변환에 적용하지 않는다.

runtime 합성 문서의 UA baseline은 `<body>`를 `display:block; margin:0`으로 둔다. 이 wrapper는 cascade parent이며 layout box가 아니므로 앱 mount를 viewport 원점에 유지한다. `font-size`, 상속되는 `direction`, custom property는 mount cascade context로 전달한다. 합성 `html/body`의 계산된 `width`·`height`·`flex-basis`, nonzero/auto margin, nonzero padding, `display:block` 이외의 display, 투명 이외의 `background-color`는 HostDocument/Taffy/GPU에서 출력할 box가 없어 `UnsupportedSyntheticDocumentStyle`로 실패시킨다. 이 검사는 합성 box 효과가 조용히 사라지는 입력을 차단한다. document/body의 일반 layout·paint는 C06.4가 지원하지 않는다.

## 목표

현재 지원 layout property에서 em과 rem을 Stylo가 계산한 typed CSS px 값으로 보존해 Taffy에 전달한다. font-size의 상속과 em·rem 계산도 Stylo computed value를 기준으로 하며, 어댑터가 단위 문자열을 다시 계산하지 않는다. font-size 자체는 text shaping이나 glyph 배치를 추가하지 않고 길이 단위 계산의 cascade 입력으로만 쓴다.

rem은 합성 CSS 문서 루트의 computed font-size를 기준으로 한다. 구현자는 HostRoot를 문서 루트로 간주하거나 rem을 고정 16px로 처리해서는 안 된다.

## 지원 범위 초안

- em·rem은 현재 runtime layout profile이 받는 width, height, flex-basis, physical/logical margin·padding, row-gap·column-gap 및 font-size에 적용한다.
- font-size의 computed CSS px 값은 Stylo의 typed ComputedValues에서 별도 추출한다. 문자열 serialization을 다시 수치로 파싱하지 않는다.
- em이 font-size에 쓰이면 부모의 computed font size를 기준으로 하고, 다른 길이 속성에 쓰이면 해당 요소의 computed font size를 기준으로 한다. rem은 CSS 문서 루트 기준을 따른다.
- %, 절대 길이, 자동 크기, 상속, var() 치환은 기존 C06.1–C06.3 및 C05 계약을 보존한다.
- ex, rex, ch, rch, cap, rcap, ic, ric, lh, rlh 등 실제 글꼴·line-height 측정 입력이 필요한 단위는 이 하위 항목에서 계산값처럼 취급하지 않는다. inline style, author stylesheet, custom property와 그 fallback에 나타나면 계산 전에 구체적인 unsupported 오류로 거부한다. 고정 FontMetrics::default()의 대체값이 실제 플랫폼 글꼴 측정과 같다고 가정하지 않는다.
- calc() 등 복합 수식은 C06.5 범위다. C06.4는 해당 문법을 임의 산술로 변환하지 않는다.
- Chromium `getComputedStyle()`의 flex item width/height는 flex layout 후 resolved/used value를 반환할 수 있다. Rust cascade snapshot은 Taffy 계산 전 computed typed value를 보존한다. 이때 Chromium의 사후 값은 최종 rect와, Stylo의 사전 값은 typed layout input 및 같은 rect와 각각 대조하며 두 단계의 숫자를 같은 필드값이라고 요구하지 않는다.
- 텍스트 노드 레이아웃, line breaking, 글꼴 로딩·fallback 선택, glyph shaping, baseline, line box, OS 접근성 글자 크기는 포함하지 않는다.

## 구현 경계와 오류

1. Stylo는 cascade, 상속, font-size, em·rem computed value를 소유한다.
2. Stylo→layout bridge는 CSS width·height·flex-basis·spacing typed 값을 CSS px/percentage/auto로 넘긴다. font-size typed px도 같은 snapshot에 기록한다.
3. Taffy는 변환된 값으로 기존 layout을 계산한다. Taffy에 font-family나 font metric 계산을 추가하지 않는다.
4. metric 단위 검사기는 cssparser token 단위로 inline style 원문, author stylesheet, custom property token 및 중첩 함수/블록을 살핀다. 주석·문자열·식별자 속의 ch 같은 글자는 단위로 오인하지 않는다. 파싱할 수 없는 검증 입력은 fail-closed한다.
5. 스타일 입력이 metric 단위를 포함하면 UnsupportedFontMetricUnit에 해당하는 식별 가능한 오류로 실패한다. 0px 또는 임의 기본값으로 대체하지 않는다.
6. font size 또는 변환값이 NaN·무한대·음수/overflow 경계에 도달하면 Stylo의 CSS invalid-value 처리를 따르며, adapter로 직접 전달된 잘못된 typed 값은 layout을 실패시킨다.
7. 이전 C04/C05 profile 동작을 보존한다. C06.4에 필요한 허용 목록·metric 거부 검사는 실제 runtime API와 전체 입력 경계에 일관되게 적용하며 fixture 전용 성공으로 끝내지 않는다.

## 비교 fixture와 완료 판정

- Chromium fixture는 document element html의 기본/명시 font-size, mount subtree 별도 글꼴 크기, em의 parent/own-element 기준, rem의 document-root 기준, root font-size: rem 특례, 상속, 중첩, zero, percentage font-size, custom-property substitution, 크기·flex basis·gap·margin·padding을 독립 case로 갖는다. Rust/Taffy oracle은 12개 HostDocument 요소의 모든 입력 computed property와 각 CSS px frame을 비교하고, 합성 html의 root font-size 및 body 상속은 별도 assertion으로 확인한다. html/body는 HostDocument frame 개수가 아니므로 browser html/body rect를 Taffy frame으로 간주하지 않는다.
- 각 case는 getComputedStyle()의 font-size·대상 속성 및 각 노드의 getBoundingClientRect()를 저장한다. 계측값은 fixture·Chromium 실행 파일·revision·viewport·DPR·capture tool hash에 묶고 기준 파일을 덮어쓰지 않는다.
- 별도 negative fixture는 각 metric 단위가 direct inline value, stylesheet value, var() value, var() fallback, 중첩 함수와 escaped unit spelling을 통과하지 못하는지 확인한다. 무관한 string/comment의 유사 텍스트는 거부하지 않는 negative control을 둔다.
- Rust typed oracle은 computed font-size와 C06 bridge 값 및 최종 Taffy geometry를 Chrome 고정 기준에 대조한다. 모든 지원 node의 x, y, width, height 최대 절대 오차는 각자 0.5 CSS px 이하여야 하며 평균으로 실패를 숨기지 않는다.
- Android API 37 emulator와 iPhone 17 Pro / iOS 26.2 Simulator에서 동일 V8 fixture를 실제 runtime cascade·Taffy·WGPU 경로로 실행한다. 상태·box 개수·표시와 지원 geometry를 별도 검증한다. 실기기·hardware GPU 성능 주장을 하지 않는다.
- 계획 완료 관문: Chromium precomparison과 아래 20개 서로 다른 계획 실패 경로의 검토 결과가 기록되어야 한다. 계획 검토가 수정으로 재개되면 해결된 항목을 포함해 20개를 다시 대조한다.
- 구현 완료 전: Rust/CSS reference tests, full workspace tests, Clippy, fmt, FFI check, Android/iOS simulator build/run 및 계획과 겹치지 않는 새 구현 실패 경로 20개 검토 완료.

## 구현·검증 결과

- 구현 비교: [고정 Chromium reference](../tests/fixtures/css/references/c06-font-relative-units-v1.json) · [구현 전 기준](../spec/internal/evidence/c06-4-font-relative-units-precomparison-2026-10-10.md)
- 계획 및 구현 검토: [계획 실패 경로 20개](../spec/internal/evidence/c06-4-font-relative-units-plan-review-2026-10-10.md) · [서로 다른 구현 실패 경로 20개와 실행 결과](../spec/internal/evidence/c06-font-relative-units-implementation-review-2026-10-10.md)
- 플랫폼 화면: [Android API 37](../spec/internal/evidence/c06-font-relative-units/android-api37.png) · [iPhone 17 Pro / iOS 26.2 Simulator](../spec/internal/evidence/c06-font-relative-units/ios-26.2.png)
- 출시 전 숫자 계약·crate 버전은 `0.1.0`으로 유지한다. 0040은 명세 ID이며 버전이 아니다. 공개 CSS 지원이나 text/font rendering 완료를 뜻하지 않는다.

## 남은 결정

- 향후 공개 DOM façade가 `document.documentElement`를 소유하는 계약과 이 내부 합성 루트의 대체 시점. C06.4 자체 구현은 합성 root semantics로 진행하며, 공개 DOM API가 구현됐다고 보고하지 않는다.
- metric 단위 지원은 별도 후속 계획으로 둔다. 실제 플랫폼 font metrics를 공급할 때 family/fallback·폰트 크기·orientation·revision 키 및 Android/iOS 간 차이 계약을 선행한다.
