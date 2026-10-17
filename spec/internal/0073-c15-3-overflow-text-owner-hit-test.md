# 0073 · C15.3 overflow text-owner hit-test probe

**버전:** `0.1.0` · 내부 초안<br>
**상태:** 고정 Ahem fixture의 제한된 runtime snapshot 확인 · 공개 text hit-test API 아님<br>
**연결:** [C15 계획](../../plan/c15-inline-text-layout.md) · [공식 상태 대장](../STATUS.md#css-구현-체크리스트) · [Chrome oracle](./evidence/c15-3-unicode-shaping-precomparison-2026-10-17/README.md) · [paint/clip 실행 근거](./evidence/c15-3-overflow-clip-precomparison-2026-10-17/README.md)

## 범위

이 probe는 이미 연결된 C15.3 고정 Ahem `XX` runtime scene에서 `RuntimeRenderSnapshot::hit_test_css_point()`가 visible·hidden overflow owner의 clip과 paint order를 따르는지 검사한다. 별도 text range hit API나 glyph-mask hit algorithm은 추가하지 않는다. 반환 target은 runtime snapshot의 `RuntimeRenderBox`이며 DOM event dispatch가 아니다.

## 고정 입력과 기대값

- 입력은 360×240 CSS px fixture, DPR 1, Ahem 10px `XX`다. text-owner box는 40×10 CSS px이며 부모의 overflow clip 폭은 15 CSS px다.
- visible sample은 부모 오른쪽 경계에서 3 CSS px 바깥의 opaque Ahem glyph pixel 중심이다. 같은 좌표에서 runtime hit-test는 visible text-owner NodeId를 반환해야 한다.
- hidden-inside sample은 clip 안의 opaque glyph pixel 중심이며 hidden text-owner NodeId를 반환해야 한다.
- hidden-outside sample은 같은 부모 경계 밖의 opaque glyph 위치다. runtime hit-test는 hidden text-owner NodeId를 반환하면 안 된다. underlying anonymous/container box가 반환될 수 있으므로 결과 `None`은 요구하지 않는다.
- 같은 visible sample 위에 더 높은 hit-test order의 overlay box를 놓은 negative control은 text-owner가 아닌 overlay NodeId를 반환해야 한다. visible owner에 hidden clip을 잘못 적용한 negative control은 fixture admission을 실패시켜야 한다.

Chrome 155 DOM oracle는 visible sample에서 `visible-text`, hidden sample에서 id가 없는 underlying element를 관찰했다. 위 runtime probe는 hit-test clip·owner·순서를 고정 입력으로 확인하지만, DOM element identity 전체와 browser의 hit geometry를 구현했다고 주장하지 않는다.

## 제외 범위

glyph coverage별 text hit geometry, whitespace hit, grapheme/source range, CSSOM `elementFromPoint()` 일반 적합성, DOM event 전달, `pointer-events`, nested inline owner, 임의 font/line layout, scroll transform, 다른 overflow 값은 제외한다. 시뮬레이터의 pixel 표본은 지정한 같은 CSS sample이 화면에 그려진 잉크인지 확인하는 별도 근거다. 이 probe는 C15.3, C15, C16, C19 완료 판정에 사용하지 않는다.
