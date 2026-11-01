# C11.4 anonymous Grid `line-break: loose` 계획 PR diff 실패 관점 검토 30개

최종 검토 대상은 계획 문서, 사전 비교 근거, Chrome 입력 fixture, font screenshot 및 계획 검토 기록이다. 이 기록 파일은 검토 후 추가한 audit 산출물이며, 아래 30개는 나머지 변경 patch를 대상으로 한다.

1. 계획 제목과 대상 C11.4 범위가 일치한다. **PASS**
2. 상위 C11·C11.4·internal contract 링크의 상대 경로가 저장소 구조와 맞다. **PASS**
3. CSS Text Level 3의 `line-break` 공식 참조는 속성 섹션으로 연결된다. **PASS**
4. UAX #14 링크는 현재 규범 문서를 가리킨다. **PASS**
5. 계획은 `Loose` enum 추가와 Parley `Loose` mapping을 정확히 분리해 요구한다. **PASS**
6. 기존 `Normal`, `Strict`, default와 `auto` fallback 불변 조건이 기록됐다. **PASS**
7. 입력 U+FF04 code point가 HTML·JSON·독립 기대에서 일치한다. **PASS**
8. `文＄文`의 expected code point 배열이 source string과 맞다. **PASS**
9. font path는 저장소 안 고정 fixture 파일을 가리키며 U+6587/U+FF04의 `hmtx` advances는 독립 조회 결과와 맞다. **PASS**
10. font hash가 로컬 bytes에서 계산한 SHA-256과 일치한다. **PASS**
11. Chrome 관측의 font-loaded flag와 font bytes 경계가 기록됐다. **PASS**
12. HTML `@font-face` 상대 URL이 fixture 파일 위치에서 font resource를 찾는다. **PASS**
13. HTML root/body reset이 default margin·padding 영향을 제거한다. **PASS**
14. `.case`의 10px font/line-height와 zero box spacing이 JSON 환경과 일치한다. **PASS**
15. scene class 이름과 `lang`/mode/track 조합이 expected JSON case IDs와 맞다. **PASS**
16. 네 장면 모두 exact direct text source를 동일하게 사용한다. **PASS**
17. `ja loose min-content` used column 10px와 row 30px이 Chrome observation과 일치한다. **PASS**
18. `ja loose max-content` used column 30px와 row 10px이 Chrome observation과 일치한다. **PASS**
19. `en loose` locale control이 20px/20px으로 Japanese tailoring을 분리한다. **PASS**
20. `ja normal` control이 20px/20px으로 computed mode 차이를 분리한다. **PASS**
21. independent expectations는 CSS Text/UAX14 규칙과 glyph advances로 파생되며 Chrome 값을 규범이라고 취급하지 않는다. **PASS**
22. screenshot 해시가 repository에 포함한 PNG와 재계산 값으로 일치한다. **PASS**
23. screenshot 캡션은 Chrome oracle만 표시하며 Spinon 실행 화면이라고 주장하지 않는다. **PASS**
24. precomparison README의 evidence 링크가 존재하는 파일을 가리킨다. **PASS**
25. plan에서 precomparison README·JSON·PNG로 가는 링크가 올바르다. **PASS**
26. 계획은 exact style owner와 document/style revision 검증을 acceptance에 포함한다. **PASS**
27. mismatch 입력은 publication 이전에 fail-closed하도록 요구한다. **PASS**
28. Android AVD와 iOS Simulator를 각각 요구하고 physical device/hardware GPU로 확대하지 않는다. **PASS**
29. product artifact와 fixture-test positive control의 경계를 따로 검사한다. **PASS**
30. 계획·runtime·PR review 단계와 official `72/213` 유지 경계가 분리돼 있다. **PASS**

결과: **30 PASS / 0 PARTIAL / 0 FAIL**. 수정이 필요한 계획·fixture·evidence link는 검토에서 발견되지 않았다.
