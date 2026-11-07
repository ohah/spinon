# C11.4 `fit-content(px)` implicit row 구현·runtime 실패 관점 검토 20개 · 2026-11-07

검토 범위는 style projection, Grid layout validation/placement, default `RuntimeSession`, fixture와 Simulator 실행이다. 계획 실패 관점 20개와 별개로 실제 구현 경로를 검토했다. 각 항목은 서로 다른 입력 경계나 불변 조건을 본다.

## 검토 항목

1. **Computed type와 authored syntax 분리** — projection은 raw 문자열이 아니라 computed `FitContent(LengthPx)`를 gate한다. Stylo가 `calc(12px + 12px)`와 `0.25in`을 `24px`로 정규화한 경우를 literal px와 같은 RuntimeSession matrix에서 확인했다. 다른 computed type은 지원 경로로 투영하지 않는다. **통과.**
2. **Cap 경계와 부동소수점 비정상 값** — finite non-negative gate와 `0px` cap을 확인했다. layout negative matrix에 `-1`, `NaN`, `+∞` cap을 추가했고 모두 `UnsupportedGridAutoRows`로 거부된다. **통과.**
3. **Max-content가 cap보다 큰 경우** — 고정 payload 34px에 cap 24px을 적용해 implicit row 24px, 다음 probe y=34px를 Layout과 RuntimeSession에서 확인했다. **통과.**
4. **Max-content가 cap보다 작은 경우** — cap 40px에서 34px payload 크기가 보존되고 다음 probe y=44px이다. **통과.**
5. **Minimum contribution이 cap을 넘는 경우** — implicit item `min-height:28px`와 cap 24px의 결과가 28px row, probe y=38px임을 확인했다. **통과.**
6. **반복 implicit row 패턴과 track 상한** — source `grid-auto-rows` 목록은 singleton이어야 한다. 두 implicit row에 반복 패턴이 적용되고 trace가 `[10,24,6]` 등 최종 rows를 확인한다. placement와 item capacity는 `GRID_MAX_TRACKS` 경계를 따른다. **통과.**
7. **Explicit row 보존** — 10px fixed explicit row가 첫 item을 그대로 수용하며 implicit sizing 변경이 explicit row나 40px column으로 번지지 않는 것을 좌표와 trace에서 확인했다. **통과.**
8. **Sparse row cursor 및 following item 위치** — 첫 item, capped item, probe 순서를 유지하고 probe y를 각 cap 장면의 뒤에 놓는다. 명시 배치에 의존하지 않는다. **통과.**
9. **Placement mode 오남용** — column flow와 명시 배치 probe를 RuntimeSession에서 거부하고 partial scene을 만들지 않는다. 내부 layout validator도 새 mode에서 `Auto` row만 허용한다. **통과.**
10. **Grid owner 수와 구조** — default Root HTML projection은 fit-content candidate 하나와 visible Grid owner 하나만 받는다. layout validator는 입력 Grid node를 먼저 확인한 뒤 owner subtree를 검증한다. **통과.**
11. **Definite sizing basis와 box sizing** — positive fixed 40×100px border-box Grid와 고정 40px column·10px row만 profile에 들어온다. 자동·percentage 축이나 border-box 외 shape은 style/layout 검사에서 거부된다. **통과.**
12. **Content/item alignment가 track 결과를 바꾸는 경우** — projection은 `align-content:start`, `justify-content:start`, normal item alignment만 허용한다. layout helper도 intrinsic mode에 허용되는 제한 value 집합만 받는다. **통과.**
13. **Gap·margin·padding·border 영향** — row/column gap과 모든 edge spacing은 zero만 허용한다. Runtime의 `row-gap:1px` control은 거부된다. border/padding/negative margin이 cap 계산에 섞이지 않는다. **통과.**
14. **Implicit columns·intrinsic explicit tracks** — auto columns만 허용하고 explicit intrinsic column control은 projection에서 실패한다. `grid-template-areas`, line names, auto-repeat 및 implicit columns 조합은 candidate 또는 layout contract에서 닫힌다. **통과.**
15. **Visible item 개수와 순서** — 세 visible direct HTML `div` 역할(10px explicit item, capped item, 6px probe)을 고정한다. hidden subtree는 visible count에서 제외하고 `order`, placement, item owner도 재검사한다. **통과.**
16. **Nested payload와 overflow** — capped item 하나에 direct 34px block payload 하나만 허용한다. Grid/item/payload overflow는 visible이어야 하며 payload는 별도 row contribution으로 오인되지 않고 item 밖으로 그려질 수 있다. **통과.**
17. **Text·whitespace·anonymous Grid 경계** — root/item/payload의 non-collapsible text와 anonymous Grid text는 fit-content direct-item slice에 합류하지 않는다. 일반 collapsible HTML whitespace는 구조 통과에 사용할 수 있다. text intrinsic sizing을 과장하지 않는다. **통과.**
18. **Item min/max·aspect·position 경계** — 가운데 item은 min-height 0 또는 28px만 허용하고 첫/probe/payload는 고정 height 조건을 사용한다. max-size, aspect ratio, margin/padding/border, absolute positioning과 item alignment는 projection/layout 검증이 닫는다. **통과.**
19. **오류 귀속·원자성·복구** — dark media의 percentage cap 실패에서 property `grid-auto-rows`, 실제 Grid owner node ID, `completed=None`을 확인한다. 같은 RuntimeSession을 light mode로 되돌리면 layout/render snapshot이 복구된다. **통과.**
20. **Revision·trace·제품 경계** — layout completed key와 computed style snapshot key가 일치한다. Layout oracle은 test-only row sizing trace와 frames를 검사한다. PR diff review에서 새 production route가 trace flag를 켜던 것을 발견해 제거했다. Android/iOS 앱은 fixture-test flags를 끈 기본 route이며 screenshot은 visual smoke만 증명한다. **수정 후 통과.**

