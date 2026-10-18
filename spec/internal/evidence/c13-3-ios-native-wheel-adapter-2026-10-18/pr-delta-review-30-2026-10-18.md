# C13.3 iOS native wheel callback 재검증 PR 변경 실패 관점 30개

검토 범위는 이번 PR의 계획·상태 기록, inner-target XCUITest harness와 실행 근거다. 제품 adapter 코드는 바꾸지 않았다. 각 관점은 앞선 구현 검토와 겹치지 않게 이번 증거 변경만 대상으로 판정했다.

1. **이전 2/4 관찰 덮어쓰기:** 기존 4방향 XCUITest 결과를 별도 역사 기록으로 보존했고 inner-target 후속 결과를 새로운 섹션에 추가했다. 통과.
2. **시뮬레이터/기기 경계:** 기기명, iOS 26.2, arm64, UDID를 고정했고 결과를 실제 trackpad 또는 물리 기기로 확대하지 않았다. 통과.
3. **실행 파일 오인:** 설치 앱 executable hash를 기존 opt-in artifact hash와 대조해 동일함을 기록했다. 통과.
4. **XCTest와 callback 혼동:** Run 02는 XCTest 2/2 통과와 app callback 3/4를 별도로 기록했다. 누락 방향을 성공으로 세지 않았다. 통과.
5. **첫 X/Y roundtrip 집계:** Run 01의 horizontal·vertical 각각 두 sample을 app log에서 확인해 native callback/process 4/4로 집계했다. 통과.
6. **첫 수평 부호·offset 복귀:** `+4.680664` 뒤 `−17.822418`과 inner X 0 복귀를 연결했다. remainder는 다른 owner가 소비했다고 주장하지 않았다. 통과.
7. **첫 수직 부호·offset 복귀:** `+16.963684` 뒤 `−20.789566`과 inner Y 0 복귀를 연결했다. 통과.
8. **반복 실패 은폐:** Run 02 xctest steps에는 negative-X 동작이 있지만 앱 log에 callback/process가 없음을 보존했다. 통과.
9. **빠진 callback 원인 추측:** XCTest synthesis, UIKit recognizer, app adapter 중 한 곳으로 단정하지 않고 추가 계측이 필요하다고 명시했다. 통과.
10. **반복 성공 과장:** Run 03은 horizontal만 재검증했다. 이 결과로 전체 ±X/±Y 행렬 반복 성공을 주장하지 않았다. 통과.
11. **target 오선택:** 처리된 모든 sample의 `target=7`을 확인했고, target 밖 입력은 해당 run의 성공으로 계산하지 않았다. 통과.
12. **단위/scale 누락:** successful callback의 `UIKitPoints`, `pointsPerCssPx=1.0`, continuous source, momentum `None`, modifier 0을 유지했다. 통과.
13. **부호 해석 오류:** normalized CSS delta와 실제 inner offset 변화·반전 sample 처리 결과를 로그로 대조했다. 통과.
14. **remainder 과대 해석:** inner scroll range를 초과한 값은 remainder로 남았다고 적고 ancestor 소비를 입증한 것으로 표현하지 않았다. 통과.
15. **revision/sequence 과장:** 상세한 revision·sequence는 원본 로그에서만 확인하며 세 run 전체에 별도 안정성 주장을 추가하지 않았다. 통과.
16. **화면 캡처의 증명력:** 스크린샷을 fixture scene 확인으로 한정하고 native callback·pixel diff·표시 완료 증거로 쓰지 않았다. 통과.
17. **표시 경계 혼합:** screenshot pixel comparison과 compositor 이후 출력은 미검증으로 남겼다. 통과.
18. **harness와 제품 oracle 혼동:** probe host app의 scroll view를 test host로만 설명했고 Spinon adapter 결과나 기준 구현으로 사용하지 않았다. 통과.
19. **완료 상태/로드맵 합계 변경:** C13.3 checkbox와 공식 완료 집계를 유지하고 plan·STATUS·public roadmap note의 미완료 판정을 맞췄다. 통과.
20. **증거 무결성과 링크:** 새 event log, xcresult summary, screenshot, harness 파일의 SHA-256을 manifest에서 검증하고 변경 문서의 상대 링크를 확인했다. 통과.

21. **Harness 대상 앱 식별자:** XCUITest가 `dev.spinon.bootstrap`을 실행하며 README의 설치 대상과 opt-in launch arguments도 같은 fixture route를 가리킨다. 별도 host app을 product app으로 오인하지 않았다. 통과.
22. **재현 명령의 test selector:** 문서의 `-only-testing` 경로와 두 inner-target test method 이름이 Xcode project의 scheme·source와 일치한다. 예시 명령이 이전 중심점 test를 실행하지 않는다. 통과.
23. **시나리오 간 상태 격리:** 각 test method가 앱을 새로 launch하고 `defer`에서 종료한다. X/Y 결과를 앞선 process의 scroll offset에 의존해 누적하지 않았다. 통과.
24. **입력 좌표 hover 순서:** 입력 전과 왕복 두 번째 sample 전에 inner coordinate에 hover를 보낸다. 실제 adapter log의 target 7은 이 입력이 의도한 inner route에 도달한 sample에 한정해 연결했다. 통과.
25. **fixture host의 역할:** XCTest host app의 scroll view는 driver 실행용이고, Spinon app의 canvas는 설치된 대상이다. harness의 자체 scroll behavior를 Spinon의 성공 oracle로 삼지 않았다. 통과.
26. **RSPress evidence 링크:** 전체 docs build에서 README의 plan·JSON·압축 log 상대 링크가 문서 route로 잘못 해석되는 것을 확인했다. 이 페이지의 artifact 및 plan 링크를 병합 후 유효한 GitHub `main` blob URL로 바꿨다. 전체 docs build는 저장소의 다른 dead link 때문에 여전히 실패하므로 성공으로 표시하지 않았다. 통과.
27. **요약본과 원본 bundle 경계:** 보관한 `xcresult-summary.json`과 XCTest step log는 요약 산출물이며 원본 `.xcresult` bundle 전체가 저장소에 있다고 주장하지 않는다. 통과.
28. **증거 파일과 PR 표시 파일 구분:** PR 본문 이미지로는 PNG 화면 캡처 하나만 사용하고, JSON/log는 다운스트림 동작 근거 링크로 제시한다. 이미지 한 장이 callback 증거를 대체하지 않는다. 통과.
29. **문서 링크 병합 후 유효성:** 변경 문서의 GitHub `main` artifact URL 경로는 이번 PR에서 추가하는 동일 파일 경로를 가리키며 main 반영 뒤 남는다. branch 이름에 의존하는 문서 링크를 두지 않았다. 통과.
30. **설치 앱 이력 고정:** manifest의 `source_commit`과 installed executable hash는 기존 opt-in build를 식별한다. 현재 documentation branch가 그 바이너리를 다시 빌드했거나 새 adapter 코드를 검증했다는 인상을 주지 않는다. 통과.

## 판정

30개 관점에서 이번 PR delta의 근거 과장, 누락 callback 은폐, 완료 상태 승격은 발견되지 않았다. 확인된 제한은 수평 왕복의 간헐적 callback 누락이며 PR과 roadmap은 그 한계를 유지한다.
