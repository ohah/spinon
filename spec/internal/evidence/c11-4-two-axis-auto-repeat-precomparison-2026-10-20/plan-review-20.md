# C11.4 양쪽 축 auto-repeat 계획 적대 검토 20개

**검토 대상:** [집중 계획](../../../../plan/c11-4-two-axis-auto-repeat.md), 최종 12-case fixture와 Chrome 155 사전 관찰.
**판정:** 계획은 기존 한-axis subset을 보존하면서 각 track list에 auto-repeat 하나씩 허용하는 범위로 작성했다. 아래 실패 경로는 계획 acceptance에 명시되어 구현 전 계획 PR 검토를 기다린다.

| # | 적대 실패 관점 | 계획의 대응·판정 |
|---:|---|---|
| 1 | CSS Grid의 “한 번” 제한을 Grid 전체에 적용하면 columns와 rows 동시 사용을 규범상 잘못 거부할 수 있다. | W3C §7.2.3.1의 단위는 track list다. §7.2.1에서 columns와 rows가 서로 다른 list라고 구분하므로 axis마다 하나를 허용하도록 scope를 썼다. |
| 2 | 두 repeat metadata가 한 list에 둘 이상을 허용하는 방향으로 과잉 일반화될 수 있다. | list별 한 개 제한을 기존 grammar처럼 유지한다. Rust parser/cascade에서 한 list의 복수 repeat 거부 regression을 보존한다. |
| 3 | 두 축이 같은 repeat count를 공유하면 비정사각 grid에서 한쪽 axis 결과가 틀릴 수 있다. | `both-fill-asymmetric`는 200px columns 2회와 266px rows 3회를 독립 oracle로 고정한다. |
| 4 | columns/rows의 `auto-fill`·`auto-fit` mode를 하나의 container mode로 합치면 mixed declaration이 손실된다. | Mixed 방향 두 개의 empty 및 second-repeat cases를 고정한다. 각 typed metadata와 resolved track list를 axis별로 비교한다. |
| 5 | repeat fragment의 두 track이나 inter-track gutter를 반복 계산에서 빠뜨리면 count boundary가 달라진다. | 사전 계산 `30 + 60n + 8(2n + 1) = 38 + 76n`과 200/266px oracle을 사용하고 1/2/3회 경계에 사용된 모든 track을 대조한다. |
| 6 | repeated sizing용 40px minimum과 최종 `1fr` used size를 혼동할 수 있다. | repeat count는 definite 40px minimum을 바탕으로 판단하고, 최종 `45px`·`40px` track size는 별도의 Chrome 및 runtime output assertion으로 본다. |
| 7 | prefix/suffix 고정 neighbor나 첫/끝 gutter를 누락해도 track만 보면 재현을 놓칠 수 있다. | 양쪽 20/10px neighbor 및 8px gap을 fixture에서 고정하고 원 grid frame과 각 used track을 모두 비교한다. |
| 8 | auto-fit의 repeat count와 collapse 이후 track list를 같은 값으로 취급하면 빈 track 개수를 잃는다. | `both-fit-empty`, mixed empty cases에서 serialization된 zero track 수와 axis mode를 따로 확인한다. collapse는 count 산정 뒤 단계로 기록한다. |
| 9 | fit axis만 collapse해야 하는 mixed empty layout에서 반대 fill axis까지 collapse하거나 유지 상태가 뒤바뀔 수 있다. | `columns-fit-rows-fill-empty`와 반대 조합이 축별 대조군을 제공한다. |
| 10 | collapse된 track 옆 gutter를 계속 8px로 취급하거나, 전체 frame에서 모든 gutter를 제거할 수 있다. | Oracle JSON 및 simulator validator에서 track list와 effective active/collapsed gutter를 독립 기록한다. PNG만으로 판정하지 않는다. |
| 11 | item이 두 track을 span할 때 반복 pattern 하나를 점유하지만 다음 repeat를 잘못 살릴 수 있다. | `both-fit-span-two`는 line 2/span 2를 양쪽 axis에서 검증하고 뒤 repeat의 빈 track collapse를 별도로 확인한다. |
| 12 | 두 번째 repeated fragment에 놓인 item의 line index가 prefix/반복 확장/collapse 때문에 한 칸 이동할 수 있다. | `both-fit-second-repeat` 및 두 mixed second-repeat cases가 exact item frame과 각 axis track sequence를 비교한다. |
| 13 | 한 쪽 cascade 값을 계산하면서 다른 쪽 metadata를 덮어쓰거나 무시할 수 있다. | 실제 CSS parser→cascade→typed projection 경로에서 one-axis controls, 같은 mode, 두 mixed mode를 검사한다. typed result를 직접 주입한 layout test만으로 대체하지 않는다. |
| 14 | projection이 axis별 repeat range 대신 두 축의 한 range를 재사용하면 한쪽 component 경계가 훼손된다. | Rust regression은 두 track list의 distinct range start/end와 mode가 typed projection에서 각각 보존되는지 확인한다. |
| 15 | columns의 metadata만 보고 rows를 검사하지 않거나 둘 중 첫 metadata만 검사하면 malformed second axis가 통과할 수 있다. | Layout validation을 axis별로 수행하고 malformed rows와 valid columns, 그 역 조합 모두에서 오류 property와 node를 검사한다. |
| 16 | 두 축 중 하나의 잘못된 range/line-name/placement profile을 다른 축의 유효성으로 가릴 수 있다. | Range, repeated-axis line-name, non-TrackSizing profile을 axis별 negative로 유지하고 성공 시에만 두 독립 Taffy template을 계산한다. |
| 17 | 두 축 지원을 열면서 line names, template areas, integer-repeat 혼합, non-px neighbors까지 허용할 수 있다. | 계획은 기존 fail-closed grammar를 보존하고 해당 조합의 error regression을 요구한다. 다른 CSS Grid feature 완료로 확대하지 않는다. |
| 18 | Taffy를 쓰므로 지원이 자동이라고 가정하거나, vendored pin과 다른 upstream 버전을 근거로 삼을 수 있다. | `spinon-layout`은 정확히 Taffy 0.15.0을 pin한다. 해당 source는 `named.rs`에서 column/row auto repetitions를 별도 읽고 `explicit_grid.rs` test에서 2-column/3-row count를 검증한다. Adapter와 실 runtime 검증은 별도로 남긴다. |
| 19 | implicit auto-placement나 text/image intrinsic contribution이 두 축 계산에 섞이면 단순 track regression의 원인을 분리할 수 없다. | Oracle은 `horizontal-tb`/LTR, 명시 line placement, fixed 12×12 item과 explicit tracks만 쓴다. Intrinsic item contribution 및 implicit track 생성은 이 slice 범위 밖으로 명시했다. |
| 20 | 33개 실행 count만 확인하면 duplicate/missing scene, simulator 증거의 실기기 과장, roadmap drift를 놓칠 수 있다. | Android/iOS acceptance는 기존 21개와 oracle 12개 ID의 중복 없는 정확한 집합을 요구한다. API 37 ARM64 AVD·iPhone 17 Pro iOS 26.2 Simulator만 claim하고 C11.4 count/check를 유지한다. 이후 Tailnet preview를 따로 렌더링 검증하며 Agent Browser process를 닫는다. |

별도로 Chrome 155의 definite minimum-size `auto-fit` axis discrepancy는 이 계획의 200/266px definite preferred-size acceptance에 섞지 않고 기존 집중 조사 문서에 남긴다. 이 계획 검토는 구현 code/runtime 검토나 계획 PR 최종 diff 30관점 검토를 대신하지 않는다.
