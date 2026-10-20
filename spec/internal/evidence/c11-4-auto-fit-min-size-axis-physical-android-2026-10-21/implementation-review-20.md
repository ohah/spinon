# 구현·실기기 결과 실패 경로 검토 20개

대상: `android-device` capture harness 추가와 SM-S731N의 53-scene 실행 결과
실행 근거: [실기기 README](./README.md), [기기 결과](./android-device-c114-auto-fit-min-size-axis-android-device-20261020T203145959Z-6d7d1d53/result.json), [AVD 비교](./android-device-c114-auto-fit-min-size-axis-android-device-20261020T203145959Z-6d7d1d53/physical-vs-avd-comparison.json)
판정: 결함을 수정한 뒤 현재 실기기 행렬과 수치 비교기를 통과시켰다.

1. **명령 대상이 연결된 다른 기기로 바뀌는 경우** — serial `R5KYB06SMPE`를 wrapper 필수 인자로 넘기고 각 install·launch·logcat·screenshot에 `adb -s`를 적용했다.
2. **에뮬레이터를 실기기 결과로 오인하는 경우** — `android-device`에서 `ro.kernel.qemu=0`을 확인했고 `device.json`에도 `qemu=0`을 기록했다.
3. **다른 제조사 기기가 고정 결과에 섞이는 경우** — 실기기 모드는 모델 `SM-S731N`을 정확히 검사한다.
4. **API 차이로 실행 subset이 달라지는 경우** — Android API 36 이상을 검사하고 결과에는 API 36·Android 16을 보존했다.
5. **32-bit 또는 다른 ABI 앱을 설치하는 경우** — 기기의 `arm64-v8a`를 검사하고 ARM64 fixture APK를 설치했다.
6. **기존 AVD gate가 느슨해지는 경우** — `android-avd`는 계속 `qemu=1`, API 37, ARM64를 요구한다.
7. **렌더러 identity가 추정값에 그치는 경우** — 실기기 `dumpsys SurfaceFlinger`의 GLES 행을 저장했으며 Xclipse 940/Vulkan을 확인했다.
8. **AVD 경로가 새 renderer probe 때문에 실패하는 경우** — 초기 검토에서 공통 SurfaceFlinger query가 AVD에 불필요한 새 실패 조건을 만든 점을 찾았다. query를 physical-device 분기에만 두고 AVD metadata 동작을 유지했다.
9. **구 APK 또는 이전 branch 앱이 남는 경우** — 현재 소스의 fixture feature build를 다시 수행하고 `adb install -r`로 설치했다.
10. **기기 설치본과 로컬 APK가 다르게 검증되는 경우** — `pm path`에서 `base.apk`를 얻고 기기 `sha256sum`을 로컬 SHA와 비교했다. 두 값은 `77e2ff1c…a91be86`으로 같다.
11. **fixture FFI가 기본 앱 산출물에 섞인다고 오해하는 경우** — 빌드 command에서 opt-in `c04-runtime-gpu-fixture-test`만 활성화했으며 결과는 제품 API 선언이 아니다.
12. **scene ID가 오래된 상태나 중복으로 실행되는 경우** — AVD `result.json`에서 53 ID를 가져오고 wrapper가 수와 중복을 확인하며 capture harness가 현재 definition과 일치하는지 검사한다.
13. **실행 label이 evidence 경로를 벗어나는 경우** — `run-kind`와 `evidence-group`은 소문자·숫자·하이픈만 허용한다.
14. **이전 scene log가 현재 결과로 섞이는 경우** — run ID·scene ID·process ID로 marker를 판별하고 scene별 원본 log를 따로 저장했다.
15. **trace chunk 일부만 있어도 통과하는 경우** — 모든 scene에서 chunk 번호와 전체 수가 맞고 payload JSON 복원이 성공해야 통과한다.
16. **실패한 frame이 layout ready에 가려지는 경우** — 전체 node frame 수·순서·ID와 x/y/width/height를 각각 1/64 CSS px 기준으로 검사하고 관측 frame을 JSON에 보존한다.
17. **track 수만 맞고 auto-fit collapse나 gap이 틀리는 경우** — columns·rows track 수, used size, collapse, gutter assertion을 모두 통과했다.
18. **layout만 성공하고 화면 draw가 안 되는 경우** — 53/53에서 scene별 box count가 맞는 `SPINON_C0410_DRAW ... presented` marker가 존재했다.
19. **표면 표본을 compositor 검증으로 확대하는 경우** — overflow PixelCopy는 앱 SurfaceView에서 4/4 exact였고 README에 compositor 이후 검증은 아니라고 한정했다.
20. **기준 AVD와 다른 수치를 허용치 없이 숨기는 경우** — 별도 comparator가 실제 node frame과 track/collapse를 scene별 비교했다. 53/53에서 최대 차이는 모두 0 CSS px, 재시도는 0이다.

추가 제한: `compileSdk 37.2`에 대해 Android Gradle Plugin 8.13.2가 36.1까지 검증됐다는 build warning은 남아 있다. 성능, 다른 기기, WPT, compositor 이후 결과는 주장하지 않는다.
