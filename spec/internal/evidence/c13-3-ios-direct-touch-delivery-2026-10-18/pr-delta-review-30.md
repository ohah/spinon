# C13.3 direct-touch delivery control PR 변경 적대 검토 30개

구현·계획 검토와 분리해, 제출할 전체 PR 변경에서 서로 다른 실패 경로와 계약 경계를 확인했다.

1. **제품 기능 완료로 잘못 승격하는가?** — 변경된 Swift는 독립 oracle harness 아래에 있고 product runtime/API 코드는 바뀌지 않았다. STATUS는 C13.3 미완료와 `74/207`을 유지한다.
2. **direct touch를 wheel/trackpad 입력과 같은 계약으로 취급하는가?** — 계획과 evidence가 입력 source 차이를 명시하고 direct-touch 결과를 positive control로만 한정한다.
3. **direct recognizer가 간접 scroll event도 받아 결과가 섞이는가?** — direct mode의 `allowedScrollTypesMask`는 빈 값이며 direct touch type만 허용한다.
4. **기존 continuous-scroll oracle의 구성이 회귀했는가?** — 비-direct branch는 `.continuous`와 빈 touch-type 배열을 유지하고 같은 실행에서 별도 selector로 실행했다.
5. **scroll view의 자체 pan이 direct control을 대신했는가?** — direct mode는 `plainPanOracleRequested` scene을 사용해 일반 UIView에 recognizer를 달며 UIScrollView를 생성하지 않는다.
6. **gesture target이 잘못 선택되거나 보이지 않는가?** — 각 fresh launch에서 accessibility id를 조회하고 존재 및 hittable을 통과한 launch만 gesture 수에 포함한다. 실패는 최종 launch assertion을 통과하지 못한다.
7. **재실행 간 callback/offset 상태가 누적되는가?** — 앱을 매 launch 뒤 terminate하고 다음 launch baseline에서 callback 0, offset 0을 확인했다(3/3).
8. **행동 분모를 중복 callback 수로 잘못 계산하는가?** — gesture마다 전후 counter 증가 여부를 한 번만 세고 총 30 action을 별도 raw observation으로 보존한다.
9. **callback 없는 offset 변경을 성공으로 계산하는가?** — callback 관찰과 예상 부호의 offset 변경을 별도 집계하고 둘 다 30/30 assertion에 포함한다.
10. **offset만 있고 callback이 없는 상황을 성공으로 계산하는가?** — callback assertion이 독립적으로 있어 offset만 바뀌면 테스트가 실패한다.
11. **잘못된 label 형식이 우연히 pass하는가?** — counter parse 실패는 `-1`, offset parse 실패는 NaN으로 귀결되어 callback 증가/부호 확인 assertion을 만족하지 않는다.
12. **gesture 방향과 기대 부호가 반대인가?** — 왼쪽은 0.8→0.2 및 음수, 오른쪽은 0.2→0.8 및 양수로 기록하고 원시 행도 방향별로 남겼다.
13. **단일 초기 화면이 정상 상태를 보장하지 못하는가?** — 세 fresh launch 모두 target 상태와 zero baseline을 각각 확인하고 분모가 3인 aggregate를 기록한다.
14. **앱 준비 실패가 callback 누락으로 오인되는가?** — target, label, hittable 준비 실패는 `setup_failed` observation으로 분리하며 `validLaunches == 3` assertion으로 run을 실패시킨다.
15. **비동기 UI 반영 지연이 성공 gesture를 놓치는가?** — action 이후 최대 2초 동안 50ms 간격으로 counter와 offset을 재조회하며 미관찰은 raw false 행으로 남는다.
16. **polling이 callback 발생 자체를 만들거나 숨기는가?** — polling은 accessibility 값만 조회하고 직접 UIKit callback을 보내지 않는다. 입력은 XCUITest `press(forDuration:thenDragTo:)`가 수행한다.
17. **action 기록이 실행 횟수와 맞지 않는가?** — 3 launch × 5 repetition × 2 direction으로 계획 분모 30이며 세 launch 첨부물과 aggregate에 같은 수가 남아 있다.
18. **중간 실패가 assertion 전에 증거를 잃는가?** — launch/action observation과 aggregate attachment를 assertion 전에 추가한다. XCTest result bundle에는 두 test의 activity와 attachment manifest가 연결된다.
19. **XCTest method pass를 callback pass로 오해하는가?** — method 2/2 pass와 direct callback 30/30, continuous callback 4/10을 별도 명시한다.
20. **상대 이동량을 CSS px나 scroll delta로 확대 해석하는가?** — README는 약 422.3–431.3 값을 UIKit point의 XCUITest drag translation 관찰치로 제한하고 CSS px/trackpad delta로 부르지 않는다.
21. **continuous-scroll 변동 결과에서 유리한 비율만 택하는가?** — 이번 run 4/10과 선행 독립 run 7/10을 함께 기록하고 안정적인 누락률 산출을 거부한다.
22. **독립 UIKit 대조를 Spinon adapter의 무죄 증거로 삼는가?** — 결과는 XCTest synthesis, Simulator 전달, UIKit recognition 중 원인을 분리하지 못했고 product adapter는 이 host에서 실행하지 않았다고 명시한다.
23. **sysdiagnose가 완전히 수집됐다고 오인하는가?** — 보조 `simctl diagnose` 중단, xcodebuild exit 0, full archive 미보존을 한국어로 기록한다.
24. **Simulator 경고가 조용히 제거되어 환경 판단이 왜곡되는가?** — orientation/launch screen warning은 README에 남기고 테스트 failure와 구분한다.
25. **스크린샷이 Spinon 제품 화면으로 보이는가?** — result screenshot은 독립 UIKit probe이고 README 및 PR 본문에서 Simulator probe임을 식별한다.
26. **Tailnet 사전 복사 중 원본 파일이 달라졌는가?** — source hash/size를 복사 전후 재확인했으며 107/107 source-target 일치를 기록한다.
27. **교체 파일의 기존 preview 콘텐츠가 백업되지 않았는가?** — 교체 28개 모두 backup hash가 preflight target hash와 일치한다(28/28). 새 route 복사와 삭제는 분리해 삭제를 하지 않았다.
28. **동기화했지만 서버가 오래된 응답을 내는가?** — 107개 route/file 응답이 HTTP 200이고 본문 byte 수 및 SHA-256이 source와 일치한다(107/107).
29. **Roadmap/STATUS 브라우저 캡처가 다른 내용이나 옛 집계인가?** — Agent Browser DOM assertion이 roadmap `74/207`과 STATUS direct-touch entry를 확인하고 각각 해당 route 캡처를 보존한다.
30. **검증 후 Agent Browser가 계속 떠 있는가?** — close 결과, inactive session 및 null pid, browser process-name scan 0건을 별도 JSON으로 남겼다.

판정: 변경 경계는 독립 iOS Simulator input oracle과 근거 문서에 한정된다. 검토에서 제품 API/완료 상태를 잘못 승격하거나 preview hash를 맞추지 못하는 결함은 발견되지 않았다. C13.3의 간접 scroll callback 전달 계층 및 Spinon product adapter 동작은 여전히 미확정이다.
