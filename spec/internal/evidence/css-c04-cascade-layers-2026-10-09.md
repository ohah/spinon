# C04.4 CSS Cascade Layers 실행 근거

**날짜:** 2026-10-09
**상태:** 제한 Flex alignment profile 구현·Chromium 비교 완료
**제품 지원:** 미완료 · runtime, 공개 API, GPU 화면, Android/iOS 앱 연결 없음

## 구현한 동작

Stylo `0.22.0`가 CSS Cascade Layers의 실제 승자를 계산한다. 새 `FlexAlignmentCascadeLayersV1` profile은 Stylo `LayerBlock`·`LayerStatement`를 허용하고 layer 안의 style rule까지 기존 declaration allowlist를 재귀 검사한다. 기존 C04.3 profile은 별도로 유지한다.

성공 입력은 named/anonymous/nested layer, order statement, layer 재개방, 다중 author stylesheet, normal·important·unlayered 순서, selector specificity와 source order를 포함한다. 미지원 at-rule·property·custom property·CSS nesting·parse diagnostic은 전체 실패한다.

## 고정 Chromium 비교

- Chrome `154.0.8037.98`, Chromium revision `@b859317bf11f6be47f9b7799ec690a0a42a1fb33`
- macOS `26.5.1` build `25F80`, arm64; Node `v24.20.0`
- viewport `301×40` CSS px, scale `1`, locale `en-US`, timezone `UTC`, `screen/light`
- 성공 case 16개, computed `align-items`·`justify-content` 문자열 정확 비교, parent와 3개 child의 64 frame 개별 좌표 오차 최대 `0.5 CSS px`
- fixture input SHA-256: `f014241101d538d2d62fe6ff8951c29235f5d0efcd060ba856340af61c31cb9d`
- fixture HTML SHA-256: `64998246c8bf06d7e3e350e5ea96fb3b8e4017deddeef7d82744831ebc68ade9`
- capture tool SHA-256: `77645537da5afc73a030a185b9cf276e1e3a48136af5a347a4f655f2df60f03a`
- Chromium executable SHA-256: `ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954`
- reference ID: `chromium-darwin-arm64-Chrome-154.0.8037.98-f014241101d5-b1f65c7f961c-77645537da5a-ccffd5c5fe77`
- reference JSON SHA-256: `e93f2aa511bcfaffa737b6c331a1dfad30c56035f70b2f978c69d2c494912c35`

16개 정상 case 결과:

| case | Chromium 계산 결과 | 확인한 실패 경계 |
|---|---|---|
| `normal-layer-order` | `justify-content:flex-end`, `align-items:flex-end` | 나중 normal layer가 승리 |
| `explicit-layer-order` | `center` | order statement가 block 출현 순서를 재정의 |
| `layer-reopen-order` | `flex-start` | named layer 재개방이 최초 순서를 유지 |
| `cross-stylesheet-order` | `space-between` | stylesheet 간 동일 origin layer tree |
| `same-layer-cross-stylesheet-order` | `flex-end` | 같은 named layer를 두 stylesheet에서 이어 쓰고 source order 유지 |
| `unlayered-normal` | `center` | unlayered normal이 explicit layer보다 우선 |
| `important-layer-reverse` | `center` | important 선언에서 layer 순서 반전 |
| `important-unlayered` | `center` | explicit important가 unlayered important보다 우선 |
| `importance-before-layer` | `flex-end` | importance 단계가 layer 단계보다 먼저 |
| `specificity-within-layer` | `center` | 같은 layer 안 ID specificity |
| `layer-before-specificity` | `space-around` | layer 우선순위가 specificity보다 먼저 |
| `nested-parent-implicit-layer` | `flex-end` | parent implicit sublayer 순서 |
| `nested-sibling-order` | `center` | nested layer statement 순서 |
| `anonymous-layer-identity` | `flex-end` | anonymous block의 별도 정체성 |
| `multi-name-layer-statement` | `space-between` | 다중 layer 이름 순서 |
| `same-layer-source-order` | `flex-end` | 동일 layer·specificity에서 뒤 선언 승리 |

## 실행 결과

