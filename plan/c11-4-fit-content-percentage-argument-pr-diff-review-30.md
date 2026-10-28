# C11.4 definite-height percentage `fit-content()` 계획 PR diff 적대 검토 30개

검토 범위는 계획 PR 변경분인 C11.4 상위 계획 링크, 새 focused plan, 계획 검토 20개, pinned-source/Chrome 사전 비교 README·JSON·screenshot이다. 구현 변경이나 앱 runtime 결과는 포함하지 않는다. 앞선 계획 검토 20개와 다른 PR diff 관점으로 검토했다.

1. **작업 branch가 main에 직접 기록될 수 있음 — PASS.** 변경은 `plan/c11-4-fit-content-percentage-argument` branch에 있다.
2. **구현이 계획 PR에 섞일 수 있음 — PASS.** 변경 파일은 계획·review·precomparison뿐이며 Rust/Android/iOS source는 수정하지 않는다.
3. **공식 상태가 미리 완료될 수 있음 — PASS.** `spec/STATUS.md`와 공식 `72/213`은 바꾸지 않는다.
4. **상위 계획에서 새 계획을 찾을 수 없음 — PASS.** WPT candidate table의 definite-height percentage row 링크가 새 plan을 가리킨다.
5. **새 계획에서 상위 문서가 연결되지 않을 수 있음 — PASS.** C11.4 plan, internal contract 0059와 official status link가 있다.
6. **계획 검토 기록이 빠질 수 있음 — PASS.** Plan review 20 문서를 status line에서 연결한다.
7. **PR diff review가 계획 review와 섞일 수 있음 — PASS.** 현재 문서는 별도의 30-item plan PR diff review로 추가한다.
8. **WPT 경로가 유사 테스트로 바뀔 수 있음 — PASS.** Canonical `css/css-grid/grid-definition/fit-content-track-sizing.html`을 지정한다.
9. **upstream revision이 흔들릴 수 있음 — PASS.** Pinned revision `c999c58338ee1d223df5ad62e48be1202ced0d37`을 고정한다.
10. **source content hash가 잘못 기록될 수 있음 — PASS.** Checkout과 live fetch의 SHA-256이 `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736`로 같다.
11. **다른 WPT case를 target으로 착각할 수 있음 — PASS.** Target DOM `percentageArgument`는 line 48로 고정한다.
12. **잘못된 assertion을 기대값으로 사용할 수 있음 — PASS.** line 107의 exact expected string `150px`을 기록한다.
13. **partial harness를 전체 실행으로 표현할 수 있음 — PASS.** Browser harness `OK`, 13/13은 upstream WPT result로 분리한다.
14. **browser build를 재현할 수 없음 — PASS.** Agent Browser 0.31.1과 HeadlessChrome 156.0.0.0 user agent를 기록한다.
15. **DPR/viewport가 빠질 수 있음 — PASS.** 1280×633 CSS px와 DPR 1을 JSON·README에 남긴다.
16. **root size를 computed track과 혼동할 수 있음 — PASS.** root height 300px과 rows 150px을 독립 필드로 보존한다.
17. **Chrome row observation이 누락될 수 있음 — PASS.** Browser observation에는 `grid-template-rows: 150px`과 item height 150px이 있다.
18. **screenshot이 target assertion을 잘라낼 수 있음 — PASS.** Full-page capture에는 13개 행이 함께 있고 target row가 보인다.
19. **screenshot artifact가 바뀔 수 있음 — PASS.** 1280×5266 PNG SHA-256 `a7eaaf9fd1d68755272c044c519d277546ec6e070861c0cf663dfa3698c77849`를 기록한다.
20. **baseline을 Spinon 성공으로 오해할 수 있음 — PASS.** README와 JSON은 upstream Chrome only로 해석하고 Spinon pass를 주장하지 않는다.
21. **percentage basis를 wrong axis에서 계산할 수 있음 — PASS.** CRD §7.2.1 row basis는 definite inner block size다.
22. **indefinite/cyclic percentage와 혼동할 수 있음 — PASS.** 입력은 definite height 300px이며 cyclic fallback은 적용하지 않는다.
23. **fit-content minimum rule이 빠질 수 있음 — PASS.** CRD §§11.2/6.6과 item `min-height:0`을 연결해 설명한다.
24. **150px 결과가 browser observation에서 역산될 수 있음 — PASS.** `300 × 0.5` basis calculation을 독립 expected value로 명시한다.
25. **400px content가 cap보다 작을 수 있음 — PASS.** Pinned nested child의 fixed height 400px이 독립 input과 계획에 남는다.
26. **runtime viewport normalization이 숨겨질 수 있음 — PASS.** WPT의 auto width를 scene에서는 60px로 고정한다고 공개한다.
27. **implicit row가 explicit sizing으로 바뀔 수 있음 — PASS.** `grid-template-rows:none`과 single `grid-auto-rows`를 요구한다.
28. **visual width 변경이 sizing input과 혼동될 수 있음 — PASS.** Child width 30px은 surface contrast용이며 block height 400px은 고정이라고 적는다.
29. **root box 바깥 overflow가 측정되지 않을 수 있음 — PASS.** `(15.5,350.5)`는 300px root 아래 50px이며 400px child 안의 pixel center다. Android PixelCopy와 iOS Metal drawable을 요구한다.
30. **검증 범위가 확대 해석될 수 있음 — PASS.** 양 Simulator, default/fixture archive, 나머지 12개 WPT, general CSS, physical GPU와 C11.4 completion을 별도 경계로 둔다.

**판정:** 계획 PR 변경분에서 고칠 오류를 발견하지 못했다. PR 본문에는 upstream Chrome screenshot임을 표시해 첨부하고, 계획 병합 뒤에만 implementation work를 시작한다.
