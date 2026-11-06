# ASCII alphanumeric intrinsic sizing 계획 PR diff 검토 30개

계획 PR의 실제 diff에 대해 서로 다른 계약·입력·근거·문서 경계를 검토했다. 검토 범위는 계획과 사전 비교 자료이며 구현 코드는 변경하지 않는다.

1. **상위 issue 추적 — PASS:** 기존 #1208이 영문 단어만 다룬다는 차이를 계획의 동기로 연결했다.
2. **C11 범위 제한 — PASS:** C11.4/C11 완료를 선언하지 않고 공식 `78/214`를 유지한다.
3. **grammar 단순성 — PASS:** allowlist 변경은 ASCII `[A-Za-z0-9]+` 토큰으로 한정한다.
4. **ASCII byte 검사 가능성 — PASS:** grammar는 Rust ASCII alphanumeric 검사로 직접 구현할 수 있다.
5. **영문 회귀 control — PASS:** 기존 `A A A A A A` 결과를 matrix에 포함했다.
6. **numeric-only positive — PASS:** `12 3 4`가 숫자 token 승인과 min-content를 함께 확인한다.
7. **letter-digit positive — PASS:** `A1 BB2 C33`이 혼합 token을 확인한다.
8. **digit-letter positive — PASS:** `1A 22B 33C`가 digit-leading token 경계를 확인한다.
9. **numeric unbreakable minimum — PASS:** `12345678` auto scene이 40px 이상 automatic minimum을 만든다.
10. **mixed unbreakable minimum — PASS:** `A1B2C3D4`가 문자·숫자 연속 run도 내부 분할되지 않는지 확인한다.
11. **min-content column — PASS:** longest-token advance를 independent track 기대값으로 둔다.
12. **auto column — PASS:** definite 40px container의 min/max-content 관계를 공식화했다.
13. **line-count oracle — PASS:** token 간 whitespace만 줄바꿈 기회로 쓰고 greedy line count를 독립 계산한다.
14. **reversed DOM order — PASS:** min-content와 auto 각각 reverse item-order control이 있다.
15. **implicit row geometry — PASS:** explicit 10px row 뒤 text contribution이 만드는 row list를 기록한다.
16. **Canvas 입력 재현 — PASS:** 10px generic monospace advance와 1/64 CSS px 정량화를 기록했다.
17. **oracle independence — PASS:** expected 계산이 Chrome used track/row를 읽지 않고 source token과 advance에서 계산한다.
18. **Chrome 버전 추적 — PASS:** Agent Browser 0.31.1, HeadlessChrome 157, viewport 및 DPR을 캡처했다.
19. **screenshot 연결 — PASS:** screenshot이 같은 fixture의 colored Grid 화면을 보여준다.
20. **scene traceability — PASS:** observation 및 expected JSON의 scene id가 fixture의 data-case와 일치한다.
21. **source/frame 관측 — PASS:** JSON은 source, track, row, item frame 및 text line rect를 보존한다.
22. **fixture 목적 제한 — PASS:** 자료는 parsed Root HTML 구현의 성공을 주장하지 않고 브라우저 사전 비교다.
23. **punctuation 거부 계약 — PASS:** punctuation은 계획과 scope에서 허용하지 않는다.
24. **Unicode 거부 계약 — PASS:** 비ASCII 문자와 숫자는 허용하지 않는다.
25. **whitespace 경계 — PASS:** SPACE/TAB/LF/CR을 허용하고 NBSP/VT/FF는 거부한다.
26. **style/profile 경계 — PASS:** font, line-height, `word-break`, `overflow-wrap`, owner 제약이 명시됐다.
27. **identity/revision 보호 — PASS:** 원문과 Node identity 보존, failure atomicity를 acceptance에 명시했다.
28. **플랫폼 검증 과장 방지 — PASS:** simulator surface sample을 device/hardware GPU claim으로 확대하지 않는다.
29. **문서 link 일관성 — PASS:** 집중 계획, 부모 C11.4 plan, fixture, observation 및 evidence 간 상대경로가 일치한다.
30. **재현 artifact hash — PASS:** fixture·capture·observation·expected·screenshot SHA-256 manifest가 검증된다.

**결론:** 30개 diff 관점 모두 PASS다. 구현 PR의 code diff는 별도로 검토해야 한다.
