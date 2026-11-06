# C11.4 ASCII 하이픈 단어 구현·런타임 적대 검토 20개

검토 대상은 `[A-Za-z0-9]+(?:-[A-Za-z0-9]+)*` direct-text allowlist, normalized line-break marker, source range 보존, 9-scene style/runtime matrices다. 결과는 수정 전후 focused 테스트와 소스 경로를 대조했다. Android/iOS Simulator 두 실행은 8-box draw/present 및 지정 overflow sample 각 1/1 exact로 통과했다. 화면과 범위는 [실행 근거](./README.md)에 기록했다.

| # | 실패 관점 | 확인 근거 | 결과 |
|---:|---|---|---|
| 1 | 토큰 시작 하이픈을 빼먹어 `-A`를 허용 | grammar unit·style/runtime atomic negative controls | 거부 유지 |
| 2 | 토큰 끝 하이픈을 허용 | `A-` negative controls | 거부 유지 |
| 3 | 빈 segment가 있는 반복 하이픈을 허용 | `A--B`, `A1--B` controls | 거부 유지 |
| 4 | ASCII가 아닌 hyphen/minus를 U+002D처럼 취급 | U+2010·U+2212 controls | 거부 유지 |
| 5 | punctuation 범위를 의도보다 넓혀 slash·underscore를 허용 | `A/B`, `A_B`, punctuation controls | 거부 유지 |
| 6 | 전각·아랍 숫자나 비ASCII 글자를 byte 검사에서 통과 | 기존 full-width·Arabic-Indic·accented controls | 거부 유지 |
| 7 | 기존 ASCII alphanumeric 또는 ASCII whitespace scene을 회귀 | 같은 focused Style-to-layout 12개와 RuntimeSession 16개 | 모두 통과 |
| 8 | source 앞뒤·연속 공백의 원문을 바꿔 source identity를 잃음 | whitespace matrix 및 `input.sources` assertions | 원문·node 유지 |
| 9 | source 순서를 뒤집었을 때 contribution이 잘못된 item에 감 | 독립 9-scene matrix의 reverse-order scene | 기대 frame·row 통과 |
| 10 | min/max/auto/fixed column 모드를 같은 방식으로 계산 | Chrome/Canvas 독립 expected JSON과 Style-to-layout matrix | 9/9 통과 |
| 11 | intrinsic row contribution을 min-content 대신 max-content로 전달 | track-sizing trace assertion | 기대 contribution 통과 |
| 12 | 하이픈 뒤 letter break를 새 marker로 중복 생성 | marker 조건이 다음 byte `digit`인지 검사, `A-A` scenes | 기존 동작 유지 |
| 13 | leading minus `-13`을 number separator로 오인 | Ahem `-13` min-content 및 one-line assertion | 줄 분리하지 않음 |
| 14 | 여러 digit-hyphen 경계에서 앞 marker 삽입이 뒤 byte 위치를 깨뜨림 | descending insertion과 복수 하이픈 scene | matrix 통과 |
| 15 | UTF-8 byte index를 non-ASCII 경계에 적용 | marker eligibility가 source와 normalized text 모두 ASCII인지 검사 | profile 밖 입력에서 비활성 |
| 16 | marker를 DOM/source text에 써서 복사·identity를 오염 | `ParagraphInput.sources[0].text`와 RuntimeSession source assertions | 원문 변경 없음 |
| 17 | marker에 빈 UTF-16 range를 주어 raster validation이 실패 | marker를 authored hyphen range에 합침; runtime render matrix | 기본/fixture 각 16/16 통과 |
| 18 | post-transform 또는 auto-hyphenation과 marker range가 충돌 | text-transform-none gate와 `hyphens:auto` gate; auto fixture 3/3 | 기존 auto 경로 유지 |
| 19 | CSS 기본 `line-break:auto`의 내부 `Strict` 매핑에서 marker가 빠짐 | text-layout test `Loose`, `Normal`, `Strict`; RuntimeSession strict-default path | 모두 동일 wrap |
| 20 | unsupported source를 부분 frame/paint로 게시 | RuntimeSession source rejection atomicity tests | fail-closed 회귀 통과 |

최종 후보의 구현 diff는 별도 [PR diff 검토 30개](./pr-diff-review-30.md)에서 다시 검토한다. Feature-enabled all-targets Clippy는 변경하지 않은 `anonymous_text_hyphens.rs:117`, `anonymous_text_text_transform.rs:507`의 `too_many_arguments` 두 건 때문에 통과하지 않았다. Fixture-feature library-only Clippy는 통과했다.
