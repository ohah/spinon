# C11.4 orthogonal row sizing을 위한 writing-mode 선행 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [orthogonal text/image row 계획](./c11-4-orthogonal-row-intrinsic-contribution.md) · [#611 direct text/image column 계획](./c11-4-mixed-direct-text-image-intrinsic.md) · [#620 direct text/image row 계획](./c11-4-mixed-direct-text-image-intrinsic-row-axis.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** 초안. 기존 orthogonal row 계획의 feasibility gate에서 멈춘 뒤 확인한 공통 차단 경로를 제한된 선행 작업으로 나눈다. 이 문서는 구현 승인이나 Taffy 변경 승인이 아니다. 계획 실패 관점 20개 검토, 독립 기대값 및 소유 파일을 확정하고 계획 PR을 병합하기 전에는 코드를 수정하지 않는다.

## 목적과 현재 차단 경로

기존 orthogonal row 계획은 horizontal Grid 안의 direct `vertical-rl` text와 natural PNG 네 장면에서 min-content/max-content row `[50]`/`[80]px`를 비교한다. Chrome 156 사전 비교와 CSS Grid 2025 CRD 독립 계산은 일치하지만 Spinon runtime 결과는 아직 없다. [원본 입력·기대값·화면](../spec/internal/evidence/c11-4-orthogonal-row-intrinsic-precomparison-2026-10-25/README.md)

현재 코드의 경계는 다음과 같다.

- `spinon-style`의 C11.4 cascade property 목록은 Stylo `writing-mode` 값을 수집하고, `ComputedElementStyle.properties`에 문자열로 보존한다. `text-orientation`은 해당 C11.4 목록에서 typed 값으로 전달하지 않는다.
- `spinon-layout::LayoutStyle`에는 물리 width/height와 `direction`이 있지만 `writing-mode`가 없다.
- `spinon-text-layout::ParagraphStyle`에는 `direction`과 글꼴·줄바꿈 값이 있지만 `writing-mode`와 `text-orientation`이 없다. 측정 결과도 현재 physical width/height로 layout에 전달된다.
- `spinon-style`과 `spinon-text-layout`은 모두 `spinon-core`에 의존하며 서로 의존하지 않는다. `spinon-style`은 Stylo/DOM cascade 소유자라 텍스트 측정 모델의 타입 owner가 될 수 없고, `spinon-text-layout`은 style crate를 의존성으로 끌어오면 순환·계층 역전을 만든다. `spinon-layout`은 이미 `spinon-text-layout`에 의존하므로 공통 타입을 그곳에 두면 text-layout이 layout에 역의존해야 한다. 따라서 두 소비 crate가 이미 공유하는 가장 낮은 의존성인 `spinon-core`가 typed enum의 owner 후보로 적절하다. 이 결정은 `spinon-core`가 CSS grammar나 Stylo 값을 해석한다는 뜻이 아니다. Stylo conversion은 style crate, logical-to-physical conversion은 layout crate가 소유한다.
- Vendored Taffy는 현재 Grid alignment에서 `horizontal-tb`만 전제로 한다. `LayoutInput::transpose()`만으로 per-item CSS orthogonal flow가 지원되는 것은 아니다.

따라서 이 계획은 cascade에 `vertical-rl` 문자열을 추가하는 데서 끝나지 않는다. CSS 논리 축을 typed style부터 text intrinsic contribution, Grid row sizing 및 item frame까지 보존해야 한다. 정확한 경로를 만들 수 없으면 지원하지 않는 값으로 닫고 이 계획의 구현을 중단한다.

## 제한된 목적과 기대값

선행 작업의 유일한 최종 사용자 입력은 기존 [orthogonal row 계획](./c11-4-orthogonal-row-intrinsic-contribution.md)의 네 exact scene이다.

