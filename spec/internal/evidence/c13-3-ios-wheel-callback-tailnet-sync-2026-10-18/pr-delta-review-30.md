# C13.3 iOS wheel callback Tailnet 동기화 PR 변경 적대 검토 30개 · 2026-10-18

1. 기준 PR이 공개 문서에서 #302와 일치하는가 — 본문과 roadmap note가 모두 #302를 참조한다.
2. 기록한 rebase merge SHA가 실제 기준 commit인가 — `ec84242d8261a64489c3ed621ede69d61c38e492`를 source base로 사용한다.
3. Run 04 negative-only callback 전달을 실제 scroll 소비와 혼동하지 않는가 — 세 건 모두 offset 0, `published=false`로 구분한다.
4. Run 04 XCTest 성공과 app callback/process 관찰을 분리하는가 — 각 3/3 수치를 따로 기록한다.
5. Run 05 XCTest 5/5를 native callback 성공률로 확대하지 않는가 — UI method와 callback을 별도 숫자로 쓴다.
6. Run 05 positive-X callback 수가 5회 반복과 맞는가 — callback/process 모두 5/5다.
7. Run 05 negative-X 누락 수가 숨겨지지 않았는가 — 3/5와 반복 1·5 누락을 모두 표기한다.
8. 요청한 XCTest delta ±38을 UIKit 실측치인 것처럼 다루지 않는가 — 실제 전달 delta와 다르다고 적는다.
9. 전달된 reverse 세 건이 offset 0 복귀를 보장했다고 주장하지 않는가 — 최종 offset 목록과 미복귀 판정을 유지한다.
10. Run 06의 저장 공간 실패를 성공으로 집계하지 않는가 — 결과 unknown·완료 테스트 0으로 제외한다.
11. Run 06 partial log의 callback을 테스트 pass 수로 세지 않는가 — README와 roadmap에서 gate 밖이라고 분리한다.
12. 누락 callback의 어느 층이 원인인지 추측으로 확정하지 않는가 — XCTest synthesis·Simulator delivery·UIKit recognizer를 미분리로 남긴다.
13. 문서 후속이 app code 수정이나 새 API 계약을 암시하지 않는가 — 문서 범위와 C13.3 미완료를 명시한다.
14. C13.3 상위 완료 체크가 바뀌지 않았는가 — STATUS 및 roadmap은 계속 미완료다.
15. 공식 완료 수를 로컬 예상치로 덮어쓰지 않았는가 — `74/207`을 Browser에서 동적으로 확인하고 기존 값과 맞춘다.
16. Android나 실제 iOS 기기 근거를 이번 PR에 잘못 포함하지 않는가 — 대상은 iPadOS Simulator 기록임을 유지한다.
17. 실제 trackpad와 simulator 합성 입력 경계를 보존하는가 — 물리 trackpad/device는 미검증으로 쓴다.
18. simulator 화면 캡처를 callback proof로 오인하게 만들지 않는가 — 문서 screenshot은 동기화 증거라고 구분한다.
19. 변경된 roadmap note에서 plan과 evidence link가 유효한 route를 가리키는가 — 세 경로를 Agent Browser에서 연다.
20. STATUS와 public roadmap의 Run 04–06 설명이 서로 충돌하지 않는가 — offset 0, callback count, Run 06 제외 사유를 대조한다.
21. README의 raw evidence 참조가 원본 JSON·log로 이어지는가 — Run 05 summary의 JSON route를 Agent Browser에서 확인한다.
22. 정적 HTML 갱신이 RSPress rendered page와 충돌하지 않는가 — `roadmap.html` source를 직접 출력 tree에 복사한다.
23. 선택 렌더의 범위를 전체 docs 성공으로 과장하지 않는가 — 선택 route 수, 비활성화된 검사와 미실행 Pages 배포를 기록한다.
24. RSPress 버전이 project pin과 일치하는가 — `2.0.22`만 사용한다.
25. preview 기존 파일 교체 전에 별도 백업이 만들어지는가 — 모든 replaced 경로의 old hash를 먼저 기록한다.
26. 백업본이 기존 파일과 동일한가 — 각 backup의 byte 수와 SHA-256을 대조한다.
27. 복사 과정에서 기존 파일이나 사용자가 만든 파일을 삭제하지 않는가 — copy-only 처리로 삭제 0을 유지한다.
28. 기존 Tailnet preview 서비스가 동기화 중 중단되는가 — listener를 재시작하지 않고 같은 preview root를 갱신한다.
29. 성공 판정이 파일시스템 복사 결과만 보는가 — Tailnet HTTP 200과 response body SHA를 각 경로에서 비교한다.
30. Agent Browser·tmux 종료 범위를 혼동하는가 — Agent Browser 프로세스만 종료·확인하고 tmux와 preview listener는 유지한다.
