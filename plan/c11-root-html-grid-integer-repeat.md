# C11.1 · parsed Root HTML Grid 정수 repeat 제한 runtime

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [Root HTML 기본 Grid 경로](./c11-root-html-grid-runtime.md) · [C11.3 정수 repeat typed profile](./c11-3-template-repeat-named-lines-areas.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md)

**사전 비교:** [기록](../spec/internal/evidence/c11-root-html-grid-integer-repeat-precomparison-2026-11-02/README.md) · [독립 기대값](../spec/internal/evidence/c11-root-html-grid-integer-repeat-precomparison-2026-11-02/expected.json) · [Chrome 관측](../spec/internal/evidence/c11-root-html-grid-integer-repeat-precomparison-2026-11-02/chrome-observation.json)

**계획 실패 관점 검토:** [독립 20개](./c11-root-html-grid-integer-repeat-plan-review-20.md)

## 목표

기본 parsed Root HTML runtime에서 CSS 정수 repeat가 계산된 Grid track 목록까지 전달되도록 한다. 기존 cascade의 bounded integer-repeat expansion과 Root HTML 경로의 finite px/percentage/fr direct-track sizing을 조합한다. 이 변경은 parsed document의 실제 computed values를 사용하며 고정 scene ID나 fixture-specific constructor를 추가하지 않는다.

반복 pattern은 finite non-negative px, definite-axis percentage, finite non-negative fraction direct track으로 제한한다. 한 track list에서 여러 integer repeat group과 한 group 안의 여러 track을 허용하고, 반복 확장 뒤 축별 10,000 explicit-track cap을 그대로 적용한다. 이름 없는 explicit line만 허용하며 line-name metadata가 하나라도 남으면 계산을 거부한다.

## 기준과 독립 기대값

