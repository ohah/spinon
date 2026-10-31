# C11.3.3 양 끝 implicit named-line 계획 PR diff 검토 20개

검토 대상은 계획 브랜치의 실제 staged diff 전체다. fixture, Chrome 관찰 JSON, 화면, 신규 계획 및 C11.3 상위 계획의 추가 링크를 구현과 분리해 검토했다.

1. **변경 경계:** Rust/runtime/공식 상태 체크를 건드리지 않는 계획 PR이다. PASS.
2. **기준 source:** Chrome 사전 비교가 Spinon baseline 실행으로 오해되지 않도록 source revision과 비실행 경계를 기록했다. PASS.
3. **브라우저 버전:** Chrome 156, Agent Browser 0.31.1, macOS, viewport, DPR을 고정했다. PASS.
4. **규범 URL:** CSS Grid 2025 CRD §8.3–8.3.1의 named-line 해석과 충돌 규칙, §7.5–7.6의 implicit track sizing을 연결했다. PASS.
5. **입력 파일:** HTML fixture의 SHA-256이 README와 일치하고, 관찰 ID들이 해당 fixture의 9 scene과 일치한다. PASS.
6. **Computed style 관측:** authored line과 CSSOM resolved track list를 서로 섞지 않고 JSON에 별도로 저장했다. PASS.
7. **Bare identifier:** area 없이 다른 이름 두 개를 쓰는 결과를 고정했다. PASS.
8. **반복 occurrence:** 2/3 occurrence에서 extra zero track과 item frame을 고정했다. PASS.
9. **같은 line:** `alpha / alpha` 결과를 같은-name 충돌 case로 분리했다. PASS.
10. **Forward pair:** 1/2 occurrence와 auto track 크기를 기록했다. PASS.
11. **Reverse pair:** 2/1 occurrence를 별도 scene으로 두어 line swap을 확인 가능하게 했다. PASS.
12. **Span distribution:** 1/3 case의 두 15px tracks가 fixed 30px contribution을 나누는 결과를 기록했다. PASS.
13. **Rows 축:** bare identifier와 1/2 occurrence를 두 scene으로 Columns 결과와 대조한다. PASS.
14. **Fixed-px control:** 동일 named placement에서 fixed `grid-auto-columns` 결과와 item offset을 분리했다. PASS.
15. **Geometry 단위:** 모든 frame을 grid root 기준 CSS px로 정의하고 root 바깥 item 좌표를 보존했다. PASS.
16. **Screenshot 증거:** PNG hash가 있고 README에서 바로 열 수 있으며, 이를 pixel-parity나 Spinon 결과라 하지 않는다. PASS.
17. **계획의 fail-closed 경계:** template areas, mixed endpoint, signed line, span, numeric line, 두 implicit axes, item sizing, 추가 child, track ceiling을 각각 제한했다. PASS.
18. **모드 격리:** 기존 positive named-start+auto-end mode를 넓히지 않고 새 mode로 후보를 분리하도록 했다. PASS.
19. **상위 문서 일관성:** C11.3 계획에서 새 후속 문서만 연결하고 초기 scope 및 공식 C11.3/C11 상태는 그대로 둔다. PASS.
20. **검증 주장:** AVD/iOS Simulator·artifact·PR merge는 계획된 절차로만 썼고 아직 실행 결과로 주장하지 않았다. PASS.

추가한 문서 간 상대 링크는 모두 해결된다. C11.3 상위 계획의 변경하지 않은 기존 본문 링크 세 개는 현재 checkout에서 대상이 보이지 않지만 이번 diff에서 새로 추가된 링크가 아니며 이 계획 PR의 변경 범위와 무관하다. 차단 결함 없음.
