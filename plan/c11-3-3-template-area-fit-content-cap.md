# C11.3.3 area-backed `fit-content()` cap와 text intrinsic 통합

**상태:** 계획 단계. Chrome 156 사전 비교만 완료했다. Spinon runtime, Android/iOS Simulator, 구현/runtime 검토는 아직 하지 않았다. C11.3.3 candidate, C11, 공식 완료 `72/213`은 미완료로 유지한다.

## 결손과 목표

PR #911은 area가 생성한 implicit track에서 기존 C11.4 typed `grid-auto-*` subset을 허용했다. 그러나 이전 `fit-content(40px)` 입력은 content의 min-content가 cap보다 클 수 있어 cap 효과를 분리하지 못한다. 이 계획은 area로 생성한 column에 Ahem wrapping text를 넣어 `min-content < 40px < max-content`를 고정하고, `fit-content(40px)`의 실제 used track과 item frame을 비교한다.

기존 구현 경계가 분리되어 있다. C11.3.3 `RuntimeBlockFlexGridTemplateV1`은 area dimension이 explicit track을 초과하면 auto track을 생성한다. C11.4의 text intrinsic fixture 경로는 `RuntimeBlockFlexGridTrackSizingV1`에서 `TextLayoutEngine`을 연결하며, 이 profile은 area dimension과 explicit track 개수가 일치해야 한다. 구현 단계에서는 두 동작을 단일 고정 scene에 한해서 fixture 전용 경로로 연결한다. 일반 author CSS admission이나 C11.4/C11.3.3 profile 전체를 넓히지 않는다.

## 비교 기준과 고정 입력

- 규범: [CSS Grid Layout Module Level 1, 2025-03-26 CRD §§7.3, 7.5–7.6, 11.4–11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/). `fit-content()` track sizing의 `max(auto minimum, min(limit, max-content))`와 implicit track sizing을 적용한다.
- 독립 oracle: [Chrome 156 Agent Browser 사전 비교](../spec/internal/evidence/c11-3-3-template-area-fit-content-cap-precomparison-2026-10-31/chrome/README.md), [fixture](../spec/internal/evidence/c11-3-3-template-area-fit-content-cap-precomparison-2026-10-31/grid-template-area-fit-content-cap.html), [기대값](../spec/internal/evidence/c11-3-3-template-area-fit-content-cap-precomparison-2026-10-31/expected-values.json), [관찰 JSON](../spec/internal/evidence/c11-3-3-template-area-fit-content-cap-precomparison-2026-10-31/chrome/observation.json), [화면](../spec/internal/evidence/c11-3-3-template-area-fit-content-cap-precomparison-2026-10-31/chrome/chrome-156-agent-browser.png). Chrome `156.0.8078.76`, Agent Browser `0.31.1`, 1280×633 CSS px, DPR 1이다.
- Ahem 10px/10px text `AA AA AA AA`의 independent min-content probe는 20px이고 max-content probe는 110px다. 20px explicit first column, `grid-template-areas: "fixed fit"`, `grid-auto-columns: fit-content(40px)`, definite 200×60px root, zero gap에서 Chrome은 columns `20px 40px`, area item frame `[20, 0, 40, 60]`을 반환했다. 계산은 `max(20, min(40, 110)) = 40px`이다.
- 시작 전에 fixture/font/screenshot SHA-256과 `document.fonts.ready`·`document.fonts.check("10px Ahem")`를 확인했다. Chrome 값은 oracle이며 Spinon 동작이나 WPT 적합성이 아니다.

## 구현과 검증 계획

