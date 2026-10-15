# C13.3 Android AVD multi-touch 로드맵 검토 20개

검토 범위는 `plan/c13-3-scroll-gestures.md`, `spec/STATUS.md`, `spec/public/roadmap.html`의 Android second-touch 상태와 범위 문구다. 제품 runtime 코드는 변경하지 않는다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | Android 결과 때문에 C13.3 전체를 완료로 읽는가 | 상태 대장과 로드맵은 전체 C13.3을 미완료로 유지하고 남은 gate를 열거한다. |
| 2 | AVD 결과를 Android 실기기 결과로 확장하는가 | `API 37 ARM64 AVD` 및 실기기 미검증을 각 요약에 명시한다. |
| 3 | 두 번째 포인터가 synthetic MotionEvent probe에 불과한가 | Linux evdev→InputReader→앱 MotionEvent 경로라고 한정하고 직접 dispatch하지 않았다고 기록한다. |
| 4 | 두 번째 contact 전에 pan이 활성화됐는지 모호한가 | 첫 pan과 revision 1이 second-contact 전에 관찰됐다고 적는다. |
| 5 | 실제 접촉 수를 앱 로그 하나로만 추정하는가 | InputReader `pointerCount=2`와 `ACTION_POINTER_DOWN`을 함께 기록한다. |
| 6 | 두 번째 contact 종료 뒤 남은 손가락의 move를 놓치는가 | InputReader 1-contact state와 그 상태에서 거부된 move를 연결한다. |
| 7 | 두 contact 중 억제된 movement 수를 과장하는가 | 시나리오 결과에 있는 세 move만 억제됐다고 기재한다. |
| 8 | all-up 이전에 gesture가 재활성화되는가 | 두 번째 up 이후에도 첫 contact가 남은 동안 거부된 move를 기록한다. |
| 9 | all-up 경계의 pointer 잔존을 놓치는가 | `android-input-all-up-state.txt`에서 최종 `pointerCount=0`을 가리킨다. |
| 10 | fresh gesture recovery를 확인하지 않는가 | all-up 다음 새 pan의 inner Y와 revision 2를 적는다. |
| 11 | cancel 처리 전 scroll 위치가 바뀌었다고 오해하는가 | 기존 inner Y/revision 1 보존과 root/outer 0을 구분해 적는다. |
| 12 | 화면 0 pixel을 전체 화면 또는 scanout 근거로 과장하는가 | 정해진 SurfaceView crop만 비교했고 compositor/scanout은 증명하지 않는다고 한정한다. |
| 13 | fresh gesture 이후 화면 변화가 확인되지 않았는가 | 별도 recovery 장면과 crop의 changed-pixel count를 제시한다. |
| 14 | ADB shell input 명령을 멀티터치 입력처럼 적는가 | `input motionevent`의 pointer 한계와 선택한 evdev 경로를 분리한다. |
| 15 | 비-root `sendevent` 권한 실패를 감추는가 | SELinux denial의 주체·대상·권한을 진단 README에 연결한다. |
| 16 | 비활성 event11을 유효한 화면 입력으로 간주하는가 | event11은 virtual display `DISABLED`, event1은 display 0 `DIRECT`로 구별한다. |
| 17 | 스크립트가 다른 기기로 입력을 보낼 수 있는가 | AVD/API/ABI/root/display preflight 및 물리 serial rejection을 근거로 든다. |
| 18 | evidence 이미지 링크가 실제 파일과 어긋나는가 | README의 cancel/recovery 이미지 경로가 같은 evidence 디렉터리에 있다. |
| 19 | STATUS와 공개 로드맵이 서로 다른 결론을 내는가 | 양쪽 모두 AVD 근거는 반영하고 실기기·제품 전체 gate는 남긴다. |
| 20 | 정적 페이지에서 evidence route가 깨지는가 | `/spinon/spec/internal/evidence/.../README.html`을 사용하고 Tailnet post-sync Agent Browser 검사에 남긴다. |

검토 뒤 문구의 AVD/실기기 경계와 C13.3 미완료 상태를 plan, STATUS, 정적 로드맵에서 대조했다. Tailnet 서버에 제공된 최종 revision과 route 동작은 PR 병합 후 별도 확인한다.