- Parent Grid: `horizontal-tb`/LTR, width `74px`, columns `[50,24]px`, 한 행 `min-content` 또는 `max-content`.
- Direct text child: `vertical-rl`/LTR, `text-orientation:mixed`, Ahem 10px, `XX XXXXX`. Text의 logical inline intrinsic contribution은 실제 Ahem shaping과 line-break 측정으로 산출한 min-content `50px`, max-content `80px`이다. 이 수치는 독립 oracle/검증 기대값일 뿐 layout 또는 text-measure 입력값이 아니다.
- Direct image child: 기존 natural PNG `24×16px`; parent의 물리 row contribution은 `16px`이다.
- Parent의 물리 row는 vertical text의 inline axis와 평행하다. 각 scene에서 row 크기는 `max(text inline contribution, image height)`여야 하므로 min-content `[50]px`, max-content `[80]px`다. DOM child 순서를 뒤집어도 값은 달라지지 않는다.

이 수치는 Chrome 관측만으로 구현하지 않는다. 원본 계획의 독립 규범 기대, Ahem·PNG hash, root/item geometry와 revision gate를 그대로 사용한다. #611의 column 결과 4 scene과 #620의 horizontal row 결과 4 scene도 변경 전후 regression gate다. 선행 enum 또는 단위 시험이 통과해도 위 네 orthogonal runtime scene이 통과한 것으로 보지 않는다.

## 제안하는 공통 타입 계약

공통 값은 `crates/spinon-core/src/writing.rs`와 `spinon-core`의 crate root에서 내부 Rust 소비자에게만 노출한다. `spinon-core`를 고르는 이유는 위 dependency DAG에 있으며, core enum은 CSS parser나 computed-style owner가 아니다.

