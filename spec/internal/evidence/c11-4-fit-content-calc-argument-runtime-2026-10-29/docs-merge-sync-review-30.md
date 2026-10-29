# C11.4 calc fit-content 병합 문서 동기화 · PR diff review 30

대상 diff는 `spec/STATUS.md`의 C11.4 runtime slice 이력과 `plan/c11-4-fit-content-calc-argument.md`의 구현 상태에 PR #778 병합 정보를 기록한 것이다. 각 항목에서 잘못된 동기화가 공개 상태·기대값·검증 범위를 바꾸는지 확인했다.

1. **PR identity:** 기록한 구현 PR은 #778이고 제목·경로가 이 작업과 일치한다. 통과.
2. **Merge state:** GitHub state는 `MERGED`다. 열린 PR을 병합 완료로 잘못 표기하지 않았다. 통과.
3. **Merge time:** `mergedAt` 값 `2026-10-29T00:00:16Z`를 API와 대조했다. 통과.
4. **Merge commit:** rebase 결과 SHA `4ce3d4697c3505e39e945a452bdda3a07a411797`를 PR API와 `origin/main`에서 확인했다. 통과.
5. **Merge strategy:** repository 설정은 rebase 허용, squash 및 merge commit 비허용이다. 기록과 일치한다. 통과.
6. **Check rollup:** PR의 `statusCheckRollup`은 빈 배열이다. CI 성공으로 표현하지 않았다. 통과.
7. **Required checks:** main required status checks endpoint는 HTTP 404로 비활성 상태다. 설정되지 않은 검사를 통과한 것으로 쓰지 않았다. 통과.
8. **PR check command:** `gh pr checks 778 --required`는 검사 없음으로 종료됐다. 상태 설명과 일치한다. 통과.
9. **Parent checklist:** C11.4 상위 체크박스는 계속 unchecked다. 통과.
10. **Official count:** 공식 `72/213`을 유지했다. 통과.
11. **Slice boundary:** 기록은 제한 opt-in runtime slice이며 C11.4 전체 완료가 아니다. 통과.
12. **Plan link:** STATUS와 계획 문서 링크가 실제 저장소 경로를 가리킨다. 통과.
13. **Evidence link:** runtime README 경로와 파일 존재를 확인했다. 통과.
14. **Precomparison link:** pinned source와 독립 기대값의 사전 비교 링크를 유지했다. 통과.
15. **Pinned WPT:** `fit-content-track-sizing.html`의 definite calculated argument assertion 하나에 범위를 고정했다. 통과.
16. **Scene identity:** `fit-content-calc-percentage-row-wpt` ID가 fixture 및 capture 결과와 같다. 통과.
17. **CSS input:** 입력은 `fit-content(calc(10% + 5px))`다. 다른 calc 입력으로 바뀌지 않았다. 통과.
18. **Percentage basis:** root block-size basis는 definite 300px이다. 통과.
19. **Expected limit:** 10%와 5px 합산 기대값은 35px이다. 통과.
20. **Overflow child:** nested child 높이는 400px이며 row의 35px에 맞춰 잘리지 않는다. 통과.
21. **Percentage regression:** 기존 `fit-content(50%)` scene 150px 결과도 regression으로 남아 있다. 통과.
22. **Typed style:** computed calc AST를 검사한다. 통과.
23. **Projected handle:** projected `LayoutCalcId`를 검사한다. 통과.
24. **Registry owner:** calc registry record의 owner는 같은 grid root다. 통과.
25. **Registry property:** registry property는 `GridTemplateRows`다. 통과.
26. **Registry expression:** registry expression은 `Sum(10% + 5px)`이며 다른 term을 허용하지 않는다. 통과.
27. **Android identity:** Android 근거는 API 37 ARM64 AVD이며 실기기라고 표기하지 않았다. 통과.
28. **Android pixels:** PixelCopy는 4개 표본이 모두 exact다. 통과.
29. **iOS evidence boundary:** iPhone 17 Pro/iOS 26.2 Simulator Metal drawable 표본은 compositor 제출 전으로 한정했다. 통과.
30. **General support claim:** 일반 CSS admission, 전체 WPT, hardware GPU 및 전체 C11.4 완료를 주장하지 않는다. 통과.

## 결과

문서가 PR 병합 이력과 실제 제한 slice의 구현·실행 근거를 함께 가리킨다. C11.4 체크박스와 공식 완료 수는 미승격 상태로 유지한다.
