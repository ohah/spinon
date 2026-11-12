# C11.4 mixed minimum runtime 실패 관점 20개

계획 검토 20개와 분리한 구현 후 RuntimeSession 실패 관점이다. 각 항목은 Chrome 157 관측과 실제 runtime frame/trace 또는 rejection 결과를 대조했다.

| # | 적대적 실패 경로 | 판정과 근거 |
|---:|---|---|
| 1 | authored cap order 하나가 빠져 특정 순서만 처리됨 | 720/720 순서, 각 192-scene batch와 metadata 집합을 확인했다. mixed metadata 및 12 matrix chunk 통과. |
| 2 | cap 중복 또는 허용되지 않은 값이 six-fit 목록에 들어옴 | typed profile은 정확히 여섯 값의 집합과 무중복을 요구한다. 720개 순열 수용 및 invalid cap/type/length rejection helper tests 통과. |
| 3 | 20% cap이 definite 220px grid block size에 연결되지 않음 | Chrome 고정 definite basis와 동일 환경을 썼고 720 authored list별 scene matrix가 모두 통과했다. |
| 4 | 다섯 text item topology의 일부 minimum mask가 빠짐 | 5개 항목 topology의 32개 mask를 전부 기록했고 mixed matrix를 통과했다. |
| 5 | 네 text item topology의 일부 mask가 빠짐 | 4개 항목 topology의 16개 mask를 전부 기록했고 mixed matrix를 통과했다. |
| 6 | autoItems와 minimumMask의 bit/id 대응이 어긋남 | record metadata와 scene adapter가 item ID에서 mask를 다시 계산해 기록 mask와 비교한다. 138,240 record 검사 통과. |
| 7 | control item이 auto minimum을 받음 | control min-height 0px를 강제하고 dark media에서 auto로 바뀌는 negative control이 atomic reject됨. |
| 8 | 보이는 span item 하나가 CSS에서 요청한 0px/auto 값을 받지 않음 | 각 item의 computed min-height를 record의 autoItems와 비교했다. 138,240-scene matrix 통과. |
| 9 | 한 mixed item의 trace가 다른 item의 automatic minimum을 물려받음 | 각 item별 Minimum contribution의 NodeId, span, automatic/specified source를 검사했다. |
| 10 | automatic trace가 row-span 적용 요건을 놓침 | auto source에서 spans-auto-min-track, multi-track, non-flexible max track, content-based 조건을 모두 확인했다. |
| 11 | zero minimum item이 content-based trace로 잘못 보고됨 | zero mask item은 SpecifiedMinimum source를 요구한다. 전 scene trace 확인 통과. |
| 12 | trace contribution span이 authored row placement와 다름 | record placement의 span과 같은 item contribution span을 직접 비교했다. |
| 13 | mixed raw row trace를 Chrome final used row와 혼동해 거짓 성공 | mixed trace는 finite/non-negative shape만 확인하고, 결과 적합성은 실제 item/payload frame으로 비교한다. assertion의 경계를 분리했다. |
| 14 | item frame이 Chrome geometry와 다름 | 모든 scene에서 item owner/frame 집합과 각 축 좌표를 Chrome observation에 대조했다. |
| 15 | text payload가 item frame 밖으로 잘리거나 잘못 늘어남 | 모든 payload frame을 Chrome에 대조하고 자동 minimum item의 높이가 payload floor 이상인지 확인했다. |
| 16 | DOM source order를 뒤집으면 결과가 달라짐 | 네 forward/reverse topology에서 같은 cap/mask/alignment의 rows, item frames, payload frames를 비교했다. |
| 17 | align-content stretch가 start 결과를 바꿈 | definite grid size에서 start/stretch 쌍의 rows와 item/payload frame 동등성을 검사했다. |
| 18 | all-zero/all-auto 끝 mask에서 mixed path가 uniform 회귀를 일으킴 | 여섯-fit uniform 11,520-scene control을 별도 binary run으로 통과시켰다. |
| 19 | 잘못된 fixed minimum 또는 control auto가 부분 snapshot을 게시함 | invalid revision에서 Failed/no completed snapshot, control-auto rejection을 확인했다. |
| 20 | 실패 후 session이 영구 손상되어 다음 valid mixed revision을 못 처리함 | 동일 RuntimeSession에서 invalid fixed-minimum 후 valid mixed revision을 Ready/completed로 회복했다. |

실행 세부는 이 디렉터리 README와 mixed-matrix-logs의 각 final 로그에 기록했다. Simulator smoke는 앱 부팅만 확인하며 위의 C11.4 판단 근거로 세지 않았다.

