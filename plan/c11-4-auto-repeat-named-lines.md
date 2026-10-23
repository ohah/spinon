# C11.4 · auto-repeat line name 보존과 배치 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** 구현 및 양 simulator 고정 scene 검증 완료. 구현 PR [#530](https://github.com/ohah/spinon/pull/530)은 2026-10-22 21:14:42 UTC Rebase and merge로 병합되어 `776810ad5b4bc097d3d18c1871161471fed07974`를 남겼다. 이 제한 slice를 병합해도 C11.4 공식 완료 수 `72/213`과 미완료 checkbox는 그대로 둔다. auto-repeat fragment의 authored line name 보존과 확장 후 named placement 해석만 다룬다.

## 결손과 실행 가능성

현재 cascade의 `computed_auto_repeat_tracks()`는 track list 바깥 line-name group과 반복 내부 `repeat.line_names`가 비어 있지 않으면 값을 거부한다. style-to-layout projection과 `spinon-layout::validate_auto_repeat_axis()`도 반복 축의 모든 line name을 거부한다. named placement는 auto-repeat count가 계산되기 전에 authored track count를 기준으로 숫자로 바꾸므로, 이를 그대로 통과시키면 반복 line index가 틀릴 수 있다.

고정된 vendored Taffy `0.15.0`은 `GridTemplateRepetition.line_names`, `GridPlacement::NamedLine`/`NamedSpan`, auto-repeat count 계산 후 `NamedLineResolver` 생성 경로를 이미 갖고 있다. 구현은 반복 수 계산을 복제하지 않고, 한정 line metadata와 symbolic placement를 Taffy에 전달하는 bridge를 우선 검토한다. Taffy의 확장 순서·경계 병합과 Spinon bridge allocation/identity 경계는 구현 단계의 runtime review에서 다시 확인한다.

기준값은 [Chrome 156 사전 비교](../spec/internal/evidence/c11-4-auto-repeat-named-lines-precomparison-2026-10-23/README.md)와 고정 [fixture](../spec/internal/evidence/c11-4-auto-repeat-named-lines-precomparison-2026-10-23/grid-auto-repeat-named-lines.html)에서 정의한다. 기대값은 CSS Grid 2025 CRD 규칙과 track/gutter 산술로 독립 계산했고 Chrome은 별도 관찰자로 기록했다.

## 구현 범위

- 기존 `RuntimeBlockFlexGridTrackSizingV1` auto-repeat subset을 유지한다. 축별 한 개의 `auto-fill` 또는 `auto-fit`, 기존 한 개 또는 두 개의 fixed-minimum repeated track, 기존 fixed-`px` prefix/suffix 조건만 통과시킨다.
- cascade snapshot은 repeated fragment의 `N+1` line-name group과 top-level component 경계 line-name group을 구조대로 보존한다. 비-auto-repeat track list의 기존 `N+1` line-name 의미는 바꾸지 않는다. allocation 실패와 잘못된 group 수는 구체적 오류로 닫는다.
- auto-repeat 축의 named start/end 및 named span을 반복 수를 아는 Taffy 단계까지 보존한다. 일반(non-repeat) 축은 기존 projector의 이름 해석을 유지한다. auto-repeat이 없는 container/profile은 변경하지 않는다.
- fixed prefix/suffix line, repeat fragment의 line, repeat 사이에서 합쳐지는 끝/시작 line을 named occurrence 검색에 포함한다. 같은 물리 line에 중복 기재된 같은 식별자는 여러 occurrence로 세지 않는다.
- `grid-template-areas`와 auto-repeat의 결합, subgrid, writing mode, auto-repeat과 integer `repeat()` 혼합은 계속 거부한다. line name이 없는 numeric placement 및 현재 auto-placement 동작은 회귀 확인한다.

## 고정 입력과 oracle

다음 8개 scene의 모든 used track, root-relative item frame과 repeat count를 고정한다. viewport는 `1280×633 CSS px`, DPR 1이며 layout 자체는 fixed-size다.

| scene | CSS 입력의 핵심 | expected root-relative item |
| --- | --- | --- |
| `positive` | 180px column, gap 10px, `repeat(auto-fill, [cell] 40px [cell])`, `2 cell / 3 cell` | `(50,0) 40×40px`; repeat count 3 |
| `bare` | 같은 grid, bare `cell / 2 cell` | `(0,0) 40×40px`; 생략 occurrence는 첫 named line |
| `negative` | 같은 grid, `-2 cell / -1 cell` | `(100,0) 40×40px`; repeat count 3 |
| `span` | 같은 grid, line 1부터 `span 2 cell` | `(0,0) 90×40px`; repeat count 3 |
| `fit` | 같은 grid에서 `auto-fit`, 두 번째 track만 점유 | `(0,0) 40×40px`; used tracks `0,40,0px` |
| `neighbors` | 260px columns, 20/10px neighbors, 40px+30px repeated pattern, named repeated placement | `(120,0) 80×40px`; repeat count 2 |
| `merge` | 120px columns, 10px neighbors, 같은 이름을 repeat와 바깥 경계 양쪽에 둠 | `(20,0) 80×40px`; repeat count 3 |
| `row` | 180px rows, gap 10px, 40px auto-fill row, `2 row / 3 row` | `(0,50) 100×40px`; repeat count 3 |

독립 계산, Chrome output, fixture SHA와 screenshot은 [사전 비교 근거](../spec/internal/evidence/c11-4-auto-repeat-named-lines-precomparison-2026-10-23/README.md)에 보존한다. Chrome observation과 CSS normative calculation은 서로의 대체 근거로 쓰지 않는다.

