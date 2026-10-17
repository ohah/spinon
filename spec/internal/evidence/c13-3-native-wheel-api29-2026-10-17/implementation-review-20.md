# C13.3 Android API 29 wheel 최소 API 검증 구현 적대 검토 20개

검토 대상은 API 29 AVD 전용 캡처기, test-only uinput helper, 사전 판정과 실행 evidence다. 제품 Android/iOS 코드는 바꾸지 않았다. 각 관점은 별도의 실패 경로를 대상으로 하며 같은 결과를 다른 말로 반복하지 않는다.

| # | 적대적 실패 관점 | 판정과 방어 |
|---:|---|---|
| 1 | 연결된 실기기 serial을 실수로 대상으로 삼는다. | 입력 serial은 `emulator-N` 형식만 허용하고 `ro.kernel.qemu=1`을 요구한다. 실기기 serial `R5KYB06SMPE`는 형식 검사에서 거부된다. 실제 실행은 `emulator-5556`으로만 보냈다. |
| 2 | 다른 API 29 AVD의 화면·밀도·page size를 이 fixture 기준으로 오인한다. | API 29, ARM64, 4 KB page, `spinon_r05_api29_fallback`, 1080×1920, density 420을 각각 검사하고 JSON에 보존한다. 다른 기기 설정은 실행 전에 거부한다. |
| 3 | API 29 ADB에 없는 `input mouse scroll`을 호출해 harness 오류를 제품 실패로 기록한다. | API 29 shell의 명령 목록에서 scroll 부재를 먼저 확인했다. AVD 내부의 별도 uinput mouse가 relative wheel을 보내 Android InputReader를 통과하며 callback에서 `ACTION_SCROLL`을 확인한다. |
| 4 | 시험 adapter가 OS callback을 우회해 Rust FFI에 바로 delta를 enqueue한다. | helper는 `/dev/uinput`에 Linux `REL_HWHEEL`/`REL_WHEEL`을 보낸다. 결과는 앱 `MotionEvent` callback에 남고 직접 FFI enqueue는 사용하지 않는다. |
| 5 | device source union 때문에 mouse event를 touchpad로 오분류한다. | app callback의 event source `0x2002`와 pointer tool `mouse`를 각각 확인한다. `device_supported_source_bits`는 분류 기준으로 쓰지 않는다. |
| 6 | wheel callback에 pointer가 없거나 잘못된 CSS point가 들어온다. | helper가 pointer를 목표 screen pixel로 이동한다. callback point는 `(132,112) CSS px`에서 한 screen-pixel 반올림 오차 이내여야 한다. |
| 7 | insets 또는 layout 이동으로 ready 뒤 SurfaceView origin이 바뀐다. | callback의 screen origin을 ready bounds와 비교해 다르면 실행을 거부한다. 입력 point는 해당 bounds에서 산출한다. |
| 8 | density 고정값을 잘못 재사용해 실제 surface scale과 Android factor가 어긋난다. | scale은 SurfaceView 실제 크기/400×240 fixture로 계산하고 callback scale과 대조한다. X/Y anisotropy와 nonfinite/nonpositive scale·factor는 거부된다. |
| 9 | Android V/H의 물리 부호를 CSS scroll 부호와 반대로 해석한다. | ±H/V 4개와 ±대각 4개를 별도 fresh launch로 보내고 Android factor 공식, 64 CSS px reference 및 실제 owner offsets를 모두 비교한다. |
| 10 | diagonal 입력이 두 callback으로 분리되거나 한 축을 잃는다. | helper는 H/V relative axes를 같은 `SYN_REPORT`에 보낸다. 각 대각에서 native callback과 queue commit이 정확히 하나이고 두 delta 축이 oracle과 일치해야 한다. |
| 11 | 이전 fixture state 또는 다른 policy scenario를 사용한다. | ready/eval log에서 `auto-boundary`, revision 1, 시작 offsets `[root 0,0; outer 0,0; inner 86,106]`를 강제한다. 매 행렬 case마다 app을 다시 실행한다. |
| 12 | stale target, 중복 sequence 또는 두 번 처리된 input이 통과한다. | callback과 process log 각각 정확히 한 건, queue sequence 1, `Queued`, target 7을 확인한다. 최종 revision은 정확히 2다. |
| 13 | remainder가 잘못된 scroll owner로 전달되어 nested boundary 결과가 달라진다. | 방향별 root/outer/inner 세 owner offset을 기존 C13.3 oracle과 exact 비교한다. 여덟 방향 전부 통과해야 한다. |
| 14 | queue는 통과했지만 실제 presented draw는 갱신되지 않는다. | revision 2의 `status=0 presented` draw와 scroll offsets가 process 결과와 일치해야 한다. |
| 15 | Control-wheel이 좌우 modifier 또는 한 축에서 누락되어 정상 입력처럼 통과한다. | 같은 uinput device에서 좌/우 Control key-down 뒤 H/V wheel, key-up을 보낸다. 4개 모두 `modifiers=2`·`control-wheel` reject여야 한다. |
| 16 | 거부 로그만 남고 실제 scroll 또는 paint 변경이 생긴다. | Control 거부에서 enqueue/process가 없고 revision 1·시작 offsets를 보존하며 aligned SurfaceView pixel 661,500개가 전부 unchanged여야 한다. |
| 17 | nested scroll paint가 root scrollport 바깥으로 새어 나온다. | positive-positive 대각의 aligned 661,500 pixel 비교에서 root border 바깥 변경 0개와 안쪽 변경 0개 초과를 요구한다. 실제 결과는 바깥 0·안쪽 50,684다. |
| 18 | surface가 이동·resize된 뒤 화면 비교가 틀린 좌표를 대조한다. | before/after bounds 각각을 캡처 좌표 안에 검증하고 SurfaceView 크기가 같아야 한다. local surface coordinates로 crop을 맞춘 뒤 비교한다. |
| 19 | 실행 실패·종료 뒤 가상 입력 device나 앱/harness 파일이 남아 다음 run을 오염시킨다. | C helper는 error/signal/normal exit에서 `UI_DEV_DESTROY`와 close를 수행한다. 매 입력 뒤 device name 부재를 검사하고 runner `finally`는 remote helper·앱·임시 host 파일을 정리한다. |
| 20 | 실행 환경·artifact가 달라졌는데 동일한 증거로 재사용하거나 SwiftShader 결과를 하드웨어 지원으로 넓힌다. | classifier Java test는 `mise exec`에서 돌리고 helper는 pinned NDK API 29 clang으로 빌드한다. APK·helper·source 파일 SHA-256, API/AVD/renderer 정보를 남기며 GL/ANGLE/SwiftShader를 확인한다. 결과 범위는 API 29 AVD fixture이고 zero-axis·touchpad·실기기·hardware GPU·iOS는 미완료로 명시한다. |

