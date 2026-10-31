# C11.4 computed `line-height` 공개 로드맵 변경 검토 30개

검토 대상은 `spec/public/roadmap.html`에 추가한 C11.4 anonymous Grid text computed `line-height` 제한 fixture 설명이다. 구현은 [PR #905](https://github.com/ohah/spinon/pull/905), 공식 상태는 [`spec/STATUS.md`](../../../../STATUS.md), 실행 근거는 [runtime README](./README.md)를 기준으로 대조했다. 각 검토 관점은 별도로 확인했다.

1. **공식 완료 수** — `72/213` 표기가 `spec/STATUS.md` 및 병합 후 기록과 같다. 통과.
2. **C11 부모 항목** — 완료됐다고 표현하지 않고 미완료라고 썼다. 통과.
3. **C11.4 항목** — 제한 fixture를 전체 C11.4 완료로 표시하지 않았다. 통과.
4. **병합 PR** — #905의 병합 상태와 URL을 확인했다. 통과.
5. **병합 시각** — `2026-10-31T05:46:15Z`가 PR 기록과 일치한다. 통과.
6. **병합 커밋** — `454036612372241fa7b162d6f87c78674ab7e559`가 PR merge SHA와 일치한다. 통과.
7. **필수 CI 표현** — 필수 check와 보고된 check이 없었다고 기록했으며 CI 성공이라고 쓰지 않았다. 통과.
8. **Fixture 경계** — 작업을 기존 opt-in fixture 경로에 한정했다. 통과.
9. **입력 범위** — Ahem 10px direct-text Grid라고 명시했다. 통과.
10. **장면 수** — Android와 iOS를 각각 16/16으로 표기했다. 통과.
11. **측정 필드** — computed layout, 줄 수, intrinsic 기여, Rows track, frame/revision만 근거 README에 맞춰 열거했다. 통과.
12. **Android 대상** — API 37 ARM64 AVD로 표기하고 실기기라고 부르지 않았다. 통과.
13. **iOS 대상** — iPhone 17 Pro / iOS 26.2 Simulator라고 표기했다. 통과.
14. **Android 화면 계층** — PixelCopy 확인을 주장하지 않았다. 통과.
15. **iOS 화면 계층** — compositor 이후 표면 확인을 주장하지 않았다. 통과.
16. **Glyph paint** — route/status 화면이며 glyph paint가 꺼졌음을 적었다. 통과.
17. **Glyph pixel** — 글자 모양·잉크의 pixel 정확성을 주장하지 않았다. 통과.
18. **일반 CSS admission** — 임의 author CSS 지원 완료를 암시하지 않았다. 통과.
19. **WPT** — WPT 실행이나 적합성을 주장하지 않았다. 통과.
20. **하드웨어 범위** — 실기기와 hardware GPU를 미검증으로 남겼다. 통과.
21. **Archive 경계** — default 제품 archive와 fixture-test archive의 scene marker 상태를 runtime evidence의 두 artifact로 한정했다. 통과.
22. **App bundle 경계** — archive 결과를 APK/iOS app bundle의 fixture 부재 증거로 확대하지 않았다. 통과.
23. **계획 링크** — 공개 경로 `/spinon/plan/c11-4-anonymous-grid-text-computed-line-height.html`가 소스 계획 파일명에 대응한다. 통과.
24. **실행 근거 링크** — `/spinon/internal/evidence/.../README.html`가 선택 빌드에 포함할 README 경로다. 통과.
25. **검토 링크** — 구현/runtime 20개와 PR diff 30개 문서의 경로가 저장소 파일명과 맞는다. 통과.
26. **현재 검토 링크** — 이 30개 리뷰의 공개 route를 추가 문단에서 가리킨다. 통과.
27. **화면 링크** — iOS 대표 화면 파일이 runtime README의 이미지와 같은 경로다. 통과.
28. **공식 상태 링크** — `/spinon/STATUS.html#css-구현-체크리스트`를 가리킨다. 통과.
29. **HTML 구조** — 새 문단은 기존 `kanban-note hero-progress` 클래스와 `<p>/<strong>/<code>/<a>` 구조를 유지하고 overview section 앞에 놓였다. 통과.
30. **변경 범위** — 공개 로드맵 항목과 이 근거 파일만 변경하며 C11 checkbox나 완료 수 원본을 수정하지 않았다. 통과.

## 결론

공개 로드맵 추가 문구는 #905의 제한된 구현·검증 범위를 유지하고, 일반 CSS·WPT·실기기·표면 pixel 및 전체 C11 완료를 과장하지 않는다. PR diff 검토에서 수정 요청은 없다.
