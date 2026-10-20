# C11.4 직접 텍스트 다중 span 기여 계획

**상태:** Chrome 155 사전 비교와 계획 실패 관점 20개 검토를 마쳤고 계획 PR #407을 머지했다. 직접 텍스트 4개 장면의 Rust·Android 실기기·Android API 37 AVD·iOS 26.2 Simulator 검증을 마쳤다. 사용자가 연결한 Android 실기기에서 최신 fixture APK로 4/4를 재실행했다([실행 결과](../spec/internal/evidence/c11-4-text-multispan-contributions-2026-10-21/android-device-c114-text-multispan-contributions-android-device-20261020T215102621Z-daee998f/result.json)). 구현/runtime 실패 관점 20개, PR #408 diff 적대 검토 30개, fixture artifact 경계 확인을 마쳤다. 구현 PR [#408](https://github.com/ohah/spinon/pull/408)은 rebase merge SHA `4b7282116c0aeab12ba10edeae51fcf7f7bb3a33`으로 병합됐다. PR 화면과 `gh pr checks --required`에는 check 결과가 없어 CI 통과로 표기하지 않는다. C11.4 체크와 공식 완료 수는 그대로 둔다.

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

## 목적과 범위

현재 C11.4 text fixture adapter는 하나의 Grid root, 하나의 직접 자식 `div`, 그 안의 직접 text node 하나만 허용한다. 따라서 text intrinsic input을 여러 grid item에 연결하면서 `span=2` item끼리 겹치거나 `span=2` 뒤 `span=3`이 오는 경우를 확인할 수 없다. 기존 multi-span 검증은 고정 크기 box contribution만 다루므로 text min-content 측정과 span stage를 함께 검증하는 작은 fixture가 필요하다.

이번 구현은 opt-in `RuntimeBlockFlexGridTrackSizingV1` fixture에서 고정 Ahem direct-leaf text item 2~3개를 한 Grid에 연결한다. 각 item은 root의 직접 자식 `div` 하나와 그 안의 non-empty text node 하나로 구성한다. 4개 `min-content` column, 3개 고정 20px row, 0 gap을 유지하고 네 scene으로 source order와 span 순서를 대조한다.

이번 범위에 포함하지 않는 입력:

- Grid root의 anonymous text item, nested wrapper, nested Grid, inline element descendant
- 한 item의 두 개 이상 text node, 여러 문단, `<br>`, whitespace-only item
- Ahem 외 font, script, writing mode, line-breaking mode, dynamic font/image update
- 다른 track 함수, gap, cyclic percentage, auto-repeat 조합, WPT와 전체 CSS Grid 지원

입력 구조와 문자열은 fixture allowlist로 고정한다. 범위 밖 text topology나 scene ID는 partial layout으로 전환하지 않고 기존 오류 경계에서 거부한다. 구현은 fixture-test feature 안에만 둔다. 내부 계약 `0059`와 공개 CSS 지원 범위는 바꾸지 않는다.

## Chrome 사전 oracle과 독립 기대값

사전 fixture는 [grid-track-sizing-c114-text-multispan-precomparison.html](../tests/fixtures/css/c11/grid-track-sizing-c114-text-multispan-precomparison.html)이다. Agent Browser가 읽은 Chrome `155.0.8059.85`, macOS `26.5.1` ARM64, DPR 1에서 Ahem font load를 기다린 뒤 track과 모든 item frame을 저장했다. 원본 수치와 환경은 [Chrome observation](../spec/internal/evidence/c11-4-text-multispan-precomparison-2026-10-21/chrome155-observation.json), 화면은 [precomparison screenshot](../spec/internal/evidence/c11-4-text-multispan-precomparison-2026-10-21/chrome155-precomparison.png)에 보존했다.

CSS Grid §11.5.1의 spanning-item 계산 규칙에서 독립 기대값을 도출했다. Ahem 10px의 고정 입력에서 `XX XXXXX`, `XXXX XXXXXXX`, 18개의 `X`는 각각 min-content 폭 50, 70, 180 CSS px다. 겹치는 두 span-2 item은 track별 계획 증가를 적용한 뒤 `[25, 35, 35, 0]`을 만든다. 그 상태에서 span-3 item의 180px 기여는 앞 세 track의 현재 합 95px을 85px 늘려 이상값 `[53⅓, 63⅓, 63⅓, 0]`이 된다. DOM source order를 뒤집어도 기대 track은 같다.

Chrome 155의 used-track 문자열은 같은 네 장면에서 `[25,35,35,0]` 또는 `[53.328125,63.328125,63.34375,0]`이었다. 두 번째 값은 각 ideal track과 1/64 CSS px 이내다. Chrome 관찰은 독립 기대값을 대신하지 않으며, Chrome의 CSSOM 직렬화와 runtime `f32` 좌표는 수치 비교로 분리한다.

