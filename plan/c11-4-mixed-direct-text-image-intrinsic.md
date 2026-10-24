# C11.4 · direct text와 image의 non-spanning intrinsic contribution

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 제한 runtime 후보다. Chrome 156 Agent Browser의 4-scene 사전 관찰과 Ahem/PNG에서 독립 계산한 기대값을 고정했다. Focused runtime/FFI test 각 1/1, workspace all-target Clippy, fixture-module boundary 12/12, 기본/opt-in FFI artifact 경계, Android API 37 ARM64 AVD·iOS 26.2 Simulator의 네 장면 실행이 통과했다. 구현/runtime 20관점 검토의 네 assertion·증거·정리 결함과 PR 변경 diff 30관점에서 발견한 문제를 수정·재검토했다. GitHub PR checks/merge와 post-merge 확인은 남아 있다. C11.4 checkbox와 공식 `72/213`은 유지한다. [사전 비교 README](../spec/internal/evidence/c11-4-mixed-direct-text-image-intrinsic-precomparison-2026-10-25/README.md) · [Chrome observation](../spec/internal/evidence/c11-4-mixed-direct-text-image-intrinsic-precomparison-2026-10-25/chrome156-observation.json) · [계획 실패 관점 검토 20개](./c11-4-mixed-direct-text-image-intrinsic-plan-review-20.md) · [계획 PR diff 검토 20개](../spec/internal/evidence/c11-4-mixed-direct-text-image-intrinsic-precomparison-2026-10-25/pr-diff-review-20.md) · [구현/runtime 실행 근거](../spec/internal/evidence/c11-4-direct-mixed-text-image-runtime-2026-10-25/README.md) · [PR diff 검토 30개](../spec/internal/evidence/c11-4-direct-mixed-text-image-runtime-2026-10-25/pr-diff-review-30.md)

## 확인된 결손과 목표

현재 opt-in runtime은 direct text-only와 image-only intrinsic scene을 각각 처리하고, 일부 고정 multi-span mixed scene도 별도 입력으로 처리한다. 그러나 text와 replaced image가 모두 direct child로 있는 하나의 비-spanning intrinsic Grid scene은 일반 text/image input 경로에서 명시적으로 거부된다. 기존 mixed scene은 span 2/3의 배분을 검사하므로 이 single-track 집계를 대신하지 않는다.

이 제한 slice는 `RuntimeBlockFlexGridTrackSizingV1` 안에서 같은 단일 column track을 점유하는 direct Ahem text item과 natural-size PNG item의 intrinsic contribution을 함께 계산한다. `min-content`와 `max-content` track을 각각 검사하고 markup source order를 뒤집어 결과가 유지되는지 확인한다. 이는 C11.4의 일반 text/image matrix 중 non-spanning direct-item 조합 하나이며, 전체 contribution algorithm을 닫지 않는다.

## 규범과 독립 기대값

- CSS Grid Layout Module Level 1, 2025 CRD §7.2.1에서 `min-content`/`max-content` track은 해당 track에 놓인 item의 같은 종류 contribution 중 가장 큰 값을 나타낸다. §11.5 step 2는 span 1인 비-spanning item을 먼저 처리하고, `min-content` minimum은 각 item의 min-content contribution 최댓값, `max-content` maximum은 max-content contribution 최댓값으로 정한다.
- fixture의 Ahem 10px 글꼴에서 `XX XXXXX`의 break opportunity는 공백 하나다. 두 단어의 glyph 수는 2와 5이고 공백 glyph를 포함한 전체는 8개다. 따라서 text min-content는 `5 × 10 = 50px`, max-content는 `8 × 10 = 80px`이다. 기존 24×16px PNG의 natural inline contribution은 두 종류 모두 24px이다.
- 한 track에 놓인 두 item의 규범적 결과는 각각 `max(50, 24) = 50px`와 `max(80, 24) = 80px`이다. 120px definite content width는 max-content growth limit보다 커서 max-content track을 80px 이상으로 늘리지 않는다. Row tracks는 두 개의 fixed 24px control이고 gap·padding·border는 0이다.
- 구현 전 Agent Browser CLI 0.31.1의 Chrome 156.0.8078.48, macOS ARM64, 1280×633, DPR 1을 관찰 기준으로 썼다. Chrome CSSOM/frame은 독립 규범 기대값과 비교할 관측이며 기대값 생성 입력으로 사용하지 않는다. [독립 계산 검증기](../spec/internal/evidence/c11-4-mixed-direct-text-image-intrinsic-precomparison-2026-10-25/validate-precomparison.py)는 1/64 CSS px 허용 범위에서 4/4 장면을 확인한다.

## 고정 장면

| Scene | Track | Source order | 독립 track 기대값 |
|---|---|---|---:|
| `direct-min-content-forward` | `min-content` | text, image | 50px |
| `direct-min-content-reversed` | `min-content` | image, text | 50px |
| `direct-max-content-forward` | `max-content` | text, image | 80px |
| `direct-max-content-reversed` | `max-content` | image, text | 80px |

