# C11.1 intrinsic Grid track과 template area · 계획 실패 관점 검토 20개

**대상:** [계획](./c11-root-html-intrinsic-template-areas.md), [Chrome 156 사전 비교](../spec/internal/evidence/c11-root-html-intrinsic-areas-precomparison-2026-11-05/README.md), [CSS Grid Level 1](https://www.w3.org/TR/css-grid-1/)

1. **별도 profile을 만들어 기본 경로 결손을 우회** — 기존 Root HTML intrinsic-track mode를 확장하도록 요구했다. 통과.
2. **area template을 단순 문자열 목록으로 받아 비직사각형 map 통과** — CSS Grid의 named-area rectangularity를 검증하고 malformed map을 거부하도록 했다. 통과.
3. **unknown `grid-area` 이름을 1×1 implicit area로 조용히 처리** — unknown name rejection을 명시했다. 통과.
4. **named area가 포함된 item의 span을 잃음** — area의 시작/끝 line과 두 축 span 보존을 요구했다. 통과.
5. **두-track spanning contribution을 한 track에 임의 배분** — Chrome area-span 장면과 독립 span invariant를 두고 track vector 전체를 비교하게 했다. 통과.
6. **intrinsic rows만 지원하고 columns는 놓침** — 고정 matrix에 columns와 rows를 모두 포함했다. 통과.
7. **`minmax()` area 경로의 min/max bound와 automatic minimum 범위를 혼동** — 별도 `minmax(min-content,max-content)` 장면을 고정하고 기존 미지원 `minmax(auto, px|%)` named-area 조합은 음성 경계로 남겼다. 통과.
8. **area/fixed와 no-area/intrinsic의 기존 결과를 변경** — 두 기존 경로의 regression을 요구했다. 통과.
9. **area template 누락 시 CSS cascade fallback을 바꿈** — valid author cascade와 invalid declaration fallback을 계층별로 보존한다. 통과.
10. **zero-basis percentage를 area 통합을 핑계로 허용** — 기존 definite-axis gate를 유지하고 rejection control을 추가했다. 통과.
11. **auto-repeat/implicit track이 우연히 통과** — 명시적으로 계속 fail-closed한다. 통과.
12. **named-line metadata와 area aliases를 섞어 line order를 손상** — named-line metadata는 이 slice에서 계속 거부한다. 통과.
13. **column-flow/dense placement를 row-sparse profile에 섞음** — row flow만 유지하고 다른 flow는 거부한다. 통과.
14. **direct-text aspect-ratio mode가 intrinsic-area mode보다 우선해야 하는 scene을 탈취** — ratio owner 경로를 별도 negative control로 둔다. 통과.
15. **revision이 다른 area와 track 값을 조합** — 동일 style/layout revision과 root identity를 유지하도록 요구한다. 통과.
16. **layout 오류 뒤 이전 scene 일부를 publication** — malformed map과 sizing failure에서 atomic rejection을 확인한다. 통과.
17. **10,000-track 및 aggregate metadata cap을 건너뜀** — 기존 caps와 overflow guard를 바꾸지 않는다. 통과.
18. **Chrome 폰트 advance를 규범상 exact 값으로 오인** — browser font observations와 independent structural invariants를 분리하고 0.5 CSS px tolerance를 쓴다. 통과.
19. **Simulator draw/present를 pixel parity로 주장** — Android PixelCopy, iOS Metal readback, draw/present를 구분해 기록한다. 통과.
20. **제한 조합 하나로 C11 completion count를 올림** — C11.3.3/C11.4/C11 체크와 전체 count를 유지한다. 통과.
