# `text-transform: full-width` PR diff 실패 관점 검토 30개

검토 대상은 구현 commit `2b6fec2572801a7e59db1db5c919587c65e792a9`, runtime/evidence commit `06e3e8483841e68c0e1a11bc225550557591c0fc`와 이 검토 기록의 변경분이다. 기존 계획·구현/runtime 검토를 재사용하지 않고, 이 PR의 코드 경계·플랫폼 adapter·상태 문서를 서로 다른 실패 경로로 확인했다. 기존 Android API 37 ARM64 AVD `android-avd-pass03`과 iPhone 17 Pro/iOS 26.2 Simulator `ios-simulator-pass02`의 source-bound 실행 기록도 대조했다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | fixture feature가 꺼져도 `FullWidth` 변환이 제품 layout에 들어감 | `supports_text_transform_profile`의 feature-off arm은 거부하고 기본 feature test는 `UnsupportedTextTransformProfile`을 확인한다. |
| 2 | opt-in mode가 일본어 locale 밖에서 허용됨 | text-layout profile은 `locale == ja`를 요구하고 `en` 거부 control을 실행한다. |
| 3 | 허용 source보다 넓은 문자열을 부분 변환함 | validator는 단일 source가 `AB 12` 또는 `AB   12`와 정확히 일치해야 한다. |
| 4 | 여러 inline/source run을 한 문장처럼 합쳐 변환함 | `sources.len() == 1`을 요구하며 split-source input을 거부한다. |
| 5 | 구현 mapping이 허용 문자 밖 punctuation을 임의 처리함 | mapping helper는 ASCII alphabet/digit/space만 치환하며 profile allowlist가 실제 입력을 더 좁게 고정한다. |
| 6 | 대문자·소문자·숫자 offset에서 Unicode full-width scalar가 틀림 | 고정 ASCII 입력의 각 scalar를 U+FF21/U+FF10 계열 규칙에 따라 만들고 expected paragraph text와 비교한다. |
| 7 | ASCII space가 일반 full-width space가 아닌 문자로 바뀜 | U+0020은 U+3000으로 치환하고 scene expected output을 `ＡＢ　１２`로 고정한다. |
| 8 | transform이 white-space collapse보다 먼저 실행되어 연속 U+3000이 남음 | normalize pipeline은 collapse 후 transform하며 `AB   12` 실행은 U+3000 하나만 남긴다. |
| 9 | 원본 HostDocument text를 변환 결과로 덮어써 retry나 DOM 조회가 바뀜 | layout test는 source text가 원문 그대로 남는지 별도 확인한다. |
| 10 | UTF-8 paragraph byte range와 UTF-16 source range를 같은 단위로 취급함 | ASCII source별 1 UTF-16 unit과 full-width output별 3 UTF-8 bytes의 mapping을 검사한다. |
| 11 | 세 번의 공백 collapse에서 원본 공백 ownership을 잃거나 순서를 바꿈 | 반복 공백 세 source range가 정규화된 U+3000 byte range에 각각 연결되는지 runtime test가 확인한다. |
| 12 | 내부 break 조정이 CSS computed `word-break` 계약을 바꿈 | runtime computed snapshot과 `ParagraphInput.style.word_break`는 `normal`/`Normal`로 유지한다. |
| 13 | 내부 `KeepAll` 우회가 FullWidth 밖의 일반 text에도 적용됨 | intrinsic adapter 조정은 FullWidth/Normal 분기이며 fixture profile validator가 exact allowlist를 먼저 제한한다. |
| 14 | KeepAll이 full-width 글자·숫자 안에서 min-content break를 허용함 | 20px min-content track과 두 line을 layout/runtime matrix에서 확인한다. |
| 15 | KeepAll이 U+3000 separator를 통한 wrap까지 막음 | 20px 폭에서 `ＡＢ`와 `１２` 두 줄이 만들어지고 각 advance는 20px다. |
| 16 | transformed text의 max-content가 font advance와 다르게 계산됨 | 50px max-content와 single-line advance를 text-layout, runtime, 두 simulator expectation에서 검사한다. |
| 17 | automatic row sizing이 min/max-content 결과와 어긋남 | min-content height 20px, max-content height 10px 및 Rows auto track을 고정한다. |
| 18 | `none` control까지 변환되거나 full-width 기대 폭을 사용함 | 고정 control은 `AB 12`, `none`, 독립 Noto Sans KR 측정 허용 오차를 유지한다. |
| 19 | `lang`가 상위 상속·`xml:lang`·누락으로 조용히 대체됨 | runtime owner validation은 직접 HTML `lang`만 요구하고 `xml:lang`, 중복/불일치/누락을 거부한다. |
| 20 | scene ID만 맞으면 잘못된 computed `text-transform`을 통과시킴 | snapshot의 실제 computed 값과 scene 값이 정확히 일치해야 한다. |
| 21 | 문서/style revision이 바뀐 뒤 과거 transform 결과가 재사용됨 | 기존 snapshot revision gate와 stale-revision 거부 regression을 보존한다. |
| 22 | 새 scene ID가 기존 direct-text retry root와 충돌함 | FFI retry root cache는 `sceneId`와 parent ownership을 같이 확인하며 다른 scene이면 새 root를 만든다. |
| 23 | runtime, FFI, Android/iOS scene 목록 사이 ID가 빠지거나 중복됨 | runtime 133개·text-transform 27개·FFI 27개 고정 catalog uniqueness test와 4 ID 교차 대조가 있다. |
| 24 | FFI fixture source가 locale/style contract를 앱과 다르게 생성함 | 신규 scene은 `lang=ja`, `word-break:normal`, `line-break:auto`, exact source/text-transform을 생성한다. |
| 25 | Android 기대값 adapter가 FullWidth scene을 기존 Unicode capitalize branch로 분류함 | 모든 computed text-transform scene을 generic intrinsic validator에 보내고 전용 metrics/output/source mapping을 둔다. |
| 26 | iOS 기대값 adapter가 Android와 다른 locale·expected text를 사용함 | Swift adapter도 같은 four scene IDs, `ja`, source/output, line/track metrics를 별도로 검사한다. |
| 27 | simulator runner가 비교해야 할 computed property를 확인하지 않음 | capture validator는 transform, white-space, word-break, line-break, font, locale과 geometry를 검사한다. |
| 28 | Chrome이 `full-width`를 거부하는 관측을 성공 oracle로 오인함 | precomparison은 CSS Text/Unicode expected mapping과 Chrome `computed none`/literal candidate 관측을 분리한다. |
| 29 | fixture payload가 기본 archive에 들어가거나 positive control에서 사라짐 | 기본 static archive marker scan은 0, opt-in fixture build에는 네 scene ID가 있으며 README는 이를 APK/IPA 전체 보증으로 확대하지 않는다. |
| 30 | 제한 후보를 C11.4 완료·공식 완료 수 증가로 기록함 | `STATUS.md`와 계획은 C11.4/C11 미완료 및 `72/213`을 유지하고 runtime candidate·glyph pixel 한계를 별도로 기록한다. |

검토 중 병합을 막는 새 결함은 발견하지 않았다. 실행 근거는 layout/CSSOM/runtime scene 검사다. representative screenshot은 `layout=ready` harness 상태만 나타내므로 glyph raster나 pixel equality를 입증하지 않는다. PR check rollup이 비어 있으면 CI 통과로 기록하지 않는다.
