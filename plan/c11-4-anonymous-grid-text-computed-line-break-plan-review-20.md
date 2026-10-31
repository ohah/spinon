# C11.4 computed `line-break` 계획 실패 관점 검토 20개

**검토 대상:** `c11-4-anonymous-grid-text-computed-line-break.md` 및 [Chrome 사전 비교](../spec/internal/evidence/c11-4-anonymous-grid-text-line-break-precomparison-2026-11-01/README.md)

이 문서는 구현 전 계획 자체를 검토한다. Spinon 구현/시뮬레이터 실행을 검증한 문서가 아니다.

1. **CSS 값 구분:** `normal`과 `strict`가 CSS Text 3의 U+301C 규칙상 다른 일본어 break 기회를 만드는가? **PASS** — §5.2는 normal 일본어에 break를 허용하고 strict는 허용하지 않는 규범 경계를 쓰며 Chrome 156 측정이 이를 보인다.
2. **언어 조건:** locale이 없이도 `normal` 값만으로 같은 결과를 내는가? **PASS** — 독립 기대값과 Chrome 관측에 `lang=en; normal` 20px negative control이 포함된다.
3. **positive language:** 일본어 장면의 `lang=ja`가 `normal` tailoring을 활성화하는가? **PASS** — style owner의 직접 HTML lang과 결과를 같은 scene record에 묶는다.
4. **엄격 모드 독립성:** `strict` 결과가 locale의 `normal` tailoring에 우연히 의존하는가? **PASS** — 계획은 동일한 `ja` 언어에서 normal/strict를 비교하고 Parley의 typed mode를 별도로 검증한다.
5. **Computed 경계:** authored declaration 문자열을 바로 text engine에 넘기지는 않는가? **PASS** — 요구 입력은 revision-bound computed-style snapshot의 `line-break`다.
6. **익명 item 소유자:** 직접 Text child가 어느 요소의 스타일을 상속하는지 모호한가? **PASS** — 해당 Grid root가 유일한 style/language owner로 고정되고 자식 element는 거부된다.
7. **언어 상속 과장:** ancestor lang 탐색을 새로 구현하거나 HTML 일반 상속을 지원한다고 주장하는가? **PASS** — fixture는 owner의 직접 `lang`만 받으며 일반 language inheritance를 범위에 넣지 않는다.
8. **XML/누락 입력:** XML lang, 비어 있는 lang, missing lang에서 locale fallback이 생길 수 있는가? **PASS** — 각 입력을 명시적으로 거부한다.
9. **문자열 안정성:** 글꼴이나 다른 문장으로 결과가 달라질 수 있는가? **PASS** — `文〜文`을 고정하고 text/font mismatch를 scene validation 거부로 지정한다.
10. **폰트 재현성:** 시스템 fallback이 기대 폭을 바꿀 수 있는가? **PASS** — hash가 고정된 Noto Sans KR을 fixture-only로 등록하고 browser load와 cmap을 확인한다.
11. **변수 글꼴 축:** 기본 `wght=100` hmtx만 보고 weight 400 폭을 잘못 계산하는가? **PASS** — weight 400으로 fontTools instancing한 값이 양 glyph 모두 1000/1000 em이고 Chrome의 10px 결과와 일치한다.
12. **Min-content 정의:** track 기대값이 CSS intrinsic sizing과 일치하는가? **PASS** — normal Japanese는 10px 단위 세 segment, strict/English는 가장 긴 두 glyph segment 20px로 계산한다.
13. **Max-content 대조:** line-break 변화가 max-content text width까지 바꿔야 한다고 잘못 가정하는가? **PASS** — 네 scene 중 max-content 둘은 30px/10px control로 고정한다.
14. **Grid row sizing:** 줄 수 변화와 auto row 사이 관계가 입력 폭과 무관한가? **PASS** — runtime fixture 폭 120px, 10px line-height, auto row 및 computed row track을 명시한다.
15. **Runtime input 동등성:** precomparison이 anonymous fixture와 다른 shrink-to-fit 모델만 비교하는가? **PASS** — runtime-style Chrome 캡처를 width 120px, auto row, one direct text node로 따로 기록한다.
16. **Typed 경로:** 이미 있는 `LineBreakMode`나 Parley locale 경로를 중복 추가하는가? **PASS** — 계획은 `Normal|Strict`, optional `ParagraphStyle.locale`, Parley 0.12.0을 재사용하고 일반 타입 확장은 요구하지 않는다.
17. **속성 조합 폭발:** word-break, white-space, direction 등 다른 mode까지 암묵적으로 지원하는가? **PASS** — normal word-break, normal wrapping, horizontal-tb/LTR, spacing zero에 고정한다.
18. **미지원 값 누수:** `loose`, `anywhere`, `auto` 또는 누락 값이 새 scene에서 임의 mapping되는가? **PASS** — 기존 auto 장면을 유지하고 새 scene은 exact normal/strict만 받게 한다.
19. **원자적 실패:** stale revision, 잘못된 owner/value 또는 unknown scene에서 부분 frame/draw가 나올 수 있는가? **PASS** — publication 이전 fail-closed와 mutation matrix를 요구한다.
20. **상태/검증 과장:** 다섯 장면의 simulator 결과를 C11.4나 CSS/WPT 완료로 확대할 수 있는가? **PASS** — default/fixture artifact 분리, Android/iOS Simulator 경계, PR별 별도 검토와 C11/72/213 미승격을 완료 기준에 고정한다.

## 판정

20개 계획 실패 관점을 모두 통과했다. 이는 계획 review 결과이며 계획 PR diff review나 이후 구현/runtime review를 대신하지 않는다.
