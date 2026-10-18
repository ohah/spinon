# C13.3 standalone UIKit 대조 PR 변경 적대 검토 30개 · 2026-10-18

검토 범위는 이번 diff의 계획·상태 대장·로드맵·독립 UIKit 호스트·실행 근거다. 각 항목을 별도 실패 경로와 계약 경계로 대조했다.

1. 변경에 제품 adapter나 Rust runtime 수정이 섞였는가 — 없다. 제품 소스는 바뀌지 않고 독립 호스트와 문서·근거만 추가했다.
2. standalone 호스트가 Spinon 실행 파일을 실수로 호출하는가 — bundle ID는 `dev.spinon.probe.oracle.host`이며 UIKit 호스트 코드를 별도 실행한다.
3. recognizer 종류가 주장한 연속 입력 실험과 일치하는가 — `UIPanGestureRecognizer`에 `.continuous` scroll mask를 설정한다.
4. 입력 타입 제한이 실험을 touch pan으로 바꾸는가 — `allowedTouchTypes = []`로 touch 타입을 허용하지 않는다.
5. `UIScrollView` 장면의 content 경계가 측정 중 callback을 잘라내는가 — Run 02는 20,000pt content와 10,000pt 시작 offset을 썼다.
6. 실제 관찰 offset이 끝 경계 부근이었는가 — 8,919~10,791pt 범위로 양쪽 끝에서 멀었다.
7. 요청한 `±38`을 UIKit 실측 delta처럼 기술하는가 — README는 XCTest 요청과 UIKit translation을 동일시하지 않는다고 밝힌다.
8. Run 02 전체 callback 수가 방향별 수와 맞는가 — 8/10이며 positive-X 3/5, negative-X 5/5다.
9. 누락된 Run 02 action에서 offset이 callback 없이 움직였는가 — positive-X 반복 2·4에서 callback과 offset 모두 변하지 않았다.
10. Run 03가 여전히 scroll view 경로에 의존하는가 — recognizer는 ordinary `UIView`에 직접 붙는다.
11. Run 03 방향별·총 callback 수가 일치하는가 — 총 7/10, positive-X 4/5, negative-X 3/5다.
12. Run 03 누락 순서가 첨부 관찰과 다르게 적혔는가 — negative 반복 3, positive 반복 4, negative 반복 5다.
13. Run 03의 callback 기반 offset이 빠진 action에서 증가한 것으로 오기됐는가 — 해당 세 action 모두 callback counter와 표시 offset이 유지됐다.
14. 반복 입력의 한 번 결과를 결정적 재현율로 일반화하는가 — 문서는 두 개의 10-action run 관찰로 한정하고 일반 비율을 주장하지 않는다.
15. XCTest method 성공이 10 action callback 전부 성공으로 혼동되는가 — method 1/1과 callback 7/10·8/10을 분리한다.
16. per-action 자료가 사라져 요약 수치를 재확인할 수 없는가 — Run 02·03 attachment와 manifest를 README에서 직접 연결한다.
17. summary만으로 XCTest 결과 트리를 재검증할 수 없는가 — 각 run에 summary, tests tree, activity tree, xcodebuild log가 있다.
18. 경계에 도달한 Run 01을 방향 비교 근거로 섞었는가 — Run 01은 boundary-confounded로 표에 표시하고 방향 판정에서 제외한다.
19. 시뮬레이터 식별 정보가 다른 run과 혼합됐는가 — 장치 모델, iOS build와 UDID를 한 환경 블록에 고정했다.
20. Simulator 결과가 실제 외장 trackpad나 물리 기기 결과로 과장되는가 — README와 plan에서 둘 다 미검증으로 남긴다.
21. discrete mouse나 Control-wheel까지 이번 실험이 검증한 것처럼 보이는가 — 범위 설명은 continuous XCTest 입력에 한정한다.
22. UIKit 대조가 제품 adapter의 무결성을 증명하는가 — 명시적으로 제품 adapter 결함을 배제하지 않는다고 기록했다.
23. 누락 위치를 XCTest·Simulator·UIKit 중 하나로 추측해 확정했는가 — 정확한 단계는 unknown으로 남긴다.
24. 대조 결과만으로 C13.3을 완료 처리했는가 — 상태 대장·계획·로드맵 모두 미완료다.
25. 공식 완료 수가 새 실험에 따라 임의 변경됐는가 — `74/207` 유지가 관련 문서에 명시된다.
26. 계획 문서의 Run 02·03 사실이 README와 어긋나는가 — run별 방향별 수와 제한을 README에 맞춰 대조했다.
27. 상태 대장의 요약이 제품 코드 변경으로 오해되는가 — 독립 호스트 결과이고 제품 코드 변경이 없다고 쓴다.
28. 로드맵 링크가 오래된 단일 Run 결과만 가리키는가 — standalone 대조 README와 계획 링크를 추가했다.
29. 문서 전달 성공을 단순 로컬 복사로만 판단했는가 — pass-1에서 95개 복사 hash, 18개 backup hash, HTTP 200·응답 hash 95개를 각각 확인했다.
30. Browser 캡처를 제품 UI callback 증거로 오인할 수 있는가 — 캡처는 roadmap 문서라고 표시하고, callback 판단은 Simulator 원본 관찰에 연결한다.

검토 중 제품 동작이나 공식 완료 상태에 대한 과장·불일치는 발견하지 않았다. 문서 렌더는 선택 route 범위만 통과했으며 전체 docs build는 별도 기존 SSR·Shiki·image import 오류로 실패한 사실을 결과 기록에 남긴다.
