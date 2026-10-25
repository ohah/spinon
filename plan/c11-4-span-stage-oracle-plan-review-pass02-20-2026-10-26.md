# C11.4 span-stage 계획 독립 검토 pass02 · 20관점


**검토일:** 2026-10-26

**대상:** [span-stage 계획](./c11-4-span-stage-oracle.md), [precomparison README](../spec/internal/evidence/c11-4-span-stage-oracle-precomparison-2026-10-26/README.md), [derive_expected.py](../spec/internal/evidence/c11-4-span-stage-oracle-precomparison-2026-10-26/derive_expected.py), [expected.json](../spec/internal/evidence/c11-4-span-stage-oracle-precomparison-2026-10-26/expected.json)

**판정:** PASS 19 · PARTIAL 1. 계획의 예상값·stage 관문은 이 고정 입력에 대해 구현을 진행할 만큼 구체적이다. 다만 trace growth_limit의 비유한 값 인코딩 때문에 일부 비정상 수치를 거부한다는 계획 문구는 검증할 수 없다. 계획 본문과 증거 본문은 수정하지 않았다. Rust test, browser, simulator는 실행하지 않았다.

| # | 독립 관점 / 실패 경로 | 판정 | 근거 및 판단 |
|---:|---|:---:|---|
| 1 | **Slice와 공식 상태의 과대완료** — 좁은 test 작업을 C11.4 또는 공식 완료 수 증가로 보고할 위험 | PASS | 계획은 기존 test assertion만 대상으로 하고, C11.4 Stage B와 공식 72/213을 미완료로 유지한다([계획 5, 35, 48, 54행](./c11-4-span-stage-oracle.md#L5)). 구현 뒤에도 slice만 완료 판정하게 제한한다. |
| 2 | **규범 판본·근거 혼동** — browser 결과를 중간 규범 oracle로 사용하거나 변경 가능한 최신 표준에 기대는 위험 | PASS | CSS Grid Level 1의 고정 2025 CRD §11.5.1/§11.6을 명시하고, 사전 비교 자료에 확인한 CRD HTML SHA-256을 남겼다([README 3행](../spec/internal/evidence/c11-4-span-stage-oracle-precomparison-2026-10-26/README.md#L3)). Chrome 값은 최종 used geometry로 분리한다(계획 29–31행). |
| 3 | **입력 축·track 함수 오지정** — definite가 아니거나 intrinsic function이 다른 입력을 같은 산술로 다루는 위험 | PASS | 네 column 모두 minmax(0px,max-content), 0 gap, definite 220px content width로 고정했다. automatic minimum·fit-content·flex·finite upper limit 등을 일반화하지 않는다고 선을 그었다(계획 17, 27행; README 7, 22행). |
| 4 | **line/span off-by-one 또는 잘못된 겹침** — CSS line 번호와 0-based track index를 혼동해 다른 track을 합산하는 위험 | PASS | A lines 1→3, B 2→4, C 1→4로 문서화되어 각각 tracks 1–2, 2–3, 1–3을 덮는다. 따라서 span-2끼리 track 2 하나를 공유하고 C는 앞 세 track을 덮는다(README 8행; script 16–19행). |
| 5 | **동일 span batch의 덧셈 오류** — 같은 그룹 item 증가를 순차 합산하여 source order에 따라 과대성장하는 위험 | PASS | span-2 A/B의 track별 planned increase 최댓값을 취해 [40,60,60,∞]를 만든다고 명시한다. exact script도 각 item을 그룹 시작 state에서 계산하고 track별 max로 묶는다(계획 17행; script 38–68행). |
| 6 | **이전 span 그룹 state 누락** — span-3 C를 span-2 update 전 state에서 계산하는 위험 | PASS | batch 순서는 span 오름차순이며 C 계산은 앞 세 growth limit 합 160에서 200을 뺀 40을 나눈다. script는 정렬한 span별로 이전 그룹에서 갱신한 growth_limits를 다음 그룹에 넘긴다(계획 17, 27행; script 107–119행). |
| 7 | **DOM source order와 batch order 혼동** — 역순 입력에서 항목 순서가 sizing 그룹 순서를 바꾸는 위험 | PASS | 네 controls가 [A,B], [B,A], [A,B,C], [C,B,A]이고 예상 batch는 [2] 또는 [2,3]이다. JSON은 source order와 normative batch order를 별도 저장하며, script가 forward/reverse 결과 동등성을 각각 assert한다(계획 27행; README 9행; script 21–26, 146–154행). |
| 8 | **스크립트/JSON drift 또는 부동소수점 역산** — 기대값이 Taffy 결과를 가져오거나 반올림 오차로 예상값을 고정하는 위험 | PASS | 산술은 Fraction이고 Taffy trace를 입력으로 쓰지 않는다고 밝힌다. read-only로 스크립트 stdout을 committed expected.json과 diff했으며 차이가 없었다(README 24, 34행; script 9, 140–178행). |
| 9 | **초기 snapshot 누락** — 후속 단계의 정확한 변화량을 확인할 기준점이 없는 위험 | PASS | 모든 positive case에서 Initialize base [0,0,0,0], growth [None,None,None,None]를 확인하도록 수정되었다(계획 19–22, 37행; README 14–17행). |
| 10 | **span batch의 stage 오선택·누락** — 첫 pass만 고르거나 실제 batch가 예상보다 많은데 마지막 값으로 통과하는 위험 | PASS | 각 IntrinsicItemBatch snapshot 모두를 순서대로 확인하고 span별 batch 수를 제한한다. 추가 Columns pass는 선택하지 않고 검토 관문으로 돌린다(계획 27, 36–38행). Taffy는 intrinsic sizing batch가 끝날 때 span과 flex 교차 여부를 붙여 stage를 기록한다([track_sizing.rs 910–920행](../vendor/taffy/src/compute/grid/track_sizing.rs#L910)). |
| 11 | **IntrinsicSizingComplete 경계 누락** — span 밖 track의 무한 growth limit 종료를 검사하지 않는 위험 | PASS | span-2-only와 span-2/3 모두 완료 stage의 track 4 growth limit을 base 0으로 기대한다. Taffy trace는 intrinsic resolution 직후 이 stage를 기록한다(계획 24, 27, 37행; [track_sizing.rs 424–440행](../vendor/taffy/src/compute/grid/track_sizing.rs#L424)). |
| 12 | **span-2-only maximize 오라클 부재** — span-3 케이스만 검사해 span-2 최종 크기·잔여 공간 결함을 놓치는 위험 | PASS | span-2-only의 MaximizeTracks base/growth [40,60,60,0]과 remaining free space 60px을 별도로 명시한다(계획 27행; README 22행; script 175–176행). |
| 13 | **span-2/3 maximize 산술 오류** — finite growth limit 합이나 definite free-space 분배를 잘못 계산하는 위험 | PASS | 최종 base/growth는 [160/3,220/3,220/3,0], 잔여 공간은 20px이다. script가 최종 벡터와 잔여량을 각각 검산한다(계획 25, 27행; script 168–174행). |
| 14 | **Chrome 양자화와 규범값 혼합** — 1/64px 직렬화 결과를 규범적 thirds로 사용하거나 잘못된 허용치를 stage에 적용하는 위험 | PASS | 규범 ideal [53⅓,73⅓,73⅓,0]과 Chrome used values [53.328125,73.328125,73.34375,0]를 구분한다. stage 비교 epsilon과 기존 Chrome 1/64px 비교도 분리한다(계획 29–31, 37행; README 22, 28–30행). |
| 15 | **다중 Columns pass에서 trace를 잘못 고름** — 기존 .find()가 재실행 중 하나만 조용히 선택하는 위험 | PASS | 한 definite Columns pass만 허용하고 수를 검사하며, extra pass가 나오면 원인 확인 전 진행을 멈춘다. 명시적인 definite width와 고정 item widths, percentage 부재도 재실행이 예상되지 않는 근거로 제시한다(계획 36행). |
| 16 | **finite epsilon·NaN/Infinity 비교** — 허용오차가 너무 넓거나 비유한 값을 유한 기대값으로 통과시키는 위험 | PARTIAL | finite 기대값에는 finite actual과 절대 오차 ≤1e-4 CSS px를 요구하고 Chrome epsilon과 분리한 점은 적절하다(계획 37행). 그러나 실제 trace serializer는 growth_limit.is_finite().then_some(growth_limit)이므로 +∞, -∞, NaN 모두 None으로 만든다([Taffy mod.rs 1615–1621행](../vendor/taffy/src/compute/grid/mod.rs#L1615)). 따라서 expected None인 초기/중간 무한값에서는 NaN 또는 -∞가 actual None과 같아도 거부할 수 없다. 이 고정 입력에서 그런 값이 발생한다는 증거는 없지만, 계획의 “NaN 또는 +/-∞ actual은 거부” 보장은 trace API로 달성 불가다. None이 비유한 모든 값의 인코딩이라는 한계를 명시하거나, 무한대와 NaN을 구별 가능한 검증 입력/trace를 제공해야 이 관문이 완전해진다. |
| 17 | **contribution 값을 잘못된 item에 연결** — 숫자는 맞지만 owner NodeId, span 또는 query kind가 다른 item인 위험 | PASS | A/B/C owner와 MaxContent query의 80/120/200px 연결을 assertion하도록 했다(계획 37행). 기존 test가 pass contribution sequence에서 NodeId·kind·span·value를 분리해 검사하는 구조와 맞는다([test helper 192–224행](../crates/spinon-layout/src/grid_multispan_chrome_oracle_tests.rs#L192)). |
| 18 | **flex batch의 selector 오염** — 이번 비-flex oracle이 flex crossing batch를 슬쩍 받아들이는 위험 | PASS | 고정 track 입력은 flexible maximum이 없고, 모든 batch의 crosses_flexible_track=false 및 span별 예상 batch 수를 검사한다. true batch generation 자체는 scope 밖으로 남긴다(계획 38행; script stage label 116행). |
| 19 | **negative control로 제품 경로 확장 또는 거짓 성공** — fixed-only early return/잘못된 placement를 positive stage로 취급하는 위험 | PASS | fixed-only는 FixedTrackEarlyReturn 발생과 batch 부재를 확인하고, 같은 시작/끝 line은 InvalidGridPlacement 오류 및 성공 output 부재를 확인한다. 두 입력은 새 placement/track 지원 없이 control로만 다룬다(계획 38행; [placement validator](../crates/spinon-layout/src/grid_placement.rs#L242); Taffy early return [track_sizing.rs 389–403행](../vendor/taffy/src/compute/grid/track_sizing.rs#L389)). |
| 20 | **검토·실행 관문 누락 또는 C11.4 범위로 확대** — 계획 review와 PR review를 하나로 세거나 앱 검증을 잘못 요구하는 위험 | PASS | 계획 20관점과 plan PR diff 30관점을 별도 요구하고, 이후 focused Rust checks, 구현/runtime 20관점 및 actual PR diff 30관점을 별도 둔다. 완료 기준도 test-only slice로 한정하며 browser/app/simulator 검증과 전체 C11.4 완료를 요구하거나 주장하지 않는다(계획 41–54행). |

## 남은 PARTIAL

Taffy trace의 public field는 finite growth limit을 Some(f32), 그 외 모든 비유한 값—including NaN—을 None으로 투영한다. 그러므로 expected None이 의미하는 +∞를 검증할 때 실제 NaN과 -∞를 구별할 수 없다. 이 보고서 기준으로 계획에 남은 정확한 부분 통과는 이 수치 구별 관문 하나다. 계획 승인 전 해당 문구를 trace가 실제로 증명할 수 있는 범위로 좁히거나, 구별 가능한 근거를 계획에 제시해야 한다.

이 검토는 pass01 표를 복사하지 않고 수정본을 새로 읽어 작성했다. 작업 대상 worktree의 plan/evidence 변경분은 읽기 전용으로 확인했으며 본문을 수정하지 않았다.
