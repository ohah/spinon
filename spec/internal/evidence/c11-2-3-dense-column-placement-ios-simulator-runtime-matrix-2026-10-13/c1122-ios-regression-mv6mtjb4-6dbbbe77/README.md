# C11.2.3 iOS 수정 후보의 C11.2.2 회귀 행렬

C11.2.3 수정 후보가 기존 C11.2.2 iOS 경로를 깨뜨리지 않는지 확인했다.

- 실행 ID: `c1122-i-mv6mtjb4-6dbbbe77`
- iPhone 17 Pro Simulator / iOS 26.2
- 16 scene: 성공 11/11, 기대 거부 5/5
- 69 node, geometry 최대 오차 0 CSS px (허용 0.5 CSS px)
- 11 scene·51 Metal drawable sample RGBA exact, 최대 채널 오차 0
- 각 scene은 새 앱 process로 실행했다. Screenshot은 pixel parity 증거가 아니다.

원시 로그, 비교 JSON 및 화면 캡처를 같은 디렉터리에 보관했다. 집계는 [`regression-summary.json`](./regression-summary.json)에 있다. 이 결과는 iOS Simulator drawable 기준이며 실기기나 compositor 이후 화면 결과가 아니다.
