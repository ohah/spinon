# C11.4 `hyphens` 구현·runtime 실패 관점 검토 20개

검토 대상은 현재 `feat/c11-4-anon-text-hyphens` source와 2026-11-01 최신 Android/iOS Simulator matrix다. 구현 로직·CSS cascade·text source mapping·Grid trace·거부 경계를 서로 다른 실패 경로로 확인했다. 아래에는 판정에 사용한 구체적 근거와 남은 한계를 적는다.

| # | 실패 관점 | 확인 및 판정 |
|---:|---|---|
| 1 | CSS declaration 이름을 바꾼 뒤 stylesheet byte offset이 밀려 cascade source provenance가 어긋남 | `hyphens`와 `--shyph`는 같은 7-byte 길이로 치환한다. `initial` 값도 `manual` 뒤 공백으로 원래 길이를 보존한다. CSS adapter source-offset tests 통과. |
| 2 | 문자열/주석 안의 `hyphens:` 텍스트를 declaration으로 오인 | cssparser 기반 declaration scanner가 string/comment를 건너뛰며 nested stylesheet test가 원문 문자열 유지와 실제 declaration 변환을 확인. |
| 3 | 중첩 rule 안의 declaration을 놓치거나 재귀 중 stack 제한 없이 계속 진입 | `@media` 등 중첩 rule body를 검사하고 nested depth 64를 넘으면 fail closed. 대상 profile 자체는 media rule을 허용하지 않는다. |
| 4 | 작성자 `--shyph`가 예약 alias를 덮어 computed `hyphens`가 위조됨 | inline과 stylesheet의 alias collision을 각각 거부하는 tests 통과. |
| 5 | `@property --shyph` 등록이 alias의 syntax/inheritance 의미를 바꿈 | 예약 property registration을 거부하는 stylesheet test 통과. |
| 6 | `all` shorthand가 alias나 `hyphens` 상태를 reset하지만 일반 선언처럼 통과 | `all`은 inline 및 stylesheet에서 지원하지 않는 author feature로 거부하는 tests 통과. |
| 7 | `var()`, 함수 또는 nested token이 런타임에 다른 keyword로 resolve | 함수·괄호 token을 포함하는 값은 normalization 단계에서 거부한다. `var(--mode)` negative test 통과. |
| 8 | `initial`이 문자열 치환 과정에서 뒤 선언의 source range를 이동 | 같은 byte 길이로 `manual`에 정규화하고 source-offset 보존 test 통과. |
| 9 | 잘못된 static keyword가 Stylo에서 silently dropped되어 초기값 `manual`로 오인 | 원문 keyword를 cascade alias에 유지한다. `balanced` computed 값은 runtime validator가 layout 전에 거부하며 Android/iOS 두 reject scene 모두 통과. |
| 10 | CSS initial value 또는 미선언 상태가 다른 mode로 처리 | 미선언 scene과 `initial` scene의 computed style은 모두 `manual`; fixed 30px 결과가 두 platform에서 동일. |
| 11 | 상속받은 computed value가 direct Grid owner에 전달되지 않음 | parent `none|manual` 두 scene에서 style owner/root/item/text node와 frame을 검사. 양 플랫폼 모두 21/21 matrix 통과. |
| 12 | selector specificity, `!important`, 같은 specificity source order를 alias가 바꿈 | cascade tests에서 parent/child inheritance, ID/class 우선도와 source order, `!important` 승자를 확인. |
| 13 | `unset`, `revert`, `revert-layer`가 기본값 처리와 잘못 결합 | computed alias test에서 이 CSS-wide keyword는 alias value를 만들지 않고 runtime initial `manual` fallback을 유지. |
| 14 | profile 밖 font/style/writing/locale을 soft-hyphen 수식에 적용 | `normalize`가 Ahem 10px/400, line-height 10px, LTR horizontal, ASCII, `en` 또는 unspecified, normal wrapping 계열과 single direct source를 검사한다. font-size/text-transform negative unit tests 통과. |
| 15 | styled inline run이나 여러 source가 source character ownership을 섞음 | profile gate는 inline styles/runs, inline ancestors, 두 번째 `TextSource`를 거부한다. general CSS inline-run 지원으로 확장하지 않는다. |
| 16 | `hyphens:none`이 authored U+00AD의 UTF-16 mapping을 잃음 | layout copy의 U+00AD를 U+2060으로 one-source-unit 치환하며 unit test에서 paragraph UTF-8 range와 source UTF-16 `(2..3)`를 함께 확인. HostDocument 원문은 불변. |
| 17 | `none`이 일반 visible U+002D break까지 막거나 min-content를 잘못 계산 | U+002D positive control `AB-CD`의 min-content 30px, max-content 50px와 두 line advance를 `none|manual`에서 확인. |
| 18 | `manual` min-content가 selected soft-hyphen glyph 폭을 누락하거나 max-content에 더함 | min-content 측정 복사본에만 U+2010을 넣어 30px; max-content는 authored `AB­CD`를 유지해 40px. unit 및 runtime scene 통과. |
| 19 | 줄 끝이 아닌 SHY, 마지막 line, 여러 SHY에서 line advance 보정이 잘못 적용 | valid text range를 먼저 확인하고 non-final line이 U+00AD에서 끝날 때만 한 em을 더한다. 1개/2개 기회 fixed-width line traces가 30/20 및 30/30/20을 확인. |
| 20 | unsupported computed 값 또는 layout 실패 뒤 partial frame이 publish됨 | `auto`, `balanced`는 정확한 error/property/node로 거부되고 summary에서 frame 부재를 확인. 두 플랫폼 expected-reject cases 2/2. |

검토 중 Clippy가 CSS scanner의 nested `if`와 11-argument scene constructor를 지적했다. 조건식은 let-chain으로 정리하고 scene 생성은 값 변경 없이 macro literal로 바꿨다. 이후 fixture-enabled Clippy, style 7/7, runtime 2/2, FFI matrix 1/1, text-layout 4/4 및 양 플랫폼 최신 matrix를 다시 확인했다. 구현/runtime 리뷰에서 추가 미해결 결함은 찾지 못했다.

이 검토는 고정 Ahem profile과 현재 두 Simulator 경로에만 해당한다. 자동 hyphenation dictionary, 임의 font/language/script, glyph raster pixels, WPT, 실기기/hardware GPU, iOS compositor 이후 출력과 일반 author CSS admission은 검증하지 않는다.

전체 runtime fixture-test 회귀는 unit 394 passed/3 ignored, integration 19 passed/0 failed로 통과했다. 그 과정에서 stale `hyphens` unknown-property assertion과 기존 word-break MaxContent line-count assertion을 현재 계약 및 scene 정의에 맞춰 갱신했다. 변경은 test-only이며 runtime implementation review findings를 바꾸지 않는다.
