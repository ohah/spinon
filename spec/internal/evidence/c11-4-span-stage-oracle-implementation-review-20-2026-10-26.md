# C11.4 span-stage implementation 독립 검토 · 20관점

**검토일:** 2026-10-26

**대상:** `crates/spinon-layout/src/grid_multispan_chrome_oracle_tests.rs` 변경 및 merged plan `plan/c11-4-span-stage-oracle.md`

**방식:** plan review와 다른 assertion false-positive/false-negative 관점으로 코드 및 Taffy trace boundary를 읽기 전용 대조했다. Rust test, browser, simulator는 실행하지 않았다. 구현자 보고상 focused test 2개, fmt, Clippy, expected JSON 재현 비교는 통과했다. 이 보고서는 그 실행을 재실행한 결과가 아니다.

## 결과

| # | 독립 실패 관점 | 판정 | 근거 및 판단 |
|---:|---|:---:|---|
| 1 | **기존 oracle 입력이 helper refactor로 달라짐** — test 대상의 definite width, row placement 또는 placement mode가 바뀌어 기존 Chrome 결과를 다른 입력에 적용 | PASS | `multi_span_input`은 track 목록과 placement mode를 매개변수화했지만 공통 root 220×40, definite item width/height, row·column placement를 유지한다([코드 71–135행](../../../crates/spinon-layout/src/grid_multispan_chrome_oracle_tests.rs#L71)). 기존 positive case는 계속 `TrackSizing`을 사용한다(146–147행). |
| 2 | **잘못된 root trace 선택** — 다른 Grid node의 stage를 root oracle로 검사 | PASS | trace 선택은 root `NodeId`와 일치하는 항목을 사용하고 누락 시 case ID를 출력한다(151–155행). fixture builder는 단일 Grid root만 만든다(48–68행). |
| 3 | **복수 Columns pass 중 첫 pass만 검사** | PASS | Columns pass를 모두 수집해 정확히 하나인지 assert한 뒤 사용한다(156–167행). |
| 4 | **중복 snapshot을 첫 항목으로 숨김** — 같은 Initialize/batch/completion/maximize가 둘 이상 기록되어도 일부만 검사 | PASS | `unique_stage`는 predicate 일치 결과가 정확히 하나여야 반환한다(299–314행). |
| 5 | **기대와 실제 span batch 순서가 다름** | PASS | 기존 `[2]`/`[2,3]` expected 배열과 모든 관측 batch span을 순서대로 비교하고(199–211행), 별도로 batch 수와 selector를 검사한다(212–238행). |
| 6 | **flex batch가 non-flex oracle에 섞임** | PASS | 각 batch의 전체 stage kind를 `IntrinsicItemBatch { span, crosses_flexible_track: false }`와 비교한다(228–236행). 추가·누락된 batch도 count에서 실패한다(222–227행). |
| 7 | **초기 base가 NaN·다른 크기인데 통과** | PASS | Initialize는 네 CSS track의 base를 finite 값으로 확인한 뒤 0px 기대값과 1e-4px 허용치로 대조한다(393–401, 345–354행). |
| 8 | **선택된 stage의 상대 순서가 바뀌어도 통과** | PASS | 고유 Initialize, 각 span batch, IntrinsicSizingComplete, MaximizeTracks index가 strictly 증가해야 한다(403–453행). |
| 9 | **trace internal index를 CSS track 순서로 오인** | PASS | raw `internal_index`를 기대하지 않고 gutter/collapsed flag로 CSS track을 거른 뒤 vector order를 비교한다(335–344행). 비대칭 첫 track과 마지막 track이 위치 오류를 구분한다. |
| 10 | **CSS track 누락·추가가 count에서 숨음** | PASS | 각 stage의 추출 CSS track 수가 기대 길이 4와 일치해야 한다(340–344행). collapsed non-gutter track도 먼저 거부한다(328–334행). |
| 11 | **finite base의 overflow/NaN** | PASS | 모든 CSS track base가 finite여야 하며 expected base와 1e-4px 이내인지 검사한다(345–354행). |
| 12 | **finite growth limit의 NaN/∞ 또는 오차** | PASS | `Some(expected)`에는 `Some(finite actual)`만 허용하고 값 차이를 1e-4px 이내로 제한한다. `None`/`Some` 혼동은 panic한다(356–372행). |
| 13 | **unbounded trace를 내부 +∞ 증명으로 과장** | PASS | `None`/`None` 대조는 trace projection만 확인하며 NaN·±∞를 구별하지 않는다는 코드 주석이 있다(356–360행). 이는 승인 plan의 제한과 일치한다([plan 37행](../../../plan/c11-4-span-stage-oracle.md#L37)). |
| 14 | **span-2 overlap에서 source-order 누적 합산** — track 2에 40+60을 더해 100으로 오산 | PASS | span-2 snapshot은 `[40,60,60,None]`을 요구하므로 track별 planned increase의 최댓값 규칙을 검사한다(418–431행). forward/reversed controls도 유지한다(513–525행). |
| 15 | **span-3 batch가 span-2 적용 전 상태를 사용** | PASS | span-3 stage는 `[160/3,220/3,220/3,None]`을 요구해 이전 batch의 growth-limit 합 160px 반영 여부를 검사한다(418–432행). span-2/3 순서 controls가 양방향이다(526–537행). |
| 16 | **span 밖 track의 completion 처리 누락** | PASS | IntrinsicSizingComplete는 네 번째 track growth를 finite 0으로 닫고, 세 기여 track은 기존 finite limit을 유지해야 한다(434–467행). |
| 17 | **Maximize 결과가 맞아 보여도 남은 free space 오류** | PASS | span-2 전용 및 span-2/3 기대 final arrays와 남은 60px/20px을 별도 비교한다(455–485행). 앞선 gutter-base 검사로 zero-gap 합산 가정도 고정한다(375–385행). |
| 18 | **zero-gap gutter가 크기를 더해도 놓침** | PASS | 존재하는 각 gutter base가 finite 0px인지 모든 검사 stage에서 대조한다(375–385행). |
| 19 | **gutter에 collapsed flag가 잘못 설정돼도 통과** | PARTIAL | collapsed 여부는 non-gutter track에만 제한한다(`is_gutter || !is_collapsed`, 328–334행). 이 입력에는 auto-fit/collapsed track이 없으므로 모든 trace entry에 `!is_collapsed`를 기대해도 안전하다. Gutter가 collapsed로 잘못 표시되어도 현재 검사는 통과할 수 있고 plan은 `is_gutter`/`is_collapsed` flag 검사를 요구한다. 모든 entry의 flag를 직접 assert하는 보완을 권한다. |
| 20 | **negative control이 실패 경계를 실제로 실행하지 않음** — fixed-only가 intrinsic batch를 만들거나 malformed placement가 성공 출력 반환 | PASS | fixed-only case는 정확히 하나의 Columns pass에서 FixedTrackEarlyReturn 1개와 batch 부재를 검사한다(551–591행). 같은 start/end line은 `Existing` placement mode의 normalizer를 지나 `InvalidGridPlacement` error를 요구해 성공 LayoutOutput은 반환할 수 없다(593–620행). |

## 남은 비차단 구조 검토

- 변경 후 파일은 621행이며 기준 파일은 305행이었다. 새 helper와 기존 multi-span oracle이 같은 책임을 공유하지만, 프로젝트 규칙의 300–500행 권장선을 넘는다. 별도 test submodule로 옮길지 검토하거나 PR 설명에서 단일 파일을 유지하는 이유를 밝혀야 한다([project rules 30–31행](../../../docs/project-rules.md#L30)). 이 항목은 계산·assertion correctness blocker는 아니다.
- 테스트는 `MaximizeTracks`까지의 plan 지정 stage와 final Chrome frame을 확인한다. 이후 `ExpandFlexibleTracks`/`StretchAutoTracks`의 growth-limit snapshot은 이 slice의 expected scope에 들어 있지 않으며, flex/auto stretch 일반 검증으로 확대하지 않는다.

## 판정

**PASS 19 / PARTIAL 1.** 표준 수치, stage selector/order, finite comparator, source-order controls, 기존 contribution/frame 회귀와 negative control에서 구현 시작을 막는 correctness blocker는 확인하지 못했다. PARTIAL은 무-auto-fit 입력에서 gutter collapsed flag를 직접 검사하지 않는 단일 assertion 경계다. 파일 길이는 코드 책임 분리 또는 PR 설명으로 처리할 non-blocking 구조 검토다. 수정한다면 19번 관점을 재확인해야 한다.
