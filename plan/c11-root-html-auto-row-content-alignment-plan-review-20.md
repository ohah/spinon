# C11.4 implicit auto-row content alignment 계획 실패 관점 20개

**대상:** [계획](./c11-root-html-auto-row-content-alignment.md), [독립 기대값](../spec/internal/evidence/c11-root-html-auto-row-content-alignment-precomparison-2026-11-07/expected-values.json), [HeadlessChrome 157 관측](../spec/internal/evidence/c11-root-html-auto-row-content-alignment-precomparison-2026-11-07/chrome-observation.json)

1. **상위 C11.4/C11 완료 또는 count 승격으로 오인 — PASS.** 고정 Root HTML scene 하나만 지원하고 parent check 및 `78/214` 유지가 분명하다.
2. **사용자에게 필요한 alignment 지원을 더 작고 편한 `start` 한 값으로 축소 — PASS.** 이 profile에서 구현 가능한 positional aliases 세트와 세 distributed values를 함께 포함한다.
3. **`normal`과 `stretch`의 auto-track sizing을 positional mode와 혼합 — PASS.** 두 값만 rows `[10,15,15]`; 나머지는 `[10,10,10]`으로 분리한다.
4. **두 개의 implicit auto tracks 중 하나만 stretch — PASS.** 10px free space를 각 track에 5px씩 균등하게 준다.
5. **explicit 10px row를 stretch 대상에 포함 — PASS.** explicit track은 고정 10px로 남고 auto rows만 변한다.
6. **`start`를 implicit track placement와 item alignment로 혼동 — PASS.** track size와 item y를 따로 비교해 Grid content alignment를 확인한다.
7. **`end`의 leading free space를 뒤에 배치 — PASS.** 첫 row y=10, 마지막 row end=40으로 고정한다.
8. **`center`의 양 끝 분배가 비대칭 — PASS.** y positions `[5,15,25]` 및 5px outer spaces를 쓴다.
9. **`space-between` outer gap을 추가하거나 내부 간격을 불균등하게 둠 — PASS.** first/last track은 edge에 맞고 두 내부 간격은 5px이다.
10. **`space-around`에서 outer half-gap이나 fractional quantization을 오판 — PASS.** 독립 fractions `[5/3,15,85/3]`와 Chrome 1/64 quantized observation을 분리한다.
11. **`space-evenly` outer gaps를 half-size로 계산 — PASS.** 세 track 사이와 양끝 네 간격을 각각 2.5px로 고정한다.
12. **`flex-start`/`flex-end`의 non-flex Grid fallback을 뒤집음 — PASS.** CSS Box Alignment §4.1 fallback에 맞춰 start/end aliases로 검증한다.
13. **CSS Grid §11.8 auto-track expansion 조건 누락 — PASS.** normative condition은 computed `normal|stretch`로 제한한다.
14. **Browser serialization을 기대값의 원천으로 사용 — PASS.** positions는 직접 산술로 계산하고 Chrome은 대조에만 사용한다.
15. **`align-content`를 `align-items`/`justify-content` 축에 적용 — PASS.** item alignment와 inline axis를 고정하고 별도로 검사한다.
16. **`place-content` shorthand의 `justify-content` 전이를 무검증 — PASS.** 양 computed axes를 각각 검증해 새 axis admission을 막는다.
17. **baseline 또는 overflow safety modifier가 일반 값으로 우회 — PASS.** 정확한 keyword allowlist와 explicit rejection boundary가 있다.
18. **non-zero gap, overflow, changed root geometry에서 계산을 재사용 — PASS.** 고정 dimensions/zero gaps/visible static Grid contract를 유지한다.
19. **Unsupported update 뒤 stale/partial layout이 남음 — PASS.** no-partial output과 same-session valid recovery를 수용 기준에 포함한다.
20. **Simulator screenshot을 geometry/pixel parity로 과장 — PASS.** screenshots는 visual smoke이며 frame assertion과 pixel readback 주장은 분리한다.

**판정:** 동일한 parsed Root HTML scene에서 고정 row sizing과 CSS Grid alignment distribution을 분리한다. 구현 범위는 이 scene에서 검증 가능한 일반 alignment value를 모두 포함하고, baseline/safety/overflow 및 arbitrary CSS admission은 이 단계의 의미로 확대하지 않는다.
