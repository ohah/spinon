# C11.1 Root HTML 한 축 고정 배치 계획 실패 관점 검토 20개

대상 계획: [`c11-root-html-grid-one-axis-auto-placement.md`](./c11-root-html-grid-one-axis-auto-placement.md)

| # | 실패 관점 | 검토와 계획의 대응 |
| --- | --- | --- |
| 1 | bounded slice가 C11 전체 완료로 과장되는가 | 첫 문단과 병합 후 단계가 C11.1/C11 및 `72/213` 유지 명시 — 통과 |
| 2 | generic `SparseRow` 제품 계약이 함께 넓어지는가 | Root HTML 전용 mode를 요구하고 기존 profile 유지 — 통과 |
| 3 | computed style 대신 authored declaration을 재해석하는가 | typed computed values와 root/document/revision identity를 사용 — 통과 |
| 4 | Grid owner별 candidate가 다른 Grid의 상태와 섞이는가 | owner별 visible child 집합을 분리하고 sibling isolation을 검증 — 통과 |
| 5 | 한 Grid에서 row-locked와 column-locked item이 섞이는가 | 동일 definite 축만 허용하고 mixed-axis scene을 거부 — 통과 |
| 6 | fully definite item이 partial item과 섞이는가 | 양 축 definite 및 mixed fully/partial을 명시적으로 거부 — 통과 |
| 7 | all-auto 기존 동작이 새 mode에 흡수되는가 | all-auto control은 기존 route에 남긴다 — 통과 |
| 8 | 한 축의 endpoint 조합이 모호하거나 span만으로 배치되는가 | numeric line/span 조합과 양수 span 조건을 validator에서 확인 — 통과 |
| 9 | negative line이 explicit grid 기준이 아닌 implicit 시작 line로 오해되는가 | 양·음수 numeric line을 고정하고 범위 밖은 거부 — 통과 |
| 10 | line/span 산술 overflow가 잘못된 frame을 만드는가 | 계산 overflow와 invalid span을 오류로 귀속하고 atomicity를 확인 — 통과 |
| 11 | definite placement가 explicit track 범위를 넘어가는가 | 범위 검사와 implicit extent 거부를 별도로 요구 — 통과 |
| 12 | auto placement가 새 implicit row를 만들고 부분 결과를 남기는가 | 결과에서 expanded track 부재를 검사하고 publish 전 거부 — 통과 |
| 13 | auto placement가 explicit column을 넘어가는가 | implicit column negative scene 및 explicit extent 검사를 요구 — 통과 |
| 14 | `grid-auto-*` 고정 sentinel이 새 implicit track을 허용하는가 | 선언값과 관계없이 implicit extent 필요 시 거부 — 통과 |
| 15 | unsupported intrinsic child가 track sizing에 섞이는가 | empty non-replaced direct block만 허용하고 text/replaced는 거부 — 통과 |
| 16 | display/visibility 차이로 occupancy가 틀리는가 | 규칙을 정하고 제품 코드 전 Chrome subscenes 및 candidate/normalizer 검증을 요구 — 통과 |
| 17 | nonzero order나 positioned child가 cursor 순서를 바꾸는가 | order 0·static 조건과 source/paint/accessibility order 보존을 명시 — 통과 |
| 18 | template area, named line, repeat 또는 flow 변형이 우회하는가 | 각 기능을 fail-closed하고 별도 scope로 유지 — 통과 |
| 19 | Chrome 관측이 제품·WPT 성공 근거로 오해되는가 | evidence와 계획에서 browser oracle, runtime, WPT를 분리 — 통과 |
| 20 | 플랫폼 readback 차이와 fixture artifact 오염이 가려지는가 | Android/iOS 근거 분리, 기본·opt-in artifact 대조 및 미검증 한계 명시 — 통과 |

검토는 계획 문서만 대상으로 했다. 구현/runtime 결과와 PR diff 검토 횟수로 재사용하지 않는다.