## 검토 중 발견해 수정한 harness 결함

- 최초 classifier 검사에서 `javac`가 없는 bare Node 환경을 호출했다. `mise exec -- node --test`로 바꿔 classifier 회귀 검사를 통과시켰다.
- API 29 logcat에 이전 app PID의 wheel line이 남아 새 run과 섞였다. 모든 buffer를 비우고 현재 app PID의 기록만 수집하도록 바꿨다.
- 앱 시작 직후 `pidof`의 exit 1을 오류로 처리했다. process가 준비될 때까지 재시도하고 ready/eval/draw를 기다리도록 수정했다.
- 초기 evidence에서 AVD 이름 property와 renderer log tag를 잘못 읽어 metadata가 비었다. 실제 boot property와 renderer line을 수집하고 AVD identity와 SwiftShader를 assert한다.
- pixel reader의 out-of-range crop을 별도 오류로 잡지 못했다. 캡처마다 SurfaceView bounds와 screenshot bounds 검사를 추가했다.

최종 재실행은 Android API 29 / Android 10 ARM64 AVD에서 방향 8/8, Control 거부 4/4, scroll 경계 외부 changed 0, Control no-op 0으로 통과했다. 이 검토는 실제 iOS callback/sign 또는 C13.3 전체 완료를 증명하지 않는다.
