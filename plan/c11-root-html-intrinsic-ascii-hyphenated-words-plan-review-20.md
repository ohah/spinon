# 하이픈 단어 intrinsic sizing 계획 적대 검토 20개

대상: [계획](./c11-root-html-intrinsic-ascii-hyphenated-words.md)과 [Chrome 사전 비교](../spec/internal/evidence/c11-root-html-intrinsic-ascii-hyphenated-precomparison-2026-11-06/README.md). 각 관점은 서로 다른 반례나 계약 경계를 검토했다.

| # | 실패 관점 | 판정과 계획의 대응 |
|---:|---|---|
| 1 | `-A`가 숫자/부호처럼 보이는 leading minus여서 break 가능성을 잘못 허용 | grammar가 leading hyphen을 거부하고 negative control에 추가한다. |
| 2 | `A-`에서 끝 hyphen을 잘못된 불완전 token으로 수용 | 마지막 non-empty ASCII run을 강제하고 trailing-hyphen 거부를 고정한다. |
| 3 | `A--B`에서 빈 segment를 건너뛰어 반복 punctuation을 수용 | 각 `-` 사이에 non-empty run을 요구한다. |
| 4 | U+2010 HYPHEN과 U+002D를 동등 처리해 승인 범위가 팽창 | U+002D만 허용하고 U+2010 negative control을 둔다. |
| 5 | U+2212 MINUS SIGN을 ASCII hyphen으로 정규화 | code point 정규화 없이 source 그대로 검사하며 거부한다. |
| 6 | `A_B`·`A/B` 같은 punctuation이 token grammar를 우회 | ASCII alphanumeric 외 문자를 계속 거부하고 별도 controls를 둔다. |
| 7 | `A-1`의 break 전에 min-content를 `A`만으로 낮춰 하이픈 advance를 잃음 | Chrome/Canvas oracle에 `12-3`, `A1-BB2`와 break segment별 독립 폭을 고정한다. |
| 8 | hyphen을 다음 line으로 옮겨 `A-`/`A`가 아닌 `A`/`-A`가 됨 | line count와 line break 위치를 9-scene Chrome observation과 비교한다. |
| 9 | max-content에서 hyphen을 제거하거나 두 segment로 계산 | `A-A` max-content scene의 full unwrapped width를 acceptance에 포함한다. |
| 10 | fixed narrow track에서 Grid min-content와 line wrapping이 혼동 | fixed 20px control은 track을 고정하고 text lines/rows를 별도 비교한다. |
| 11 | auto track의 available-size 공식이 min/max contribution과 섞임 | `auto` scene의 40px definite axis와 independently computed min/max를 고정한다. |
| 12 | 여러 hyphenated token 사이의 collapsible whitespace가 source와 다르게 적용 | leading/trailing/중간 SPACE 및 기존 TAB/LF/CR profile을 유지하고 source exact를 검사한다. |
| 13 | reverse DOM order에서 contribution이 source-order item에 잘못 부착 | 역순 scene의 item frame·trace contribution owner를 확인한다. |
| 14 | ASCII 숫자와 영문 혼합 token의 방향별 사례만 통과 | `12-3`와 `A1-BB2`를 모두 포함해 앞/뒤 segment 유형을 덮는다. |
| 15 | 길고 끊기지 않는 마지막 segment의 auto size/overflow가 과소 산정 | `12345678-12` auto scene에서 폭, wrapping, final row를 비교한다. |
| 16 | text source를 분리하거나 재작성해 Text node identity/UTF-16 offsets 손상 | acceptance가 exact source string, same Text node handle, revision identity를 요구한다. |
| 17 | HTML entity 해석 후 U+002D와 유사 문자가 합쳐져 allowlist를 우회 | parsed DOM의 실제 source text code point로 gate하고 dash lookalike를 거부한다. |
| 18 | negative control rejection 뒤 partial layout 또는 이전 revision의 frame이 섞임 | 실패 시 atomic publication 부재와 revision key 불일치를 확인한다. |
| 19 | Simulator 화면을 glyph pixel parity나 실기기 근거로 과장 | 계획은 draw/present smoke와 PixelCopy/Metal sample의 주장 경계를 분리한다. |
| 20 | 계획 slice를 C11.4/C11 완료 또는 공식 지원으로 승격 | 체크박스·`78/214`·계약 버전을 유지하고 plan-only 상태를 명시한다. |

검토 결과: 범위·oracle·거부 경계·runtime acceptance가 이 후보를 분리해 검증하기에 충분하다. 구현 전 Chrome 기준과 독립 계산은 [고정 evidence](../spec/internal/evidence/c11-root-html-intrinsic-ascii-hyphenated-precomparison-2026-11-06/README.md)에 있다.
