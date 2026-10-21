# C11.4 PR #453 병합 상태 문서 PR 적대 검토 30개

PR #453 병합 뒤 상태 대장·계획·내부 인덱스를 현재 사실에 맞게 고치는 diff를 30개 서로 다른 사실·계약 관점에서 확인했다. Tailnet pass38은 별도로 진행 중이며 여기서는 완료로 표시하지 않았다.

1. **PR identity** — 모든 문서가 구현 PR #453을 가리킨다.
2. **GitHub state** — PR API의 `MERGED` 상태와 본문 서술이 일치한다.
3. **merge strategy** — Rebase and merge를 명시하며 squash나 merge commit으로 잘못 기록하지 않는다.
4. **merge SHA** — 문서의 `619a99209ec38d838c90428b8d153cf24a1aa0d3`가 GitHub `mergeCommit.oid`와 맞다.
5. **merge time** — `2026-10-21T15:46:18Z`를 KST `2026-10-22 00:46:18`로 변환했다.
6. **base branch** — 병합 대상은 `main`이며 다른 유지 브랜치로 표기하지 않는다.
7. **head commit** — 병합 직전 PR head는 `014a7fb07e4f0321351e8acff974c9a1f9c588ab`로 확인했다.
8. **status rollup** — 병합 시 `statusCheckRollup`이 빈 배열이었다고 적어 CI 통과로 과장하지 않는다.
9. **required check** — main required status check API가 미설정 응답을 반환한 사실과 모순되지 않는다.
10. **C11.4 completion** — 상위 C11.4 checkbox는 계속 unchecked다.
11. **official count** — `72/213` 집계는 바꾸지 않았다.
12. **feature scope** — 네 fixed fixture scene에 대한 제한이라는 표현을 유지한다.
13. **scene count** — 공식 지원 scene 수나 전체 Grid case 수로 확장해 쓰지 않는다.
14. **axis scope** — width/height와 Columns/Rows 동시 sizing이라는 기능 요약이 변경과 맞다.
15. **depth scope** — depth-one control과 depth-two nested Grid를 구분한다.
16. **platform facts** — Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator 기록은 runtime 근거와 일치한다.
17. **physical-device claim** — Android 실기기 검증을 완료한 것으로 쓰지 않는다.
18. **WPT claim** — WPT/full CSS Grid 적합성을 완료했다고 쓰지 않는다.
19. **pixel claim** — Simulator PNG를 glyph equality나 compositor-after proof로 부르지 않는다.
20. **fixture feature** — 동작을 일반 runtime CSS admission으로 표현하지 않는다.
21. **public API** — 공개 API 추가 또는 명세 완료를 주장하지 않는다.
22. **contract version** — 내부 계약 0059의 0.1.0 유지 사실을 훼손하지 않는다.
23. **plan status** — 계획 문서는 구현 PR merge 완료와 Tailnet 작업 중 상태를 분리한다.
24. **status row** — `STATUS.md`는 PR merge 사실을 기록하되 미완료 상위 항목을 완료로 바꾸지 않는다.
25. **internal index** — PR 및 근거 링크가 같은 evidence 경로를 사용한다.
26. **relative link root** — Markdown relative paths는 `spec/internal/README.md` 기준으로 계산했다.
27. **evidence SHA list** — 추가 review file을 SHA256SUMS에 넣어 오래된 manifest가 남지 않게 한다.
28. **Tailnet completion** — preview 갱신 전이므로 in-progress로 남겨 둔다.
29. **Pages deployment** — GitHub Pages 배포를 언급하거나 실행하지 않는다.
30. **source freshness** — 문서 PR은 병합된 `main`에서 분기하며 feature code를 다시 포함하지 않는다.

**검토 결과:** 병합 상태와 제한 범위를 과장하는 문구를 찾지 못했다. 이 문서 PR의 목적은 merge 사실을 기록하고 Tailnet 갱신 전후 상태를 분리하는 데 한정된다.
