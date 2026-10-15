# C13.3 Android AVD targetable touchpad 계획 검토 · 20개 관점 · 2026-10-16

구현을 시작하기 전에 계획과 기존 실패 자료를 서로 다른 실패 경로 20개로 검토했다. 기준, 허용 event shape, fail-closed 보류 판정, uinput cleanup을 고정했다.

1. **ADB 연결 실패로 잘못 분류** — 앱에 도착한 source와 좌표 없는 event 로그를 기준으로 하며 연결은 성공했다. 통과.
2. **touchpad source를 pointer class로 가정** — `SOURCE_TOUCHPAD`의 source class와 AOSP X/Y 주입 조건을 함께 적었다. 통과.
3. **device source union을 현재 event로 사용** — queue admission은 event source와 tool만 사용하도록 고정했다. 통과.
4. **mouse positive를 touchpad positive로 게시** — mouse control과 touchpad event를 별도 기록한다. 통과.
5. **이전 cursor 위치를 재사용해 잘못된 target 수락** — 좌표 없는 touchpad는 반드시 거부한다. 통과.
6. **직접 FFI 호출로 native path를 우회** — direct FFI는 positive evidence에서 금지했다. 통과.
7. **가짜 app callback으로 OS 경로를 모방** — InputReader와 `ACTION_SCROLL` 로그를 요구한다. 통과.
8. **uinput 좌표를 CSS px로 오해** — screen px→surface px→CSS px 변환과 0.5 px 허용오차를 고정했다. 통과.
9. **scroll delta 부호 혼동** — 원본 H/V, factor, normalized delta와 Chrome owner offset을 모두 비교한다. 통과.
10. **source/tool 교차 조합 허용** — `SOURCE_MOUSE + finger` 또는 좌표 있는 명시 source만 허용하고 그 외는 reject한다. 통과.
11. **pointer 없는 event가 target에 닿음** — pointer count와 실제 point를 함께 검사한다. 통과.
12. **표면 바깥 event가 queue에 들어감** — expected target 밖 negative control에서 enqueue/process 0을 요구한다. 통과.
13. **거부 뒤 state가 일부 변경** — revision, 시작 offsets, pixel 무변경을 함께 검사한다. 통과.
14. **queue target과 draw owner 불일치** — target 7, revision 2, owner offset을 고정 oracle과 대조한다. 통과.
15. **stale surface의 point 사용** — generation 경계를 기록하고 stale sample은 거부하도록 계획했다. 통과.
16. **helper 강제 종료 뒤 가상 device 잔존** — 정상·오류 종료 모두 destroy/close와 사후 device inventory를 요구한다. 통과.
17. **물리 device 연결을 실수로 사용** — `emulator-5554`, QEMU/API/ABI 확인을 필수 사전 조건으로 둔다. 통과.
18. **기본 앱 artifact에 fixture route 포함** — opt-in boundary를 유지하고 artifact hash/feature 상태를 기록한다. 통과.
19. **AVD 결과를 외장 trackpad 지원으로 확대** — 문서와 공식 상태는 미완료로 유지하도록 판정 범위를 제한했다. 통과.
20. **AVD가 event를 생성하지 못했는데 성공 처리** — uinput 불가 시 harness 경계로 기록하고 targetable 입력을 미검증으로 둔다. 통과.

**판정:** 계획에는 수용된 실패 경로가 없고, 구현 후 실제 source/action/point 관측이 필수다. ADB touchpad의 좌표 부재는 앱에서 추측값으로 보정하지 않는다.
