# `text-transform: full-width` 구현/runtime 실패 관점 검토 20개

검토 대상 source commit `2b6fec2572801a7e59db1db5c919587c65e792a9`의 `TextTransformMode::FullWidth`, anonymous Grid text-transform fixture 경로, Android/iOS runner mapping, 그리고 Chrome에서 관찰할 수 없는 표준 기대값 경계다. Android `android-avd-pass03`와 iOS `ios-simulator-pass02` commit-bound matrix 결과를 계획 review와 다른 실제 코드·런타임 관점에서 검토했다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | fixture feature가 꺼진 제품 경로에서 변환이 실행됨 | `supports_text_transform_profile`의 FullWidth arm은 feature-off에서 거부한다. 기본 text-layout test도 `UnsupportedTextTransformProfile`을 확인한다. |
| 2 | FullWidth mode가 non-Japanese locale에서 허용됨 | `locale == ja`를 요구하고 `en` negative control을 거부한다. |
| 3 | `lang` 누락·상속·`xml:lang`이 고정 언어처럼 처리됨 | Grid owner의 단일 직접 `lang="ja"`만 허용하고 상속/`xml:lang` 거부 tests를 유지한다. |
| 4 | computed `text-transform`이 scene ID와 다르게 cascade됨 | 네 scene별 computed value와 HostDocument style snapshot을 대조한다. 다른 value/누락은 layout publication 전에 거부한다. |
| 5 | scene ID만으로 stale document/style snapshot 재사용을 허용함 | 기존 revision 검증을 통과해야 하며 text-transform stale-revision rejection regression이 통과한다. |
| 6 | punctuation, lowercase, 임의 full-width source가 일부만 변환됨 | raw source 전체가 `AB 12` 또는 `AB   12`와 일치해야 하며 다섯 변형 negative inputs를 거부한다. |
| 7 | adjacent Text node나 inline owner를 넘어 변환 범위가 넓어짐 | text-layout profile은 source 하나만 받고 inline ancestor/style은 공통 gate에서 거부한다. |
| 8 | repeated whitespace가 FullWidth 매핑 전에 남아 출력에 여러 U+3000이 생김 | Phase I collapse 후 replacement가 실행된다. `AB   12`의 output은 `ＡＢ　１２`다. |
| 9 | multi-byte output range가 원본 UTF-16 unit과 혼동됨 | 각 full-width scalar의 paragraph UTF-8 byte range는 3 bytes, source range는 대응하는 1 UTF-16 unit임을 unit/runtime tests가 확인한다. |
| 10 | collapse된 U+0020 source 3개 중 일부 provenance가 사라짐 | 반복 공백 source 3개 모두가 같은 normalized U+3000 byte range를 가리키는지 확인한다. |
| 11 | Parley가 A/B 또는 1/2 사이에 min-content break를 삽입함 | 정확한 FullWidth/Normal 조합에서만 internal `KeepAll`; 20px min-content와 두 줄을 확인한다. |
| 12 | internal `KeepAll`이 U+3000 separator의 wrap을 막음 | 좁은 20px layout은 두 line으로 나뉘며 각 line advance가 20px임을 확인한다. |
| 13 | workaround가 max-content advance 또는 row sizing을 바꿈 | max-content 50px·one line 및 min/max-content heights 20/10px을 확인한다. |
| 14 | `none` control도 일본어 변환 경로에서 수정됨 | 고정 control은 raw/output `AB 12`와 Chrome target width tolerance 안의 max-content track을 유지한다. |
| 15 | computed CSS `word-break`가 Parley의 내부 setting으로 덮임 | runtime style snapshot은 CSS `normal`, `ParagraphInput.style.word_break`는 `Normal`이다. `TextLayoutSnapshot`에 없는 field를 보고한다고 주장하지 않는다. |
| 16 | computed `line-break:auto`가 scene contract와 다르게 입력됨 | CSS style snapshot의 `auto`와 typed `ParagraphInput`의 adapter mapping `Strict`를 각각 고정한다. |
| 17 | Grid owner direction/writing mode/font/whitespace 제한이 빠짐 | 공통 text profile validator가 LTR, horizontal-tb, Noto Sans KR, 10px/400, collapse+wrap, exact word/line-break를 검사한다. |
| 18 | FFI source가 허용되지 않은 scene, language, retry root를 만든다 | 27 unique IDs, exact source/text/transform, scene-bound retry cache, `ja` direct owner를 FFI source test로 확인한다. |
| 19 | 한 플랫폼 native runner는 새로운 scene/style/metric을 모름 | Android와 iOS scene dispatcher, source/output, locale, metrics, tolerance를 같은 4-ID matrix로 연결하고 앱 build와 simulator captures를 실행한다. |
| 20 | opt-in fixture 문자열이 기본 FFI artifact에 노출되거나 positive control도 누락됨 | `c04-runtime-gpu` archive marker scan은 0, `c04-runtime-gpu-fixture-test` positive control에는 네 scene marker가 있다. APK/IPA 전체 exclusion으로 확대 해석하지 않는다. |

추가로 implementation review 과정에서 `fixture-module-boundary.test.mjs`가 runtime scene 총계를 129로 고정하고 있어 네 scene 추가 뒤 실패했다. 총계를 133으로 갱신한 뒤 boundary test 32/32가 통과했다. 이는 변경으로 드러난 검사 유지보수 결함을 고친 것이며 제품 동작 실패는 아니다.

시뮬레이터 screenshot은 fixture host의 `layout=ready` 상태를 보이며 renderer의 검은 surface에 glyph를 그리지 않는다. 따라서 본 review와 run은 CSSOM/paragraph/intrinsic/grid layout 경로를 확인하지만 glyph raster·pixel equality·실기기 GPU는 확인하지 않는다. Chrome 156 computed `text-transform:none`은 unsupported-value negative control이다. CSS Text Level 3 CRD의 at-risk `full-width` 동작과 Chrome 상호운용성을 주장하지 않는다.
