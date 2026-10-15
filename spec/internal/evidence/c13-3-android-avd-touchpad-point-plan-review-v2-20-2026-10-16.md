# C13.3 Android AVD touchpad 좌표 계획 보완 검토 · 20개 관점

대상은 [AVD touchpad 후속 계획](../../../plan/c13-3-android-avd-touchpad-point.md)의 ADB 고정 원점 harness 조정이다. 첫 계획의 uinput 양성안은 실제 실행에서 `ACTION_SCROLL`을 만들지 못해 채택하지 않는다. 이 검토는 구현 전 계획 검토이며, C13.3 성공·공식 완료·기기 지원을 주장하지 않는다.

| # | 실패 관점 | 검토와 판정 기준 |
|---:|---|---|
| 1 | opt-in 외 제품 장면에도 surface translation이 적용되는가 | 별도 intent와 C13.3 fixture-test 빌드 조건을 모두 요구한다. 기본 장면은 기존 위치를 유지한다. |
| 2 | translation 없이 실행한 기존 ADB 입력 거부 대조가 보존되는가 | 기존 surface origin `(15,991)`에서 `point-outside-surface`와 queue/process 0을 확인한다. |
| 3 | raw point가 없는 event에서 이전 mouse cursor 위치를 재사용하는가 | event `getRawX/Y()`만 읽고 이전 위치를 참조하지 않는다. 이전 위치 재사용은 계획에서 금지한다. |
| 4 | ADB `input touchpad scroll`가 실제 `ACTION_SCROLL` 대신 keyboard command를 만드는가 | callback의 action·event source·tool을 기록하고 positive gate에 고정한다. |
| 5 | AOSP의 source class가 위치 축 누락을 다시 일으키는가 | ADB 입력은 raw `(0,0)`으로 들어오며 translation은 app view mapping만 바꾼다. event source나 axes를 변조하지 않는다. |
| 6 | raw `(0,0)`이 Android system bar에 막혀 app callback까지 오지 않는가 | handler callback 발생이 첫 양성 관문이다. callback이 오지 않으면 계획은 harness blocker로 남는다. |
| 7 | 음수 SurfaceView screen origin이 `getLocationOnScreen()`에 반영되지 않는가 | 위치를 log로 남기고 계산 결과 CSS point가 `(132,112)`에서 축별 0.5 CSS px 안인지 확인한다. |
| 8 | density와 SurfaceView 실제 크기가 다르지만 `density` 상수로 좌표를 옮기는가 | scale은 측정된 surface pixel 크기와 고정 viewport CSS 크기에서 계산하고 균일성을 검사한다. |
| 9 | translation 반올림이 허용 CSS 오차를 넘는가 | 정수 screen origin·실제 event point·계산 CSS point를 기록하고 0.5 CSS px gate를 적용한다. |
| 10 | translation이 CSS viewport나 frame geometry를 재계산하게 하는가 | translation은 Android view transform에만 적용하고 runtime viewport·CSS scene은 바꾸지 않는다. |
| 11 | target `(132,112)`가 기존 Chrome oracle와 달라지는가 | 동일 C13.3 fixed fixture와 기존 Chrome 155 coordinate oracle를 사용한다. |
| 12 | point가 inner scroll owner 7 밖으로 떨어져 다른 owner를 맞히는가 | runtime hit trace에서 target 7과 owner chain을 직접 요구한다. |
| 13 | `MotionEvent.getSource()` 대신 device 지원 source 합집합을 positive 근거로 쓰는가 | source classification은 event source·action·tool 기반이며 `getSources()`는 진단 필드에 한정한다. |
| 14 | uinput two-finger pan이 trackpad native wheel로 오인되는가 | uinput 관측 `SOURCE_MOUSE`, `ACTION_DOWN/MOVE/UP`, finger tool은 negative control로 분리한다. |
| 15 | H/V axis 부호나 scroll factor가 잘못되어 queue 결과만 보고 통과하는가 | raw axes, platform factor, normalized CSS delta와 Chrome 기대값을 모두 대조한다. |
| 16 | callback 좌표가 맞아도 queue는 거부되거나 다른 sequence를 처리하는가 | sequence 1 admission, process 결과, target 7, revision 2를 모두 확인한다. |
| 17 | 거부 대조에서 화면은 바뀌지만 revision만 유지되는가 | 같은 AVD SurfaceView capture의 changed pixel 0을 요구한다. |
| 18 | helper 종료 뒤 virtual input device가 남거나 touch 상태가 유지되는가 | `UI_DEV_DESTROY`, fd close, `dumpsys input` 제거 상태와 meta state 0을 확인한다. |
| 19 | 실기기·다른 serial에 명령이 전달되는가 | AVD identity를 재확인하고 모든 ADB 동작을 `-s emulator-5554`로 제한한다. |
| 20 | 제한된 AVD 결과가 C13.3·공개 CSS·실기기 완료로 확대되는가 | `spec/STATUS.md` 체크는 그대로 두고 AVD-only harness 결과와 미검증 경계를 명시한다. |

계획 보완은 실패 원인을 보존하면서 고정 원점 ADB event를 검증할 방법을 좁힌다. 구현 뒤에는 새 runtime·코드 실패 관점 20개와 PR 변경 적대 검토를 별도로 수행한다.
