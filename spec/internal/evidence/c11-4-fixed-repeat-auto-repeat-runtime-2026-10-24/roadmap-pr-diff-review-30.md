# PR #591 병합 후 로드맵·상태 동기화 diff 검토

PR #591 병합 결과를 `spec/STATUS.md`, 집중 계획, 실행 근거와 공개 로드맵에 반영하는 문서 전용 변경을 별도 관점으로 대조했다.

1. **상위 체크:** C11.4 intrinsic track sizing은 계속 미완료다.
2. **공식 완료 수:** 부모 체크를 바꾸지 않아 72/213을 유지한다.
3. **집계 의미:** 72는 로드맵 공식 표시 대상의 체크 수이며 PR 개수나 공개 지원 API 수가 아니다.
4. **상위·하위 구분:** 부분 runtime slice가 새 공식 완료 ID로 계산되지 않는다.
5. **병합 식별자:** PR 번호가 #591이고 URL이 같은 저장소 pull request를 가리킨다.
6. **병합 SHA:** rebase merge 결과 `79a2f3e2ec412d8a039125cf8930b3c633c04b63`를 세 문서가 동일하게 쓴다.
7. **병합 시간:** `2026-10-24T08:48:30Z`가 GitHub `mergedAt` 값과 일치한다.
8. **병합 방식:** PR은 Rebase and merge로 기록되며 merge commit으로 오인되지 않는다.
9. **필수 check 상태:** `gh pr checks --required`가 check을 보고하지 않았다고 적는다.
10. **branch protection 주장:** required checks endpoint의 404 결과와 맞춰 미설정이라고 표현하며 CI 통과를 주장하지 않는다.
11. **plan 상태:** “병합 진행 중” 같은 과거 표현이 남지 않는다.
12. **evidence 상태:** 실행 evidence가 PR #591 병합 뒤에도 slice 전용 범위로 묘사된다.
13. **문서 계약:** 내부 계약 숫자 버전 0.1.0은 바뀌지 않는다.
14. **computed boundary:** fixed pattern을 authored token이 아닌 finite nonnegative computed `LengthPx`로 적는다.
15. **repeat cardinality:** 한 list의 fixed-repeat 하나와 auto-repeat 하나라는 경계가 계획·상태·로드맵에서 일치한다.
16. **order:** fixed-repeat prefix/suffix 허용 범위가 문서에서 바뀌지 않는다.
17. **track count:** pattern 1–2개 track 제한을 그대로 유지한다.
18. **percentage:** unresolved percentage/`calc()`가 지원된 것처럼 읽히지 않는다.
19. **unsupported shape:** intrinsic/flex/minmax, line names, areas 및 추가 repeat은 여전히 제외된다.
20. **scene 수:** 고정 9 scene은 구현 fixture 개수로만 기록한다.
21. **frame·trace 결과:** 각 simulator 9/9 검증은 runtime result JSON과 맞는다.
22. **pixel 표본:** PixelCopy와 Metal drawable 11/11 exact 표본 수가 README·STATUS에서 일치한다.
23. **Android 한계:** AVD software renderer 결과를 hardware GPU 또는 실기기로 확대하지 않는다.
24. **iOS 한계:** Metal drawable 표본이 compositor 제출 전이라는 제한을 보존한다.
25. **artifact 경계:** Debug APK와 Simulator bundle의 기본/opt-in marker 대조를 Release APK/IPA 검증으로 확대하지 않는다.
26. **screenshots:** 로드맵의 Android/iOS 링크가 PR #591에 포함된 마지막 source-diff run screenshot을 계속 가리킨다.
27. **STATUS 위치:** 새 C11.4 기록이 C12 상위 항목 아래로 밀리거나 다른 ID를 덮지 않는다.
28. **로드맵 출력:** 새 문구가 static roadmap HTML에 있고, 동적 check state는 `STATUS.md` fetch로 계속 계산된다.
29. **preview 경로:** 미리보기 동기화는 Tailnet preview 파일을 대상으로 하며 GitHub Pages 배포를 주장하지 않는다.
30. **문서 후속 범위:** 이 diff는 병합 사실과 확인 결과만 반영하며 런타임 코드·API admission·공식 지원 범위를 바꾸지 않는다.

**결과:** 30개의 서로 다른 문서 실패 경로와 계약 경계를 대조했다. C11.4 완료 표시는 올리지 않았고 공식 72/213을 유지했다. 전체 RSPress 문서 build는 기존 별도 dead link 제한 때문에 이 PR의 통과 증거로 사용하지 않으며 선택 route build와 Tailnet served hash를 따로 확인한다.
