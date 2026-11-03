# C11.1 Root HTML `grid-auto-rows` calc 계획 PR 변경 검토 20개

**대상 변경:** 계획, 구현 전 비교 fixture/evidence, 계획 실패 관점 검토. 구현 변경은 포함되지 않는다.

1. **PR 범위가 Root HTML implicit row calc slice와 일치하는가 — PASS.** 변경된 계획은 `RuntimeRootHtmlTextPaintV1`의 row auto-track 경로만 다룬다.
2. **기존 fixed-px 및 percentage row-list 계획을 중복하는가 — PASS.** 새 범위는 calc tree 투영, row math binding과 extent 검증으로 구별된다.
3. **기존 fraction auto-row 지원을 실수로 제외하는가 — PASS.** direct `fr` controls와 calc+fr neighbor를 계획에 포함한다.
4. **fixture가 두 explicit row와 여섯 implicit row를 실제로 만들 수 있는가 — PASS.** explicit `10px 20px`, 한 explicit column, item 8개와 row flow가 구성과 일치한다.
5. **main-scene percentage basis가 명확한가 — PASS.** definite content height는 100px이고 padding/border가 0이며 row-gap은 3px로 분리되어 있다.
6. **independent main used rows의 산술이 일관되는가 — PASS.** `10%+2=12`, `25%=25`, `15px`, pattern 2회가 `[12,25,15,12,25,15]`를 만든다.
7. **main row starts가 track과 gutter를 분리하는가 — PASS.** 기대 시작점 `[0,13,36,51,79,97,112,140]`은 각 직전 track 뒤 3px gap을 한 번만 더한다.
8. **negative boundary가 syntax error와 혼동되는가 — PASS.** Chrome은 `calc(1px - 10%)`를 지원하고 typed computed calc와 used `0px`를 관측한다.
9. **negative range 규칙을 관측값만으로 정당화하는가 — PASS.** 기대값은 별도 CSS Values range 규칙에 근거한다.
10. **over-100% row를 잘못 clip하거나 clamp하는가 — PASS.** 독립 기대값과 Chrome 관측 모두 used track `121px` 및 visible overflow를 둔다.
11. **zero-height basis를 indefinite와 혼동하는가 — PASS.** fixture height 0은 direct definite length이고 기대 calc result는 5px다.
12. **row-gap이 calc percentage basis에 영향을 주는가 — PASS.** 100px의 10%와 5px fixed term으로 15px row를 계산한 뒤 10px gutter를 더한다.
13. **computed style에 남는 calc와 length-only normalize된 calc를 혼동하는가 — PASS.** 계획은 typed calc tree만 새 범위로 삼고 LengthPx 정규화는 기존 경로에 둔다.
14. **CSS math ID를 row 대신 column에 연결할 위험 — PASS.** `GridTemplateRows`와 같은 node의 binding, missing/duplicate/wrong binding rejection을 명시한다.
15. **math AST 지원 범위가 불명확한가 — PASS.** 직접 px 및 percentage 한 항씩의 2-term Sum만 허용하고 다른 tree를 제외한다.
16. **resource/extents가 frame 작성 뒤에만 검사되는가 — PASS.** 계획은 track quota, 개별/누적 extent와 failure atomicity를 layout 전에 확인하게 한다.
17. **실패 케이스가 CSS parse 오류만 포함하는가 — PASS.** syntax-invalid cascade와 syntax-valid unsupported math를 별도로 구분한다.
18. **브라우저 캡처를 Spinon 또는 표면 픽셀 증거로 과장하는가 — PASS.** README와 계획은 Chrome visual observation, runtime, draw/present, readback을 구분한다.
19. **fixture와 JSON artifact가 재현 가능한가 — PASS.** 외부 stylesheet 없이 고정 HTML, raw value JSON, expected JSON과 1280×633 PNG가 PR 변경에 포함된다.
20. **계획 PR이 공식 상태·로드맵 완료 수를 조기에 바꾸는가 — PASS.** 변경은 plan/evidence만 추가하고 C11.1/C11 checkbox와 official count를 유지한다.

**결론:** 변경 diff는 계획 진입에 필요한 fixture, browser observation, 산술 기대값과 별도 plan review를 함께 제공한다. JSON 형식, 내부 Markdown 링크, `git diff --check`를 검증했다. 기능 테스트·컴파일·simulator 실행은 계획 PR에서 수행했다고 주장하지 않는다.
