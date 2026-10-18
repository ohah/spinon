# C13.3 iOS scroll type 분류 구현·실행 적대 검토 20개

계획 검토와 분리해 변경 code, Simulator 실행 및 결과 해석에서 서로 다른 실패 경로를 확인했다.

1. **진단 변경이 제품 adapter에 유입됐는가?** — 수정 파일은 독립 C133 UIKit oracle app/test와 계획·근거뿐이다. product runtime/adapter/API 파일은 변경하지 않았다.
2. **새 launch mode가 기존 direct/continuous mode를 덮어쓰는가?** — `--plain-scroll-type-mask-oracle`만 세 recognizer를 만들고 기존 direct-touch/continuous branches는 별도로 유지했다.
3. **직접 손가락 입력이 scroll type 분류에 섞이나?** — 세 scroll recognizer 모두 빈 `allowedTouchTypes`를 사용하고 별도 direct control method를 수행했다.
4. **UIScrollView가 event를 먼저 소비하거나 추가 pan을 만드나?** — type-mask mode는 일반 UIView target만 만들며 UIScrollView를 구성하지 않는다.
5. **mask names가 잘못 배정되나?** — `ObjectIdentifier`별로 continuous/discrete/union 키를 등록하고 handler에서 같은 identity를 조회했다.
6. **동시 pan recognizer 경합으로 하나가 임의 선택되나?** — 세 recognizer 모두 같은 delegate를 쓰고 동시에 인식하도록 허용했다. 20 observed action에서 continuous/union counter가 함께 증가했고 typed/union 불일치는 0이었다.
7. **translation baseline이 세 recognizer 사이에 섞이나?** — 새 mode는 recognizer identity별 translation map을 사용하고 callback count도 mask별 dictionary로 분리한다.
8. **point delta를 CSS px 또는 native wheel unit으로 해석하나?** — 앱 log는 `translation_points`/`delta_points`로 표기하며 계획과 README에서 UIKit point 단위로 제한한다.
9. **세 callback 상태를 읽는 UI test parser가 잘못된 값을 성공으로 처리하나?** — parse 불가는 `-1`로 남아 launch baseline assertion을 실패시킨다.
10. **cold launch가 이전 callback을 포함하나?** — 세 fresh launch 모두 counter baseline `continuous=0, discrete=0, union=0`이었다(3/3).
11. **launch 실패가 30개 action 표본에 포함되나?** — target 존재·hittable·counter label 준비 실패는 `setup_failed`로 분리되고 launch 수 assertion이 실패한다.
12. **요청 action 수와 분모가 다른가?** — 3 launch × 5 repetition × 양방향 2개로 30/30 action row를 보존했다.
13. **한쪽 방향의 입력만 지연/누락되나?** — positive와 negative가 각각 10/15 continuous-only, 5/15 no callback으로 같은 분포였다.
14. **callback 접근성 갱신 지연으로 무 callback을 만든다고 단정하나?** — 각 action에서 union을 최대 2초 poll하고 150ms 안정 구간 뒤 모든 counts를 읽었다. 10개 `none`은 세 mask 모두 counter 불변으로 기록됐지만 event 소실 계층은 단정하지 않았다.
15. **union positive control invariant가 실제로 검증됐나?** — continuous/discrete typed 증가가 union 증가 없이 나타난 행은 0이었다. observed callback 41개는 continuous 41, union 41, discrete 0이었다.
16. **테스트 method pass를 30개 scroll 성공으로 세나?** — UI test 2/2 pass와 source-classified action 20/30을 분리 기록했다. method는 zero baseline·row count·invariant만 assertion한다.
17. **callback count를 action 수로 오해하나?** — action별 callback delta 수와 `C/D/A` changed 여부를 별도 필드에 기록한다. 총 callback 41은 event action 30의 분모로 쓰지 않는다.
18. **OS log의 process/thread 식별자가 공유 evidence에 남나?** — 공유 callback 파생물과 xcodebuild/activity 로그에서 local process/thread identifiers를 제거했다. raw simulator log와 xcresult bundle은 temp run에만 남겼다.
19. **direct touch control이 scroll classification을 증명한다고 과장하나?** — 직접 gesture control은 별도 입력 source이며 `30/30` callback과 sign을 보조 control로만 report했다.
20. **result가 discrete misclassification 또는 제품 결함의 증명으로 승격되나?** — 이 Simulator run에서 관찰된 callback은 모두 continuous mask와 union에 같이 도달했고 discrete는 0이었다. 나머지 10 action의 lost stage는 XCTest synthesis, Simulator delivery, UIKit recognition 중 미확정이다.

판정: `.discrete` mask 누락이 관찰된 callback의 원인이 아니라는 source 분류 결과를 얻었다. 일부 XCUITest scroll action은 세 recognizer 모두에 도달하지 않았으므로 pipeline 원인은 아직 특정되지 않았다. C13.3와 제품 지원 상태는 미완료다.
