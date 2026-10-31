# 구현/runtime 적대 검토 20개

대상은 `ParagraphStyle.letter_spacing` 전달, Parley line advance 및 intrinsic 계산, C11.4 anonymous Grid fixture adapter, FFI 장면, Android/iOS Simulator validator다. 구현 전 계획 검토와 분리해 실제 소스, Rust test, 두 simulator matrix, static archive 경계를 대조했다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | 새 style 필드의 기본값이 기존 text layout 폭을 바꾼다 | **통과.** `ParagraphStyle::default()`와 기존 initializer가 모두 0 CSS px를 설정한다. text-layout·runtime 회귀가 통과했다. |
| 2 | `NaN`/무한대가 Parley shaping 전에 들어온다 | **통과.** `ParagraphStyle::validate`가 non-finite 입력을 `letter-spacing` 오류로 거부하고 unit test가 `NaN`을 검사한다. |
| 3 | profile가 고정하지 않은 값이 scene ID만 바꿔 허용된다 | **통과.** runtime 값은 정확한 16-scene ID와 computed serialization에 동시에 묶여 있다. |
| 4 | `normal`이 0 CSS px와 다른 intrinsic advance를 낸다 | **통과.** validator가 `normal`을 0으로 변환하고 Ahem unit test의 normal case를 min/max와 line advance 모두 확인한다. |
| 5 | authored `0px`의 Stylo `normal` serialization을 fixture가 거부한다 | **통과.** 고정 scene source는 authored `0px`이고 기대 computed serialization은 `normal`; simulator matrix에서 두 scene이 통과했다. |
| 6 | 값 parser가 공백·비표준 숫자 serialization을 조용히 정규화한다 | **통과.** scene의 정확한 문자열을 먼저 대조한다. `-1.00px`와 예상하지 않은 `0px` serialization은 거부된다. |
| 7 | `em`, 상대값 또는 `calc()`가 px처럼 처리된다 | **통과.** `px` suffix 및 고정 scene serialization만 허용하고 `2em`/`calc()` 거부 unit이 있다. |
| 8 | 음수 zero, NaN, Infinity 또는 범위 밖 숫자가 통과한다 | **통과.** `-0px`, `infpx`, `NaNpx`, `2.01px`, `3px`가 거부된다. |
| 9 | parser가 -2px를 읽는다는 이유로 기대 scene이 없는 runtime 입력도 허용된다 | **통과.** `anonymous-letter-spacing-minus-two-min-content`는 allowlist에 없고 validator negative test에서 `-2px`와 함께 거부된다. parser 한계와 실제 scene 범위는 계획/내부 계약에서 구분했다. |
| 10 | 중복되거나 빠진 scene이 simulator에서 성공으로 집계된다 | **통과.** FFI·runtime 고정 matrix가 16개 고유 ID와 negative control을 확인하고 Android/iOS allowlist assertion도 있다. |
| 11 | Grid item의 anonymous direct text 외 span/image 입력까지 이 변경이 승인한다 | **통과.** FFI source는 root 아래 단일 text node만 생성하고 runtime profile은 글꼴·방향·white-space 및 Grid style을 제한한다. 일반 CSS admission은 추가하지 않았다. |
| 12 | 다른 node의 computed style을 root 값으로 사용한다 | **통과.** validator는 root computed style의 `node_id`와 properties를 받고, 통합 test가 해당 root ID 및 item의 anonymous text layout을 확인한다. |
| 13 | 오래된 document revision의 computed value가 layout에 섞인다 | **통과.** integration 및 simulator trace가 계산·HostDocument document revision 일치를 요구한다. |
| 14 | computed `letter-spacing`은 확인하지만 `ParagraphStyle`에 전달하지 않는다 | **통과.** Rust integration은 paragraph input의 정확한 f32 값을 검사하고, FFI runtime trace에도 computed property가 기록된다. |
| 15 | Parley에는 값이 전달되지 않아 shaping/line break가 다른 입력을 쓴다 | **통과.** `StyleProperty::LetterSpacing`을 설정하고 Ahem의 `XX`·`XX XX` unit oracle 및 simulator matrix가 intrinsic/line advance를 대조한다. |
| 16 | line 끝에도 마지막 atom 간격을 남기거나 두 번 억제한다 | **통과.** 모든 비어 있지 않은 line에서 한 번 보정한다. 5개 spacing 값의 `XX` 폭과 min-content로 감긴 두 line advance가 독립 기대값과 일치한다. |
| 17 | min-content가 단어별 break 대신 한 줄 max-content 폭을 사용한다 | **통과.** `XX XX`의 min-content는 두 line을 만들고 CSS Sizing 기대치 20, 19, 20.5, 21, 22px를 맞춘다. |
| 18 | max-content가 공백을 잘라내거나 강제 줄의 폭을 합산한다 | **통과.** 무제한 break 결과의 line advance 최댓값을 선택한다. `XX XX`의 한 줄 폭 50, 46, 52, 54, 58px를 확인했다. |
| 19 | 음수 spacing의 underflow/비유한 값이 track sizing에 발행된다 | **통과.** text layout과 intrinsic 단계 모두 non-finite 또는 음수 advance를 오류로 반환한다. runtime mismatch test는 오류가 layout publication 전에 발생하는지 확인한다. |
| 20 | bounded 증거를 glyph paint, 전체 앱 release 또는 C11 완료로 확대 주장한다 | **통과.** Android/iOS 각 16/16은 computed style·track·frame 증거로만 기록했다. renderer screenshot은 검고 pixel readback은 없으며, archive 검사는 정적 `spinon-ffi` 한정이다. C11.4/C11 공식 수는 72/213으로 유지한다. |

**판정:** 20개 구현/runtime 관점을 대조했고 이 검토에서 수정한 admission 경계(`-2px` scene 미등록) unit test는 통과했다. Rust focused tests, 두 simulator matrix, default/fixture static archive 검사와 rustfmt가 통과했다. strict Clippy는 변경하지 않은 `runtime_layout.rs:960`의 `suspicious_else_formatting`에서 경고로 실패했고, 동일 경고 하나만 억제한 상태에서 대상 3개 crate의 나머지 경고를 오류 처리해 통과했다. 이 자료는 전체 C11.4/C11, 임의 font/text, WPT, glyph pixels 또는 실기기 동작을 증명하지 않는다.