- CSS Writing Modes Level 3에서 `writing-mode`는 `horizontal-tb` initial, inherited, computed value는 specified value다. `text-orientation`은 `mixed` initial, inherited, computed value는 specified value다. 이 fixture의 `div`와 `img`는 두 속성의 적용 대상이다. [writing-mode 정의](https://www.w3.org/TR/css-writing-modes-3/#writing-mode) · [text-orientation 정의](https://www.w3.org/TR/css-writing-modes-3/#text-orientation) · [orthogonal flow 축 매핑](https://www.w3.org/TR/css-writing-modes-3/#orthogonal-flows).
- Scene root는 Stylo computed `HorizontalTb`/`Mixed`, text child는 authored `vertical-rl`과 inherited `mixed`의 computed `VerticalRl`/`Mixed`, image는 부모로부터 상속한 computed `HorizontalTb`/`Mixed`여야 한다. Authored declaration 누락은 CSS initial/inheritance가 처리한다. Rust 측에 computed field가 없다는 뜻의 missing은 CSS initial로 대체하지 않고 incomplete snapshot 오류로 닫는다.
- `WritingMode`는 이 fixture에서 `HorizontalTb`와 `VerticalRl`, `TextOrientation`은 `Mixed`만 표현·허용한다. Stylo의 다른 computed keyword, unknown value 또는 지원하지 않는 조합은 profile의 구체적인 unsupported 오류로 닫는다. Invalid authored declaration은 고정 HTML source/hash와 Stylo의 표준 cascade가 처리하며 runtime matcher는 최종 computed value만 판정한다.
- Enum에는 `Default`를 구현하지 않고 `Option::unwrap_or(HorizontalTb/Mixed)` 형태의 대체도 두지 않는다. 기존 horizontal-only profile은 기존 코드를 그대로 사용한다. 새 opt-in profile에서는 root/text/image 모두의 computed 값을 검사한 뒤에만 typed value를 layout에 전달한다.
- 이 타입은 내부 typed transport만 정의한다. HTML/CSS public admission, general writing-mode support, glyph painting, 공개 API 또는 내부 계약 0059 숫자 버전 변경을 뜻하지 않는다. 계약 0059는 `0.1.0`으로 유지한다.

계획 review에서는 dependency DAG를 근거로 `spinon-core` owner 선택을 확인하고, inheritance·computed-value·missing/unsupported/default semantics가 위의 CSS 계약과 일치하는지 검토한다. 이 결정 뒤에 owner를 바꾸려면 코드 작업 전에 이 계획과 소유 파일을 다시 검토한다.

## 작업 분해와 의존성

### 0. 공통 타입 계약

계획 PR 병합 후 통합 owner가 `spinon-core`에 위의 최소 enum을 추가하고 지원 variant·unsupported conversion의 단위 검사를 작성한다. Enum에 implicit default를 두지 않는다. 이는 병렬 작업의 선행 commit이다. 두 구현 agent가 공통 type 파일을 동시에 수정하지 않는다.

### 1A. Stylo typed extraction

공통 타입이 병합된 뒤 `spinon-style` owner가 아래 파일 경계 안에서 C11.4 profile의 computed value를 typed snapshot에 보존한다.

- 소유 후보: `crates/spinon-style/src/stylo_dom/cascade/runtime_grid.rs`, `snapshot.rs`, `device.rs`, `incremental/`의 C11.4 경로 및 `runtime_grid_tests/`의 새 테스트 파일.
- C11.4 track-sizing source admission과 computed-value 수집은 분리한다. Exact fixture의 authored `writing-mode: vertical-rl`을 통과시키기 위해 `runtime_block_flex_grid_track_sizing_author_properties()`에 `"writing-mode"`만 추가한다. `"text-orientation"`은 fixture가 authored declaration 없이 CSS initial/inheritance의 `mixed`를 검증하므로 author allowlist에 추가하지 않는다. 이 admission 변경은 `RuntimeBlockFlexGridTrackSizingV1`에만 적용하고 다른 Grid/Block/Flex profile은 바꾸지 않는다.
- Stylo 0.22의 generated longhand gate에서 `writing-mode`는 `layout.writing-mode.enabled`(기본 false), `text-orientation`은 `layout.unimplemented`를 참조한다. C11.4 track-sizing cascade에서만 `device.rs`의 profile-scoped guard가 `STYLO_PREFERENCE_LOCK` write lock으로 동시 preference 변경을 직렬화하고, 이전 `layout.writing-mode.enabled` 값을 저장한 뒤 true로 켠다. Standard pref로 만들어진 view는 이 guard 안에서 `rebuilt_with_current_stylo_preferences()`로 다시 구성한 뒤 inline author-property scan, cascade 및 typed extraction에 사용한다. Guard는 이 전체 구간을 감싸며 성공·오류 모두 `Drop`에서 이전 값을 복원한다. 다른 profiles는 기존 standard preference 경로를 쓰고 true가 누출되지 않아야 한다.
- C11.4 computed-value 목록 `runtime_block_flex_grid_track_sizing_properties()`는 `LonghandId::WritingMode`를 유지하고 `LonghandId::TextOrientation`도 포함해야 한다. 두 computed longhand를 Stylo 최종값에서 `spinon-core`의 typed `WritingMode`/`TextOrientation`으로 변환해 C11.4 snapshot에 함께 보존한다. 현재 `writing-mode`의 raw computed string 보존만으로 typed snapshot 요구를 충족한 것으로 보지 않는다.
- `writing-mode`와 `text-orientation`의 최종 Stylo computed value를 공통 enum으로 변환한다. Root `HorizontalTb`/`Mixed`, text item `VerticalRl`/`Mixed`, inherited image `HorizontalTb`/`Mixed`만 이 fixture profile에서 허용한다. `text-orientation`을 text child에서 따로 author하지 않아도 computed `Mixed`가 부모로부터 상속되어야 한다.
- CSS source가 유효하지만 property가 선언되지 않은 경우에는 Stylo computed value가 상속/initial semantics에 따라 생성된다. C11.4 snapshot에서 property 자체 또는 typed field가 빠지면 incomplete-style 오류다. Unknown/unsupported computed keyword는 unsupported-style 오류다. 다른 Grid profile의 값 수집, 성공·오류 동작 및 snapshot revision 의미는 바꾸지 않는다.
- Stylo 0.22 `longhands.toml`에서 `text-orientation`은 `inherited_box`이고 `servo_pref=layout.unimplemented`다. Generated `enabled_for_all_content()` gate는 이 pref를 content author declaration parse/admission에 적용하며, computed cascade에는 `TextOrientation` 상속 경로가 있다. Exact fixture는 이 속성을 authored하지 않으므로 broad `layout.unimplemented`를 켜지 않고 pref=false인 상태에서 computed `mixed`의 initial/inherited 경로를 사용한다. 그 상태로 computed snapshot에 도달하지 않는다는 증거가 나오면 여기서 1A를 중지하고, `layout.unimplemented`를 켜는 변경은 같은 serialized save/restore 계약으로 별도 계획·검토를 받은 뒤에만 고려한다.
- 1A cascade/preference regression gate는 exact fixture의 `writing-mode: vertical-rl`이 C11.4 track-sizing author allowlist에서만 허용되고, authored `text-orientation`은 계속 거부되며, declaration이 없는 `text-orientation`은 `layout.unimplemented=false` 상태에서 Stylo computed `mixed`로 snapshot에 도달하는지 확인한다. C11.4 guard가 성공·cascade 오류 뒤 이전 `layout.writing-mode.enabled` 값을 복구하고, 이어 실행한 non-C11.4 profile에 true가 누출되지 않는지도 확인한다. 다른 profile의 author allowlist, preference 값, computed snapshot 결과는 기존 판정을 유지한다.
- 산출물은 source node identity와 같은 `computed_style_revision`에 묶인 typed value다. 이 lane은 LayoutStyle, ParagraphStyle, Taffy 또는 runtime fixture 파일을 수정하지 않는다.

### 1B. text-layout logical intrinsic input

공통 타입이 병합된 뒤 `spinon-text-layout` owner가 별도 파일 경계에서 paragraph model과 intrinsic measurement를 준비한다.

- 소유 후보: `crates/spinon-text-layout/src/model.rs`, 기존 paragraph measurement module과 그 crate의 전용 tests, 그리고 `ParagraphStyle`의 기존 horizontal literal migration을 위한 `crates/spinon-layout`, `crates/spinon-style-to-layout`, `crates/spinon-ffi`, `crates/spinon-runtime`의 제한된 callsite.
- `ParagraphStyle`에 공통 `WritingMode`와 `TextOrientation` 입력을 필수 필드로 추가한다. `ParagraphStyle::default()`에는 기존 horizontal 호출부와 같은 `HorizontalTb`/`Mixed`를 명시하되, 그 기본값은 incomplete C11.4 computed-style snapshot을 보충하는 데 사용하지 않는다. C11.4 생성 경로는 Stylo에서 받은 두 typed 값을 반드시 전달하며 누락을 오류로 닫는다. 기존 `ParagraphStyle` struct literal 호출부는 1B의 compile-time migration에서 모두 갱신한다. 기존 horizontal profile의 paragraph 결과를 보존하고, 미지원 조합은 명시적으로 거부한다.
- 현재 checkout에서 `crates/spinon-runtime`의 실제 `ParagraphStyle` struct literal은 22개이며 21개 Rust 파일에 있다. 이 중 2개는 production root-HTML 생성자이고 20개는 `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/**` 아래의 기존 C11.4 horizontal fixture builder다. 넓은 `ParagraphStyle {` 검색은 함수 반환 타입까지 포함해 35건을 내므로 compile migration 수는 반환 타입 13건을 제외한 실제 struct literal 22개로 판정한다(기존 fixture literal 20개와 production `Ok(ParagraphStyle {` 2개). production 경계는 `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/root_html_text_paint/text.rs::paragraph_style`와 `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/root_html_default_render/style.rs::paragraph_style`다.
- 두 production 생성자는 현재 horizontal 동작만 제공한다. `root_html_default_render/style.rs`는 `text_flow_style()`의 기존 `writing-mode: horizontal-tb` gate를 유지하고 `HorizontalTb`/`Mixed`를 명시한다. `root_html_text_paint/text.rs`도 `root_html_text_paint/tree.rs`의 text-owner matcher가 `writing-mode: horizontal-tb`를 확인하는 경계를 유지한 뒤 같은 두 값을 명시한다. 이 matched fixed profile에서 `Mixed`는 현재 horizontal paragraph input을 보존하는 값이며 vertical orientation을 인정하지 않는다. 두 경로 모두 C11.4의 누락된 computed snapshot을 `Default`로 보충하지 않으며 이 migration에서 vertical/root-HTML writing-mode admission을 추가하지 않는다.
- `spinon-text-layout`은 style에 포함된 writing mode/orientation을 써서 실제 Ahem font를 shape하고 실제 line-break opportunity로 `LogicalIntrinsicSize { inline_px, block_px }`를 반환한다. 요청은 `MinContent` 또는 `MaxContent` sizing mode를 명시하며 결과는 finite/nonnegative CSS px여야 한다. Ahem/문자열/hash/shape readiness가 맞지 않으면 측정 오류로 닫는다.
- Ahem `XX XXXXX`의 min-content `inline_px=50`과 max-content `inline_px=80`은 독립 기대 assertion이다. Measure API, runtime scene config, FFI report 또는 layout input에 이 값을 상수로 넣어서는 안 된다. 실행 근거는 측정 결과에 연결된 실제 Ahem glyph advance와 break decision을 남겨 예상값 재주입을 구별한다.
- 이 payload는 logical axes만 표현하며 physical width/height 변환은 소유하지 않는다. Text layout은 회전 glyph paint나 arbitrary vertical shaping behavior를 추가하지 않는다.
- 1B owner는 `spinon-text-layout` model/measurement/tests와 네 consumer crate의 기존 horizontal `ParagraphStyle` literal field migration만 수정한다. `spinon-layout`, `spinon-style-to-layout`, `spinon-ffi`는 기존 horizontal literal에 명시적 `HorizontalTb`/`Mixed`를 공급하는 변경으로 제한한다. `spinon-runtime`에서는 위 22개 실제 literal만 compile migration 대상으로 삼고, 두 production root-HTML 생성자의 horizontal 경계와 20개 기존 C11.4 horizontal fixture builder를 보존한다. 이 범위는 새 orthogonal runtime scene, fixture dispatcher, FFI scene, Android/iOS adapter를 포함하지 않는다. 이들은 feasibility gate 뒤 Step 3에서 순차로 수정한다. 그 외 Style snapshot, layout projection, Grid intrinsic transport, Taffy는 수정하지 않는다. Glyph orientation/raster painting, arbitrary Unicode, line breaking 외 추가 CSS text 기능을 구현하지 않는다.

### 1A/1B 병렬 실행 조건

공통 타입 단계가 병합되고 owner가 각 전용 파일 목록을 확인한 뒤에만 1A와 1B를 병렬로 수행한다. 두 lane은 서로의 crate를 수정하거나 공통 enum을 재정의하지 않는다. 충돌이 보이면 해당 파일 소유 작업을 중지하고 재배정한다. 별도 checkout은 실제 경로 충돌을 발견한 경우에만 사용한다.

### 2. layout projection 및 Taffy feasibility gate — 순차

1A/1B의 typed 결과가 확정된 뒤 통합 owner가 `spinon-style-to-layout`/`spinon-layout` 경로를 연결한다. `spinon-style-to-layout`은 computed root/item writing mode를 확인하고, `spinon-layout/src/intrinsic_measurement.rs`에 `LayoutGridItemIntrinsicSource::OrthogonalText { min_content, max_content, child_writing_mode, parent_writing_mode, text_orientation, font_registry_revision }` 변형을 추가한다. 두 size field는 `LogicalIntrinsicSize`, writing-mode/orientation field는 `spinon-core`의 typed enum이다. 이 source는 기존 `LayoutGridItemIntrinsicInput`과 `LayoutGridItemIntrinsicContributionInput.source`에 `LayoutInputRevision`과 함께 보존한다. `LayoutStyle`에는 새 전역 필드를 추가하지 않는다. 변경은 `RuntimeBlockFlexGridTrackSizingV1`의 opt-in fixture 경계로 제한한다. 이 경계는 #611의 direct text/image column 네 scene과 #620의 direct text/image horizontal row 네 scene을 변경 전 기준으로 실행하고 동일 assertion으로 변경 후 회귀 제어한다.

text-layout은 min-content와 max-content 각각에 `{ inline_px, block_px }` `LogicalIntrinsicSize`를 반환한다. 정확한 scene의 parent는 `horizontal-tb`이므로 parent inline axis는 physical X, parent block/row axis는 physical Y다. Child `vertical-rl`에서는 child inline axis가 physical Y, child block axis가 physical X다. 단일 conversion owner는 `spinon-layout::LayoutGridItemIntrinsicInput::contribution_input()`으로 정한다. 이 함수만 `OrthogonalText`의 writing mode pair를 검증하고 min/max logical size를 기존 물리축 입력에 매핑한다: `width ← block_px`, `height ← inline_px`, 따라서 parent `row ← child inline`, `column ← child block`이다. Taffy와 FFI/host는 변환된 width/height만 소비하며 추가 축 교환을 하지 않는다. Natural image는 physical `24×16px`이므로 parent row contribution은 16px이다. Source writing mode, source axis, parent target axis, min/max logical·physical size, `LayoutInputRevision`, `FontRegistryRevision`은 같은 trace entry에 남긴다.

이 단계의 첫 변경은 output trace와 입력 변환을 계측하는 feasibility gate다. CSS logical contribution을 실제 Taffy sizing callsite에 전달할 수 있는지, contribution revision과 used row track이 같은 layout revision에 묶이는지 확인한다. `LayoutInput::transpose()`나 값 복사만으로 지원을 주장하지 않는다. 수평 fallback, 예상값 주입, fixture-only test에서만 다른 값 계산, 공용 validator 완화는 허용하지 않는다.

다음 조건이면 이 계획을 여기서 중지한다.

1. Taffy의 현재 per-item measure/track-sizing 경로로 parent row와 child inline axis를 일관되게 연결할 수 없다.
2. `[50]`/`[80]px` used rows와 `50×50`/`50×80px` text frames를 얻기 위해 vendored Taffy의 generic Grid algorithm, global axis semantics, 다른 formatting model 또는 제품 API를 변경해야 한다.
3. `horizontal-tb` 기존 scene, intrinsic measurement provenance 또는 다른 layout profile을 보존할 수 없다.

중지 시 실제 callsite, 입력·출력 trace, 예상과 차이를 기록하고 implementation/runtime PR을 만들지 않는다. 이보다 넓은 Taffy 또는 writing-mode 지원이 필요하면 별도 상위 설계와 20관점 계획 검토를 먼저 추가한다.

### 3. orthogonal fixture/runtime integration — gate 통과 뒤 순차

Feasibility gate가 정확한 row/frame 계산을 입증한 뒤에만 기존 orthogonal row 계획의 네 fixture scene을 통합한다. FFI scene dispatcher와 Android/iOS host adapter는 각각 한 owner가 순차로 수정한다. 1A/1B와 이 shared-file integration은 동시에 편집하지 않는다. 원본 계획의 scene ID, full style matcher, revision chain, CSS Grid/Chrome 기대값 및 alignment 조건을 그대로 유지한다.

## 실패 경계와 수용 기준

- 누락·unknown writing mode, `vertical-lr`, sideways/upright orientation, 혼합/RTL direction, 다른 font/text, incomplete cascade, source/style/layout revision 불일치, image decode 전 값은 구체 오류로 닫는다. 이를 `HorizontalTb`로 대체하거나 일부 frame을 내보내지 않는다.
- 1A는 C11.4 opt-in computed snapshot에서만 typed value를 내며 다른 profile의 기존 성공·오류 경계를 보존한다.
- 1B는 기존 horizontal paragraph 결과를 보존하고 vertical fixture 입력의 logical inline metric을 독립 단위 검사로 구별한다. 1B compile/regression gate는 네 consumer crate의 기존 horizontal literal 전체를 포함하며, `spinon-runtime`의 22개 실제 literal이 모두 새 필드를 명시하는지 확인한다. 특히 두 production root-HTML constructor의 수평 경계·paragraph output과 20개 기존 C11.4 horizontal fixture의 결과를 변경 전 기대값과 대조한다. 새 orthogonal runtime scene이나 adapter는 이 gate에 끼워 넣지 않고 Step 3에서 별도로 검증한다. 이 결과는 Grid runtime 검증과 분리해 보고한다.
- Feasibility gate는 실제 Ahem shaping/line measurement에서 나온 logical `inline_px=50/80`, 실제 `block_px`, `Columns [50,24]` 뒤 `Rows [50]`/`[80]`, image row contribution `16`, root `74×50`/`74×80`, text physical frame `50×50`/`50×80`, forward/reversed 순서 동일, 완전한 revision/trace provenance를 보여야 한다. 기대값을 측정 경로에 주입한 결과는 실패다.
- Gate 통과 후 #611 column regression 네 장면, #620 horizontal row regression 네 장면, 새 orthogonal 네 장면을 합친 12-scene matrix를 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 실행한다. 세 그룹은 4/4씩 별도 보고하고 합산 성공 수로 각 그룹의 누락을 감추지 않는다. AVD renderer와 iOS drawable/compositor 경계를 명시한다. 이 제한 실행은 물리 기기·hardware GPU·iOS compositor 이후 pixels 또는 일반 CSS admission을 주장하지 않는다.
- 기본/product artifact에는 fixture ABI·payload가 없어야 하고 opt-in fixture-test positive control에는 있어야 한다. archive 검사를 APK/IPA 전체 제외 근거로 확대하지 않는다.
- 실제 구현 변경은 계획 review와 별도로 구현/runtime 실패 관점 20개, 실제 PR diff 실패 관점 30개를 검토한다. PR은 project rule에 따라 Rebase and merge한다.

이 선행 계획 또는 네-scene slice는 C11.4 완료를 뜻하지 않는다. 공식 C11.4 checkbox는 미완료로 두고 완료 수 `72/213`과 내부 계약 0059 `0.1.0`을 유지한다.

## 파일 소유 요약

| Lane | 전용 소유 파일 | 의존성 | 동시 수행 |
| --- | --- | --- | --- |
| 공통 타입 | `spinon-core/src/writing.rs`, crate root, 해당 tests | 계획 PR 병합 | 단독 선행 |
| 1A Stylo typed extraction | `spinon-style` C11.4 cascade/snapshot/tests | 공통 타입·CSS cascade semantics | 1B와 가능 |
| 1B Ahem logical measurement | `spinon-text-layout` model/measurement/tests 및 네 consumer crate의 기존 horizontal `ParagraphStyle` literal migration (`spinon-layout`, `spinon-style-to-layout`, `spinon-ffi`, `spinon-runtime`; runtime은 현재 실제 literal 22개/21개 파일, production 2·기존 C11.4 fixture 20) | 공통 타입·actual Ahem shaping | 1A와 가능 |
| 2 layout projection/Taffy gate | `spinon-style-to-layout`, `spinon-layout`, 필요한 Taffy test adapter | 1A·1B 결과 | 순차 |
| 3 fixture integration | C11.4 runtime/FFI fixture, Android/iOS demo adapter, evidence | gate 통과 | 순차, 파일별 owner 한 명 |
