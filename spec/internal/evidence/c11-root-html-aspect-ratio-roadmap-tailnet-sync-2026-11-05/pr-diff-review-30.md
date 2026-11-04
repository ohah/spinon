# 로드맵 동기화 PR diff 적대 검토 30개

범위: `spec/public/roadmap.html`, C11.1 aspect-ratio 실행 근거의 병합 상태 정정과 해시 목록, Tailnet 동기화·Agent Browser 증거. 저장소 코드나 공식 완료 상태를 변경하지 않는다.

1. **병합 상태:** GitHub PR #1161이 `MERGED`이며 merge SHA가 `f774af7f1b7441708495b74a063796122a4f1128`인지 확인했다. 페이지와 실행 근거가 같은 SHA를 쓴다.
2. **날짜 경계:** 새 요약은 2026-11-05이고 #1156 등 이전 기록은 2026-11-04다. 이전 수치를 현재값처럼 덮어쓰지 않는다.
3. **공식 완료 수:** 새 요약과 동적 상태가 모두 78/214이며, 이 문서 PR이 완료 수를 직접 올린다고 주장하지 않는다.
4. **상위 상태:** C11.1과 C11이 미완료로 남아 있어 제한 구현을 부모 항목 완료로 오해시키지 않는다.
5. **기능 범위:** aspect-ratio 설명이 기본 parsed Root HTML `RuntimeRootHtmlTextPaintV1`의 direct-text Grid slice로 한정되어 있다.
6. **사례 범위:** 두 direct block-text item 중 `AAAA`에 numeric 2/1 ratio를 연결한 예로 기술하며 일반 CSS 지원으로 확대하지 않는다.
7. **Chrome 근거:** 4-scene 사전 비교 주장은 연결된 pass04 README의 Chrome 156 precomparison과 일치한다.
8. **Android 종류:** Android API 37 ARM64 AVD를 명시하고, SwiftShader software renderer임을 숨기지 않는다.
9. **iOS 종류:** iPhone 17 Pro / iOS 26.2 Simulator로 표시해 실기기 결과로 오인되지 않는다.
10. **실행 관찰:** 두 플랫폼의 `layout=ready boxes=14`와 14-box draw/present는 연결된 로그 요약과 일치한다.
11. **readback 제한:** Android PixelCopy 및 iOS compositor 이후 readback을 수행하지 않았다고 적어 캡처 범위를 과장하지 않는다.
12. **집중 검증:** focused cascade/projection/layout/runtime 및 rustfmt·Clippy 통과 문구는 pass04 근거에 포함된다.
13. **전체 테스트 실패:** style-to-layout 10 failures와 layout C15 실패를 통과로 숨기지 않고 기록한다.
14. **기준 HEAD 대조:** 전체 실패가 base HEAD에서도 같은 이름으로 재현됐다는 제한을 같이 표시한다.
15. **CI 판정:** GitHub checks가 없었다고 기록하며 CI 통과로 표현하지 않는다.
16. **실행 근거 상태:** 연결 README의 과거 “PR 열려 있음” 문구를 MERGED와 실제 merge SHA로 고쳤다.
17. **근거 무결성:** 수정된 pass04 README의 `SHA256SUMS`를 새 digest로 갱신하고 전체 목록 검증을 수행한다.
18. **계획 링크:** aspect-ratio 계획 링크의 저장소 경로를 확인해 화면에서 이동할 문서를 지정한다.
19. **실행 링크:** 로드맵이 pass04 README를 연결하고, 해당 페이지가 Chrome/시뮬레이터 제한을 설명한다.
20. **계약 링크:** 내부 계약 0059를 연결하며 공개 CSS 전체 지원 명세처럼 표시하지 않는다.
21. **HTML 문맥:** 새 진행 문단이 기존 hero 영역의 `hero-progress` 문단들과 같은 구조에 있고 태그 nesting을 깨지 않는다.
22. **문자열 이스케이프:** `aspect-ratio`, `2/1`, `78/214`, PR SHA 같은 본문 텍스트가 HTML 요소를 깨지 않도록 code/anchor 요소 안에 둔다.
23. **기존 기록 보존:** 새 항목을 첫 C11 기록으로 추가하고 기존 PR 기록의 본문은 변경하지 않는다.
24. **과거 집계 해석:** 날짜가 붙은 과거 72/213 표기는 역사 기록으로 남기고 새 요약과 동적 78/214가 현재값임을 구분한다.
25. **Tailnet 경로:** preview 서버가 저장소 파일을 직접 서빙한다고 오해하지 않도록 복사 대상 `SpinonPreview/roadmap.html`을 기록한다.
26. **서버 수명:** 기존 PID 28040을 전후 동일하게 유지하고 서버를 재시작하지 않았다는 증거를 남긴다.
27. **교체 안전성:** 대상 파일을 백업하고 같은 디렉터리 atomic rename으로 교체했으며 파일 삭제가 없다고 기록한다.
28. **해시 대조:** 저장소 roadmap과 Tailnet 대상·로컬 HTTP route의 SHA-256이 동일한지 검증 자료로 확인한다.
29. **반응형 관찰:** Agent Browser 390×844에서 document/body 폭이 390px이고 horizontal overflow가 false임을 기록한다.
30. **변경 격리:** 변경한 tracked 파일은 roadmap과 연결 README/digest로 한정하고, 작업 트리의 기존 untracked 증거는 stage하지 않는다.

발견 및 수정: pass04 README의 PR #1161 상태가 병합 뒤에도 “열려 있음”으로 남아 있었다. GitHub merge SHA로 문구를 정정하고 해당 실행 근거의 해시 manifest를 갱신했다. 다른 범위·완료 상태·CI 주장을 넓히지 않는다.
