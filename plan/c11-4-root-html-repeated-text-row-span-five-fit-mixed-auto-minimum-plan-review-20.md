# C11.4 five-fit row-span mixed minimum 계획 실패 관점 검토 20개

계획의 목표 범위, 독립 기준, matrix 산식, admission 실패 동작과 회귀 경계를 기존 five-fit/#1360 및 six-fit mixed/#1369 구현에 대조했다. 제품 코드는 아직 변경하지 않았다.

| # | 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | 이미 병합된 six-fit mixed-minimum과 중복 구현 | PASS — 이 계획은 정확히 five-fit list만 열고 six-fit은 #1369 회귀 경계로 둔다. |
| 2 | five-fit auto-placement와 row-span owner 혼동 | PASS — 기존 five-fit row-span explicit placement를 유지하고 auto-placement는 제외한다. |
| 3 | all-zero baseline 또는 all-auto endpoint를 누락 | PASS — mask 0과 all-bits-set을 지정하고 기존 uniform observation과 대조한다. |
| 4 | mask bit가 DOM source order에 따라 바뀜 | PASS — bit는 semantic ID A–E/A–D로 고정하며 forward/reverse에서 유지한다. |
| 5 | span2Six에 4-bit 또는 혼합 topology에 5-bit mask 사용 | PASS — topology별 32/16 mask와 대상 ID를 분리한다. |
| 6 | five-fit cap permutation 수를 잘못 계산 | PASS — 6개 fit-position 조합 × 120 cap order × 3 singleton tail = 2,160 list다. |
| 7 | total matrix에서 alignment나 reverse DOM 누락 | PASS — family별 source order 두 값과 alignment 두 값을 산식에 포함한다. |
| 8 | total matrix 수치가 축 정의와 불일치 | PASS — 2,160 × (32×2 + 16×2) × 2 = 414,720로 재계산했다. |
| 9 | blank explicit-row control에 automatic minimum 적용 | PASS — 모든 mask에서 control의 `min-height:0`을 요구한다. |
| 10 | fit-content cap이 item content floor를 clamp한다고 가정 | PASS — §6.6 경계를 명시하고 item-floor 관측을 독립 확인한다. |
| 11 | 모든 row-span item이 automatic-minimum eligible하다고 가정 | PASS — item의 실제 span tracks, overflow, flexible track 유무로 개별 eligibility를 계산한다. |
| 12 | scrollable overflow에서 content-based minimum이 남는다고 주장 | PASS — visible non-scrollable overflow만 허용하고 scrollable overflow는 rejection이다. |
| 13 | track-list의 px와 percentage 타입을 평탄화 | PASS — 0px·10px·20%·30px·40px를 authored type/position 그대로 검사한다. |
| 14 | tail이 없거나 중복인데 accepted | PASS — singleton `min-content|max-content|auto`만 허용한다. |
| 15 | source-order flip에서 semantic content와 placement가 달라짐 | PASS — item ID 기준 frame/row/payload frame 불변을 독립 비교한다. |
| 16 | `align-content:stretch`가 intrinsic tail도 늘림 | PASS — auto tail과 min/max-content tail을 서로 다른 기대 경계로 둔다. |
| 17 | intermediate masks가 uniform endpoint만으로 검증됨 | PASS — 30개 partial masks의 실제 Chrome observations를 전부 수집·독립 검증했다. 414,720 scene 전체와 topology별 mask 수를 확인했다. |
| 18 | rejected revision의 partial frame/trace가 게시됨 | PASS — failure snapshot 부재와 same-session valid revision recovery를 별도 요구한다. |
| 19 | browser oracle를 제품 RuntimeSession 또는 모바일 결과로 확대 | PASS — Chrome, RuntimeSession, Android/iOS smoke의 증거 등급과 한계를 분리한다. |
| 20 | bounded slice 병합으로 C11.4/C11 또는 공식 수치를 완료 처리 | PASS — checkbox, `78/214`, contract `0.1.0` 유지와 일반 CSS 미지원 범위를 명시한다. |

계획 판정: 차단 항목 없음. 후속 Chrome 사전 관측은 414,720개 입력, 기존 uniform endpoint 34,560개의 geometry 차이 0 CSS px, eligible automatic-minimum의 floor violation 0건을 확인했다. `min-height:0`의 floor 미달과 Grid-end 초과도 실제 기준선 결과로 계획서에 반영했다. 범위와 acceptance criteria는 바꾸지 않고 제한 구현 단계로 진행한다.
