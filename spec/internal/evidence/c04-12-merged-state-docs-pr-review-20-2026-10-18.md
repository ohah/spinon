# C04.12 병합 상태 문서 수정 PR 실패 관점 검토 20개

**검토 대상:** `spec/STATUS.md`와 `plan/c04-12-document-canvas-background.md`의 PR 병합 상태 문구
**검토 기준:** 2026-10-18 현재 `origin/main`, GitHub PR 상태, 기존 구현 근거
**변경 범위:** 구현 동작·상태 체크·완료 수를 바꾸지 않는 문서 정정

1. **상위 완료 승격:** `spec/STATUS.md`의 C04.12 상위 체크가 바뀌지 않았고 미완료다 — PASS.
2. **완료 수 오염:** 문서 수정에 74/207 집계 변경이 없고 기존 수를 보존한다 — PASS.
3. **구현 PR 상태:** PR #309는 GitHub에서 `MERGED`이며 병합 시각과 SHA가 확인된다 — PASS.
4. **로드맵 PR 역할:** PR #310은 Tailnet 공개 로드맵 이력 변경이며 기능 구현 PR로 표현되지 않는다 — PASS.
5. **계획의 완료 범위:** 계획 상태는 Canvas 하위 구현만 병합됐다고 적고 C04.12 전체 미완료를 유지한다 — PASS.
6. **Android 환경 과장:** 근거는 API 37 ARM64 AVD이며 Android 실기기 결과로 확대하지 않는다 — PASS.
7. **iOS 환경 과장:** 근거는 iPhone 17 Pro / iOS 26.2 Simulator이며 iOS 실기기 결과로 확대하지 않는다 — PASS.
8. **표시 경계 과장:** 캡처가 native content root 색 확인임을 유지하고 compositor 이후 픽셀로 주장하지 않는다 — PASS.
9. **body box 혼동:** Canvas의 선택된 body 배경값과 body box 자체의 paint를 구별한다 — PASS.
10. **일반 문서 paint 과장:** 일반 HTML layout·text·paint를 완료했다고 적지 않는다 — PASS.
11. **선택 순서 손상:** 기존 HTML root → 투명할 때 body → scheme fallback 순서를 유지한다 — PASS.
12. **색 범위 확대:** 정확히 표현되는 8-bit sRGB 제약을 유지한다 — PASS.
13. **실패 경계 누락:** 지원하지 않는 색이 fail-closed된다는 제한을 유지한다 — PASS.
14. **검증 수치 변조:** runtime/style/integration/Clippy 수치는 기존 병합 구현 근거의 기록과 일치하도록 그대로 둔다 — PASS.
15. **코드 변경 오인:** 이 수정은 상태·계획 문서만 바꾸며 구현 코드 변경으로 표시하지 않는다 — PASS.
16. **테스트 실행 오인:** 이번 문서 정정에서 새 코드 테스트를 실행했다고 주장하지 않는다 — PASS.
17. **계획 링크:** `plan/c04-12-document-canvas-background.md` 상대 링크의 저장소 대상이 존재한다 — PASS.
18. **실행 근거 링크:** Chrome·Simulator·Rust 근거와 검토 문서의 대상 파일이 저장소에 존재한다 — PASS.
19. **PR 링크:** 구현 PR #309와 Tailnet 이력 PR #310을 각각 실제 목적에 맞게 연결한다 — PASS.
20. **변경 경계·서식:** 변경은 이 하위 slice의 병합 상태만 정정하고 `git diff --check`에 공백 오류가 없다 — PASS.

**결과:** 위 실패 경계 20개에서 불일치를 찾지 못했다. C04.12 전체 지원, 물리 기기, hardware GPU 또는 전체 CSS 적합성 판정에는 이 기록을 사용하지 않는다.
