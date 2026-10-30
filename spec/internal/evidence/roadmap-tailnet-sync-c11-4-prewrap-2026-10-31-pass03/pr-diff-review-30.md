# C11.4 PreWrap 병합 상태와 Tailnet final sync PR diff 실패 관점 30개

계획 검토나 #870 구현 검토를 재사용하지 않고, 이번 문서 PR의 status reconciliation, Tailnet preflight/copy/verification과 브라우저 lifecycle 변경을 별도로 확인했다.

1. **계획 PR과 구현 PR 혼동** — 계획 #869와 제한 구현 #870을 각각 병합 기록했다. 통과.
2. **PR state가 실제와 다름** — GitHub PR #870 조회 결과 `MERGED`다. 통과.
3. **merge timestamp/timezone 오류** — `2026-10-30T18:51:40Z`를 GitHub `mergedAt`에서 기록했다. 통과.
4. **merge SHA 오기** — `5265793d71a02a6206a09b9bbbc9658696dfd992`가 GitHub `mergeCommit.oid` 및 main HEAD와 일치한다. 통과.
5. **병합 전 문서에서 병합 주장** — merge ancestor 확인 후 문서와 Tailnet publication을 만들었다. 통과.
6. **PR head 변경 범위 누락** — GitHub PR diff와 로컬 branch patch의 changed lines가 일치했으며 hunk label 차이를 분리했다. 통과.
7. **C11 상위 checkbox 승격** — C11 미완료를 유지한다. 통과.
8. **C11.4 checkbox 승격** — C11.4 미완료를 유지한다. 통과.
9. **공식 완료 count 변동** — STATUS와 Tailnet 두 페이지에서 `72/213`을 유지한다. 통과.
10. **CI success 과장** — check rollup은 비었고 main required status checks는 없다. CI pass를 주장하지 않는다. 통과.
11. **workspace test 실패 누락** — C07.3 pinned-Chromium string mismatch의 expected/actual을 문서에 유지한다. 통과.
12. **workspace 실패를 구현이 유발했다고 단정** — 변경 밖 crate 실패로 적고 base에서의 결과는 미확정으로 둔다. 통과.
13. **일반 CSS 지원 과장** — 구현 범위를 opt-in fixture로 한정한다. 통과.
14. **root HTML parity 과장** — root-HTML white-space mapping은 남은 범위로 유지한다. 통과.
15. **일반 `tab-size` 지원 과장** — 고정 unit-test profile 밖 지원을 주장하지 않는다. 통과.
16. **Android simulator를 실기기로 표현** — API 37 ARM64 AVD로 한정한다. 통과.
17. **iOS simulator를 실기기로 표현** — iPhone 17 Pro/iOS 26.2 Simulator로 한정한다. 통과.
18. **시뮬레이터 캡처를 glyph pixel 증거로 사용** — 캡처의 실제 범위를 설명한다. 통과.
19. **static archive 결과를 APK/IPA 전체 제외로 확대** — archive 검사의 경계를 명시한다. 통과.
20. **선택 publication을 전체 docs build로 확대** — manifest와 README에 full link validation이 아님을 기록한다. 통과.
21. **잘못된 publication source** — RSPress 2.0.22와 selected-root source files의 hash를 manifest에 묶었다. 통과.
22. **RSPress output과 publication manifest 불일치** — rebuilt site의 340 route를 manifest bytes/hash와 비교해 일치했다. 통과.
23. **route path traversal** — script가 absolute path, `..`, backslash와 symlink를 거부한다. 통과.
24. **selected root overwrite** — root 구성기가 기존 manifest/build path를 덮어쓰지 않는다. 통과.
25. **open handle 대상 덮어쓰기** — preflight에서 target open handle 0을 확인했다. 통과.
26. **기존 target 미백업** — 교체 20개를 backup하고 각각 source hash와 대조했다. 통과.
27. **의도치 않은 삭제** — copy/verify report는 noDeletes를 기록하고 삭제를 하지 않았다. 통과.
28. **HTTPS route 누락·오류** — 340/340 URL이 HTTP 200이다. 통과.
29. **served response 손상** — 340/340 byte count 및 SHA-256이 publication과 일치한다. 통과.
30. **브라우저 검증 누락 또는 프로세스 잔류** — Agent Browser에서 로드맵, STATUS, runtime evidence를 확인하고 close/close-all 뒤 Agent Browser/Chrome 잔류 프로세스가 없었다. 터미널 브라우저는 쓰지 않았다. 통과.

**판정:** 변경 문구와 선택 Tailnet 배포에서 blocker를 찾지 못했다. 이는 전체 문서 링크나 전체 C11 완료 판정이 아니다.
