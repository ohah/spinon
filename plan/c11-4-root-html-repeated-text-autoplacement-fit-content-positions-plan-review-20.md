# C11.4 반복 text auto-placement fit-content 위치 계획 · 실패 관점 검토 20개

검토 대상: [제한 구현 계획](./c11-4-root-html-repeated-text-autoplacement-fit-content-positions.md), [Chrome 사전 비교](../spec/internal/evidence/c11-4-root-html-repeated-text-autoplacement-fit-content-positions-precomparison-2026-11-09/README.md), 1,440 scene NDJSON observation, 독립 검증기, 최종 fixture. 이 검토는 실제 구현/runtime 검토나 계획 PR diff 검토를 대신하지 않는다.

| # | 실패 관점 | 깨질 수 있는 계약 | 확인한 방어·근거 | 판정 |
|---:|---|---|---|---|
| 1 | CSS 위치와 zero-based 배열 index 혼동 | `fit-content` 위치 2를 세 번째 implicit row로 계산 | Fixture가 1-based position에서 `splice(position - 1)`을 사용하고 verifier가 authored 순서를 재구성 | 통과 |
| 2 | list 삽입을 기존 member 대체로 구현 | 여섯-member list가 다섯 개 또는 다른 multiplicity로 변함 | 모든 scene에서 list 길이 6과 fit/min/max/auto 개수를 exact 검사 | 통과 |
| 3 | 동일 tail permutation 중복 | 30개 고유 순서 중 빠진 값을 중복 scene이 숨김 | Multiset permutation을 독립 생성하고 전체 key 고유성·30개 coverage를 검사 | 통과 |
| 4 | fixture가 선언된 list 순서를 잃음 | CSSOM 계산 과정이 member 순서를 바꿔도 통과 | 각 scene의 authored와 computed `grid-auto-rows` order를 비교 | 통과 |
| 5 | explicit row를 auto-row cycle에 포함 | fit 위치와 실제 implicit track index가 한 칸 어긋남 | explicit 10px row 뒤의 8개 implicit row만 six-member cycle로 검증 | 통과 |
| 6 | 마지막 cycle 반복 누락 | 위치 1·2의 일곱째·여덟째 implicit track을 검사하지 않음 | 8행 모두를 순회해 fit 위치당 1,920 cycle 발생을 점검 | 통과 |
| 7 | 위치 4–6이 실제로 sizing되지 않음 | list token만 옮기고 해당 row contribution이 0으로 남음 | 위치 4의 세 줄 text가 20px와 30px를 분리하고 위치 5·6은 두 줄, cap 0은 0px로 변함 | 통과 |
| 8 | 과도한 text가 Grid sizing을 포화 | 모든 cap 차이가 track collapse에 묻힘 | 첫 36-line trial을 폐기하고 17-line corpus로 수정; 여섯 위치 모두 cap 반응 확인 | 통과 |
| 9 | percentage basis 오용 | `20%`와 `120%`가 viewport나 intrinsic size에서 계산됨 | definite 220px block-size에서 44/264px로 고정; Chrome used row와 independent cap 관계 검사 | 통과 |
| 10 | zero percentage를 잘못된 floor로 해석 | `0%`에서 min-height floor를 누락하거나 min-content floor를 강제 | 정확한 item style은 `min-height:0`; 관측된 fit row가 모든 위치·cycle에서 0px | 통과 |
| 11 | fit-content cap을 항상 상한으로 취급 | content contribution보다 작은 used size에 minimum floor가 필요한 경우를 놓침 | 현재 제한 fixture만 다루고 `min-height:0` 전제와 공식 Grid 관계를 명시; 일반 fit-content로 확대 주장하지 않음 | 통과 |
| 12 | 큰 cap이 authored value 그대로 used size가 됨 | `120%`에서 contribution 높이보다 큰 row를 생성 | 독립식 `min(payload height, cap)`으로 검증; position 4에서 30px contribution이 264px cap을 이김 | 통과 |
| 13 | item frame과 row trace의 불일치 | row size와 item frame이 서로 다른데 text만 맞음 | 모든 item의 y/height가 cumulative used row track과 일치하는지 0.01px 허용치로 검사 | 통과 |
| 14 | payload line count와 height 불일치 | line wrapping 관측만 맞고 실제 payload frame은 다름 | 고정 Chrome에서 text Range line count를 관측하고 payload 높이와 `10px × line count`를 비교 | 통과 |
| 15 | stretch가 intrinsic track을 변경 | `align-content:stretch`가 fit/min/max row에 공간을 배분 | 720 start/stretch 쌍에서 auto row만 남은 공간을 균등 배분하는 독립 관계 검사 | 통과 |
| 16 | visible overflow가 clip 또는 item frame과 혼동 | text payload의 부모 Grid 바깥 출력이 숨겨짐 | Grid와 item은 overflow visible이며 54 scene·최대 20px payload overrun을 별도 검사 | 통과 |
| 17 | text grammar와 font 환경이 이동 | 지원 범위 밖 Unicode/inline/fallback이 결과에 섞임 | 1–32 ASCII bytes, 대문자 단어·한 칸 공백, monospace 10px line-height 고정; Chrome build/DPR 기록 | 통과 |
| 18 | row-span profile까지 잘못 확장 | 네 번째 row를 만들지 않는 span profile에서 위치 4–6을 허용 | 계획은 9-item auto-placement predicate만 확장하고 span profile의 위치 1–3 경계를 고정 | 통과 |
| 19 | 브라우저 결과를 제품/네이티브 근거로 과장 | Chrome 사전 비교가 Spinon이나 모바일 동작 검증으로 표현됨 | README·계획은 browser precomparison과 future Android/iOS default-route gate를 분리 | 통과 |
| 20 | 작업 뒤 simulator/browser가 남음 | 새 검증 프로세스·device가 사용자 세션을 점유 | 기존 상태를 먼저 저장하고 이번 검증에서 시작한 AVD/Simulator/Agent Browser만 종료·확인하도록 완료 관문에 포함 | 통과 |

**결과:** 서로 다른 20개 계획 실패 관점을 검토했다. 첫 시도의 36-line corpus가 위치 4–6에서 cap을 구분하지 못해 교체했고, 최종 matrix는 모든 위치를 cap-sensitive하게 만든다. 계획 PR diff 검토 30개와 구현/runtime 검토 20개는 각각 별도로 수행한다. 제품 코드 변경 전 gate는 충족했다.
