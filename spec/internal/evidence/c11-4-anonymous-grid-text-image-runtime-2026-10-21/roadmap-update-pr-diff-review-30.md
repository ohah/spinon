# 로드맵 반영 PR 변경 실패 관점 검토 30개

대상 diff는 `spec/STATUS.md`의 C11.4 anonymous text/image 후속 기록과 `spec/public/roadmap.html`의 병합 결과 note다. 검토 기준은 병합된 PR #425, 고정 Chrome 155 5-scene oracle, [실행 근거](./README.md), C11.4 미완료 및 공식 집계 불변이다.

1. **상위 체크 승격** — `C11.4 intrinsic track sizing` 체크박스는 수정 diff에 없다. 미완료를 유지한다.
2. **공식 집계 변경** — checkbox 개수와 `[x]` 개수를 `origin/main`과 대조한다. 72/213 상태를 바꾸는 항목을 추가하지 않는다.
3. **상위 상태 문구** — 새 note가 전체 CSS Grid 구현으로 오해되지 않도록 `제한 후속`으로 부른다.
4. **계획 상태 일관성** — `plan/c11-4-anonymous-text-image-integration.md`의 구현·시뮬레이터 완료 상태와 새 상태 note가 일치한다.
5. **같은 revision 보장** — text와 image intrinsic snapshot을 동일 document/style revision의 계산으로 한정해서 적는다.
6. **한 번의 Grid 계산** — 입력 결합을 여러 계산의 결과처럼 표현하지 않는다.
7. **fixture 격리** — opt-in fixture adapter라고 명시해 일반 앱 경로 지원으로 확대하지 않는다.
8. **blockify 범위** — inline `img` blockification이 fixture 경로로 제한됨을 기술한다.
9. **기준 브라우저 버전** — oracle 기준을 Chrome 155로 적고 최신/모든 Chrome 동작으로 일반화하지 않는다.
10. **규범 절 참조** — 독립 기대 track 기준인 CSS Grid §6 및 §11.5.1을 그대로 표기한다.
11. **scene 수** — 검증 범위를 고정 5개 scene으로 한정한다.
12. **Android 기기 종류** — API 37 ARM64 AVD를 명시해 실기기와 구별한다.
13. **Android GPU 범위** — SwiftShader임을 남겨 hardware GPU로 오인되지 않게 한다.
14. **실기기 주장** — Samsung 실기기 실행을 이번 결과에 포함하지 않는다.
15. **iOS 환경** — iPhone 17 Pro / iOS 26.2 Simulator로 적고 iOS 실기기로 바꾸어 쓰지 않는다.
16. **scene gate 결과** — 각 플랫폼의 5/5가 track, frame, intrinsic trace, draw/present에 대한 결과임을 적는다.
17. **픽셀 의미** — draw/present를 compositor 이후 표면 검증으로 표현하지 않는다.
18. **이미지 texel 한계** — PNG texel paint를 확인한 것처럼 쓰지 않는다.
19. **일반 HTML 한계** — fixture 결과를 일반 HTML 결합 지원으로 확대하지 않는다.
20. **중첩 Grid 한계** — nested Grid 지원은 미완료로 유지한다.
21. **WPT 한계** — oracle 비교를 WPT 적합성으로 혼동하지 않게 남긴다.
22. **C11.4 완료 한계** — 전체 C11.4 구현 미완료를 두 문서에 모두 유지한다.
23. **PR 번호** — status와 roadmap이 구현 PR #425를 같은 대상으로 가리킨다.
24. **계획 링크** — `/spinon/plan/c11-4-anonymous-text-image-integration.html`이 현 계획의 RSPress route다.
25. **실행 근거 링크** — README route가 simulator matrix와 한계 설명을 제공한다.
26. **Android 이미지 경로** — 고정 AVD run의 실제 PNG 파일이 repository에 존재한다.
27. **iOS 이미지 경로** — 고정 iOS Simulator run의 실제 PNG 파일이 repository에 존재한다.
28. **캡처 대표성** — 링크는 대표 scene 한 장임을 명시적 설명으로 제한하며 다섯 scene 전체 화면인 것처럼 부르지 않는다.
29. **기존 #422 항목** — anonymous text-only 선행 PR의 별도 note를 삭제하거나 #425로 덮어쓰지 않는다.
30. **작업 트리 경계** — 변경 목록은 두 roadmap 문서와 이 검토 기록만 포함하며 선행 미추적 evidence를 stage하거나 삭제하지 않는다.

## 판정

위 30개 관점에서 문서 diff를 대조했다. 공식 checkbox 및 집계는 유지되고, simulator·fixture·pixel 검증 경계가 실제 근거와 맞으며, 새 링크는 병합된 #425 산출물로 연결된다. 추가 수정 사항은 발견하지 않았다.
