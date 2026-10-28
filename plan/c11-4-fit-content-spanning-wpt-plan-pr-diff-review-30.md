# C11.4 fit-content spanning WPT 계획 PR diff 검토 30개

**검토 대상:** `c11-4-fit-content-spanning-wpt` 계획 PR의 staged diff. 계획 문서 자체의 별도 20개 검토는 [`c11-4-fit-content-spanning-wpt-plan-review-20.md`](./c11-4-fit-content-spanning-wpt-plan-review-20.md)에 기록했다. 구현 전 계획 PR이며 Rust/Java/Swift source는 변경하지 않는다.

1. **PR scope:** 계획·상태 대장·사전 evidence만 바뀌었는가? — PASS. 구현 소스와 public API 변경은 없다.
2. **Source revision:** WPT revision이 plan·README·JSON에서 같은가? — PASS. 모두 `c999c58338ee1d223df5ad62e48be1202ced0d37`이다.
3. **Source hash:** source digest가 세 artifact에서 일치하는가? — PASS. pinned HTML의 SHA-256이 `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736`로 동일하다.
4. **Source lines:** target DOM/assertion 링크가 서로 뒤바뀌지 않았는가? — PASS. DOM 56–58행, assertion 114–116행이다.
5. **Assertion identity:** plan의 “span-2”와 WPT item의 `grid-row:span 2`가 같은가? — PASS. 대상은 하나의 item과 하나의 assertion이다.
6. **Baseline count:** upstream 13/13을 target-only 1/1로 오독하지 않는가? — PASS. 전체 harness count와 single target assertion을 구분했다.
7. **Engine label:** Chrome result가 Spinon 결과로 섞이는가? — PASS. README/JSON/STATUS에서 upstream 관찰과 미실행을 표시했다.
8. **UA precision:** 화면 캡처와 보고서의 browser version이 충돌하지 않는가? — PASS. HeadlessChrome 156과 `156.0.0.0` UA 문자열을 사용했다.
9. **Viewport metadata:** screenshot dimension과 CSS viewport가 다른가? — PASS. 1280×633 viewport와 PNG가 대응하고 DPR 1을 기록했다.
10. **CSSOM value:** `[40,40]` 표기를 computed string `40px 40px`와 혼동하지 않는가? — PASS. JSON은 수치 배열, README는 CSSOM string으로 구분했다.
11. **Root geometry:** Browser viewport-wide root와 planned 60px runtime root를 하나의 observed value로 합치는가? — PASS. JSON은 root height만 browser observation으로 두고 width 60은 독립/runtime 기대값으로 구분했다.
12. **Track count:** implicit rows가 두 개라는 주장이 source span placement에 연결되는가? — PASS. `grid-auto-rows`와 `grid-row:span 2`를 함께 고정했다.
13. **Sizing function:** `fit-content(40px)`가 `minmax(0,40px)`로 바뀌었는가? — PASS. 계획은 fit-content의 auto minimum을 보존한다.
14. **Minimum contribution:** `min-height:0`을 max-content contribution까지 0으로 만드는 것으로 서술하는가? — PASS. minimum floor만 분리하고 200px content를 유지한다.
15. **Content source:** Nested fixed child가 text/font 근거를 필요로 하는가? — PASS. 예상 geometry는 고정 block size로만 산출한다.
16. **Span-group sizing:** §11.5 single-span과 span-2 batch를 같은 경로라고 단정하는가? — PASS. spanning-items 절과 trace 확인을 별도로 요구한다.
17. **Fit-content cap:** 40px limit이 각 row에 적용되는가? — PASS. 독립 표와 final tracks `[40,40]` 모두 명시했다.
18. **Item alignment:** 80px item frame이 200px child와 같은 높이라고 잘못 기재되는가? — PASS. 각각 80px 및 200px이고 overflow는 120px이다.
19. **Root free space:** definite 300px height가 fit-content rows를 300px까지 늘리는가? — PASS. remaining space 220px이 rows에 분배되지 않는다고 설명했다.
20. **Normative source:** 기대값이 Chrome 결과만으로 정당화되는가? — PASS. CSS Grid 2025 CRD 조항 및 산술을 따로 썼다.
21. **Screenshot meaning:** 상단 screenshot이 13개 행 전부를 보여준다고 잘못 부르는가? — PASS. 설명을 viewport에 보이는 첫 assertion과 harness summary 캡처로 제한했다.
22. **Screenshot proof boundary:** PNG가 RGBA exact 또는 Spinon rendering을 입증한다고 쓰는가? — PASS. upstream harness 표시 증거만 주장한다.
23. **Evidence links:** README와 JSON 링크의 상대 경로가 branch root 기준으로 유효한가? — PASS. 두 evidence file 모두 새 위치에 있다.
24. **Plan links:** 상위·선행 plan 참조가 기존 문서를 가리키는가? — PASS. `c11-4-grid-track-sizing`, auto-minimum floor, auto-neighbor 문서가 존재한다.
25. **Status hierarchy:** 신규 entry가 C11.4가 아닌 다른 milestone 아래에 들어갔는가? — PASS. unchecked C11.4 하위에만 추가했다.
26. **Completion count:** 계획 PR만으로 official count가 변하는가? — PASS. STATUS와 JSON은 unchecked 및 `72/213`을 그대로 둔다.
27. **Interface/version contract:** API 문서를 발명하거나 version을 올리는가? — PASS. 새 public/internal API는 계획하지 않는다.
28. **Runtime contract:** future simulator outcomes를 이미 검증한 것으로 쓰는가? — PASS. STATUS는 app runtime과 Simulator가 아직 실행되지 않았다고 명시한다.
29. **Artifact boundary:** fixture code가 product artifact에 포함될 것을 승인하는가? — PASS. 계획은 existing opt-in feature와 negative/positive controls를 요구한다.
30. **Staging boundary:** unrelated snapshots/evidence가 PR에 포함되는가? — PASS. stage 대상은 계획 slice 파일 목록으로 한정한다.

**판정:** 이 diff에서 차단 결함을 찾지 못했다. 리뷰는 계획 PR 변경에만 해당하며 구현 코드/runtime 검토를 대체하지 않는다.
