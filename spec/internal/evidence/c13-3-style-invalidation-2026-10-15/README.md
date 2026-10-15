# C13.3 style invalidation · 실행 근거

구현 PR: [#189](https://github.com/ohah/spinon/pull/189)

## 최신 Simulator 행렬

최종 run [`20261015T112615633Z-3abc0b5e`](20261015T112615633Z-3abc0b5e/README.md)은 현재 수정본으로 다시 빌드한 iOS·Android 앱, 여섯 시나리오 log, 명령 기록, screenshot, pixel 표본, matrix 및 SHA-256 manifest를 담는다. 앞선 성공 run [`20261015T111411692Z-d846f2a9`](20261015T111411692Z-d846f2a9/README.md)와 진단 run은 원본 실행 흐름 보존을 위해 그대로 둔다.

Xcode build transcript의 읽기용 `.log` 사본은 끝 공백만 정규화했다. 원본 bytes는 각 `.log.raw.gz` 파일에 그대로 보존한다.

- Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 policy-only, geometry-changing, paint-only를 각각 통과했다(6/6).
- 여섯 log 모두 style-update 보고가 1건이고 중복은 0건이다. 입력 batch도 validator가 중복을 제거한 뒤 검사한다.
- policy-only 갱신은 geometry와 scroll revision을 유지하고 활성 pan을 이어간다. 갱신된 `touch-action:none`은 다음 gesture에 적용된다.
- geometry 갱신은 기존 gesture를 `GeometryChanged`로 끝내고, 새 gesture는 변경된 range에서 시작한다.
- paint-only 갱신은 활성 pan과 scroll revision을 유지한다. 양 플랫폼에서 inner pixel은 `[23,37,58,255]`에서 `[14,86,80,255]`로 바뀌고 inner scrollport 밖은 `[27,52,65,255]`를 유지한다.
- 기본 Android debug APK와 iOS Simulator `.app`에서는 이 slice의 style ABI·payload·marker가 빠지고, opt-in 양성 대조에는 들어간다. [산출물 경계 기록](artifact-boundary/README.md).
- Rust와 JS 회귀 결과 및 원본 log는 [검증 기록](verification/README.md)에 있다.

## 확인한 실패와 수정

초기 시뮬레이터 시도에서 cascade 요청이 세션의 현재 값 대신 `StyleRevision::INITIAL`을 사용했고, 그 수정 뒤에는 기존 scroll offset을 복구하기 전에 geometry를 비교해 style-only 갱신이 scroll revision을 잘못 올리는 문제가 드러났다. cascade 요청부터 cache·layout까지 현재 style revision을 전달하고, offset 복구 뒤 geometry key를 비교하도록 고쳤다. 그 결과 scenario 8·10은 geometry-neutral로 남고 scenario 9만 실제 geometry 변경으로 취소된다.

Android native와 Java 경계에서 입력 batch가 중복 기록됐고 JNI와 Java가 같은 style-update를 각각 출력하던 문제도 확인했다. JNI 중복 출력을 제거하고, 캡처 검증기는 잘린 후행 eval 진단이 다른 같은 payload 중복만 합치며 가장 긴 원문을 보존한다. 기본 APK에서 Java 상수 문자열로 style marker/payload가 새던 것도 런타임 문자열 조립과 모듈 경계 회귀 검사로 막았다.

별도 review에서 iOS 앱이 style barrier 중 background 진입 시 deferred cancel보다 gesture ID를 먼저 초기화할 수 있는 경로를 찾아, barrier completion까지 ID를 유지하도록 수정했다. 고정 simulator matrix는 background timing 자체를 주입하지 않는다.

이전 run 디렉터리에는 수정 전 log와 진단 화면이 남아 있다. 최신 matrix는 수정 뒤 앱을 다시 빌드해 수집했다.

## 범위 한계

고정된 opt-in fixture와 시뮬레이터 관찰이다. Chrome parity, public Pointer/Wheel/scroll event, CSSOM scroll API, 실기기, hardware GPU, compositor/scanout 이후 화면, C13.3 전체 완료를 증명하지 않는다. Chrome 155 active-touch policy 관찰과의 차이는 [계획](../../../../plan/c13-3-style-invalidation.md)에 남겼다.
