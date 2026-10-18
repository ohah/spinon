# C13.3 iOS adapter Tailnet 문서 PR 변경 관점 검토 30개 · 2026-10-18

1. 구현 merge SHA가 PR #298의 Rebase and merge 결과 `3024f6c9e6fa726e4eeae44d952f56ab0a3ec6af`와 일치하는가 — status·index·로드맵 메모에서 같은 값으로 고정했다.
2. 부분 구현을 공식 완료로 올리지 않았는가 — `STATUS.md`의 C13.3 상위 체크는 변경하지 않았다.
3. 이 PR에서 공식 완료 수를 바꾸지 않았는가 — 체크 항목 추가 완료 없이 상태 설명과 Tailnet 사본만 갱신한다.
4. 실행한 UI method 수와 adapter callback 수를 혼합하지 않았는가 — 4/4 UI method와 2/4 callback을 각각 기록한다.
5. callback이 관찰되지 않은 두 방향을 실패 없는 런타임 성공으로 표시하지 않았는가 — 성공 판정을 부여하지 않는다.
6. negative-Y 표본이 queue 처리된 뒤 boundary no-op이었다는 제한이 보이는가 — revision·offset이 유지됐다고 적는다.
7. positive-X에서 실제 확인된 범위만 기록했는가 — target 7, revision 1, inner X `+19.394547 CSS px`로 한정한다.
8. feature-off 경계를 과장하지 않았는가 — default executable/archive에서 marker와 FFI symbol이 없었다는 근거로 제한한다.
9. public `WheelEvent` 또는 전체 CSS 입력 지원이 추가된 것처럼 보이지 않는가 — 공개 event 미구현을 계속 명시한다.
10. iPadOS Simulator 결과를 실기기 검증으로 확대하지 않았는가 — device 범위를 Simulator로 표기한다.
11. discrete mouse가 검증됐다고 읽힐 여지가 없는가 — 미검증으로 남긴다.
12. Control-wheel runtime을 확인한 것으로 읽힐 여지가 없는가 — 미검증으로 남긴다.
13. 반복 가능한 네 방향 callback 행렬을 주장하지 않는가 — 전체 ±X/±Y는 미검증으로 기록한다.
14. 다른 UIKit point scale 결과가 일반화되지 않았는가 — 다른 scale은 미검증으로 남긴다.
15. 실제 물리 trackpad 결과가 섞이지 않았는가 — 실제 trackpad와 실기기를 미검증으로 분리한다.
16. C13.3 상위 항목과 S04.10의 미완료가 유지되는가 — 로드맵 문구에 C13.3 미완료를 명시한다.
17. 계획 페이지가 같은 iOS 결과를 가리키는가 — 업데이트된 native wheel 계획 route를 확인했다.
18. 상태 대장에 구현 결과와 한계가 연결됐는가 — PR #298 및 실행 근거 링크를 유지한다.
19. 내부 인덱스에서 이전 preflight 증거와 새 adapter 동기화 증거를 혼동하지 않는가 — 링크 레이블을 분리했다.
20. custom roadmap이 최신 adapter 기록을 실제로 노출하는가 — 별도 C13.3 iOS 문구를 추가하고 Tailnet route에서 확인한다.
21. roadmap의 상태 수가 정적으로 과거 값에 고정되지 않는가 — 공식 `STATUS.md`를 `main`에서 불러오는 동작을 유지한다.
22. Tailnet screenshot이 simulator screenshot으로 오인되지 않는가 — Agent Browser 문서 화면 증거라고 명시한다.
23. simulator scene 증거와 문서 화면 증거가 구분되는가 — 원래 adapter README의 scene 및 runtime log를 별도 링크한다.
24. Tailnet의 계획 route가 올바른 문서인지 확인했는가 — Agent Browser에서 제목과 iOS 요약을 읽었다.
25. Rspress 버전이 프로젝트 고정값과 같은가 — `2.0.22`를 사용한다.
26. 선택 빌드의 범위를 전체 docs 적합성으로 표현하지 않는가 — 제한된 route 렌더로만 기록한다.
27. 선택 root에서 링크·이미지 검사를 비활성화한 한계를 숨기지 않는가 — 전체 dead-link/image 검증은 주장하지 않는다.
28. 교체 전 파일을 보존했는가 — 각 교체 대상 backup의 SHA-256을 원본과 비교한다.
29. 동기화가 무관한 파일을 삭제하지 않았는가 — 복사 manifest의 삭제 수를 0으로 기록한다.
30. 성공 판정이 파일 복사 완료만으로 끝나지 않는가 — 로컬/Tailnet HTTP status·served hash 및 Agent Browser의 동적 상태·검색 자산 결과를 함께 확인한다.
