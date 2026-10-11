# C12.4 · WPT 후보의 런타임 의미 확인

**상태:** 고정 WPT 원문 SHA-256 확인 · 공식 WPT 자동 후보 4/4 통과 · 수동 후보 1개는 Chrome 화면으로 별도 확인 · Spinon 자체 WPT 적합성 완료 아님
**계획:** [C12.4](../../../plan/c12-4-stacking-context.md) · **원본 비교:** [Chromium 154 사전 비교](c12-4-stacking-precomparison-2026-10-11.md)
**실패 관점 검토:** [20개 런타임 검토](c12-4-wpt-runtime-implementation-review-2026-10-11.md)

## 검증 방법과 경계

WPT 원문 다섯 개를 고정 revision `c999c58338ee1d223df5ad62e48be1202ced0d37`에서 읽고 저장소 inventory의 SHA-256과 대조했다. `wpt run` 공식 실행기로 자동 후보 네 개를 Chrome에서 실행했다. Chrome stable `154.0.8037.98`, ChromeDriver `154.0.8037.92` 조합이며 실행기 결과는 `4 ran as expected, 0 skipped`다. WPT Chrome runner의 `.test` 호스트 매핑을 썼고 시스템 hosts 파일은 수정하지 않았다. 다섯 번째 `z-index-stack-001.xht`는 WPT manifest에서 자동 실행에 포함되지 않는 수동 시각 검사다. 이를 Chrome 154 Headless에서 열어 파랑→주황→노랑 순서를 화면으로 확인했지만, 자동 WPT 결과 수에는 넣지 않았다.

중요한 경계: 이 실행은 Chrome이 고정 WPT 원문을 통과하는지 확인한 것이다. Chrome 실행 결과만으로 Spinon의 WPT 적합성을 주장할 수 없다. Spinon 대응 동작은 저장소 Rust/WGPU 회귀 테스트와 별도로 비교한다.

## 후보 대응 및 결과

| 고정 WPT 후보 | SHA-256 | Spinon 제한 profile 대응 | 결과와 경계 |
| --- | --- | --- | --- |
| `css/CSS2/zindex/z-index-stack-001.xht` | `71ddebb4643d0a99f4b07e7fa9e8f507c6ce2319be0ba4d3ffff5a37264440d1` | `wpt_css2_z_index_stack_001_orders_distinct_positive_levels` | 공식 runner 자동 실행 대상이 아닌 수동 시각 test다. Chrome 화면에서 겹친 상자의 아래→위 순서 파랑·주황·노랑을 확인했다. Spinon 쪽 Rust paint-rank 회귀 결과는 자동 WPT 결과가 아니다. [수동 확인 캡처](c12-4-stacking-runtime-2026-10-11/wpt-z-index-stack-001-manual-2026-10-11.png) |
| `css/CSS2/zindex/z-index-abspos-003.xht` | `d04f95aefe8ba06a49cd567e03595cd35dc580d32e234d5d79535e18cd9852c6` | `wpt_css2_z_index_abspos_003_paints_negative_child_after_context_background` | level 0 context 배경과 음수 자손의 순서를 확인했다. 원문 WPT 파일 실행 결과는 아니다. |
| `css/css-position/position-absolute-under-non-containing-stacking-context.html` | `6413b79fab31d82c3e84729e7e7364a93526ae5ffb19ba59c08dc25d9003e7ed` | `c124_profile_rejects_opacity_context_before_publishing_a_render_snapshot` | 원문이 요구하는 opacity 효과는 profile 미지원이므로 layout/render snapshot 전에 `unsupported_inline_property`로 거부한다. 이는 지원·WPT pass가 아니다. scroll/overflow 장면도 실행하지 않았다. |
| `css/css-flexbox/flexbox-items-as-stacking-contexts-001.xhtml` | `4ca1879dd67d79897770e949064838b97d95c517abc8e8fc343038942bcb5ddb` | `wpt_flexbox_items_as_stacking_contexts_keeps_descendant_inside_item` | static Flex item의 정수 `z-index`와 item subtree 격리를 검사했다. 원문의 border·전체 자식 배치 행렬은 포함하지 않는다. |
| `css/css-flexbox/flexbox-paint-ordering-002.xhtml` | `ebb748144bddd3b1fc6aa68b0a45d7a869c07db632ced97ab62fc2c7ff2c293d` | `wpt_flexbox_paint_ordering_002_uses_order_for_equal_z_index_ties` | 같은 양수 level에서 `order` tie와 낮은 level 분리를 검사했다. 원문 전체 order·z-index 조합과 border painting은 포함하지 않는다. |

