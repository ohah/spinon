# C13.3 wheel delta routing 계획 적대 검토 · 2026-10-15

계획을 구현 관점과 분리해, 서로 다른 실패 경로 20개를 사전에 검토했다. 변경 코드가 생긴 뒤에는 이 표를 재사용하지 않고 별도 구현 검토를 작성한다.

| # | 실패 관점 | 계획의 방어/판정 |
|---:|---|---|
| 1 | fixture 밖에서도 새 ABI를 호출할 수 있음 | Cargo fixture-test feature, iOS/Android build define 및 일반 artifact negative control을 요구한다. |
| 2 | native wheel 주입이 아닌데 OS 입력으로 보고됨 | FFI/app log 및 screenshot metadata에 synthetic normalized fixture input을 명시한다. |
| 3 | Chrome diagonal `contain/auto` 결과를 표준 기대값처럼 복사 | Chrome 관찰과 C13.3 per-axis 계약을 분리하고 `(0,40)`을 사전 판정한다. |
| 4 | `touch-action:none`이 wheel을 부당하게 막음 | 별도 fixed scenario에서 inner Y 이동 `(0,40)`을 요구한다. |
| 5 | enqueue point hit-test miss가 빈 target으로 통과 | finite point와 clip-aware hit target 확인 뒤에만 queue admission한다. |
| 6 | dequeue 때 scrolled content에서 target을 다시 찾음 | enqueue target 보존을 고정하고 route-for-target를 사용한다. |
| 7 | surface generation이 재생성 뒤 오래된 sample을 허용 | input generation과 ordered surface lineage를 기록하고 generation barrier 사례를 넣는다. |
| 8 | 새 CSS/style snapshot 이후 옛 policy 사용 | style revision/epoch 불일치 시 reject하고 최신 style로 임의 재계산하지 않는다. |
| 9 | tree/document/environment revision 변경을 scroll-only lineage로 오인 | lineage를 non-scroll render key 전체에 묶고 외부 publication에서 초기화한다. |
| 10 | queue 안에서 먼저 적용된 pan이 wheel revision을 낡게 보이게 함 | accepted input queue scroll revision만 명시적 lineage로 이어준다. |
| 11 | unrelated external scroll이 동일 lineage로 섞임 | 일반 snapshot publication은 lineage barrier가 되며 stale negative control을 둔다. |
| 12 | no-op wheel이 scroll revision을 올림 | offset 불변 입력에서 revision 불변을 별도로 관찰한다. |
| 13 | delta 일부만 적용하고 나머지를 버리거나 이중 소비 | per-axis consumed/remainder 합과 owner offset 변화를 보고한다. |
| 14 | `contain` X가 독립 Y chaining까지 막음 | 대각 sample에서 outer X=0, outer Y=40을 요구한다. |
| 15 | X/Y 대각 delta 순서나 부호를 뒤집음 | 고정 `(40,40)` sample과 양 축 final offset을 정확 비교한다. |
| 16 | NaN/무한대/overflow 값이 부분 상태를 게시 | invalid sample은 queue에서 거부되고 offset/revision 불변을 요구한다. |
| 17 | queue가 source, phase, unit, modifier가 다른 sample을 합침 | 기존 full coalesce key regression과 분리된 sample report를 확인한다. |
| 18 | wheel saturation이 gesture terminal reservation을 손상 | pressure test는 wheel 전체 reject 및 이후 fresh gesture recovery를 요구한다. |
| 19 | Simulator scale/point 값이 CSS 좌표로 오인 | CSS 입력과 coordinate scale/raw unit을 로그에 분리하고 CSS viewport 400×240 고정한다. |
| 20 | 화면/문서가 제품 수준·compositor 결과를 과장 | 앱 실제 surface screenshot, border pixel 비교와 명시된 미완료 gate를 유지한다. |

계획은 이 검토 뒤에도 범위를 고정한다. Android/iOS simulator 주입은 OS native wheel 수집의 증거가 아니며, Chrome 차이는 독립 증거로 남는다.
