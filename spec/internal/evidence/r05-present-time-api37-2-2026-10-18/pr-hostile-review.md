# PR 적대적 검토 · R05 API 37.2 재실행

이 문서는 이번 증거·상태 문서 변경에 대한 서로 다른 30개 실패 관점을 점검한다. 앱 구현 변경은 없으므로 기능 구현 후 20개 코드 검토를 새로 했다고 주장하지 않는다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 다른 AVD 또는 물리 장치에 명령을 보내 결과가 섞임 | 모든 install·launch·tap·logcat 명령이 `-s emulator-5562`를 사용했다. |
| 2 | 기존 API 37.1 AVD를 API 37.2 결과로 오인 | 실행 serial과 `ro.build.version.sdk_full=37.2`를 따로 확인했다. |
| 3 | 물리 기기에서 얻은 결과를 시뮬레이터 결과로 기록 | 실기기를 사용하지 않았고 AVD 이름·fingerprint를 보존했다. |
| 4 | AVD 설정이 잘못되어 실제 API level과 target이 다름 | `target=android-37.2`, SDK full 37.2, extension 24를 확인했다. |
| 5 | API 37.0/37.1 또는 단순 SDK 37을 37.2로 부름 | runtime의 `ro.build.version.sdk_full=37.2`와 extension level 24를 확인했다. |
| 6 | 4KB 이미지 결과를 16KB image로 잘못 기술 | image path가 `google_apis_ps16k/arm64-v8a`다. |
| 7 | ADB `device` 상태만으로 OS boot 완료라고 오인 | `sys.boot_completed=1`까지 읽었다. |
| 8 | emulator 버전이 오래되어 API 실행 경계를 충족하지 않음 | Emulator 37.2.12.0, build 16428233을 기록했다. |
| 9 | 이전 AVD의 `target=android-0` 문제를 제품 코드 결함으로 분류 | AVD를 CLT 23으로 다시 생성한 뒤 HVF와 ADB boot가 정상화됐다. 소스 수정은 하지 않았다. |
| 10 | `service.sf.present_timestamp=0`만으로 unknown의 원인을 확정 | property를 진단값으로만 기록하고 판정은 실제 JankData raw callback에 맡겼다. |
| 11 | API SDK warning을 build failure로 기술 | AGP 미검증 경고와 Gradle `BUILD SUCCESSFUL`을 분리했다. |
| 12 | 다른 APK를 설치했거나 빌드 hash가 기록과 다름 | 설치 APK SHA-256 `bc5505…3ce6e7`을 보존했다. |
| 13 | Android native `.so`가 앱 APK와 다른 runtime source | `.so` SHA-256을 별도로 기록했다. |
| 14 | V8 checkout drift로 다른 JS engine을 포함 | pinned V8 revision `7b50b62c…`와 정적 archive hash를 기록했다. |
| 15 | probe source가 시험 뒤 변경되어 로그와 구현이 다름 | probe Java source hash와 base checkout SHA를 기록했다. |
| 16 | GPU runtime feature가 꺼져 probe 경로가 아니라 기본 UI가 실행 | Gradle build flag `c04_runtime_gpu=1`로 앱을 빌드했다. |
| 17 | WGPU block이 실제 WGPU renderer가 아니었음 | 세 log 모두 `renderer=wgpu` Surface와 callback을 기록한다. |
| 18 | GLES control block이 WGPU renderer로 실행 | 세 log 모두 `renderer=opengl_es`로 기록한다. |
| 19 | listener 등록 실패인데 timestamp 수집 성공으로 오인 | 6/6 block에서 `available=true reason=registered`를 확인했다. |
| 20 | 화면 크기/밀도가 taps마다 달라 target 좌표가 바뀜 | AVD display 1080×2400, density 420을 확인하고 중앙 좌표를 고정했다. |
| 21 | tap 좌표가 target 영역 밖이라 입력을 놓침 | 6개 log의 입력 sequence와 submit이 block마다 11/11이다. |
| 22 | 같은 process에서 target registry가 누적되어 다음 block이 오염 | block마다 `force-stop` 후 새 MainActivity process를 시작했다. |
| 23 | drain tap을 점수 표본에 포함 | 각 block의 sequence 1–10만 scored로 계산하고 sequence 11은 drain으로 제외했다. |
| 24 | input 성공만 세고 draw submission 누락을 숨김 | 각 block에서 input 11건과 submit 11건을 따로 집계했다. |
| 25 | callback 배열 순서로 input과 VSync를 추정 | 점수화 60개는 `target_vsync_id`와 callback `vsync_id` exact match만 인정했다. |
| 26 | 중복 ID·stale surface·wrong renderer가 exact count에 들어감 | scored records의 duplicate/stale/wrong-renderer count는 0이다. |
| 27 | queue reject 또는 flush reject가 표본 누락을 숨김 | callback batch의 reject counter는 모두 0이다. |
| 28 | `PRESENTATION_TIME_UNKNOWN=-1`을 positive로 취급 | 60/60 scored record의 raw 값은 `-1`, state는 `unknown`이다. |
| 29 | `unknown`, `unset`, invalid negative를 하나의 상태로 합침 | 전체 74 record 중 unknown 74, unset 0, invalid 0, presented 0이다. |
| 30 | 화면 캡처나 exact VSync join을 실제 표시/latency로 과장 | screenshot은 앱 화면만 증명한다. positive timestamp와 latency는 0건이며 R05.3/R05 완료 상태를 유지하지 않았다. |

## 결론

이 증거 업데이트는 AVD의 frame-token 연결 재현성을 확인하지만 actual-present timestamp 관문을 통과하지 못한다. 상태와 완료 수를 올리지 않는다.
