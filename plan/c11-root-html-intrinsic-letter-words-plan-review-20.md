# C11.4 Root HTML ASCII word intrinsic sizing 계획 실패 관점 검토 20개

**대상:** [구현 전 계획](./c11-root-html-intrinsic-letter-words.md) 및 [Chrome 사전 비교](../spec/internal/evidence/c11-root-html-intrinsic-letter-words-precomparison-2026-11-06/README.md)

**범위:** plan PR 이전 검토. 구현 코드·Simulator 동작·C11.4 전체 완료는 판정하지 않는다.

| # | 서로 다른 실패 경로 | 검토 결과와 판정 기준 | 결과 |
|---:|---|---|---|
| 1 | Unicode alphabetic 문자가 Rust `is_alphabetic`으로 실수로 허용된다 | 계약을 ASCII `[A-Za-z]+`로 고정하고 Unicode를 negative control로 둔다 | 통과 |
| 2 | 숫자나 punctuation이 line-break 규칙 확인 없이 새로 허용된다 | `[A-Za-z]+` 외 token은 계속 fail-closed한다 | 통과 |
| 3 | `lang`에 따른 line-break variation이 기대값을 뒤집는다 | fixture `lang="en"`과 `word-break:normal`을 고정한다 | 통과 |
| 4 | 단어 내부를 임의로 분리해 min-content 폭이 줄어든다 | normal whitespace의 공백 경계만 break opportunity로 사용하고 token은 unbreakable로 둔다 | 통과 |
| 5 | CSS `normal`의 연속 whitespace가 둘 이상의 폭으로 계산된다 | SPACE/TAB/LF/CR를 허용하며 브라우저와 독립 계산 모두 collapsed line-break semantics를 사용한다 | 통과 |
| 6 | CR·LF source preprocessing이 source-preservation 계약과 충돌한다 | fixture는 브라우저 DOM이 만든 text node를 관찰하고 runtime은 원문 source identity를 유지한다 | 통과 |
| 7 | 앞뒤 collapsible whitespace가 token grammar에서 유실된다 | 허용 범위에 source 앞뒤 whitespace를 넣고 rewrite 금지를 명시한다 | 통과 |
| 8 | 빈 문자열이 `all()` 기반 검사로 허용된다 | empty/whitespace-only source를 명시적으로 거부하고 기존 gate를 보존한다 | 통과 |
| 9 | `min-content` track 기대값이 전체 문장 폭으로 잘못 계산된다 | 최대 unbreakable token advance에서 독립적으로 산출한다 | 통과 |
| 10 | `auto` track의 max-content 성장과 available-size 분배를 혼동한다 | 기대식 `max(min-content, min(40px, max-content))`로 고쳤고 네 auto source가 max-content 40px 이상인지 산출한다 | 통과 |
| 11 | overwide token이 container에서 clamp되어 border overflow가 숨는다 | `AAAAAAAA`의 48.171875px track 및 overflow를 positive control로 둔다 | 통과 |
| 12 | CSS font advance와 Grid 1/64px quantization의 rounding이 다르다 | pinned generic monospace word/space advance를 측정하고 ceil quantization 후 used tracks와 독립 비교한다 | 통과 |
| 13 | 서로 다른 token 내용의 글자 폭을 한 개 baseline으로 대체한다 | `AA`, `AB`, `BBB`, `aa`, `ccc`별 canvas advance를 저장한다 | 통과 |
| 14 | token 조합에서 whitespace가 max-content 계산에 누락된다 | max-content는 각 word advance와 사이의 collapsed space advance 합으로 계산한다 | 통과 |
| 15 | 긴/짧은 text item 역순에서 row contribution이 원래 DOM 순서를 잃는다 | min-content와 auto mode 각각에 reverse-order case를 추가한다 | 통과 |
| 16 | 줄 수만 맞고 text item의 row 위치·높이가 틀린다 | expected data가 rows와 root-relative item frames를 별도로 비교한다 | 통과 |
| 17 | item `min-height`, line-height 또는 width가 브라우저 oracle에 고정되지 않는다 | fixture는 10px line-height와 auto dimensions, visible overflow를 고정한다 | 통과 |
| 18 | 실제 Android/iOS generic monospace metrics가 Chrome과 달라 parity가 오인된다 | 두 Simulator에서 동일 source의 geometry를 비교하고 허용오차를 넘으면 성공으로 기록하지 않는다 | 통과 |
| 19 | text paint가 viewport 또는 grid edge에서 잘려도 geometry만 보고 성공한다 | border 바깥의 grid-relative PixelCopy/Metal sample을 acceptance에 넣는다 | 통과 |
| 20 | opt-in fixture symbol·payload가 기본 artifact로 누출되거나 partial frame이 게시된다 | feature-test boundary 비교와 source/style/layout revision 및 rejection atomicity를 acceptance에 넣는다 | 통과 |

**검토에서 반영한 수정:** 첫 계산 초안은 `auto` track을 `max(40px,min-content)`로 단순화하고 source order를 한 방향만 포함했다. max-content/available-size를 반영한 식으로 바꾸고 양쪽 track mode의 역순 case를 추가했다. 이후 10/10 독립 계산이 Chrome used track/row·line count와 일치했다.

**결과:** 계획 gate 통과. 이 표는 구현/runtime 20개 검토 또는 plan PR diff 검토를 대신하지 않는다.
