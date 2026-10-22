# C11.4 로드맵 정정 PR diff 적대 검토 30개

검토 대상은 `spec/public/roadmap.html`의 최신 C11.4 카드 한 단락이다. 목적은 병합된 PR #534의 고정 simulator slice를 반영하고, 전체 C11.4 완료로 확대하지 않는 것이다.

1. **이전 기능 기록 보존:** PR #530의 authored line name slice와 여덟 scene이 남아 있다 — 통과.
2. **오래된 제외 문구 제거:** 전체 missing-name fallback이 아직 미검증이라는 기존 문장을 새 요약으로 대체했다 — 통과.
3. **fallback 범위 제한:** PR #534가 단일 auto-fill 축의 slice라고 적었다 — 통과.
4. **track grammar 제한:** implicit `px` track subset을 명시했다 — 통과.
5. **결손 이름 입력:** authored 이름이 없을 때 동작을 포함한다 — 통과.
6. **부족한 occurrence 입력:** authored occurrence가 모자랄 때도 포함한다 — 통과.
7. **두 placement 유형:** named line과 named span을 함께 표기했다 — 통과.
8. **사전 oracle scene 수:** Chrome 156 oracle 10 scene을 PR #534 사전 비교 근거와 대조했다 — 통과.
9. **Android 실행 대상:** API 37 ARM64 AVD라고 적었고 runtime 기록과 일치한다 — 통과.
10. **iOS 실행 대상:** iPhone 17 Pro / iOS 26.2 Simulator라고 적었고 runtime 기록과 일치한다 — 통과.
11. **scene 통과 수:** 두 simulator 각각 10/10으로 runtime 결과와 일치한다 — 통과.
12. **node frame 수:** 각 실행의 20 node frame과 일치한다 — 통과.
13. **layout 단계 관찰:** track trace와 draw/present 결과를 함께 표기했다 — 통과.
14. **Android pixel 경계:** PixelCopy 30/30 exact를 Android에 연결했다 — 통과.
15. **iOS pixel 경계:** compositor 제출 전 Metal drawable 30/30 exact를 iOS에 연결했다 — 통과.
16. **compositor 과장 방지:** iOS compositor 제출 이후 출력 검증은 제외한다고 썼다 — 통과.
17. **border stroke 과장 방지:** fixture가 border stroke를 검사하지 않는다고 명시했다 — 통과.
18. **공개 CSS admission 경계:** 제한 fixture를 일반 공개 CSS 지원으로 확대하지 않고 C11.4를 미완료로 유지한다 — 통과.
19. **공식 완료 수:** `77/213`을 유지해 해당 PR이 상위 항목 완료 수를 올리지 않는다 — 통과.
20. **auto-fit 조합:** auto-fit fallback은 후속 범위로 남겼다 — 통과.
21. **복수 repeat:** 복수 auto-repeat을 지원 범위 밖으로 남겼다 — 통과.
22. **Grid 조합:** areas와 subgrid를 미검증으로 남겼다 — 통과.
23. **축·방향 변형:** writing mode를 미검증으로 남겼다 — 통과.
24. **임의 입력 범위:** arbitrary CSS와 WPT를 미검증으로 남겼다 — 통과.
25. **플랫폼 범위:** Android 실기기와 hardware GPU를 검증했다고 주장하지 않는다 — 통과.
26. **PR #530 연결:** 이전 PR URL과 원래 집중 계획이 계속 연결된다 — 통과.
27. **PR #534 연결:** 올바른 PR URL과 merge SHA `098a2501cef5885031bbd829df58228bb6cdcd6d`를 썼다 — 통과.
28. **집중 계획 링크:** PR #534 전용 계획 route가 빌드 선택 사이트에 존재한다 — 통과.
29. **실행 근거 링크:** runtime README와 Chrome 사전 비교 route가 빌드 선택 사이트에 존재한다 — 통과.
30. **화면 링크:** Android와 iOS PNG route가 각각 빌드 선택 사이트에 있고 runtime README의 기기별 결과와 맞는다 — 통과.

추가 구조 점검에서 변경은 해당 카드 한 단락에 한정됐고, `git diff --check`가 통과했다. merge 당시 PR check rollup은 비어 있었으며 GitHub main required status checks는 설정되지 않아 있었다. 이 문서 검토는 PR #534의 runtime 재검증이나 C11.4 전체 적합성 판정으로 확대하지 않는다.
