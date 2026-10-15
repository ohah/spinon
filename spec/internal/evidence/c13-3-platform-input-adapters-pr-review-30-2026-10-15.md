# C13.3 Android/iOS 입력 어댑터 PR 변경 검토 30개

검토일은 2026-10-15다. 검토 대상은 현재 작업 브랜치의 C13.3 platform adapter diff와 연결 문서·산출물이다. 구현 failure review 20개와 별도로 변경이 build graph, FFI 경계, 두 플랫폼, 증거 및 공개 상태에 미치는 통합 영향을 확인했다.

## 변경 검토

1. **Cargo feature 그래프:** `c13-3-gesture-fixture-test`가 C04.10 fixture-test host 및 `spinon-render` feature에만 추가되어 기본 feature로 자동 활성화되지 않는다. 확인.
2. **Android Gradle 환경값:** `SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST`는 `0/1`로 검증되고 C04.10 runtime GPU 없이 켜지지 않는다. 확인.
3. **Android native build forwarding:** Gradle task가 opt-in 값을 Rust feature 및 C++ define까지 전달한다. 확인.
4. **iOS xcodebuild forwarding:** simulator script가 같은 flag를 xcodebuild property로 전달하고 project configuration이 C/ObjC++ macro와 Swift compilation condition으로 소비한다. 확인.
5. **iOS device configuration parity:** Debug/Release project configurations 모두 C13.3 macro 기본값 `0`과 Swift condition을 갖는다. 프로젝트 파일 대조 확인.
6. **build flag 오류 입력:** Android/iOS shell script와 Gradle은 `0/1` 이외 값을 거부한다. 소스 대조 확인.
7. **C header와 Rust ABI:** host 생성·eval·enqueue·process의 포인터, 정수, 좌표 scale 및 output 인수가 선언/구현에서 일치한다. 확인.
8. **C ABI 기본 빌드 격리:** C header 선언과 Rust export 모두 opt-in define/feature로 막힌다. default app artifact 검사에서 C13.3 symbol 0개. 확인.
9. **양성 대조:** iOS opt-in Mach-O에서 C13.3 FFI export 4개와 scene payload가 존재한다. 기본 artifact의 부재 검사를 무효화하지 않는 positive control이다. 확인.
10. **Android 앱 설정 산출:** build config field는 opt-in 값을 route에서 읽을 수 있게 하지만 default 값은 false다. 코드 및 AVD feature-off 화면 대조 확인.
11. **고정 scene과 제품 CSS 경계:** native gesture host는 `RuntimeSession::new_runtime_gpu_block_flex_overflow_clip_fixture`와 작업 전 정의 scene source를 쓴다. 일반 runtime CSS 표면으로 route가 열리지 않는다. 확인.
12. **Scene identity validation:** layout frame 수, scroll container 수, 부모 연결, owner identity와 기대 max range 검증이 완료된 뒤 ready snapshot을 게시한다. 확인.
13. **초기 snapshot sequence:** eval 중 presentation update가 생기면 stale sequence를 거부하고 interactive snapshot을 잘못 게시하지 않는다. 코드 대조 확인.
14. **queue 별도 소비자:** native callback은 bounded queue enqueue만 수행하며 background consumer가 session 적용 및 snapshot 게시를 담당한다. AVD 1초 worker-delay log에서 대기하지 않는 callback을 확인.
15. **queue capacity 일치:** Rust queue는 regular 64와 terminal reservation 8이고 JNI/ObjC++ process 호출 상한은 둘을 합친 72다. 확인; 동시 72 event burst는 직접 실행하지 않았다.
16. **Move coalescing policy epoch:** enqueue는 snapshot style revision을 policy epoch으로 전달하고 consumer는 accepted cumulative sample을 최신 snapshot key로 재구성해 처리한다. 코드 대조 확인.
17. **terminal identity:** End/Cancel은 gesture ID·pointer ID·surface generation을 유지하고 sequence를 부여한다. FFI/queue 대조 확인.
18. **registry 수명:** FFI producer lookup은 registry의 `Arc` 상태를 얻고 host free는 key를 제거한다. host 종료와 native producer가 경합하지 않도록 안전 계약 및 platform queue 순서를 확인.
19. **Android input ownership:** opt-in route에서만 SurfaceView touch listener가 gesture adapter로 입력을 넘긴다. Android 실기기/AVD 테스트 이외 route는 바꾸지 않는다. 코드 대조 확인.
20. **Android pointer 선택:** 첫 finger pointer의 ID를 고정하고 후속 Move에서 ID로 현재 index를 다시 찾으며 index 소실은 cancel/suppress한다. synthetic second pointer probe 한계를 문서에 표시했다. 확인.
21. **Android pixel→CSS mapping:** surface/400 CSS px scale 및 x/y 균일성을 확인한 뒤 시작점·누적 이동을 CSS px로 전달한다. API 37 AVD density 2.625와 captured trace를 대조했다.
22. **Android slop의 OS 출처:** `ViewConfiguration.scaledTouchSlop`을 사용하며 직접 상수를 만들지 않는다. 5px negative control은 21px보다 작아 pan이 시작되지 않았다.
23. **iOS recognizer contract:** UIPan `.began/.changed/.ended/.cancelled/.failed`를 adapter phase로 매핑하고 UIKit system slop을 명시하며 수치 추정을 피한다. 확인.
24. **iOS active touch tracking:** canvas view의 active `UITouch` identity count로 두 번째 pointer cancellation과 모든 touch 종료 후 재활성화 경계를 구현했다. 실제 iOS multi-touch는 미실행이라고 제한을 적었다.
25. **iOS point→CSS mapping:** recognizer translation과 canvas bounds/viewport를 사용하고 drawable scale이 균일하지 않으면 입력을 거부한다. 시뮬레이터 결과와 코드 대조 확인.
26. **lifecycle generation binding:** Android/iOS surface 변경·destroy와 app pause/background는 active input을 취소하고 새 surface generation에 이전 입력을 적용하지 않는다. AVD pause/fresh swipe를 실행 확인; 모든 조합은 미실행.
27. **queue wake coalescing:** Android atomic과 iOS lock 기반 `drainAgain`이 중복 schedule 중 도착한 입력을 다시 읽는다. 코드 대조 확인; 경쟁 입력 주입은 하지 않았다.
28. **render request 경계:** consumer는 처리 후 requestDraw만 요청하고, 실제 draw/render lane은 기존 surface generation 및 presentation sequence 규칙을 거친다. S04.10 displayed-frame hit target을 주장하지 않는다. 확인.
29. **증거의 상한:** Android는 API 37 ARM64 AVD/SwiftShader, iOS는 iPhone 17 Pro Simulator/iOS 26.2다. 캡처는 screenshot pixel 경계 비교이며 hardware GPU·실기기·compositor scanout으로 확대하지 않는다. 캡처 해시와 경계를 별도 파일에서 확인.
30. **공식 진행 상태:** `spec/STATUS.md`, 내부 계약 0063 및 plan은 C13.3 미완료와 style policy, wheel/trackpad, tap/events, S04.10/S05.2 잔여 범위를 유지한다. 공개 roadmap은 관찰된 부분 실행을 진행 설명으로만 추가하며 완료 체크를 올리지 않는다. 확인.

## 판정

이 diff에서 blocking integration 결함은 찾지 못했다. 기본/opt-in artifact 분리, Android/iOS simulator swipe·nested chaining, below-slop negative control, Android pause 복구와 화면 경계 측정은 연결된 근거로 확인했다. queue 최대치 72를 가득 채우는 동시 burst와 실제 iOS multi-touch는 실행하지 않았으므로 정적 코드 검토로만 남긴다. 이 결과는 C13.3 전체 또는 제품 CSS 지원 완료 판정이 아니다.
