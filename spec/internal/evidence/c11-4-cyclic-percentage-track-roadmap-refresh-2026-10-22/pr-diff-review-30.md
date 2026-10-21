# C11.4 공개 roadmap 갱신 PR diff 검토 30개

검토 대상은 `spec/public/roadmap.html`에 추가한 2026-10-22 C11.4 cyclic percentage runtime 진행 기록과 링크다. 코드 구현 review와 별개로 문서 diff에서 잘못된 상태·수치·링크·플랫폼 주장을 확인했다.

1. **최신 기록 위치:** 새 2026-10-22 항목이 10-21 C11.4 기록보다 앞에 있어 시간순 갱신을 반영하는가? hero 제목 바로 다음에 추가했다. PASS.
2. **문서 영역:** 새 기록이 구현 로드맵 hero에 들어가 다른 C11.4 계획·완료 기록을 덮지 않는가? 전용 `kanban-note` 문단 하나로 분리했다. PASS.
3. **공식 완료 수:** fixed fixture 성과가 체크리스트 수를 늘리거나 `72/213`을 바꾸는가? 공식 완료 수가 그대로라고 명시한다. PASS.
4. **C11.4 완료 상태:** 세 scene 결과를 C11.4 전체 완료로 표현하는가? C11.4 전체가 미완료임을 명시한다. PASS.
5. **C11.3 경계:** C11.3.3 named-area 진행과 C11.4 결과가 섞이는가? 새 내용은 C11.4 Grid track sizing에만 한정했다. PASS.
6. **fixture 개수:** 지원 범위를 고정 세 scene으로 제한하는가? cyclic column·row와 definite column control만 명시한다. PASS.
7. **column oracle:** cyclic column 결과를 잘못 `[20]` 등으로 기록하는가? expected Columns `[12]`로 적었다. PASS.
8. **row oracle:** cyclic row 결과를 잘못 16px로 기록하는가? expected Rows `[8]`로 적었다. PASS.
9. **definite control:** definite 40px basis의 50% 결과를 누락하거나 혼동하는가? definite Columns `[20]`로 분리해 적었다. PASS.
10. **반대 축 검증:** 한 축만 측정했다고 오해하게 만드는가? 두 축 track과 전체 node frame을 검사했다고 썼다. PASS.
11. **Android target:** Android 근거가 실기기로 오인되는가? API 37 ARM64 AVD라고 정확히 적었다. PASS.
12. **iOS target:** iOS 근거를 실기기 결과로 보이게 하는가? iPhone 17 Pro / iOS 26.2 Simulator로 명시한다. PASS.
13. **scene pass 수:** 3/3을 양 플랫폼 합산이나 일부 장면 결과로 오해하는가? 각 3/3 장면을 양 simulator 문맥 안에서 표현한다. PASS.
14. **pixel sample 수:** 두 경계 검사 합계를 빠뜨리는가? PixelCopy와 Metal drawable을 각각 6/6 exact로 명시한다. PASS.
15. **compositor 경계:** iOS Metal drawable 값을 최종 화면 pixels로 과장하는가? compositor 제출 전이라고 명시한다. PASS.
16. **초기 실패 이력:** 첫 AVD projection 거부를 숨기는가? 미지원 `justify-items:start`를 제거하고 전체 matrix를 재실행했다고 기록했다. PASS.
17. **artifact 경계:** fixture-test 전용 payload가 제품 artifact에 들어간다고 오해하는가? default FFI release 제외와 positive control 포함을 분리했다. PASS.
18. **merge SHA:** PR #469 merge commit을 다른 hash로 연결하는가? `6173c435afe9134e25a8e71b4c7f61152f5c8e7b`를 PR merge result와 대조했다. PASS.
19. **merge 방식:** PR을 일반 merge로 오기하는가? Rebase and merge라고 적었다. PASS.
20. **원격 checks:** 필수 check가 있었거나 성공한 것처럼 말하는가? 병합 당시 required status check 미설정과 빈 status rollup을 함께 공개한다. PASS.
21. **arbitrary CSS:** fixed fixtures를 일반 CSS admission으로 확대하는가? arbitrary CSS는 미완료로 남긴다. PASS.
22. **calc track:** 미검증 `calc()` percentage track을 지원하는 것처럼 보이는가? 미완료 목록에 넣었다. PASS.
23. **minmax/fit-content:** percentage limit 변형을 이 결과로 보증하는가? 둘 다 미완료라고 적었다. PASS.
24. **여러 percentage track:** 여러 cyclic track을 확인했다고 확대하는가? 복수 track을 미완료로 남겼다. PASS.
25. **양축 동시 cyclic:** 두 axis의 동시 cyclic resolve를 검증한 것으로 보이는가? 미완료 목록에 명시했다. PASS.
26. **nested Grid:** 세 scene을 nested Grid 지원 증거로 확대하는가? nested Grid는 미완료다. PASS.
27. **WPT 적합성:** 고정 oracle을 WPT conformant 결과라고 표현하는가? WPT 미완료라고 명시한다. PASS.
28. **실기기:** AVD/Simulator를 physical device evidence로 혼동하는가? 실기기 미완료라고 적었다. PASS.
29. **plan/evidence/review 링크:** 계획, 구현 review, PR diff review와 새 근거 README가 Tailnet base `/spinon/` 아래 route로 가는가? 현재 PR diff의 절대 경로가 pass40에서 생성·확인된 route와 일치한다. PASS.
30. **화면 링크:** Android/iOS 화면이 raw local path나 잘못된 PR 경로를 가리키는가? 현재 PR diff는 pass40에서 해시 확인된 PNG의 `/spinon/internal/evidence/...` route를 사용한다. PASS.

판정: 위 30개 문서 실패 지점을 확인했다. 이 문단은 진행 기록이며 official roadmap checklist나 `72/213` 집계를 변경하지 않는다.
