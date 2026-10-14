# C13.1 전체 viewport hit 구현 실패 관점 20개 · 2026-10-14

검토 대상은 `full_viewport_hit.rs`, pinned Chrome observation과 capture validator다. 구현은 새 CSS 동작이 아니라 host renderer의 기존 hit-test 동작을 고정 oracle과 비교한다.

| # | 실패 관점 | 결과와 근거 |
|---:|---|---|
| 1 | viewport 면적을 행 수나 target 합계로 축소한다 | PASS — 1,280×1,100 모든 좌표를 순회하고 총 1,408,000점을 assert한다. |
| 2 | CSS px와 device px 좌표계를 혼용한다 | PASS — DPR 1을 oracle/test에서 확인하며 정수 CSS 좌표만 조회한다. |
| 3 | Chrome의 scroll offset을 Rust 원점으로 간주한다 | PASS — `scrollX/scrollY == 0`을 observation에서 고정한다. |
| 4 | Chrome build 변경을 같은 oracle로 취급한다 | PASS — 버전, revision, UA를 test assertion으로 고정한다. |
| 5 | fixture 변경 뒤 과거 observation을 재사용한다 | PASS — fixture SHA와 이전 target-frame observation SHA를 모두 확인한다. |
| 6 | 13개 case 중 하나가 순서 변경/누락된다 | PASS — ordered case ID와 target ID 13개를 각각 검사한다. |
| 7 | Chrome frame이 다른 좌표인데 identity만 비교한다 | PASS — section·box·target·inner frame 각 축 차이가 0.5 CSS px 이내인지 먼저 확인한다. |
| 8 | section 여백 또는 제목을 잘못 parent identity로 분류한다 | PASS — Chrome `case`와 Rust section `NodeId`를 별 identity로 비교한다. |
| 9 | clip box padding/border hit를 target으로 오인한다 | PASS — `box` identity를 모든 viewport run에서 exact `NodeId`로 대조한다. |
| 10 | overflow visible target이 clip 사례보다 일찍 잘린다 | PASS — `visible` 전체 target 도달 구간을 Chrome RLE와 점별 비교한다. |
| 11 | clip-x가 y축까지 제한하거나 clip-y가 x축까지 제한한다 | PASS — 두 단일축 사례의 전체 점별 identity 지도가 포함된다. |
| 12 | clip margin reference box가 틀려 padding/content/border 결과가 섞인다 | PASS — 세 reference case의 독립 target/box/root identity를 비교한다. |
| 13 | hidden이 clip margin을 적용한다 | PASS — `clip-margin-hidden` 전체 viewport identity가 pinned Chrome과 비교된다. |
| 14 | mixed visible/hidden 및 clip/auto 정규화를 놓친다 | PASS — 두 mixed axis case를 별도 scene node와 oracle ID로 유지한다. |
| 15 | positioned-contained absolute child가 containing block clip을 벗어난다 | PASS — target frame과 frame 밖 query 모두 전체 grid에서 비교된다. |
| 16 | positioned-escape absolute child가 static clip box에 갇힌다 | PASS — section containing block 아래 target ID를 정확히 확인한다. |
| 17 | nested clip inner가 parent 또는 target과 같은 identity로 접힌다 | PASS — `inner` node/frame을 별도 ID로 유지하고 unique identity 집합을 assert한다. |
| 18 | 잘못된 margin/float row를 성공 scene 범위로 과장한다 | PASS — 13개 valid row만 Rust scene이며 나머지 DOM은 Chrome root bucket으로 명시한다. |
| 19 | viewport 바깥 null control을 full-grid 좌표 수에 포함하거나 생략한다 | PASS — 내부 1,408,000점과 별개 네 점을 저장하고 Rust `None`을 확인한다. viewport 내부에는 null이 없으므로 control을 별도 채집했다. |
| 20 | 많은 hit sample을 surface/event/WPT 또는 C13 완료 증거로 오해한다 | PASS — 문서와 상태 대장은 host query 범위 및 남은 gate를 구분한다. |

초기 계획은 내부 grid에 실제 Chrome null이 있다고 가정했으나 viewport root가 전 영역을 덮어 표본이 없었다. 계획/validator를 고치고 네 방향 바깥 Chrome control을 같은 1280×1100 화면에서 다시 채집해 Rust `None`으로 비교했다. 구현 관점 재검토에서 코드 P1/P2는 남지 않았다.
