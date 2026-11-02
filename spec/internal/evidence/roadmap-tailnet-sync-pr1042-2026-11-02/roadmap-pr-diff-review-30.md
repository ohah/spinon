# C11 PR #1042 공개 로드맵 반영 · PR diff 적대 검토 30개

검토 대상은 `spec/public/roadmap.html`의 새 C11.1 진행 문단과 이 기록이다. 각 항목은 서로 다른 실패 경로·계약 경계·표시 조건을 확인한다.

1. 병합 시각이 PR #1042 병합 기록과 일치하는가 — PASS, `2026-11-02T07:49:42Z`.
2. 공개 문단이 GitHub PR #1042를 직접 가리키는가 — PASS.
3. 병합 tip SHA가 정확한가 — PASS, `6f4d4a7f4cd58577541a89b9323910e7777a52a2`.
4. 코드 commit과 merge tip을 혼동해 단일 값으로 보고하지 않는가 — PASS, 문단은 merge tip을 표시한다.
5. 공식 완료 총계를 근거 없이 증가시키지 않는가 — PASS, `72/213` 유지.
6. C11.1 또는 C11 상위 항목을 완료로 표현하지 않는가 — PASS, 둘 다 미완료로 명시.
7. 적용 경로를 일반 CSS 경로로 확대하지 않는가 — PASS, parsed Root HTML route로 한정.
8. 첫 implicit row부터 pattern을 시작한다는 정의가 맞는가 — PASS, 실행 근거와 일치.
9. explicit rows에 auto-row list를 적용한다고 잘못 쓰지 않았는가 — PASS, 적용하지 않는다고 표시.
10. track 목록 허용 범위를 finite·non-negative px로 한정했는가 — PASS.
11. 빈 목록 거부를 명시했는가 — PASS.
12. 비 px 값 거부를 명시했는가 — PASS.
13. 음수 값 거부를 명시했는가 — PASS.
14. expansion 초과 거부를 명시했는가 — PASS.
15. `grid-auto-columns` 목록이나 implicit columns를 지원한다고 암시하지 않는가 — PASS, 범위를 rows로 한정.
16. named-area 조합이 포함됐다고 암시하지 않는가 — PASS, 해당 문단에 조합 주장을 두지 않음.
17. dense/column flow를 지원한다고 암시하지 않는가 — PASS, 기본 제한 구현 설명만 사용.
18. cascade 검사 수가 근거와 일치하는가 — PASS, 4/4.
19. projection 검사 수가 근거와 일치하는가 — PASS, 11/11.
20. layout 및 allocation-recovery 수를 구분하는가 — PASS, 6/6와 1/1로 분리.
21. runtime 검사 수가 근거와 일치하는가 — PASS, 20/20.
22. scoped Clippy를 전체 워크스페이스 성공으로 과장하지 않는가 — PASS, scoped 범위 명시.
23. Android 대상이 실기기가 아닌 API 37 ARM64 AVD라고 표시하는가 — PASS.
24. iOS 대상이 iPhone 17 Pro 실기기가 아닌 iOS 26.2 Simulator라고 표시하는가 — PASS.
25. 두 플랫폼 결과가 기본 route의 layout-ready·draw/present 관찰로 한정되는가 — PASS.
26. screenshot을 GPU 표면 readback 또는 픽셀 동등성으로 부풀리지 않는가 — PASS, 한계 명시.
27. WPT 적합성 주장을 추가하지 않는가 — PASS.
28. hardware GPU·물리 기기 검증을 주장하지 않는가 — PASS.
29. PR status check가 없던 사실을 CI 통과로 바꾸지 않는가 — PASS, CI 통과를 주장하지 않는다고 명시.
30. 모든 내부 링크가 계획·실행·검토·병합·플랫폼 화면·공식 상태를 분명히 나누고 올바른 route를 가리키는가 — PASS, bounded Rspress build에서 roadmap card의 8개 local href source와 `STATUS.html#css-구현-체크리스트`를 확인했다. generated `STATUS.html`, plan, README route의 산출물 hash는 `selected-route-verification.json`에 둔다.

## 검증 범위

선택 root로 Rspress 2.0.22를 실행해 `STATUS.html`, 전용 계획, runtime README 세 route를 생성했고 카드의 local 링크 여덟 개가 각각 생성 페이지 또는 추적 source에 존재함을 확인했다. 이 검증은 full docs build가 아니다. 표준 전체 build 시도는 기존 저장소 dead-link, `rust,ignore` Shiki language, S04 상대 이미지 import 오류로 종료 코드 1을 반환했다. 해당 문서들은 이 diff에서 변경하지 않았다. 실행 결과와 route hash는 `selected-route-verification.json`에 기록한다.

## 판정

서술 범위와 수치가 병합된 구현 및 근거에 맞는다. 30개 관점에서 과장, 완료 상태 오인, 근거 종류 혼동을 발견하지 못했다. 내부 링크의 생성 route와 공식 상태 anchor를 선택 문서 build에서 확인했다.
