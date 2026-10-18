# C13.3 iOS direct-touch delivery control 계획 적대 검토 20개

구현 전 계획의 독립 실패 관점을 검토했다. 확인한 위험과 닫는 조건은 다음과 같다.

1. **직접 손가락 swipe를 trackpad scroll의 대체 oracle로 오해하는가?** — 두 입력을 서로 다른 source로 명시하고 결과는 pipeline 분리 힌트로만 사용한다.
2. **`allowedScrollTypesMask = []`만으로 touch도 막히는가?** — Apple 문서가 touch scroll과 별도임을 명시하므로 direct `allowedTouchTypes`를 설정한다.
3. **빈 `allowedTouchTypes`가 direct input을 막는가?** — control에서는 `.direct.rawValue` 하나만 지정한다. 기존 indirect test는 빈 배열을 유지한다.
4. **비교 recognizer 구성이 바뀌어 차이가 생기는가?** — target/handler/translation 누산은 공유하고 허용 source mask만 바꾼다.
5. **ScrollView 소비가 control 결과를 오염시키는가?** — control 장면에는 UIScrollView를 생성하지 않는다.
6. **화면 edge gesture가 system recognizer를 가로채는가?** — target 안쪽 중앙선의 0.8↔0.2 좌표로 drag한다.
7. **target이 화면에 없거나 hit 불가능한데 0 callback을 입력 실패로 분류하는가?** — 존재·hittable을 확인하고 시작 offset/counter가 0인지 기록한다.
8. **좌우 방향이 label과 반대로 기록되는가?** — x-origin, x-destination과 누적 offset 차이의 예상 부호를 각 행에 함께 기록한다.
9. **gesture ended callback의 0 delta로 방향을 잃는가?** — callback 개수와 gesture 전체 누적 offset을 분리해 기록한다.
10. **Accessibility 조회 지연이 가짜 누락을 만드는가?** — 각 입력 뒤 최대 2초까지 counter가 바뀌는지 polling하고 timeout은 실패 행으로 남긴다.
11. **UI test가 실제 UIKit callback 대신 자기 입력 성공만 주장하는가?** — callback counter 증가와 handler가 누산한 offset 부호를 각각 확인한다.
12. **한 번의 앱 상태가 반복을 오염시키는가?** — direct control은 앱을 세 batch마다 terminate/launch해 state를 초기화하고 한 결과 bundle 안에 세 번의 초기 offset/counter를 기록한다.
13. **이전 continuous test와 비교 조건이 다르게 고정되는가?** — 같은 Simulator, 동일 독립 host revision, 같은 recognizer handler를 쓰고 입력 API/source mask 차이를 문서화한다.
14. **결과 선택 편향이 생기는가?** — 세 direct run과 continuous 재현을 모두 저장하고 최선 run만 선택하지 않는다.
15. **3회 30 gesture를 독립 기기 표본으로 과장하는가?** — 같은 simulator/device 안의 반복이라고만 보고한다.
16. **직접 touch 성공으로 XCTest scroll synthesis를 특정 원인으로 단정하는가?** — 세 계층 중 정확한 위치는 미확정으로 유지한다.
17. **테스트 method pass가 10/10 delivery로 둔갑하는가?** — 실제 callback/offset 조건을 별도로 assertion하고 원시 행을 보존한다.
18. **실패 시 attachment가 assertion 전에 사라지는가?** — 관찰 attachment를 추가한 뒤 aggregate assertion을 실행한다.
19. **결과가 Spinon 제품 adapter의 무결성을 보증하는가?** — 독립 host와 제품 범위 밖이라는 한계를 계획·결과 양쪽에 남긴다.
20. **실험 후 roadmap 완료 수가 잘못 상승하는가?** — C13.3 미완료 및 공식 완료 수 74/207을 유지한다.

검토 결론: 진단 질문과 사전 판정 기준이 별도 입력 source의 한계를 포함했다. 결과에서는 계획대로 3 fresh launch, 30/30 callback, 30/30 방향 일치를 확인했다. continuous-scroll은 4/10으로 재현됐으나 이전 plain UIView 표본 7/10과 달라 비율을 안정 추정치로 해석하지 않았다.
