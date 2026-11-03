# C11.5 parsed Root HTML Grid item auto-size and stretch 제한 계획

**상위 계획:** [C11 CSS Grid](./c11-css-grid.md) · [기존 fixed-size self-alignment 구현](./c11-root-html-grid-item-self-alignment.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 제한 구현 PR [#1132](https://github.com/ohah/spinon/pull/1132)은 2026-11-03T20:03:49Z Rebase and merge됐다(merge SHA `8e3d6b4ac3391a1d8e7601a96ab5fe870c825be0`). Android API 37 ARM64 AVD/iPhone 17 Pro iOS 26.2 Simulator 검증 근거는 아래에 연결한다. Rust accepted/negative controls, PixelCopy 11/11, Metal drawable 11/11, 기본 artifact 제외 검사를 기록했다. [구현/runtime 실패 관점 검토 20개](../spec/internal/evidence/c11-5-grid-item-auto-stretch-candidate-2026-11-04/implementation-runtime-review-20.md)와 [PR diff 실패 관점 검토 30개](../spec/internal/evidence/c11-5-grid-item-auto-stretch-candidate-2026-11-04/pr-diff-review-30.md)를 마쳤다. review에서 찾은 content-box admission 회귀를 수정하고 기존 fixed-size route를 두 플랫폼에서 다시 검증했다. PR #1132 check rollup은 비어 있어 CI 성공으로 세지 않는다. 공식 `STATUS.md`에서 C11과 C11.5는 계속 미완료이며 전체 집계 `72/213`을 유지한다.

**계획 실패 관점 검토:** [20개](./c11-root-html-grid-item-auto-stretch-plan-review-20.md)

## 결손과 목표

기존 제한 profile은 고정 폭·높이 Grid item의 self-alignment 위치를 연결했다. 이 후속은 같은 parsed Root HTML 및 fixed-track 경로에서 빈 일반 `div`의 `width`/`height`가 `auto`인 경우와 computed `normal`/`stretch` self-alignment를 다룬다. 목표는 자동 크기 축에서만 cell 크기로 늘어나고, 고정 크기 축은 기존 크기를 유지하며, `auto` self-alignment는 부모의 해당 `align-items`/`justify-items` 결과를 따르는 것이다.

이 작업은 고정 크기 slice에 덧붙는 제한 구현이다. 공개 CSS author admission, 전체 C11.5 또는 C11 완료를 뜻하지 않는다.

## 규범과 profile

- [CSS Box Alignment Level 3](https://www.w3.org/TR/css-align-3/#align-justify-self-property)은 `auto`가 부모의 해당 items 정렬값을 따르며, `stretch`가 해당 축의 computed size가 `auto`일 때 auto margin이 없으면 alignment area를 채우는 used size를 만든다고 정의한다. 이 profile의 margin은 0이다.
- [CSS Grid Layout Level 2](https://www.w3.org/TR/css-grid-2/#grid-item-align)은 preferred aspect ratio가 없는 Grid item에서 `normal`이 stretch처럼 동작하는 규칙을 둔다. 이 profile은 비대체(non-replaced) 빈 `div`만 받아 이 규칙에 해당한다.
- 허용 child 크기는 각 축에서 `auto` 또는 유한한 0 이상 고정 `px`다. `min-width/height:auto`, `max-width/height:none`를 요구한다. Container와 item의 padding/border가 0이므로 `box-sizing:content-box`와 `border-box` 모두 동일한 used geometry를 만든다. Runtime fixture는 두 값을 함께 포함한다.
- item은 기존 eligible static empty `div`, 단일 fixed explicit row/column cell, auto placement, visible overflow, zero margin/padding/border, horizontal-tb/LTR, no aspect ratio, no text/child 조건을 유지한다. Grid container의 definite `40×30px` size, fixed `40px`/`30px` tracks, zero gap 조건도 유지한다.
- 기존 self-alignment allowlist에 computed `normal`과 `stretch`를 더한다. `auto`는 기존처럼 자식의 computed value로 보존하고, 정확한 parent `align-items`/`justify-items` computed value와 함께 해석한다. 기타 값은 기존처럼 node·property·value를 포함해 scene publication 전에 거부한다.
- `normal`/`stretch`는 자동 크기 축에서만 사용 크기를 늘린다. 고정 축은 그대로 유지한다. 한 축만 stretch인 경우 다른 축의 기존 positional alignment와 automatic fit-content sizing을 유지한다.

## 비교 자료와 독립 기대값

[사전 비교 fixture](../tests/fixtures/css/c11/root-html-grid-item-auto-stretch-precomparison.html)는 Agent Browser HeadlessChrome 156에서 10개 scene을 관측했다. [관측 결과](../spec/internal/evidence/c11-5-grid-item-auto-stretch-precomparison-2026-11-04/chrome156-observation.json), [규범에서 독립 산출한 기대값](../spec/internal/evidence/c11-5-grid-item-auto-stretch-precomparison-2026-11-04/expected.json), [실행 조건·hash·제한](../spec/internal/evidence/c11-5-grid-item-auto-stretch-precomparison-2026-11-04/README.md)을 구현 입력으로 보존한다.

10 scene은 양 축 auto+parent normal, 양 축 self normal, 양 축 self stretch, 양 축 positional start, parent start에 self auto, 양 축 fixed+stretch, width-only fixed, height-only fixed, align-only stretch, justify-only stretch를 다룬다.

## 구현 및 검증 순서

1. Existing Root HTML self-alignment projection의 width/height eligibility가 auto와 fixed px만 받도록 좁게 확장한다. 새 CSS profile이나 parser 경로를 추가하지 않는다.
2. `normal`/`stretch` computed value와 parent `align-items`/`justify-items`를 정확한 item style snapshot에서 검증해 Grid item의 typed style로 전달한다. 각 axis의 auto/fixed 조건을 Taffy Grid sizing 결과에서 확인하며 scene ID에 따른 값 선택은 금지한다.
3. Chrome 고정 입력 10 scene을 독립 기대 JSON과 비교한다. 각 scene에서 computed parent/self values, item owner 및 source/style revision, Grid frame, item frame을 검증한다. 기존 fixed-size 19 scene을 회귀 대조군으로 유지한다.
4. Negative controls는 aspect ratio, nonzero item/container margin/padding/border, min/max 변경, non-px size, replaced/content-bearing child, unsupported self value, implicit/auto tracks, multi-cell span을 각각 거부하고 부분 scene을 발행하지 않는지 확인한다. Content-box는 zero-edge positive control로 유지한다.
5. Default/product artifact와 opt-in test route를 분리한다. 기본 artifact에 fixture payload/전용 ABI가 없는지, opt-in artifact에만 존재하는지 기존 경계 검사를 실행한다.
6. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 같은 앱 경로에서 layout geometry와 화면 근거를 수집한다. Android software-renderer/PixelCopy 및 iOS Metal drawable/compositor 경계를 따로 기록하고, 확인하지 않은 GPU·compositor 단계는 주장하지 않는다.
7. 계획 실패 관점 20개, 구현/runtime 실패 관점 20개, 실제 PR diff 실패 관점 30개를 서로 다른 기록으로 작성한다. 모든 수정 후 해당 검토 항목을 다시 대조한다.
8. 구현 병합 후 실제 상태와 공개 roadmap 원본을 갱신하되 C11.5/C11 체크와 `72/213`은 유지한다. Tailnet preview를 갱신할 때는 실제 serve root와 파일 hash를 확인하고 Agent Browser로 검증한 뒤 해당 브라우저 세션을 종료한다.

## 성공 조건

- 10개 scene의 computed alignment values 및 각 축의 used item size/offset이 독립 기대값과 Chrome 156 관측에 일치한다.
- `auto` parent inheritance, `normal` Grid behavior, two-axis and one-axis stretch, fixed-axis preservation, empty fit-content positional controls가 모두 검증된다.
- 각 item frame은 해당 fixed cell 안에 놓이고 경계 밖으로 나가지 않는다. `overflow:visible`은 유지하되 콘텐츠 넘침 동작은 이 빈-item slice의 주장 범위에 넣지 않는다.
- 동일 source/style snapshot, item owner, root identity, revision이 보존되며 거부 입력은 diagnostic을 내고 partial scene을 publish하지 않는다.
- 기존 fixed-size C11.5 19-scene 결과와 앱 runtime slice가 회귀하지 않고 fixture/default artifact 경계가 보존된다.
- 계획·구현/runtime·PR diff 리뷰가 각각 독립 기록으로 남는다.

## 제외 범위

Replaced elements, preferred aspect ratio, content/text, auto margins, nonzero edges, min/max changes, percentage/calc sizing, intrinsic track sizing, implicit/auto tracks, spans, baseline, safe/unsafe overflow behavior, arbitrary CSS admission, WPT/전체 Grid 적합성, real-device/hardware GPU, iOS compositor 이후 pixel 출력은 범위 밖이다. 이 결과는 C11.5 또는 C11 전체 완료를 뜻하지 않는다.
