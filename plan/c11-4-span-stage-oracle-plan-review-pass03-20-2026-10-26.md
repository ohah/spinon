# C11.4 span-stage 계획 독립 검토 pass03 · 20관점


**검토일:** 2026-10-26

**대상:** [span-stage 계획](./c11-4-span-stage-oracle.md), [precomparison README](../spec/internal/evidence/c11-4-span-stage-oracle-precomparison-2026-10-26/README.md), [expected 산출 스크립트](../spec/internal/evidence/c11-4-span-stage-oracle-precomparison-2026-10-26/derive_expected.py), [expected.json](../spec/internal/evidence/c11-4-span-stage-oracle-precomparison-2026-10-26/expected.json)

**판정:** 내용 검토 PASS 20 / PARTIAL 0. pass02의 비유한값 한계가 계획과 evidence 양쪽에 명시되어, trace None을 관찰한다는 좁은 주장은 검증 가능하다. 이 범위 변경은 test-only 구현을 시작할 만큼 충분하다. 다만 계획 자체에 적힌 별도 plan PR diff 30관점은 아직 이 검토에 포함되지 않으며, 계획의 순서를 그대로 따르면 그 관문 완료 후 구현해야 한다. Rust tests/browser/simulators는 실행하지 않았다.

| # | 실패 관점 | 판정 | 판정 근거 |
|---:|---|:---:|---|
| 1 | **내부 수치와 trace projection 혼동** — trace의 None을 internal +∞의 증명으로 보고할 위험 | PASS | 계획은 serializer가 모든 non-finite growth limit을 None으로 투영한다고 쓰고, 이 slice가 내부 +∞를 증명하지 않는다고 한정한다(계획 37행). evidence README도 동일한 제한을 규범 산술 설명에 연결한다(README 22행). |
| 2 | **이전 limitation의 문구만 바꾸고 검증 요구를 남김** — 비유한값 구별이 불가능한데도 NaN/±∞ 거부를 계속 요구하는 위험 | PASS | 바뀐 comparator는 expected finite에 Some(finite)만 허용하고, expected unbounded에는 None projection만 기록한다. 이전의 “NaN 또는 ±∞ actual은 거부” 요구는 사라졌다(계획 37행; README 22행). |
| 3 | **비유한 expected state 비교가 모호함** — expected None과 actual Some(∞) 같은 잘못된 비교 규칙 | PASS | expected None은 Taffy trace의 None과만 일치시키며 Some(finite)는 허용하지 않는다. 해당 비교가 actual 내부 분류가 아닌 public trace 값 비교임을 계획에서 명시한다(계획 37행). |
| 4 | **유한값의 None 허용** — finite stage에서 trace projection으로 잘못된 무한값을 가리는 위험 | PASS | 기대 finite 값은 actual Some(finite)여야 하며, None은 거부한다고 고정했다. 따라서 span batch 후의 성장 limit과 완료·최대화 결과에서는 non-finite 값이 통과하지 않는다(README 22행). |
| 5 | **허용오차와 정밀도 불일치** — f32 thirds의 정상 rounding이 실패하거나 큰 오류가 통과하는 위험 | PASS | 유리수 ideal 대비 stage 허용치는 absolute 1e-4 CSS px로 작고, final Chrome의 1/64px 허용치와 따로 둔다. expected 값은 53⅓·73⅓에 해당하는 160/3·220/3을 보존한다(계획 25, 37행; README 20, 22행). |
| 6 | **Chrome 양자화를 규범 expected로 재사용** — 1/64 CSS px 직렬화 오차가 stage oracle을 왜곡하는 위험 | PASS | Chrome 155 결과를 최종 geometry 확인에 한정하고, normative ideal thirds와 양자화된 값의 역할을 분리한다(계획 29–31행; README 26–30행). |
| 7 | **초기 geometry 입력이 산술과 어긋남** — gap, content box 또는 definite width가 다른데 같은 expected를 쓰는 위험 | PASS | 고정 입력은 네 max-content maximum column, 0 gap, 220px definite content width이며 기존 helper의 220×40 root와 fixed widths를 재사용한다(계획 17, 35행; 기존 helper [grid_multispan_chrome_oracle_tests.rs 76–129행](../crates/spinon-layout/src/grid_multispan_chrome_oracle_tests.rs#L76)). |
| 8 | **track placement 선후관계 오해** — span line 수를 틀려 겹침·합계가 달라지는 위험 | PASS | A 1→3, B 2→4, C 1→4로 span과 대상 track을 명확히 적었다. 입력의 겹침은 span-2의 track 하나이며 C는 처음 세 track을 덮는다(README 8행; script 16–19행). |
| 9 | **한 batch 안의 item별 증가를 누적** — 동일 span A/B 처리 순서가 결과를 바꾸는 위험 | PASS | span-2 group은 per-item 증가를 track별 최댓값으로 적용한다. script는 그룹 update 전에 각 item의 current state를 읽고 track별 max를 적용한다(계획 17행; script 38–68행). |
| 10 | **span별 batch 순서가 source order에 종속** — C가 A/B보다 먼저 sizing되는 위험 | PASS | 각 case의 expected batch order는 span 오름차순이며 reversed source order에서도 [2] 또는 [2,3]으로 고정한다(계획 27행; script 107–119행). |
| 11 | **span-3 계산에서 선행 그룹 state를 잃음** — C의 extra 40px을 잘못 산출하는 위험 | PASS | span-2 뒤 첫 세 growth limit의 합 160을 입력에 사용하고 C의 200−160을 세 track에 균등 배분한다. script는 첫 group update 후 다음 group을 계산한다(README 17–18행; script 112–118행). |
| 12 | **JSON에 forward/reverse 차이가 숨음** — source order를 이름만 기록하고 동등성을 검사하지 않는 위험 | PASS | 네 controls 모두 source order를 고정하며 script에서 span2-only 쌍과 span2/3 쌍의 stage·completion·maximize 결과를 비교한다(README 9행; script 21–26, 146–153행). |
| 13 | **Initialize stage 또는 track 수 누락** — 후속 snapshot을 일부 트랙에만 대조하는 위험 | PASS | 네 CSS track 각각의 초기 base와 expected unbounded growth trace를 검사하도록 계획했다. JSON도 네 항목 모두를 저장한다(계획 21, 37행; expected.json results 초기 배열). |
| 14 | **IntrinsicItemBatch 후 snapshot 경계 오인** — 다른 stage 값을 batch 결과로 대조하는 위험 | PASS | Taffy는 batch sizing을 처리한 뒤 span 및 crosses-flex 정보를 포함한 IntrinsicItemBatch stage를 기록한다([track_sizing.rs 910–920행](../vendor/taffy/src/compute/grid/track_sizing.rs#L910)). 계획은 모든 batch snapshot과 순서를 검사한다(계획 37행). |
| 15 | **무한 growth 종료 시점 오류** — span 밖 4번 트랙이 completion 후에도 unbounded로 남는 위험 | PASS | span batch까지는 4번 트랙 expected trace가 None, IntrinsicSizingComplete 뒤에는 base size 0인 finite growth로 바뀐다(계획 22–25행). Taffy trace상 완료 stage는 intrinsic resolution 직후 기록된다([track_sizing.rs 424–440행](../vendor/taffy/src/compute/grid/track_sizing.rs#L424)). |
| 16 | **MaximizeTracks의 span2-only 미검증** — span3 케이스 통과만으로 경쟁 item의 maximize 오류를 놓침 | PASS | span2-only control은 최종 base/growth [40,60,60,0]과 free space 60px을 별도로 기대한다(계획 27행; script 175–176행). |
| 17 | **MaximizeTracks의 span2/3 잔여 공간 오류** — track 합은 얼추 맞아도 remaining free space를 잘못 계산 | PASS | ideal 최종 track sum 200px, definite available 220px, 남은 free space 20px으로 계산하고 script가 결과를 assert한다(계획 25행; script 168–174행). |
| 18 | **internal index를 CSS index로 취급** — gutter 슬롯 때문에 잘못된 배열 요소를 비교하는 위험 | PASS | CSS track은 gutter·collapsed가 아닌 항목만 vector order로 추출하고, raw internal_index 0..3을 전제하지 않는다. CSS track count와 flag도 검사하도록 했다(계획 37행). |
| 19 | **기존 oracle / negative controls 범위 이탈** — 중간 trace를 Chrome이 제공한다고 주장하거나 새 제품 경로·flex 지원을 끼워 넣는 위험 | PASS | Chrome은 final geometry만 확인한다. 새 범위는 기존 test assertion이고, fixed-only early-return, malformed placement 오류, non-flex batch count를 negative/control gate로 둔다. flex true batch 및 runtime/app 검증은 제외한다(계획 31, 35, 38–39행). |
| 20 | **구현 승인·완료 게이트 누락** — 20관점 plan review를 PR 30관점이나 구현 검증으로 대체하는 위험 | PASS | focused test·fmt·clippy·산출물 비교, 구현/runtime 새 20관점, PR diff 독립 30관점과 test-only 완료 범위를 각각 명시한다. 또 계획은 plan review 20과 plan PR diff 30을 모두 끝내기 전에 구현을 시작하지 말라고 한다(계획 41–54행). 본 보고서는 plan review 20만 확인했으므로 그 별도 30관점은 선행되어야 한다. |

## 구현 시작 판정

pass02 PARTIAL은 계획/evidence가 serializer projection을 명시적으로 좁혀 해결했다. 이 범위에서 test는 네 track의 normative stage 기대 배열을 trace의 공개값과 비교하고, finite expected에 대해서는 실제 finite 값도 강제한다. internal +∞와 NaN/-∞를 구분하지 않는다는 제한은 이제 숨은 실패 조건이 아니라 명시된 비목표다. 따라서 **내용상 범위 축소는 test-only 구현 시작에 충분하다.**

단, 계획 44–45행의 절차 관문은 별도다. 계획 PR diff 30관점 검토가 아직 보고되지 않았으므로, 해당 검토가 완료되기 전에는 계획 본문의 자체 규칙상 구현을 시작하지 않는다. 이 보고서는 해당 30관점을 대신하거나 통과 판정하지 않는다. 구현 후에는 계획과 다른 runtime 20관점 및 독립 PR diff 30관점이 남는다.

이 pass03은 pass01/pass02의 표를 재사용하지 않고 수정된 계획과 증거를 새로 대조했다. 기대 산출 스크립트 stdout은 committed expected.json과 read-only diff에서 일치했다. 계획·코드와 evidence 본문은 수정하지 않았고, Rust tests/browser/simulators는 실행하지 않았다.
