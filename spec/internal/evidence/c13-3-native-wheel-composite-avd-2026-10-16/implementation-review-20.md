# C13.3 Android 복합 wheel AVD 구현 실패 관점 검토 20개

검토 대상은 `capture-c13-3-native-wheel-composite-android.mjs`와 `c13_3_android_composite_wheel_uinput.c`다. API 37 ARM64 AVD 실행 로그, InputReader/getevent 장치 정보, before/after screenshot과 pixel 비교 JSON을 대조했다. 앱의 native source classifier는 이번 변경에서 수정하지 않았다.

| # | 적대 관점 | 확인과 결과 |
|---|---|---|
| 1 | serial을 빠뜨리거나 물리 Android 기기에 명령을 보내는가 | `--android-serial`을 필수로 받고 `emulator-N` 형식, `qemu=1`을 검사한다. 모든 ADB 호출은 해당 serial을 지정한다. 이번 run은 `emulator-5554`만 대상으로 했다. 통과. |
| 2 | 이름만 emulator인 잘못된 AVD가 조건을 통과하는가 | 모델 `sdk_gphone16k_arm64`, API 37, 1080×2400, density 420을 각각 검사한다. 캡처 manifest가 실제 값을 보존한다. 통과. |
| 3 | APK 경로가 없거나 fixture가 빠진 오래된 artifact로 성공을 주장하는가 | APK non-empty와 SHA-256을 기록하고, 실제 앱에서 eval/ready/bounds 로그를 요구한다. opt-in flags도 manifest에 남긴다. fixture가 빠진 APK는 readiness 단계에서 실패한다. 통과. |
| 4 | 임의 NDK 또는 잘못된 host compiler로 helper를 빌드하는가 | 저장소의 pinned NDK 버전과 일치하는 설치만 찾고 그 LLVM clang/readelf를 쓴다. 다른 버전만 있으면 실패한다. 통과. |
| 5 | helper가 Android 16 KB ELF 제한을 어기는가 | compile 단계에서 모든 `LOAD` segment의 정렬값이 16,384인지 검사하고 program header 원본을 저장한다. 네 segment 모두 16,384였다. 통과. |
| 6 | 장치가 mouse event와 touchpad capability를 실제 함께 광고하는가 | helper가 cursor 상대축, multitouch 절대축, `INPUT_PROP_POINTER`와 `INPUT_PROP_BUTTONPAD`를 같은 uinput descriptor에 넣는다. `getevent -pl`에 두 capability가 함께 남았다. 통과. |
| 7 | InputReader가 장치를 복합 장치로 인식하지 않았는데 그럴듯한 이름만으로 통과하는가 | `dumpsys input`의 같은 장치 블록에서 `CURSOR`와 `TOUCHPAD` class를 필수 검사했다. 실제 블록은 `TOUCH | CURSOR | TOUCH_MT | TOUCHPAD`였다. 통과. |
| 8 | helper의 wheel event가 다른 event code나 축으로 바뀌는가 | compiler source와 helper 로그를 대조했다. positive case는 `REL_HWHEEL` 한 단위를 보내며 실행 중 `hscroll=1.0`으로 기록됐다. 통과. |
| 9 | device source 합집합을 event source로 잘못 읽는가 | 앱 진단의 event source `0x2002`와 device supported-source union `0x10200a`를 별도 필드로 비교했다. 값이 다르고 두 source bit 집합이 모두 존재했다. 통과. |
| 10 | 복합 장치의 mouse event가 `finger` tool로 전달되어 결과가 우연히 맞는가 | callback은 `pointer_count=1`, `pointer_tool=mouse`, `pointer_tool_type=3`을 기록했고 assertion이 정확히 확인했다. 통과. |
| 11 | source union에 touchpad bit가 있어도 분류가 다른 경로에서 나온 값인가 | 같은 native callback line에 device union, event source/tool, `source=MouseWheel`을 함께 보존한다. 한 입력의 상관 관계가 확인된다. 통과. |
| 12 | mouse event를 touchpad로 오분류해도 queue가 진행되는가 | positive assertion은 callback 분류가 `MouseWheel`일 것을 요구하고 `TrackpadScroll`은 통과하지 못한다. 실제 union은 `0x10200a`였다. 통과. |
| 13 | 화면 안에서 의도한 nested target 대신 root나 다른 child가 선택되는가 | cursor 위치에서 계산된 CSS point가 `(132,112)`에 대해 x ±0.5, y ±0.75 이내인지 검사한다. 실제 `(132.09375,111.43936)`, target 7이었다. 통과. |
| 14 | raw wheel 단위를 CSS px로 잘못 환산하거나 부호가 뒤집히는가 | 원본 axis 1, ViewConfiguration factor 168 physical px, scale 2.625를 기록한다. 결과 delta는 `(64,-0)` CSS px였다. 통과. |
| 15 | callback만 있고 queue admission이 없는데 입력 성공으로 기록하는가 | `sequence=1`, `admitted=true`, `Queued`, target 7을 요구한다. 실제 process trace는 wheel 1건을 기록했다. 통과. |
| 16 | queue 이후 offset, revision 또는 presented draw가 다른가 | process/draw 모두 revision 2를 요구한다. owner offset은 `root:0,0;outer:64,0;inner:86,106`이고 presented draw에도 같은 값이 남았다. 통과. |
| 17 | SurfaceView 밖 입력이 queue나 화면을 바꾸는가 | 두 번째 scenario는 같은 복합 source와 point-outside-surface reason을 요구하고 queue/process 부재, revision 1, 초기 offsets를 확인한다. 통과. |
| 18 | READY가 떴지만 SurfaceView 합성 전 screenshot을 baseline으로 쓰는가 | 첫 시도에서 조기 capture가 배경 전환을 픽셀 변화로 세는 실패를 재현했다. READY 후 500ms 기다리고 bounds/draw를 새로 읽도록 수정했으며, baseline은 그 안정화 뒤에 캡처한다. 재실행은 통과. |
| 19 | SurfaceView 위치 변화나 crop 경계 오류로 바깥 변경을 놓치는가 | 두 screenshot을 독립 bounds로 정렬하고 crop이 1050×630인지 검사한다. root scrollport `[42,42,1008,588)` px 내부/외부를 pixel 단위로 비교했다. 양성은 외부 0·내부 50,774, 거부는 양쪽 0이다. 통과. |
| 20 | assertion 실패 때 callback 진단을 잃거나 가상 장치가 남는가 | validation 전에 callback log를 쓰며 pixel 실패도 기록한다. helper는 정상 종료 시 `UI_DEV_DESTROY`; 각 scenario 뒤 InputReader 제거와 최종 원격 binary 정리를 확인했다. 장치/getevent/header 원문은 JSON에 보존하고 읽기용 sidecar만 줄 끝 공백을 정리한다. 두 case 모두 통과. |

범위 밖은 물리 composite peripheral, 실제 trackpad, iOS UIKit callback, public DOM event, hardware GPU와 compositor scanout이다. 이 검토는 C13.3 완료 판정을 바꾸지 않는다.
