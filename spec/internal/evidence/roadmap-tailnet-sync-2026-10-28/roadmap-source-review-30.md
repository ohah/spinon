# 공개 로드맵 #734 항목 변경 검토 30개

검토 대상은 `spec/public/roadmap.html`에 추가한 단일 #734 진행 기록이다. 기준 근거는 병합된 구현 PR #734, `plan/c11-4-mixed-flex-hidden-item.md`, `spec/internal/evidence/c11-4-mixed-intrinsic-flex-fixed-runtime-2026-10-28/README.md`다. 로컬 링크 8개가 현재 추적 파일에 대응하는지 확인했고 Android/iOS PNG 원본을 Agent Browser에서 열었다.

1. **시간 순서:** #734 항목은 기존 마지막 C11.4 기록 #729 다음에 놓이고 관문 섹션 전에 끝난다. 통과.
2. **변경 범위:** 기존 로드맵 기록을 편집하지 않고 새 진행 문단 하나만 추가했다. 통과.
3. **스타일 계약:** 기존 `kanban-note hero-progress` 구조를 사용해 다른 기록과 같은 스타일을 적용했다. 통과.
4. **PR 링크 대상:** GitHub URL이 `ohah/spinon/pull/734`를 가리킨다. 통과.
5. **병합 방식 표기:** 실제 병합 기록과 같은 `Rebase and merge`를 표시한다. 통과.
6. **병합 시각:** `2026-10-28T04:03:05Z`는 관찰된 GitHub PR 병합 시각이다. 통과.
7. **병합 SHA:** 전체 `50e6286cf7b1c86a8c0398221751a42db105cb05`가 PR의 merge SHA와 일치한다. 통과.
8. **입력 범위:** hidden case를 기존 mixed-track fixture의 제한 입력으로 설명한다. 일반 CSS admission으로 확대하지 않는다. 통과.
9. **노드 모양:** 15자 direct `<span>`과 `display:none`이라는 추가 조건을 나타낸다. 통과.
10. **축 범위:** Columns와 Rows만 다루며 두 축 전체 CSS 지원을 주장하지 않는다. 통과.
11. **Android positive 수:** Android API 37 AVD의 10/10 scene 결과와 일치한다. 통과.
12. **iOS positive 수:** iPhone 17 Pro/iOS 26.2 Simulator의 10/10 scene 결과와 일치한다. 통과.
13. **Android negative 경로:** 미등록 scene, visible control, 충돌 profile의 세 거부 대조군을 요약한다. 통과.
14. **iOS negative 경로:** 같은 세 경로가 양쪽 앱 실행에서 확인된 범위로 서술한다. 통과.
15. **Intrinsic 계산:** hidden text가 intrinsic sizing 입력에서 제외됨을 설명한다. 통과.
16. **Track 기여:** hidden text가 track contribution에서 제외됨을 설명한다. 통과.
17. **Track 결과:** `[100, 40, 140] CSS px` 값이 README 결과와 일치한다. 통과.
18. **남은 공간:** remaining free space `0px`를 기록한다. 통과.
19. **내부 frame 해석:** hidden node의 0×0 frame을 bookkeeping record라고 명시한다. 통과.
20. **Box/paint 과장 방지:** 그 frame이 보이는 box나 paint를 의미하지 않는다고 구별한다. 통과.
21. **Android PNG 경로:** 링크된 `android-column-hidden.png`가 Git에 추적되며 77,930 bytes다. 통과.
22. **iOS PNG 경로:** 링크된 `ios-row-hidden.png`가 Git에 추적되며 146,555 bytes다. 통과.
23. **Android 기기 설명:** API, ARM64, 16 KB AVD 조건을 기록하고 실기기라 부르지 않는다. 통과.
24. **iOS 기기 설명:** iPhone 17 Pro/iOS 26.2 Simulator라고 명시한다. 통과.
25. **집중 계획 링크:** `plan/c11-4-mixed-flex-hidden-item.md`에 대응한다. 통과.
26. **실행 근거 링크:** runtime README route에 대응한다. 통과.
27. **검토 자료 링크:** hidden implementation/runtime 20개, PR diff 30개, 문서 sync 30개 route가 각각 추적된 Markdown에 대응한다. 통과.
28. **계약 링크:** 내부 계약 0059의 `.md` 원본이 추적되어 있다. 통과.
29. **한계와 CI:** check rollup 공란을 CI pass로 바꾸지 않고, 실기기·hardware GPU·overflow pixel·일반 CSS·전체 WPT를 미검증으로 남긴다. 통과.
30. **공식 상태:** C11.4 미완료 및 `72/213` 유지가 명시적이고 `spec/STATUS.md` 변경은 없다. 통과.

## 화면 원본

- Android: Agent Browser에서 commit `50e6286cf7b1c86a8c0398221751a42db105cb05`의 PNG를 열어 `1080×2400`을 확인했다.
- iOS: 같은 commit의 PNG를 열어 `1206×2622`를 확인했다.
- Agent Browser session `spinon-roadmap-pr736`을 종료했고 세션 목록은 비었다. 프로세스 스캔에서 해당 브라우저 세션은 남지 않았다.