앱 fixture route ID에는 `direct-` 접두어를 붙여 기존 cyclic mixed scene의 `max-content-forward` ID와 겹치지 않게 한다. 사전 비교 HTML의 `data-scene` label은 독립 관찰 fixture의 장면 이름으로 유지한다.

모든 scene은 같은 `XX XXXXX` direct text leaf와 `c14-1-png-basic.png` direct `img`를 사용한다. 둘 다 column 1에 있고 각자 fixed row 1 또는 2에 배치되어 span은 1이다. root는 120×48px, rows는 24px씩, Ahem은 10px/10px이다. Forward/reversed 장면의 markup 순서만 바뀐다.

## 구현과 완료 관문

1. 계획이 병합된 뒤 현재 source를 다시 확인한다. `C11_4MixedInput`의 고정 scene route를 확장하되 global runtime의 unsupported mixed-input rejection을 제거하지 않는다. scene ID가 정확히 일치하는 opt-in fixture 외에는 지원 범위를 넓히지 않는다.
2. `grid-template-columns:min-content|max-content`, 두 fixed rows, direct text leaf, decoded natural PNG, item order, CSS box edges, `RuntimeBlockFlexGridTrackSizingV1` profile과 parent/snapshot identity를 모두 검증한다. 다른 child 수·tag·text·image source·track/span/style·profile은 기존처럼 fail-closed한다.
3. Layout 결과에서 text/image 각각의 `grid_item_intrinsic_inputs` source, axis, revision, span-1 contribution과 같은 revision의 column stage trace를 확인한다. Used column track은 forward/reversed 모두 50px 또는 80px이어야 한다. item order, raw source order와 paint order는 각자 보존한다.
4. Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 4개 scene을 실행해 node identity/order, track list, item/root frame, trace completeness 및 draw/present를 확인한다. PR 본문에는 각 플랫폼의 대표 screenshot을 첨부하고 Agent Browser로 이미지 link가 실제로 열리는지 확인한 뒤 이번 브라우저 session을 닫는다. 이 결과는 simulator 증거이며 실기기·hardware GPU 또는 compositor 이후 pixel proof가 아니다.
5. Focused Rust/runtime/FFI checks, `cargo fmt --all -- --check`, workspace all-target Clippy와 fixture-module boundary 검사를 실행한다. 기본/product artifact에는 새 fixture scene marker와 FFI entry가 없고 opt-in fixture-test positive control에는 있는지 확인한다. Android/iOS fixture app을 각각 빌드한다.
6. 구현/runtime 실패 관점 20개와 실제 PR diff의 서로 다른 실패 관점 30개를 별도로 기록한다. 발견 결함을 고친 뒤 영향받는 관점을 다시 검토한다. PR live check 및 merge 상태를 확인하고 Rebase and merge한 다음 `main`에서 focused 결과와 문서 상태를 다시 확인한다.
7. `spec/STATUS.md`와 내부 계약 0059의 C11.4 progress log를 갱신한다. 이 고정 fixture가 통과해도 C11.4 checkbox, public CSS support 또는 공식 `72/213`을 올리지 않는다.

## 제외 범위

다중 track span, mixed span 배분 및 item source-order interaction beyond span-1 aggregation, nested/anonymous text, 다른 글꼴·스크립트·line-break mode, flex/automatic minimum, intrinsic row sizing, percentage/calc contributions, baseline shims, writing mode/RTL, author CSS 일반 admission, WPT suite, 실기기/hardware GPU, Android compositor 이후 표면 및 iOS compositor 이후 pixels는 포함하지 않는다. 이 slice에서의 성공은 `min-content`/`max-content`와 mixed text/image 전체 지원을 뜻하지 않는다.

## 구현/runtime 실행 결과

- `RuntimeBlockFlexGridTrackSizingV1` opt-in fixture에 네 개의 exact scene을 추가했다. 일반 mixed-input rejection은 유지한다.
- Android API 37 ARM64 16 KB AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 각 4/4 scene의 node frame/order, used tracks, intrinsic revision, trace completeness와 draw/present를 확인했다. 실패한 API 37.0 AVD의 Activity/Package 서비스 상태는 별도 로그로 보존하며 통과 건수에 포함하지 않는다.
- 화면 캡처는 배치 참고 자료다. 이번 실행은 text glyph 및 PNG texel의 pixel equality, Android 실기기/hardware GPU, iOS compositor 이후 pixels를 검증하지 않았다. runtime 로그도 `runtime_text_count=0` 및 `image_draw_count=0`이다.
- FFI archive 기본/positive-control 경계, workspace Clippy 및 두 플랫폼의 current-source build provenance는 확인했다. 남은 review와 PR 검증 결과는 [실행 근거](../spec/internal/evidence/c11-4-direct-mixed-text-image-runtime-2026-10-25/README.md)에 추가한다.
