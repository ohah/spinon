# C11.4 image auto/max 병합 상태 동기화 PR 변경 검토 30개

- 검토 범위: `spec/STATUS.md`의 PR #643 결과 행, `plan/c11-4-image-auto-max-mixed-track.md`의 최신 병합·계획 상태 문구, 이 기록에 연결되는 merge/evidence 참조
- 기준 main: `4be4035a38a97df7a4215605e8ddaba4d75297e5` (PR #643 rebase merge)
- 검토 유형: 문서 변경 실패 경로 검토. 기능 구현·시뮬레이터·CI를 다시 실행한 검토가 아니다.

1. PASS — 병합 대상 PR 번호가 `#643`이며 GitHub PR이 `MERGED` 상태인 것을 대조했다.
2. PASS — merge 시간 `2026-10-26T02:19:53Z`가 GitHub 기록과 일치한다.
3. PASS — 병합 방식을 `Rebase and merge`로 적었고 squash/merge commit으로 잘못 부르지 않았다.
4. PASS — 문서의 merge SHA `4be4035a38a97df7a4215605e8ddaba4d75297e5`가 GitHub의 merge commit과 일치한다.
5. PASS — merge commit과 실행 source `cac4e339`를 구분해 artifact source를 merge SHA로 오기하지 않았다.
6. PASS — 최신 실행 label `final05`가 해당 source run directory로 연결된다.
7. PASS — 공식 완료 수 `72/213`을 유지했다.
8. PASS — C11.4 상위 완료 체크를 바꾸거나 전체 완료로 승격하지 않았다.
9. PASS — image-only 제한 fixture임을 밝혀 일반 CSS admission과 혼동하지 않는다.
10. PASS — Android 기기를 API 37 ARM64 AVD로 정확히 한정했다.
11. PASS — iOS 기기와 OS를 iPhone 17 Pro / iOS 26.2 Simulator로 정확히 한정했다.
12. PASS — 두 플랫폼 실행 결과를 각각 기록해 한 플랫폼 결과가 다른 쪽을 가리지 않는다.
13. PASS — root frame `100×40`이 계획 입력의 root geometry와 맞는다.
14. PASS — image item frame `24×16`이 PNG natural size와 맞는다.
15. PASS — `minmax(auto, max-content) 17px` track pattern과 5px column gap을 기록해 slice의 재현 입력을 보존한다.
16. PASS — strict intrinsic-input matcher와 trace matcher를 실행 source revision에 묶었다.
17. PASS — trace를 6-stage raw track trace로 한정해 전체 Grid algorithm 지원처럼 쓰지 않는다.
18. PASS — `status=0`와 draw/present를 기록하되 이를 픽셀 동등성으로 확대하지 않는다.
19. PASS — `runtime_image_count=0`와 `image_draw_count=0`을 명시해 PNG texel paint가 검증되지 않은 점을 남긴다.
20. PASS — Agent Browser에서 PR screenshot 링크를 확인한 사실을 별도 링크 확인으로만 취급하고 paint 증거로 바꾸지 않는다.
21. PASS — overflow 동작이 범위 밖임을 유지한다.
22. PASS — 일반 CSS admission이 범위 밖임을 유지한다.
23. PASS — WPT 적합성을 확인한 것으로 쓰지 않는다.
24. PASS — Android 실기기 검증을 완료한 것으로 쓰지 않는다.
25. PASS — simulator 결과를 hardware GPU 결과로 부르지 않는다.
26. PASS — PR의 빈 GitHub check rollup을 기록하고 CI 성공 주장을 만들지 않는다.
27. PASS — 문서가 공개 API나 내부 계약 버전 변경을 암시하지 않는다.
28. PASS — STATUS에서 계획으로 가는 상대 링크가 `spec/` 기준 경로와 일치한다.
29. PASS — 계획 문서의 evidence 및 review 상대 링크가 기존 파일 위치와 일치하며 새 지원 계약을 만들지 않는다.
30. PASS — 변경 범위가 해당 image slice의 merge/status 동기화에 한정되고 C11.4의 다른 slice, 문서 사이트 배포, preview 배포를 건드리지 않는다.

## 판정

30개 문서 검토 관점에서 merge provenance·공식 집계·fixture 범위·검증 한계에 대한 문서화 결함을 찾지 못했다. `git diff --check`는 별도 저장소 형식 검사로 확인한다. 이 검토는 CI, 런타임, 시뮬레이터, pixel output을 재실행하지 않았다.