## 검토 결과

구현상 차단 결함은 찾지 못했다. 이 검토에서 두 가지 증거 강화를 추가했다. Layout oracle은 허용된 zero cap과 내부 입력의 negative/non-finite cap을 직접 확인하고, RuntimeSession rejection은 오류 node가 Grid owner를 가리키는지 확인한다. Layout focused 2/2, RuntimeSession focused 2/2와 기존 explicit `fit-content` regression 2/2를 다시 실행해 통과했다.

추가 `bool` argument가 projection helper의 Clippy argument 수 한계를 넘긴 것을 workspace Clippy가 발견했다. 이를 두 mode enum `RootHtmlAutoRowsSupport::{None, Intrinsic, FitContentPx}`로 묶어 의미를 분리했고 광범위한 lint suppression은 추가하지 않았다. 이후 별도 PR diff 검토에서 `track_sizing_trace_enabled`가 기본 production route에도 켜지는 것을 찾아 기존 intrinsic rows 외에는 끄도록 수정했다. Trace 없이도 fit-content sizing이 되는 focused RuntimeSession 2/2와 layout 2/2를 다시 확인했고, final pass 4 APK/iOS executable은 최종 source hashes와 같은 revision에서 빌드·실행했다. Workspace Clippy, rustfmt와 diff check도 최종 source에서 통과했다. 상세 source/artifact binding은 `final-pass04-source-revision.json`에 있다.

남은 범위는 Chrome/WPT 전체 적합성, 일반 CSS Grid admission, surface pixel readback, 실기기/hardware GPU와 C11.4/C11 전체다. Android pass 4는 기존 AVD에 `adb install -r`로 설치했고 app-data contents는 검사하지 않았다. iOS pass 4는 새 빈 Simulator에서 실행했다. PR diff 관점 30개 검토는 이 문서와 별도로 기록한다.
