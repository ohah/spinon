# PR #1187 병합 후 로드맵 변경 적대 검토 30개

대상은 공개 `spec/public/roadmap.html`의 새 candidate 기록, official-status 보존, Tailnet 단일 파일 교체 근거, Agent Browser screenshot 및 측정이다. 구현 PR diff 검토 30개는 재사용하지 않았다.

1. **Chronology:** 새 2026-11-06 항목은 최신 PR #1184 뒤, 2026-11-04 항목 앞에 놓였다.
2. **Timezone:** PR merge time은 GitHub의 `2026-11-05T17:12:55Z` 그대로 기록해 KST 날짜와 UTC timestamp를 섞지 않았다.
3. **PR identity:** 문서 링크는 실제 merged PR #1187을 가리킨다.
4. **Merge identity:** merge SHA `0c32617f76e18694bd692ff84a8035325d2ad020`를 GitHub merge 결과와 대조했다.
5. **No CI claim:** rollup이 비어 있으므로 check pass 또는 CI green이라고 쓰지 않았다.
6. **Required checks:** `main`의 required status checks가 null이라는 live repository 결과와 일치한다.
7. **Official count:** 완료 집계 `78/214`를 정확히 유지하고 PR을 공식 완료 1건으로 더하지 않았다.
8. **Checkbox status:** C11.4/C11 미완료 설명을 유지하고 `spec/STATUS.md` checkbox를 변경하지 않았다.
9. **Candidate wording:** 제목과 본문은 제한 구현으로 표시해 전체 C11.4와 구분한다.
10. **Selector name:** 지원 CSS 값은 singleton `grid-auto-rows: min-content`로 정확히 표현했다.
11. **Owner bound:** 한 Grid owner 전제는 PR contract와 맞는다.
12. **Track bound:** 한 explicit column·row와 implicit min-content row 경계를 요약 문구에 보존했다.
13. **Item bound:** direct empty HTML `div` 및 fixed item size 조건을 빠뜨리지 않았다.
14. **Placement bound:** all-auto item placement와 row flow 전제를 적었다.
15. **Gap bound:** zero gap 조건을 적어 일반 gap 지원으로 오해되지 않는다.
16. **Overflow bound:** visible overflow 지원만 기록했다.
17. **Failure semantics:** out-of-scope 조합은 원자 거부로 설명하고 layout success로 부풀리지 않았다.
18. **Focused counts:** cascade/projection/capacity/layout/RuntimeSession의 1/1·1/1·2/2·11/11·2/2 값을 evidence와 대조했다.
19. **Broad candidate cohort:** 133/9 수는 candidate 로그에 맞는다.
20. **Baseline cohort:** 131/9는 세 style-to-layout 변경 파일을 HEAD로 되돌린 baseline 결과와 맞는다.
21. **Failure identity:** candidate와 baseline에 같은 9개 C11.5 실패가 남았다고 기록해 추가 통과로 합산하지 않았다.
22. **Android target:** Android API 37 ARM64 AVD 정보를 보존하고 실기기라고 부르지 않는다.
23. **iOS target:** iPhone 17 Pro / iOS 26.2 Simulator로 기록한다.
24. **Runtime result:** 두 simulator의 `layout=ready` 7 boxes와 draw/present 사실은 runtime log와 맞는다.
25. **Screenshot class:** OS screenshot을 PixelCopy나 Metal drawable readback으로 승격하지 않았다.
26. **Platform limits:** Android hardware GPU, iOS compositor 이후 pixel 및 WPT를 검증했다고 주장하지 않는다.
27. **Internal links:** plan, internal spec, evidence, implementation review 링크는 merge된 main URL을 사용한다.
28. **Image immutability:** simulator screenshot 링크는 moving branch 대신 merge SHA raw URL을 사용한다.
29. **Tailnet scope:** serving 변경은 `roadmap.html` 단일 파일이며 backup hash와 before/after bytes가 기록된다.
30. **Preview geometry and cache:** desktop/mobile viewport에서 카드 bounds와 overflow를 검사하고, stale browser cache 화면은 최신 serving 결과와 분리해 기록했다.

문서 변경은 코드 지원 범위나 공식 완료 상태를 바꾸지 않는다. Tailnet server restart, GitHub Pages 배포와 파일 삭제는 하지 않았다.
