# C11.4 repeated-text auto-placement fit-content plan PR · diff 실패 관점 검토 30개

검토 대상: staged plan PR 전체 diff의 계획 문서, 계획 자체 검토 20개, Chrome fixture/NDJSON/검증기/요약/hash manifest/screenshot, `spec/STATUS.md`, 공개 roadmap. 검토는 구현 PR 검토를 대신하지 않는다. 최종 raw observation은 1,440 scene과 metadata 한 줄이며 1.7 MB다.

| # | PR diff 공격 관점 | 깨질 수 있는 부분 | 실제 diff에서 확인한 방어 | 판정 |
|---:|---|---|---|---|
| 1 | 계획 목표가 #1299와 중복 | 이미 병합된 row-span position 2·3을 새 작업으로 다시 기록 | 새 범위는 9-item auto-placement만 대상으로 하고 #1299는 선행 구현으로 연결 | 통과 |
| 2 | 자동 배치와 span topology 혼합 | 3/4-item span이 8 implicit row list를 실제 사용한다고 오인 | span profile은 position 1–3 유지, 새 위치는 9-item profile로 제한 | 통과 |
| 3 | 신규 위치 수량 오류 | 위치 2–6의 새 matrix를 position 1–6 총량과 혼동 | 1,200 new scenes와 1,440 total scenes를 별도 산식으로 문서화 | 통과 |
| 4 | CSS 1-based 위치 삽입 off-by-one | position 6이 마지막 member가 아님 | fixture는 `splice(fitPosition - 1)`을 쓰고 verifier가 매 scene authored list를 재구성 | 통과 |
| 5 | 키워드 개수 유지 실패 | 삽입 구현이 기존 tail member를 덮어씀 | 다섯-member multiset 및 여섯-member 전체 길이를 independent `Counter`로 검사 | 통과 |
| 6 | permutation 중복·누락 | 서로 다른 30개 tail 순서를 재현하지 못함 | verifier가 자체 multiset permutation을 생성하고 unique scene key 1,440개를 검증 | 통과 |
| 7 | matrix 위치 편향 | 일부 fit position에 cap/alignment 조합이 없음 | 위치마다 정확히 240 scene 및 cap 4 × alignment 2를 검사 | 통과 |
| 8 | explicit row가 반복 cycle을 이동 | track position과 row position이 한 칸 어긋남 | explicit row 10px를 분리하고 첫 implicit row부터 six-member cycle을 확인 | 통과 |
| 9 | cycle 두 번째 반복 누락 | 위치 1·2의 row 7·8 사용을 놓침 | 모든 8 implicit row의 fit track을 검사; cycle occurrence 1,920개 | 통과 |
| 10 | 네 번째 위치에 실제 cap 경계 없음 | token이 위치 4에 있어도 content가 0이거나 cap-independent | row 4의 세 줄 payload가 cap 20px에서 20px, cap 44/264px에서 30px로 분리 | 통과 |
| 11 | 위치 5·6의 text contribution이 약함 | cap 0 외에 used row가 0이라 test가 무의미 | 두 줄 payload의 row가 `20px`, `20%`, `120%`에서 20px이고 `0%`에서 0px | 통과 |
| 12 | 과도한 corpus가 Grid 높이를 포화 | track collapse가 새로운 위치의 cap 효과를 감춤 | 36-line 실패 시도를 문서화하고 17-line 최종 corpus를 고정 | 통과 |
| 13 | percent cap basis가 흐림 | 20%·120%를 viewport나 intrinsic dimension에서 환산 | definite 220px block-size와 44/0/264px cap을 계획·verifier에 고정 | 통과 |
| 14 | zero cap에서 minimum floor 오해 | `min-height:0` 전제를 빼고 0px expected를 일반화 | 계획/README/verifier가 이 fixture의 zero floor 전제를 함께 기록 | 통과 |
| 15 | payload max-content 용어 과장 | 관측 line count를 규범 max-content contribution으로 잘못 표현 | diff를 검토 중 `10px × observed line count` 관계로 고쳐 범위를 고정 | 통과 |
| 16 | item frame이 track frame과 다름 | 각 child의 y/height만 검사하고 track 순서 누락 | verifier가 cumulative track y, item 높이, explicit item을 별도로 비교 | 통과 |
| 17 | line count와 payload height 분리 | Range line count는 맞아도 payload가 잘림 | 각 payload 높이를 fixed 10px line-height 곱과 대조 | 통과 |
| 18 | stretch가 fit/min/max에 퍼짐 | align-content가 비-auto row를 바꿈 | 720 start/stretch pair에서 auto row만 남은 공간을 균등 분배하는지 확인 | 통과 |
| 19 | outer overflow 누락 | row collapse로 긴 text가 Grid 경계에서 잘림 | 54개의 payload overflow scene 및 최대 20px overrun 검사 | 통과 |
| 20 | browser 환경이 변함 | 다른 Chromium·DPR·platform 결과를 같은 corpus로 사용 | observation metadata와 verifier가 HeadlessChrome 157, MacIntel, DPR 1을 검사 | 통과 |
| 21 | fixture grammar 밖 문자열 유입 | unicode, 숫자, lowercase, 연속 공백이 positive corpus에 섞임 | expected payload를 고정하고 ASCII byte/word grammar를 재검사 | 통과 |
| 22 | raw observation이 계획만으로 검증 불가 | 산출물을 읽기 어려운 형태로 올려 검토가 막힘 | 303k-line pretty JSON을 제거하고 scene별 NDJSON 1,441줄로 정리 | 통과 |
| 23 | raw output truncation 또는 일부 scene 누락 | capture가 성공해도 저장 파일이 불완전 | metadata cardinality, scene count, unique key, 모든 cross product를 verifier에서 확인 | 통과 |
| 24 | expected data가 capture에서 복사됨 | 브라우저 원시값을 독립 기대값이라고 부름 | README는 관측과 CSS 관계 검증을 분리하고 verifier는 cap/cycle/frame/stretch 관계를 계산 | 통과 |
| 25 | screenshot을 geometry oracle로 사용 | 시각 캡처로 숫자 일치·플랫폼 일치를 주장 | README와 PR body에서 sample screenshot을 visual context로만 표시 | 통과 |
| 26 | hash가 fixture와 맞지 않음 | 캡처 후 입력이 바뀌어 증거가 다른 scene을 가리킴 | fixture·observation·verifier·summary·screenshot SHA-256 manifest를 추가하고 재검증 | 통과 |
| 27 | 지역 문서 link가 끊김 | 계획 또는 증거 링크가 다른 상대 경로에서 해석됨 | plan, review, evidence README의 local Markdown link를 filesystem 기준으로 검사 | 통과 |
| 28 | roadmap의 다음 항목이 계속 generic | 보드 메모가 공식 다음 후보와 다름 | 공개 roadmap heading과 `execution.detailNext`를 동일한 position 2–6 slice로 변경 | 통과 |
| 29 | official status가 구현 완료처럼 바뀜 | C11.4/78/214 카운트가 precomparison만으로 변경 | status는 후보·계획 PR 전·구현 전으로 적고 두 checkbox와 78/214 유지 | 통과 |
| 30 | simulator/browser lifecycle 근거가 현재 상태를 넘음 | 종료된 S03 simulator나 runtime 검증을 했다고 오기록 | 현재 iOS device는 모두 shutdown이며 계획은 이후 작업이 띄운 대상만 종료하도록 서술 | 통과 |

**결과:** staged 전체 diff를 30개의 서로 다른 실패 경로로 검토했다. 발견된 diff 가독성 문제(pretty JSON 크기)와 기술 용어 범위(max-content 표현)는 수정한 뒤 verifier/hash manifest를 다시 생성하고 재검사했다. 현재 계획 PR은 plan-only이며 제품 코드는 변경하지 않았다.
