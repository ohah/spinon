# ASCII alphanumeric intrinsic sizing 계획 실패 관점 검토 20개

계획 PR 전 독립 실패 경로와 계약 경계를 검토했다. 아래는 계획 검토이며 코드 구현이나 runtime pass를 뜻하지 않는다.

1. **기존 구현의 결손과 범위 — PASS:** #1208 alphabetic allowlist에 숫자를 더하는 bounded gap으로 특정했다.
2. **ASCII 명시성 — PASS:** Unicode digit이나 locale-dependent character를 포함하지 않고 `[A-Za-z0-9]+`로 고정했다.
3. **token 내부 break — PASS:** 숫자·문자가 섞인 token은 이 profile에서 unbreakable word로 다룬다.
4. **숫자 단독 입력 — PASS:** `12 3 4`가 grammar와 min-content matrix에 있다.
5. **문자 뒤 숫자 — PASS:** `A1`, `BB2` 및 혼합 auto fixture로 확인한다.
6. **숫자 뒤 문자 — PASS:** `1A`, `22B`, `33C`를 별도 source case로 둔다.
7. **긴 numeric automatic minimum — PASS:** 40px container를 넘는 `12345678`을 auto case로 둔다.
8. **긴 혼합 automatic minimum — PASS:** `A1B2C3D4`로 letter/digit composition을 확인한다.
9. **min-content와 auto의 구분 — PASS:** 양쪽 column mode를 fixture에 넣고 independent formula를 기록했다.
10. **normal wrapping의 source token 경계 — PASS:** line count는 whitespace-separated source tokens에서 계산한다.
11. **whitespace preservation — PASS:** 허용 separator와 source 보존 요구를 명시하고 normalization을 금지했다.
12. **비허용 whitespace — PASS:** NBSP, vertical tab, form feed는 fail-closed 요구로 남겼다.
13. **빈 입력 — PASS:** empty/whitespace-only source는 거부 범위에 둔다.
14. **punctuation 경계 — PASS:** 하이픈, underscore, decimal punctuation은 이 변경에서 수용하지 않는다.
15. **Unicode 경계 — PASS:** full-width 및 기타 비ASCII 문자·숫자는 범위 밖이다.
16. **style/owner 경계 — PASS:** 다른 font/style/node ownership, hidden/detached node는 계속 거부한다.
17. **source identity — PASS:** exact Text node identity와 source bytes 보존을 acceptance에 둔다.
18. **revision atomicity — PASS:** failed projection이 이전 frame이나 partial draw를 새 결과로 publish하지 않도록 명시했다.
19. **플랫폼 증거 경계 — PASS:** Android/iOS simulator와 sampled surface 결과를 실기기·hardware GPU로 확대하지 않는다.
20. **완료 집계 경계 — PASS:** C11.4/C11 checkbox와 `78/214` 공식 count를 유지한다.

**결론:** 20개 계획 관점에서 구현 전에 고정해야 할 누락은 발견하지 못했다. 실제 PR diff 검토와 구현/runtime review는 후속 단계다.
