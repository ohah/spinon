# C11.4 여러 direct text item 계획 PR diff 적대 검토 30개

**검토 대상:** 계획 PR의 실제 변경분 — 계획, 계획 검토 20개, Chrome fixture·관측·기대값·화면 · **기준 코드:** `ddfb2782fb6b5d3b8eeeff0df9c1c24f58d95121`
**검토 원칙:** 각 항목은 다른 실패 경로 또는 계약 경계를 확인한다. 구현 검토 20개를 대신하지 않는다.

1. **규범 URL:** 고정 W3C CSS Grid 2025 CRD 페이지를 가리키며 임시 편집기 초안에 의존하지 않아 통과.
2. **규범 조항:** §7.6 implicit rows와 §11.1/§11.5 sizing/contribution의 역할을 분리해 적어 통과.
3. **Algorithm overclaim:** 고정 one-column, one-row-span 장면의 근거로 전체 Grid sizing algorithm 지원을 주장하지 않아 통과.
4. **Explicit/implicit 경계:** 첫 10px row와 이후 implicit rows를 별도로 계산해 통과.
5. **Placement sequence:** four direct items의 auto placement 순서와 row index가 명시돼 통과.
6. **Long text contribution:** 긴 text의 2줄×10px 기대값과 frame height 20px가 일치해 통과.
7. **Short text contribution:** 짧은 text의 1줄×10px 기대값과 frame height 10px가 일치해 통과.
8. **Track sum:** `[10,20,10,34]` 합과 마지막 item의 y=40/bottom=74가 일치해 통과.
9. **Container overflow:** Grid box는 40px로 유지되고 content bottom 74px, overflow 34px 계산이 맞아 통과.
10. **Chrome source identity:** JSON과 독립 기대값에 같은 fixture SHA-256 `3232e75c…657970`이 기록돼 통과.
11. **Chrome environment:** HeadlessChrome 156, DPR 1, 최종 viewport 160×100, file URL을 관측 JSON에 보존해 통과.
12. **Declared/resolved width:** text item declared `auto`, computed 40px를 분리 기록해 통과.
13. **Text line evidence:** Range line rectangles와 row frame을 구분해 glyph pixel 증거로 오인할 여지가 낮아 통과.
14. **Source-order reversal:** DOM 순서 변경 후 row heights/frame 변화를 실제 Agent Browser에서 확인해 통과.
15. **Fixed-row control:** `grid-auto-rows:10px` 결과가 intrinsic positive와 다르고 마지막 item y가 30px로 이동해 통과.
16. **CSS source stability:** Chrome 관측 전후 fixture 파일은 수정하지 않았고 동일 hash를 사용해 통과.
17. **Screenshot scope:** viewport screenshot은 시각 참고로만 적고 native readback으로 표현하지 않아 통과.
18. **Text grammar:** 계획의 ASCII `A` token grammar는 fixture의 `A` 및 `A A A A A A` 입력과 일치해 통과.
19. **Whitespace rejection:** leading/trailing/multiple/other whitespace가 미지원임을 지원 경계에서 확인해 통과.
20. **Style admission:** font, line-height, whitespace wrapping, direction, writing mode 등 기존 typed style boundary를 빠뜨리지 않아 통과.
21. **Item topology:** 단일 direct text child, no nested markup 및 empty fixed control 분리를 명시해 통과.
22. **Owner topology:** visible Grid owner 하나 및 node ownership identity 조건을 명시해 통과.
23. **Hidden/detached cases:** 기존 visibility/ownership 처리를 유지하고 hidden regression을 계획해 통과.
24. **Revision consistency:** text, style, layout revision이 같은 snapshot이어야 한다는 조건을 명시해 통과.
25. **Contribution identity:** test는 각 item의 contribution identity/kind/value를 검사하도록 요구해 aggregate-only assertion을 막아 통과.
26. **Count resource bound:** 10,000/10,001 track 경계와 무거운 text layout 전 rejection 요구가 있어 상한 우회 위험을 낮춰 통과.
27. **Lookup complexity:** multi-item 증가로 반복 전체 style 검색이 quadratic해질 위험을 확인하고 node index 재사용을 계획에 넣어 통과.
28. **Atomic failure:** unsupported second item에서 completed frame/paint가 남지 않게 하는 negative control을 요구해 통과.
29. **Platform/artifact limits:** Android/iOS Simulator와 build/artifact checks를 요구하되 physical device, hardware GPU, compositor 이후 pixel은 주장하지 않아 통과.
30. **Repository/workflow boundaries:** C11.4/C11/count는 유지하고, 계획·구현 PR을 현재 checkout에서 순차 처리하며 Agent Browser를 닫도록 명시해 통과.

## 결과

30개 관점에서 blocking issue를 찾지 못했다. 이 검토는 계획과 사전 비교 artifact의 정확성만 다루며 구현 성공, CI 통과, PR merge 또는 공식 roadmap 완료를 뜻하지 않는다.
