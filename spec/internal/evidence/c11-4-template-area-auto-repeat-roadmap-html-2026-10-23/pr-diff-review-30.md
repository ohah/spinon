# C11.4 roadmap HTML PR diff 검토 30개

대상 변경은 `spec/public/roadmap.html`의 기존 미완료 설명 한 곳 교체와 PR #539 runtime 결과 문단 추가다. 검토는 2026-10-23에 현재 `origin/main`과 병합 PR #539의 문서·실행 근거를 대조해 수행했다. 공식 상태 대장이나 구현 계약을 이 문서 PR에서 변경하지 않는다.

1. **상태 승격 여부** — 새 문장이 C11.4 전체를 완료로 보이게 하는가? 상위 항목은 미완료라고 명시하고 완료 수를 그대로 둔다.
2. **공식 완료 수** — `77/213`이 최신 상태 대장과 일치하는가? 병합된 #539 상태와 맞고 별도 증가로 서술하지 않는다.
3. **계획과 구현 구별** — 이전의 “runtime은 시작하지 않았다”가 남아 최신 구현과 충돌하는가? 기존 문장을 #538 계획 병합 및 아래 구현 기록을 가리키도록 교체한다.
4. **구현 PR 식별** — 결과가 #539에 연결되는가? 제목 문구와 GitHub 링크가 병합된 구현 PR을 가리킨다.
5. **병합 SHA** — 표기한 SHA가 실제 merge commit인가? #539의 merge commit `0f46437c299df6f0c069b694a3f2184ac464ad3d`와 일치한다.
6. **기능 범위** — 문장이 일반 CSS Grid 지원으로 과장되는가? 단일 auto-repeat과 template areas의 제한 runtime slice로 한정한다.
7. **반복 종류** — auto-fill과 auto-fit 모두를 무제한 지원한다고 읽히는가? fixed-minimum 단일 repeat fixture 경계 안에서만 서술한다.
8. **반복 축** — 한 scene에서 두 축 동시 지원을 암시하는가? 후속 한계 문구에서 미검증임을 분명히 한다.
9. **area 동작** — named area의 전체 CSS 동작을 약속하는가? rectangle과 generated alias가 선택된 resolver로 전달된 범위만 기록한다.
10. **resolver 의존성** — resolver가 자체 구현이라고 오해될 수 있는가? Taffy 0.15.0 named resolver를 명시한다.
11. **Android 환경** — 실기기 증거로 오해될 수 있는가? API 37 ARM64 AVD라고 정확히 표시한다.
12. **iOS 환경** — 실기기 증거로 오해될 수 있는가? iPhone 17 Pro라는 Simulator 대상과 iOS 26.2 Simulator를 함께 적는다.
13. **scene 수** — `13/13`이 양 플랫폼 각각의 결과인지 모호한가? Android와 iOS 각각이라고 문장에 쓴다.
14. **surface 표본 수** — `66/66`을 전체 이미지나 모든 픽셀로 오해할 수 있는가? PixelCopy 및 제출 전 Metal 표본으로 한정한다.
15. **Metal 경계** — iOS compositor 이후 픽셀까지 검증했다고 읽히는가? compositor 제출 전 drawable이라고 적고 후단 pixels는 미검증으로 남긴다.
16. **FFI artifact 주장** — static archive 결과를 APK/IPA 전체로 확대하는가? archive 경계만 비교했고 APK/IPA 제외는 미검증이라고 쓴다.
17. **내부 계약 상태** — 버전이 올라간 것으로 오해될 수 있는가? 0059의 `0.1.0` 유지 사실만 기록한다.
18. **parser 미지원 값** — `visibility:hidden` 결과를 통과로 오인하는가? parser 미지원이라 충돌 scene visibility를 검증하지 않았다고 밝힌다.
19. **여러 repeat** — multiple repeat가 완료된 것으로 읽히는가? 미검증 목록에 둔다.
20. **subgrid** — subgrid 범위를 암시하는가? 미검증 목록에 명시한다.
21. **일반 CSS admission** — runtime fixture를 임의 웹 CSS 지원으로 읽을 수 있는가? 일반 CSS admission은 미검증으로 분리한다.
22. **WPT** — Web Platform Tests 수행을 주장하는가? WPT는 미검증이라고 적는다.
23. **Android GPU/기기** — SwiftShader AVD 결과를 hardware GPU 또는 실기기 검증으로 읽을 수 있는가? hardware GPU와 Android 실기기를 미검증으로 둔다.
24. **계획 링크** — 집중 계획 URL이 저장소 문서 경로와 일치하는가? `plan/c11-4-auto-repeat-template-areas.md` 페이지 alias로 연결한다.
25. **계약 링크** — 0059 문서 링크가 정확한가? `spec/internal/0059-c11-grid-layout.md` alias 경로를 사용한다.
26. **runtime 근거 링크** — README 경로가 존재하며 플랫폼 증거를 모으는가? 병합된 runtime README 경로를 가리킨다.
27. **PR diff 검토 링크** — 현재 구현 PR의 검토와 새 roadmap 문서 PR의 검토를 혼동하지 않는가? 링크는 #539 구현 diff 검토 30개로 라벨링한다.
28. **Android 화면 링크** — 화면 파일이 저장소에서 추적되는가? 병합된 #539 Android AVD PNG 경로가 존재한다.
29. **iOS 화면 링크** — 화면 파일이 저장소에서 추적되는가? 병합된 #539 iOS Simulator PNG 경로가 존재한다.
30. **상태 anchor** — 처음 사용한 `#c11-css-grid`가 렌더된 공식 상태 페이지에 존재하는가? pass65 Agent Browser에서 target이 없음을 발견했다. 후속 PR #541은 실제 렌더된 `#css-구현-체크리스트` heading으로 바꾸고, 이 anchor가 Tailnet HTML에 존재하는지 다시 확인한다.

## 검토 결과

초기 diff 검토의 나머지 29개 관점은 통과했다. Tailnet pass65 Agent Browser 검증에서 기존 `#c11-css-grid` target이 없음을 발견해 30번 관점을 수정했다. PR #541은 `#css-구현-체크리스트`로 고치며 이 렌더된 heading을 다시 검증한다. 따라서 최초 PR 검토 결과에는 이 후속 수정 기록을 함께 읽어야 한다.