1. 기존 C11.3.3 39-scene runtime matrix를 변경하기 전에, 동일한 profile split을 보존하는 고정 `fit-content-area-cap` fixture route를 추가한다. root, fixed-area item, fit-area text item의 node identity와 `createTextNode` 내용이 실행 source까지 보존되는지 확인한다.
2. Fixture 전용 adapter는 `RuntimeBlockFlexGridTemplateV1`의 area expansion과 기존 Ahem `TextLayoutEngine` 경로를 결합한다. feature-off 기본 product 경로의 CSS admission/API는 바꾸지 않는다. text item만 paragraph owner로 등록하고 다른 node에 text shaping을 임의로 열지 않는다.
3. 적용된 scene의 typed area-created column trace가 40px이고 min/max text intrinsic 값이 20px/110px인지 Rust assertion으로 확인한다. root와 item frame은 각각 `[0,0,200,60]`, `[20,0,40,60]`이어야 한다. item이나 area 생성이 누락되거나 text source가 사라지면 성공 대신 명시적인 실패를 낸다.
4. Chrome fixture와 runtime source 입력을 묶은 고정 reference를 추가한다. fit item 내부 경계 표본 `(59,30)`은 초록, track 바깥 `(60,30)`은 root 배경이어야 한다. 좌표는 CSS px/DPR 1로 지정하고 native 캡처 origin·scale 변환을 각 플랫폼 로그에 기록한다.
5. 기존 C11.3.3 focused Rust/fixture 검사를 재실행한다. 새 scene은 Android API 37 ARM64 AVD/SwiftShader와 iPhone 17 Pro/iOS 26.2 Simulator에서 최신 source로 실행한다. 전체 39-scene 회귀를 유지하며, 새 scene의 frame과 적용 가능한 PixelCopy/Metal drawable 표본을 별도로 보고한다. Metal drawable 결과는 compositor 제출 전으로 표시한다.
6. `c04-runtime-gpu` 기본 Release artifact와 opt-in fixture positive control을 대조해 새 scene marker/fixture ABI가 product archive에 들어가지 않고 fixture-test archive에는 들어가는지 확인한다. 이 archive 대조를 APK/IPA 전체 exclusion 근거로 과장하지 않는다.
7. 기능 변경 뒤 새 실패 관점 20개 구현/runtime 리뷰를 하고, 실제 PR diff는 별도의 30개 관점으로 검토한다. 계획 PR은 구현 전에 계획 자체 20개 관점과 계획 PR diff 30개 검토를 마친 뒤 Rebase and merge한다.
8. 이 제한 scene만으로 C11.3.3, C11.4, C11 또는 `72/213`을 체크하지 않는다. WPT 전체, 실기기, Android hardware GPU, iOS compositor 이후 pixel, 일반 CSS author admission은 미검증으로 남긴다.

이 계획의 실패 관점 검토는 [계획 검토 20개](../spec/internal/evidence/c11-3-3-template-area-fit-content-cap-precomparison-2026-10-31/plan-review-20.md), 계획 PR의 실제 변경 검토는 [계획 PR diff 검토 30개](../spec/internal/evidence/c11-3-3-template-area-fit-content-cap-precomparison-2026-10-31/plan-pr-diff-review-30.md)에 기록한다.

## 완료 조건

- 고정 Chrome 입력과 Spinon fixture의 typed area-created track, intrinsic contributions, item frame이 위 CSS-pixel 기대값과 일치한다.
- 기본 C11.3.3 39-scene regression과 Android/iOS Simulator 실행이 통과하고, 내부 surface 경계 sample은 각 플랫폼에서 색상 기대값과 일치한다.
- 잘못된 scene ID와 feature-off product archive가 새 fixture ABI/source에 접근하지 못하고 opt-in fixture positive control은 해당 scene을 포함한다.
- 별도 구현/runtime 20관점 검토, PR diff 30관점 검토, 증거·한계·계획 문서가 같은 범위를 가리킨다.
- 공식 상태 대장과 공개 지원 상태는 그대로 유지한다.

## 이 계획에서 완료로 주장하지 않는 것

Chrome 156 결과는 독립 비교 기준일 뿐이다. 제한 fixture 1개의 pass는 전체 `fit-content()`·CSS Grid·WPT 지원이 아니다. AVD는 실기기가 아니며 SwiftShader software renderer를 사용한다. iOS Metal drawable readback은 compositor 제출 전이다.
