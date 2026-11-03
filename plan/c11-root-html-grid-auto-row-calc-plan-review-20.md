# C11.1 Root HTML `grid-auto-rows` calc 계획 실패 관점 검토 20개

**대상:** [계획](./c11-root-html-grid-auto-row-calc.md), [Chrome 156 관측](../spec/internal/evidence/c11-root-html-grid-auto-row-calc-precomparison-2026-11-03/chrome156-observation.json), [독립 기대값](../spec/internal/evidence/c11-root-html-grid-auto-row-calc-precomparison-2026-11-03/expected-values.json)

1. **C11.1 또는 C11 전체 완료로 승격될 위험 — PASS.** 계획은 제한 Root HTML slice만을 목표로 하고 상위 완료와 공식 count 승격을 금지한다.
2. **기존 px/%/fr implicit row list 회귀 — PASS.** 기존 direct scalar/list 표현을 유지하고 해당 track 종류를 positive controls로 요구한다.
3. **explicit rows가 auto-row cycle에 포함됨 — PASS.** pattern index 0은 첫 implicit row이며 주 scene에서 `[12,25,15]`를 분리 계산했다.
4. **%의 basis를 viewport나 border-box로 오인 — PASS.** definite inner content block size를 기준으로 고정했고 box edge를 고정 fixture에서 0으로 제한한다.
5. **row-gap을 percentage basis에서 차감 — PASS.** `10% + 5px = 15px` track과 10px gap을 별도 더해 두 번째 시작을 25px로 고정한다.
6. **negative sum이 음수 track으로 Taffy에 도달 — PASS.** `1 - 10 = -9px`을 non-negative track range 기준 `0px`로 기대한다.
7. **100% 초과 값을 잘못 clamp — PASS.** 121px track과 visible overflow 보존을 수락값으로 명시했다.
8. **definite zero height를 indefinite로 처리 — PASS.** `0×10% + 5px = 5px`로 직접 검증하고 height 0을 허용한다.
9. **indefinite/cyclic height 계산이 우연히 통과 — PASS.** direct finite definite height만 받고 math-backed height 등은 거부한다.
10. **calc grammar와 computed-style normalization 혼동 — PASS.** typed two-term signed px/% Sum만 새로 허용하고, length-only math가 direct `LengthPx`로 정규화되는 경로는 기존 지원으로 둔다.
11. **계산식 node/property binding 누락 — PASS.** `GridTemplateRows`, node owner, missing/duplicate/mismatched ID를 명시했다.
12. **정수/track limit 및 empty list 문제 — PASS.** non-empty list와 `GRID_MAX_TRACKS` cap을 projection/layout 양쪽 관문에 포함했다.
13. **개별 값은 유한하지만 누적 extent가 overflow — PASS.** expanded tracks 및 gap 전체의 finite f32 extent를 발행 전 검증하도록 요구했다.
14. **unsupported row가 explicit-only scene에서만 우연히 숨음 — PASS.** invalid value, list shape와 unsupported math는 implicit row 수와 무관하게 거부하는 matrix를 계획했다.
15. **column/row property math namespace 혼용 — PASS.** property binding을 `GridTemplateRows`로 한정하고 wrong-property negative case를 둔다.
16. **named area, dense/column flow 또는 authored placement로 admission이 전파 — PASS.** 해당 조합을 명시적으로 계속 거부한다.
17. **syntax-invalid declaration과 valid unsupported computed math를 같은 오류로 취급 — PASS.** cascade diagnostic과 profile rejection을 별도 검사한다.
18. **실패 후 partial frame이나 stale 결과가 남음 — PASS.** publication 전 검증, partial output 부재와 후속 valid request recovery를 요구한다.
19. **Chrome serialization을 normative truth로 삼음 — PASS.** 독립 산술과 고정 명세를 판정 근거로 두고 browser는 observation으로 제한한다.
20. **Simulator 화면을 pixel/WPT/기기 근거로 확대 — PASS.** runtime, draw/present, readback, 실기기, GPU, WPT의 증거 범위를 분리한다.

**판정:** 계획은 기존 percentage/fraction row list 구현과 계산식 column sibling의 경계를 이어받되, row property의 typed math binding과 extent 검사라는 별도 실패 표면을 분리한다. 구현 진입 전 기준 fixture·관측·독립 기대값이 고정됐다. 구현/runtime 검토 및 PR diff 검토는 이 표에 포함하지 않는다.
