# C11.4 cyclic percentage `fit-content()` 병합 후 문서 diff 검토 30개

PR #474가 main에 Rebase and merge된 뒤, 병합 SHA·상태·공식 완료 수를 반영한 plan, STATUS, roadmap, runtime README 변경을 별도로 검토했다. PR #474의 implementation 20개 및 code diff 30개 검토와 구분되는 문서 변경 검토다.

1. **merge SHA 불일치:** 문서가 head SHA를 merge SHA로 잘못 기록하는가? GitHub live PR 값 `df0c607f83732081a47ceb8435f3a174289acee7`과 일치한다. PASS.
2. **merge 시간대 혼동:** UTC 시각을 local KST처럼 표기하는가? `2026-10-21 21:23:35 UTC`를 그대로 사용한다. PASS.
3. **PR 상태:** 병합된 PR을 open/review 상태로 남기는가? STATUS, plan, roadmap, runtime README가 병합으로 기록한다. PASS.
4. **merge 방식:** repository 정책과 다른 merge 방식으로 기록하는가? PR #474는 Rebase and merge였고 문서 표기도 같다. PASS.
5. **required status 설정:** 설정되지 않은 필수 check가 있었던 것처럼 쓰는가? live branch protection의 `required_status_checks`는 null이었다. PASS.
6. **check rollup 주장:** check가 보고되지 않았는데 모두 통과했다고 표현하는가? “PR check rollup은 비어 있었다”고 구분해 쓴다. PASS.
7. **status checkbox:** 세 simulator fixture를 C11.4 전체 완료로 승격하는가? 상위 C11.4 checkbox는 계속 `[ ]`다. PASS.
8. **완료 집계:** 완료 수를 바꾸거나 계산 근거 없이 올리는가? plan/status/roadmap은 모두 `72/213`을 유지한다. PASS.
9. **지원 범위:** fit-content 일반 CSS admission으로 오해될 문구가 있는가? fixed opt-in scenes 및 일반 지원 미완료를 명시한다. PASS.
10. **plan 진행 단계:** 이미 끝난 구현/runtime 작업을 future checklist로 남기는가? 실행 결과와 merge 상태를 기록한다. PASS.
11. **규범 기준:** 독립 기대값의 근거가 빠지거나 Chrome observation과 합쳐지는가? 2025 CRD와 별도 Chrome 사전 비교 link를 유지한다. PASS.
12. **시뮬레이터 한계:** iOS Metal sample을 compositor 이후 값으로 쓰는가? compositor 제출 전임을 roadmap/runtime README에 유지한다. PASS.
13. **Android 대상:** AVD evidence를 Android 실기기로 잘못 부르는가? API 37 ARM64 AVD라고 명시한다. PASS.
14. **iOS 대상:** Simulator 결과를 iOS 실기기 실행으로 잘못 부르는가? iPhone 17 Pro/iOS 26.2 Simulator라고 명시한다. PASS.
15. **surface 표본 의미:** 6/6 exact를 glyph·border 또는 전체 pixel equality로 과장하는가? item/background 경계 표본으로 한정한다. PASS.
16. **artifact 범위:** `.rlib` hash를 APK/IPA release exclusion으로 확장하는가? artifact README가 release app 검증이 아니라고 제한한다. PASS.
17. **내부 계약 버전:** fixture slice만으로 contract 0059 version을 올리는가? 기존 계약 0.1.0을 유지하고 새 공개 API를 주장하지 않는다. PASS.
18. **구현 review 링크:** implementation 20개를 plan review로 혼동하는가? 계획 20개와 구현 20개 파일을 다른 링크로 유지한다. PASS.
19. **PR diff review 링크:** PR 30개 review가 빠지거나 구현 review와 합쳐지는가? 독립 `pr-diff-review-30.md` 링크가 있다. PASS.
20. **merge record 위치:** 공식 STATUS가 C11.4 상위 항목 밖에 새 완료 ID를 만드는가? 기존 C11.4 bullet의 nested note에 기록한다. PASS.
21. **로드맵 순서:** 새 C11.4 기록이 지난 항목 아래에 밀리거나 기존 기록을 덮는가? 새 dated paragraph를 기존 PR #469 note 바로 앞에 둔다. PASS.
22. **HTML paragraph structure:** PR note 삽입으로 `<p>` nesting이나 close tag가 깨지는가? 새 paragraph 하나를 기존 paragraph 경계에서 교체했다. PASS.
23. **HTML route:** Markdown link가 generated `.html` 경로와 다르게 작성됐는가? plan, evidence, review, screenshots가 기존 `/spinon/...` route 형식을 따른다. PASS.
24. **파일 경로:** plan/runtime README 상대 경로가 다른 evidence ID를 가리키는가? 각 local path가 main의 해당 파일로 resolve된다. PASS.
25. **screenshot link:** roadmap 화면 링크가 이전 simulator capture를 가리키는가? post-commit Android/iOS run directory의 committed image를 가리킨다. PASS.
26. **PR image evidence:** PR 본문 raw links가 load되지 않는 문제가 남는가? Agent Browser에서 두 이미지가 1080×2400, 1206×2622로 렌더링됐다. PASS.
27. **source revision 시각:** code capture commit과 PR merge commit을 같은 값으로 표기하는가? runtime metadata는 implementation commit `9af5ee8f`; merge record는 `df0c607f`로 구분한다. PASS.
28. **검증 범위 반복:** merge note가 WPT·실기기·glyph 검증을 새로 수행했다고 쓰는가? 남은 범위에 그대로 미검증으로 둔다. PASS.
29. **문서 배포:** roadmap 갱신을 GitHub Pages deploy로 표현하거나 workflow를 실행하는가? PR은 source docs만 수정하며 Pages workflow를 실행하지 않는다. PASS.
30. **문서 변경 범위:** unrelated roadmap/STATUS history를 재정렬하거나 수정하는가? 해당 계획 문서, 단일 C11.4 note와 단일 runtime README만 바꾼다. PASS.

판정: post-merge 문서 기록은 GitHub merge 결과와 일치하며 C11.4 전체 미완료 상태를 유지한다. Tailnet preview는 main 최신 문서 build·hash 검증 후 별도로 갱신한다.
