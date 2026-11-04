# C11.1 mixed explicit/one-axis 계획 PR 실제 diff 적대 검토 20개

검토 대상: 계획 [c11-root-html-grid-mixed-explicit-one-axis.md](./c11-root-html-grid-mixed-explicit-one-axis.md), 별도 계획 실패 관점 [20개](./c11-root-html-grid-mixed-explicit-one-axis-plan-review-20.md), [Chrome 사전 비교 증거](../spec/internal/evidence/c11-root-html-grid-mixed-placement-precomparison-2026-11-04/README.md), [C11 상위 계획 링크](./c11-css-grid.md), 고정 [fixture](../tests/fixtures/css/c11/root-html-grid-mixed-explicit-one-axis-precomparison.html).

| # | 실제 diff에서 확인한 실패 경로 | 판정 | 근거 |
|---:|---|---|---|
| 1 | 상위 C11 계획 링크가 새 문서 경로와 불일치 | PASS | 상위 계획의 새 링크가 추가한 문서를 가리킨다. |
| 2 | 계획 상태가 구현 완료로 오해됨 | PASS | 머리말은 계획 후보이며 구현/runtime을 아직 하지 않았다고 명시한다. |
| 3 | 기존 one-axis 계약이 수정되는 것처럼 서술됨 | PASS | 혼합 사례에 별도 placement mode를 두고 기존 mode 동작을 보존한다고 명시한다. |
| 4 | Root HTML 경계 없이 일반 CSS에 적용될 수 있음 | PASS | Parsed Root HTML, area 없음, row flow, 단일 owner로 제한한다. |
| 5 | fully definite item만 있는 layout도 새 경로로 들어감 | PASS | fully definite item과 partial item이 모두 있어야 한다. |
| 6 | all-auto item이 새 경로에 조용히 들어감 | PASS | all-auto는 기존 경로 대조이며 새 mode에서 제외한다. |
| 7 | row 고정과 column 고정 partial item이 섞임 | PASS | 모든 partial item이 같은 definite 축을 사용해야 한다. |
| 8 | fully definite blocker가 partial 자동 배치 뒤에 놓임 | PASS | §8.5 예약 순서와 sparse 선행 단계를 명시한다. |
| 9 | 숨김 item 의미가 서로 바뀜 | PASS | `display:none`은 제외하고 `visibility:hidden`은 점유한다고 구분한다. |
| 10 | 여러 track을 차지하는 blocker를 놓침 | PASS | row와 column span 장면을 각각 고정 행렬에 둔다. |
| 11 | implicit extent를 제품 성공으로 처리 | PASS | Chrome 기준은 기록하되 제품 계약은 원자적 거부를 요구한다. |
| 12 | 숫자 오버플로·잘못된 span을 허용 | PASS | 기존 numeric normalizer의 한계와 endpoint 오류를 유지한다. |
| 13 | named line/area 입력이 numeric placement로 강제 변환 | PASS | named 값과 area는 selector 승인 범위 밖이다. |
| 14 | dense/column 경로가 잘못 우선 선택 | PASS | row flow로 제한하고 selector 분기를 별도로 둔다. |
| 15 | sibling/nested owner 사이 상태가 새어 나옴 | PASS | owner별 상태를 분리하고 nested Grid item은 제외한다. |
| 16 | 지원하지 않는 content나 box style이 track sizing에 개입 | PASS | 빈 non-replaced block과 edge/gap 0 profile을 명시한다. |
| 17 | Chrome 관측을 제품 결과처럼 게시 | PASS | 증거 문서에서 브라우저는 독립 oracle이며 runtime/WPT와 구별한다고 적었다. |
| 18 | 브라우저 관측 환경이 고정되지 않거나 화면 hash가 낡음 | PASS | UA, viewport, DPR과 fixture/observation/screenshot SHA-256을 기록한다. |
| 19 | 독립 규칙 없이 browser 값을 기대값으로 복사 | PASS | expected JSON은 authored track과 §8.5 순서를 근거로 삼고 verifier가 각 frame을 비교한다. |
| 20 | 계획만으로 공식 C11 완료 수가 올라감 | PASS | C11.1/C11과 공식 `72/213`을 유지한다고 적었다. |

## 판정

PASS 20/20, diff blocker 0개. 이 계획 PR은 구현/runtime 성공을 포함하지 않는다.
