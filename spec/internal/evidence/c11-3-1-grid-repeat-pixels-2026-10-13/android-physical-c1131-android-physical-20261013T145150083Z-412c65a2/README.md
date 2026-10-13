# C11.3.1 Grid repeat 표면 픽셀 검증

- 실행 ID: `c1131-android-physical-20261013T145150083Z-412c65a2`
- 대상: `android-physical`
- 고정 Chrome 기준: Chrome/154.0.8037.92, DPR 1, `multi-track-repeat`
- 비교 표본: 5개. Chrome screenshot의 PNG와 표본 RGBA를 fixture hash와 함께 고정했습니다.
- 비교 범위: Android 실기기 SurfaceView의 PixelCopy입니다. 다른 기기로 일반화하지 않습니다.
- source gate 후보 Git tree: `96f2c8c2a75bd2ce43be735a27b212271bd0c51a` · [통합 manifest](../source-gate.json)

## 결과

- 상태: `matched`
- 일치한 표본: 5/5
- 최대 채널 오차: 0
- 설치 검증 산출물: `platforms/android/app/build/outputs/apk/debug/app-debug.apk` (SHA-256 `aa72e2e5185d45ba43e99818dbfaf48f3dfb94bbcca8d94bc0d93249c4f839ba`)

## 파일

- `result.json`: 장치 정보, 고정 oracle, 실행 hash 및 비교 결과
- `runtime.log`: 해당 앱 process의 run ID 로그
- `pixelcopy.png`: PixelCopy 원본 표면
- `android-screen.png`: Android 화면 상태
