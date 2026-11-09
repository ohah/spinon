# iOS Simulator 실행 및 정리

- 사용한 기기: 임시 `SpinonC114Temp20261109` / iPhone 17 Pro / iOS 26.2 (`AC2081D5-0D39-440A-96F4-4B35894AD1D4`). 기본 인자 없이 `dev.spinon.bootstrap`을 실행했다.
- 화면 캡처는 `screenshot.png`, layout/draw 근거는 `runtime-markers.log`에 보존했다. 로그에서 기본 Root HTML 환경의 `layout=ready`, `root_css_px=402x220`, Metal surface, `SPINON_C0410_DRAW ... presented`를 확인했다.
- 테스트 앱을 종료하고 임시 기기를 shutdown 및 삭제했다. 기존 iPhone 17 Pro 프로필(`ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`)도 shutdown 상태이며 기존 설치/데이터를 수정하지 않았다.
- 기존 프로필의 앱 컨테이너 조회는 서로 일치하지 않는 경로를 반환해 그 프로필에 앱을 설치하거나 실행하지 않았다. 사용 가능한 iOS 26.2 기기 목록에는 `S03` 프로필이 없었다.
