# Android 실기기 baseline 실행 근거 검토

| # | 확인 관점 | 근거와 판정 |
|---:|---|---|
| 1 | 실기기와 AVD 구분 | ADB transport가 USB이고 모델이 SM-S731N임을 실행 environment에서 확인했다. |
| 2 | OS API 정합성 | SDK 36, `SDK_INT_FULL=36.1`, Android 16을 기기 property에서 확인했다. |
| 3 | ABI·page size | `arm64-v8a`, 4096-byte page를 기기에서 수집했다. |
| 4 | 화면 해상도·밀도 | 실행 기록에 1080×2340과 density 450이 있다. |
| 5 | refresh 조건 과장 | 기기가 지원하는 60/120Hz와 현재 renderFrameRate 60Hz를 분리 기록했다. |
| 6 | 실제 입력과 synthetic 입력 혼동 | 실행 명령은 `adb shell input tap`; 보고서와 protocol 모두 synthetic이라고 표시했다. |
| 7 | 원래 설치 앱과 테스트 바이너리 불일치 | 설치 APK SHA가 이전 실기기 fixture의 기록된 SHA와 일치한다. |
| 8 | 소스 snapshot 불일치 | R05 관련 source manifest 8개 SHA가 `origin/main` checkout과 일치한다. |
| 9 | 실행한 renderer 오표기 | 앱 log에 `renderer=wgpu`, `backend=Vulkan`, device `Samsung Xclipse 940`가 있다. |
| 10 | click이 입력으로 처리되지 않음 | 30개 `SPINON_R05_INPUT`과 30개 `SPINON_R08_TOUCH`가 기록됐다. |
| 11 | 입력·revision 교차 연결 | input sequence와 revision이 각각 1–30으로 일치한다. |
| 12 | submit 누락 | 30개 입력 각각 `SPINON_R05_SUBMIT draw_accepted=true`가 있다. |
| 13 | callback 누락·중복 | 30개 입력 모두 callback을 하나씩 받았고 sequence mismatch는 0이다. |
| 14 | 오래된 surface 결과 | 30개 callback 모두 generation 1의 current surface 상태다. |
| 15 | 유효 fence를 signaled로 과장 | descriptor valid와 signal state를 별도로 집계했다. 30개 모두 pending이다. |
| 16 | pending을 실패·0ms로 변환 | usable signal은 0/30이며 latency 계산을 하지 않았다. |
| 17 | API capability 부재 은폐 | JankData 미지원 사유 `api_below_37`, VSync ID -1을 원본 log에 보존했다. |
| 18 | 색 변화 증거가 짝수 탭으로 가려짐 | 별도 one-tap smoke의 파랑/주황 계열 screenshot 두 장을 나란히 보존했다. |
| 19 | process 오류·환경 변화 | PID 대상 app 오류 0, app force-stop, Chrome focus 복귀, 화면 설정 전후 동일을 확인했다. |
| 20 | 개인정보·장치 상태 노출 | 공개 artifact에서 실제 ADB serial을 제외했고 logcat은 해당 app PID로 필터링했다. |

이 결과는 callback 시점의 즉시 fence 상태에 한정한다. fence의 이후 signal, 실제 손가락 touch, 표시 완료 latency와 optical scanout은 이 실행으로 확인하지 않았다.