## 공식 WPT 실행 결과

- revision `c999c58338ee1d223df5ad62e48be1202ced0d37`에서 `./wpt run --binary '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' --webdriver-binary ../wpt-c124-tools/chrome/chromedriver --headless --no-manifest-download chrome <고정 후보 5개>` 실행. 실행기에 수동 test가 자동 편입되지 않아 Chrome 자동 검증은 4개 후보였다.
- Chrome `154.0.8037.98`; ChromeDriver `154.0.8037.92`; 자동 실행 4개가 예상 결과로 종료했고 skip은 0개였다. [원본 실행 요약](./c12-4-wpt-candidate-runtime-2026-10-11.log).
- `z-index-stack-001.xht`는 Chrome 154 Headless에서 별도 열어 문서의 기대 순서(아래→위: 파랑, 주황, 노랑)를 육안으로 확인했다. 이는 자동 WPT runner의 결과가 아니다. [수동 화면 캡처](c12-4-stacking-runtime-2026-10-11/wpt-z-index-stack-001-manual-2026-10-11.png).

## Spinon 대응 회귀 결과

- `mise exec -- cargo test --locked -p spinon-style-to-render runtime_stacking_wpt -- --nocapture` — 4/4 통과.
- `mise exec -- cargo test --locked -p spinon-runtime c124_profile_rejects_opacity_context_before_publishing_a_render_snapshot -- --nocapture` — 1/1 통과. 반환 진단은 속성 `opacity`의 `unsupported_inline_property`다.
- `mise exec -- cargo test --locked -p spinon-render-wgpu --test runtime_stacking wpt_derived_stacking_cases_match_offscreen_pixels -- --nocapture` — macOS offscreen WGPU 표본 3/3의 RGBA가 정확히 일치했다.
- `mise exec -- bun run test:css-reference` — 168/168 통과. 이 실행은 고정 Chromium reference와 해당 inventory의 일관성을 검증한다.
- `mise exec -- cargo test --locked --workspace` — 전체 workspace 통과.
- `mise exec -- cargo clippy --locked --workspace --all-targets -- -D warnings`, `mise exec -- cargo fmt --all -- --check`, `git diff --check` — 통과.

## 남은 검증

공식 WPT 4개 자동 후보는 Chrome 154에서 통과했고, 한 수동 후보는 같은 Chrome 화면에서 기대 순서를 확인했다. 이는 Spinon 제품을 WPT runner에 연결해 얻은 결과가 아니며, opacity 후보는 profile 거부 경계로 확인했을 뿐 의미를 렌더링하지 않는다. position 후보의 scroll/overflow 동작도 실행하지 않았다. Flex 후보는 현재 제한 profile에서 표현 가능한 stacking 의미만 축약했다. Spinon 쪽에서는 별도 Rust paint-rank/runtime 회귀 4개, opacity 거부 1개, macOS offscreen WGPU 표본 3개를 실행했지만 이 결과도 공식 WPT 적합성이 아니다. Android AVD 26-scene runtime 및 21-scene surface 비교, iOS Metal reverse Flex 8-sample surface 비교, iOS Simulator 26-scene geometry matrix는 별도 evidence에 있다. 실기기 GPU/RSS, global allocator OOM 복구와 Spinon WPT runner 통합은 미확정이다. 실제 app worker 1,024-depth 성공·1,025-depth 거부·복구, 합성 painter 50,000단계/형제 stress, painter `Vec::try_reserve` 주입은 확인했으나 default test stack 256-depth overflow는 그대로 문서화한다. 따라서 이 후보 결과만으로 전체 CSS 지원이나 C12.4 완료를 표시하지 않는다.
