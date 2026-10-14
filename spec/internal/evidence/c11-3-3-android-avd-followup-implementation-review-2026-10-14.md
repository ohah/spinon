# C11.3.3 Android AVD 후속 구현 검토 · 2026-10-14

이 후속 검토는 Android AVD 실행 경로를 추가하고 앱 런처의 C11.3.3 인자 연결 결함을 수정한 뒤 수행했다. 20개 관점은 원래 sparse-area 구현 검토와 별개다. Android API 37 AVD는 사용자의 이번 단계 시뮬레이터 허용에 따른 검증이며 SM-S731N 또는 hardware GPU 증거가 아니다.

| # | 실패 관점 | 결과와 근거 |
| ---: | --- | --- |
| 1 | AVD 모드가 실기기 모델을 자동 선택한다 | **PASS.** selector는 `emulator-숫자` serial만 받는다. AVD/실기기가 함께 있는 입력 단위 검사가 AVD만 선택한다. |
| 2 | offline·unauthorized 대상을 실행 중 AVD로 센다 | **PASS.** ADB 상태가 `device`인 emulator serial만 후보가 된다. offline negative가 거부된다. |
| 3 | 임의의 실기기 serial을 명시해 AVD 모드 경계를 우회한다 | **PASS.** serial 형식 검사에서 거부한다. SM-S731N 혼합 입력 negative가 확인한다. |
| 4 | 존재하지 않는 emulator serial이 자동 fallback된다 | **PASS.** 명시 serial 불일치가 연결 오류로 종료되며 다른 AVD로 조용히 바뀌지 않는다. |
| 5 | 실행 중인 AVD가 없을 때 빈 결과를 기기 입력으로 사용한다 | **PASS.** 빈 ADB 목록이 명시 오류를 낸다. |
| 6 | AVD가 여러 대일 때 첫 serial을 비결정적으로 선택한다 | **PASS.** 다중 AVD는 ambiguity 오류를 내고 명시 serial을 요구한다. |
| 7 | emulator serial만 보고 실제 qemu guest 여부를 생략한다 | **PASS.** 런타임에서 `ro.kernel.qemu=1`을 확인하며 결과 metadata에도 기록한다. |
| 8 | physical/AVD target과 guest 속성이 서로 달라도 계속 실행한다 | **PASS.** target별 qemu 값이 맞지 않으면 즉시 실패하도록 assertion을 둔다. |
| 9 | API 37이 아닌 AVD를 고정 근거로 오기록한다 | **PASS.** API 37 검사를 통과했고 결과는 Android 17/API 37이다. |
| 10 | hardware renderer를 AVD 소프트웨어 결과로 표시한다 | **PASS.** SwiftShader renderer를 요구한다. 실행 metadata는 ANGLE/Vulkan/SwiftShader를 기록한다. |
| 11 | 격리 clone의 V8 source가 없거나 다른 revision인데 빌드·근거를 성공으로 기록한다 | **PASS.** 첫 빌드가 pin 검사에서 실패해 원인을 보존했다. 수정 파일 없이 고정 commit의 별도 V8 checkout을 재사용한 뒤 빌드했다. |
| 12 | C11.3.3 intent가 Android `MainActivity` 분기까지 연결되지 않는다 | **FIXED/PASS.** 최초 실행은 `SPINON_C111_GRID_ARGUMENT_ERROR`를 기록했다. MainActivity의 flag/run ID/scene 검증과 C0410 경로 진입을 추가했고 후속 AVD 성공 실행으로 확인했다. |
| 13 | malformed C11.3.3 run ID가 다른 fixture log와 섞인다 | **PASS, 코드 경계.** MainActivity와 C0410 양쪽에서 허용 문자·길이를 제한한다. 이 형식 오류를 별도 앱 실행으로 주입하지는 않았다. |
| 14 | 다른 Grid profile flag가 C11.3.3 요청과 함께 전달된다 | **PASS, 코드 경계.** repeat/named-line/numeric placement와 충돌하면 MainActivity가 C11.3.3 argument error로 중단한다. 복합 flag 앱 실행은 별도 수행하지 않았다. |
| 15 | 알 수 없는 C11.3.3 scene이 C11.1 기본 장면으로 fallback된다 | **PASS.** `unknown-c1133-scene` negative가 `SPINON_C1133_GRID_ARGUMENT_ERROR`로 거부됐고 runtime summary·draw·PixelCopy는 시작하지 않았다. |
| 16 | 유효한 `basic-rectangles`가 C11.3.3 runtime 대신 기존 C11 scene parser에서 거부된다 | **FIXED/PASS.** 최초 `C111` 거부를 수정한 APK는 AVD에서 `basic-rectangles` runtime marker와 draw/present까지 완료했다. |
| 17 | 성공 scene의 HostDocument node order 또는 frame이 달라진다 | **PASS.** root와 item의 2 node preorder, ID, `160×120` 및 `(40,40) 120×40` frame을 확인했다. |
| 18 | area/explicit-track 수 불일치가 partial frame을 남긴다 | **PASS.** `area-track-count-mismatch` 기대 거부와 frame 부재를 같은 Android 앱 실행에서 검사했다. |
| 19 | PixelCopy가 다른 scene, cache path 또는 Chrome fixture sample을 비교한다 | **PASS.** scene/run ID, 전용 cache directory, 두 fixture 표본을 검사했고 Chrome RGBA 2/2 exact, 최대 channel 차이 0이었다. |
| 20 | 좌표 변조가 surface readback 전에 차단되지 않거나 Android 화면 PNG를 pixel oracle로 쓴다 | **PASS.** 바꾼 좌표가 `rejected-before-readback`으로 확인됐다. 비교 입력은 PixelCopy PNG이며 화면 PNG는 UI 확인 자료로만 저장한다. |

## 재실행 범위

최신 통과 실행은 Android API 37 ARM64 AVD `spinon_api37_0_applied_txn`, serial `emulator-5554`, SwiftShader software renderer에서 success/rejection 2 scene·4 node를 실행했다. 앱 SHA-256은 `32d25aa1fae57fd1e9e5f1773f24e77726f9f77345ace3c568bb8cd1122ce814`다. 자세한 frame·surface·negative control은 [실행 README](./c11-3-3-grid-template-areas-runtime-2026-10-14/android-avd-c1133-android-avd-20261014T011622837Z-13204dc1/README.md)와 [result.json](./c11-3-3-grid-template-areas-runtime-2026-10-14/android-avd-c1133-android-avd-20261014T011622837Z-13204dc1/result.json)에 있다.

이 후속 검토 시점의 2-scene Android AVD smoke는 26-scene 행렬을 포함하지 않았다. 이후 최신 source의 [Android AVD 26-scene matrix](./c11-3-3-grid-template-areas-runtime-2026-10-14/android-avd-c1133-matrix-20261014T023829118Z-d027504f/README.md)가 통과했다. Spinon WPT runner·적합성, SM-S731N 실기기, hardware GPU, C11.3.3 전체 CSS/Grid 또는 공개 CSS 지원은 여전히 입증하지 않는다.
