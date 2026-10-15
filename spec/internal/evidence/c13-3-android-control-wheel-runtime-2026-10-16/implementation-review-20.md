# C13.3 Android Control-wheel 검증 도구·실행 적대 검토 20개 · 2026-10-16

검토 대상은 AVD 전용 uinput helper, Node capture runner, API 37 ARM64 AVD 실행 로그와 화면 비교다. 계획 검토 20개와 별개로 구현·실행 경로를 검토했다.

| # | 실패 관점 | 코드·실행 확인 | 판정 및 조치 |
|---:|---|---|---|
| 1 | 물리 장치 선택 | runner가 `emulator-N` serial 형식 외 대상을 ADB 호출 전에 거부한다. `localhost:5555` negative control은 exit 1로 끝났다. | 통과. physical device와 alias는 접근하지 않는다. |
| 2 | 실제 AVD 아닌 emulator serial | `ro.kernel.qemu=1`과 `get-state=device`를 함께 요구한다. | 통과. property가 다르면 실행을 중단한다. |
| 3 | 다른 API/ABI를 잘못 분류 | API 37, `arm64-v8a`, page size 16384를 각각 검사한다. | 통과. API 37 `sdk_gphone16k_arm64`만 통과했다. |
| 4 | NDK drift | `tools/android-ndk-version.txt`의 고정 버전과 host clang을 찾는다. 다른 `ANDROID_NDK_HOME`은 SDK 고정 NDK 경로로 대체한다. | 통과. NDK 27.1.12297006을 사용했다. |
| 5 | ELF 적재 alignment 불일치 | 세 PT_LOAD segment 각각의 `Align`이 0x4000인지 `llvm-readelf` 결과로 확인한다. | 통과. 모든 LOAD alignment는 16 KB다. 기존 4 KB helper는 AVD에서 시작 중 종료돼 재빌드했다. |
| 6 | malformed key/axis/좌표 | C parser는 좌우 Ctrl, H/V 축, 완전히 파싱된 양의 좌표만 허용한다. | 통과. 유효하지 않은 입력은 `/dev/uinput` open 전에 종료한다. |
| 7 | uinput 권한을 몰래 확대 | `/dev/uinput` open 실패를 그대로 보고하고 권한 변경, root, su 경로가 없다. | 통과. AVD shell 권한으로만 실행했다. |
| 8 | Android가 mouse로 분류하지 않음 | `INPUT_PROP_POINTER`, `BTN_LEFT`, EV_REL과 두 wheel 축을 설정하고 InputReader 추가 로그를 확인했다. | 통과. callback은 `source=MouseWheel`을 기록했다. |
| 9 | 이전 pointer 위치 의존 | helper가 corner reset 뒤 screen point를 이동하고 runner는 ready SurfaceView origin에서 `(132,112)` CSS px를 계산한다. | 통과. 네 case point는 `(362,1285)`였다. |
| 10 | 조기 baseline screenshot | runner는 ready, eval-ready, revision-1 draw와 1초 안정화 뒤 baseline을 얻는다. | 통과. 첫 실험에서 드러난 ready 직후 첫 presentation race를 수정했다. |
| 11 | 좌 Ctrl 미검증 | 실제 `KEY_LEFTCTRL`과 horizontal/vertical 입력을 새 app launch에서 보냈다. | 통과. 두 case 모두 raw meta 12288, normalized modifier 2였다. |
| 12 | 우 Ctrl raw bit 오판 | 우측 key bit는 좌측과 다르므로 side별 expected meta를 둔다. | 통과. 두 case 모두 raw meta 20480, normalized modifier 2였다. 초기 동일 raw 값 assertion은 수정했다. |
| 13 | vertical 축 방향 오판 | Linux `REL_WHEEL +1`의 Android callback 값을 독립 확인했다. | 통과. callback `vscroll=1.0`을 기대하도록 probe assertion을 수정했다. |
| 14 | 잘못된 rejection reason으로 거짓 통과 | callback은 source, axis, raw meta, normalized modifier와 exact `control-wheel` reason을 기록한다. | 통과. 네 case 모두 target point에서 기대 reason을 냈다. |
| 15 | reject 후 enqueue | native success marker와 `event=wheel` process marker 부재를 검사한다. | 통과. 네 case 모두 각각 0건이다. |
| 16 | reject 뒤 상태가 바뀜 | 모든 draw가 revision 1과 초기 root/outer/inner offsets인지 확인한다. | 통과. 네 case 모두 revision 1, `[0,0;0,0;86,106]`이었다. |
| 17 | SurfaceView 안의 작은 시각 mutation | 두 screenshot의 SurfaceView bounds를 정렬해 1050×630 전 pixel을 비교한다. | 통과. case별 661,500 pixel 중 changed 0개다. before/after PNG SHA-256도 일치한다. |
| 18 | key-up 또는 device destroy 누락 | helper는 key-up 후 `UI_DEV_DESTROY`와 close를 수행하고 runner는 device 제거, key-down 0, meta 0을 확인한다. | 통과. 네 case 모두 후속 InputReader dump가 정리 상태였다. |
| 19 | 항상 reject하는 잘못된 harness | 무 modifier `input mouse scroll` positive control이 modifier 0으로 enqueue·process·draw되는지 검사한다. | 통과. target 7, revision 2, outer X 64 CSS px였다. |
| 20 | 범위 과장 | evidence는 API 37 AVD opt-in fixture, iOS callback 미관찰, trackpad와 public event 미검증을 명시한다. | 통과. C13.3과 전체 61/197 완료 수는 변경하지 않았다. |

결과: 20개 구현·실행 관점에서 미해결 blocker를 찾지 못했다. 조기 캡처, 수직축 test expectation, 좌우 Ctrl raw meta expectation을 수정한 뒤 새 opt-in APK로 네 거부 case와 ordinary-wheel positive control을 다시 통과시켰다.
