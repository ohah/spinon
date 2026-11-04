# C11.1 혼합 fully definite·one-axis auto placement 계획 실패 관점 검토 20개

대상: [`c11-root-html-grid-mixed-explicit-one-axis.md`](./c11-root-html-grid-mixed-explicit-one-axis.md)와 [고정 Chrome/독립 기대값](../spec/internal/evidence/c11-root-html-grid-mixed-placement-precomparison-2026-11-04/README.md).

| # | 실패 관점 | 판정 | 계획에 고정한 방어 |
|---:|---|---|---|
| 1 | 전부 fully definite인 Grid가 새 혼합 mode로 들어감 | PASS | fully definite와 partial item을 각각 1개 이상 요구한다. |
| 2 | all-auto item이 조용히 기존 mixture에 포함됨 | PASS | all-auto는 새 mode에서 명시적으로 거부하고 기존 control로 둔다. |
| 3 | Row-locked와 column-locked partial item이 한 Grid에 섞임 | PASS | 모든 partial item의 definite axis를 하나로 고정한다. |
| 4 | Fully definite item이 뒤에 있어 partial item보다 늦게 occupancy됨 | PASS | §8.5 preplacement 및 existing sparse normalizer의 예약 단계를 acceptance에 둔다. |
| 5 | DOM order가 계산 frame 또는 paint order에서 변경됨 | PASS | source order·paint order 보존을 별도 계약으로 둔다. |
| 6 | Multi-track span과 기존 full blocker가 겹치는데 통과함 | PASS | row/column span 양성 scene과 occupancy/frame exact 판정을 고정한다. |
| 7 | `visibility:hidden`을 occupancy에서 빼서 overlap 발생 | PASS | hidden full blocker scene에서 occupied span을 건너뛰게 한다. |
| 8 | `display:none`을 occupancy에 포함하거나 candidate가 사라짐 | PASS | selector·occupancy 모두 제외하고 별도 control을 둔다. |
| 9 | Fully definite item의 zero/negative numeric line이 잘못 허용됨 | PASS | existing numeric normalizer 검증을 재사용하고 invalid endpoint를 거부한다. |
| 10 | Negative line이 explicit track 끝 기준이 아닌 임의로 처리됨 | PASS | 기존 Grid line resolution을 유지하며 범위 밖 결과는 거부한다. |
| 11 | `span 0`·overflow 산술·0 cell span이 통과함 | PASS | numeric normalizer의 span/arithmetic errors를 유지한다. |
| 12 | Area/named endpoint가 numeric처럼 강제 변환됨 | PASS | 새 selector는 numeric Line/Span만 받고 named values는 fail-closed한다. |
| 13 | Auto placement가 implicit row나 column을 만들고 partial 결과를 남김 | PASS | explicit extent 한계와 atomic rejection을 요구한다. |
| 14 | Named areas 또는 line names와 경합해 우선순위가 달라짐 | PASS | no-area·no-line-name owner로 제한한다. |
| 15 | Dense·column flow route와 candidate selector가 경쟁함 | PASS | row-flow만 지원하고 dense route를 먼저 제외한다. |
| 16 | Nested Grid item의 inner owner가 outer occupancy를 손상시킴 | PASS | nested Grid item은 지원하지 않고 owner state를 독립 검증한다. |
| 17 | Text·image·replaced item의 intrinsic contribution이 섞임 | PASS | empty non-replaced direct block만 허용한다. |
| 18 | Nonzero gap/edge·position·order가 geometry 판정을 왜곡함 | PASS | 기존 제한 Root HTML style subset을 유지하고 unsupported profile은 거부한다. |
| 19 | sibling owner, stale revision 또는 실패 후 이전 frame 재사용 | PASS | owner/revision identity와 rejection 후 partial/stale output 부재를 acceptance에 둔다. |
| 20 | 기본 artifact에 fixture code가 들어가거나 bounded slice를 C11 완료로 확대 | PASS | default/opt-in artifact 비교와 `72/213`, C11 미완료 유지 문구를 계획에 둔다. |

## 판정

PASS 20/20, 계획 blocker 0개. 이 검토는 implementation 성공이나 실제 PR diff 검토를 뜻하지 않는다.
