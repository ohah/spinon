# C11.2.3 iOS 실행 경로 복구 계획 검토

계획 검토에서 발견한 실패 가능성과 대응이다. 이는 구현 검토가 아니며, 임시 후보의 결과를 병합된 코드 결과로 취급하지 않는다.

1. scene 인수만 들어오고 별도 boolean flag가 빠지는 실행기를 놓칠 수 있다. 두 인수 형태를 모두 라우팅 조건에 포함한다.
2. boolean flag만 들어올 때 scene ID가 빈 값일 수 있다. 화면은 열되 C0410 입력 검증이 진단 오류를 내도록 한다.
3. 알 수 없는 scene을 기본 앱 화면으로 조용히 보내면 timeout 원인을 숨긴다. C0410에 진입시켜 명시적으로 거부하도록 한다.
4. C11.2.2 scene 인수가 C11.2.3 경로로 잘못 분류될 수 있다. prefix를 정확히 구분하고 16 scene 회귀 행렬을 실행한다.
5. C11.2.1·일반 C11 grid 실행 플래그의 경로를 바꾸면 안 된다. 기존 조건을 유지하고 C11.2.3 조건만 추가한다.
6. C11.3.1 플래그와 우선순위가 충돌할 수 있다. 새 수정 PR에는 C11.3.1 조건을 넣지 않고, C11.3.1은 별도 PR로 유지한다.
7. Objective-C 헤더 selector가 구현과 다르면 컴파일은 되더라도 호출이 달라질 수 있다. `.mm` 구현과 선언을 selector 단위로 대조한다.
8. 반환 타입이나 `uint64_t`, `NSUInteger` 폭이 다르면 ABI 호출이 깨질 수 있다. 헤더와 구현의 타입을 한 글자씩 비교한다.
9. 헤더에 선언을 추가했지만 Xcode target이 다른 헤더를 가져올 수 있다. 실제 simulator 빌드 로그와 컴파일 입력을 확인한다.
10. 중첩 조건식을 단순화하면서 C11.3.1·C11.2.3 우선순위가 바뀔 수 있다. 원래 조건 순서를 보존한 `if / else if`로 옮기고 기본 경로도 확인한다.
11. Swift optional 결과를 분기 변경 중 강제로 unwrap할 위험이 있다. 기존 optional 오류 처리와 `missing-report` 경로를 유지한다.
12. 앱 프로세스를 재사용하면 지난 scene ID가 다음 장면 결과에 섞일 수 있다. scene별 새 프로세스와 run ID 일치를 계속 확인한다.
13. app launch 인수가 전달됐다는 사실만으로 화면 진입을 보장하지 않는다. C0410 결과 marker와 draw marker를 모두 요구한다.
14. iOS 표면 픽셀 결과를 전체 화면 parity로 오해할 수 있다. 지정 19 scene·99 표본과 drawable 한계만 주장한다.
15. Simulator Metal 결과를 iOS 실기기 GPU 결과로 과장할 수 있다. 기기 종류와 compositor 전 readback 한계를 계속 표시한다.
16. geometry 결과가 픽셀 readback 성공을 대체할 수 있다. 둘을 독립 acceptance gate로 확인한다.
17. 기대 거부 6개가 앱 crash나 timeout으로도 통과 처리될 수 있다. 예상 진단 marker와 종료 결과를 정확히 대조한다.
18. 변조된 표본 좌표가 실제 Metal readback까지 도달할 수 있다. negative control에서 `readbackStarted=false`를 확인한다.
19. 4개 oracle-only 후보까지 Spinon 구현 성공으로 셀 수 있다. 고정 fixture inventory의 outcome 분류와 25개 matrix summary를 분리해 기록한다.
20. 임시 후보 통과만으로 C11.2.3을 완료 처리할 수 있다. 전용 PR 병합 후 `origin/main`에서 재현하기 전에는 상태를 미완료로 둔다.

## 계획 단계 통과 조건

20개 공격 관점 중 미해결 항목은 없다. 구현 전에 특히 재현된 white-screen 경로를 그대로 negative control로 보존하고, 병합된 기준에서 새 프로세스 25-scene matrix를 다시 실행한다.
