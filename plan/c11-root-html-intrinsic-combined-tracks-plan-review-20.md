# C11.4 intrinsic explicit columns + min-content implicit text rows 계획 적대 검토 20개

구현 전 계획을 다음 서로 다른 실패 관점으로 검토했다. 변경 코드를 먼저 수정하지 않고 계획의 oracle, admission 및 수용 기준을 보완한다.

| # | 실패 관점 | 계획상 통제와 검증 |
| ---: | --- | --- |
| 1 | Chrome 측정값을 CSS Grid 규범 기대값으로 오인 | §11.1/11.5 계산 규칙과 별도 min/max-content text probe를 먼저 적고 Chrome은 교차 확인 자료로 둔다. |
| 2 | 열 계산 전 폭으로 행 min-content contribution을 계산 | `min-content` column 6.03125px → 6 lines → 60px row 장면을 matrix 핵심 판정으로 둔다. |
| 3 | max-content column을 Grid width 40px로 잘라 text가 wrap된 것으로 처리 | used track/frame 66.234375px와 1-line control을 기록하고 overflow를 보존한다. |
| 4 | auto track과 flexible maximum의 결과를 같은 알고리즘으로 뭉뚱그림 | `auto`와 `minmax(min-content,1fr)` 장면을 분리하고 expected track sizes 및 lines를 각각 비교한다. |
| 5 | Grid item default stretch가 min-content constraint를 덮어씀 | target `width:auto`, `min-width:auto`와 computed/item frame을 같이 검사하고 CSS declaration을 고정한다. |
| 6 | Empty sibling의 zero width가 min-content column을 키움 | fixed sibling 폭을 0으로 고정하고 intrinsic contribution은 text item에서만 나오는지 별도 expected로 확인한다. |
| 7 | Explicit fixed row를 implicit row로 계산하거나 첫 item 배치를 혼동 | 첫 `10px` explicit row와 후속 auto-placement rows를 rows trace에서 구분한다. |
| 8 | Implicit row 반복 pattern 순서/row index가 틀림 | 세 item의 DOM order와 row starts `0,10,70` 또는 gap scene `0,12,74`를 확인한다. |
| 9 | Paragraph shaping line box 11px 관측을 CSS line-height 10px로 오해 | Range line rect는 line count 보조 자료로만 두고 block contribution은 computed `line-height`, Taffy frame과 expected sizing으로 판정한다. |
| 10 | Chrome/macOS monospace metric을 Android/iOS metric으로 강제 | Chrome precomparison과 runtime/system-font evidence를 분리하고 Ahem을 쓰는 Rust unit test는 별도 deterministic gate로 둔다. |
| 11 | Percentage row gap의 basis 또는 used size가 불명확 | `5%` computed value와 y-offset에서 관측한 2px gap을 각각 기록하고 definite 40px container만 허용한다. |
| 12 | Gap이 track base size에 더해지거나 중복 반영 | Tracks와 gutters를 trace에서 분리하고 grid/item frame offsets를 함께 비교한다. |
| 13 | `fit-content()`나 auto-repeat이 새로운 combined mode를 우회 | 기존 explicit intrinsic admission을 재사용하고 fit-content/auto-repeat/line-name negative controls를 둔다. |
| 14 | Named-area projection 또는 implicit column path와 새 gate가 충돌 | Named areas와 implicit columns를 explicit rejection matrix에 두고 기존 positive named-area tests를 regression으로 실행한다. |
| 15 | C11.1 direct-text aspect-ratio route의 mode 우선순위 변경 | ratio-positive/ratio-negative controls로 projection mode가 바뀌지 않고 현재 지원 route만 유지되는지 검사한다. |
| 16 | Stale style/layout revision이 일부 track/frame으로 섞임 | source/node/revision identity를 style snapshot, trace, frame 결과에서 비교하고 stale input은 전체 reject한다. |
| 17 | Text measurement 누락이 빈 item으로 fallback | text source identity와 paragraph measurement가 없거나 변조되면 `layout=ready`/frame/scene이 없는 negative control로 확인한다. |
| 18 | Track quota·checked arithmetic이 combined algorithm에서 무력화 | 기존 10,000-track ceiling과 size/row counts를 경계값 및 10,001 reject로 회귀 확인한다. |
| 19 | Simulator draw/present를 exact pixels 또는 hardware evidence로 과장 | Rust frames, app status, screenshot, PixelCopy/Metal readback을 개별 표기하고 실제 표면 readback이 없으면 visual smoke로만 쓴다. |
| 20 | 제한 integration을 C11.4/C11 완료 또는 공식 counter 증가로 승격 | plan/evidence/status는 bounded slice만 기록하고 C11 checkboxes와 공식 `78/214`를 변경하지 않는다. |

이 계획 review는 계획 PR 변경분 review, 구현/runtime review 및 구현 PR diff review와 별개다.
