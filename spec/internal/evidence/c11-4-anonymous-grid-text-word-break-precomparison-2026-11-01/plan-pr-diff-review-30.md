# C11.4 익명 Grid 텍스트 computed `word-break` 계획 PR 변경 검토 30개

계획 PR의 최종 변경 묶음을 계획 검토표와 구분해 30개 독립 관점으로 확인했다. 대상은 구현 계획, 계획 20개 검토표, HTML oracle fixture, Chrome 관측 JSON, 캡처 및 이 사전 비교 README다. runtime 구현이나 플랫폼 통과를 뜻하지 않는다.

| # | PR 변경 실패 관점 | 확인 결과 | 판정 |
| ---: | --- | --- | --- |
| 1 | 이미 병합된 anonymous text slice를 다시 구현하도록 계획 | `origin/main`의 anonymous adapter는 `normal`만 허용하고 word-break 확장 PR은 없음을 재확인했다. | 통과 |
| 2 | 열린 PR과 새 계획이 중복 | 현재 열린 PR 목록은 비어 있다. | 통과 |
| 3 | 작은 fixture를 C11.4 완료로 과장 | 계획은 C11.4/C11 미완료와 공식 `72/213`을 명시한다. | 통과 |
| 4 | 계획 변경으로 내부 계약 버전 상승 | 계약 0059의 숫자 버전 `0.1.0`을 유지한다. | 통과 |
| 5 | anonymous direct text의 style source가 불명확 | 상속 style owner인 같은 Grid container의 computed 값으로 한정했다. | 통과 |
| 6 | HTML document `lang`을 Grid owner 언어로 오인 | 문서 `lang=en`, scene Grid owner `lang=en|ko`를 구분했다. | 통과 |
| 7 | `xml:lang` 우선순위가 HTML `lang`과 충돌 | Korean owner의 XML lang 존재·충돌을 pre-publication 거부 조건으로 적었다. | 통과 |
| 8 | 지원 언어 범위가 BCP 47 전체로 확대 | 고정 profile은 정확한 `lang=ko`만 지원한다. | 통과 |
| 9 | computed style이 아닌 authored 값만 전달 | 동일 owner/revision의 computed serialization과 typed `WordBreakMode`를 맞춰 검사한다. | 통과 |
| 10 | `break-word`를 `normal`로 대체하여 semantics 손실 | deprecated 값은 거부하고 `overflow-wrap:anywhere` 결합은 범위 밖으로 둔다. | 통과 |
| 11 | `overflow-wrap:anywhere`가 min-content oracle에 혼입 | fixture는 `overflow-wrap:normal`을 고정하고 다른 값은 거부한다. | 통과 |
| 12 | line-break 차이를 word-break 지원으로 오인 | writing mode·방향·line-break·whitespace 입력을 제한하고 문장부호 없는 네 Hangul syllable만 locale 비교에 사용한다. | 통과 |
| 13 | CSS 기본값 변화에 기대값이 잠김 | fixture에서 `line-break:auto`를 명시해 입력을 고정한다. | 통과 |
| 14 | Ahem 또는 OS fallback이 oracle 글꼴을 대체 | CSS font load/check 및 Ahem SHA-256을 근거에 남겼고 앱 입력은 고정 bytes로 계획했다. | 통과 |
| 15 | Noto variable font 축·버전이 맞지 않음 | Noto Sans KR 2.004 hash, upstream commit, 400 weight와 glyph advance 근거를 연결했다. | 통과 |
| 16 | 폰트 파일을 새로 복사해 중복·라이선스 위험 발생 | 저장소 C15.3에 이미 고정된 font fixture를 재사용한다. | 통과 |
| 17 | Ahem `XXXX` min-content 계산 오류 | 독립 기대값 40px(`normal`) 및 10px(`break-all`)이 advance와 break opportunity에 맞다. | 통과 |
| 18 | 두 단어 사이 공백이 intrinsic width에서 누락 | `XX XX` min-content 20px, max-content 50px 산식과 관측이 일치한다. | 통과 |
| 19 | `break-all`이 max-content를 잘못 축소 | 두 Ahem 문자열의 max-content는 각 전체 advance 40px/50px로 유지한다. | 통과 |
| 20 | Korean `normal`에서 locale 없음을 정상처럼 취급 | owner `lang=ko`와 Parley Locale 전달을 요구하고 normal min-content 9.2px을 고정한다. | 통과 |
| 21 | `keep-all`을 nowrap fallback으로 구현 | min-content 36.8px·row 10px과 normal의 9.2px·row 40px을 분리한다. | 통과 |
| 22 | max-content를 mode별로 잘못 변경 | 한국어 두 mode 모두 36.8px/10px 기대값을 둔다. | 통과 |
| 23 | Chrome 양자화를 엔진 허용오차에 합침 | font-table 기대값, Chrome 1/64px 값, runtime 0.05px 허용값을 분리한다. | 통과 |
| 24 | 관측 JSON에서 입력과 결과를 재구성할 수 없음 | UA, DPR, font load, 각 scene의 computed keyword·text·locale·track·row·font가 기록되어 있다. | 통과 |
| 25 | 12개 scene 누락 또는 중복 | JSON의 관측 수는 12이며 두 font, 세 word-break 값과 min/max track 조합을 README 표와 맞췄다. | 통과 |
| 26 | 스크린샷이 잘렸거나 대상 장면을 식별하지 못함 | PNG는 브라우저에서 확인한 12개 장면 전체 캡처이며 PR 본문에 실제 이미지로 연결하도록 계획에 적었다. | 통과 |
| 27 | 일반 CSS admission을 fixture-only adapter 수정과 혼동 | 계획은 opt-in anonymous route에만 scene allowlist를 추가하고 일반 author API를 제외한다. | 통과 |
| 28 | fixture 코드·대형 Noto payload가 제품 artifact에 포함 | 계획은 기본 artifact negative와 fixture-test positive control을 별도로 요구한다. | 통과 |
| 29 | Simulator screenshot이 glyph paint나 compositor 이후 픽셀을 증명한다고 오해 | track/frame/trace 범위로 제한하고 검정 renderer와 iOS drawable 한계를 명시한다. | 통과 |
| 30 | 계획 PR 병합만으로 구현 완료·로드맵 승격 | 계획 PR과 구현 PR을 분리하고 구현 검증 후에도 C11.4 checkbox와 `72/213`을 유지한다. | 통과 |

이 검토는 변경 파일의 구조·범위·근거 연결을 대상으로 했다. 구현 후에는 별도 runtime 실패 관점 20개, 구현 PR에는 새 PR diff 실패 관점 30개가 필요하다.
