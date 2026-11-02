# `full-size-kana` 구현/runtime 실패 관점 검토 20개

검토 대상은 제한 opt-in 후보의 구현 source `e6729ffbde7955f14f762232f188c02e845a755d`와 이를 고정하는 scene/test/runtime adapter다. 이후 `cf0265b41`은 fixture boundary test의 scene 수 기대값만 갱신했다. 계획 자체의 검토와 PR 변경 전체 검토는 별도 기록한다. 이 검토는 C11.4 또는 CSS 공개 지원을 승인하지 않는다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | 기본 빌드에서 새 변환이 일반 text layout에 새어 나오는가 | `c11-4-text-transform-unicode-fixture-test` 미사용 분기에서 `FullSizeKana`를 지원하지 않고 focused feature-off test가 거부를 확인한다. |
| 2 | Android/iOS harness가 fixture opt-in을 우회하거나 서로 다른 flag를 사용하는가 | 두 플랫폼 빌드는 `SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1` opt-in을 명시했다. 기본/opt-in static archive scan도 각각 marker 0개와 10개를 확인한다. APK/IPA 전체 번들의 제외 여부는 확인하지 않았다. |
| 3 | enum에 새 variant를 추가하면서 다른 text-transform mode의 dispatch가 바뀌는가 | `TextTransformMode` dispatch에서 `FullSizeKana`를 별도 분기했고 기존 ASCII, Unicode capitalize 및 full-width 분기를 따로 유지한다. text-layout/runtime 회귀 로그가 각각 통과했다. |
| 4 | scene ID만 주면 CSS 값이 없어도 새 변환이 승인되는가 | validator는 scene의 expected computed value와 실제 `text-transform` property를 exact 비교한다. 누락·다른 값 및 대소문자/공백 변형의 negative assertion이 있다. |
| 5 | `lang=ja`가 조상 상속만으로 통과하는가 | adapter는 Grid style owner element에서 직접 `lang` 하나를 확인한다. 조상에만 언어가 있는 경우는 거부된다. |
| 6 | `xml:lang`이 직접 `lang` 대신 허용되는가 | XML namespace의 `lang` 존재는 profile 밖으로 거부하고 unqualified `lang="ja"` 하나만 인정한다. |
| 7 | 언어 값의 대소문자, 공백 또는 확장 태그가 느슨하게 정규화되는가 | 허용 문자열은 exact `ja`다. 다른 값은 owner-language 검증에서 실패한다. |
| 8 | 잘못된 방향·줄바꿈·공백·분할 정책에서 변환이 실행되는가 | text-layout profile은 LTR, normal white-space, normal word-break, strict line-break와 Japanese locale을 요구한다. RTL, pre, break-all, 잘못된 line-break 각각의 거부 회귀가 있다. |
| 9 | 변환 대상이 고정된 한 개 source보다 넓어질 수 있는가 | paragraph source가 정확히 하나이고 source text가 9개 allowlist 값 중 하나인지 검사한다. 미등록 문자와 여러 source 조각은 거부된다. |
| 10 | Appendix G 매핑표에 source 중복·누락이 있거나 58개 외 항목이 들어가는가 | Rust table 길이와 source uniqueness를 검사하며 모든 table row를 lookup/replacement에 대조한다. 고정 매핑 자료는 Unicode 15.0 Appendix G 58행에 한정된다. |
| 11 | 매핑 목적 코드 포인트가 잘못되거나 단순 호환 분해로 확대되는가 | 독립 기대값 JSON과 script-group scene을 대조했다. 정규화/호환 분해 라이브러리를 호출하지 않고 명시된 source→destination 쌍만 바꾼다. |
| 12 | 매핑되지 않는 문자, ASCII와 큰 Kana까지 바뀌는가 | `あ`와 `A`의 direct lookup은 None이며 mixed pass-through scene은 `あカンABC 12` identity를 확인한다. |
| 13 | 공백 정규화 결과를 기준으로 source byte offset을 계산할 때 경계가 어긋나는가 | replacements는 normalized paragraph UTF-8 character boundary에서 만들고, 기존 normalization source range를 output character 단위로 다시 투영한다. |
| 14 | supplementary Kana가 2 UTF-16 unit에서 1-unit output으로 바뀔 때 HostDocument range를 잃는가 | text-layout matrix는 각 source character의 원문 UTF-16 range와 output UTF-8 range를 따로 검증하고, HostDocument 원문 보존 runtime matrix도 통과했다. |
| 15 | 한 source가 여러 output character가 되는 상황에서 mapping algorithm이 잘못된 가정을 하는가 | 현재 58개 mapping은 source code point당 destination code point 하나다. replacement remapper는 output character range를 순회하므로 같은 크기라고 가정하지 않는다. 다문자 expansion은 이 profile 밖이다. |
| 16 | replacement가 없는 pass-through에서 원문 mapping이나 tab offset을 불필요하게 다시 쓰는가 | replacement 목록이 비면 remapper를 호출하지 않고 기존 normalized result를 유지한다. 고정 none 및 transform pass-through scene이 동일 입력/출력 geometry를 대조한다. |
| 17 | 변환이 line breaking 또는 intrinsic sizing 뒤에 적용돼 track 기여가 원문 기준으로 남는가 | normalize pipeline은 white-space normalization 뒤 `text_transform::apply`, 이후 shaping/intrinsic sizing을 수행한다. scene 및 text-layout 검사에서 변환 결과의 min/max intrinsic이 literal 기대 입력과 일치한다. |
| 18 | 고정 scene 기대값과 플랫폼 보고의 node identity/frame cardinality가 맞지 않는데 geometry만 통과하는가 | Android/iOS verifier는 full-size-kana에서 root와 직접 text item 두 frame의 index/node ID와 각 x/y/width/height를 `1/32 CSS px` 허용오차로 확인한다. |
| 19 | font advance 차이나 trace serialization 차이를 CSS parity로 과장하는가 | Chrome 비교를 literal geometry에만 제한하고 Katakana min-content의 `9.5/10px` 대 `10/10px` line advance 차이를 명시했다. runtime 판정은 fixed expected metrics와 trace 기준이며 glyph pixel parity를 주장하지 않는다. |
| 20 | 장문 일본어 trace가 UTF-8 byte 수와 Swift character index를 혼동해 일부 chunk를 누락하는가 | 재현된 iOS pass01의 chunk 수 불일치를 `trace.count` 기준으로 고쳤고 source e672의 iOS pass02 10/10으로 확인했다. 실패 기록은 별도 보존했다. |

## 판정

위 20개 관점에서 이 제한 opt-in fixture profile을 차단할 추가 결함은 확인하지 못했다. 이는 일본어 일반 단어 분절 경고, Chrome positive oracle 부재, Katakana line-advance 차이, glyph paint 미검증, WPT/실기기/hardware GPU 부재를 해소하지 않는다. 따라서 이 후보는 PR 검토 대상으로 올릴 수 있지만 C11.4/C11 공식 완료나 공개 CSS admission으로 승격할 수 없다.

실행 세부값과 실패 cohort는 [runtime README](./README.md), [Android matrix](./android-avd-pass05/matrix.tsv), [iOS matrix](./ios-simulator-pass02/matrix.tsv)에 있다.
