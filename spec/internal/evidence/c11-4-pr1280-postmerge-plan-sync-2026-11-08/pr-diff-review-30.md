# C11.4 PR #1280 병합 후 계획 동기화 PR diff 검토 30개

검토 대상은 `plan/c11-css-grid.md`, `plan/c11-4-grid-track-sizing.md`, `plan/c11-4-root-html-intrinsic-auto-rows-max-content.md` 및 이 검토 기록이다. 각 항목은 현재 PR diff와 공식 상태·PR #1280 기록을 대조했다.

1. **PASS — PR 식별자:** C11 overview 및 track-sizing 계획은 해당 limited implementation PR #1280을 가리킨다.
2. **PASS — 병합 시각:** `2026-11-08T04:28:20Z` 표기는 GitHub metadata 및 `spec/STATUS.md`와 일치한다.
3. **PASS — merge SHA:** 두 계획의 `b1b27b02491f1614ded260fd3b66f50184c5bb80` 값이 GitHub와 공식 대장 값에 일치한다.
4. **PASS — 병합 방식:** `Rebase and merge` 표기는 PR 기록과 일치한다.
5. **PASS — 지원 profile:** overview 계획은 제한 범위를 `RootHtmlFitContentAutoRowsV1` parsed Root HTML 경로로 한정한다.
6. **PASS — track list:** 네 track type이 각각 한 번 나타나는 24개 순서로 적었다.
7. **PASS — cap 단위:** px/% cap 지원을 기록하고 arbitrary cap 문법을 암시하지 않는다.
8. **PASS — percentage basis:** 별도 definite basis 경계 밖의 보편적 percentage 지원을 주장하지 않는다.
9. **PASS — alignment:** `start`/`stretch`만 해당 slice의 검증 설명에 포함된다.
10. **PASS — Chrome denominator:** Chrome 157 independent comparison 96/96은 사전 비교 기록과 일치한다.
11. **PASS — Layout denominator:** Layout 12/12는 공식 대장 및 실행 기록과 일치한다.
12. **PASS — projection denominator:** 96 typed cap 조합을 전체 Style suite 성공으로 확대하지 않는다.
13. **PASS — RuntimeSession denominator:** focused RuntimeSession 7/7만 적었다.
14. **PASS — overflow denominator:** fit-content cap overflow 48/48은 24 순서×px/% 경계 비교다.
15. **PASS — Android 기기 구분:** API 37 ARM64 AVD 결과를 실기기로 부르지 않는다.
16. **PASS — iOS 기기 구분:** iPhone 17 Pro/iOS 26.2 Simulator 결과를 실기기 실행으로 부르지 않는다.
17. **PASS — fixture 일치:** 두 플랫폼 기본 route가 같은 fixture hash를 사용한 사실만 기록한다.
18. **PASS — box 수:** 기본 route의 14 boxes 값은 공식 대장 실행 기록과 일치한다.
19. **PASS — 화면 증거:** OS screenshot을 visual smoke로 분류한다.
20. **PASS — readback 한계:** PixelCopy/Metal surface pixel equality를 확인하지 않았다고 명시한다.
21. **PASS — 구현 리뷰:** implementation/runtime 20개가 PR #1280 기록에 있는 것과 일치한다.
22. **PASS — 계획 리뷰:** 계획 수정 검토 20개를 제품 구현 리뷰와 분리해 기록한다.
23. **PASS — PR diff 리뷰:** PR #1280의 30개 검토 완료 상태와 혼동하지 않도록 이번 문서 PR 검토도 별도 파일로 둔다.
24. **PASS — CI 경계:** 빈 workflow/status 결과를 CI green으로 주장하지 않는다.
25. **PASS — intrinsic 한계:** 고정 높이 payload의 동일 geometry가 임의 content의 min/max 정확성을 입증하지 않는다고 남긴다.
26. **PASS — C11.4 상태:** 제한 slice 병합으로 C11.4 전체를 완료 표시하지 않는다.
27. **PASS — C11 상위 상태:** C11 상위 항목을 완료 표시하지 않는다.
28. **PASS — 공식 집계:** `78/214`를 유지한다.
29. **PASS — 다음 후보 분리:** singleton `max-content` 후보를 기존 네-member `RootHtmlFitContentAutoRowsV1` profile과 별도 `RootHtmlIntrinsicTrackSizingV1` 경로로 구분한다.
30. **PASS — 다음 후보 상태와 링크:** 사전 비교·계획 리뷰 완료와 구현 전 상태를 함께 적고 링크 대상 파일을 확인했다.

## 남은 범위

이 검토는 문서 diff의 정확성·상태 경계만 확인한다. 제품 테스트, 전체 CSS Grid 적합성, WPT, Tailnet 제공본, simulator 화면 또는 새 singleton `max-content` 구현은 검증하지 않는다.
