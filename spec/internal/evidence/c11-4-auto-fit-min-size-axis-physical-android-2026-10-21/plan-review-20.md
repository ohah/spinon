# 계획 실패 경로 검토 20개

대상: [C11.4 `auto-fit` 축별 실기기 후속 계획](../../../../plan/c11-4-auto-fit-axis-physical-android.md)
검토 시점: 구현·기기 실행 전
판정: 아래 위험을 계획의 oracle, target gate, 관찰 자료와 실패 기준에 반영했다.

1. **ADB가 다른 기기를 선택할 수 있음** — serial을 필수 입력으로 전달하고 모든 명령에 `adb -s`를 붙인다.
2. **연결 대상이 사실은 에뮬레이터일 수 있음** — `ro.kernel.qemu=0`을 실기기 gate로 검사한다.
3. **API 버전이 달라 fixture 경로가 바뀔 수 있음** — Android 16/API 36 이상만 허용하고 실제 release/API 값을 metadata에 남긴다.
4. **CPU ABI가 기대와 다를 수 있음** — `arm64-v8a`를 검사하고 APK를 이 환경에서 다시 빌드한다.
5. **이전 revision APK가 재사용될 수 있음** — current checkout에서 fixture feature를 켜 재빌드하고 local/install hash를 비교한다.
6. **fixture ABI가 제품 빌드에 섞일 수 있음** — `c04-runtime-gpu-fixture-test` opt-in 빌드만 사용하고 제품 지원 선언을 하지 않는다.
7. **ADB 기본 대상이 emulator로 되돌아갈 수 있음** — 실행·로그·스크린샷·설치·pull을 모두 pinned serial을 통해 수행한다.
8. **기존 53 ID와 현재 harness scene 정의가 어긋날 수 있음** — 모든 요청 ID가 현재 scene 정의에 정확히 한 번 존재해야 하며 불일치는 시작 시 실패시킨다.
9. **완료된 53-scene 입력 집합에 누락·추가가 생길 수 있음** — 기존 AVD `result.json`의 ID 집합을 고정해 exact equality를 확인한다.
10. **Chrome의 관찰값을 규범값으로 오인할 수 있음** — CSS Grid §7.2.3.2와 고정 fixture expectation만 acceptance에 사용한다.
11. **fractional track 비교가 문자열 비교로 잘못될 수 있음** — 최대 1/64 CSS px 수치 허용치로 track과 frame을 비교한다.
12. **auto-fit count만 맞고 collapse/gutter가 틀릴 수 있음** — 각 축 track count, collapsed flag, 내부 gutter 크기를 독립 검사한다.
13. **부분 trace가 정상으로 오인될 수 있음** — trace chunk 총수와 각 부분 sequence가 완전한지 검사한다.
14. **다른 실행의 log가 현재 scene을 통과시킬 수 있음** — process ID, run ID, scene ID가 함께 일치할 때만 결과를 수집한다.
15. **layout 성공이 화면 제출을 보장한다고 오해할 수 있음** — 각 scene에서 box 수가 맞는 draw/present marker를 별도 요구한다.
16. **SurfaceView 표본을 OS compositor 검증으로 확대 해석할 수 있음** — 계획과 최종 설명에서 PixelCopy가 앱 표면까지만 확인한다고 한정한다.
17. **overflow scene의 표본 좌표가 density 때문에 잘못될 수 있음** — fixture가 정의한 CSS 좌표와 surface mapping을 유지하고 exact RGBA 결과를 기대한다.
18. **GPU 차이를 기록하지 않아 결과 재현이 어려울 수 있음** — renderer/vendor 정보와 기기 fingerprint를 장치 metadata에 남긴다.
19. **실패한 첫 시도가 재시도로 가려질 수 있음** — superseded 또는 실패 log를 별도 보존하고 재시도 횟수를 결과에 기록한다.
20. **53 scene 반복이 기기에 무관한 성능 보장처럼 읽힐 수 있음** — 시간·성능 주장을 하지 않고 단일 SM-S731N/API 36 결과로 한정한다.

추가로 확인한 harness gap은 설치 APK hash를 원격 기기에서 검증하지 않는 점이다. 구현 단계에서 `pm path`로 설치된 base APK를 가져와 로컬 APK와 SHA-256이 다르면 실행을 거부하도록 보강한다.
