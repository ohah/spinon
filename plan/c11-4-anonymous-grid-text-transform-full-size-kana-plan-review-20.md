# C11.4 `full-size-kana` 계획 실패 관점 검토 20개

**대상:** [계획](./c11-4-anonymous-grid-text-transform-full-size-kana.md), [CSS Text CRD](https://www.w3.org/TR/2026/CRD-css-text-3-20260814/#small-kana), [사전 비교](../spec/internal/evidence/c11-4-anonymous-grid-text-transform-full-size-kana-precomparison-2026-11-03/README.md)

1. **At-risk 상태를 안정 표준으로 오인** — 계획과 제외 범위에 CRD의 at-risk 분류 및 향후 재판정을 명시했다. **통과**
2. **Chrome 156의 computed `none`을 변환 정답으로 사용** — CSS.supports와 computed style이 negative support 관찰임을 고정했다. **통과**
3. **literal candidate 폭을 변환 증거로 과장** — 브라우저 폭은 글꼴 측정 참고값일 뿐 정답이 아니라고 구분했다. **통과**
4. **Appendix G 일부 행 누락** — 고정 표는 header 제외 58행이며 원본 추출 JSON과 code point 표를 연결했다. **통과**
5. **Unicode normalization이 Appendix G 매핑을 대체** — 정규화·호환 분해를 금지하고 정확한 코드 포인트 치환을 요구한다. **통과**
6. **Halfwidth kana를 놓침** — 별도 입력 그룹과 표의 halfwidth mapping을 모두 acceptance에 포함했다. **통과**
7. **작은 katakana 확장 문자 누락** — Appendix G 전체 행 검증이 U+31F0대 mapping을 포함한다. **통과**
8. **Supplementary kana를 BMP로 잘못 가정** — 별도 그룹과 UTF-16 source-range 검사를 둔다. **통과**
9. **source/output UTF-16 길이 차이로 range 손상** — 2-unit→1-unit 사례를 별도 실패 관점으로 고정했다. **통과**
10. **HostDocument 원문을 변환 문자열로 덮어씀** — presentation-only transform과 원문 보존을 acceptance에 명시했다. **통과**
11. **Transform을 white-space Phase I보다 앞서 적용** — normalization 이후 순서를 계획에서 고정했다. **통과**
12. **Transform을 line breaking 뒤에 적용** — intrinsic shaping·line breaking 전에 적용하며 track 결과를 확인하도록 했다. **통과**
13. **비매핑 문자를 삭제하거나 비슷한 kana로 추정** — 표 밖 문자는 identity로 두고 추정하지 않는다. **통과**
14. **비-Japanese source까지 일반 지원으로 주장** — opt-in profile은 exact `lang=ja`로 제한하고 일반 지원을 제외했다. **통과**
15. **Combined values가 우연히 허용** — 단일 computed 값만 통과시키고 복합 값은 범위에서 분리했다. **통과**
16. **다른 DOM node의 computed style을 빌려옴** — exact owner 및 동일 revision 검사를 명시했다. **통과**
17. **Inline run 경계에서 원문 매핑 불일치** — inline style/run을 거부하고 한 paragraph input을 요구한다. **통과**
18. **기본 제품 artifact에 fixture 코드 포함** — default/product와 opt-in positive control 경계를 확인한다. **통과**
19. **Simulator screenshot을 glyph pixel proof로 표현** — 화면은 layout smoke이며 raster/compositor 증거가 아님을 기록한다. **통과**
20. **제한 slice로 C11.4 완료 수를 올림** — parent checkbox와 `72/213` 유지 조건을 계획에 포함했다. **통과**
