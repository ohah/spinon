# C11.4 span-stage implementation review addendum


**검토일:** 2026-10-26

**대상:** [implementation review 20](./c11-4-span-stage-oracle-implementation-review-20-2026-10-26.md), 최종 코드·계획 diff

**방식:** 기존 PARTIAL 관점만 읽기 전용으로 재확인했다. Rust test, browser, simulator는 실행하지 않았다.

## 이전 관점 19 재검토


**PASS.** `assert_stage_snapshot`은 trace 길이를 CSS track `N`개에 대한 `2N+1`로 고정하고, 짝수 index는 gutter, 홀수 index는 CSS track인지 검사한다. 이어서 첫·마지막 boundary gutter만 collapsed이고 그 밖의 모든 entry는 non-collapsed인지 각 stage에서 비교한다([구현](../../../crates/spinon-layout/src/grid_multispan_chrome_oracle_tests.rs#L328)). 이 4-column fixture의 9개 entry에서는 index 0/8만 collapsed여야 하므로 내부 zero-gap gutter의 잘못된 collapsed flag도 실패한다.

대조한 Taffy 경계는 일치한다. `initialize_grid_tracks`는 생성 후 첫·마지막 grid-line entry를 collapse하고([Taffy 초기화](../../../vendor/taffy/src/compute/grid/explicit_grid.rs#L444)), trace는 `is_gutter`와 `is_collapsed`를 그대로 기록한다([trace projection](../../../vendor/taffy/src/compute/grid/mod.rs#L1615)). Auto-fit이 없는 fixture의 내부 gutter와 CSS track은 collapsed sentinel이 아니다.

## 계획·코드 일치 및 최종 판정

- 계획은 gutter/track의 `2N+1` pattern과 첫·마지막 boundary sentinel만 collapsed라는 입력 경계를 명시하며, 구현의 assertion과 일치한다([계획](../../../plan/c11-4-span-stage-oracle.md#L37)).
- 계획은 Taffy가 `growth_limit.is_finite().then_some(value)`로 투영해 `None`이 실제 `+∞`와 NaN/`-∞`를 구별하지 못한다고 한정한다. 구현 주석도 같은 한계를 적고, vendor trace projection과 일치한다(구현 366–369행, vendor 1620행).
- 이전 보고서의 유일한 PARTIAL을 해소했다. **유효 implementation review는 20/20 PASS**이며, 이 재검토에서 새 correctness blocker는 발견하지 못했다. 구현자 focused test/fmt/clippy 결과는 기존 보고서에 기록된 값이며 재실행하지 않았다.

## 남은 PR 구조 메모

테스트 파일은 631행으로 프로젝트 권장 300–500행을 넘는다. 이는 계산/assertion correctness blocker는 아니다. PR 설명에서 oracle과 그 negative controls를 한 파일에 둔 응집 이유를 적거나, 책임 분리가 적절한지 설명해야 한다([project rules](../../../docs/project-rules.md#L30)).
