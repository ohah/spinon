# C13.3 standalone UIKit 대조 설계 실패 관점 검토 20개

1. **Spinon queue에 결함이 있어도 대조가 실패하지 않는가?** — 독립 호스트는 Rust queue를 호출하지 않는다. 이 실험을 제품 adapter 적합성 검사로 취급하지 않는다.
2. **XCUITest method pass를 native callback pass로 오해하는가?** — 각 입력별 counter와 offset을 attachment에 기록하고 1/1 method 통과를 분리했다.
3. **최대/최소 scroll offset이 callback 방향 결과를 가리는가?** — Run 01을 경계 confound로 분류하고 Run 02는 20,000pt content와 x=10,000pt에서 시작한다.
4. **Run 02의 입력 연속 누적으로 경계에 닿는가?** — 모든 측정 offset이 8,919~10,791pt여서 0/max와 멀다.
5. **UIScrollView 자체가 입력을 소비해 ancestor recognizer를 막는가?** — Run 03은 UIScrollView를 제거하고 일반 UIView에 recognizer를 직접 부착한다.
6. **일반 view 측정에서 app offset label이 scroll offset인 것처럼 보이는가?** — README는 Run 03 offset이 callback delta를 수동 누산한 표시임을 구별한다.
7. **검사자가 실제 요청값 ±38을 UIKit translation으로 가정하는가?** — 입력 요청과 callback의 translation 관찰은 별도로 기록한다.
8. **방향 표본 수가 편향되어 있는가?** — 매 run에 양·음 X 각 5개를 번갈아 요청하고 방향별 분모를 명시한다.
9. **누락 위치를 특정하지 않고도 특정한 것처럼 결론내리는가?** — XCTest synthesis, Simulator delivery, UIKit recognition을 모두 남은 후보로 둔다.
10. **호스트에 Spinon framework가 우연히 링크되는가?** — Xcode 프로젝트는 UIKit test host이며 Spinon 제품 빌드·Rust FFI를 호출하지 않는다.
11. **recognizer 종류가 달라 비교가 무효가 되는가?** — 세 run 모두 `UIPanGestureRecognizer`의 continuous scroll 경로를 사용한다.
12. **touch 입력이 scroll recognizer로 들어와 결과를 오염하는가?** — `allowedTouchTypes=[]`로 손가락 입력을 제외한다.
13. **앱 pan과 scroll recognizer 설정이 크게 달라지는가?** — target·계층은 다르며, 이 차이를 README에서 제품 adapter에 대한 한계로 남긴다.
14. **callback counter가 비동기 갱신보다 먼저 읽히는가?** — 각 입력 뒤 250ms 대기 후 읽고, Run 02·03에서 offset 변화와 counter 변화를 함께 보존한다.
15. **UI automation log의 일부 손실이 실행 결과를 바꾸는가?** — Run 02·03은 XCTest `XCTAttachment`를 통해 전체 10개 observation을 xcresult에서 추출했다.
16. **초기 Run 01 결과를 경계 없는 측정치로 취급하는가?** — 첫 양의 입력이 최대 offset에 닿아 방향 비율에서 제외했다.
17. **direction callback rate를 보편적 확률로 일반화하는가?** — 각 run은 5쌍 단일 반복이며 비율은 해당 고정 실행만 설명한다.
18. **xcodebuild 전체 성공 문구가 assertion 미실행을 숨기는가?** — attachment·`tests.json`의 method result와 개별 관찰 데이터를 별도로 저장한다.
19. **Simulator 결과를 실기기나 실제 trackpad 증거로 확대하는가?** — 환경과 비목표 범위를 문서와 plan/status에 명시한다.
20. **관찰만으로 speculative production fix를 요구하는가?** — 특정 단계가 확인되지 않아 제품 코드는 변경하지 않고 다음 계측 관문을 남긴다.
