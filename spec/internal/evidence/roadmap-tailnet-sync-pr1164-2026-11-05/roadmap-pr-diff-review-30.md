# PR #1164 병합 후 상태·로드맵·Tailnet 변경 적대 검토 30개

범위는 PR #1164의 병합 기록을 `STATUS.md`, 내부 계약 0059, 공개 로드맵에 반영하고, 해당 로드맵·상태·계약·계획·근거 route를 기존 Tailnet 미리보기에 동기화하는 문서 변경이다. 각 검토는 서로 다른 데이터·링크·배포 실패 경계를 확인한다.

1. **공식 완료 수가 후보 병합으로 증가하는가 — PASS.** 세 문서는 `78/214`를 유지한다. #1164 bounded slice를 새 공식 완료로 세지 않는다.
2. **C11 상위 checkbox가 완료로 바뀌는가 — PASS.** STATUS의 C11 checkbox는 `[ ]`다.
3. **C11.1 하위 checkbox가 완료로 바뀌는가 — PASS.** 제한 구현 설명과 별개로 해당 항목은 미완료 상태다.
4. **병합 날짜가 로컬 시각과 UTC 사이에서 뒤집히는가 — PASS.** GitHub의 `2026-11-04T20:24:23Z`를 원문 그대로 기록한다.
5. **merge SHA가 PR head SHA로 잘못 기록되는가 — PASS.** 문서의 `3e5aed60…`는 GitHub `mergeCommit.oid`와 일치한다.
6. **구현 head가 merge SHA와 혼동되는가 — PASS.** 내부 계약은 구현 head `20b9a1a…`를 별도 표기한다.
7. **PR 상태가 아직 열린 후보로 남는가 — PASS.** STATUS와 계약은 Rebase and merge 완료를 기록한다.
8. **비어 있는 check rollup을 CI 성공으로 해석하는가 — PASS.** `statusCheckRollup: []`와 맞춰 check run 미보고라고만 쓴다.
9. **계획 PR #1163이 구현 PR #1164와 합쳐지는가 — PASS.** 로드맵은 계획 PR을 별도 링크로 유지한다.
10. **#1161과 #1164의 merge SHA가 뒤바뀌는가 — PASS.** 각 카드의 SHA는 GitHub의 개별 merge metadata와 일치한다.
11. **#1161의 기존 제한 범위를 #1164 범위로 덮어쓰는가 — PASS.** 두 roadmap 단락과 evidence 경로가 분리되어 있다.
12. **finite-positive 값 확장이 임의 ratio 지원으로 과장되는가 — PASS.** 기존 owner/topology 내 typed ratio 범위로 한정한다.
13. **기본 Root HTML fixture를 일반 author CSS 지원으로 확대하는가 — PASS.** STATUS·계약·로드맵 모두 public CSS와 구분한다.
14. **두 fixture의 oracle 수가 빠지거나 바뀌는가 — PASS.** 14-scene 주 oracle과 2-scene 자동 최소 크기 보충을 구분해 적었다.
15. **Rust focused 결과가 broad-suite 성공으로 표현되는가 — PASS.** `6+1+1+1` focused 결과와 broad failure 수를 별도로 둔다.
16. **layout broad suite가 전부 green처럼 보이는가 — PASS.** `441/442`의 남은 실패를 그대로 기록한다.
17. **style-to-layout broad suite가 전부 green처럼 보이는가 — PASS.** `391/401`의 기존 assertion 실패를 그대로 기록한다.
18. **AVD가 Android 실기기로 묘사되는가 — PASS.** Android API 37 ARM64 AVD임을 명시한다.
19. **iOS Simulator가 iOS 실기기로 묘사되는가 — PASS.** iPhone 17 Pro/iOS 26.2 Simulator임을 명시한다.
20. **draw/present 도달을 픽셀 일치로 확대 해석하는가 — PASS.** visual smoke와 readback 부재를 분리한다.
21. **Metal 결과가 compositor 이후 결과로 오인되는가 — PASS.** compositor parity를 확인하지 않았다고 표시한다.
22. **Android surface readback을 PixelCopy 검증으로 오인하는가 — PASS.** PixelCopy를 수행하지 않았다고 표시한다.
23. **내부 계약 숫자 버전이 문서 동기화만으로 올라가는가 — PASS.** `0.1.0`을 유지한다.
24. **#1161 계획 route가 누락되는가 — PASS.** 선택 root에 tracked 계획 원본을 포함하고 출력 route를 검사한다.
25. **#1164 계획 route가 누락되는가 — PASS.** 선택 root에 tracked 계획 원본을 포함하고 출력 route를 검사한다.
26. **#1161 실행 근거 링크가 #1164 근거를 잘못 가리키는가 — PASS.** 각 카드와 contract entry가 자신의 tracked evidence subtree를 가리킨다.
27. **새 #1164 Android screenshot link가 빠지는가 — PASS.** 로드맵은 해당 runtime evidence의 Android 캡처를 가리킨다.
28. **새 #1164 iOS screenshot link가 빠지는가 — PASS.** 로드맵은 해당 runtime evidence의 iOS 캡처를 가리킨다.
29. **공개 로드맵 집계와 공식 STATUS 집계가 달라지는가 — PASS.** 두 source 모두 `78/214`를 유지한다.
30. **Tailnet 동기화가 기존 서버를 재시작하거나 파일을 삭제하는가 — PASS.** 기존 PID를 유지하고 선택 route만 사전검사·백업·원자 교체하며 삭제하지 않는다.

대조 출처는 현재 GitHub PR #1161/#1164 merge metadata, tracked source files, selected Rspress build, route/hash manifest, local·HTTPS served verification, 그리고 Agent Browser의 desktop/mobile DOM 관찰이다. 전체 문서 사이트의 dead-link completeness를 주장하지 않는다.
