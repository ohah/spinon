# C11.3.1 Grid repeat 표면 픽셀 검증

- 실행 ID: `c1131-android-avd-20261013T102344630Z-a7301ccd`
- 대상: `android-avd`
- 고정 Chrome 기준: Chrome/154.0.8037.92, DPR 1, `multi-track-repeat`
- 비교 표본: 5개. Chrome screenshot의 PNG와 표본 RGBA를 fixture hash와 함께 고정했습니다.
- 비교 범위: Android AVD SurfaceView의 PixelCopy입니다. AVD 렌더러가 소프트웨어이면 실기기 GPU 결과로 해석하지 않습니다.

## 결과

- 상태: `matched`
- 일치한 표본: 5/5
- 최대 채널 오차: 0
- 설치 검증 산출물: `platforms/android/app/build/outputs/apk/debug/app-debug.apk` (SHA-256 `878e09405e1dcd75e95e9d6c2caac1716a216e0687cb9b691343fb86c711131b`)

## 파일

- `result.json`: 장치 정보, 고정 oracle, 실행 hash 및 비교 결과
- `runtime.log`: 해당 앱 process의 run ID 로그
- `pixelcopy.png`: PixelCopy 원본 표면
- `android-screen.png`: Android 화면 상태
