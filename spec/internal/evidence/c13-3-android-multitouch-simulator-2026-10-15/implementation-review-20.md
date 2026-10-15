# Android evdev 멀티터치 harness 구현 실패 관점 20개

검토 대상은 이 디렉터리의 `android-evdev-multitouch.sh`와 고정 API 37 AVD 입력 경로다. 앱의 C13.3 runtime 코드는 수정하지 않았다.

| # | 공격 관점 | 확인과 결과 |
| --- | --- | --- |
| 1 | 실기기 serial로 실행 | serial 형식이 `emulator-*`가 아니면 ADB 호출 전에 exit 2. `physical-unauthorized` negative control 통과. |
| 2 | 이름만 emulator인 다른 system image | AVD 이름과 API 37을 동시에 고정 확인. 다른 이름/API는 주입 전에 거부하도록 구현했다. |
| 3 | 잘못된 CPU ABI | `arm64-v8a`를 요구한다. 이번 실행 AVD에서 일치했다. |
| 4 | evdev 쓰기 권한 부족 | uid 0을 요구한다. 비-root 시도에서 SELinux `input_device` write denial을 확인했고 AVD에만 `adb root`를 적용했다. |
| 5 | shell 문자열을 넣은 input path | `/dev/input/event[0-9]+` 외 경로는 차단. `;touch /tmp/...` negative control에서 파일 side effect가 없었다. |
| 6 | virtual-display event node 오선택 | `dumpsys input`의 EventHub→InputReader mapping을 교차 검사한다. event11 `DISABLED`/virtual display control이 exit 2로 차단됐다. |
| 7 | display mode/크기 불일치 | `DIRECT`, displayId 0, 1080×2400을 요구. event1이 세 조건을 통과했다. |
| 8 | 이전 touch stream 잔존 | 주입 전 `Last Cooked Touch: pointerCount=0`을 요구. 최종 종료 후도 pointerCount 0이었다. |
| 9 | 앱이 foreground가 아님 | `topResumedActivity`가 Spinon MainActivity인지 확인. 테스트 동안 해당 Activity가 top이었다. |
| 10 | feature-off 또는 잘못된 app fixture | 현재 process log의 `SPINON_C133_READY input=MotionEvent`가 없으면 중단. opt-in fixture가 준비된 상태에서만 진행했다. |
| 11 | 두 contact가 서로 다른 device에서 옴 | 같은 `/dev/input/event1` type-B slot 0/1에 tracking ID 201/202를 넣었다. evdev trace와 InputReader raw state가 둘 다 확인했다. |
| 12 | 불완전한 kernel event frame | down/move/up 각 frame에 `SYN_REPORT`가 있다. raw evdev trace에 각 frame 경계가 기록됐다. |
| 13 | 두 번째 contact가 pan보다 먼저 도착 | 첫 `ACTION_DOWN` 뒤 native slop 21 px를 넘는 `PanBegan`과 revision 1이 먼저 기록됐다. |
| 14 | event5가 실제 두 번째 contact가 아닌 경우 | 앱 `event_action=5`와 동시 InputReader `pointerCount=2`/pointer id 0,1을 함께 확인했다. |
| 15 | pointer-down이 cancel을 유실 | `SecondPointer` cancel enqueue와 worker terminal `Cancel(SecondPointer)` 결과가 모두 로그에 있다. |
| 16 | cancel 뒤 두 contact가 겹친 동안 이동 수락 | 겹친 상태에서 move 2개 모두 `accepted=false`. |
| 17 | 두 번째 contact up 뒤 stream이 조기 재활성화 | InputReader가 pointerCount 1인 상태에서 추가 move 1개도 `accepted=false`. |
| 18 | cancel이 적용 offset/revision을 되돌리거나 재적용 | cancel 전/후 inner Y `38.058037 CSS px`, revision 1이 동일하고 root/outer는 0이다. |
| 19 | all-up까지 화면의 지연 변화 | cancel과 all-up SurfaceView crop의 RGBA 차이가 0 pixel이다. |
| 20 | suppression이 새 gesture까지 영구 고착 | all-up 뒤 새 down/pan은 revision 2, inner Y `76.11607 CSS px`를 만들었다. trap은 종료 시 slot 1/0을 release하고 최종 InputReader pointer count 0을 확인했다([상태](android-input-all-up-state.txt)). |

추가 화면·해시·kernel/app trace는 [README](README.md), [pixel 비교](canvas-pixel-comparison.txt), [시나리오 판정](scenario-results.txt)에 있다.
