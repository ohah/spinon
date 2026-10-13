# C11.2.3 iOS route 누락 negative control

헤더와 Swift compile 오류만 수정하고 AppDelegate C11.2.3 진입 조건을 그대로 둔 별도 후보를 실행했다. 앱에 C11.2.3 scene 인수가 전달됐지만 AppDelegate가 C0410 runtime 화면을 선택하지 않아 흰 기본 화면이 열렸고 첫 `row-sparse-baseline` 장면의 runtime/draw marker가 나오지 않았다.

- [`failure.txt`](./failure.txt): 첫 scene timeout
- [`row-sparse-baseline-launch.log`](./row-sparse-baseline-launch.log): launch 결과
- [`unrouted-white-screen.png`](./unrouted-white-screen.png): 실제 화면 캡처
- [`c1123-runtime-attempt-build.log`](./c1123-runtime-attempt-build.log): compile repair가 적용된 빌드

이 실패 경로를 negative control로 보존했다. AppDelegate에 C11.2.3 boolean flag 및 scene prefix를 추가한 뒤 25-scene 행렬이 통과했다.
