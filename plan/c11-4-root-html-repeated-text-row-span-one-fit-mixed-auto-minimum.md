# C11.4 · one-fit repeated-text row-span item별 mixed min-height:0|auto

**상태:** 구현 검증 완료 · PR #1381 검토 중 · Chrome 157 및 RuntimeSession 120,960/120,960 scene · C11.4/C11 미완료 · 공식 완료 78/214 유지

**선행 uniform 구현:** [positions 1–3](./c11-4-root-html-repeated-text-row-span-one-fit-auto-minimum.md) · PR #1326 · [positions 4–6](./c11-4-root-html-repeated-text-row-span-positions-4-6-auto-minimum.md) · PR #1331

**인접 mixed 구현:** [three-fit](./c11-4-root-html-repeated-text-row-span-three-fit-mixed-auto-minimum.md) · PR #1378 · [four-fit](./c11-4-root-html-repeated-text-row-span-four-fit-mixed-auto-minimum.md) · PR #1375 · [five-fit](./c11-4-root-html-repeated-text-row-span-five-fit-mixed-auto-minimum.md) · PR #1372 · [six-fit](./c11-4-root-html-repeated-text-row-span-mixed-auto-minimum.md) · PR #1369

**사전 비교:** [Chrome 157 120,960-scene matrix](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-one-fit-mixed-auto-minimum-precomparison-2026-11-14/README.md) · 고정 fixture `tests/fixtures/css/c11/root-html-grid-repeated-text-row-span-one-fit-mixed-auto-minimum-precomparison.html`

**검토:** [계획 실패 관점 20개](./c11-4-root-html-repeated-text-row-span-one-fit-mixed-auto-minimum-plan-review-20.md) · [계획 PR diff 실패 관점 30개](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-one-fit-mixed-auto-minimum-precomparison-2026-11-14/plan-pr-diff-review-30.md)

## 현재 경계와 선택 이유

기존 one-fit row-span 경로는 span text item 전체가 min-height:0인 baseline 또는 전체가 min-height:auto인 candidate를 지원한다. 일부 text item만 auto인 mask는 거부된다. 그런데 CSS Grid automatic minimum은 grid item마다 독립적으로 판정한다. 기존 one-fit track list와 row-span topology 안에서 각 span text item의 computed min-height를 0px|auto로 조합하는 제한 slice를 검증한다.

이번 범위는 기존 six-member grid-auto-rows profile의 one fit-content()와 기존 cap·tail 순열만 사용한다. positions 1–3의 three-row 및 positions 4–6의 six-row topology를 포함해, 기존 one-fit row-span의 여섯 fit 위치를 모두 보존한다. 첫 explicit-row empty control은 계속 min-height:0px다. Two-fit 이상, zero-fit, 새로운 Grid topology, auto-placement, flex/grid item 교차 profile, 새 text grammar 및 arbitrary author CSS는 추가하지 않는다.

## 기준과 비교 결과

