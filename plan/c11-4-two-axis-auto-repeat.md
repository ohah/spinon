# C11.4 · 양쪽 Grid 축의 auto-repeat 계획

**상위 원본:** [CSS Grid 계획](c11-css-grid.md) · [C11.4 track sizing 계획](c11-4-grid-track-sizing.md) · [CSS 적합성 계약](../spec/0001-conformance.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** 계획 PR [#367](https://github.com/ohah/spinon/pull/367)이 Rebase and merge됐다. 계획 자체의 독립 실패 관점 20개 검토 뒤 제한 구현을 진행했고, 구현 후 코드·런타임 20관점과 Android/iOS Simulator 33-scene 행렬을 마쳤다. 구현 PR [#368](https://github.com/ohah/spinon/pull/368)을 열었으며 최종 diff 30관점 검토와 병합은 남아 있다.

## 목적과 규범 근거

현재 C11.4 제한 subset은 `grid-template-columns` 또는 `grid-template-rows` 중 한 track list에만 auto-repeat 하나를 허용한다. [CSS Grid Level 1 §7.2.3.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#repeat-notation)는 auto-repeat을 각 track list에서 최대 한 번으로 제한한다. `grid-template-columns`와 `grid-template-rows`는 서로 다른 track list이므로 한 개씩 두 축에 함께 두는 것은 이 제한을 넘지 않는다. 같은 규칙과 [§7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#repeat-notation)에서 반복 수 산정 및 `auto-fit` collapse를 도출한다.

이 slice는 이미 구현된 한 축 subset을 보존하면서 두 축을 독립적으로 확장한다. 각 track list는 기존과 같이 auto-repeat 하나만 가질 수 있다. 두 축 각각 `auto-fill`/`auto-fit`을 선택할 수 있어야 하며, 두 축의 반복 수·collapse·line range는 서로 덮어쓰거나 공유하지 않는다.

## 범위

- `RuntimeBlockFlexGridTrackSizingV1`에서 columns와 rows에 각각 auto-repeat 하나를 허용한다. 둘 중 한 축만 쓰는 현재 동작도 회귀 없이 유지한다.
- 기존 parser subset을 유지한다: 한 list 안의 auto-repeat 하나, 제한된 fixed-minimum repeated track pattern, finite non-negative fixed `px` explicit neighbors, 반복 축의 line names 없음, `grid-template-areas` 없음. Integer repeat/auto-repeat 혼합, 여러 auto-repeat/list, 비-`px` neighbor와 다른 track grammar는 새로 허용하지 않는다.
- Columns/rows에서 `auto-fill`과 `auto-fit`을 독립 선택한다. 혼합 mode의 빈 축 및 두 번째 반복에만 놓인 item을 포함한다.
- Grid container에 definite 200×200px 또는 200×266px size, `8px` row/column gap, prefix `20px`, suffix `10px`, 반복 fragment `minmax(40px, 1fr) 20px`를 사용한다. Fixture는 `horizontal-tb`/LTR이고 item은 명시 line에 놓인 12×12px fixed box다. text/image contribution과 implicit track 생성을 새 범위로 끌어들이지 않는다.
- one-axis control 2개와 dual-axis case 10개를 포함한 oracle 12개 ID를 app fixture에 연결한다. [기존 실행 근거](../spec/internal/evidence/c11-4-auto-repeat-two-track-min-size-2026-10-20/README.md)의 C11.4 explicit-neighbor scene 21개도 유지하여 Android/iOS에서 **33개 scene** 전체를 재실행한다.

## 구현 전 oracle과 판정

[사전 비교 기록](../spec/internal/evidence/c11-4-two-axis-auto-repeat-precomparison-2026-10-20/README.md)은 독립 규범 산출, fixture·JSON·PNG hash, Chrome 결과 및 해석 한계를 보존한다. 입력 fixture는 [grid-auto-repeat-both-axes-c114.html](../tests/fixtures/css/c11/grid-auto-repeat-both-axes-c114.html)이다.

양쪽 explicit neighbor가 있고 definite axis size가 200px일 때 반복 횟수 `n`의 최소 tracks+gutters 사용량은 `20 + 10 + 60n + 8(2n + 1) = 38 + 76n`이다. 따라서 가장 큰 양의 정수 반복 수는 두 번이고 두 `1fr` track은 남은 10px을 나눠 각각 45px이 된다. 266px row axis는 세 번 반복하며 각 `minmax(40px, 1fr)`는 40px이다. `auto-fit`은 반복 수를 먼저 산정한 다음 placement 결과에 따라 빈 repeated track과 해당 gutter를 접는다.

- Oracle: W3C CSS Grid Level 1 2025 CRD와 서로 독립된 Chrome 155 Agent Browser 관찰.
- 재현 환경: Agent Browser 0.31.1, HeadlessChrome `155.0.0.0` UA, arm64 macOS host, viewport 1280×633 CSS px, DPR 1. 관찰값은 DOM의 `getComputedStyle()`과 `getBoundingClientRect()`로 수집했다.
- 비교: 두 axis의 모든 resolved track, root/item relative frame, 8px gap, repeat count, empty/occupied collapse와 columns/rows stage trace. JSON은 원 관찰값이며 기대값을 앱 출력에서 역산하지 않는다.
- 정수 tracks와 frame은 정확 비교한다. fractional 결과가 나오면 개별 track 및 frame 오차를 1/64 CSS px 이내로 둔다.
- 화면 PNG는 fixture와 DOM 관찰이 연결되는지 확인하는 재현 보조물이다. Spinon 또는 native surface 결과를 증명하지 않는다.

## 구현·검증 관문

1. **Cascade:** columns/rows의 독립 parse 결과와 각 `ComputedCssGridAutoRepeat` metadata를 모두 보존한다. 두 축 조합을 거부하던 조건만 제거한다. line names, template areas와 같은 기존 unsupported 조합은 fail-closed로 유지한다.
2. **Typed projection/layout:** 각 axis의 `GridAutoRepeat` range를 독립 투영한다. layout validator는 각 axis를 따로 검증하고 두 auto-repeat이 모두 있어도 각각 valid하면 허용한다. 반복 track range 오류, 잘못된 `placement_mode`, repeated-axis line names, 잘못된 gap/track 입력은 기존 구체 오류로 거부한다. Taffy 0.15.0의 두 track-list 알고리즘은 축별 repetition count를 읽지만, 프로젝트 adapter와 실제 runtime 결과를 독립 검증한다.
3. **Rust regression:** cascade·projection·layout 경계에서 one-axis auto-fill/fit, two-axis fill/fill, fit/fit, fill/fit, fit/fill, 축별 반복 횟수 차이, 두 번째 repeat placement, one-/two-track `auto-fit` occupancy를 검사한다. 기존 두 축 거부 테스트는 valid acceptance와 malformed/unsupported rejection으로 나눈다.
4. **App fixture:** oracle의 12개 ID를 Rust scene source와 Android/iOS allowlist에 1:1로 연결하고, 기존 21 scene을 유지한다. 플랫폼별 기대값 검사는 기존 33개 ID의 중복 없는 정확한 집합과 일치해야 한다. 기대 node frame·used track·gap/collapse·axis별 trace 비교를 모두 수행하고, 성공 로그에 scene count, per-axis tracks, draw/present marker가 있어야 한다.
5. **플랫폼:** Android API 37 ARM64 AVD와 iPhone 17 Pro / iOS 26.2 Simulator에서 각각 33/33 scene을 실행한다. 실기기 대신 simulator 사용을 허용한다. Native app 화면은 각 simulator의 실행 결과와 surface 캡처로 검사한다. Chrome oracle과 Tailnet preview 렌더링은 Agent Browser로 확인하며, Terminal Browser와 tmux는 사용하지 않는다. 각 Agent Browser 실행은 종료하고 남은 agent-browser process를 확인한다.
6. **오류·기본 산출물:** unsupported 문법의 기존 오류 동작과 기본 artifact의 fixture 경계를 확인한다. fixture 전용 코드·FFI에 변경이 있을 때는 기본 artifact negative와 fixture-test positive control을 각각 확인한다.
7. **기록·PR:** 내부 계약 0059, 적합성 요약, C11.4 상세 계획과 실행 근거를 실제 결과에 맞춘다. 전체 C11.4/WPT/공개 CSS 완료를 주장하지 않고 `spec/STATUS.md` 체크 및 완료 수를 유지한다. 구현 후 새로운 실패 경로 20개를 code/runtime 대상으로 검토한다. PR 본문에 Android/iOS 동작 캡처를 이미지로 붙일 수 있으면 실제 simulator 캡처를 첨부한다.
8. **Tailnet roadmap:** 구현 병합 뒤 사용자 요청에 따라 공개 preview roadmap을 동기화한다. 변경된 경로와 SHA를 검증하고 Agent Browser로 렌더링된 C11.4 항목과 count를 확인한 뒤 세션/daemon을 종료한다. 이를 위한 sync PR은 구현 PR 리뷰와 별도로 diff 적대 검토 30개를 수행한다.
9. 계획·구현·동기화 PR은 각각 독립된 최종 diff 실패 관점 30개를 기록하고 current required checks가 통과한 뒤 Rebase and merge한다. 계획 20개 검토, 구현 후 새로운 code/runtime 20개 검토, PR diff 30개 검토는 서로 대체할 수 없다.

## 제외 및 미해결 범위

이 slice는 한 list 안의 여러 auto-repeat, repeated line name, `grid-template-areas`, integer repeat와 auto-repeat 혼합, non-px explicit neighbor, arbitrary intrinsic contribution 및 10,000-track boundary matrix를 닫지 않는다. Chrome 155의 definite minimum-size `auto-fit` 축 차이는 [별도 후속 계획](./c11-4-auto-repeat-two-track-min-size.md)에 기록되어 있으며 이 slice에서 규범을 Chrome 진단에 맞추지 않는다. WPT 실행, 실기기, hardware GPU, iOS compositor-after pixel equality와 C11.4 전체 적합성도 이 행렬로 증명하지 않는다.

## 관문 상태

- [x] Chrome 사전 비교 fixture와 원 관찰 기록
- [x] 서로 다른 실패 관점 20개 계획 검토 ([검토 표](../spec/internal/evidence/c11-4-two-axis-auto-repeat-precomparison-2026-10-20/plan-review-20.md))
- [x] 계획 PR #367 검토·Rebase and merge
- [x] 새 코드/runtime 실패 관점 20개 검토 ([구현 검토](../spec/internal/evidence/c11-4-two-axis-auto-repeat-2026-10-20/implementation-review-20.md))
- [x] Android/iOS Simulator 33-scene 행렬 ([실행 근거](../spec/internal/evidence/c11-4-two-axis-auto-repeat-2026-10-20/README.md))
- [ ] 구현 PR 최종 diff 30관점 검토·Rebase and merge
