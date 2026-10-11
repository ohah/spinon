# C12.4 CSS worker stack bounds와 상주 페이지 표본

- 실행 ID: c124-ios-20261011T124128613Z-2398be2f
- 플랫폼: ios
- process-level 동시 session 측정: [c124-ios-20261011T124128613Z-2398be2f](../../c12-4-concurrent-session-stack-cost-2026-10-11/c124-ios-20261011T124128613Z-2398be2f/README.md)
- 각 C12.4 CSS worker에서 OS stack bounds를 얻고 coordinator에서 mincore를 세 번 표본화했다.
- 원시 주소가 든 계측 파일은 공개 저장소와 Funnel 미리보기에서 제외하고 로컬에만 보존한다.
- resident page·RSS·PSS·요청 stack 크기는 별개 metric이다. commit·high-water mark·실기기 수치로 해석하지 않는다.
