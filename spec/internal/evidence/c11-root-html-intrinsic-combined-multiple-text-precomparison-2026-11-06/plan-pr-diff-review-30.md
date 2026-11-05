# 계획 PR #1201 실제 diff 적대 검토 · 30개 관점

대상 PR: `plan(c11): C11.4 intrinsic columns와 여러 text row 계획` · head `1ece9a5a031a8b56cd6cf0f4bced2607d7637188` · base `main`의 PR 실제 변경 목록 10개 파일.

1. **변경 범위:** 실제 diff는 계획, 계획 검토, precomparison fixture/evidence만 추가하며 production Rust 코드나 기존 상태 문서를 수정하지 않는다.
2. **공식 집계:** plan/PR 본문은 C11.4/C11 미완료와 `78/214` 유지라고 명시한다. 완료 checkbox 변경이 없다.
3. **내부 계약 버전:** 숫자 버전 상향이나 기존 계약 변경을 제안하지 않는다.
4. **fixture hash:** expected JSON에 적힌 `06330a…afc872`를 실제 fixture SHA-256과 대조했다.
5. **observation linkage:** README, plan, expected JSON은 같은 fixture와 evidence directory를 가리킨다.
6. **browser provenance:** raw JSON은 macOS arm64, HeadlessChrome 156, DPR 1, 1280×633 viewport, loaded font 상태를 기록한다.
7. **mode coverage:** `min-content`, `max-content`, `auto`, `minmax(min-content,1fr)` 모두 raw observation과 expected JSON에 있다.
8. **min-content used size:** Chrome raw observation의 6.03125px column과 expected track 값이 일치한다.
9. **max-content used size:** 66.234375px expected는 Chrome의 computed grid track 66.2344px와 precision 차이를 구분해 기록한다.
10. **auto sizing:** definite 40px Grid 안에서 auto column의 40px used size와 2-line long text가 일치한다.
11. **flex sizing:** 1fr representative scene은 별도 raw case로 측정해 auto와 같다는 점을 관측으로 명시한다.
12. **source reversal:** 각 mode의 observation은 long/short DOM reversal을 실행하고 두 row order를 기록한다.
13. **item identity:** observation은 id별 frame을 보존해 row reversal과 item identity를 혼동하지 않는다.
14. **explicit row:** 모든 scene에서 첫 10px explicit row와 나머지 implicit rows를 분리해 기록한다.
15. **per-item contribution:** 긴/짧은 text의 줄 수와 각 implicit row를 별도로 적고 합산 contribution으로 뭉개지 않는다.
16. **max-content line wrapping:** one-line long text와 `max-content` column overflow가 서로 구분된다.
17. **visible overflow:** min-content/max-content/auto 각 경우의 final content bottom과 root 하단 관계가 계획에 기록되어 있다.
18. **empty controls:** control items have auto width, so they do not create an unintended fixed 40px intrinsic minimum.
19. **root topology:** one Grid owner plus direct children is stated; multi-owner parser admission is not implied.
20. **text source boundary:** ASCII `A` tokens and one U+0020 separators are explicit; general Unicode/string support is not implied.
21. **item topology:** nested markup/additional descendants and anonymous direct text are rejected in plan boundary.
22. **hidden-node semantics:** existing display-none/visibility handling is retained rather than redefined.
23. **computed style owner:** each item must pass the pre-existing style, source and revision checks individually.
24. **explicit track grammar:** the plan reuses the existing combined intrinsic-column route and does not claim new grammar.
25. **row-axis grammar:** one positive fixed explicit row and singleton `grid-auto-rows:min-content` are required.
26. **forbidden grid features:** fit-content, intrinsic explicit rows, auto-repeat, named areas/lines, implicit columns, dense/column flow and explicit placement are listed as outside scope.
27. **gaps:** zero gaps in the fixed matrix prevent a new percentage/math-gap interaction from entering this slice.
28. **resource limits:** checked track/item limits and optional diagnostic trace budgets remain implementation requirements.
29. **platform claims:** plan PR makes no Android/iOS implementation, hardware, WPT or CI-success claim; follow-up verification is explicit.
30. **evidence type:** browser screenshots and Chrome measurements are labeled precomparison, not Spinon product screenshots or runtime proof.

검토 결과: plan PR actual diff에 수정이 필요한 오류는 없었다. This is a review of the planning PR only; it does not replace the planned 20-case implementation/runtime review or implementation PR review.
