# C13.3 iOS scroll type mask 분류 PR 변경 적대 검토 30개

계획·구현 검토와 별도로 PR 전체 변경을 검토했다. 각 항목은 별도 실패 경로 또는 계약 경계를 확인한다.

1. **실험용 변경이 product event 지원으로 바뀌었나?** — Swift 변경은 독립 UIKit oracle harness에만 있고 product runtime/adapter/public API는 바뀌지 않았다.
2. **C13.3가 완료로 잘못 표시됐나?** — `spec/STATUS.md`는 C13.3 미완료 및 공식 `74/207`을 유지한다.
3. **direct-touch control이 trackpad 표본에 합쳐졌나?** — direct touch 30/30은 별도 test와 다른 input source로 기록하고 scroll 분모에서 제외한다.
4. **Apple type semantics를 임의 지정했나?** — official `UIScrollType` docs는 discrete mouse wheel, continuous trackpad로 나눈다. 계획은 의미 참조만 하고 XCUITest 입력을 실제 기기의 대용으로 만들지 않는다.
5. **XCUITest scroll이 discrete였다고 단정하나?** — callback이 확인된 action은 `.continuous`와 union에서만 잡혔고 `.discrete`는 0이다. no-callback 10개는 type 분류가 아니라 no-recognizer result다.
6. **union observer가 typed observer를 감췄나?** — union·continuous·discrete counts를 따로 저장했고 typed/union invariant 위반은 0이다.
7. **recognizer 경합으로 category 결과를 바꾸나?** — simultaneous recognition delegate를 사용하고 event별 counters가 continuous/union에 같이 증가했다.
8. **direct touch가 scroll mask로 잘못 흘러들어가나?** — 세 간접 observer 모두 `allowedTouchTypes=[]`; direct mask는 별도 launch mode다.
9. **UIScrollView가 scroll event를 가져가나?** — type-mask scene에는 일반 UIView만 있고 scroll view를 만들지 않는다.
10. **launch flag 조합이 구 legacy mode를 덮어쓰나?** — direct, plain, type-mask flags가 별도 판정되고 기존 continuous branch의 `.continuous` setup을 유지한다.
11. **recognizer 간 translation이 상호 오염되나?** — 새 mode는 recognizer `ObjectIdentifier`별 translation baseline과 mask별 callback count를 갖는다.
12. **UIKit point를 CSS px로 확대 해석하나?** — callback log는 `translation_points`/`delta_points`를 쓰며 결과 README는 CSS px normalization 증거가 아니라고 제한한다.
13. **AX count parser 실패가 silent pass하나?** — 누락 parse는 -1로 남고 three-zero baseline assertion을 통과하지 못한다.
14. **target이 hittable하지 않은데 0 callback 표본을 만들었나?** — 각 launch에서 존재·hittable gate를 통과한 후 action을 보낸다.
15. **fresh launch별 상태가 재사용됐나?** — counter zero baseline을 세 launch에서 각각 기록했다(3/3).
16. **분모가 계획과 일치하나?** — 3 × 5 × 양방향으로 30 action rows를 보존했다.
17. **한쪽 부호에서만 source가 바뀌나?** — positive/negative 각 15개 중 callback이 각각 10개였다. 부호별 분포를 함께 보존한다.
18. **callback event 수를 input action 성공 수로 세나?** — 41 callback rows/mask와 30 action rows를 별도 단위로 유지한다.
19. **action을 못 받은 행이 aggregate에서 빠지나?** — 10 `none` 행을 포함해 모든 30 action을 부호·launch·callback delta로 저장한다.
20. **method pass를 전체 source delivery pass로 오해하나?** — 2/2 XCTest pass와 20/30 observed continuous classification을 분리했다.
21. **union-only 결과나 invariant 위반을 숨기나?** — outcome vocabulary에 `union-only`/`invariant-violation`을 두고 해당 결과는 0이라고 요약한다.
22. **Simulator callback extraction이 임의 association을 만드나?** — marker가 있는 unified log에서 41 continuous+41 union row를 추출해 action counters와 대조했다. discrete marker는 없었다.
23. **공유 log에 개인 simulator PID가 남나?** — 공유 xcodebuild/activity log의 process/thread identifiers를 대체했고 callback derivative는 마커 뒤 fields만 남겼다.
24. **screenshot이 제품 app 화면으로 보이나?** — screenshots는 standalone UIKit probe의 counters/teal view이고 README와 PR에서 그 범위를 명시한다.
25. **Simulator 기기·OS를 적지 않아 재현 대상이 흔들리나?** — iPad Pro 13-inch (M5), iOS 26.2 build 23C54와 Simulator UUID, Xcode/macOS version을 기록했다.
26. **Info.plist runtime warning을 test failure로 취급하나?** — orientation/launch-screen 경고는 test tree에 남지만 2개 method가 pass, failure/skip 0이다.
27. **simctl diagnose가 완료된 것처럼 보이나?** — 3분 이상 대기한 auxiliary child 중단과 xcodebuild exit 0을 기록하고 archive 수집을 주장하지 않는다.
28. **계획과 실행 결론이 불일치하나?** — plan 결과는 observed 20 continuous-only/10 none, discrete 0 및 미확정 pipeline stage를 기록한다.
29. **실제 trackpad 또는 제품 adapter 버그를 결론내리나?** — physical/real trackpad와 adapter는 실행하지 않았고 no-callback stage는 미확정으로 유지한다.
30. **Tailnet preview가 stale하거나 target 파일을 삭제하나?** — pass-3에서 rendered output과 원본 plan·evidence 228개 경로를 확인했다. 63개 신규·변경 파일만 복사했고 기존 교체 파일 40개 백업, 전체 복사 hash 228/228, HTTP 200 및 응답 body hash 228/228이었다. `no_deletes=true`다. Agent Browser는 roadmap `74/207`·C13.3 미완료, STATUS 근거 링크 4/4, evidence 이미지 4/4 및 같은 출처 링크 27/27을 확인하고 세션/프로세스를 종료했다. [사전 목록](./tailnet-sync/preflight-pass-3.json) · [백업](./tailnet-sync/backup-verification-pass-3.json) · [복사](./tailnet-sync/copy-verification-pass-3.json) · [응답](./tailnet-sync/served-verification-pass-3.json) · [browser 결과](./tailnet-sync/browser-verification-pass-3.json) · [종료 확인](./tailnet-sync/browser-session-shutdown-pass-3.json).

판정: 이 변경은 XCUITest input의 observed UIKit mask를 분리했다. `.discrete` misclassification은 callback 누락의 설명으로 관찰되지 않았으며 no-callback 10 action의 lost stage는 특정되지 않았다. 제품 코드와 roadmap completion state를 잘못 승격한 결함은 없다.
