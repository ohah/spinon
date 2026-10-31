# C11.4 anonymous Grid text computed `line-height` 구현/runtime 실패 관점 20개

계획 검토와 분리해 최종 Rust adapter, `ParagraphStyle`, 고정 FFI scene, Android/iOS harness 및 최신 Simulator 실행을 대조했다. 검토 중 scene별 computed serialization을 확인하지 않는 결함을 발견해 수정한 뒤 관련 검사와 두 플랫폼의 전체 행렬을 다시 실행했다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | 0px 장면에 10px 등 다른 유효 CSS 값이 들어와도 layout이 발행된다 | **초기 실패 후 수정.** 첫 검토에서 parser가 유효한 값이면 scene과 다른 computed serialization도 받는 결함을 찾았다. `expected_line_height_serialization`이 16개 신규 scene의 값을 고정하고 기존 scene은 `10px`를 요구한다. 변경값은 runtime integration negative test에서 layout 발행 전에 거부된다. |
| 2 | 기존 anonymous-text 55 scene이 새 parser의 기본값에 기대어 스타일을 바꾼다 | **통과.** 신규 16 scene 외에는 기존 serialization `10px`를 검사하고 기존 C11.4 `c114_` runtime 회귀 23/23을 실행했다. |
| 3 | 음수·비유한·잘못된 단위·공백이 유효한 line-height로 변환된다 | **통과.** parser가 음수, NaN/Infinity, overflow, 대문자 단위와 주변 공백을 닫으며 parser 표적 테스트가 이를 검사한다. |
| 4 | unitless 숫자를 px 길이와 같은 값으로 사용한다 | **통과.** computed number는 고정 10px font size에 곱하고 px 값은 그대로 쓴다. `0.5`, `1`, `1.5`, `2`와 5px/12.5px 값의 기대 사용 높이를 비교한다. |
| 5 | percentage, `em`, `calc()` author token을 runtime이 직접 계산해 cascade 결과를 우회한다 | **통과.** adapter는 author 문법을 해석하지 않고 `ComputedElementStyle.properties["line-height"]`를 읽는다. 125%, 1.25em, calc scene이 각각 Stylo의 `12.5px` serialization과 실제 style snapshot revision을 확인한다. |
| 6 | `normal`을 고정된 수치 길이로 표시해 normal semantics를 잃는다 | **통과.** `normal`은 `line_height_is_normal`로 전달하고 Ahem 기준 10px line metric을 독립 scene 기대값으로 검사한다. 일반 font의 normal metric으로 범위를 넓히지 않는다. |
| 7 | 0px line-height에서 line 수·baseline·advance가 비유한 값이 되거나 paragraph 검증이 거부한다 | **통과.** `ParagraphStyle`만 유한한 0을 허용하고 Ahem paragraph test가 두 줄, 0 높이, 유한 line metrics를 확인한다. `InlineRunStyle`의 0 거부는 별도 회귀 assertion으로 유지한다. |
| 8 | 다중 강제 줄바꿈을 하나의 line으로 합치거나 높이를 잘못 곱한다 | **통과.** 2-line scene의 줄 수, 각 line-height와 0·5·10·12.5·15·20px track 높이를 Rust integration 및 양 플랫폼 matrix에서 비교한다. |
| 9 | single-line scene에서 line-height를 두 번 곱한다 | **통과.** `normal`과 0.5 single-line control은 각각 10px와 5px row로 Rust 및 두 Simulator에서 확인했다. |
| 10 | soft wrap에서 min-content와 max-content block contribution을 같은 값으로 취급한다 | **통과.** 1.5 soft-wrap은 min-content block 30px, max-content block 15px이며 final row 30px임을 trace와 integration test에서 확인한다. 0 soft-wrap은 두 contribution 모두 0px다. |
| 11 | `preserve-breaks`와 `collapse` scene의 줄 분리가 runtime whitespace mode와 어긋난다 | **통과.** 명시 개행 scene은 `PreLine`, soft-wrap scene은 `Collapse`로 고정하고 normalized text와 줄 수를 비교한다. |
| 12 | font 또는 font-size가 달라져 unitless 계산과 고정 기대값이 무효가 된다 | **통과.** 허용 profile은 Ahem 10px이며 root computed style 검증에서 family/font-size를 고정한다. 다른 글꼴·크기의 지원은 선언하지 않는다. |
| 13 | 임의 Grid topology나 알 수 없는 scene이 line-height adapter를 우회한다 | **통과.** scene allowlist와 direct text child topology를 확인하며 미등록 ID와 누락 text는 fail-closed 처리한다. 신규 scene 목록의 중복 및 text shape도 단위 검사한다. |
| 14 | text node의 소유 Grid, 순서, 문서 revision이 intrinsic source와 달라진다 | **통과.** integration test가 anonymous text node, Grid owner, source revision 일치와 text source 종류를 확인한다. |
| 15 | 계산한 text height는 맞아도 Grid의 최종 Rows auto track 또는 max-content 기여가 다르다 | **통과.** 16 case 각각에서 text layout 높이, row pass 마지막 stage, gutter 제외 track과 max-content contribution을 대조한다. |
| 16 | Android harness의 장면 기대 frame·box count·intrinsic 값이 runtime 결과를 검증하지 않는다 | **통과.** 고정 16 ID와 root frame, zero-height box count, text intrinsic source 및 네 min/max 축 값을 확인했다. 최신 Android API 37 ARM64 AVD 실행은 16/16이다. |
| 17 | iOS harness가 Android와 다른 기대값 또는 장면 목록을 사용한다 | **통과.** iOS도 같은 16 scene, geometry, revision과 intrinsic source를 검사하며 허용 오차는 1/64 CSS px다. 최신 iPhone 17 Pro / iOS 26.2 Simulator 실행은 16/16이다. |
| 18 | 일부 trace chunk 누락, 다른 revision 또는 이전 설치 앱으로 PASS를 만든다 | **통과.** capture runner가 설치 hash를 build hash와 대조하고 summary·trace chunk 완전성·revision을 검사한다. Android scene은 5 chunk, iOS는 10 chunk 전체를 기록했다. |
| 19 | 고정 fixture ID나 payload가 기본 제품 static archive에 유입된다 | **통과.** 최신 release archive에서 기본 `c04-runtime-gpu`는 16 marker 모두 부재, `c04-runtime-gpu-fixture-test` positive control은 16/16 존재한다. 이 검사는 host static archive 범위다. |
| 20 | fixture 결과를 공개 CSS·glyph paint·실기기 또는 전체 C11 완료로 과장한다 | **통과.** 증거는 Android AVD와 iOS Simulator의 layout/trace에 한정한다. Android 검은 screenshot은 사용하지 않고 iOS 화면은 route/status만 증명한다. 공식 C11/C11.4 checkbox와 `72/213`은 바꾸지 않았다. |

**판정:** 검토 중 발견한 scene/computed-value 불일치를 고쳤고 변경 후 runtime line-height 5/5, 기존 C11.4 runtime 23/23, FFI matrix 1/1, ParagraphStyle 4/4, Android AVD 16/16, iOS Simulator 16/16을 통과했다. Workspace Clippy와 fixture module-boundary 검사도 통과했다. 이는 제한 fixture 구현 후보이며 C11.4 또는 C11 전체 완료 판정은 아니다.