| Scene | Span 구성 | 독립 기대 Columns (CSS px) | Chrome used Columns (CSS px) |
| --- | --- | --- | --- |
| `text-span2-forward` | 두 개 span-2, forward DOM | `25, 35, 35, 0` | `25, 35, 35, 0` |
| `text-span2-reversed` | 같은 입력, reverse DOM | `25, 35, 35, 0` | `25, 35, 35, 0` |
| `text-span2-before-span3` | span-2 두 개 뒤 span-3 | `53⅓, 63⅓, 63⅓, 0` | `53.328125, 63.328125, 63.34375, 0` |
| `text-span3-before-span2` | 같은 입력, span-3 우선 DOM | `53⅓, 63⅓, 63⅓, 0` | `53.328125, 63.328125, 63.34375, 0` |

규범 기준은 [CSS Grid Layout Level 1 §11.5.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-spanning-items)이다. Chrome 관찰은 이 네 fixed input과 Ahem font의 비교 자료다.

## 구현 절차

1. 계획 PR을 `Rebase and merge`한 뒤 최신 `main`에서 구현 branch를 만든다. 별도 worktree를 만들지 않는다.
2. fixture-test 경계에 multi-text scene router를 추가한다. 각 root, child order, item ID, text string, explicit line placement, Ahem style과 font revision을 exact allowlist로 확인한다.
3. 각 direct item node ID에 paragraph style을 각각 연결하고, 모든 item에서 text intrinsic input이 만들어지는지 검사한다. 기존 단일-item text 및 auto-repeat text fixture의 입력·실패 경계를 회귀 검사한다.
4. Rust oracle에서 각 item의 min/max-content 원본 측정값과 span을 확인하고, min-content track이 요청하는 `Minimum`·`MinContent` contribution, 처리 단계 `[2]` 또는 `[2, 3]`, 최종 track, 모든 root-relative item frame을 검사한다. 이 트랙 정의에서는 각 item의 `Minimum` contribution이 자동 최소 크기 규칙에 따라 0이며 `MaxContent`는 원본 측정 입력에 보존되지만 track pass가 별도 질의하지 않는다.
5. Android/iOS app fixture에 네 scene을 추가한다. Android validator는 finite 수치와 1/64 CSS px 허용치, exact node ID/order, 모든 trace chunk 및 draw/present를 검사한다. iOS Simulator도 같은 scene/frame/track/stage/draw 기준을 사용한다.
6. 기본 product artifact에 새 scene ID/fixture symbol/text가 빠지고 opt-in fixture artifact에는 포함되는지 확인한다. `cargo fmt`, focused Rust tests, workspace Clippy와 fixture app builds를 실행한다.
7. Android API 37 ARM64 AVD, 현재 연결된 Samsung SM-S731N Android 16/API 36, iPhone 17 Pro iOS 26.2 Simulator에서 4/4 scene을 실행해 Chrome expectation과 대조한다. Android physical 기기는 serial을 명시하고 설치 APK hash, model, API, ABI, renderer, run ID를 기록한다.
8. 실제 코드/runtime을 새 실패 관점 20개로 검토하고 구현 PR diff를 별도 30개 관점으로 검토한다. 필요한 CI와 review를 통과한 뒤 `Rebase and merge`한다.

## 완료 판정과 한계

- Chrome precomparison 네 scene의 font-ready, intrinsic widths, used tracks와 frames가 이 문서의 입력에 고정돼야 한다.
- Rust 및 app harness 네 scene 모두에서 contribution item/span mapping, stage sequence, tracks와 root-relative frames를 확인해야 한다.
- Android API 37 AVD, Android 실기기, iOS Simulator에서 각각 4/4 scene이 통과해야 한다. OS compositor 이후 화면, iOS 실기기, glyph paint는 이 slice의 완료 조건이 아니다.
- 기본/fixture artifact 경계, 20개 구현/runtime review, 30개 PR diff review와 현재 required CI 상태를 분리해 기록한다.
- 이 제한 fixture 완료는 C11.4 전체, 일반 font/script, nested/anonymous item, 전체 intrinsic stage, WPT 또는 공개 CSS Grid 지원을 의미하지 않는다. STATUS checkbox와 공식 완료 수를 올리지 않는다.

## 현재 실행 근거

- [Android 실기기·API 37 AVD·iOS Simulator 실행, 화면, 빌드·검사 결과](../spec/internal/evidence/c11-4-text-multispan-contributions-2026-10-21/README.md)
- [구현/runtime 적대 검토 20개](../spec/internal/evidence/c11-4-text-multispan-contributions-2026-10-21/implementation-review-20.md)
- [기본/fixture artifact 경계](../spec/internal/evidence/c11-4-text-multispan-contributions-2026-10-21/artifact-boundary.md)
- PR #408 diff 30개 검토와 병합은 완료했다. 근거는 [PR diff 검토](../spec/internal/evidence/c11-4-text-multispan-contributions-2026-10-21/pr-diff-review-30.md)에 있다. PR에 check 결과가 없어 CI 통과 여부를 주장하지 않는다. C11.4 checkbox와 공식 완료 수는 그대로다.
