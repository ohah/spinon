# C13.3 iOS scroll type mask 분류 계획 적대 검토 20개

구현 전에 fixture·관찰값·판정 기준을 계획 자체의 서로 다른 실패 관점으로 검토했다.

1. **direct touch와 scroll event를 같은 입력으로 보나?** — direct-touch 30/30은 별도 control이며 이번 30개 scroll action의 source 판정에 섞지 않는다.
2. **continuous callback만 못 받았다고 discrete로 단정하나?** — 세 typed/union recognizer를 동시에 기록하고 no-callback outcome도 독립 분류한다.
3. **Apple의 scroll type 의미를 추정에 의존하나?** — `.discrete`와 `.continuous` 의미를 각 Apple API 문서에서 고정했다.
4. **`allowedScrollTypesMask`가 finger touch까지 받아 결과를 오염하나?** — 모든 간접 scroll recognizer에 빈 `allowedTouchTypes`를 설정한다.
5. **union mask가 분류용 positive control인가?** — typed mask 두 개와 union을 같이 측정하며 `C/D` callback이면 `A`도 변해야 한다는 invariant를 둔다.
6. **동일 view의 pan recognizer들이 서로 배타적으로 실패하나?** — delegate 동시 인식을 켜고 callback type mask를 분리해 기록한다. typed recognizer별 반복은 raw evidence로 확인한다.
7. **UIScrollView가 자체 pan으로 callback 분배를 바꾸나?** — 일반 UIView만 쓰고 UIScrollView는 만들지 않는다.
8. **target 밖/비가시 입력이 전달 누락처럼 보이나?** — 각 launch에서 accessibility target 존재와 hittable 상태를 먼저 통과한다.
9. **기존 callback 또는 translation이 새 launch에 남나?** — 앱을 3회 새 프로세스로 띄우고 각 launch 세 counter가 0인지 확인한다.
10. **한 방향만 골라 type 분포를 왜곡하나?** — 동일한 다섯 회 왕복 순서로 positive/negative를 각각 5개 포함한다.
11. **callback 개수와 action 개수를 혼합하나?** — action 전후 per-recognizer callback 증가 여부 및 실제 증가량을 별개 field로 저장한다.
12. **union callback의 접근성 갱신이 늦어 false no-callback을 만드나?** — 최대 2초 poll 뒤 150ms 정착 구간에서 세 counter를 한꺼번에 읽는다.
13. **callback 없는 action의 원본 행이 사라지나?** — source outcome과 무관하게 30개 action row를 attachment에 기록한다.
14. **action method 통과를 source delivery 통과로 세나?** — 판정은 action별 세 counter와 direct control에서 하고 XCTest method 수는 별도 result로 남긴다.
15. **두 typed callback을 성공률로 합쳐 source 불변식을 감추나?** — C, D, A를 각각 기록하며 both와 all-only를 별도 outcome으로 둔다.
16. **direct-touch success가 scroll pipeline 전체를 증명하나?** — direct control은 다른 source이고 OS scroll event delivery를 입증하지 않는다고 계획에 고정했다.
17. **실제 trackpad/mouse 결과로 확대 해석하나?** — XCTest의 generated scroll만 쓰며 physical input 및 실기기 결과로 보고하지 않는다.
18. **mask 결과에 맞춰 제품 adapter를 성급히 수정하나?** — 어떤 category 분포가 나와도 이번 계획은 진단만 하며 product change를 승인하지 않는다.
19. **callback 부재를 OS event 부재로 단정하나?** — union mask도 callback이 없는 경우 결론을 recognizer 관찰 한계로 제한한다.
20. **C13.3나 roadmap 완료 수가 올라가나?** — C13.3 미완료 및 공식 `74/207` 유지와 non-product 증거를 명시했다.

판정: 비교 입력과 source mask의 oracle은 고정됐고, 두 mask 동시 관찰의 분류 불변식과 불확실 결과를 포함했다. 계획 검토 뒤에만 harness 구현을 시작한다.
