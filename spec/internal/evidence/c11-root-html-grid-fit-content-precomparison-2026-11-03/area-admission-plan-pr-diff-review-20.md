# C11.1 fit-content named-area admission 계획 정정 PR diff 검토 20개

이번 실제 PR diff(전용 계획, 상위 C11 계획, 현재 source 경계 근거와 계획·PR diff 검토표)를 다음 20개 독립 관점으로 검토했다.

| # | PR diff 관점 | 결과 |
| --- | --- | --- |
| 1 | Chrome의 `"fixed fit"`을 기존 제품 지원으로 쓰는지 | named-area baseline 표에 Chrome 기준이며 현재 Root HTML에서 거부된다고 명시했다. |
| 2 | source 근거가 실제 validator와 다르게 요약되는지 | 현재 exact area profiles와 `project_grid_layout`의 dimension gate를 source와 대조한다. |
| 3 | 상위 C11 index가 다음 slice를 정확히 요약하는지 | explicit tracks, repeat와 explicit track 수가 맞는 area admission을 전용 계획 링크와 함께 적었다. |
| 4 | `fit-content(%)`/`calc()`와 direct percentage를 혼동하는지 | FitContent argument만 거부하며 기존 direct percentage area profile을 유지한다고 분리했다. |
| 5 | `grid-template-areas` dimension이 맞지 않아 implicit track을 만드는지 | 두 axis 모두 explicit track count와 일치하도록 요구한다. |
| 6 | area CSS validity를 직접 다시 파싱하는지 | 기존 parser의 computed rectangular area 출력을 소비하며 parser 변경을 금지한다. |
| 7 | 임의의 area identifier가 하드코딩되는지 | 계획은 name/scene identifier를 성공 분기에서 배제하고 이름을 바꾼 projection 대조를 요구한다. |
| 8 | track validation이 일부만 또는 다른 순서로 적용되는지 | 모든 explicit row/column track은 finite non-negative px/FitContent(px)여야 하고 FitContent가 적어도 하나 있어야 한다. |
| 9 | zero cap 및 잘못된 float 경계가 모호한지 | zero는 허용하고 negative, NaN, infinity는 matrix에서 거부한다. |
| 10 | fit-content가 없는 named area까지 범위가 넓어지는지 | 새 area 경로는 fit-content track을 적어도 하나 요구한다. |
| 11 | 기존 fixed 2×2/fraction/percentage area가 회귀하는지 | 기존 area profiles를 회귀 검증 대상으로 명시했다. |
| 12 | auto-repeat/named lines/implicit area tracks가 잘못 허용되는지 | 기존 fail-closed 경계를 유지한다. |
| 13 | Chrome area case가 columns라 rows가 빠지는지 | projection tests는 row와 column fit-content를 모두 포함하고 runtime row 장면도 유지한다. |
| 14 | 관찰 없이 browser screenshot/oracle를 갱신하는지 | 기존 7-scene Chrome evidence는 그대로 두며 이 PR은 현재 코드 admission 범위만 바로잡는다. |
| 15 | projection 검증을 runtime 근거라고 부르는지 | 계획은 projection, default route, simulator surface를 분리한다. |
| 16 | AVD/Metal 결과를 과장하는지 | AVD software renderer와 compositor 전 Metal 한계를 계속 기록한다. |
| 17 | PR이 제품 코드/API 또는 계약 버전을 바꾼다고 보이는지 | diff는 계획과 근거 경계 문서만 포함하며 source, public API, version은 수정하지 않는다. |
| 18 | 계획 정정으로 status/counter가 올라가는지 | `spec/STATUS.md`를 수정하지 않으며 C11/C11.1과 `72/213`은 유지한다. |
| 19 | 정정 검토가 구현 검토를 대체하는지 | 향후 별도 구현/runtime 20개와 구현 PR 20개 관문을 연결해 둔다. |
| 20 | admission 계획 병합 전에 제품 코드를 시작하는지 | area 정정 PR 병합을 code-entry gate로 둔다. |

이번 계획 PR diff에서 확인한 source/plan 불일치는 반영했으며 제품 구현이나 simulator 결과는 주장하지 않는다.
