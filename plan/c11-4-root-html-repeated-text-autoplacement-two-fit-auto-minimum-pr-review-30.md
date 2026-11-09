# C11.4 두-fit auto-placement 자동 최소 크기 계획 PR · 적대적 검토 30개

검토 대상은 이 계획 PR의 최종 변경 diff다. 계획 문서 자체에 대한 별도 실패 관점 20개 검토는 [전용 기록](./c11-4-root-html-repeated-text-autoplacement-two-fit-auto-minimum-plan-review-20.md)에 남겼다. 이 30개 검토는 Chrome 비교 산출물, 계획·상태 문서와 공개 roadmap 변경을 별도로 대조하며, 제품 구현 검토로 세지 않는다.

| # | 실패 관점 | diff 대조와 결과 | 판정 |
|---:|---|---|---|
| 1 | 행렬 분모 누락 | 15개 fit 위치 쌍 × 8개 ordered cap assignment × 12개 tail order × 2개 alignment = 2,880 scene으로 계산되며 verifier의 전체 수와 일치한다. | 통과 |
| 2 | fit 위치 조합 누락 | 6개 위치 중 서로 다른 2개를 선택하는 15쌍이 README·verifier에 고정돼 있다. | 통과 |
| 3 | cap 순서 소실 | 8개 cap assignment가 위치가 지정된 순서 있는 입력으로 저장돼 역방향 assignment를 별도 관측한다. | 통과 |
| 4 | tail의 원소 수 또는 순서 변형 | 두 min-content, 한 max-content, 한 auto의 정확한 12개 순서만 계획과 검증기에 있다. | 통과 |
| 5 | percentage basis 모호성 | Grid block size가 definite 220px로 고정되고 직접 percentage cap만 범위에 들어간다. | 통과 |
| 6 | 0px와 0% 병합 | Oracle fixture와 독립 검증이 typed authored value를 보존하고 두 값을 별도 대조한다. | 통과 |
| 7 | 기준군의 입력 불일치 | 기준과 auto-minimum fixture의 차이를 item `min-height`로 한정하고 동일 scene index와 payload를 사용한다. | 통과 |
| 8 | explicit control 오염 | 빈 explicit-row control은 `min-height:0`이며 auto-minimum 변경 대상에서 제외된다. | 통과 |
| 9 | 적용 대상 수 초과 | `min-height:auto`는 여덟 auto-placement direct-text item에만 적용되고 다른 topology의 admission은 명시적으로 제외된다. | 통과 |
| 10 | 텍스트 높이의 불안정 | 고정 monospace 10px/10px, ASCII payload grammar, 줄 수 `[2,2,2,3,2,2,3,1]`을 유지한다. | 통과 |
| 11 | scrollable overflow 조건 오용 | visible overflow 입력과 CSS Grid §6.6의 scrollable-overflow 조건을 연결하고 scrollable 사례로 일반화하지 않는다. | 통과 |
| 12 | 자동 최소 크기 track 전제 오용 | 해당 축에서 auto minimum track을 포함하는 경우만 content-based minimum 범위로 계획했다. | 통과 |
| 13 | fit-content cap를 floor 상한으로 취급 | 0px cap에서 자동 최소 크기 20px인 대조를 포함하고 §6.6에 따라 cap가 floor를 깎지 않는다고 분리한다. | 통과 |
| 14 | floor가 실재하지 않는 장면을 과장 | 0px/0% 기준과 2·3줄 payload를 두어 content floor와 기준 결과 차이를 독립 검증한다. | 통과 |
| 15 | track list cardinality 오류 | 두 fit-content, 두 min-content, 하나 max-content, 하나 auto의 여섯 member를 계획·fixture·검증기가 공유한다. | 통과 |
| 16 | explicit row 뒤 cycle offset 오류 | 10px explicit row 다음 여덟 item의 implicit-row 순환을 관측하고 verifier가 row 순서를 확인한다. | 통과 |
| 17 | authored list 중복 집계 | 1,440개 고유 authored track list를 별도 집계하고 2,880 scene 분모와 구분한다. | 통과 |
| 18 | alignment pair 누락 | 각 authored list에서 start/stretch를 한 쌍으로 비교해 1,440 pair를 만든다. | 통과 |
| 19 | fit row occurrence 분모 혼동 | 두 alignment 합계 7,680, start 한정 3,840 occurrence를 JSON과 독립 verifier로 구분한다. | 통과 |
| 20 | 1,920 floor 사례 범위 과장 | floor 아래였던 기준 occurrence 1,920개는 start-aligned fit rows로 한정하고 전체 track 또는 구현 통과로 표현하지 않는다. | 통과 |
| 21 | start 총 높이 불변 위반 | 1,440개 목록 모두 180px 합계를 verifier가 확인한다. | 통과 |
| 22 | stretch 분모 오류 | stretch 합계 220px, 추가 공간 40px가 관측 결과와 맞는다. | 통과 |
| 23 | 비-auto track의 stretch 증가 | auto row occurrence가 1회면 40px, 2회면 각 20px를 받고 non-auto 최대 변화는 0px다. | 통과 |
| 24 | text overflow를 놓침 | 모든 auto-minimum scene의 text overflow가 0개인지 독립 검증한다. | 통과 |
| 25 | 손상되거나 섞인 evidence | 8개 산출물 SHA256 검증이 통과했고 fixture·collector 해시가 matrix JSON과 일치한다. | 통과 |
| 26 | 브라우저 환경 드리프트 | 관측 metadata가 HeadlessChrome 157, MacIntel, DPR 1, 1280×900을 고정한다. | 통과 |
| 27 | Oracle 화면을 앱 근거로 오인 | PNG와 README가 Chrome 사전 비교 표본임을 표시하고 RuntimeSession·native parity 근거로 주장하지 않는다. | 통과 |
| 28 | 작업용 브라우저·simulator 잔존 | lifecycle 기록은 Agent Browser를 닫았고 Terminal Browser와 simulator를 사용하지 않았다고 기록한다. | 통과 |
| 29 | 계획을 일반 지원 또는 완료로 과장 | one-fit auto-minimum, 임의 CSS, WPT, surface parity와 공식 C11.4 완료를 제외하고 rejection·atomicity는 미래 구현 관문으로 둔다. | 통과 |
| 30 | 상태·roadmap·빌드 결과 불일치 | `spec/STATUS.md`와 공개 roadmap은 구현 전, 78/214, C11.4 미완료를 표시하고 계약 0.1.0을 유지한다. docs build는 1,518 dead-link source 오류로 종료 코드 1이며 통과라고 기록하지 않는다. | 통과 |

**결과:** 서로 다른 PR diff 실패 관점 30개를 확인했다. 검토 중 STATUS의 잘못된 줄바꿈·중복 과거 기록과 계획 PR 검토가 완료된 듯한 문구를 발견해 수정했다. docs build는 실패 상태로 보고하며 Chrome 관측은 제품 구현 완료를 뜻하지 않는다.
