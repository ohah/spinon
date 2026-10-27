# C11.4 · generic direct mixed text/image intrinsic runtime

**상위:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [direct text/image fixed-scene slice](./c11-4-mixed-direct-text-image-intrinsic.md) · [generic direct-text collector](./c11-4-generic-direct-text-intrinsic-input.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** 제한 opt-in runtime fixture 구현 및 Android/iOS Simulator 검증 완료. 현재 mixed adapter는 span, direct column, direct row scene을 고정 root/item ID로 dispatch한다. 이 slice는 그중 네 direct column 입력과 같은 제한 HTML/CSS를 새로운 HostNode ID로 구성해도 opt-in fixture adapter가 source와 computed style로 혼합 입력을 식별하고 fail-closed하는지 확인했다. 제품 CSS admission이나 C11.4 완료를 뜻하지 않는다. [구현·실행·화면 근거](../spec/internal/evidence/c11-4-generic-mixed-text-image-runtime-2026-10-28/README.md).

## 확인된 경계와 목표

`c11_4_mixed::prepare`는 고정 scene adapter를 generic direct-text collector보다 먼저 호출한다. 기존 non-spanning mixed scene은 `SCENES`의 root ID와 각 item ID를 기대값에 직접 묶는다. `generic_direct_text::prepare`는 의미 있는 direct text item이 두 개 미만이면 기존 `TEXT_CASES` fallback을 위해 `Ok(None)`을 반환하고, 두 후보와 direct HTML `<img>`가 함께 있으면 image 혼합을 거부한다. 그 경계를 제거하거나 generic text collector의 성공 조건을 완화하면 안 된다.

이번 slice는 opt-in `RuntimeBlockFlexGridTrackSizingV1` fixture 경로에 별도 content/style 검증기를 추가한다. 유효한 제한 mixed 입력은 새롭고 고정 목록에 없는 HostNode root/item ID를 사용해도 `C11_4MixedInput`으로 준비돼 기존 한 번의 Grid 계산에 들어가야 한다. 혼합 입력 모양이 발견됐지만 약속한 source/style과 맞지 않으면 error를 반환해 text-only 또는 image-only handler로 잘못 흘러가지 않게 한다. 혼합 입력 모양이 없는 root는 `Ok(None)`으로 계속 진행한다.

## 고정 입력과 독립 기대값

기준은 [기존 Chrome 156 observation](../spec/internal/evidence/c11-4-mixed-direct-text-image-intrinsic-precomparison-2026-10-25/README.md) 및 해당 `chrome156-observation.json`이다. 그 HTML은 class 기반 CSS를 사용하고 root/item `id` selector에 의존하지 않는다. source order만 반대인 새 runtime HostNode ID를 만들며, 고정 CSS 및 content는 바꾸지 않는다.

| 장면 | Grid column | direct child source order | 독립 used track 기대값 | root/frame |
|---|---|---|---:|---|
| min forward | `min-content` | text, image | 50px | root 120×48; text (0,0) 50×20; image (0,24) 24×16 |
| min reversed | `min-content` | image, text | 50px | root 120×48; text (0,0) 50×20; image (0,24) 24×16 |
| max forward | `max-content` | text, image | 80px | root 120×48; text (0,0) 80×10; image (0,24) 24×16 |
| max reversed | `max-content` | image, text | 80px | root 120×48; text (0,0) 80×10; image (0,24) 24×16 |

Text 입력은 direct HTML `div`와 한 개의 text node `XX XXXXX`, Ahem 10px/10px, normal whitespace와 word breaking이다. min-content 폭은 `5 × 10 = 50px`, max-content 폭은 `8 × 10 = 80px`이다. 나머지 direct child는 기존 24×16 PNG를 natural size로 표시하는 leaf HTML `img`다. CSS Grid §7.2.1 및 §11.5 step 2에 따라 결과는 `max(50,24)=50px`, `max(80,24)=80px`이며 source order는 어느 기여값도 바꾸지 않는다. Chrome 값은 비교 관찰이고 산술 기대값의 출처가 아니다.

## 입력 허용과 거부 계약

Fixture adapter는 같은 immutable document/style revision에서 아래 조건을 모두 만족할 때만 `Some(C11_4MixedInput)`을 반환한다.

- Root는 fixture profile의 HTML `div` Grid이고 border-box 120×48px, `min-content` 또는 `max-content` column 하나, 24px fixed row 두 개, gap 0, margin/padding/border 0이어야 한다.
- Direct HTML element child는 정확히 두 개다. 하나는 direct text node `XX XXXXX` 하나만 가진 `div`, 다른 하나는 기존 fixture PNG source를 쓰는 leaf `img`다.
- Root의 computed alignment, writing mode와 direction은 oracle과 같아야 한다(`align-items`, `align-content`, `justify-content`, `justify-items`는 `start`; horizontal-tb와 LTR). 두 item은 block, static, border-box이며 margin/padding/border는 0이고 self-alignment는 `auto`다.
- 두 item은 column 1/span 1에 놓이고 text는 row 1, image는 row 2에 놓인다. Text는 Ahem 10px/10px, weight 400, normal style/whitespace/wrapping, horizontal-tb LTR이다. Image는 width/height가 `auto`이고 aspect-ratio override가 없으며 natural size 24×16으로 decode된다.
- 기대 scene, source 종류 또는 결과 선택에 root/item HostNode ID를 사용하지 않는다. Forward와 reversed DOM 순서를 모두 유지한다.

유효하지 않은 HostNode ID를 가진 root에 의미 있는 direct text candidate와 direct HTML image가 함께 있지만 고정 topology, source, revision, profile, track, placement 또는 box-style 조건을 어기면 구체적인 unsupported-input error를 반환한다. text-only 또는 image-only partial input을 만들지 않는다. 기존 고정 mixed scene ID는 기존 adapter가 계속 처리하며 신규 generic adapter가 이를 가로채지 않는다. 기존 one-item legacy fallback과 두 text item + image의 generic collector 거부 동작을 유지한다.

Adapter와 추가 FFI scene route는 `c11-4-grid-track-sizing-fixture-test` feature로 계속 gate한다. Author-document route, product ABI, public API, 일반 image/text 허용, font 확장, CSS 알고리즘 변경은 범위 밖이다.

## 구현과 검증 순서

1. 먼저 계획 PR을 병합한다. 현재 checkout에서 한 단계씩 진행하고 worktree나 병렬 subagent lane은 만들지 않는다.
2. 기존 static mixed-scene ID lookup에서 찾지 못한 경우 `c11_4_mixed::prepare`가 `None`을 반환하기 전에 content/style mixed adapter를 호출한다. 고정 장면은 현재 경로를 유지하고 알 수 없는 HostNode ID는 generic direct-text collector보다 먼저 검사한다. 현재 static mixed scene과 validation은 바꾸지 않는다.
3. 새 root/item ID를 사용하는 네 order/track 장면의 Rust positive test를 추가한다. text 변경, image source 오류/누락, nested/non-leaf source, 추가 child, track/placement/box edge 불일치, profile 불일치, stale snapshot/style revision을 거부하는 negative control도 추가한다. 거부 경로가 partial mixed input을 반환하지 않는지 확인한다. legacy one-item fallback과 두 text item + image 거부가 유지되는 직접 regression test도 보존한다.
4. 서로 다른 route ID와 HostNode ID를 갖는 opt-in FFI runtime route 네 개를 추가하고 Android/iOS strict matcher에 연결한다. source order, node identity, image natural-size input, text intrinsic input, 단일 revision의 축별 trace, track, frame, draw/present marker를 비교한다. 화면 캡처만으로 glyph 또는 PNG pixel 결과를 주장하지 않는다.
5. focused Rust/FFI 검사, formatter, 필요한 feature-specific lint, 기본/fixture artifact 경계 검사를 실행하고 네 scene을 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 실행한다. 정확한 source commit, APK/app bundle hash, log, screenshot을 기록하고 직접 실행한 앱 process만 종료한다.
6. 서로 다른 실패 관점 20개의 새로운 구현/runtime 검토와 별도의 실제 PR diff 30관점 검토를 완료한다. blocker를 수정하면 영향받은 관점을 재검토한다. 가능한 경우 대표 Android/iOS screenshot을 PR 본문에 넣고 Agent Browser로 각 이미지가 열리는지 확인한 뒤 이번에 연 Agent Browser session/process를 종료한다. Terminal Browser는 사용하지 않는다.
7. PR checks와 merge 상태를 다시 확인한다. GitHub check rollup이 비어 있으면 CI 통과로 기록하지 않는다. Rebase and merge한 뒤 실제 merge SHA에 맞춰 `spec/STATUS.md`, 공개 roadmap과 evidence를 조정한다. C11.4 checkbox는 미완료로 두고 공식 완료 수 `72/213`을 유지한다.

## 명시적 제외

이 slice는 임의 text, font, script, image source, source tree, CSS 값, replaced-element sizing override, span, nested/anonymous item, writing mode, row-axis contribution, WPT, 실기기, Android hardware GPU, glyph/PNG pixel equality 또는 각 OS compositor 이후 pixel을 다루지 않는다. 위에 고정한 direct text/image 입력의 identity-independent routing만 fixture에서 확인한다.

## 검토 기록

- 구현 전 서로 다른 실패 관점 20개: [계획 검토](./c11-4-generic-mixed-direct-text-image-intrinsic-plan-review-20.md).
- 계획 PR의 실제 변경 diff 검토 20개: [PR diff 검토](./c11-4-generic-mixed-direct-text-image-intrinsic-plan-pr-diff-review-20.md).
- 최종 구현/runtime 실패 관점 20개: [구현/runtime review](../spec/internal/evidence/c11-4-generic-mixed-text-image-runtime-2026-10-28/implementation-runtime-review-20.md).
- 최종 구현 PR diff 실패 관점 30개: [PR diff review](../spec/internal/evidence/c11-4-generic-mixed-text-image-runtime-2026-10-28/pr-diff-review-30.md).
