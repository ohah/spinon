# C11.4 · auto-repeat named-line implicit fallback 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [auto-repeat authored line names](./c11-4-auto-repeat-named-lines.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 구현 전 계획. C11.4 미완료 checkbox와 공식 `77/213`을 유지한다.

## 결손과 구현 경계

PR [#530](https://github.com/ohah/spinon/pull/530)은 authored auto-repeat line names를 보존하고 반복 수 계산 후 Taffy의 named-line resolver까지 넘긴다. 그 다음에도 style projection [`named_line.rs`](../crates/spinon-style-to-layout/src/projection/style_values/grid_placement_values/named_line.rs)와 layout의 [`validate_auto_repeat_placement`](../crates/spinon-layout/src/grid_auto_placement/dense_column.rs)는 name이 outer/repeated authored metadata에 없으면 거부한다. 따라서 `ghost` 같은 이름이 없거나 `cell` occurrence가 부족한 경우 Taffy의 기존 implicit-line fallback에 도달하지 못한다.

CSS Grid 2025 CRD §8.3은 integer named line에서 이름이 붙은 explicit line 수가 부족하면 implicit grid lines를 해당 이름으로 간주한다. Named span은 search direction에 놓이는 implicit line을 같은 방식으로 센다. 구현은 이 제한 auto-repeat axis에서 membership guard만 제거하고, 기존 integer/span 범위, zero rejection, paired-span conflict, auto-repeat profile 및 allocation 오류 계약은 유지하는 방향으로 검토한다. Repeat count나 line lookup을 Spinon에서 다시 구현하지 않는다.

## 구현 범위

- `auto-fill` 한 개, 고정 `40px` repeated track, 고정 `20px` implicit track, gap 0으로 만든 existing auto-repeat profile 안에서만 지원한다.
- authored name 부재와 occurrence 부족을 구분해 고정 fixture에서 검증한다. Positive/negative integer named line, forward/backward named span을 포함한다.
- row axis에서 positive/negative absent-name control을 두어 placement validation이 축 공통 경로인지 확인한다.
- symbolic `NamedLine`/`SpanNamed`를 auto-repeat expanded line metadata에 보존해 Taffy가 계산하도록 한다. Bridge가 새 이름 문자열을 복사하는 경우 기존 fallible allocation 경계를 유지한다.
- parent `spec/STATUS.md`의 C11.4 ID는 완료로 바꾸지 않는다. 이 제한 fixture에서 관찰한 런타임 동작을 공개 CSS 지원으로 넓혀 해석하지 않는다.

## 기준과 고정 입력

[Chrome 156 사전 비교](../spec/internal/evidence/c11-4-auto-repeat-named-line-implicit-fallback-precomparison-2026-10-23/README.md)는 W3C 규칙에 따른 독립 기대값, raw Agent Browser 관측, screenshot, fixture/hash를 보존한다. Acceptance scene은 10개다. 각 scene의 track list/group과 repeat count는 exact, item frame은 최대 `1/64 CSS px` 오차로 비교한다. Track list 또는 frame 중 하나만 맞으면 scene은 실패다.

## 구현 후 판정과 검증

1. 계획 PR 병합 후 최신 `main`에서 구현 branch를 만든다. 기존 미추적 증거·tmux와 이미 booted 된 device/simulator는 보존한다.
2. Cascade, `project_axis_for_auto_repeat`, layout validator, `GridPlacementInput`, Taffy style projection, `NamedLineResolverAxis`를 다시 추적하고 두 membership gate만 변경하는지 diff로 확인한다.
3. 10개의 precomparison scene을 Rust/layout fixture에 연결한다. Name absent/insufficient, positive/negative occurrence, forward/backward span, row-axis, fixed implicit track sizing, line/group boundary merging, numeric placement, bare authored name을 각각 회귀 control로 둔다. Invalid zero/out-of-range placement와 두 끝 span 충돌은 기대 동작을 확인한다.
4. Rust focused tests 및 해당 workspace checks를 실행하고 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 최신 빌드에서 각 10/10 scene을 확인한다. Android PixelCopy와 iOS compositor 제출 전 Metal 표본은 explicit border/item edge와 neighboring track 좌표를 고정하고, 실제로 읽은 pixel 위치에 대해서만 exact를 주장한다.
5. 설치 APK/Simulator app은 해당 빌드 SHA와 일치해야 한다. 기본/product와 fixture-test artifact 경계는 각각 negative/positive control로 검사한다. AVD와 iOS Simulator 결과는 실기기, hardware GPU, WPT, OS compositor 이후 pixel parity를 증명하지 않는다.
6. 구현/runtime의 새 실패 관점 20개와 PR diff의 독립 실패 관점 20개를 별도로 검토한다. 발견 오류를 고친 뒤 영향을 받은 관점을 재검토하고 실제 수행한 검사만 기록한다.
7. PR은 작업 유형/영역 라벨, 상태 ID, 명세·oracle·runtime 근거, 남은 한계를 채운 양식에 맞춰 작성한다. Tailnet preview를 sync/검증한 다음 Agent Browser로 동작 화면을 확인하고 Browser session/process를 닫는다. GitHub merge state와 live required checks를 확인하고 Rebase and merge를 사용한다.

## 제외 범위

`auto-fit` collapse와 fallback의 조합, 복수 auto-repeat, areas와 named line의 우선순위, subgrid, writing mode, 다른 track sizing grammar, 임의 CSS 입력, WPT, 실기기 및 OS compositor 이후 pixels는 후속이다. 이 계획이나 구현 slice만으로 C11.4 및 전체 CSS Grid 지원을 완료로 표시하지 않는다.

## 계획 검토와 근거

- [계획 실패 관점 검토 20개](./c11-4-auto-repeat-named-line-implicit-fallback-plan-review-20.md)
- [Chrome 156 사전 비교·고정 입력](../spec/internal/evidence/c11-4-auto-repeat-named-line-implicit-fallback-precomparison-2026-10-23/README.md)
- [계획 PR diff 검토 20개](../spec/internal/evidence/c11-4-auto-repeat-named-line-implicit-fallback-precomparison-2026-10-23/plan-pr-diff-review-20.md)
