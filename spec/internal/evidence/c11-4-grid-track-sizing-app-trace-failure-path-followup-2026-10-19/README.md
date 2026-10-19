# C11.4 frame assertion 실패 경로 trace 보존 후속 검증

**목적:** PR #345 diff 검토에서 앱 코드가 frame assertion을 통과한 뒤에야 stage trace를 로그로 나누는 순서를 발견했다. 기대 frame이 다르면 진단 trace를 남기지 않고 반환하므로, Android/iOS 양쪽에서 trace chunk 로깅을 assertion보다 먼저 실행하도록 수정했다.

## 검증 환경

- Android: `emulator-5554`, `sdk_gphone16k_arm64`, Android 17/API 37 ARM64 AVD. 실기기는 사용하지 않았다. [app build log](android-avd/build.log)
- iOS: iPhone 17 Pro Simulator, iOS 26.2, UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`. [읽기용 build log](ios-simulator/build.log), [정확한 원본 gzip](ios-simulator/build.raw.log.gz)
- Logcat을 지우지 않고 각 실행에 서로 다른 run ID를 사용해 C11.4 행만 추렸다. iOS unified log도 같은 방식으로 고유 run ID를 필터링했다.

## 결과

두 플랫폼에서 수정한 앱을 다시 빌드한 뒤 기존 C11.4 7개 scene을 실행했다. 각 scene의 frame assertion과 trace chunk 전체 복원이 통과했고, canonical trace JSON SHA-256은 7/7 scene에서 양 플랫폼이 같았다. [run summary](runtime-stage-summary.json), [Android logs](android-avd/), [iOS logs](ios-simulator/).

이 재실행은 성공 scene의 회귀 확인이다. frame mismatch를 주입하는 별도 simulator 실행은 하지 않았다. 실패 시 trace가 먼저 남는 사실은 두 앱의 제어 흐름에서 호출 순서를 확인했다. 기존 화면은 바뀌지 않아 screenshots는 [앞선 7-scene evidence](../c11-4-grid-track-sizing-app-stage-trace-2026-10-19/README.md)를 참조한다.

일부 생성 로그에는 줄 끝 공백이 있다. 읽기용 text 파일은 끝 공백을 정리하고 정확한 원본은 대응하는 `.raw.log.gz`에 보존했다. `SHA256SUMS`는 이 디렉터리에서 검증할 수 있다.
