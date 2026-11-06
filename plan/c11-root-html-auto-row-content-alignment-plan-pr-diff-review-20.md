# C11.4 implicit auto-row content alignment 계획 PR diff 실패 관점 20개

**검토 대상:** 계획·사전 비교 evidence PR diff. 계획 자체 검토는 별도 [20관점 문서](./c11-root-html-auto-row-content-alignment-plan-review-20.md)에 기록했다.

1. **계획 변경이 제품 구현 완료로 보임 — PASS.** 문서 상태는 계획 단계이며 상태 대장·공식 count 승격은 이 PR에서 하지 않는다.
2. **PR #1234의 기존 `normal|stretch` 제한을 사실과 다르게 설명 — PASS.** head 및 merged plan의 현재 allowlist와 대조했다.
3. **CSS Grid §10.5/§11.8 의미를 잘못 귀속 — PASS.** §10.5 track alignment와 §11.8 `normal|stretch` auto-track growth를 분리했다.
4. **Position values 의미를 flex layout에서 가져옴 — PASS.** Grid에서 `flex-start|flex-end`의 CSS Box Alignment fallback을 근거로 썼다.
5. **Distributed values를 tracks sizing과 혼동 — PASS.** `normal|stretch`만 row sizes를 키우고 나머지는 track group을 배치한다.
6. **independent expected table과 JSON 간 값 불일치 — PASS.** 열 개의 computed values, rows와 y positions를 대조했다.
7. **`space-around` third fractional position 계산 오류 — PASS.** `5/3`, `15`, `85/3`과 Chrome quantized data를 나눠 보존했다.
8. **Quantization tolerance가 observed delta보다 작음 — PASS.** `1/32 CSS px`는 각 세 observed delta를 포함한다.
9. **HeadlessChrome/Agent Browser/version/viewport 환경 누락 — PASS.** user agent, DPR 1과 1280×633 viewport를 JSON에 남겼다.
10. **Screenshot을 wrong alignment로 부르거나 hash를 잘못 기록 — PASS.** start/stretch를 별도 캡처하고 파일별 SHA/크기를 계산했다.
11. **직전 stretch precomparison screenshot의 잘못된 SHA를 유지 — PASS.** `fixture.png`를 새 stretch capture로 교체하고 expected hash를 수정했다.
12. **이전 screenshot evidence를 파괴 — PASS.** 기존 start screenshot bytes가 새 `align-content-start.png`에 같은 SHA로 보존됐다.
13. **새 screenshot을 Spinon/runtime output으로 오인 — PASS.** README와 JSON에서 browser-only 한계를 표시한다.
14. **Canonical HTML이 코드에서 변경돼 결과 비교 불가 — PASS.** 기존 fixture를 수정하지 않고 SHA를 다시 대조했다.
15. **Scene contract에서 child source order 또는 dimensions 누락 — PASS.** fixed empty item과 `A`/`B` 10px text 및 content dimensions를 명시했다.
16. **새 API/profile/FFI 또는 contract version 변경이 계획에 암시 — PASS.** 재사용 경계를 명시하고 숫자 버전 0.1.0을 유지한다.
17. **지원 범위 밖 alignment/shorthand가 새 gate를 우회 — PASS.** baseline, modifiers, computed value allowlist와 shorthand axis 검사를 수용 기준으로 뒀다.
18. **제한 구현의 simulator 요구를 생략 — PASS.** Android/iOS positional 및 distributed representatives를 수용 기준에 둔다.
19. **적대 검토 횟수가 중복되거나 계획 PR diff 검토가 누락 — PASS.** 계획 review 20개와 실제 PR diff review 20개를 분리했다.
20. **문서-only PR을 runtime/CI 통과로 과장 — PASS.** 계획 PR에는 제품 test나 CI 성공 주장이 없다.

**결론:** 발견된 mismatch는 기존 stretch screenshot이 start control 캡처와 같은 SHA라는 증거 오류였다. 이를 수정하고 이전 byte를 start capture로 보존했다. 그 외 계획/evidence diff는 명시한 계산과 범위에 일치한다.
