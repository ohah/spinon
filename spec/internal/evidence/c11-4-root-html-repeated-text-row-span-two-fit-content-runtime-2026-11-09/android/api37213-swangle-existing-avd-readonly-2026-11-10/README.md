# Android API 37.2 Root HTML sentinels

기존 `spinon_api37_2_presenttime_probe_20261018` AVD를 Emulator 37.2.13으로 실행했다. Android 17/API 37, `arm64-v8a`, Google APIs 16k 시스템 이미지, 화면 1080×2400/density 420이며 `-gpu swangle`은 Vulkan SwiftShader 경로를 선택했다. 앱 표면은 1080×2209였고 CSS viewport는 411.42856×841.52374였다. 정확한 SDK와 실행 인수는 [환경](android-environment.txt)과 [실행 명령](emulator-launch.txt)에 있다.

원래 AVD 프로필은 `-read-only -no-snapshot-load -no-snapshot-save`로 열고 이번 실행의 `-data`/`-cache`만 격리 임시 디렉터리로 보냈다. 각 장면은 기본 `dev.spinon.bootstrap/.MainActivity`의 `source=index.html` 경로에서 실행했다. `examples/bootstrap/index.html`에는 해당 Chrome 사전 관측 장면을 일시적으로 넣어 APK를 만들고, 빌드 직후 추적 파일을 원래 해시로 복원했다. 세 장면의 APK·HTML·공유 `root-app.js` 해시는 [manifest](apk-source-hashes.txt)에 있다.

| Chrome 관측 | 입력 | Android 결과 |
|---|---|---|
| 0 | early `[1,2]`, cap `0px / 0%`, `align-content:start`, `span2Six-forward` | `layout=ready`, 13 boxes, root `411.42856×220 CSS px`, 5 text draws, draw presented |
| 2471 | adjacent `[3,4]`, cap `20% / 10px`, `align-content:stretch`, `span2Span3Six-reverse` | `layout=ready`, 12 boxes, root `411.42856×220 CSS px`, 4 text draws, draw presented |
| 1222 | separated `[1,6]`, cap `20% / 120%`, `align-content:stretch`, `span2Span3Six-forward` | `layout=ready`, 12 boxes, root `411.42856×220 CSS px`, 4 text draws, draw presented |

각 항목에는 [Chrome 관측 JSON](scenes), `SPINON_ROOT_HTML_DEFAULT_APP_ROUTE source=index.html`, layout/draw 로그와 화면이 있다:

- [early/start/forward 로그](scenes/early-start-span2-six-forward/android-runtime.log) · [화면](scenes/early-start-span2-six-forward/screenshot.png) · [관측](scenes/early-start-span2-six-forward/chrome-observation.json)
- [adjacent/stretch/reverse 로그](scenes/adjacent-stretch-span2-span3-six-reverse/android-runtime.log) · [화면](scenes/adjacent-stretch-span2-span3-six-reverse/screenshot.png) · [관측](scenes/adjacent-stretch-span2-span3-six-reverse/chrome-observation.json)
- [separated/stretch/forward 로그](scenes/separated-stretch-span2-span3-six-forward/android-runtime.log) · [화면](scenes/separated-stretch-span2-span3-six-forward/screenshot.png) · [관측](scenes/separated-stretch-span2-span3-six-forward/chrome-observation.json)

화면 캡처는 visual smoke다. SwiftShader의 표면 readback, PixelCopy, 픽셀 동등성, 실기기 또는 hardware GPU 결과로 해석하지 않는다. 앞서 실패한 임시 AVD의 부팅 진단은 [API 37.2 offline 기록](../api37213-emulator-offline.md)과 [API 37.0 QEMU 기록](../api370-emulator-hang.md)에 그대로 보존했다.

검증 뒤 Android Emulator를 종료하고 이번 실행의 임시 userdata/cache 폴더와 생성된 APK 출력 폴더를 삭제했다. 기존 AVD profile/snapshot과 공용 SDK 도구 및 빌드 캐시는 유지했다. 현재 이 검증에서 띄운 Android/iOS 시뮬레이터는 실행 중이지 않다.
