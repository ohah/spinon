# C11.4 Unicode titlecase 구현 실패 경계 검토 20개

계획 검토와 분리해 source commit 84d70e8e, opt-in dependency 경계, app validator 및 최종 Android/iOS 23-scene 결과를 기준으로 서로 다른 실패 경로를 확인했다.

| # | 실패 관점 | 확인 및 결과 |
| --- | --- | --- |
| 1 | Unicode case mapping dependency가 기본 제품에 포함됨 | 기본 spinon-text-layout dependency tree에는 icu_casemap이 없고 fixture feature를 켠 tree에만 있다. 기본 FFI rlib/staticlib에는 Unicode scene marker가 없고 fixture artifact에는 있다. |
| 2 | fixture feature가 꺼져도 Unicode 경로가 활성화됨 | Unicode test module과 case mapping dependency는 opt-in feature로 gate된다. feature-off text-layout 96/96, feature-on 102/102가 통과했다. |
| 3 | transform이 white-space collapse보다 먼저 실행됨 | normalize가 NormalizedText를 완성한 뒤 text_transform::apply를 호출한다. 연속 공백 입력은 Sseta Éclair이며 원문 공백 source 세 범위가 같은 collapsed output byte span을 가리킨다. |
| 4 | 단어 분절이 ASCII 공백 기준으로만 동작함 | ICU WordSegmenter UAX #29 경계로 well-known은 Well-Known, o'neill은 O'neill이 된다. 두 경계 유형이 rust와 23-scene app matrix에 포함됐다. |
| 5 | 숫자도 첫 typographic unit 규칙에서 빠짐 | 첫 단위가 Number인 49ers는 capitalize에서도 그대로이며 none control과 폭이 일치한다. |
| 6 | 여는 문장부호 뒤 첫 글자를 찾지 못함 | «élan»에서 punctuation unit을 건너 첫 소문자 letter를 É로 바꾸며, runtime scene이 두 platform에서 통과했다. |
| 7 | 모든 scalar를 대문자화하거나 뒤 문자의 case를 덮어씀 | titlecase mapper는 word segment의 첫 lowercase typographic unit만 바꾸고 trailing case를 unchanged로 설정한다. 기대 문자열 Sseta와 o'neill이 두 platform에서 통과했다. |
| 8 | full case mapping의 scalar 확장이 유실됨 | ß→Ss, ǆ→ǅ, Deseret 𐐨→𐐀을 확인했다. runtime Sseta의 두 출력 scalar는 모두 같은 원문 UTF-16 span을 보존한다. |
| 9 | locale을 적용하지 않고 모든 언어에서 같은 casing을 선택함 | Unicode profile은 정확한 en만 허용한다. de locale negative test는 UnsupportedTextTransformProfile로 거부된다. |
| 10 | white-space:pre 등 미지원 모드에서 collapse 가정을 적용함 | Unicode profile은 white-space:normal로 제한한다. Pre 입력 negative test가 layout publication 전에 거부된다. |
| 11 | inline run 또는 styled ancestor를 평문 paragraph로 잘못 합침 | inline styles, inline run styles, source inline ancestors 중 하나라도 있으면 profile validation이 실패한다. inline source negative test가 거부를 확인했다. |
| 12 | 임의 Unicode source가 allowlist 경계를 통과함 | supports_unicode_capitalize_source는 고정 문자열만 허용한다. allowlist 밖 ßeta! negative test가 거부된다. |
| 13 | 기존 ASCII-only uppercase/lowercase 지원이 우연히 Unicode 전체 casing으로 넓어짐 | Unicode accented input의 uppercase mode는 거부된다. 이는 이 candidate가 capitalize fixture slice만 추가함을 유지한다. |
| 14 | 인접 direct Text node를 단어 경계로 취급함 | ß와 eta 두 direct source node는 한 문자열 Sseta로 변환된다. source node identity와 각 UTF-16 범위도 보존된다. |
| 15 | supplementary source scalar의 UTF-16 길이를 1로 가정함 | source normalization은 char.len_utf16() 기준이다. case-map unit test가 supplementary Deseret 변환을 확인하고 전체 기존 text-layout suite도 통과했다. |
| 16 | combining mark를 별도 word로 바꾸거나 삭제함 | e + U+0301 입력은 E + U+0301로 보존된다. source mapping test와 iOS/Android combining scene이 통과했다. |
| 17 | 변환된 UTF-8 byte range의 경계를 다시 매기지 않음 | 각 출력 scalar마다 고유 paragraph byte range를 생성하고 source range를 복사한다. ß→Ss 및 collapse range assertions가 통과했다. |
| 18 | superseded eval 재시도 때 같은 scene을 중복 생성함 | FFI source는 동일 document에 root가 남아 있으면 root와 nodeIds를 재사용한다. source guard 순서와 stored node IDs 검사가 통과했고 두 simulator의 최종 재시도 가능한 matrix가 23/23이다. |
| 19 | Unicode 기대값 분기가 기존 ASCII matrix 검증까지 바꿈 | 첫 iOS run은 신규 11개는 통과하고 ASCII 12개는 app 내부 validator false로 실패했다. 플랫폼 helper를 Unicode scene 전용으로 좁힌 뒤 구조 검사 및 최종 양 플랫폼 ASCII 12개가 통과했다. 실패 artifact는 보존했다. |
| 20 | 표준 기대값, Chrome 관측, 실제 app layout을 같은 수치로 혼동함 | well-known Chrome CSSOM 56.171875와 Canvas 56.165863을 분리하고 app actual 56.139996을 0.03125 CSS px tolerance로 비교했다. 두 simulator가 scene을 통과했으며 screenshots는 glyph pixel 증거로 취급하지 않았다. |

## 판정

최종 source commit에서 제한된 Unicode candidate 및 보존된 ASCII 회귀 행렬은 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 23/23 통과했다. 전체 workspace Clippy의 기존 미수정 warning은 [실행 근거](README.md)에 분리 기록했다. 일반 CSS admission, hardware/physical device, glyph raster, full WPT 및 C11.4/C11 completion은 이 검토의 판정 범위 밖이다.
