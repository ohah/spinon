# C11.4 anonymous Grid `text-indent` 구현 PR 변경 검토 30개

검토 대상은 고정 C11.4 anonymous direct-text Grid `text-indent` 변경 전체다. 구현 실패 관점 20개 검토와 분리해 최종 source, opt-in fixture 경계, Android/iOS harness, 공식 상태·명세 문서 변경을 다시 대조했다.

| # | 별도 실패 관점 | 판정과 근거 |
|---:|---|---|
| 1 | 후보가 일반 CSS author route에 우발적으로 열리는가 | 통과. 새 scene은 C11.4 opt-in fixture FFI adapter에만 등록되고 unlisted scene은 source 생성 및 runtime 준비 단계에서 거부된다. |
| 2 | scene ID 목록이 기대 matrix와 어긋나는가 | 통과. FFI source·runtime allowlist·독립 expected JSON은 동일한 18개 unique scene을 사용한다. |
| 3 | 임의 node topology가 direct-text 입력으로 오인되는가 | 통과. runtime validator는 장면별 정확한 단일 direct Text child와 고정 source 문자열을 요구한다. |
| 4 | 테스트 입력이 `text-indent` 값을 실제 cascade에 전달하지 않는가 | 통과. FFI source는 root inline style에 정확한 `0px`·`5px`·`10px`을 기록하고 root computed style에서 대조한다. |
| 5 | computed serialization과 scene 값이 불일치해도 layout이 publish되는가 | 통과. runtime validator는 scene별 정확한 serialization과 수치 bit pattern을 검증하고 mismatch에서는 layout 결과를 거부한다. |
| 6 | parser가 고정 px profile 밖의 표기까지 허용하는가 | 통과. parser는 `px` suffix, whitespace, 유한 수치, 0..10 범위와 scene별 exact serialization을 함께 확인한다. |
| 7 | 유효한 음수 CSS를 일반적으로 무효라고 잘못 문서화하는가 | 통과. 음수는 이 positive-px profile에서만 거부하며 계획·계약·검토 기록은 CSS 전체의 무효값이라고 주장하지 않는다. |
| 8 | percentage·`hanging`·`each-line`이 px 값으로 취급되는가 | 통과. 이 값들은 candidate profile에서 거부되고 CSS 전체 지원 여부와 구분되어 문서화된다. |
| 9 | 기존 장면에 비영(非零) indent가 새어드는가 | 통과. 등록되지 않은 기존 anonymous-text scene은 computed `text-indent:0px`을 계속 요구하고 기존 `ParagraphStyle` 호출은 0을 받는다. |
| 10 | 새 `ParagraphStyle` field가 기존 기본 동작을 바꾸는가 | 통과. `Default` 및 기존 명시 literal은 0을 지정하며 기존 기본 horizontal profile 회귀가 실행됐다. |
| 11 | NaN·무한값이 Parley layout까지 전파되는가 | 통과. `ParagraphStyle::validate`에서 finite 값을 요구하고 비유한 metric 회귀 검사가 있다. |
| 12 | Parley에 indent가 줄바꿈 후 늦게 전달되는가 | 통과. layout build 직후 `set_text_indent`를 호출하고 intrinsic line breaking보다 먼저 설정한다. |
| 13 | indent가 모든 soft line에 반복 더해지는가 | 통과. intrinsic 폭은 첫 formatted line에만 더하고 여러 줄 사례는 첫 glyph와 line advance를 별도 확인한다. |
| 14 | min-content가 첫 단어만 보고 더 긴 후속 단어를 놓치는가 | 통과. 독립 unit regression은 `X XXXXX`에서 후속 50px 단어를 유지하고 `XXXXX X`에서는 첫 줄 indent를 더해 55px로 계산한다. |
| 15 | max-content에서 separator advance를 빠뜨리는가 | 통과. Ahem 10px 기준 독립식은 남은 collapsed separator를 포함하고 18 scene의 min/max 기대값과 일치한다. |
| 16 | indent를 max-content에 두 번 더하는가 | 통과. 단어·separator의 unindented advance와 indent를 분리하며 5px 두 단어 값 55px 및 양 플랫폼 18 scene trace를 대조한다. |
| 17 | line advance와 first glyph 위치를 같은 폭으로 혼동하는가 | 통과. layout line advance는 unindented 값으로 유지하고 첫 glyph x에는 indent를 적용한다. Rust integration test와 simulator runner가 두 값을 독립 검사한다. |
| 18 | `white-space:normal` 반복·후행 공백이 글자 폭으로 남는가 | 통과. raw `XX   XX XX  `와 normalized `XX XX XX`를 구분하고 2개 separator 기준 값을 검사한다. |
| 19 | indent가 row height 또는 줄 수를 바꾸는가 | 통과. min-content 줄 수 1·2·3과 max-content 1줄, line-height 10px 및 최종 Rows track을 확인한다. |
| 20 | vertical intrinsic profile이 미검증 동작으로 확장되는가 | 통과. 해당 profile 조건은 `text_indent == 0`을 요구하고 nonzero 거부 회귀가 있다. |
| 21 | computed style이 오래된 HostDocument revision에서 재사용되는가 | 통과. 같은 계산 root의 document revision과 Grid diagnostic source revision을 snapshot revision에 연결한다. |
| 22 | Grid adapter가 paragraph 값과 최종 track contribution을 분리하지 못하는가 | 통과. 통합 검사는 intrinsic min/max, item frame, Columns track·contribution, Rows track을 각각 확인한다. |
| 23 | FFI trace가 Swift/Java에 필요한 값을 내보내지 않는가 | 통과. `paragraphStyle.textIndent`를 수치로 기록하고 양 플랫폼이 정확한 기대값과 비교한다. |
| 24 | Android fixture가 제품 빌드에서 켜지는가 | 통과. text-indent scene route는 Android fixture-test BuildConfig 조건에 묶였고 기본 artifact 음성 대조가 별도로 기록됐다. |
| 25 | iOS scene 진입 조건이 Android와 달라 장면을 건너뛰는가 | 통과. iOS track-sizing scene gate에 candidate detector를 연결했으며 최신 iOS 빌드·18/18 실행 로그가 있다. |
| 26 | 두 harness가 `availableWidth`와 shaped text width를 혼동하는가 | 통과. available width는 track 폭, shaped line width는 indent 제외 advance로 비교하며 수정된 Android/iOS 양쪽 행렬이 통과했다. |
| 27 | 고정 expected 데이터가 Chrome 관측을 그대로 복사하는가 | 통과. expected JSON은 Ahem font advance와 독립 min/max 식으로 계산하고 Chrome 156 값은 사전 관측 파일에 별도로 둔다. |
| 28 | Simulator 행렬이 다른 build 또는 설치본을 검사하는가 | 통과. Android APK와 iOS executable의 build/install SHA가 simulator metadata에 같게 기록되고 두 matrix에 18 scene PASS가 있다. |
| 29 | opt-in fixture가 기본 Rust artifact에도 들어가는가 | 통과. macOS release static archive 기본/positive-control에서 scene payload와 adapter/validator 심볼을 각각 0개/예상 개수로 대조했다. 이 근거를 APK/IPA 전체 경계로 확대하지 않는다. |
| 30 | slice를 C11.4/C11 공식 완료나 픽셀 적합성으로 과장하는가 | 통과. `STATUS.md`, 내부 계약, plan, 실행 README는 parent check 미완료와 `72/213`을 유지하고 black renderer 한계를 명시한다. |

## 판정

이번 구현 diff에서 수정이 필요한 추가 결함은 찾지 못했다. 검증 근거에는 `spinon-text-layout` 76 passed, runtime 356 passed·3 ignored, FFI 177 passed·2 ignored, Node 26 passed, Android/iOS Simulator 각각 18/18, 관련 Clippy/build와 기본/fixture-test archive 비교가 포함된다. `spinon-style-to-layout`은 변경 밖 C07.3 aspect-ratio 직렬화 기대값 차이 한 건으로 267/268이며 이 변경의 성공으로 계산하지 않았다. Simulator 화면의 renderer 영역은 검으므로 glyph paint나 compositor 이후 픽셀 근거가 아니다.
