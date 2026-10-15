# C13.3 Android AVD touchpad raw-point 구현 검토 · 20개 관점

검토 대상은 Android fixture adapter의 AVD raw-point follow-up, opt-in Gradle source set, uinput negative-control helper와 실제 API 37 AVD 실행이다. 계획 검토 파일을 재사용하지 않았다. 각 관점은 최종 source-set 분리 뒤의 코드와 artifact/log를 확인했다.

| # | 실패 관점 | 확인과 판정 |
|---:|---|---|
| 1 | 기본 APK에 test-only origin marker가 포함되는가 | 초기 inline lambda와 무조건 기록한 MotionEvent 진단에서는 DEX에 marker가 남는 것을 발견했다. helper를 opt-in source set으로 옮기고 진단을 compile-time flag로 묶은 뒤 default APK DEX scan에서 intent key·marker·helper class·새 진단 문자열이 모두 사라졌다. Opt-in APK에서는 세 항목이 확인된다. 결함 수정됨. |
| 2 | Gradle flag와 source set 선택이 어긋나는가 | default build log의 c13_3 flag 0, opt-in build log의 flag 1을 대조했다. source dir은 같은 환경 flag가 1일 때만 추가된다. |
| 3 | helper class가 빠진 기본 빌드에서 reflection이 실행되는가 | main activity 진입은 compile-time BuildConfig flag와 gesture route 및 extra가 모두 참이어야 한다. 기본 build에는 origin intent key와 helper class가 없다. |
| 4 | test-only extra가 C13.3 gesture route 없이 허용되는가 | route allowlist는 C13.3 fixture route 내부에서만 적용되며 별도 route 선택으로 세지 않는다. |
| 5 | fixture extra가 다른 C13 route와 충돌하는가 | 기존 selected-route 수 검사를 유지한다. origin extra는 C13.3 gesture route의 하위 옵션이라 두 번째 route로 오인되지 않는다. |
| 6 | default surface에서 clip 설정이 달라지는가 | clipping 변경은 opt-in helper 안에서만 수행한다. 기본 negative run의 surface origin은 이전과 같은 (15,991)이다. |
| 7 | 입력 event 자체를 고쳐서 queue에 넣는가 | Android input shell이 보낸 raw point (0,0), source 0x100008과 action 8이 log에 그대로 남는다. 이전 cursor 좌표를 재사용하지 않는다. |
| 8 | 화면 좌표를 local 좌표로 바꿀 때 stale origin을 쓰는가 | helper는 layout 때 getLocationOnScreen 결과로 translation을 계산하며 최종 origin을 별도로 기록한다. |
| 9 | density 대신 고정 비율을 써서 실제 surface scale과 다른가 | scale은 measured 1050×630 px와 400×240 CSS px에서 각각 2.625로 구한다. 두 축 일치도 검사한다. |
| 10 | surface가 아직 0 크기일 때 잘못된 변환을 하는가 | width/height가 양수가 아니면 helper는 좌표를 바꾸지 않고 오류 경로로 끝난다. |
| 11 | NaN, infinite 또는 축별 다른 scale을 허용하는가 | Float.isFinite, 양수, 두 축 차이 0.01 이내 조건을 모두 확인한다. |
| 12 | 정수 screen-origin 반올림이 target 허용 오차를 넘는가 | 실제 origin (-347,-294), 계산점 (132.19048,112.0)로 X 오차 0.5 CSS px 이하를 만족했다. |
| 13 | layout listener와 post 초기 실행 순서가 달라지는가 | listener는 layout 변경 후 보정하고 post는 최초 측정 뒤 재확인한다. positive run에서 ready bounds가 target 허용치 안에 있었다. |
| 14 | 재배치로 CSS viewport나 render scene이 바뀌는가 | renderer report는 viewport 400×240 CSS px, surface texture 1050×630 px를 유지한다. test harness의 native view translation만 바뀐다. |
| 15 | 표면 밖 event가 다른 owner에 잘못 전달되는가 | translation 없는 negative control은 point-outside-surface로 reject되며 queue/process와 revision 변경이 없다. |
| 16 | 양성 event가 잘못된 target을 맞히는가 | log는 계산 point 132.19048,112.0, sequence 1, target 7을 기록한다. process 결과는 outer owner 4가 X 64를 소비한다. |
| 17 | source 분류가 장치 지원 source 합집합에 의존하는가 | callback은 event source와 pointer tool을 기록하고 device-supported source bits 0x301을 보조 진단으로 분리한다. |
| 18 | zero/negative axis, Ctrl-wheel 등 기존 reject가 우회되는가 | origin helper는 coordinate mapping만 바꾼다. delta, modifier, native enqueue classifier는 건드리지 않는다. 기존 거부 행렬은 별도 근거로 유지한다. |
| 19 | uinput touch gesture를 native scroll 성공으로 오인하는가 | saved log는 SOURCE_MOUSE ACTION_DOWN/MOVE/UP과 finger tool을 기록한다. ACTION_SCROLL positive gate에 넣지 않는다. |
| 20 | helper 종료 뒤 AVD input device가 남거나 물리 device에 명령하는가 | 모든 ADB 동작은 emulator-5554 명시 선택으로 실행했다. helper device 제거 뒤 dumpsys input 검색 결과가 비어 있다. |

최종 구현 검토에서 미해결 P1/P2는 없다. 확인한 artifact 경계 문제는 opt-in source set 분리로 수정했고 default·opt-in APK를 다시 빌드했다. 실행·hash 자료는 [AVD evidence](c13-3-android-avd-touchpad-point-2026-10-16/README.md)에 있다.
