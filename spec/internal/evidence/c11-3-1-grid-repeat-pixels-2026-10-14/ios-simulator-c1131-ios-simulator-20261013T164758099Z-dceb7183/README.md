# C11.3.1 Grid repeat 표면 픽셀 검증

- 실행 ID: `c1131-ios-simulator-20261013T164758099Z-dceb7183`
- 대상: `ios-simulator`
- 고정 Chrome 기준: Chrome/154.0.8037.92, DPR 1, `multi-track-repeat`
- 비교 표본: 5개. Chrome screenshot의 PNG와 표본 RGBA를 fixture hash와 함께 고정했습니다.
- 비교 범위: WGPU Metal drawable의 compositor 제출 전 readback입니다. iOS 화면 캡처와 OS compositor 이후 픽셀을 뜻하지 않습니다.

## 결과

- 상태: `matched`
- 일치한 표본: 5/5
- 최대 채널 오차: 0
- 설치 검증 산출물: `build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app/SpinonBootstrap` (SHA-256 `3ad46b0b6c3311e6fb89833c9f07bde8ff47802af89005347f3ea70b03ee1bbf`)

## 파일

- `result.json`: 장치 정보, 고정 oracle, 실행 hash 및 비교 결과
- `runtime.log`: 해당 앱 process의 run ID 로그
- `metal-surface-comparison.json`: Metal 표본별 비교
- `ios-screen.png`: Simulator 화면 상태
