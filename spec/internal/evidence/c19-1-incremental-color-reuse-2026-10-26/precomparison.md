# C19.1 incremental color 재사용 사전 비교 · 2026-10-26

## 고정 기준

- Git 기준: `origin/main` SHA `29449e4f794e53255909b58ed8fb2fd4995dd5d2`.
- 동작 계약: [`0068-c19-1-computed-color.md`](../../0068-c19-1-computed-color.md)의 `C19ComputedColorV1` computed `color`/`background-color` carrier.
- 이전 결과 oracle: 현재 문서 snapshot에서 `compute_c19_computed_color_cascade_with_stylesheets()`가 수행한 full cascade. 이것은 CSS 색상 의미를 재판정하는 oracle이 아니라, 같은 Stylo 계산을 재사용 경로가 보존하는지 확인하는 기준이다. C19.1의 Chrome/Stylo 색상 fixture는 기존 [C19.1 사전 비교](../c19-1-computed-color-precomparison-2026-10-16/README.md)를 계속 기준으로 쓴다.
- 재현 환경: Rust `spinon-style` crate 단위 테스트, `StyloDocumentView::new_html_fragment_child_shared`, 고정 `CssViewport` 및 `StyleRevision`. Browser, simulator, GPU 표면은 이 cache equivalence 범위에서 사용하지 않는다.

## 입력과 현재 관찰

고정 4-element HTML fragment는 root, container, container child, root sibling으로 구성한다. 초기에는 C19.1 full cascade로 이전 snapshot을 만든다. 다음 문서 commit은 container inline `color`와 child inline `background-color`를 바꾸며 child의 `color`는 상속되도록 둔다. dirty root는 container 하나이고 `StyleRevision`은 직전 값에서 정확히 1 증가한다.

고정 source에서 `runtime_properties_for_profile()`은 `RuntimeBlockPaintV1`, `RuntimeRootHtmlTextPaintV1` 등 일부 profile을 반환하지만 `C19ComputedColorV1`은 열거하지 않아 API가 `None` fallback을 낸다. 같은 profile은 incremental compute core의 author-property allowlist와 C19 typed color output 경로에는 이미 등록돼 있다. 기존 full C19.1 출력 property list는 Block layout/font 항목도 포함하지만 carrier 계약은 color 값이다. 구현에서는 full 및 incremental 경로 모두 `background-color`와 `color` 두 Longhand만 같은 순서로 출력하며, author declaration 허용 범위와 typed carrier 계산은 건드리지 않는 것으로 고정한다.

## 사전 판정 기준

구현 후보가 통과하려면 다음 조건을 모두 충족해야 한다.

1. C19.1 incremental 호출이 `Some(snapshot, stats)`를 반환한다.
2. incremental `elements`와 diagnostics가 현재 문서의 새 full cascade와 정확히 같다. 직렬화 property key는 `background-color`·`color`만 있어야 한다. 동등성은 `computed_color`와 `computed_background_color`의 모든 source/conversion 필드 및 legacy `background_color`·`background_paint`의 `None` 경계를 포함한다.
3. 변경 container와 child는 재계산되고 context root output과 변경 없는 sibling output은 재사용된다. 통계 개수와 node별 출력으로 각각 확인한다.
4. parent에서 상속한 child `color`와 dirty child의 새 background carrier가 변경된 full cascade 값과 일치한다. 이전 color 값이 재사용되는 stale-cache 부정 대조는 실패해야 한다.
5. author stylesheet, profile mismatch, viewport/environment mismatch, document identity/revision mismatch, 범위 밖 style revision은 계속 `None`을 돌려줘야 한다. 호출자는 full cascade fallback을 선택할 수 있어야 한다.

## 해석 제한

동등성은 cache 재사용의 정확성만 판정한다. 이 검사는 C19.1 CSS Color 4 적합성, Chrome parity, gamut mapping 알고리즘 우열, C19 paint/pixel 결과 또는 C19 완료를 새로 입증하지 않는다.
