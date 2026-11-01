# C11.3.3 negative named-line pair의 leading implicit auto tracks

**상태:** CSS Grid 2025 CRD의 named-line 규칙과 독립 계산을 고정했고 Chrome 156 Agent Browser에서 10개 사전 비교가 모두 일치했다. 제한 구현 commit [`0096e39d`](https://github.com/ohah/spinon/commit/0096e39d59988abb60768129a5cdada0ec077497)을 만들었다. `spinon-layout` 328/328, 대상 style projection/fixture tests, CSS reference 18/18, rustfmt와 대상 Clippy가 통과했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 전용 runtime은 각각 17 frame·8/8 표면 표본 exact이며, 기존 C11.3.3 전체 39-scene·126-node 행렬도 양쪽에서 최대 geometry 오차 0 CSS px다. 기본 Android linked ELF와 opt-in fixture-test positive control의 경계도 확인했다. 구현/runtime 실패 관점 20개와 실제 PR diff 실패 관점 30개 검토를 마쳤다. 구현 PR [#967](https://github.com/ohah/spinon/pull/967)은 2026-11-01T04:44:03Z Rebase and merge됐다(merge SHA `eeda2155354236d3e97954f75217228893ceca21`). GitHub check run은 보고되지 않았다([병합 확인](../spec/internal/evidence/c11-3-3-negative-named-line-pair-runtime-2026-11-01/merge-PR967.md)). 이 작업만으로 C11.3.3·C11 checkbox나 공식 완료 `72/213`을 바꾸지 않는다. [사전 비교](../spec/internal/evidence/c11-3-3-negative-named-line-auto-precomparison-2026-11-01/README.md) · [계획 실패 관점 검토](../spec/internal/evidence/c11-3-3-negative-named-line-auto-precomparison-2026-11-01/plan-review-20.md) · [계획 PR diff 검토](../spec/internal/evidence/c11-3-3-negative-named-line-auto-precomparison-2026-11-01/plan-pr-diff-review-30.md) · [구현·runtime 근거](../spec/internal/evidence/c11-3-3-negative-named-line-pair-runtime-2026-11-01/README.md) · [구현/runtime 실패 관점 20개](../spec/internal/evidence/c11-3-3-negative-named-line-pair-runtime-2026-11-01/implementation-review-20.md) · [실제 PR diff 실패 관점 30개](../spec/internal/evidence/c11-3-3-negative-named-line-pair-runtime-2026-11-01/pr-diff-review-30.md)

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**규범:** [CSS Grid Layout Module Level 1, 2025-03-26 CRD §§7.6, 8.1.3, 8.3.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)

## 현재 결손

[PR #933](https://github.com/ohah/spinon/pull/933)은 영역이 없는 C11.3.3 fixture에서 한 축의 양 끝이 모두 positive trailing implicit named line으로 해석되는 fixed-size item에 `grid-auto-* : auto` 또는 fixed-px auto-track list를 연결했다. 그 계획은 negative occurrence와 named span을 명시적으로 거부한다. C11.3.3 [leading named implicit-line profile](./c11-3-3-leading-named-implicit-lines.md)은 negative occurrence와 leading signed extent를 다루지만 auto/intrinsic implicit sizing을 열지 않고 fixed CSS-pixel track만 허용한다.

현재 projection은 positive named-line pair를 별도 허용 flag로 식별하고, signed extent는 `TemplateGridSignedImplicitTracks` 경로로 보낸다. Layout의 positive pair mode는 `PositiveNamedLineImplicit` track expansion을 사용한다. 따라서 negative named-line pair의 leading extent와 `auto` contribution을 함께 허용하면 안 되는 현재 guard가 유지되는 한 이 조합은 거부되어야 한다. 이번 slice는 기존 positive route와 fixed-px signed route를 그대로 보존하면서 이 조합을 별도 제한 profile로 추가한다.

## 목표와 고정 범위

- `RuntimeBlockFlexGridTemplateV1`의 C11.3.3 opt-in fixture route에서만 허용한다. `grid-template-areas`가 없는 container만 대상으로 한다.
- 한 container에 layout-participating direct grid item 하나만 둔다. item width와 height는 모두 fixed이며, Rows 또는 Columns 중 정확히 한 축의 start/end가 같은 custom-ident와 negative integer occurrence를 가진다. 양 endpoint는 authored matching line이 아니라 모두 leading implicit line으로 해석되어야 한다. 반대 축은 기존 profile의 definite placement여야 한다.
- 지원 track list는 대상 축의 단일 `auto` 값 또는 비어 있지 않은 finite, non-negative fixed-px 목록이다. 비대상 축은 기존 profile의 지원 범위를 유지한다. Leading side 목록은 explicit grid edge에서 start 방향으로 확장하고, 결과 배열은 CSSOM처럼 grid의 start-to-end 순서로 비교한다.
- Chrome 비교 fixture는 negative occurrences `-4/-1`, reversed/equal endpoints, one-track `-2/-1`, offset span `-5/-2`, auto/fixed-px list, row/column 축, 기존 positive trailing control 및 authored-name precedence control을 포함한다. [입력](../spec/internal/evidence/c11-3-3-negative-named-line-auto-precomparison-2026-11-01/input.json)과 [독립 기대값](../spec/internal/evidence/c11-3-3-negative-named-line-auto-precomparison-2026-11-01/expected-values.json)을 기준으로 한다.
- `-4 ghost / -1 ghost`는 이름이 없는 explicit grid 앞에서 네 leading implicit track을 요구하고 endpoint 사이의 item span은 세 track이다. `[5px,7px]` 목록은 explicit grid를 향해 역방향으로 순환한다. auto control의 18px item은 span의 auto columns/rows 세 개에 6px씩 기여하고, span 밖 네 번째 leading auto track은 0px이다. Equal endpoint는 §8.3.1에 따라 end line을 제거하고 남은 definite line의 기본 1-track span을 쓴다. 이는 두 축 모두 automatic인 auto-placement와 구분한다.
- 잘못된 revision, 잘못된 root/item, additional in-flow child, mixed sign, named span, 양 축 implicit extent, auto-sized item, unsupported/non-finite track, overflow와 10,000-track 초과는 새 positive 경로로 묵인하지 않고 frame 생성 전에 거부한다. Authored named occurrence가 있으면 implicit fallback보다 우선한다. 새 mode는 두 endpoint가 실제 leading implicit line으로 해석된 때만 선택하고, 기존 route가 그 precedence 조합을 지원하지 않으면 새 mode로 억지 변환하지 않고 거부한다. Numeric-only implicit placement와 두 축이 모두 automatic인 placement는 거부한다.

## 구현 접근과 검증

1. `project_grid_item_placement`의 sign-aware result, positive pair admission, `requires_signed_extent` 분기와 `expand_tracks`의 `AutoTrackExpansionMode`를 함께 추적한다. `TemplateGridSignedImplicitTracks`의 고정-px 계약을 일반화하지 말고, negative pair의 한 축만 허용하는 별도 `GridPlacementMode` 또는 동등한 typed policy를 사용한다.
2. CSS Grid §8.1.3의 negative named occurrence와 implicit-line fallback, §8.3.1의 endpoint swap/remove 및 end-line removal 뒤의 기본 1-track span, §7.6의 `grid-auto-*` pattern을 각각 독립적으로 검증한다. authored name이 존재하면 implicit fallback보다 우선해야 한다. row/column 축은 같은 비교를 대칭으로 수행한다.
3. Projection·layout-core focused tests에는 positive control, negative occurrence count, reverse/equal endpoints, equal-end의 end-line removal과 기본 span, authored-name precedence, leading list phase, `auto` contribution, fixed-px list, zero/positive/negative mixed endpoints, one/both axis, span, child count, stale snapshot, quota와 failure atomicity를 둔다.
4. Fixture source, runtime matrix와 Android/iOS strict allowlists를 함께 갱신한다. 사전 비교는 negative-leading positive cases와 positive trailing·authored-name controls를 합쳐 10개다. Runtime allowlist에는 이미 같은 양성 대조 scene이 있으면 중복 ID를 추가하지 말고 새 negative-leading cases만 추가한다. 신규 입력과 기존 C11.3.3 area·named-line runtime matrices를 마지막 source에서 Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator로 각각 다시 실행한다. geometry, resolved track order/size, NodeId/source order, frame, draw/present 및 표면 readback의 실제 계층을 구분해 기록한다. AVD software renderer, iOS compositor 제출 전 drawable, 실기기 미검증 한계를 적는다.
5. 기본 product artifact에서 fixture marker/FFI symbol/payload가 없고 opt-in fixture-test positive control에는 있는지 검사한다. 필요한 scoped Rust tests, `cargo fmt --all -- --check`, 관련 Clippy, Android/iOS app build와 app runtime, manifest/hash validation을 기록한다.
6. 구현/runtime 동작은 계획 검토와 다른 20개의 독립 실패 관점으로 검토하고, 구현 PR diff는 프로젝트 규칙에 따라 30개의 별도 관점으로 검토한다. 계획 PR diff도 계획 검토와 분리해 30개 관점으로 검토한다. 결함 수정 뒤 영향을 받는 관점을 다시 확인한다.
7. PR 본문에 실제 Android/iOS simulator 화면을 첨부할 수 있으면 첨부하고 Agent Browser에서 이미지 로드를 확인한다. 브라우저 session을 닫고 `agent-browser` process 종료를 확인한다. 병합 뒤 공식 상태·C11.3.3 plan·공개 Tailnet roadmap을 실행 근거에 맞게 갱신한다.

## 완료 범위와 제외

이 slice는 모든 C11.3.3 named-line fallback, arbitrary numeric implicit placement, named span, mixed positive/negative extent, two-axis implicit sizing, 두 축이 모두 automatic인 placement, intrinsic item dimensions, arbitrary `grid-auto-*` grammar, public CSS admission, WPT 적합성 또는 전체 CSS Grid 지원을 선언하지 않는다. Chrome 156 관찰은 oracle이며 Spinon 구현 결과와 구분한다. C11.3.3, C11, 공식 완료 `72/213`은 이 제한 fixture 구현 뒤에도 미완료다.

## 사전 비교 검토

- [계획 실패 관점 20개](../spec/internal/evidence/c11-3-3-negative-named-line-auto-precomparison-2026-11-01/plan-review-20.md)
- [계획 PR diff 실패 관점 30개](../spec/internal/evidence/c11-3-3-negative-named-line-auto-precomparison-2026-11-01/plan-pr-diff-review-30.md)
