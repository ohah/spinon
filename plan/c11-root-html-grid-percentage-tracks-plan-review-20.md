# C11.1 Root HTML Grid percentage-track plan adversarial review

**대상:** [`c11-root-html-grid-percentage-tracks.md`](./c11-root-html-grid-percentage-tracks.md) · [`expected.json`](../spec/internal/evidence/c11-root-html-grid-percentage-precomparison-2026-11-02/expected.json) · 고정 Chrome observation

**검토 시점:** plan PR 작성 전, 2026-11-02 · 구현은 시작하지 않음

| # | 실패 관점 | 검토 결과 및 요구 조치 |
|---:|---|---|
| 1 | 열 percentage를 block 축 기준으로 계산 | 통과. CSS Grid §7.2.1의 inner inline size 규칙과 240px 계산을 고정했다. |
| 2 | 행 percentage를 inline 축 기준으로 계산 | 통과. §7.2.1의 inner block size를 별도로 사용하며 120px fixture가 있다. |
| 3 | border-box 외곽 크기를 content percentage basis로 오인 | 통과. 기존 Root HTML edge gate가 border/padding을 0으로 제한해 명시 크기와 inner size가 일치한다. |
| 4 | column-gap을 percentage basis에서 먼저 차감 | 통과. `both-axes`와 `% + fr`에서 percentage를 전체 inner size로 계산하고 gap은 track-origin/free-space 단계에서 처리한다. |
| 5 | `% + fr` 처리에서 같은 gap을 두 번 차감 | 통과. 25%×240=60, gap 10, 1fr=170 독립 기대와 Chrome 관측을 고정했다. |
| 6 | 100%를 넘는 track을 조용히 100%로 자름 | 통과. 120%/10%의 `[240,20]`과 8px gap, track end 268을 양성 기준에 둔다. |
| 7 | 0% track이 음수 또는 최소 콘텐츠로 확대됨 | 통과. `[0,200]` track과 첫 8px marker의 visible overflow를 관찰한다. |
| 8 | definite zero-width axis를 indefinite로 혼동 | 통과. width 0px의 `[50%,50%]`에서 `[0,0]`을 별도 기준으로 둔다. |
| 9 | indefinite percentage를 현재 zero/min-content로 대체 | 통과. CSS의 cyclic intrinsic sizing 경로와 Root HTML의 fail-closed 진단을 분리했다. |
| 10 | inline/row 방향을 writing mode 변경에도 그대로 적용 | 통과. 현재 Root HTML의 horizontal writing mode 제한을 유지하며 다른 writing mode를 수락하지 않는다. |
| 11 | percentage에 `calc()`, `minmax()` 또는 `fit-content()`가 섞여 direct value 검사를 우회 | 통과. 오직 direct `Percentage` typed track을 추가하고 조합 함수는 거부한다. |
| 12 | percentage gap이 새 track 지원에 딸려 들어옴 | 통과. gap converter는 fixed-px-only로 유지하고 별도 negative control이 있다. |
| 13 | named-area의 40/80px·20/50px 고정 제한이 무심코 완화 | 통과. percentage + template areas를 negative control로 두고 기존 validator는 유지한다. |
| 14 | computed `NaN`/infinity 또는 음수 percentage가 Taffy까지 전달 | 통과 기준. projection에서 finite non-negative 값만 받으며 음수 declaration은 CSS parser의 unsupported/auto 결과도 통과하지 않아야 한다. 구현 테스트에 넣는다. |
| 15 | finite percentage × definite size가 f32 overflow를 만들어 무한 track 또는 partial frame을 게시 | 통과 기준. resolve된 track과 fixed aggregate/gap의 f32 representability를 확인하고 overflow를 layout 전에 거부하도록 계획에 포함했다. |
| 16 | 10,000 direct-track 상한이 Percentage 변형에서 우회 | 통과. 기존 `spinon-layout` 상한 및 pre-Taffy 거절을 유지하는 regression을 요구한다. |
| 17 | item의 min-content contribution이 fixed percentage track 크기를 오염 | 통과. runtime positive source는 fixed-size empty block item만 사용하고 invalid intrinsic/text/image 경계는 rejection으로 둔다. |
| 18 | track이 item보다 작거나 zero여도 기본 visible overflow가 clip으로 바뀜 | 통과. marker는 0px track에서 8px이고 overflow clipping/scroll은 추가하지 않는다. |
| 19 | root ID, class, source order 또는 stale revision이 성공 결과를 결정 | 보완 요구. 구현 수락 기준에 selector/ID 재작성과 reversed source-order positive control을 추가해 semantic tracks 및 source-order placement를 확인한다. |
| 20 | fixture 또는 implementation claim이 simulator draw/present를 pixel/WPT proof로 과장 | 통과. source/build hash와 tracks/frames/draw/present를 구분하고 readback 없이는 pixel equality를 말하지 않으며 C11/WPT/physical-device 범위를 제외한다. |

## 반영 상태

관점 19의 source identity 대조를 plan 수락 기준에 추가했다. 나머지 관점은 계획에 정의된 oracle, typed input 경계, fail-closed 조건, platform/artifact 제한으로 대응한다. 이 리뷰는 계획 검토이며 구현/runtime evidence가 아니다.
