# C13.3 direct-touch control 실행·구현 적대 검토 20개

계획과 별도로 변경 코드 및 실제 Simulator 관찰을 20개 실패 관점으로 검토했다.

1. **continuous scroll 대신 직접 touch만 우연히 잘 되는가?** — 결과를 입력 source별로 분리했고 direct path를 positive control로만 판정했다.
2. **direct recognizer가 scroll events도 계속 받는가?** — `allowedScrollTypesMask=[]`, `allowedTouchTypes=[direct]`를 코드에서 확인했다.
3. **기존 continuous oracle mask가 회귀했는가?** — `.continuous`와 빈 touch types branch는 유지됐고 이번 run에서 그대로 실행됐다.
4. **plain scene에 UIScrollView가 숨어 있는가?** — 두 mode에서 같은 plain UIView host를 쓰며 direct mode는 scroll view를 만들지 않는다.
5. **동일 accessibility target이 실제 hittable했는가?** — direct 세 launch에서 target 존재·hittable gate를 통과해 10 gesture씩 수행했다.
6. **재실행에서 offset 또는 callback count가 남는가?** — fresh-launch baseline은 3/3 `0, 0.0`이다.
7. **열 개 action이 아니라 callback 호출 수를 action 수로 잘못 세는가?** — per-action counter 전후를 비교했고 30개 action 각각을 따로 판정했다.
8. **callback 증가 없이 offset만 갱신된 행을 성공으로 처리하는가?** — 30/30 판정은 callback 증가와 부호 있는 offset 모두를 요구했다.
9. **callback만 있고 방향 이동은 없는 행을 성공으로 처리하는가?** — 부호 일치 offset도 30/30 별도 집계했다.
10. **XCUITest drag는 방향과 무관하게 같은 이동량을 보장하는가?** — 이를 가정하지 않고 약 422.3–431.3 pt 분산과 일부 9 pt 누적 drift를 그대로 보존했다.
11. **UIKit point를 CSS px 또는 wheel delta로 오해하는가?** — 움직임 단위가 UIKit pt이며 product normalization 증거가 아니라고 README에 제한했다.
12. **이전 continuous 7/10과 다른 4/10 중 하나를 숨기는가?** — 두 run의 원본을 모두 링크하고 callback 비율은 불안정하다고 적었다.
13. **4/10의 action 분모가 불명확한가?** — 다섯 반복×positive/negative 두 방향, 총 열 요청을 첨부물에 보존했다.
14. **continuous 입력이 바뀐 test method에서 실행됐는가?** — 기존 selector와 XCTest API를 재사용했고 same simulator/run bundle에서 기록했다.
15. **XCTest test pass를 각 indirect event 전달로 계산하는가?** — summary의 2/2 pass와 관찰된 4/10을 분리했다.
16. **직접 입력이 Simulator event path를 우회하는가?** — `XCUICoordinate.press`를 사용했으나 이 역시 Simulator XCTest injection이라 물리 touch라고 주장하지 않는다.
17. **제품 adapter가 독립 host의 성공으로 면책되는가?** — Spinon 코드·framework를 링크하지 않았고 제품 adapter에 대한 결론을 내리지 않았다.
18. **sysdiagnose 수집 중단을 숨기거나 test failure로 혼동하는가?** — 보조 diagnostic만 중단했고 xcodebuild exit 0 및 정상 summary/attachments를 기록했다. full system diagnostics는 미보존이다.
19. **Simulator runtime warning이 기능 실패 또는 무시 가능한 환경인지 잘못 분류되는가?** — Info.plist orientation/launch-screen warning을 test summary의 non-failure warning으로 남겼다.
20. **screenshot이 제품 화면 증거로 오인되는가?** — 독립 UIKit oracle host screenshot이며 Spinon 제품 화면이나 trackpad support 증거가 아니다.

판정: direct touch 30/30 control은 유효하다. indirect continuous-scroll callback delivery는 이번 실행 4/10, 직전 7/10이라 불안정하게 재현됐다. C13.3는 미완료이며 누락의 정확한 pipeline stage를 아직 특정하지 않았다.
