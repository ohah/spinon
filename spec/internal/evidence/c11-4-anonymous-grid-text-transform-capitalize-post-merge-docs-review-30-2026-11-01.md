# C11.4 `capitalize` 병합 상태 문서 검토: 30개 관점

대상은 PR #986 병합 뒤 `spec/STATUS.md`와 `spec/public/roadmap.html`의 상태 메모 두 곳이다. 구현 코드는 변경하지 않는다.

1. **PR identity:** 양쪽 문서가 방금 병합한 구현 PR #986을 가리키는지 확인한다.
2. **Merge SHA:** 표기한 `d5951d09390c746d29389e821f24088fbf51245b`가 GitHub의 병합 commit과 일치한다.
3. **Merge timestamp:** 두 문서에 UTC 기준 `2026-11-01T12:09:30Z`를 동일하게 기록했다.
4. **Source identity:** Android/iOS 실행 증거가 결합된 코드 source commit `9b124ae`를 merge commit과 혼동하지 않는다.
5. **PR review commit:** 30 관점 검토 파일의 branch commit `28f0503`를 제품 코드 source commit으로 오인하지 않는다.
6. **Official counter:** 이번 메모는 완료 checkbox를 변경하지 않으며 공식 `72/213`을 유지한다.
7. **C11 parent:** C11 상위 항목은 미완료 그대로다.
8. **C11.4 parent:** C11.4 intrinsic track sizing도 미완료 그대로다.
9. **Support boundary:** 병합 사실이 제한 fixture slice를 일반 CSS 지원으로 확대하지 않는다.
10. **Input boundary:** direct `lang="en"`, ASCII letters, U+0020 profile을 기존처럼 명시한다.
11. **Unicode boundary:** locale/Unicode casing과 UAX #29가 제외된 상태를 계속 드러낸다.
12. **Source text:** HostDocument Text가 유지된다는 설명은 구현/runtime 증거 범위와 맞는다.
13. **Platform evidence:** Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 각각 6/6은 unchanged다.
14. **Artifact identity:** 앱 설치 hash와 sourceHead가 고정됐다는 설명을 유지한다.
15. **iOS failure cohort:** pass01·pass03의 `status=-12` 실패를 성공 실행에 합치지 않는다.
16. **Retry language:** pass04와 보충 15/15가 앞선 실패를 고쳤다고 문구를 바꾸지 않는다.
17. **Screenshot meaning:** `layout=ready` 캡처를 glyph pixel 근거로 과장하지 않는다.
18. **WPT boundary:** WPT 및 전체 C11.4/C11 적합성 미완료를 계속 명시한다.
19. **CI boundary:** GitHub check이 없었다는 기존 진술과 충돌하는 CI 통과 주장을 추가하지 않는다.
20. **Contract version:** 내부 계약 0059의 `0.1.0` 버전 정책은 변경하지 않는다.
21. **Status hierarchy:** merge 기록은 C11.4 세부 상태 아래에 남고 상위 체크를 올리지 않는다.
22. **Roadmap hierarchy:** 공개 roadmap 카드가 공식 상태와 같은 범위를 사용한다.
23. **PR permalink:** roadmap PR 링크는 안정적인 GitHub PR URL을 사용한다.
24. **Local review route:** roadmap의 PR diff 검토 링크는 저장소 evidence 파일의 RSPress `.html` route와 일치한다.
25. **Markdown review link:** STATUS의 PR diff 검토 링크는 실제 `.md` 원본 위치를 가리킨다.
26. **Encoding:** `<code>`와 한글 문구를 HTML 문맥에 맞게 유지해 markup을 깨뜨리지 않는다.
27. **Roadmap order:** 새 merge 기록을 최신 C11.4 진행 카드에 붙이고 이전 PR #982 이하의 기록은 바꾸지 않는다.
28. **Counter interpretation:** 전체 로드맵 체크 수와 공식 완료 수를 혼동하지 않는다.
29. **Change scope:** diff는 STATUS의 한 C11.4 문장과 roadmap의 대응 카드만 수정한다.
30. **Publication claim:** 문서 diff만으로 Tailnet 게시 완료를 주장하지 않는다. Tailnet 반영은 별도 route/hash 및 Agent Browser 확인 뒤 기록한다.

**판정:** 문서에 PR #986의 병합 식별 정보와 PR diff 검토 링크를 보강해도 C11/C11.4의 공식 완료 판단이나 `72/213`은 바뀌지 않는다. Tailnet 동기화는 이 PR의 병합 뒤 별도 작업으로 검증한다.
