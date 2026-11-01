# C11.4 anonymous Grid `line-break: loose` 계획 실패 관점 검토 20개

검토 대상은 [`c11-4-anonymous-grid-text-line-break-loose.md`](./c11-4-anonymous-grid-text-line-break-loose.md), Chrome 관측 JSON, CSS Text 기반 독립 기대 JSON 및 고정 HTML 입력이다. 각 항목은 구현 이전에 별개의 계획 실패 가능성을 대조했다.

1. **규범 키워드 혼동:** CSS Text의 `loose` 규칙은 `normal`과 별도다. U+FF04 뒤 일본어 break를 구분하는 값과 control을 명시했다. **PASS**
2. **locale tailoring 누락:** `ja loose`만을 긍정으로 두지 않고 `en loose` 음성 control을 둬 writing-system 조건을 확인한다. **PASS**
3. **mode control 부족:** `ja normal`을 넣어 locale은 같고 computed keyword만 다른 control을 포함했다. **PASS**
4. **max-content 오해:** `max-content`는 모든 break opportunity를 사용하지 않아 30px/10px이 되어야 한다. 별도 scene으로 고정했다. **PASS**
5. **문자점 혼동:** 대상 문자는 ASCII `$`가 아니라 fullwidth U+FF04다. 계획·fixture·기대 JSON이 모두 `文＄文`을 사용한다. **PASS**
6. **잘못된 규범 분류:** U+FF04는 East Asian Fullwidth `PR` prefix다. UAX #14 LB25의 `PR × ID`를 유지한 뒤 CSS Text loose의 일본어 tailoring이 뒤 break를 추가한다. **PASS**
7. **source string 순서 오류:** 기대 분할은 `[文][＄][文]`; JSON의 문자 위치와 10px advance가 이를 나타낸다. **PASS**
8. **font fallback:** font bytes hash와 browser `document.fonts.check()` 결과를 기록했다. 시스템 fallback 관측을 기준값으로 사용하지 않는다. **PASS**
9. **advance 독립성:** 10px glyph advance와 30px max-content 합계를 Chrome의 used track 관찰과 분리해 기록했다. **PASS**
10. **line-height 누락:** 10px line-height와 자동 row 결과를 함께 고정해 min-content wrapping이 행 높이에 반영되는지 판별한다. **PASS**
11. **Grid 사용값과 text 결과 혼동:** min/max-content column 값과 root auto-row 높이를 별도 관찰한다. **PASS**
12. **가용 폭 간섭:** Grid root는 120px이지만 열만 intrinsic sizing이며 border/padding/gap이 0이다. 외부 폭이 min-content를 제한하지 않는다. **PASS**
13. **source-order 단위 오차:** 세 글자는 BMP 단일 UTF-16 code unit이라 Range index가 문자 범위와 일치한다. 코드 포인트 배열도 기대 JSON에 고정했다. **PASS**
14. **언어 상속 가정:** 네 장면은 Grid style owner에 직접 `lang`을 둔다. 이 계획이 일반 ancestor language inheritance를 주장하지 않는다. **PASS**
15. **기존 `auto` fallback 변경:** 계획은 새 고정 scene 외 `auto`의 strict mapping을 바꾸지 않는다고 명시한다. **PASS**
16. **기존 normal/strict 회귀:** 이전 line-break scene matrix와 focused tests를 구현 시 회귀 대상으로 지정했다. **PASS**
17. **unknown value 부분 출력:** mismatched `auto|normal|strict|anywhere`, unknown scene 및 stale revision을 layout publication 전에 거부하도록 명시했다. **PASS**
18. **다른 text style 간섭:** word-break, white-space, font, weight, indent, writing mode를 고정하고 mismatch rejection을 acceptance에 포함했다. **PASS**
19. **fixture/profile 과대 주장:** 일반 CSS admission, WPT, arbitrary language/font, glyph pixels, physical devices와 전체 C11.4/C11을 명시적으로 제외한다. **PASS**
20. **artifact/roadmap 승격:** default/product vs fixture-test static archive 경계를 요구하며 공식 `72/213`과 C11.4/C11 checkbox를 유지한다. **PASS**

결과: **20 PASS / 0 PARTIAL / 0 FAIL**. 차단되는 계획 결함은 없으며 기대값·fixture·거부 경계를 모두 계획 본문에 연결했다.
