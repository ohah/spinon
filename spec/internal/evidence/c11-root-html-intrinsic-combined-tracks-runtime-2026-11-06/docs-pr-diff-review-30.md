# C11.4 combined intrinsic tracks 문서 PR diff 검토 · 30개 관점

대상 변경은 `spec/STATUS.md`, `spec/internal/0059-c11-grid-layout.md`, `spec/public/roadmap.html`, `plan/c11-root-html-intrinsic-combined-tracks.md`의 PR #1195 병합 후 동기화다. GitHub에서 구현 PR 상태와 merge SHA를 조회하고, 실행 근거 README·계획·새 문서 diff를 대조했다. 이 검토는 문서 정확성만 다루며 기능 구현 재검증이나 C11 완료 판정이 아니다.

1. **대상 PR 식별:** PR #1195 제목·상태·병합 시각을 GitHub에서 다시 조회했다. `MERGED`, `2026-11-05T21:25:28Z`가 문서와 일치한다.
2. **merge SHA:** 상태 대장·내부 계약·공개 roadmap은 `3641212a08a9d5174880dac07f4e5aa146ae52e7`을 동일하게 쓴다.
3. **서울 날짜:** UTC 병합일은 11월 5일이고 문서 작업 날짜는 KST 11월 6일이어서 날짜 표기가 충돌하지 않는다.
4. **공식 완료 집계:** 상태 대장·내부 계약·공개 roadmap·계획에 `78/214`를 기록했고 상위 완료 수를 증가시키지 않았다.
5. **C11.4 checkbox:** 기존 `- [ ]` 상태를 보존한다.
6. **C11 parent checkbox:** 상위 C11 체크를 완료로 승격하지 않았다.
7. **집계 parser 영향:** 새 PR 설명 문장에 Markdown 체크박스를 추가하지 않아 공식 작업 수를 늘리지 않는다.
8. **지원 범위 표현:** bounded Root HTML slice라고 명시해 전체 C11.4 지원처럼 읽히지 않는다.
9. **공개 CSS admission:** 신규 public CSS 지원을 추가했다고 주장하지 않는다.
10. **기존 track grammar:** explicit column grammar는 기존 admission을 재사용한다고 기록하며 확장으로 오인하지 않게 했다.
11. **Container 축:** positive definite fixed-px container 조건을 계약에 적었다.
12. **Explicit rows:** 단일 positive fixed-px explicit row 경계를 명시했다.
13. **Implicit rows:** singleton `grid-auto-rows:min-content`만 추가되는 조합임을 기록했다.
14. **Implicit columns:** `grid-auto-columns:auto` 조건을 누락하지 않았다.
15. **Placement:** sparse row auto-placement와 `horizontal-tb`/LTR 제약을 계약에 남겼다.
16. **Gap 범위:** 기존 px/percentage gap만 유지하고 math-backed gap을 새 admission으로 확대하지 않는다고 썼다.
17. **Text item shape:** 단일 direct text child item과 sibling 조건이 기존 profile에 묶여 있다.
18. **Computed text style:** font, line-height, wrapping 및 revision 검증을 기존 bounded profile에서 유지한다고 기록했다.
19. **Cross-axis 순서:** column sizing이 text inline size를 정하고 실제 text measurement가 implicit row contribution을 정한다는 의존성을 설명한다.
20. **Min-content 기대값:** `[6.03125]px` column과 `[10,60,34]px` rows의 조합이 계획 matrix와 일치한다.
21. **Max-content 기대값:** `[66.234375]px` column과 `[10,10,34]px` rows를 독립 비교 README와 대조했다.
22. **Auto/flexible 기대값:** `[40]px` column과 `[10,20,34]px` rows를 계획 matrix와 대조했다.
23. **Gap scene:** 5% row-gap 장면은 definite-height basis를 사용하며 신규 gap 계산 규칙으로 과장하지 않는다.
24. **Fixture identity:** 세 문서의 fixture SHA-256은 실행 README의 `fc9ff608f630e7a6e6e2f185f8e49da93b696f5ea1b167efca4996082403380e`와 같다.
25. **Android 실행 범위:** API 37 ARM64 AVD와 ANGLE/SwiftShader를 명시해 실기기 GPU로 확대하지 않는다.
26. **iOS 실행 범위:** iPhone 17 Pro / iOS 26.2 Simulator임을 명시해 실기기 결과로 확대하지 않는다.
27. **Screenshot 증거:** screenshots를 visual smoke로 한정하고 PixelCopy·Metal drawable readback을 하지 않았다고 표시한다.
28. **Focused 검사 수:** layout 12/12, style-to-layout 23/23, runtime 5/5와 scoped Clippy/rustfmt가 evidence README와 일치한다.
29. **Broad suite 결과:** layout 453/1, style-to-layout 413/10, runtime 283/8/3을 실패 포함 값으로 기록하고 base 비교가 없어 기존 실패로 분류하지 않는다고 명시한다.
30. **CI와 제외 범위:** PR check rollup이 비었다고 기록하며 WPT, hardware GPU, iOS compositor pixels 및 전체 C11.4/C11 완료를 주장하지 않는다.

## 검증 한계

`mise exec -- bun run docs:build`는 전역 dead-link 검사에서 종료 코드 1을 반환했다. 출력은 이번 변경 문서 외 여러 internal evidence route의 dead links를 보고했으며, 변경 전 기준 checkout에서 같은 빌드를 실행하지 않아 이를 기존 문제라고 분류하지 않는다. 이 결과를 성공으로 표시하지 않으며, Tailnet 제공 후 Agent Browser에서 실제 공개 roadmap 카드·링크와 desktop/mobile overflow를 별도로 확인한다.
