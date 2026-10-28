# C11.4 `fit-content()` automatic-minimum floor 계획 PR diff 적대 검토 30개

이 검토는 계획 검토 20개와 별개로, 계획 PR에 실제로 포함된 변경 파일 사이의 누락·불일치·근거 과장을 살핀다. 코드/runtime 구현 검토는 포함하지 않는다.

1. **계획 PR에 구현 코드가 섞일 수 있음 — PASS.** 변경 파일은 계획·검토·C11.4 상태 기록·precomparison 자료뿐이며 Rust/Java/Swift/FFI source는 없다.
2. **미구현 slice를 완료로 표시할 수 있음 — PASS.** 새 STATUS 기록과 계획 상태는 `구현/runtime 미시작`이며 C11.4 checkbox는 그대로 미체크다.
3. **공식 완료 집계가 오를 수 있음 — PASS.** `72/213`은 STATUS의 숫자나 집계식에서 변경되지 않았고 관련 문서에서 유지한다고 적었다.
4. **새 집중 계획이 상위 계획 인덱스에서 빠질 수 있음 — PASS.** `plan/c11-4-grid-track-sizing.md` 맨 앞 날짜 기록이 집중 계획, 사전 비교, 계획 review 20개를 연결한다.
5. **상태 대장이 잘못된 상위 작업 아래 기록할 수 있음 — PASS.** 새 줄은 C11 CSS Grid의 unchecked C11.4 바로 아래에 들어갔다.
6. **계획 파일 상대 링크가 오타날 수 있음 — PASS.** 내부 계약, expected JSON, README, plan review 등 새 링크 대상을 모두 파일 시스템에서 확인했다.
7. **WPT 경로가 인접한 auto-repeat test로 바뀔 수 있음 — PASS.** 계획·README·expected JSON 모두 정확히 `fit-content-track-sizing.html`을 가리킨다.
8. **WPT revision이 계획과 기계 판독 자료에서 다를 수 있음 — PASS.** 세 파일 모두 `c999c58338ee1d223df5ad62e48be1202ced0d37`이다.
9. **고정 source hash가 오래됐거나 오기일 수 있음 — PASS.** checkout 파일의 SHA-256을 재계산했고 `wpt.live` bytes를 별도 fetch해 같은 `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736`임을 확인했다.
10. **틀린 test case 줄을 연결할 수 있음 — PASS.** 원본 DOM `31–34`, target assertion `90–92`, min-zero WPT control `36–38`/`94–96`이 각각 실제 source와 맞는다.
11. **전체 upstream pass가 Spinon 결과로 보일 수 있음 — PASS.** README와 expected JSON 모두 HeadlessChrome의 13/13을 upstream-only라고 명시한다.
12. **Chrome 버전이 계획에 근거 없이 붙을 수 있음 — PASS.** 실제 Agent Browser `navigator.userAgent`는 `HeadlessChrome/156.0.0.0`이었다.
13. **13 tests가 끝나기 전에 computed value를 읽을 수 있음 — PASS.** browser run은 `13 Pass`까지 기다린 뒤 target computed values를 읽었다.
14. **대상 computed row serialization이 기대 JSON과 다를 수 있음 — PASS.** live target은 `10px 100px`, height `300px`이었고 expected JSON의 `[10,100]` 및 `[60,300]`과 일치한다.
15. **grid item automatic minimum 조건이 바뀔 수 있음 — PASS.** live item `min-height` 두 값은 모두 `auto`; WPT CSS에는 overflow override가 없어 기본 visible 조건이 보존된다.
16. **nested content heights가 계획 밖으로 바뀔 수 있음 — PASS.** browser observation은 child `10px`·`100px`, source DOM과 계획도 같은 값을 둔다.
17. **formula 출처가 비규범 문서일 수 있음 — PASS.** 계획과 README는 규범 근거로 W3C CSS Grid 2025 CRD §6.6·§11.2·§11.5를 사용한다.
18. **fit-content cap이 minimum을 clamp한다고 잘못 설명할 수 있음 — PASS.** CRD §6.6의 fit-content argument 예외와 §11.2의 `max(minimum,min(limit,max-content))`를 함께 기록한다.
19. **첫 번째 track 계산이 틀릴 수 있음 — PASS.** `max(10,min(40,10))=10px`로 기대 JSON·README·계획에 일치한다.
20. **두 번째 track이 40px로 잘릴 수 있음 — PASS.** `max(100,min(40,100))=100px`이며 CRD의 automatic minimum floor가 근거다.
21. **root size, alignment와 track size를 하나의 oracle로 합칠 수 있음 — PASS.** root height 300px, `align-content:normal`, main Rows `[10,100]px`을 분리해 두고 fit-content row를 auto max-track stretch 대상이라고 취급하지 않는다.
22. **min-zero control을 main WPT DOM과 같다고 과장할 수 있음 — PASS.** 계획은 control을 differential control로 한정하고 WPT 전체 assertion 구현으로 세지 않는다.
23. **control overflow가 자동 clipping될 수 있음 — PASS.** control track 40px, child 100px, visible overflow 60px을 각각 기대하고 표면 sample 대상으로 둔다.
24. **auto-width deviation이 숨겨질 수 있음 — PASS.** runtime 60px width 조정과 row/block-axis 비교 범위가 계획·expected JSON·README에 명시돼 있다.
25. **existing implementation scene ID와 충돌할 수 있음 — PASS.** 양 scene ID를 FFI/layout fixture 구현 경로에서 검색했고 현재 충돌은 없다.
26. **정확히 두 scene만 allowlist 된다는 범위를 빠뜨릴 수 있음 — PASS.** plan은 WPT main과 min-zero control의 exact ID, 변형 topology/style 및 unknown ID 거부를 요구한다.
27. **fixture ABI가 기본 제품 archive에 들어갈 수 있음 — PASS.** default/product negative control과 opt-in fixture positive control을 모두 수용 기준으로 둔다.
28. **PR 본문 screenshot raw URL이 잘못된 commit SHA를 가리키거나 앱 결과처럼 해석될 수 있음 — PASS.** PR metadata 대조에서 초기 URL의 SHA 오기를 찾아 실제 PR head `6efc7f71348c22cefc0186a7f1602bf85b753ae7`로 수정했다. Agent Browser에서 PR #763 본문 이미지를 읽어 `complete:true`, `1280×5266`을 확인했고 alt text도 upstream WPT baseline으로 표시된다.
29. **screenshot 또는 JSON이 수정 뒤 이전 hash를 유지할 수 있음 — PASS.** expected JSON parse, SHA256SUMS의 세 파일 checksum, screenshot 1280×5266 및 digest를 현재 bytes로 확인했다.
30. **Agent Browser가 끝난 뒤 살아 있을 수 있음 — PASS.** capture session을 닫은 후 `session info`가 `active:false`, `pid:null`을 반환했다.

**판정:** 변경 파일과 출처·계산·범위의 상호 불일치나 별도 수정이 필요한 누락은 발견하지 못했다. 계획 PR 변경 검토는 **30 PASS / 0 PARTIAL / 0 FAIL**이다.