| 명령 | 결과 |
|---|---|
| `mise exec -- node tools/css-reference/capture-c04-cascade-layers.mjs` | Chromium 16 case·64 frame 수집, immutable 새 경로 생성 |
| `mise exec -- node --test tools/css-reference/c04-cascade-layers.test.mjs` | 입력·HTML·도구·helper hash, case 순서와 frame 구조 1/1 통과 |
| `mise exec -- cargo test --locked -p spinon-style-to-layout` | 15/15 통과. C04.2·C04.3 회귀, 16개 layer geometry case와 실패 경계 포함 |
| `mise exec -- bun run test` | JS 2/2, CSS reference 16/16, Android 측정 도구 29/29, Rust workspace 테스트 전체 통과 |
| `mise exec -- cargo clippy --workspace --all-targets -- -D warnings` | 통과 |
| `mise exec -- cargo fmt --all -- --check` 및 `git diff --check` | 모두 통과 |
| `mise exec -- cargo check -p spinon-style-to-layout --target aarch64-apple-ios-sim` | 통과 |
| `mise exec -- cargo check -p spinon-style-to-layout --target aarch64-apple-ios` | 통과 |
| `mise exec -- cargo check -p spinon-style-to-layout --target aarch64-linux-android` | 통과 |

## 구현 후 적대 검토 · 별도 실패 관점

| # | 실제 실패 관점 | 코드·fixture에서 확인한 결과 |
|---:|---|---|
| 1 | 새 enum variant가 C04.3 값 whitelist를 바꿈 | 기존 C04.3 진입점은 새 `@layer`를 거부한다. |
| 2 | normal layer 승자가 반대로 나옴 | `normal-layer-order`의 computed string과 좌표가 Chromium과 일치한다. |
| 3 | order statement보다 block 출현 순서를 사용 | `explicit-layer-order`가 CSS 명시 순서를 따른다. |
| 4 | layer 재개방이 layer를 뒤로 이동 | `layer-reopen-order`에서 최초 순서를 보존한다. |
| 5 | 같은 이름의 layer가 author stylesheet마다 별개로 계산됨 | `same-layer-cross-stylesheet-order`에서 뒤 stylesheet 선언이 같은 layer winner에 반영된다. |
| 6 | unlayered normal이 explicit normal보다 약함 | `unlayered-normal`에서 unlayered winner가 확인됐다. |
| 7 | important에서 normal과 같은 방향으로 layer 순서 적용 | `important-layer-reverse`에서 첫 layer가 승리한다. |
| 8 | unlayered important가 explicit important를 덮음 | `important-unlayered`가 explicit layer를 선택한다. |
| 9 | layer 순위가 importance보다 먼저 적용 | `importance-before-layer`에서 낮은 layer의 important가 이긴다. |
| 10 | 같은 layer의 specificity를 무시 | ID와 class 충돌에서 ID computed value를 얻었다. |
| 11 | specificity가 layer order보다 먼저 적용 | 낮은 specificity의 후순위 layer geometry가 맞았다. |
| 12 | parent 직접 선언과 nested child를 평탄화 | parent implicit sublayer expected frame과 일치한다. |
| 13 | nested sibling order statement를 무시 | reversed block order에서도 statement의 later layer가 승리한다. |
| 14 | anonymous block 두 개를 같은 layer로 합침 | 두 번째 anonymous layer의 computed value가 선택됐다. |
| 15 | 한 statement의 여러 name 등록 순서를 잃음 | 세 layer statement에서 override layer가 승리한다. |
| 16 | 동일 layer·specificity의 source order를 뒤집음 | 마지막 rule의 computed value와 frame이 일치한다. |
| 17 | 허용 declaration 검사 누락이 layer 내부에서 생김 | nested traversal이 `align-self`·custom property를 거부한다. |
| 18 | 미지원 at-rule/CSS nesting/`@import`를 통과시켜 외부 CSS를 불러옴 | `@supports`, `@media`, nested selector와 `@import layer()`를 전체 거부한다. `@import` 로더는 꺼져 있으며 registry에서 parse diagnostic도 확인한다. |
| 19 | malformed layer 뒤 partial output을 반환 | parse diagnostic을 `CascadeDiagnostic`로 실패 처리한다. |
| 20 | 새 profile에서 문서·style·environment revision 소실 | nonzero revision echo와 foreign generation 거부를 확인한다. |

이 변경은 현재 Rust fixture adapter의 CSS 동작이다. 제품 DOM/runtime 및 화면 호환성 결과로 확대 해석하지 않는다.
