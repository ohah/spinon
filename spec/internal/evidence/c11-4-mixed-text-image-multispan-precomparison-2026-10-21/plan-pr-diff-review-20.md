# C11.4 mixed text/image multi-span 계획 PR 변경분 적대 검토 20개

| # | 실패 관점 | diff에서 확인한 위험과 판정 |
| ---: | --- | --- |
| 1 | official-state 승격 | STATUS의 상위 C11.4 checkbox가 바뀌면 계획을 구현 완료로 오해한다. 기존 unchecked 상태를 그대로 두고 pending implementation이라고 표시했다. |
| 2 | 계획 링크 누락 | 새 slice가 C11.4 실행 순서에서 고립될 수 있다. `plan/c11-4-grid-track-sizing.md`와 공식 STATUS 양쪽에 같은 계획·사전 비교 링크를 연결했다. |
| 3 | 범위가 기존 slice와 중복 | text/image를 각각 분리 검증한 PR #408/#411을 다시 계획할 수 있다. 이 계획은 동일 Grid/trace에서 두 source 종류를 함께 다루는 공백으로 제한했다. |
| 4 | 규범 원본 버전 이동 | 최신 명세 변경과 기준의 drift가 섞일 수 있다. 날짜가 포함된 2025 CRD URL의 §11.5.1과 배분 알고리즘을 고정했다. |
| 5 | span-2 text expectation | max-content나 item frame 폭을 min-content contribution으로 잘못 쓸 수 있다. Ahem `XX XXXXX` min-content 50px을 고정하고 text frame은 별도 관찰값으로 둔다. |
| 6 | span-2 image expectation | natural width 24px과 CSS height에서 전달되는 inline contribution을 혼동할 수 있다. natural 24×16과 CSS frame 72×48을 나눠 기록했다. |
| 7 | planned increase merge | 같은 span group의 item 기여를 합산하거나 DOM 순서대로 적용하면 overgrowth가 난다. 별도 item 증가의 track별 최대값으로 `[25,36,36,0]`을 계산했다. |
| 8 | span-3 후속 단계 | span-3을 span-2보다 먼저 계산하면 기대값이 바뀐다. 이전 세 track 합 97px, residual 83px, rational `/3` 증가를 명시했다. |
| 9 | quantization 허용치 | Chrome CSSOM 문자열과 독립 rational 값을 동일한 exact oracle로 취급할 수 있다. 두 값을 병렬로 보존하고 1/64px tolerance를 각각 적용한다. |
| 10 | web-font fallback | Ahem 준비 전 측정하면 50px/180px source 조건이 아니다. fixture가 font readiness를 기다리고 observation에 결과를 저장한다. |
| 11 | image decode race | PNG decode 전에 width/height를 읽으면 0×0 sample이 생긴다. decode complete 및 24×16 natural size가 observation에 들어간다. |
| 12 | row-axis 교란 | image height 48px와 20px row가 overlap하거나 row contribution에 영향을 줄 수 있다. 최종 fixture는 세 fixed 60px row를 사용하고 화면에서도 분리된다. |
| 13 | DOM order control 유효성 | 반전 scene에서 style placement가 바뀌면 순서 비교가 아니다. 각 item은 같은 explicit row/column span을 유지하고 child order만 역전한다. |
| 14 | unused track sizing | fourth track에 item이 걸리거나 row auto sizing이 섞이면 0px control이 사라진다. four columns와 exact placement를 고정하고 fourth track을 기대값에 포함했다. |
| 15 | definite root clamp | 220px container가 180px intrinsic span을 축소한다고 오해할 수 있다. root used frame과 used columns를 별도 기록하고 min-content track sum을 비교한다. |
| 16 | runner가 다른 scene 허용 | 임의 source나 topology를 허용하면 네 장면 결과만으로 과장한다. 정확한 scene, count, item type, source/revision, style과 placement를 allowlist할 계획이다. |
| 17 | partial output 실패 | decode/measurement 오류에서 일부 item만 반환할 수 있다. 계획 완료 조건은 기존 전체 오류 경계를 유지하고 부분 `LayoutOutput`을 금한다. |
| 18 | fixture ABI 유출 | 검증 심볼이나 payload가 제품 산출물에 포함될 수 있다. default artifact 부정 대조와 opt-in positive control을 각각 gate로 둔다. |
| 19 | 실행 근거 과장 | Chrome screenshot을 Spinon/device/pixel/WPT 결과로 읽을 수 있다. README·계획은 사전 oracle 한정이며 device 행렬은 구현 후 별도 gate다. |
| 20 | review 관점 재사용 | 구현 review나 PR review를 계획 review로 재계산할 수 있다. 계획 자체 review와 이 PR diff review를 독립 문서로 유지하고 각각 새 관점만 센다. |

## 판정

계획 PR의 상태 표현, source values, normative arithmetic, scene fixture, 해시·링크, artifact 및 실행 관문을 서로 다른 20개 실패 경로로 확인했다. 계획 검토에서 찾은 cross-axis 겹침은 fixed-row 높이를 60px로 바꾸어 해결했고 최종 Chrome observation/screenshot을 다시 생성했다. 현재 PR diff에 구현 완료 주장이나 C11.4 체크 변경은 없다.
