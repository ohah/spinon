# `full-size-kana` 계획 PR diff 적대적 검토 30개

**대상:** `plan/c11-4-anonymous-grid-text-transform-full-size-kana.md`, 계획 검토 20개, C11.4 master plan 갱신, 고정 HTML fixture 및 사전 비교 evidence

1. **잘못된 CSS 문서 revision 링크** — 날짜 고정 CRD 2026-08-14와 Appendix G anchor를 사용한다. **통과**
2. **at-risk 분류가 빠져 향후 호환성 보장으로 보임** — 계획·README·master plan에 at-risk 상태와 재판정을 기록했다. **통과**
3. **전체 CSS Text 구현을 약속** — opt-in C11.4 fixture candidate로 한정하고 공개 지원은 제외했다. **통과**
4. **정의가 small kana를 다른 normalization으로 바꿈** — Appendix G의 직접 코드 포인트 쌍을 독립 oracle로 고정했다. **통과**
5. **매핑 건수와 table rows 혼동** — source JSON은 header 포함 59 rows, mapping table은 58 pairs를 기록한다. **통과**
6. **매핑표가 runtime 기대값과 분리되지 않음** — 정규화된 JSON과 code point를 포함해 코드·테스트에서 대조하도록 계획했다. **통과**
7. **Halfwidth kana를 빠뜨림** — fixture·판정표에 halfwidth group을 별도로 둔다. **통과**
8. **Kana Supplement/Extended characters 누락** — Appendix G 전체 table 검사가 U+1B132/U+1B150대 및 U+31F0대 행을 포함한다. **통과**
9. **2-unit source를 1-unit destination으로 대체하며 source range 손실** — supplementary mapping의 UTF-16 range 검사를 acceptance에 명시했다. **통과**
10. **HostDocument 원문이 표시값으로 치환** — presentation-only transform과 원문 보존을 구분한다. **통과**
11. **white-space collapse보다 앞선 변환** — Phase I 뒤라는 CSS Text 순서를 고정했다. **통과**
12. **line-breaking 이후 변환** — intrinsic shaping·line breaking 전에 적용하도록 요구한다. **통과**
13. **비매핑 문자를 제거** — 표 밖 문자는 그대로 보존하도록 명시했다. **통과**
14. **표에 없는 소형 kana를 추정 매핑** — Unicode property 범위나 heuristic을 쓰지 않고 58개 표에 한정한다. **통과**
15. **언어 상속을 임의로 추론** — 계획 profile은 exact `lang=ja`만 받아들인다. **통과**
16. **computed style 대신 authored string으로 동작 결정** — 같은 revision computed owner 값을 요구한다. **통과**
17. **inline run boundaries에서 잘못 치환** — inline style/run은 negative input으로 거부한다. **통과**
18. **combined `text-transform` 값이 우연히 허용** — 단일 computed value만 이 계획에서 다룬다. **통과**
19. **Chrome 156을 semantic oracle로 오용** — `CSS.supports=false` 및 computed `none`을 unsupported 관찰로 기록했다. **통과**
20. **literal text candidate가 transformed target처럼 보임** — 사전 비교 문구와 README에서 literal 폭의 제한을 명시한다. **통과**
21. **Noto Sans KR 폭을 규범 기댓값으로 해석** — 글꼴 해시와 측정 폭은 브라우저 환경 참고값으로만 사용한다. **통과**
22. **screenshot이 변환 성공 화면으로 읽힘** — 화면 제목에 literal candidate와 unsupported target 한계를 표시했다. **통과**
23. **font asset 출처·hash 누락** — 기존 C15 NotoSansKR-VF와 SHA-256을 연결했다. **통과**
24. **fixture가 repository 밖 자원에 의존** — fixture는 저장소 내 Noto Sans KR를 상대 경로로 읽는다. **통과**
25. **계획 추가가 공식 상태 완료 수를 올림** — C11.4/C11 및 `72/213` 유지가 문서에 명시됐다. **통과**
26. **Simulator 화면을 glyph raster 검증으로 과장** — 화면은 layout smoke로 제한했다. **통과**
27. **실기기·hardware GPU로 오해** — 계획은 AVD와 iOS Simulator를 지정하고 해당 범위를 구분한다. **통과**
28. **WPT runner 완료로 과장** — full WPT conformance를 제외하고 Chrome의 unsupported status도 기록했다. **통과**
29. **review 문서가 구현 검토로 재사용** — 계획 단계만 검토하며 향후 implementation/runtime 20개와 별도 PR 검토를 요구한다. **통과**
30. **master plan에서 후보를 찾을 수 없음** — C11.4 master plan에 날짜, at-risk 상태, 계획 링크와 공식 집계 경계를 추가했다. **통과**
