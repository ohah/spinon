# C12.4 CSS worker stack bounds와 상주 페이지 표본

- 실행 ID: c124-android-20261011T124627532Z-f8f10c7b
- 플랫폼: android
- process-level 동시 session 측정: [c124-android-20261011T124627532Z-f8f10c7b](../../c12-4-concurrent-session-stack-cost-2026-10-11/c124-android-20261011T124627532Z-f8f10c7b/README.md)
- 각 C12.4 CSS worker에서 OS stack bounds를 얻고 coordinator에서 mincore를 세 번 표본화했다.
- Android에서는 같은 앱 UID로 thread별 stack VMA와 smaps RSS/PSS를 교차 확인하고 해제 뒤 worker stack VMA 부재를 확인했다.
- 원시 주소가 든 계측 파일은 공개 저장소와 Funnel 미리보기에서 제외하고 로컬에만 보존한다.
- resident page·RSS·PSS·요청 stack 크기는 별개 metric이다. commit·high-water mark·실기기 수치로 해석하지 않는다.
