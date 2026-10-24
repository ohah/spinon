# Fixed-repeat + auto-repeat + template areas 계획 실패 관점 검토 20개

검토 대상은 [`c11-4-fixed-auto-repeat-template-areas.md`](./c11-4-fixed-auto-repeat-template-areas.md)와 Chrome 156 precomparison이다. 각 관점에서 별도 실패 경로를 확인했고, 수정 항목은 계획에 반영했다. 이는 계획 PR diff 30관점 검토나 구현/runtime 검토를 대신하지 않는다.

| # | 실패 관점 | 근거와 계획상 대응 |
|---:|---|---|
| 1 | 기존 거부 guard보다 더 넓게 제거해 fixed-repeat+auto-repeat 교차 축 조합이 열릴 수 있음 | Cascade predicate와 현재 error property를 source audit했다. 같은 track list의 area guard만 대상이고 cross-axis mixed repeat은 negative regression으로 유지한다. |
| 2 | 두 auto-repeat이 같은 track list에 들어가면 반복 수 계산과 alias metadata가 모호해짐 | 기존 parser gate와 negative fixture를 유지하며 두 번째 auto-repeat은 fail-closed한다. |
| 3 | `auto-fill`이 definite container 크기를 초과하는 반복 수로 계산될 수 있음 | 독립 하한 `72 + 24n`과 148/168/188px 경계를 고정했다. n=4/5 경계도 비교한다. |
| 4 | gutter를 제외하거나 fixed-repeat breadth를 한 번만 합산해 repeat count가 달라질 수 있음 | 4 fixed prefix tracks, 2 suffix tracks 및 `(5+n)` gaps를 별도로 산출해 공식에 보존한다. |
| 5 | auto-repeat name group 경계가 fixed-repeat 종료 line과 합쳐지지 않음 | Chrome CSSOM line names와 vendored Taffy resolver를 대조하고 line index 5 seam을 명시 기대값에 기록한다. |
| 6 | fixed-repeat의 여러 authored line-name occurrence를 반복 확장할 때 동일 물리 line이 중복 occurrence로 계산됨 | CSSOM line groups와 Taffy `area_edges_are_merged_with_explicitly_named_lines` test를 참조한다. 합쳐진 distinct physical line list를 validator가 검사한다. |
| 7 | `grid-template-areas`가 만든 alias가 같은 이름의 authored line을 덮어씀 | W3C §7.3.2는 implicit alias가 동일 이름의 추가 line이라고 규정한다. 계획과 scene은 authored-before-area 충돌을 별도로 고정한다. |
| 8 | `grid-area: hero`가 명시된 `hero-start/end`를 무시하는 결과를 Chrome 관측만으로 잘못 채택함 | W3C §8.3의 earlier-authored-line note와 Taffy named resolver를 먼저 판정 기준으로 둔다. Chrome geometry는 교차 관찰이며 validator는 두 위치를 구분한다. |
| 9 | 양수 occurrence의 기점이 explicit grid start가 아니라 repeat start로 구현됨 | `hero-start 3 / hero-end 3`이 first auto-repeat track인 line 5/6을 선택하는 독립 line list와 rect를 기록한다. |
| 10 | 음수 occurrence가 area-added trailing explicit track을 기준으로 잘못 세어짐 | negative occurrence probe는 area/template dimension이 같은 collision scene에 둔다. Area-larger scene은 count semantics와 분리한다. |
| 11 | named area matrix가 auto-repeat expanded template보다 커도 Taffy에 전달되지 않거나 reject됨 | area dimension 10/template 9 case와 `grid-auto-columns/rows:20px`를 positive test로 둔다. 추가 explicit track 및 20px overflow를 검사한다. |
| 12 | area matrix가 template보다 작으면 남은 explicit suffix tracks가 잘리거나 negative line이 달라짐 | area 8/template 10 scene에서 tail을 `-2 / -1`로 두고 9번째 track frame을 고정한다. |
| 13 | area rectangle의 최대 row/column만 사용해 sparse unnamed cell dimension을 잃음 | CSSOM matrix dimension과 named rectangle을 따로 기록하고 area count를 전체 authored matrix로 검사한다. |
| 14 | area가 선언됐다는 이유만으로 auto-fit 반복 track이 점유됨 | `columns-fit-empty`/`rows-fit-empty`는 area만 남기고 repeated item을 뺀다. 네 반복 track과 주변 gutter의 collapse를 검사한다. |
| 15 | spanning area item이 일부 repeated track만 점유했는데 auto-fit이 전부 collapse함 | occupied `hero` area는 four repeated tracks를 전부 가로지른다. 각 20px used track과 92px area frame을 확인한다. |
| 16 | columns path와 rows path의 area axis 전치 또는 line index order 오류 | Collision, auto-fit, larger-area 장면을 두 축에 둔다. Validator는 column/row axis의 area dimensions와 relative rectangles를 독립 비교한다. |
| 17 | cascade는 열리지만 style-to-layout projection에서 template area를 버려 default layout처럼 보임 | `GridContainerInput.template_areas`와 `CompactTaffyStyle` 전달을 확인하고, 9개 runtime scene의 used tracks/frames를 요구한다. |
| 18 | line metadata와 area metadata를 별도 budget으로 세어 합산 1 MiB 제한을 우회함 | 구현 계획에 combined exact/over budget, fallible reservation failure, no-partial-layout과 same-engine recovery를 추가했다. |
| 19 | longhand만 고쳐 `grid-template`/`grid` shorthand 또는 incremental revision에서 stale area/alias가 남음 | Shorthand cascade equivalence와 same-node area→none→area revision regression을 구현 acceptance에 추가했다. |
| 20 | 제한된 Chrome/Simulator fixture를 공개 CSS 지원, WPT pass, hardware/GPU proof 또는 C11 전체 완료로 과장함 | 계약 `0059` 0.1.0, C11.3.3/C11.4/C11 미완료, official `72/213`, Android AVD/iOS pre-compositor 경계를 유지한다. Tailnet preview는 별도 동기화 후 Agent Browser로 확인한다. |

## 검토 결론

계획에는 기준 입력, independent count/line rules, 같은 축 admission 경계, row/column matrices, areas/template 크기 차이, overflow, memory/revision 실패 경계와 플랫폼 evidence가 구분되어 있다. 위에서 발견한 shorthand/revision 및 combined metadata budget 누락을 보강했다. Chrome fixture와 validator는 현재 source를 기준으로 `9 scenes / 30 child boxes` 통과했고, 계획 자체를 추가 수정할 blocker는 찾지 못했다.
