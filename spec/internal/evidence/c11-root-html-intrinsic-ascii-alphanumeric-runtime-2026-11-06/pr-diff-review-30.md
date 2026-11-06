# C11.4 ASCII alphanumeric intrinsic sizing PR diff 실패 관점 검토 30개

검토 대상은 PR [#1212](https://github.com/ohah/spinon/pull/1212)의 head `132c3a42c80b687091b760abccbd1523c3c73402`, 44개 변경 파일이다. 실제 GitHub PR diff, 기준 기대값, 실행 기록 및 Rust 구현을 대조했다. 각 항목은 서로 다른 입력·회귀·경계 실패 가능성을 확인한다.

1. **변경 범위 — PASS:** 제품 경로의 의미 변경은 direct-text token predicate의 ASCII alphabetic→alphanumeric 확장 하나이며 track sizing·paint 산식은 바꾸지 않았다.
2. **ASCII 판정 — PASS:** UTF-8 token의 모든 byte에 `is_ascii_alphanumeric()`을 적용해 Unicode letters/digits가 ASCII로 오인되지 않는다.
3. **Separator 보존 — PASS:** U+0020 SPACE, TAB, LF, CR만 token separator로 허용하고 이전 whitespace slice의 동작을 유지한다.
4. **여러/경계 separator — PASS:** 앞뒤 공백과 separator 연속 run을 허용하고 source 문자열을 trim·rewrite하지 않는다.
5. **빈 source — PASS:** 빈 문자열과 whitespace-only 문자열은 `has_token`이 false여서 계속 거부된다.
6. **구두점 차단 — PASS:** `A-B`, `A1-B`, `B-1`, `1.2`, underscore 및 기존 punctuation controls는 allowlist 밖이다.
7. **Full-width digit — PASS:** `１２`가 명시적인 negative unit control에 있다.
8. **Arabic-Indic digit — PASS:** `١٢`가 별도 negative unit control에 있다.
9. **Non-ASCII letter 혼합 — PASS:** `é1`은 ASCII digit이 있어도 전체 token 검증에서 거부된다.
10. **다른 공백 경계 — PASS:** form-feed, vertical-tab, NBSP, NEL, em-space negative cases를 기존 Style-to-layout 및 RuntimeSession matrices가 유지한다.
11. **기존 alphabet 회귀 — PASS:** 이전 ASCII letter scene set을 별도 테스트로 계속 실행한다.
12. **Digits-only 양성 — PASS:** `12`, `12345678` scene이 digit-only admission과 sizing을 검증한다.
13. **영문 뒤 숫자 — PASS:** `A1`, `BB2`, `C33`의 suffix-digit grammar를 확인한다.
14. **숫자 뒤 영문 — PASS:** `1A`, `22B`, `33C`의 prefix-digit grammar를 확인한다.
15. **긴 numeric token — PASS:** `12345678`의 독립 advance `48.171875px`가 40px auto track의 automatic minimum을 넘는 기대값이다.
16. **긴 mixed token — PASS:** `A1B2C3D4`의 8-character unbreakable word도 같은 경계를 독립 검증한다.
17. **Min-content track — PASS:** numeric·mixed word의 widest token advance와 `min-content` column 기대값을 source token들로 계산한다.
18. **Auto minimum·overflow — PASS:** auto track에서 40px definite size와 48.171875px min-content contribution의 최댓값을 유지하며 item을 40px로 clamp하지 않는다.
19. **Normal wrapping/row height — PASS:** token 사이의 기존 normal wrapping opportunity와 expected text line count·implicit row sizes를 함께 비교한다.
20. **Reverse source order — PASS:** mixed token source order를 뒤집은 scene이 track과 row-owner 이동을 검사한다.
21. **Oracle 독립성 — PASS:** 1/64 CSS px로 양자화한 Canvas advance 기반 independent track/row/line 계산을 Chrome 157 관측과 별도 필드로 비교한다.
22. **Style-to-layout 행렬 — PASS:** 기존 letter cases와 신규 10-scene JSON matrix가 각각 실행되어 computed layout projection을 검증한다.
23. **RuntimeSession 기본 경로 — PASS:** same fixed scenes가 parsed Root HTML `RuntimeSession`을 통해 default route에서 평가된다.
24. **Identity/revision/trace — PASS:** source, Text node, owner, source order와 layout/paint revision 일관성을 확인하며 원문을 재작성하지 않는다.
25. **거부 원자성 — PASS:** alphanumeric은 이제 유효 입력이므로 과거 `A1`/`B1` negative controls를 punctuation 포함 값으로 교체했고 invalid scene은 partial frame/paint를 내지 않는다.
26. **Fixture/API 격리 — PASS:** 새 parser, public CSS/API, runtime profile, FFI 또는 platform pixel adapter 변경은 없다. 진단 trace는 기존 opt-in fixture-test feature에서만 검사한다.
27. **Artifact 경계 — PASS:** default release Rust rlib에서 fixture marker 부재, fixture-test positive-control executable에서 marker 존재를 확인하고 이를 APK/iOS app-bundle scan으로 확대하지 않았다.
28. **Android 대상 가드 — PASS:** 실행기는 `emulator-5554`의 API 37·`arm64-v8a`를 확인하고 물리 Android 단말을 선택하지 않는다.
29. **iOS 및 실행 자료 수명 — PASS:** 지정 iPhone 17 Pro/iOS 26.2 simulator boot와 앱 종료 상태를 검사하며, 임시 assets를 원본 복사 후 byte-for-byte 복원한다. PixelCopy cache 하나는 이전 PNG를 먼저 보존한 뒤에만 제거한다.
30. **표면/PR 화면 범위 — PASS:** PixelCopy와 compositor 이전 Metal drawable의 지정 CSS `(42,19)` 표본만 1/1 exact로 주장한다. PR 본문의 Android 1080×2400 및 iOS 1206×2622 이미지가 Agent Browser에서 `complete=true`로 열림을 확인한 뒤 브라우저를 종료했다.

**발견 사항:** 검토 중 코드 수정은 추가로 필요하지 않았다. PR head에서 GitHub combined status와 PR workflow-run 목록은 모두 비어 있었으므로 CI 통과로 간주하지 않는다. 로컬 focused tests, scoped Clippy/rustfmt와 simulator evidence는 위 범위로만 기록했다.
