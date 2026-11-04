# C11.1 column-flow mixed placement 계획 실패 관점 검토 20개

**대상:** [계획](./c11-root-html-grid-column-flow-mixed-cross-axis.md), [독립 기대값](../spec/internal/evidence/c11-root-html-grid-column-flow-mixed-precomparison-2026-11-04/expected.json), [Chrome capture](../spec/internal/evidence/c11-root-html-grid-column-flow-mixed-precomparison-2026-11-04/capture.json), 현재 `origin/main` `089b76be997aa0114d752be494d318c93eea37df`.

| # | 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | column-flow인데 row-flow의 고정-axis 우선순위를 그대로 적용한다 | **PASS** — 독립 모델은 fully definite 뒤 column-locked를 먼저 놓고 row-locked/auto를 cursor phase에 남긴다. Chrome의 interleaved DOM scene이 이를 교차 확인한다. |
| 2 | sparse cursor를 column 우선으로 진행해 실제 CSS column flow를 반대로 구현한다 | **PASS** — 계획은 한 column에서 row를 진행하고 끝에서 다음 column으로 이동한다고 명시한다. 비정사각형 40×30 tracks가 축 교환을 드러낸다. |
| 3 | column-locked item이 column 위치를 유지하지 못하거나 row-locked item이 row를 잃는다 | **PASS** — 두 class의 definite axis를 별도로 검사하고 expected geometry가 고정 track에서 각각 유지되는지 검사한다. |
| 4 | fully definite item을 partial item보다 나중에 처리해 occupancy 충돌이 달라진다 | **PASS** — fully definite reservation을 첫 단계로 고정하고 DOM에서 나중에 선언한 definite item을 포함하는 oracle scene이 있다. |
| 5 | two-axis span이 점유한 셀을 한 축만 계산해 후속 item이 겹친다 | **PASS** — span scene에서 2-column 및 2-row 점유와 후속 row-locked placement를 모두 비교한다. |
| 6 | 음수 line이 끝 기준으로 해석되지 않거나 양수 line과 축이 바뀐다 | **PASS** — `-2`를 각 explicit axis 끝에서 해석하는 positive scene을 별도 고정하고 rows/columns 값을 서로 다르게 둔다. |
| 7 | implicit track이 필요한 입력을 clamp해 부분 배치 또는 성공 frame을 게시한다 | **PASS** — row/column extent를 각각 요구하는 negative scene을 두고 전체 Grid atomic rejection을 요구한다. |
| 8 | `visibility:hidden`을 occupancy와 mode classification에서 제외한다 | **PASS** — hidden occupancy 장면에 더해 row-locked/column-locked 각 class의 유일한 item이 hidden인 qualification 장면을 각각 추가해 selector와 후속 배치를 고정한다. |
| 9 | `display:none` child가 definite cell을 예약한다 | **PASS** — 숨김 subtree를 candidate classification 전에 제외하고 display-none control에서 해당 cell의 재사용 위치를 고정한다. |
| 10 | sibling 또는 nested Grid의 item이 row/column lock count를 만족시킨다 | **PASS** — selector와 validator는 단일 owner의 direct children만 집계하고 nested/sibling ownership 격리를 요구한다. |
| 11 | authored CSS text나 fixture ID가 mode를 선택해 실제 cascade 값과 어긋난다 | **PASS** — mode 선택은 typed computed snapshot과 owner-scoped item placements에 기반한다고 제한한다. |
| 12 | all-auto column-flow route가 새 mixed mode로 이동해 기존 column count 또는 frames가 변한다 | **PASS** — mixed mode 선택에 두 종류의 partial item을 각각 요구하고 all-auto 회귀를 별도로 둔다. |
| 13 | column-only 또는 row-only partial Grid가 우연히 mixed mode로 admission된다 | **PASS** — selector는 row-locked와 column-locked를 모두 요구하며 단일-axis controls를 새 mode 바깥에 둔다. |
| 14 | 기존 row-flow mixed route에 column-flow 변경이 전파된다 | **PASS** — 별도 placement mode와 분기된 validator/selector를 요구하고 기존 row-mixed matrix를 회귀 gate로 둔다. |
| 15 | `column dense`가 sparse와 동일한 결과라고 처리된다 | **PASS** — computed flow가 정확히 sparse column인 경우만 허용하고 dense는 명시적으로 거부한다. |
| 16 | named line/area 또는 zero/negative span이 숫자 parser 경로에 섞인다 | **PASS** — endpoint grammar를 Auto/numeric Line/positive Span으로 한정하고 unsupported placement를 node/property 오류로 반환한다. |
| 17 | nonzero `order`를 무시한 채 DOM order로 배치해 사용자에게 다른 결과를 준다 | **PASS** — `order=0`만 admission하고 사용자 order가 0인 경우에만 DOM 순서가 order-modified order와 일치한다. |
| 18 | Root width/height, explicit track 합, gap, alignment가 달라 Chrome 좌표와 비교 불가능해진다 | **PASS** — definite root size와 track 합, zero gap/edges, start alignment, LTR/horizontal-tb를 계획에 고정한다. |
| 19 | 오래된 projection/revision 또는 중복 NodeId 결과가 새 owner의 occupancy에 섞인다 | **PASS** — owner/revision 동일성 및 duplicate/missing ID rejection을 계획 테스트와 runtime trace에서 확인하도록 요구한다. |
| 20 | 시뮬레이터/CI 결과를 hardware GPU·실기기·C11 완료로 확대 보고한다 | **PASS** — AVD software renderer, iOS compositor 전 drawable, 빈 CI rollup, 72/213 및 C11 unchecked 한계를 명시한다. |

## 판정

20개 관점을 모두 계획에 반영했다. 미해결 차단 항목은 없다. 계획 PR diff는 별도 검토 문서에서 실제 변경 파일 기준으로 다시 평가한다.