## 구현 및 검증 절차

1. 계획 PR 병합 뒤 최신 `main`에서 작업 브랜치를 만든다. 기존 미추적 evidence와 tmux 세션은 보존한다.
2. cascade→projection→layout→Taffy style→named-line resolution 순서와 line-group 의미를 다시 추적한다. `GridPlacementInput`의 Copy 경계를 바꾸거나 deferred name을 전달하는 방법을 결정하고, name copy 및 vector allocation은 Spinon bridge에서 fallible 하게 처리한다. repeat count 계산을 두 번째 구현하지 않는다.
3. 구현 전 사전 comparison의 fixture/data hash와 Taffy 버전을 재확인하고, focused layout/Rust/runtime fixture 및 기존 named-line, integer-repeat, numeric auto-repeat 회귀 대조를 추가한다. named-line input이 없거나 미지원 조합이면 partial layout 없이 실패하는 negative controls를 둔다.
4. 8개 고정 입력의 `grid-template-columns/rows`, 반복 횟수, Taffy expanded line names, resolved placements, all node frame 및 stage trace를 비교한다. Chrome 기준은 float track/frame 비교 최대 `1/64 CSS px`로 제한하고 integer line lookup, repeat count와 collapse는 정확히 일치해야 한다.
5. 최신 앱의 `c04-runtime-gpu-fixture-test` opt-in fixture 경로를 사용해 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 8/8 scene을 실행한다. Android PixelCopy와 iOS Metal drawable 표본에서 각 item의 안쪽, 바깥쪽 edge와 인접 track/gutter를 확인한다. 앱 renderer가 실제 border stroke를 그리지 않으면 border paint 통과를 주장하지 않는다. 설치 artifact SHA가 빌드 산출물과 일치해야 한다.
6. 기본/product archive와 fixture-test archive를 음성/양성 대조하고 Rust focused tests, `cargo fmt --all -- --check`, workspace Clippy, Android/iOS simulator build 및 fixture module-boundary 검사 결과를 별개로 기록한다. 실제 수행한 검사만 통과로 쓴다.
7. 구현과 runtime을 계획 검토와 다른 실패 관점 20개로 검토한다. 최종 PR diff를 30개의 서로 다른 계약·회귀 관점으로 검토하며, 발견 결함은 고친 뒤 영향받는 관점을 재검토한다.
8. 구현 PR 본문에는 C11.4 미완료와 `72/213` 유지, 명세·Chrome oracle·simulator evidence 및 제외 범위를 적는다. 가능한 simulator screenshot을 PR 본문에 넣고 Agent Browser로 이미지 URL이 실제 표시되는지 확인한다. 브라우저는 그 확인 뒤 닫고 `agent-browser session list`와 process 목록이 비었는지 검사한다.
9. 최신 GitHub required checks와 merge state를 확인하고 Rebase and merge한다. 병합 SHA/rollup을 기록한 뒤 기존 Tailnet preview 경로를 hash로 동기화·검증하고, Agent Browser로 Tailnet roadmap과 연결된 evidence image를 확인한 후 브라우저를 종료한다.

## 제외 범위와 완료 해석

여러 auto-repeat component, 한 axis의 여러 auto-repeat, areas와 repeat, missing-name implicit line 생성, arbitrary track grammar, percentage·intrinsic repeat combination, subgrid, RTL/writing mode, WPT, browser 간 호환, Android 실기기, Android hardware GPU, iOS compositor 이후 pixel equality는 포함하지 않는다. 이 slice가 병합되어도 C11.4는 완료되지 않으며 공개 CSS admission으로 승격하지 않는다. 내부 계약 0059는 `0.1.0`으로 유지한다.

## 계획 검토

- [계획 실패 관점 검토 20개](./c11-4-auto-repeat-named-lines-plan-review-20.md)
- [Chrome 156 사전 비교와 고정 fixture](../spec/internal/evidence/c11-4-auto-repeat-named-lines-precomparison-2026-10-23/README.md)

## 구현/runtime 결과

- [Android API 37 AVD·iPhone 17 Pro/iOS 26.2 Simulator 8-scene 결과, screenshot, artifact hash, 테스트와 제한](../spec/internal/evidence/c11-4-auto-repeat-named-lines-runtime-2026-10-23/README.md)
- [구현/runtime 독립 실패 관점 20개](../spec/internal/evidence/c11-4-auto-repeat-named-lines-runtime-2026-10-23/implementation-runtime-review-20.md)
- [PR diff 실패 관점 30개](../spec/internal/evidence/c11-4-auto-repeat-named-lines-runtime-2026-10-23/pr-diff-review-30.md)

missing/insufficient named-line implicit fallback은 후속의 별도 고정 slice로 추가했다. 범위는 단일 `auto-fill` 축의 integer named line과 forward/backward named span에 한정하며, [집중 계획](./c11-4-auto-repeat-named-line-implicit-fallback.md), [Chrome 사전 비교](../spec/internal/evidence/c11-4-auto-repeat-named-line-implicit-fallback-precomparison-2026-10-23/README.md), [Android/iOS Simulator 실행](../spec/internal/evidence/c11-4-auto-repeat-named-line-implicit-fallback-runtime-2026-10-23/README.md)을 따른다. 전체 Grid 지원이나 C11.4 완료로 확대하지 않는다.