기준은 CSS Grid Layout Module Level 1 2025 CRD의 [repeat() 문법 §7.2.3.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#repeat-syntax)이다. 양의 정수 repeat는 두 번째 인자의 track list를 지정 횟수만큼 반복한다. 고정 HTML 입력은 [precomparison fixture](../tests/fixtures/css/c11/root-html-grid-integer-repeat-precomparison.html)다. 구현 전 profile 대조에서 제외 대상인 CSS `background` shorthand와 alignment 선언을 제거/정규화했고 새 source hash `f2d0d35e88ce89d64ebaac5d37365ad9621735f54f6399e90a1959a7412ccd5d`의 Chrome 156 DOM/CSSOM 관측을 보존했다. background 색과 각 root/item frame은 바뀌지 않았다.

독립 계산:

| 장면 | authored track list | definite axis | 기대 used tracks |
|---|---|---:|---|
| px-columns | repeat(2, 30px 50px) | width 160px | [30, 50, 30, 50] |
| px-rows | repeat(2, 20px 30px) | height 100px | [20, 30, 20, 30] |
| percentage-columns | repeat(2, 25% 25%) | width 200px | [50, 50, 50, 50] |
| fraction-columns | repeat(2, 1fr 2fr) | width 150px | [25, 50, 25, 50] |
| mixed-units | 20px repeat(2, 20% 1fr) | width 200px | [20, 40, 50, 40, 50] |
| multiple-groups | repeat(2, 20px) 10px repeat(2, 30px) | width 110px | [20, 20, 10, 30, 30] |
| single-count | repeat(1, 30px 50px) | width 80px | [30, 50] |

Gap은 0이며 각 item은 해당 track 크기와 고정 20px cross-axis 크기를 가진다. 기본 row auto-placement에서 item frame의 inline/block 시작점은 앞선 used track 크기의 누적값이다. 표 값은 Chrome 결과 복사가 아니라 CSS track expansion과 definite-basis 산술로 먼저 계산하며 browser observation은 독립 교차 확인에만 쓴다.

## 입력 계약과 거부 경계

- 기본 RuntimeRootHtmlTextPaintV1 parsed Root HTML route만 변경한다.
- CSSOM declaration string을 adapter에서 다시 파싱하지 않는다. Stylo cascade가 integer repeat를 typed track vector로 확장하고 현재 projection이 각 expanded track을 기존 direct px/%/fr 경로로 변환해야 한다.
- repeat expansion 뒤 computed style, source order, NodeId provenance, style/layout revision을 보존한다.
- 같은 computed track의 positive control은 root tag, ID, class와 selector 이름을 바꾸고 child source order를 뒤집어 실행한다. Track list는 같아야 하며 item frame은 새로운 source order에 따른다.
- auto-fill/auto-fit, repeat 안의 minmax/fit-content/auto/intrinsic track, named line, named authored line placement, implicit columns, nested Grid, non-row flow, nonzero border/padding/margin/gap, unsupported overflow/writing mode는 성공 scene을 게시하지 않고 기존 contextual error로 닫는다.
- 축별 10,000 expanded-track cap과 1 MiB line metadata quota를 우회하거나 완화하지 않는다. Over-cap track count와 over-budget repeated line metadata는 각각 contextual error를 내며 partial track list나 이전 scene을 발행하지 않는다.
- Existing named-area, fixed-px gap, percentage/fr track, implicit-row profile과 C11.3 integer-repeat fixture route를 별도 회귀 확인한다.

## 구현 순서와 검증

1. Chrome 156 Agent Browser에서 고정 fixture의 authored/computed track list, root/item frame, viewport와 source hash를 저장한다. 기대 track 값은 CSS 산술에서 별도로 도출하고 observation과 대조한다.
2. 계획을 서로 다른 20개 실패 관점으로 검토하고 수정 결과를 이 문서 옆에 기록한다. 이 검토가 끝날 때까지 제품 코드를 바꾸지 않는다.
3. Root HTML cascade profile에서 integer repeat expansion을 켜고 expanded typed track만 기존 Root HTML validator/projection으로 전달한다. Auto-repeat과 intrinsic sizing 허용 flag는 바꾸지 않는다.
4. Rust cascade, style-to-layout, default Root HTML runtime에 positive/negative matrix를 둔다. Positive는 위 7 scene, equivalent direct-track control, renamed selector/NodeId 및 reversed source-order control이다. Negative는 named line, auto-repeat, intrinsic repeat, 10,001 expanded tracks, over-budget repeated line metadata, unsupported placement/style, stale revision, layout/allocation failure를 포함한다. Syntax-invalid repeat 뒤의 valid declaration은 CSS cascade fallback으로 분리한다.
5. Rust와 두 simulator에서 같은 7-grid HTML fixture와 source hash를 사용한다. Track/frame/scene output과 draw/present를 확인하고 screenshots를 보존한다. Surface pixel readback은 실제로 하지 않으면 근거로 주장하지 않는다. Android AVD software path와 iOS compositor 경계를 기록한다.
6. Relevant Rust suites, fixture/profile boundary, Clippy, rustfmt, Android/iOS app builds와 default-versus-fixture archive boundary를 실행한다. PR checks가 비어 있으면 CI 통과라고 쓰지 않는다.
7. 구현/runtime 코드를 별개의 실패 관점 20개로 검토한다. 모든 발견을 수정하고 연관 검토를 다시 실행한다.
8. 구현 PR 최종 diff는 계획·Rust·Swift/Kotlin·증거·공식/공개 roadmap 각각에서 독립 관점으로 검토한다. 프로젝트 PR gate 30개 항목과 user-requested 20회보다 작은 subset으로 대체하지 않는다.
9. 제한 결과를 내부 계약, STATUS, C11 plan, public roadmap에 반영한다. C11.1/C11 및 공식 aggregate 72/213은 미완료로 유지한다.

## 제외 범위

Auto-repeat, arbitrary line names/placement, minmax/fit-content/intrinsic track repeats, all Grid grammar, public CSS support, WPT suite conformance, general dynamic layout combinations, hardware-GPU/physical-device performance, iOS post-compositor pixels, C11.1 or C11 completion.

## 구현 실행·병합 결과

PR #1038로 병합된 구현은 `RuntimeRootHtmlTextPaintV1`의 integer-repeat 확장만 켠다. `allow_intrinsic_tracks`와 `allow_auto_repeat`은 그대로 닫혀 있다. Chrome 156 사전 비교 7/7 scene·35 column/row tracks·28 items가 독립 expected와 일치하고, default Root HTML runtime matrix는 typed tracks·root/item frame·37 scene boxes를 검사한다. Root HTML runtime 30/30, `spinon-style` Root HTML 13/13, `spinon-style-to-layout` Root HTML 15/15, scoped Clippy가 통과했다. 정확히 10,000 tracks, 10,001 거부, 1 MiB metadata quota, allocation failpoint별 복구와 empty named-area track projection rejection도 검사한다. 별도 [구현/runtime 적대적 검토 20개](../spec/internal/evidence/c11-root-html-grid-integer-repeat-runtime-2026-11-02/implementation-runtime-review-20.md)는 계획 검토표를 재사용하지 않는다.

Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator는 동일 fixture hash로 default runtime boot, layout-ready 37 boxes, draw/present를 기록했다. Android는 ANGLE/SwiftShader software renderer다. Simulator screenshots는 visual smoke evidence이며 Android PixelCopy나 Metal surface readback 근거가 아니다. 모든 task fixture-test build flags는 꺼져 있고 `examples/bootstrap/index.html`·`root-app.js`는 원본 hash로 복원했다. [Chrome 및 Rust·Android·iOS logs, hashes, screenshots, 종료·복구 기록](../spec/internal/evidence/c11-root-html-grid-integer-repeat-runtime-2026-11-02/README.md).

구현 PR [#1038](https://github.com/ohah/spinon/pull/1038)은 2026-11-02T06:23:58Z Rebase and merge로 병합됐다(merge SHA `b3ad473d4bfef4bb2f54a61603d498e620c03b0a`). GitHub PR checks는 보고되지 않았다. [병합 근거](../spec/internal/evidence/c11-root-html-grid-integer-repeat-runtime-2026-11-02/merge-PR1038.md) · [병합 후 문서 검토 30개](../spec/internal/evidence/c11-root-html-grid-integer-repeat-runtime-2026-11-02/roadmap-postmerge-docs-pr-diff-review-30.md). C11.1/C11 완료 체크와 공식 `72/213`은 변경하지 않는다.
