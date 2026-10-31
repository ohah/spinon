# C11.4 anonymous Grid text computed `line-height` 제한 계획

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [CSS 적합성 계약](../spec/0001-conformance.md)

**상태:** Chrome 156 관찰, 독립 기대값과 구현 전 계획 검토 20개를 기록했고 제한 fixture 구현 및 최신 Android/iOS Simulator 16-scene 실행을 마쳤다. 별도 구현/runtime 20관점과 PR diff 30관점 검토도 마쳤으며 구현 PR [#905](https://github.com/ohah/spinon/pull/905)은 2026-10-31T05:46:15Z Rebase and merge로 병합됐고 merge SHA는 `454036612372241fa7b162d6f87c78674ab7e559`다. 필수 checks와 보고된 check은 없었다. C11/C11.4 checkbox와 공식 완료 `72/213`은 그대로 둔다. [사전 비교](../spec/internal/evidence/c11-4-anonymous-grid-text-computed-line-height-precomparison-2026-10-31/README.md) · [실행 근거](../spec/internal/evidence/c11-4-anonymous-grid-text-computed-line-height-runtime-2026-10-31/README.md) · [구현/runtime 검토 20개](../spec/internal/evidence/c11-4-anonymous-grid-text-computed-line-height-runtime-2026-10-31/implementation-runtime-review-20.md) · [PR diff 검토 30개](../spec/internal/evidence/c11-4-anonymous-grid-text-computed-line-height-runtime-2026-10-31/pr-diff-review-30.md) · [병합 후 상태 검토 30개](../spec/internal/evidence/c11-4-anonymous-grid-text-computed-line-height-runtime-2026-10-31/post-merge-status-review-30.md) · [계획 검토 20개](https://github.com/ohah/spinon/blob/main/plan/c11-4-anonymous-grid-text-computed-line-height-plan-review-20.md)

## 목적과 경계

현재 `RuntimeBlockFlexGridTrackSizingV1` snapshot에는 Stylo가 계산한 root `line-height` 문자열이 있지만, anonymous Grid text validator가 `10px`만 허용하고 `anonymous_paragraph_style`이 `10px`을 직접 고정한다. 그래서 computed style과 Parley text layout 입력이 분리되어 있다. CSS Inline Layout Level 3 §5.1은 `normal`, 0 이상 number, 0 이상 length-percentage를 정의하며, 숫자 값은 computed font-size에 곱하고 percentage는 font-size에 대한 길이로 계산한다. `normal`은 font metrics에 따른다.

이번 slice는 기존 `c11-4-grid-track-sizing-fixture-test` opt-in gate의 anonymous direct-text Grid 입력만 다룬다. 고정 조건은 HTML `div` Grid root, direct text, Ahem 10px/weight 400, `horizontal-tb`, LTR, 기존 supported white-space profile, zero margin/padding/border/gap, 기존 placement·track·order 계약이다. root의 computed `line-height`를 같은 HostDocument/style revision의 `ParagraphStyle`로 전달한다.

일반 CSS author admission, root HTML 전체 parsing, 공개 API/ABI, 임의 CSS parser, 다른 font/writing mode/inline span, number와 length의 자식 상속 차이, WPT 적합성 또는 전체 C11.4/C11 완료를 포함하지 않는다. 앱의 default/product artifact에 fixture code·payload가 들어가지 않아야 한다.

## 사전 비교와 판정 기준

[Chrome 156 사전 비교](../spec/internal/evidence/c11-4-anonymous-grid-text-computed-line-height-precomparison-2026-10-31/README.md)는 해시가 고정된 Ahem 10px과 direct-text Grid row 하나를 사용한다. CSS Grid 2025 CRD §11.5의 track sizing 절차와 CSS Inline §5.1 규칙으로 Chrome 관찰과 독립된 산식을 만들었다.

- Ahem `font-size:10px`; 두 개 강제 line break; 한 개 `auto` row: 기대 track은 두 line box의 used line-height 합이다.
- unitless `n`: 각 line `n × 10px`; `5px`, `12.5px` 같은 absolute length는 지정 길이다.
- `125%`, `1.25em`, `calc(10px + 2.5px)`는 Stylo computed style에서 확인한 absolute px 값만 전달한다. 이 fixture는 CSS author string을 자체 계산하지 않는다.
- `normal`은 browser/font-metric 의존 값이다. Chrome Ahem 관찰은 한 줄 10px, 두 줄 20px이지만 이를 모든 플랫폼의 규범 상수로 일반화하지 않는다. Android/iOS 결과는 각각 Ahem metrics와 simulator runtime에서 별도 대조한다.
- row track, root border box, text Range fragment와 glyph pixel은 서로 다른 관찰값이다. Track/frame oracle는 track trace 및 node geometry로 비교하고 Range rect나 black glyph screenshot을 track 증거로 쓰지 않는다.

작업의 Chrome 기준 matrix는 12개 line-height source value의 두 줄 강제 break, 한 줄 `normal`·`0.5`, 20px 폭 soft-wrap의 `1.5`·`0` control이다. 기대값은 [expected-values.json](../spec/internal/evidence/c11-4-anonymous-grid-text-computed-line-height-precomparison-2026-10-31/expected-values.json)과 Chrome raw observation에 고정되어 있다.

## 계획된 변경

1. C11.4 anonymous text adapter에서 `ComputedElementStyle.properties["line-height"]`를 정확한 Grid root와 현재 style snapshot에서 가져온다. 장면 ID가 숫자나 길이를 대신 선택하지 않게 한다.
2. Stylo `computed_value_to_string` 출력 계약을 style projection test로 먼저 고정한다. 허용 직렬화는 `normal`, finite nonnegative unitless number, finite nonnegative `px` length다. unitless number는 고정 computed font-size 10px와 곱하고, Stylo가 이미 resolve한 px 값은 그대로 사용한다. `%`·`em`·`calc()`를 source HTML에서 다시 parse하지 않는다. 예상하지 못한 suffix, negative, non-finite, parse overflow나 표현 가능한 범위 밖의 결과는 layout 시작 전에 fail closed한다.
3. `anonymous_paragraph_style`의 scalar `10.0`을 제거하고 계산된 `LineHeight` 값을 받도록 한다. `normal`은 `line_height_is_normal:true`로 Parley metrics path에 전달하고, 다른 값은 절대 CSS px로 전달한다. white-space, font, direction 등 기존 fixture 조건은 유지한다.
4. 이 direct-text 경로가 사용하는 `ParagraphStyle::validate`에서 CSS가 허용하는 finite zero line-height를 받도록 조정한다. 음수, NaN, ±Infinity와 invalid font size는 계속 거부한다. Parley line box, line advance와 baseline이 0px에서 유한하고 일관되며 partial output이 남지 않는 test를 추가한다. Span 전용 `InlineRunStyle` 및 C15.2b 계약은 이 작업에서 바꾸지 않는다.
5. Rust test는 12개 value matrix에서 computed-style parse, `normal` flag, number×font-size, lengths, `normal`, zero/invalid boundary와 intrinsic min/max block size를 확인한다. 별도 scene은 한 줄, 강제 두 줄, soft wrap을 비교하고, 10px baseline은 기존 anonymous Grid 회귀 전체에서 그대로 나와야 한다. Grid layout test는 computed text input, source/style revision, Rows contribution/final track, root/item frame과 overflow를 검사한다.
6. Runtime fixture는 해당 scene IDs와 정확한 direct-text shape·root/style profile·computed property serialization을 검증한다. computed `line-height` mismatch, unsupported string, 다른 order/topology, 숨김·stale revision과 잘못된 feature 경로는 layout frame/trace 발행 전에 거부한다. 테스트는 default profile, 기존 10px scenes, 새로운 16 reference scenes를 함께 실행한다.
7. 기존 opt-in Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 앱 경로에서 신규/기존 matrix의 computed style, text intrinsic, Grid contribution/track, node frame, revision, draw/present를 비교한다. Android는 현재 C11 fixture renderer를 사용하고, iOS Metal sample은 제출 전 drawable 한계로 기록한다. 실기기와 hardware GPU 실행은 이 계획의 성공 조건이 아니다.
8. 기본 product와 fixture-test static archive를 각각 빌드해 새 scene marker·fixture FFI symbol/payload의 부재와 opt-in positive control의 존재를 검사한다. 앱 bundle 전체의 fixture exclusion으로 확대 해석하지 않는다. 관련 Rust focused tests, formatting, Clippy, Android/iOS builds 및 변경 경계 검사를 수행한다.
9. 구현 뒤에는 별도 실패 관점 20개로 코드/runtime을 검토하고 PR source diff는 저장소 규칙에 맞춰 독립 실패 관점 30개로 검토한다. 계획 PR에는 계획 검토 20개와 별도 PR diff 검토 30개를 포함한다. 결함을 찾으면 고친 뒤 영향받은 관점을 재검토한다.

계약 문서에서는 [내부 Grid 계약 0059](../spec/internal/0059-c11-grid-layout.md)에 이 제한 opt-in scene 범위와 provenance를 기록하고, [내부 text 계약 0070](../spec/internal/0070-c15-1-inline-text-layout.md)은 C15 generic text가 계속 fixture-scalar인 상태와 C11.4 전용 예외를 구분해 쓴다. 내부 숫자 버전 `0.1.0`을 유지한다. 공식 `spec/STATUS.md` 체크박스는 바꾸지 않는다.

## 실패 기준과 남은 한계

- 입력 computed `line-height`가 Parley에 전달한 값과 다르거나 scene-selected constant를 소비하면 실패다.
- 16 Chrome reference cases 중 row track, intrinsic contribution, frame 또는 same-revision trace가 독립 기대값과 다르면 실패다. `normal`은 fixed metric input과 Chrome 관찰을 별도로 보고한다.
- 0 line-height에서 line count가 보존되지 않거나 glyph bounds와 line box 크기를 혼동하거나 finite geometry를 잃으면 실패다.
- 기존 computed `10px` matrix가 바뀌거나 unsupported/negative/stale input에서 partial frame이 발행되면 실패다.
- 신규 fixture marker/ABI가 default artifact에 있거나 opt-in positive control에서 사라지면 실패다.
- 이 slice는 공개 CSS 지원, 일반 HTML cascade, WPT pass, glyph paint/pixels, 실기기/hardware GPU, iOS compositor 이후 화면, 전체 C11.4 또는 C11 완료를 뜻하지 않는다.

## 구현 결과

기존 C11.4 fixture-test 경계 안에서 computed `line-height`를 anonymous direct-text `ParagraphStyle`에 연결했다. `normal`, finite non-negative number와 `px` 값만 runtime adapter가 받으며, Stylo가 percentage·`em`·`calc()`를 computed `px`로 직렬화한 값은 그 결과를 사용한다. 직접 text paragraph의 finite zero line-height는 허용하고 span 전용 `InlineRunStyle` 검증은 유지했다. 미지원·음수·비유한 입력은 frame 생성 전에 거부한다.

고정 16-scene 기대값은 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 16/16 통과했다. 실행기는 computed text layout, 줄 수, intrinsic min/max block contribution, Rows auto track, item/root geometry 및 revision을 검증하며 설치 APK·Simulator executable hash를 빌드 산출물과 대조한다. 기본 FFI product archive에는 16개 scene marker가 모두 없고 `c04-runtime-gpu-fixture-test` positive control에는 모두 있다. 원본 로그와 범위 한계는 [실행 근거](../spec/internal/evidence/c11-4-anonymous-grid-text-computed-line-height-runtime-2026-10-31/README.md)에 기록한다.

iOS Simulator 캡처는 fixture app route와 `layout=ready` 상태를 보여주며 text glyph paint는 꺼져 있다. Android AVD 대표 캡처는 화면이 검게 나와 사용하지 않는다. 이 작업은 glyph pixel, Android PixelCopy, iOS compositor 이후 화면, 실기기·hardware GPU, WPT 또는 일반 authored CSS admission을 검증하지 않았다. 전체 C11.4/C11은 계속 미완료다.

## 계획 적대 검토

[20개 독립 실패 관점](https://github.com/ohah/spinon/blob/main/plan/c11-4-anonymous-grid-text-computed-line-height-plan-review-20.md)은 구현 전 이 계획에만 적용한다. 기존 white-space 및 tab-size 계획 검토는 재사용하지 않는다.
