# ASCII alphanumeric intrinsic sizing 구현/runtime 실패 관점 검토 20개

범위는 grammar allowlist, Style-to-layout/RuntimeSession matrix, Android API 37 AVD와 iOS Simulator의 고정 numeric app scene이다.

1. **ASCII only — PASS:** `is_ascii_alphanumeric()`은 Unicode letters/digits를 허용하지 않는다.
2. **Empty input — PASS:** split 뒤 빈 token을 건너뛰지만 `has_token`이 없어 empty/whitespace-only input을 거부한다.
3. **Digit-only token — PASS:** `12`, `12345678`을 positive matrix와 simulator scene에 둔다.
4. **Letter-digit token — PASS:** `A1`, `BB2`, `A1B2C3D4`를 검사한다.
5. **Digit-letter token — PASS:** `1A`, `22B`, `33C`를 검사한다.
6. **Punctuation exclusion — PASS:** `A-B`, `A1-B`, `B-1`, `1.2`, underscore 등은 거부한다.
7. **Non-ASCII rejection — PASS:** full-width Latin, full-width/Arabic-Indic digits 및 Unicode letter/space controls가 계속 거부된다.
8. **Whitespace contract — PASS:** SPACE/TAB/LF/CR 구분자를 보존하고 form-feed/vertical-tab/NBSP/NEL/em-space를 거부한다.
9. **Source preservation — PASS:** RuntimeSession은 단일 Text source의 exact string을 independent case source와 대조한다.
10. **Node ownership — PASS:** owner box, Text node identity, item source order를 각 case에서 검사한다.
11. **Min-content track — PASS:** widest token의 independent advance를 Chrome 157 expected track과 비교한다.
12. **Auto minimum — PASS:** 40px definite container에서 8-character numeric·mixed token이 48.171875px track을 만든다.
13. **Normal wrapping — PASS:** token 사이 greedy line count와 실제 line count/height를 대조한다.
14. **Implicit rows — PASS:** track trace의 used rows와 long/short item min-content contribution을 비교한다.
15. **Reverse DOM order — PASS:** 두 column mode에서 item order reversal 뒤 row/paint ownership을 검사한다.
16. **Revision coherence — PASS:** cascade/layout key와 text paint document/tree/style revision이 일치해야 한다.
17. **Unsupported style — PASS:** 기존 wrong font, nowrap, fixed size 및 unsupported track controls가 계속 fail-closed한다.
18. **Failure atomicity — PASS:** punctuation·구조 거부 시 partial layout/paint snapshot을 publish하지 않는 회귀 테스트를 유지한다.
19. **Platform route — PASS:** Android/iOS default Root HTML route는 numeric source DOM, ready marker, 8 boxes, two text draws를 확인한다.
20. **Artifact/pixel boundaries — PASS:** Rust release rlib에는 fixture marker가 없고 fixture-test positive control에는 있다. 두 simulator의 지정 표본만 exact이며 전체 geometry, physical GPU, compositor-after pixel로 확대하지 않는다.

**결론:** 구현/runtime 20개 관점에서 확인한 범위 내 추가 실패를 찾지 못했다. 실제 PR diff 검토는 별도로 수행한다.
