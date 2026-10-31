# PR #942 상태 및 공개 로드맵 변경 검토 30개

검토 대상은 `spec/STATUS.md`의 C11.4 word-break 실행 기록과 `spec/public/roadmap.html`의 최상단 진행 카드다. 기준 구현 PR은 이미 merge된 #942이며, 이 검토는 상태/로드맵 동기화 변경만 다룬다. 각 항목은 별도의 오류 경로 또는 공개 범위 경계를 확인했다.

1. **Merge 상태** — GitHub PR #942의 `mergedAt`, `state`, `mergeCommit`을 현재 조회 결과와 대조했다. 표시한 `MERGED` 상태와 시간이 일치한다.
2. **Merge SHA** — 상태 대장과 로드맵의 SHA가 PR merge commit `845f59ed378e7b3bbbabee3ed2eb0926b81f5359`와 일치한다.
3. **PR 링크** — 두 문서의 구현 PR 링크가 같은 저장소의 #942를 가리킨다.
4. **날짜 해석** — 표기된 UTC merge 시각을 날짜 문구와 대조했다. 로컬 날짜 차이를 숨기지 않고 원본 UTC 시각을 남겼다.
5. **12-scene 총수** — 구현 근거에 고정 scene 수 12와 Android/iOS 결과가 각각 기록되어 있다.
6. **Ahem 하위 집합** — 카드와 상태 기록의 Ahem 8개 표본 설명이 Chrome 사전 비교/실행 근거와 일치한다.
7. **한국어 하위 집합** — Noto Sans KR 4개 표본을 별도로 세어 Ahem 장면과 혼동하지 않는다.
8. **속성 범위** — `normal`, `break-all`, `keep-all`의 표기가 제한 fixture 값과 일치하고, CSS 전체 키워드 지원을 암시하지 않는다.
9. **축 범위** — min-/max-content track 언급이 다른 Grid 축이나 일반적인 track sizing 완료로 확장되지 않는다.
10. **Android 결과** — API 37 ARM64 AVD 12/12를 실기기 결과로 표현하지 않는다.
11. **iOS 결과** — iPhone 17 Pro/iOS 26.2 Simulator 12/12를 실제 iPhone 증거로 표현하지 않는다.
12. **설치본 동일성** — APK/executable hash 일치는 source/build identity 확인으로만 기술한다.
13. **Glyph paint 한계** — 검은 text surface를 명시해 시각적 glyph paint 검증으로 오해할 경로를 닫았다.
14. **Pixel 표면 한계** — PixelCopy, Metal drawable 또는 compositor 이후 픽셀 일치로 주장하지 않는다.
15. **CI 상태** — check rollup 부재와 required status check 부재를 모두 적고 CI 통과로 바꾸어 쓰지 않는다.
16. **공식 집계** — 새 slice 때문에 공식 완료 수가 바뀌었다고 쓰지 않고 `72/213`을 유지한다.
17. **부모 C11 상태** — 전체 CSS Grid 완료를 뜻하지 않는다고 명시한다.
18. **자식 C11.4 상태** — intrinsic track sizing 및 전체 C11.4 완료로 승격하지 않는다.
19. **CSS admission 경계** — fixture 전용이라는 표현을 유지하고 웹 입력 전체에 값이 적용된다고 주장하지 않는다.
20. **언어 지원 경계** — 임의 언어·폰트 전반을 지원한다고 일반화하지 않는다.
21. **WPT 경계** — WPT 적합성 결과로 표현하지 않는다.
22. **Hardware GPU 경계** — AVD/Simulator 결과를 hardware GPU 또는 실기기 결과로 승격하지 않는다.
23. **Locale 전달 의미** — `lang=ko`를 locale metadata로 전달한다는 사실과 pinned Parley가 관측된 한국어 분절을 만들지 않는다는 한계를 함께 둔다.
24. **Parley 원인 과장 방지** — Parley 동작의 관찰 범위만 설명하고 미검증 원인이나 보편적인 locale 동작으로 단정하지 않는다.
25. **사전 비교 링크** — roadmap의 Chrome 비교 링크가 PR #942 word-break 사전 비교 README 경로와 대응한다.
26. **실행 근거 링크** — roadmap의 Android/iOS 실행 링크가 PR #942 runtime README 경로와 대응한다.
27. **내부 계약 링크** — `0059` 링크가 같은 공개 base path의 Grid 내부 계약 페이지를 가리킨다.
28. **계획 링크** — word-break 계획 문서가 저장소에 존재하며 Rspress plan 경로와 이름이 대응한다.
29. **카드 우선순위** — #942 카드가 더 오래된 #938 카드보다 위에 있어 최근 실행 내역이 먼저 보인다.
30. **로드맵/상태 일관성** — 두 문서의 핵심 사실(merge, 제한 fixture, 플랫폼 수치, 72/213, 미완료)이 서로 충돌하지 않는다.

검토 결과, 문구와 공개 범위에 수정이 필요한 불일치는 발견하지 못했다. 이 변경은 구현 완료 체크를 추가하지 않는다.
