# C13.3 computed overscroll Simulator matrix 계획 실패 관점 20개 · 2026-10-15

검토 시점에는 구현 코드를 수정하지 않았다. 검토 대상은 제한된 `contain-x/auto-y` computed policy를 두 simulator의 실제 input callback까지 연결하는 계획과 판정 기준이다.

1. **기본 경로가 변하는가:** 새 route scenario는 기존 default `auto/auto`, zero-offset 경로를 보존하고 test feature 안에만 둔다.
2. **unknown scenario가 조용히 기본으로 열리는가:** enum/argument 검증 실패를 명시적 오류로 처리하고 silent fallback을 금지한다.
3. **CSS를 우회해 policy를 직접 주입하는가:** app scene은 CSS declarations만 바꾸고 computed cascade → snapshot metadata → router 경로를 통과해야 한다. ready report로 결과 policy를 읽는다.
4. **X/Y 선언이 뒤바뀌는가:** X=`contain`, Y=`auto`를 각각 읽고 X-only·Y-only·diagonal 결과를 별도 assertion으로 둔다.
5. **대각선 Chrome 차이가 pass를 오염시키는가:** 기존 JSON은 Chrome의 대각선 차이를 기록한다. 이 계획은 C13.3의 축 독립 계약을 pass oracle로 분명히 정하고 oracle 차이를 보존한다.
6. **입력이 실제 boundary remainder를 만드는가:** 각 fresh run은 inner를 실제 `(86,106)` max에 둔 뒤 측정 swipe를 보내며 host가 시작 offset을 확인한다.
7. **seed가 실제 손가락 입력으로 오인되는가:** test-host seed를 측정 trace와 분리해 기록하고 성능·입력 수치에서 제외한다.
8. **seed가 ancestor도 움직이는가:** 준비 후 inner `(86,106)`, outer/root `(0,0)`을 검사한다. 불일치하면 해당 run을 판정하지 않는다.
9. **starting point가 inner clip 밖인가:** 고정 CSS viewport point를 기존 clip-aware hit result와 대조하고 miss/outside-border point를 input target으로 허용하지 않는다.
10. **touch direction 변환이 반대인가:** finger 이동 방향과 scroll offset 증가 방향을 각각 로그에 남기며 baseline auto control로 부호를 검증한다.
11. **native slop이 delta를 제거하는가:** callback을 받았다는 사실과 실제 commit을 구분하고, slop 미달은 더 긴 동일 방향 input으로 재실행한다.
12. **ADB가 다른 emulator에 전달되는가:** 모든 Android 명령에 `-s emulator-5554`를 쓰며 device inventory와 모델/serial을 evidence에 저장한다.
13. **iOS Simulator target이 바뀌는가:** 모든 `idb`/`simctl` 호출에 iPhone 17 Pro UDID를 고정하고 booted runtime을 확인한다.
14. **플랫폼 단위 변환이 비교를 왜곡하는가:** raw pixel/point, surface scale, CSS start/delta를 함께 보존하고 공통 400×240 CSS viewport 기준으로 판정한다.
15. **policy variant가 기하를 바꾸는가:** 두 scene의 node count·owner ids·ranges·viewport·colors가 동일해야 하며 바뀌면 정책 비교를 무효로 본다.
16. **X contain이 Y 전달까지 막는가:** contain 대상은 X offset 불변과 Y outer 증가를 같은 computed snapshot에서 확인한다.
17. **기본 route에서 X remainder가 전달되는 양성 대조가 있는가:** auto/auto variant가 같은 seeded geometry와 same input으로 ancestor X를 움직이는지 확인한다.
18. **캡처가 clip 바깥의 red marker를 놓치는가:** screenshot을 육안으로만 판단하지 않고 marker pixel bbox와 blue inner border/canvas bounds를 수치 비교한다.
19. **테스트 route가 기본 artifact에 섞이는가:** feature-off/default와 opt-in artifact를 각각 build하고 marker/fixture FFI 문자열의 부재·존재를 비교한다.
20. **검증 결과가 C13.3 전체 완료로 과장되는가:** `touch-action`, wheel, tap, invalidation, iOS multi-touch와 display/event gates를 명시적으로 후속으로 남긴다.

## 계획 검토 판정과 반영

- **보완 필요 1건:** 첫 계획 문안은 seed와 measured input을 분리한다고만 썼지만 시작 snapshot에 ancestor offset이 섞이는 검출 기준이 부족했다. 준비 종료 조건으로 owner별 exact offsets와 ranges를 추가했고 불일치 시 run을 판정하지 않도록 했다.
- **보완 필요 1건:** screenshot을 clip 안에서 확인한다는 표현만으로는 부분적으로 가려진 marker를 놓칠 수 있다. red marker bbox와 inner border/canvas bounds의 pixel assertion을 추가했다.
- **통과:** 독립된 나머지 18개 관점은 계획에 강제 조건으로 반영돼 있다.

코드 구현 전 계획 검토다. 구현 후 새 20개 코드/runtime 실패 경로 검토와 PR 변경분 별도 검토를 다시 수행한다.
