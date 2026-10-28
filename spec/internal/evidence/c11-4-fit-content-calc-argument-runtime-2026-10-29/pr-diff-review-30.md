# C11.4 definite calc fit-content · implementation PR diff review 30

구현 diff와 증거 연결을 서로 다른 failure perspective 30개로 살폈다. pass는 수정된 최종 tree 기준이다. Simulator run은 captured dirty-source digest가 최종 코드에 대응하며, 뒤에 추가한 문서/evidence 파일은 실행 코드에 영향을 주지 않는다.

최종 변경 검토에서는 610줄 fixture 검증 코드의 `style_validation.rs` 분리도 반영했다. 30개 항목을 새 parent/helper 경계에 대조했고, helper는 private `super` API만 사용하며 계산 조건은 그대로 유지되는 것을 확인했다. module boundary test에 새 파일과 typed calc·registry assertion을 고정한 뒤 2/2 재실행했다. 최종 source tree의 Rust/FFI tests, Clippy, Android/iOS build와 simulator evidence도 다시 확인했다.

1. **Fixture-only surface area:** `StyleLayoutOutput.css_math`은 `c11-4-grid-track-sizing-fixture-test` feature 뒤에 있다. Pass.
2. **Existing layout input ownership:** registry clone은 기존 `with_css_math(projected.css_math)` 경로를 보존한다. Product projection semantics 변경 없음. Pass.
3. **No general parser expansion:** CSS grammar/admission/Stylo profile allowlist를 넓히지 않았다. Pass.
4. **No accidental allocation exposure:** 추가 clone은 opt-in C11.4 build에만 포함되고 scene adapter에서 검사한다. Pass.
5. **Observation lifetime:** captured registry is immutable `Arc<[LayoutCssMathValue]>` matching the observation snapshot. Pass.
6. **Correct call-site wiring:** new registry argument is passed only to the intended percentage/calc observer. An earlier compile error caught the neighboring call-site miswire; corrected tree compiles/tests. Pass.
7. **Scene dispatch:** percentage and calc cases use different scene IDs, node IDs, CSS text, and expected heights. Pass.
8. **Existing percentage regression:** original `fit-content(50%)` remains row 150px and passes both platform runs. Pass.
9. **Calc arithmetic:** definite 300px basis resolves `10% + 5px` to 35px independently of 400px max-content. Pass.
10. **Computed value exactness:** calc terms/order and serialized computed property are exact. Pass.
11. **Typed AST exactness:** `ComputedCssMath::Sum([Percentage(0.1), LengthPx(5.0)])` is required. Pass.
12. **Layout input exactness:** exactly one implicit row, one 60px column, row auto-flow and track-sizing placement. Pass.
13. **ID extraction failure:** malformed/missing `FitContent(Calc(id))` fails closed. Pass by source/test inspection.
14. **Registry bijection:** calc handle maps to exactly one registry record. Pass.
15. **Owner identity:** record must identify the root grid node. Pass.
16. **Property identity:** record must use `GridTemplateRows`. Pass.
17. **Expression identity:** exact sum expression is checked; wrong length is a negative control. Pass.
18. **Stale ID control:** altered registry ID is rejected. Pass.
19. **Extra registry control:** duplicate/unrelated records are rejected by exact slice equality. Pass.
20. **Post-layout validation:** output validation repeats style/input/registry checks against stored observation before accepting frames/trace. Pass.
21. **Frame assertion:** root 60×300, row item 60×35, child 30×400 are all exact. Pass on both simulators.
22. **Trace assertion:** six row sizing stages and expected final base-size sequence are checked. Pass.
23. **FFI serialization:** diagnostic report exposes registry id, node, property, expression without changing its generic envelope. Pass.
24. **Allowlist scope:** only existing percentage scene and one planned calc scene enter this specialized adapter. Pass.
25. **Android matcher robustness:** matcher compares structured registry fields and derives ID from projected autoRows. Initial suffix bug was exposed by capture and fixed; rerun passes.
26. **iOS matcher robustness:** Swift matcher validates empty percentage registry or one matching calc record. Simulator run passes.
27. **Capture assertions:** JS requires scene-specific computed style, frames, trace, registry binding and every declared surface sample. Pass.
28. **Scene catalog count:** module-boundary fixture catalog expected count is updated by one and named test checks both scenes. Pass 2/2.
29. **Surface coordinate bounds:** calc scene samples cover row interior, item, below row and child outside root at y=350 within 320×400 viewport. Pass 4/4 on both platforms.
30. **Contract and status honesty:** internal contract 0059 documents the fixture diagnostic without changing its fixed `0.1.0` version; plan, C11.4 status and evidence mark one opt-in slice, while the C11.4 checkbox and official `72/213` remain incomplete. Pass.

## Corrective findings

The review process caught two implementation mistakes before final evidence: a new observation argument was attached to the adjacent auto-neighbor call site, and Android's structured calc-ID parser used a suffix one closing parenthesis short. Compiler/capture failures pinpointed both. The final runtime tests, both app builds, and combined simulator matrices passed after correction. Existing unrelated untracked files and historical worktrees were not added or removed.