기준은 [CSS Grid Layout Module Level 1 · 2025 CRD §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#min-size-auto), [§7.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizing), [§11.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items)다. Item의 scrollable overflow, span track의 automatic minimum, flexible-track 예외와 fit-content minimum floor를 각각 적용한다. min-height:auto item이 모두 eligible일 것이라고 가정하지 않는다.

독립 검증 결과 `spec/internal/evidence/c11-4-root-html-repeated-text-row-span-one-fit-mixed-auto-minimum-precomparison-2026-11-14/verification.json`:

- 120,960 Chrome scene, 900 unique six-member track list, 기존 one-fit positions 1–6, 30 tail order, topology별 4–32개 minimum mask를 확인했다.
- 기존 uniform min-height:0|auto oracle 14,400 endpoint와 대조해 row, item/payload frame 및 text line rectangle 차이가 모두 0 CSS px였다.
- Eligible item 169,344회 중 payload content-floor 위반은 0건이다. Zero baseline에서 geometry가 바뀐 mixed scene은 90,096개다.
- Source-order 60,480쌍과 alignment 60,480쌍을 비교했다. Source-order geometry 차이와 non-auto row stretch 차이는 각각 0 CSS px; auto-row 최대 stretch는 180px다.
- 이 입력 범위에서 Grid content edge 바깥으로 넘친 payload는 0 scene이었다. Pixel fixture는 overflow를 강제로 만들지 않는다.
- 독립 원본과 재현 명령은 [evidence README](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-one-fit-mixed-auto-minimum-precomparison-2026-11-14/README.md)에 있다. 이 결과는 Chrome CSS 동작이며 Spinon 구현 근거가 아니다.

## 고정 지원 profile

- Root HTML RootHtmlFitContentAutoRowsV1, 단일 Grid owner, definite 40×220px Grid, 40px explicit column과 10px explicit row를 유지한다. LTR, sparse row flow, row-gap:0, column-gap:0, visible overflow와 align-content:start|stretch를 사용한다.
- 기존 six-member list fit-content(cap), min-content×2, max-content×2, auto를 유지한다. 기존 one-fit position별 finite non-negative px/direct-percentage cap profile과 30개의 중복 제거 tail 순열만 허용한다. Cap 사전 표본은 지원값 allowlist가 아니다.
- Positions 1–3은 기존 one-fit three-row의 두 span-2와 span-2/span-3 source-order topology를 쓴다. Positions 4–6은 기존 six-row의 다섯 span-2 item 및 span-2/span-3 topology를 쓴다. 각 topology의 forward/reverse order를 유지한다.
- 첫 explicit-row control은 empty direct child이며 min-height:0px여야 한다. 다른 direct text item은 기존 fixed 10px monospace text payload 하나만 가질 수 있다. 각 item은 min-height:0px 또는 auto다.
- Minimum mask는 semantic item ID에 고정한다. Authored DOM 순서 변경이 mask 소유권을 바꾸지 않는다. Runtime admission은 정확히 one-fit row-span track-list predicate와 위 topology에서만 활성화한다.
- Scrollable overflow, flexible track span, unsupported Grid owner/revision, line placement·span·child·style·cap 변형은 계속 fail-closed한다. Public API/FFI는 추가하지 않고 internal contract 0.1.0을 유지한다.

## 구현·검증 계획

1. Style projection과 Layout에 one-fit mixed-mask 전용 predicate를 연결한다. Explicit control의 0px를 강제하고 각 span item의 0px|auto만 보존한다. Existing all-zero/all-auto endpoint를 포함하며 두-fit, 세-fit 이상 profile predicate를 넓히지 않는다.
2. 기본 Root HTML RuntimeSession에서 이 계획의 120,960 Chrome scene을 모두 실행한다. 같은 fixture/revision에서 computed track order, per-item computed minimum, eligibility, track trace, row/span/payload frame, visible overflow, source order와 alignment를 대조한다. Browser observations와 mask IDs는 immutable fixture hash로 결합한다.
3. Positive controls는 기존 uniform one-fit positions 1–3 및 4–6 full matrices다. Negative/recovery controls는 control auto, span item의 min-height:min-content·nonzero length, unsupported cap·calc(), zero/two-fit, list/tail·span·flow·alignment·payload 변형, extra Grid owner, missing/stale/reordered/duplicate trace를 포함한다. Reject 전후 published revision/frame 무변경과 same-session recovery를 확인한다.
4. 기존 one-fit row-span uniform 5,760 및 8,640 scenes, two-fit row-span 3,840 scenes, one-/two-fit auto-placement, repeated-text 810 corpus, Grid track-sizing/projection 회귀를 보존한다. Focused Rust tests, 변경 crate scoped Clippy -D warnings, cargo fmt --all -- --check를 실행하고 base에서 확인되는 실패는 candidate 실패와 분리 기록한다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 동일 source revision의 Root HTML route를 실행한다. Chrome 사전 비교의 mixed item mask를 고정한 화면, layout-ready, draw/present와 앱 입력 hash를 기록한다. 화면은 visual smoke이며 fixture matrix 전체, PixelCopy/Metal drawable readback, compositor parity, 실기기 또는 hardware GPU를 뜻하지 않는다.
6. 한정 구현 뒤 별도 실패 관점 20개 implementation/runtime review와 실제 변경 PR diff의 서로 다른 실패 관점 30개를 기록한다. PR 본문에는 범위·측정 분모·남은 검증 경계와 raw Android/iOS screenshot을 commit-pinned 링크로 넣고 Agent Browser에서 실제 로드를 확인한다.
7. 구현 PR이 병합된 뒤 spec/STATUS.md, 이 계획, 내부 계약 0059와 공개 roadmap을 같은 상태로 갱신한다. 공식 78/214, C11.4/C11 unchecked, internal contract 0.1.0을 유지한다. 사용자가 정한 Tailscale preview만 선택 경로 백업·hash 검증과 Agent Browser desktop/mobile 확인으로 갱신한다.

## 2026-11-14 구현 실행 결과 · PR 준비 단계

- 기본 Root HTML RuntimeSession matrix: 120,960/120,960 scenes, unique track-list batch 900/900, eligible automatic-minimum item occurrence 169,344; 실행 2,633.00초.
- 별도 회귀: one-fit uniform positions 1–3 2,880 pairs, positions 4–6 4,320 pairs, two-fit row-span 3,840 scenes, one-fit auto-placement 1,440 scenes, two-fit auto-placement 및 repeated-text 810 corpus 통과. 관련 rejection/recovery와 invalid-trace controls도 통과했다.
- 수정 발견: empty trace owner index의 panic을 fail-closed 오류로 바꾸고 layout regression 56/56을 재실행했다. five-fit position helper 테스트의 오래된 기대값도 고쳤고 projection 관련 5/5 통과했다. 전체 matrix의 한 장면이 5초 제한에서 Pending으로 반환되어 wait를 30초로 늘린 뒤 전체 matrix를 다시 통과했다.
- Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 입력 SHA-256 `a944c85929ac16065cd950d4edc58788f8eb98ab2759644aa6ceb23874b57676`를 사용한 app light/dark 화면 smoke를 완료했다. 모두 `layout=ready`, 12 boxes, `draw status=0 presented`를 기록했다. Android는 Vulkan llvmpipe 소프트웨어 renderer다. 화면은 pixel parity 검증이 아니다. [플랫폼 화면·정리 근거](../spec/internal/evidence/c11-4-root-html-repeated-text-row-span-one-fit-mixed-auto-minimum-runtime-2026-11-14/platform-visual-smoke.md).
- 구현/runtime 20개와 실제 PR #1381 diff 30개 실패 관점 검토를 작성했다. PR은 draft이며 GitHub check/workflow 상태 확인, 병합과 최종 Tailnet roadmap 동기화가 남아 있다. 공식 `78/214`, C11.4/C11 미완료 및 내부 계약 `0.1.0`은 유지한다.

## 완료 판정

Chrome precomparison alone은 지원 완료가 아니다. Product code는 전체 mixed-mask RuntimeSession matrix와 atomic rejection/recovery, endpoint·기존 profile 회귀, focused lint/format, Android/iOS simulator 실행 근거, implementation/runtime review 20개, 실제 PR diff review 30개, 명세·상태·roadmap sync와 Rebase and merge를 확인한 뒤에만 이 bounded slice로 기록한다. 이 범위는 일반 CSS Grid, WPT 전체 적합성 또는 C11.4/C11 완료로 승격하지 않는다.
