# C11.4 Android API 37 runtime evidence 20관점 검토

**범위:** 실패 AVD metadata diagnosis, corrected API 37.0 ARM64 AVD, final APK의 기본 Root HTML route, 실행 로그와 OS screenshot. 이 검토는 code implementation/runtime 20개와 PR diff 30개에 별도이며, Chrome matrix나 Android screenshot의 pixel-exact proof를 대신하지 않는다. **결과:** 20/20 확인.

| # | 독립 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | API 37 image에 잘못된 AVD target이 숨겨지는가 | 실패 AVD의 root `.ini`는 `android-0`, image path는 `android-37.0`이었다. 실패 로그에 QEMU `-enable-hvf`가 없고 mprotect 오류와 exit 139가 남았다. |
| 2 | ARM64 ABI와 system image가 다른 기기에서 나온 결과인가 | corrected AVD `config.ini`와 emulator log가 모두 `arm64-v8a` 및 `android-37.0/google_apis`를 가리킨다. |
| 3 | 재생성에 사용한 CLI가 기존 실패 도구와 같은가 | 설치된 `latest-2`는 Command-line Tools 23.0이며, 새 root `.ini` target은 `android-37.0`으로 기록됐다. |
| 4 | host 가속 검사가 실패했는데 HVF 사용을 주장하는가 | emulator는 `CPU Acceleration: working`을 기록하고 실제 QEMU argv에 `-enable-hvf`가 포함된다. |
| 5 | ADB가 단지 daemon에 등록됐고 guest boot는 끝나지 않았는가 | install 전에 `sys.boot_completed=1`과 Android API `37`을 확인했다. |
| 6 | ADB serial이 `offline`인 상태를 online으로 잘못 읽었는가 | 검증 시 serial `emulator-5584` 상태는 `device`였고 APK install 결과는 `Success`다. |
| 7 | PR 대상 코드와 다른 checkout에서 APK가 생성됐는가 | metadata의 source revision은 PR head `f117d72cc6ab97db20853fd101a31f5bc20f5d13`이다. |
| 8 | APK hash가 기록 파일과 다른 build output인가 | 최종 APK SHA-256 `e1496de1…acb3578`이 `final-candidate.sha256` 및 device metadata와 일치한다. |
| 9 | fixture SHA가 APK에 복사된 `assets/index.html`과 다른가 | APK asset과 저장소 fixture가 모두 `923a9d831a6509c1eb577428296535369146034336ac5af6658af0506157b8a9`이다. |
| 10 | 앱이 테스트 전용 activity나 다른 package로 실행됐는가 | 설치 package `dev.spinon.bootstrap`, 실행 activity `.MainActivity`, 로그 route `default Root HTML`이 일치한다. |
| 11 | 비어 있는 bootstrap script 때문에 native input check가 실패하는가 | final APK의 `root-app.js`는 non-empty readiness marker이며 앱 로그의 `SPINON_ROOT_HTML_DEFAULT_APP_BOOT status=pass`가 통과했다. |
| 12 | HTML parse 뒤 layout을 호출하지 않고 route 시작만 보고하는가 | 앱 로그가 `SPINON_C0410_ENVIRONMENT ... layout=ready`를 기록한다. |
| 13 | box count가 다른 fixture나 stale frame에서 나온 값인가 | route fixture SHA가 pinned fixture와 일치하고 같은 layout report에 `boxes=20`이 기록된다. |
| 14 | layout 완료 뒤 GPU draw가 실제로 제출되지 않았는가 | 별도 `SPINON_C0410_DRAW` log가 `status=0 presented boxes=20`을 기록한다. |
| 15 | text payload가 빠졌는데 geometry만 ready로 나타나는가 | draw report의 `runtime_text_count=8` 및 `glyph_batch_draw_count=8`이 8개 payload와 일치한다. |
| 16 | screenshot 크기나 file이 다른 실행에서 왔는가 | PNG는 1080×2400이며 SHA-256 `ed9e5d56…30d6676`이 `final-candidate.sha256`에 있다. |
| 17 | screenshot 자체가 렌더되지 않은 빈 화면인가 | screenshot을 시각 확인해 status/nav bar와 색상별 row 및 줄바꿈된 고정 텍스트가 보인다. |
| 18 | SwiftShader 결과를 실제 GPU 결과로 과장하는가 | metadata는 SwiftShader라고 명시하고 README는 visual smoke 한계와 hardware GPU 미검증을 분리한다. |
| 19 | OS screenshot을 PixelCopy/readback 결과로 잘못 부르는가 | metadata·README·PR 설명 모두 PixelCopy, GPU surface readback, post-compositor pixel comparison을 주장하지 않는다. |
| 20 | 실패한 시뮬레이터와 새 검증 AVD가 계속 실행 중인가 | 실패 AVD는 원인 로그를 보존하고 삭제했다. 검증 AVD는 ADB `emu kill` 뒤 process 종료를 확인하고 다음 C11.4 검증용으로 등록만 남겼다. |

근거 파일은 [runtime README](./README.md), [AVD metadata](./android/device-metadata.json), [QEMU fail/fix log](./android/emulator-invalid-target-20261108.log), [corrected boot log](./android/emulator-valid-target-20261108.log), [app process log](./android/app-current-process.log), [screenshot](./android/screenshot.png)다.
