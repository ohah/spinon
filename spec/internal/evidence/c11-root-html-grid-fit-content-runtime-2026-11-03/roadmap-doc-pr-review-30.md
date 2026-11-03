# PR #1093 병합 후 상태·로드맵 문서 PR 검토 30개 · 2026-11-03

검토 대상은 `plan/c11-root-html-grid-fit-content.md`, `spec/STATUS.md`, `spec/internal/0059-c11-grid-layout.md`, 이 디렉터리의 runtime README/hash 목록, `spec/public/roadmap.html`이다. 기준 main은 PR #1093의 병합 tree(`24610c5731b66f076908b2ecf7cd5c4104729369`)다. 구현 PR의 기존 30개 검토를 재사용하지 않고, 문서·상태·배포 실패 경계를 따로 확인했다.

1. **PASS — C11 상위 완료 표시:** 상태 대장 C11 부모 항목은 계속 `[ ]`다.
2. **PASS — 제한 slice 승격:** 새 `fit-content(px)` 기록은 완료 checkbox가 아닌 날짜별 하위 근거다.
3. **PASS — 공식 완료 수:** 공식 `72/213`을 유지하며 증가시키지 않았다.
4. **PASS — 공식 상태 변경 면적:** STATUS diff에는 새 C11.1 근거 한 줄만 추가했고 기존 집계를 고치지 않았다.
5. **PASS — PR 상태:** `gh pr view 1093`에서 `MERGED`를 확인했다.
6. **PASS — merge SHA:** GitHub 반환값과 STATUS·plan·contract·runtime README가 모두 `24610c5731b66f076908b2ecf7cd5c4104729369`로 일치한다.
7. **PASS — merge 시각:** GitHub 반환값과 문서가 `2026-11-03T00:25:34Z`로 일치한다.
8. **PASS — merge 방식:** 기록은 실제 PR의 Rebase and merge 방식과 일치한다.
9. **PASS — PR checks:** API의 `statusCheckRollup`은 빈 배열이며 문서는 CI 통과를 주장하지 않는다.
10. **PASS — required checks:** main branch protection endpoint는 HTTP 404 `Required status checks not enabled`를 반환했다.
11. **PASS — plan 상태 문구:** 이전 “검토 대기/병합 남음” 문구를 삭제하고 병합 결과로 바꿨다.
12. **PASS — runtime README 상태:** 제목과 본문을 “merged bounded implementation”으로 고쳐 후보가 열린 것처럼 보이지 않게 했다.
13. **PASS — 내부 계약 버전:** 계약 0059 숫자 버전은 `0.1.0`으로 유지했다.
14. **PASS — API 경계:** 기존 PR과 같이 공개 API·FFI ABI·parser·runtime profile이 추가되지 않았음을 문서에서 유지한다.
15. **PASS — runtime 경로:** 기본 parsed Root HTML 경로로 한정하고 일반 author CSS admission으로 넓히지 않았다.
16. **PASS — 명시 track 범위:** 새 지원은 explicit rows/columns의 허용 track에 한정했다.
17. **PASS — repeat 범위:** repeat 지원을 fixed integer `repeat()` expansion으로 제한했다.
18. **PASS — cap 값 범위:** 유한·비음수 CSS px `fit-content()` cap만 기록한다.
19. **PASS — named area 구조:** 기존 제한 map과 dimension-matched track 조합, area별 item 요구를 그대로 적었다.
20. **PASS — named area gap guard:** 기존 zero-gap 제한을 유지한다고 명시했다.
21. **PASS — percentage 제외:** `fit-content(%)`를 이번 구현이 지원한다고 오해할 문구가 없다.
22. **PASS — math 제외:** `calc()`/typed math limit은 지원 범위 밖이다.
23. **PASS — implicit sizing 제외:** implicit `grid-auto-*`의 `fit-content()`는 추가하지 않았다.
24. **PASS — auto sizing 제외:** auto/intrinsic sizing을 구현 범위에 넣지 않았다.
25. **PASS — conformance 경계:** 일반 CSS admission과 WPT 적합성을 완료로 주장하지 않는다.
26. **PASS — focused test 집계:** 223개 focused Rust test 수치는 기존 README 집계와 일치하며, 별도 4개 runtime test를 중복 더하지 않는다.
27. **PASS — Rust 품질 검사:** scoped Clippy, rustfmt 및 diff check의 범위를 기존 실행 근거와 맞췄다.
28. **PASS — Android 해석:** PixelCopy 7/7 exact를 기록하고 AVD SwiftShader software renderer임을 분리했다.
29. **PASS — iOS 해석:** Metal drawable 7/7 표본을 compositor 제출 전 결과로 한정했다.
30. **PASS — 로드맵 표시·좁은 화면·route wiring:** 정적 roadmap 새 카드의 PR·SHA·범위·미완료 상태와 네 내부 href의 build route를 확인했다. 선택 RSPress build는 roadmap·상태·계약·계획·runtime 근거·PR 검토·문서 검토를 포함한 7개 page route를 만들었다. Agent Browser 1440×1000에서 document/body width 1440px, 390×844에서 각각 390px이고 카드 내부는 362/362px여서 수평 overflow가 없었다. build report `selected-docs-build-report-final-2026-11-03-pass02.json` · [Desktop](./roadmap-doc-pr-candidate-desktop-2026-11-03.png) · [Mobile](./roadmap-doc-pr-candidate-mobile-2026-11-03.png).

## 별도 build 결과

`mise exec -- bun run docs:build`는 exit 1이다. 표준 RSPress root가 `spec/`만 포함해 프로젝트 root의 `plan/` route를 찾지 못하고, 여러 `.log`·`.png` 상대 artifact 링크도 `.html` page route로 처리해 실패한다. 전체 docs build 성공으로 표시하지 않는다. 별도 Tailnet 선택 root에는 필요한 `plan/` route를 포함했다. Rspress 2.0.22 selected build는 7개 page route와 네 roadmap href 대상이 생성됨을 확인했지만 `checkDeadLinks`/`checkDeadImages`는 꺼져 있으므로 전체 link/image validation으로 보지 않는다. 추가 Markdown 경로도 파일 존재를 따로 확인했다.

판정: 위 30개 문서·상태 경계는 통과했다. 전체 RSPress build는 독립적인 전역 dead-link 문제로 실패했으며 해당 제한을 유지한다.
