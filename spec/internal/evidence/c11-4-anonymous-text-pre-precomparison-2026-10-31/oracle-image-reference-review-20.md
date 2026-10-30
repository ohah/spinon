# C11.4 `white-space:pre` Chrome oracle 이미지 참조 검토 20개

1. **상대 이미지 링크 대상 누락 — PASS.** `chrome-156.png`가 같은 디렉터리에 추가되며 README 링크와 경로가 일치한다.
2. **Spinon runtime 캡처로 오인 — PASS.** 설명은 Agent Browser HeadlessChrome 156 reference screenshot이며 Spinon output이 아니라고 명시한다.
3. **사전/사후 비교 순서 전도 — PASS.** 이미지 설명은 implementation 전 Chrome oracle 관찰로 고정되어 있다.
4. **fixture 누락 또는 해시 불일치 — PASS.** README가 동일 폴더의 tracked fixture와 당시 SHA-256을 기록한다.
5. **폰트 로딩 실패 상태를 oracle로 사용 — PASS.** `document.fonts.check("10px Ahem") = true`가 README에 남아 있다.
6. **브라우저 버전 불명 — PASS.** HeadlessChrome 156.0.8078.76과 UA Client Hints full version이 README에 적혀 있다.
7. **screenshot이 CSS 전체 적합성 증거처럼 서술 — PASS.** README는 정확한 fixture 관찰로 제한하고 general conformance를 주장하지 않는다.
8. **screenshot이 앱 글자 렌더링 증명 — PASS.** `not Spinon output` 설명으로 대상 경계를 분명히 한다.
9. **Ahem 글꼴 폭 및 line-height 혼동 — PASS.** 관찰표는 고정 10px Ahem, 10px line-height profile과 별도 결과를 쓴다.
10. **min-content와 max-content 결과 혼동 — PASS.** 두 track mode 결과가 각자 표에 있다.
11. **preserved spaces가 normal control에 누락 — PASS.** normal/pre max-content 및 min-content 비교가 모두 보존된다.
12. **LF forced break의 zero-width Range rect 오해 — PASS.** 관찰 문장은 두 line과 Range의 zero-width LF rect를 구분한다.
13. **fixed-width overflow를 track overflow와 혼동 — PASS.** 25px root 안의 track과 50px line box를 분리해 적는다.
14. **임의 문서·font·script scope로 확대 — PASS.** 범위는 direct anonymous Grid text와 고정 ASCII strings다.
15. **mobile/platform 결과로 오해 — PASS.** screenshot은 desktop headless Chrome reference로만 취급한다.
16. **screenshot binary의 과도한 크기 — PASS.** image file은 약 4KB이며 matrix screenshots는 포함하지 않는다.
17. **이미지 교체로 원본 evidence 덮어쓰기 — PASS.** 기존 untracked oracle file bytes를 그대로 staging하고 내용을 재생성하지 않는다.
18. **Tailnet manifest의 source 불일치 — PASS.** 후속 publication은 병합된 main에서 tracked asset bytes를 복사하고 SHA-256을 기록한다.
19. **GitHub Pages 배포 범위 혼동 — PASS.** 이 문서 PR은 파일을 저장소에 추가할 뿐 Pages workflow를 실행하지 않는다.
20. **official implementation count 변화 오인 — PASS.** 이 이미지 참조 보완은 `spec/STATUS.md`의 C11 상태와 `72/213`을 바꾸지 않는다.

최종 상태: 기존 Chrome oracle image가 README 링크를 만족하며 Spinon runtime output으로 오인되지 않도록 caption을 명확히 한다.
