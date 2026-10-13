# C11.3.2 iOS routing 실패 기록

- 실행 ID: `c1132-ios-simulator-20261013T182854547Z-fab08993`
- 앱 설치와 launch는 성공했고 PID `21284`를 확인했다.
- 150초 동안 C11.3.2 runtime/Metal 표식을 받지 못했다. 화면은 기본 흰 화면이었다.
- unified log에는 기본 bootstrap 결과만 있었고 named-line runtime은 시작되지 않았다.
- 원인: `AppDelegate`의 runtime GPU 진입 조건이 `--spinon-c1132-grid-named-lines` 인수를 처리하지 않았다.
- 수정: AppDelegate route에 해당 인수를 추가하고 정적 회귀 검사를 추가했다. 이 시도는 기능 성공 근거로 사용하지 않는다.
