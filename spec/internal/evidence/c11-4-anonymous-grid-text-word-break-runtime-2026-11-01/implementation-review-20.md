# C11.4 익명 Grid 텍스트 `word-break` 구현/runtime 실패 관점 20개

대상은 `WordBreakMode`·`ParagraphStyle.locale`, opt-in anonymous-text route, 12개 FFI/runtime scene, Android/iOS fixture adapter와 실행 로그다. 계획 검토 20개 및 계획 PR diff 검토 30개와 별도로 코드를 다시 살폈다. 발견한 호스트 기대값 오류 두 건(폰트 registry revision 고정값, Chrome 양자화 값을 runtime 값으로 사용)은 수정한 뒤 두 Simulator 행렬 전체를 재실행했다.

| # | 독립 실패 관점 | 확인한 경로와 판정 |
| ---: | --- | --- |
| 1 | 임의 scene ID나 비슷한 이름이 fixture allowlist에 들어감 | FFI와 runtime scene catalog가 정확히 12개 ID만 반환하며, `break-word` 변형 요청은 scene으로 등록되지 않는다. unknown anonymous/word-break scene은 준비 단계에서 거부한다. **통과** |
| 2 | `word-break` computed property 누락이 `normal`로 조용히 대체됨 | `validate_text_profile_properties`는 scene별 expected keyword와 정확히 비교하고 missing value도 실패한다. **통과** |
| 3 | authored keyword와 computed serialization이 달라 typed mode가 어긋남 | FFI 고정 style, runtime computed style, paragraph `WordBreakMode`, Android/iOS expected record를 12 ID별 대조했다. mismatched `break-all` Kotlin fixture는 layout publication 전에 거부한다. **통과** |
| 4 | deprecated `break-word`를 `break-all`이나 `normal`로 오인 | FFI generator·runtime catalog가 `break-word` scene을 반환하지 않고 unit test가 미등록을 확인한다. **통과** |
| 5 | `lang` 누락·영어/한국어 혼동 시 전역 또는 기본 언어로 fallback | owner 자체의 HTML `lang` 하나가 scene의 정확한 `en`/`ko`와 다르면 실패한다. missing 및 wrong-language negative cases가 통과했다. **통과** |
| 6 | 바깥 문서 `lang`을 익명 Grid item의 locale 근거로 오인 | 검증 함수가 direct text의 style owner인 Grid node만 읽는다. 고정 runtime source에서도 root에 각 scene의 `lang`을 직접 설정한다. 부모 언어 상속은 지원한다고 선언하지 않는다. **통과** |
| 7 | `xml:lang` 우선순위를 무시하거나 HTML `lang`과 조용히 병합 | XML namespace의 `lang`은 값이 같아도 준비 단계에서 거부하며 negative case로 확인했다. **통과** |
| 8 | locale 태그 파싱 실패가 기본 Locale이나 무언어로 계속 진행 | `ParagraphStyle::validate`와 Parley 변환 양쪽에서 parse 실패를 오류로 돌린다. 빈 문자열·공백·잘못된 복합 태그 거부 및 유효 태그 수용을 단위 테스트했다. **통과** |
| 9 | 기존 paragraph가 새 locale 필드 때문에 예기치 않게 언어가 고정됨 | `ParagraphStyle::default()`은 `None`을 유지하고 기존 fixture literal도 `None`을 준다. 기본값과 invalid-tag 검사를 실행했다. **통과** |
| 10 | `keep-all`이 `normal` 또는 nowrap 대체로 구현됨 | `KeepAll`은 Parley `WordBreak::KeepAll`에 연결한다. Korean `normal` 9.2px/40px과 `keep-all` 36.8px/10px 차이를 고정 Noto bytes로 검사했다. **통과** |
| 11 | `break-all`이 min-content만 줄이고 실제 max-content도 줄임 | Ahem `XXXX` 및 `XX XX`에서 min-content 10px, max-content 40px/50px을 Rust와 두 Simulator의 intrinsic inputs·line output·track으로 비교했다. **통과** |
| 12 | `normal` Ahem의 unbroken word가 잘못 분할됨 | Ahem `XXXX` normal은 min/max 모두 40px, 한 줄이다. Rust와 두 플랫폼에서 확인했다. **통과** |
| 13 | collapsed space가 normal max-content 또는 min-content에서 빠짐 | `XX XX` normal은 min-content 20px, max-content 50px, 행 높이 20px/10px으로 고정해 Rust와 양 플랫폼에서 확인했다. **통과** |
| 14 | Korean `normal`에 Noto 폰트가 아닌 system fallback이 사용됨 | 고정 Noto Sans KR 2.004 bytes를 등록하고 paragraph family와 registry revision 2를 검사한다. 앱/설치물 hash를 기록한 Android·iOS 행렬이 통과했다. **통과** |
| 15 | 한국어 `normal`의 line-break 결과를 locale 효과로 잘못 귀속하거나 locale metadata를 누락 | paragraph trace에 `locale=ko`가 있고, pinned Parley 0.12.0 strict segmenter는 ko를 분할에 사용하지 않는다는 source를 대조했다. locale 있음/없음의 intrinsic geometry는 같고 `keep-all`만 min-content와 행 높이를 바꾼다. **통과** |
| 16 | Korean max-content가 word-break mode에 따라 변하거나 row height가 min-content 값으로 남음 | `normal`과 `keep-all` max-content는 모두 36.8px/10px이다. 각 scene에서 intrinsic height와 final auto row를 비교했다. **통과** |
| 17 | min-/max-content 장면에서 잘못된 track sizing keyword 사용 | runtime validator가 고정 scene의 정확한 `MinContent`/`MaxContent` track을 요구하고, 다른 길이 track 대조군을 거부한다. 앱 trace는 최종 Columns/Rows 크기를 검사한다. **통과** |
| 18 | line advance는 맞지만 줄 수·line-height·line position이 어긋남 | 12개 scene의 line count/advance, line-height, finite baseline/top을 Rust trace 및 simulator capture에서 검증했다. **통과** |
| 19 | Chrome 1/64px 직렬화 값을 Parley/Taffy runtime 기대값으로 강제 | oracle 기록의 9.203125/36.8125px과 독립 글꼴 계산·runtime 9.2/36.8px을 별도 필드와 문서로 분리하고, 양자화 tolerance만 적용한다. **통과** |
| 20 | stale HostDocument revision이나 다른 node의 결과를 해당 장면 성공으로 보고 | runtime test와 앱 capture가 root/item identity, HostDocument generation/document revision, summary revision 일치를 검사한다. 각 scene은 새 run ID와 revision trace를 보존했다. **통과** |

확인 범위는 12개 고정 scene의 text layout/intrinsic sizing, Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator다. 캡처는 glyph painting·실기기·일반 CSS admission·모든 언어/line-break 조합을 증명하지 않는다. [런타임 행렬과 로그](./README.md).
