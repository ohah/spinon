# C13.3 UIApplication event 경계 PR 변경 적대 검토 30개

최종 PR diff 전체를 제품 경계, 앱 진입, UIKit event forwarding, observation 기록, 재현 근거와 공개 상태 일관성 관점에서 다시 대조했다. 구현 검토 20개와 별개로 PR에 들어가는 harness·문서·증거 파일을 확인했다.

1. **제품 adapter가 진단 목적으로 바뀌었나?** — 수정 경로는 기존 독립 iOS probe app, XCUITest, 계획·근거·상태/로드맵 문서다. Spinon runtime, native wheel adapter, API는 diff에 없다.
2. **custom `UIApplication`이 실제 principal class로 선택되지 않을 수 있나?** — 새 `main.swift`가 `UIApplicationMain`에 `C133ProbeApplication`을 전달하고, Simulator run 3/3 fresh launch가 accessibility counter에서 같은 class를 확인했다.
3. **delegate가 연결되지 않아 probe 화면이 비어 있을 수 있나?** — principal delegate 이름을 함께 전달하고 `didFinishLaunching`에서 `UIWindow`와 기존 `C133WheelOracleView`를 구성한다. 세 launch 모두 target이 존재하고 hittable이었다.
4. **기존 SwiftUI `@main`과 새 UIKit 진입점이 중복될 수 있나?** — 앱 파일에 기존 `@main` 선언은 없고 Xcode source phase에는 `main.swift`가 한 번 추가됐다. clean build가 통과했고 focused test 두 개를 최종 revision에서 확인했다. 전체 suite 17/17은 이전 revision 결과이며 최종 revision 전체 suite는 결과 bundle summary를 저장하지 못해 판정 근거로 쓰지 않는다.
5. **계측이 event forwarding을 누락·중복할 수 있나?** — `sendEvent(_:)` override는 opt-in count/marker 기록 뒤 `super.sendEvent(event)`를 단 한 번 호출하고 화면·접근성 notification은 super 뒤에 발행한다. 초기 구현의 동기 UI 갱신이 super 전 실행될 수 있음을 검토에서 발견해 순서를 고쳤다. scroll recognizer와 direct-touch 대조가 callback을 기록했다.
6. **진단이 기존 probe launch를 바꾸나?** — event count와 event marker 기록은 `--app-event-boundary-oracle`가 있을 때만 활성화된다. 기존 launch modes에서는 audit `enabled`가 false다.
7. **새 `UIEvent.EventType`이 `.scroll`로 잘못 분류되나?** — 알려진 type을 각각 세고 `@unknown default`는 `other`에 넣어 미래 enum을 scroll로 합치지 않는다.
8. **다른 event type이 누락되어 scroll 전용 결과로 오해되나?** — scroll 외 touches·hover·transform·presses·motion·remote control·other counter를 함께 보존한다. 최종 scroll run은 hover delta도 row마다 분리했다.
9. **fresh process 간 count가 섞이나?** — `configure`가 앱 launch마다 전체 count dictionary를 0으로 교체한다. 3/3 launch의 event·recognizer zero baseline을 확인했다.
10. **SwiftUI view가 audit update 전에 생성되어 화면 counter가 stale 상태로 남나?** — view controller가 notification observer를 등록하고 현재 audit snapshot을 즉시 읽으며 이후 변경도 반영한다. 화면 캡처에 class 및 count가 있다.
11. **event marker 알림 observer가 view 해제 후 남나?** — controller `deinit`에서 observer를 제거하는 경로가 있다. test는 각 launch 종료 후 앱을 terminate한다.
12. **화면 요약이 잘려 parser와 캡처를 속이나?** — event label은 두 줄·고정 높이로 배치해 최초 한 줄 clipping 발견을 수정했다. 최종 focused run 화면에서 `scroll=29`, `continuous=12`, `union=12`를 읽을 수 있고 direct-touch 화면은 `scroll=0`, `touches=730`이다.
13. **event source와 recognizer target이 달라 비교가 무효인가?** — 같은 plain UIView target과 continuous/discrete/union pan recognizer 구성을 사용하고 app boundary mode만 추가했다.
14. **indirect scroll mask가 direct touch를 받아 event source를 혼합하나?** — scroll oracle recognizer는 touch types를 비우고 direct-touch control은 별도 test method에서 direct touch만 허용한다.
15. **typed recognizer callback이 union recognizer 없이 기록되어 mask 판정이 모순되나?** — continuous/discrete 증가에는 union 증가가 있어야 한다는 invariant를 별도 counter와 assertion으로 검사했고 위반은 0이었다.
16. **XCUITest 전후 기준점이 없거나 다른 값을 비교하나?** — 각 action 직전 recognizer와 전체 app event snapshot을 취하고 action 뒤 같은 접근성 element를 다시 읽는다.
17. **첫 event에 poll이 끝나 burst 후속 count를 누락하나?** — 변화 감지 후 150ms settle하고 delta를 계산한다. 관측 window를 event 단위 또는 action당 단일 event로 주장하지 않는다.
18. **무응답을 충분히 기다리지 않고 no-event로 분류하나?** — 변화가 없으면 최대 2초까지 50ms polling하고, 최종 snapshot에서 각 counter window를 유효성 검사한다.
19. **XCTest action과 UIKit event 사이의 직접 인과관계를 과장하나?** — `UIEvent`에 XCTest action ID를 부여하지 않았음을 README·계획·로드맵에 표시하고 수치를 post-action window delta로 제한했다.
20. **counter 접근성 텍스트 파싱 실패가 0 또는 no-event로 흡수되나?** — 모든 필드의 sentinel `-1`, 앱 class, 비음수·단조 증가를 확인하며 invalid window는 별도 category 및 실패 assertion에 들어간다.
21. **recognizer 접근성 값이 부분 누락된 채 유효 window로 통과하나?** — continuous/discrete/union 키 세 개를 모두 요구하고 nonnegative·monotonic gate에 포함한다.
22. **invalid window가 category aggregate에 정상 row처럼 섞이나?** — event/recognizer validity를 각각 보존하고 `invalid-counter-window`를 집계하며 valid window 수 30/30·invalid 0을 요구한다.
23. **setup 또는 launch 실패를 denominator에서 조용히 제외하나?** — target·callback·event label 존재와 target hittable을 확인한다. setup 실패도 observation에 남고 최종 fresh launch·row assertions를 만족하지 못한다.
24. **class identity 불일치가 0 baseline으로 오판되나?** — accessibility value에서 실제 application class를 따로 읽고 기대 principal class 3/3 assertion을 둔다.
25. **direct-touch 대조가 간접 scroll 결과와 합산되나?** — 별도 method와 aggregate를 유지하고 `.touches` 증가, `.scroll` 불변, callback 및 signed offset을 각 30 row에서 따로 센다.
26. **direct-touch 결과가 settling 이전 stale translation을 쓰나?** — 150ms 뒤 callback·offset과 event snapshot을 다시 읽고 최신 전후 delta로 판정한다.
27. **summary가 event 개수와 observation window 수를 혼합하나?** — 최종 README는 30 windows, 총 90 event, window별 2~4개를 각각 구분하고 callback 21/9도 window 분류로 기록한다.
28. **다른 소스/실행의 스크린샷 또는 로그가 최신 코드에 붙나?** — `source-revision.json`은 harness source와 실제 app `debug.dylib`·test executable SHA-256, 실행·Simulator·toolchain을 기록한다. 이전 full-suite source와 current focused source는 별도 hash record에 묶고 attachment manifest와 `runs/SHA256SUMS`가 exported 자료를 고정한다.
29. **상태 대장과 공개 로드맵이 C13.3 완료나 count 증가를 암시하나?** — 두 문서는 harness-only 후속, 미검증된 `super.sendEvent` 내부 단계, 실기기·trackpad 한계, 공식 `74/207` 및 미완료 상태를 유지한다.
30. **PR 이미지가 로드되지 않거나 화면이 동작 근거를 대신하나?** — 본문은 raw GitHub screenshot URL 두 개로 실행 화면을 보여주고 README는 같은 첨부와 xcresult 원본으로 연결한다. 화면은 UIKit report이며 product rendering·physical input 증거라고 설명하지 않는다.

판정: 최종 diff에서는 제품 지원을 바꾸거나 완료 수를 올리는 변경을 찾지 못했다. 최종 source focused Simulator run은 2/2 pass다. 전체 XCUITest suite 17/17은 직전 notification-order revision에서 통과했고, 최종 source full-suite retry는 disk-full로 summary를 저장하지 못해 pass로 주장하지 않는다. 남은 근거 한계는 action ID join, `super.sendEvent` 이후 세부 UIKit 단계, trackpad·실기기, 제품 adapter 경로다.
