# C11.3.1 Grid repeat 표면 픽셀 검증

- 실행 ID: `c1131-ios-simulator-20261013T144912595Z-43f20ba7`
- 대상: `ios-simulator`
- 고정 Chrome 기준: Chrome/154.0.8037.92, DPR 1, `multi-track-repeat`
- 비교 표본: 5개. Chrome screenshot의 PNG와 표본 RGBA를 fixture hash와 함께 고정했습니다.
- 비교 범위: WGPU Metal drawable의 compositor 제출 전 readback입니다. iOS 화면 캡처와 OS compositor 이후 픽셀을 뜻하지 않습니다.
- source gate 후보 Git tree: `96f2c8c2a75bd2ce43be735a27b212271bd0c51a` · [통합 manifest](../source-gate.json)

## 결과

- 상태: `matched`
- 일치한 표본: 5/5
- 최대 채널 오차: 0
- 설치 검증 산출물: `build/spinon/DerivedData/Build/Products/Debug-iphonesimulator/SpinonBootstrap.app/SpinonBootstrap` (SHA-256 `4fbcbc5d83f72c5dbebe5baa1c83ac2f666829a30f930b0d217b5fd4ce5b2f0b`)

## 파일

- `result.json`: 장치 정보, 고정 oracle, 실행 hash 및 비교 결과
- `runtime.log`: 해당 앱 process의 run ID 로그
- `metal-surface-comparison.json`: Metal 표본별 비교
- `ios-screen.png`: Simulator 화면 상태
