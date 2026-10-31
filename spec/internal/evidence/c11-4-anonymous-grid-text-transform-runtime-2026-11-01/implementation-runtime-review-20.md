# C11.4 computed `text-transform` 구현·runtime 실패 관점 검토 20개

대상은 [source-revision.json](./source-revision.json)에 해시한 최종 구현, Android/iOS 실행 matrix, feature-test archive다. 기준 동작은 [Chrome 156/독립 font advance 사전 비교](../c11-4-anonymous-grid-text-transform-precomparison-2026-11-01/README.md)와 [작업 계획](../../../../plan/c11-4-anonymous-grid-text-computed-text-transform.md)이다. 검토 중 발견해 고친 항목은 아래 표에 결과를 기록했다.

| # | 실패 관점 | 확인한 경계와 결과 |
| ---: | --- | --- |
| 1 | 기존 paragraph의 기본 mode 회귀 | `TextTransformMode::None`이 기본값이다. 전체 `spinon-text-layout` 86/86, feature runtime 379/379 본 테스트에서 기존 paragraph 회귀는 없었다. |
| 2 | 변환이 DOM/source text를 덮어씀 | normalization은 `SourceCharacter` 복사본의 layout 문자를 바꾼다. runtime은 HostDocument 원문 `IIII ii`와 layout 문자열을 따로 확인한다. |
| 3 | 대소문자 치환으로 UTF-16/UTF-8 range가 이동 | 허용 입력의 ASCII mapping은 code point당 한 글자다. 7개 source range가 `0..1`부터 `6..7`까지 text node와 normalized range에 그대로 연결되는 테스트가 통과했다. |
| 4 | Unicode casing을 불완전하게 지원한다고 오인 | `text-transform != none`일 때 non-ASCII source는 shaping 전에 거부한다. locale-sensitive casing, 확장·다중 code point mapping은 이번 profile에 들어오지 않는다. |
| 5 | inline ancestor/style/run에 부분 적용 | non-None 변환에서 inline styles, inline run styles와 inline ancestor가 있으면 `UnsupportedTextTransformProfile`로 실패한다. |
| 6 | `capitalize`, `full-width` 등 미지원 키워드 허용 | runtime allowlist는 정확히 none/uppercase/lowercase 여섯 scene이다. 다른 computed serialization은 `unsupported_c114_anonymous_text_style`로 실패한다. |
| 7 | scene ID만으로 computed CSS 값을 대신함 | adapter는 root의 computed property를 scene 기대값과 비교한 뒤 typed mode를 만든다. 값 불일치와 property 누락은 layout 없이 실패한다. |
| 8 | transform scene 접두어의 미등록 ID가 일반 경로로 흘러감 | unknown `text-transform-*` scene은 opt-in anonymous-text adapter에서 거부한다. FFI는 고정 여섯 ID만 생성한다. |
| 9 | 자손/다른 node의 style을 Grid owner로 사용 | runtime은 Grid root NodeId의 `ComputedElementStyle`을 조회하고 직접 Text child 구조를 검증한다. frame과 trace도 같은 source node를 사용한다. |
| 10 | 조상 `lang`을 조용히 상속 | 정확한 Grid owner의 비 namespace `lang="en"`만 허용한다. 조상에만 언어가 있는 경우와 `en-US`는 거부된다. |
| 11 | `xml:lang`을 HTML `lang` 대신 사용 | XML namespace `lang`이 있으면 거부한다. 일반 CSS 상속 계약을 이 fixture에서 넓히지 않는다. |
| 12 | stale computed-style revision 사용 | adapter 진입에서 profile과 generation/document/render-tree revision을 HostDocument snapshot과 대조하도록 보강했다. 고정 테스트는 document와 render-tree revision 불일치를 거부한다. |
| 13 | 추가 element/text child가 fixed layout에 섞임 | direct child validator는 예상하는 단일 text source 외의 topology와 내용을 거부한다. runtime positive matrix는 동일 node의 단일 source를 확인한다. |
| 14 | system font 대체로 기대 advance를 우연히 만족 | Noto Sans KR의 고정 font bytes를 명시 등록하고 10px/400·`lang=en`을 검사한다. simulator trace의 font family/revision과 독립 glyph advance를 대조한다. |
| 15 | min-content를 전체 string advance로 계산 | 두 단어의 독립 advance 중 큰 값을 min-content로 사용한다. min scene은 두 줄·20px auto row와 최종 column track을 검사한다. |
| 16 | max-content에서 collapsed separator 누락/중복 | 고정 source `IIII ii`의 공백 하나를 포함한 전체 advance를 max-content와 한 줄 line advance로 검사한다. |
| 17 | 대소문자별 track 또는 auto-row metric 혼동 | none/uppercase/lowercase × min/max 여섯 경우의 independent expected와 Android/iOS trace가 모두 0.03125 CSS px 허용치 안이다. |
| 18 | computed mode가 trace equality/revision에서 빠짐 | `TextLayoutSnapshot`은 `text_transform`을 저장하고 equality에 포함한다. JSON trace와 두 host validator가 typed mode를 확인한다. |
| 19 | Android fixture-off 경로에서 helper가 새 scene을 해석 | 검토 중 Android style helper가 flag 밖에서 mode를 고르는 점을 발견했다. `BuildConfig.SPINON_C04_RUNTIME_GPU_FIXTURE_TEST` 검사로 닫고 boundary Node 검사를 추가했다. 최종 boundary suite는 30/30이다. |
| 20 | 실행·artifact 근거를 더 강하게 해석 | Android AVD와 iPhone Simulator는 각 6/6이고 설치 artifact hash가 일치한다. macOS debug FFI archive raw marker는 feature-off 0개/opt-in scene 6개와 fixture export-name marker 2개다. Xcode `nm`의 LLVM 22 archive 읽기 실패 때문에 symbol-table 검사는 주장하지 않으며 APK/IPA 전체 제외도 주장하지 않는다. 화면은 `layout=ready` 상태뿐이고 glyph paint 근거가 아니다. |

검토 도중 기존 font-size fixture-boundary 정규식이 `ValidatedAnonymousTextStyle` 인자 구조 변경을 반영하지 않아 한 차례 실패했다. 검사는 새 구조의 font-size 검증과 paragraph 전달을 확인하도록 고쳤고, boundary suite 최종 실행은 30/30 통과했다. 별도의 simulator 실행 실패는 [보존된 시도](./README.md#preserved-rejected-attempts)에 원인과 재시도를 분리해 기록했다.

이 검토는 제한 opt-in runtime fixture의 실패 경계를 대상으로 하며 일반 CSS admission, 전체 C11.4/C11, WPT, glyph pixels, 실기기/hardware GPU 또는 iOS compositor 이후 픽셀을 확인하지 않는다.
