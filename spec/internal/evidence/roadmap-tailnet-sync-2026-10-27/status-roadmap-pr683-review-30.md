# PR #683 병합 후 상태·로드맵 동기화 검토 30개 관점

검토 대상은 PR #683 병합 사실을 `spec/STATUS.md`, C11.4 계획, 공개 `roadmap.html`에 반영한 변경이다. 공식 완료 수와 완료 checkbox는 올리지 않는다.

1. **공식 분모:** 세 문서 모두 `72/213`을 보존한다. 대장 집계 자체는 이번 diff에서 바뀌지 않았다.
2. **상위 상태:** C11.4와 C11 상위 checkbox를 건드리지 않았고, 제한 slice를 전체 기능 완료로 승격하지 않는다.
3. **PR 식별:** 세 기록이 #683을 가리켜 #679 direct-text 기록과 혼동되지 않는다.
4. **병합 상태:** 기록한 병합 방식은 Rebase and merge이며 GitHub는 PR을 `MERGED`로 반환했다.
5. **병합 시각:** 문서 시각 `2026-10-27T05:54:04Z`는 GitHub의 `mergedAt`과 일치한다.
6. **병합 SHA:** `83df2c59dd63d83b53ff68f8a4717e0fae5b841e`가 GitHub merge commit 및 현재 `origin/main`과 일치한다.
7. **기준 WPT:** 이름은 auto-repeat intrinsic-size 변경 테스트 하나로 한정돼 다른 WPT를 대표하지 않는다.
8. **WPT 성공 주장:** WPT 원본은 oracle이며 Spinon 전체 WPT 통과로 서술하지 않는다.
9. **상태 수:** 8개 style 상태와 Android/iOS 각각의 8개 scene을 구별해 기록한다.
10. **mutation 경계:** 하나의 `HostDocument`와 동일 node handles를 쓴 근거는 Rust runtime sequence에만 귀속된다.
11. **앱 장면 경계:** Android/iOS scene은 각각 독립 입력이라 문서 mutation 증거로 세지 않는다.
12. **revision 경계:** document, render-tree, style revision 진행과 invalidation은 계획·실행 근거가 지원하는 범위로 적었다.
13. **geometry 경계:** item과 absolute probe geometry 재계산을 기록하되 모든 Grid geometry 지원으로 확대하지 않는다.
14. **auto-fill 범위:** 열 축 단일 auto-fill 뒤의 한 trailing neighbor로 한정한다.
15. **minmax 범위:** minimum이 min-content이고 maximum이 유한한 0 이상 px인 형태만 허용한다고 적었다.
16. **track sizing 의미:** repeat-count 계산의 고정 max와 track sizing의 min-content minimum을 혼동하지 않는다.
17. **probe 범위:** 임의 absolute Grid placement 지원을 주장하지 않고 WPT 고정 profile로 제한한다.
18. **item 폭 allowlist:** 이 요약이 모든 item 폭을 admission한다고 오해되지 않으며 구현 README에 정확한 폭 allowlist와 90px rejection control이 남아 있다.
19. **platform 이름:** Android API 37 ARM64 AVD를 Android 실기기로 부르지 않는다.
20. **iOS 이름:** iPhone 17 Pro / iOS 26.2 Simulator로 기록해 실기기와 구분한다.
21. **scene 판정:** `drawPresented=true`는 앱 결과가 제출 경계까지 갔음을 뜻하며 픽셀 exact 결과로 표현하지 않는다.
22. **GPU 범위:** Android hardware GPU를 검증했다고 쓰지 않는다.
23. **compositor 범위:** iOS compositor 이후 화면 픽셀 검증을 완료했다고 쓰지 않는다.
24. **일반 admission:** 일반 CSS admission은 남은 범위로 명시한다.
25. **전체 conformance:** 전체 WPT와 C11.4가 미완료임을 세 문서에서 유지한다.
26. **계획 상태:** 구현 진행 중이라는 옛 top-line을 병합된 slice 정보로 갱신하고 전체 단계는 열린 채 둔다.
27. **계획 이력:** 계획 상단과 하단의 PR 병합 상태가 서로 모순되지 않는다.
28. **상대 경로:** STATUS의 계획·실행·검토 링크는 저장소 기준 상대 경로로 연결된다.
29. **공개 경로:** roadmap 카드의 링크는 Tailnet `/spinon/` route 형식이며 route root를 중복하지 않는다.
30. **이미지 route:** Android/iOS 링크가 PR #683의 최종 캡처 run 디렉터리를 가리킨다. PR 본문에서도 두 원격 이미지가 실제 로드되는 것을 Agent Browser로 확인했다.
