# C11.1 · parsed Root HTML intrinsic Grid percentage gap

**상태:** 구현·Android/iOS 시뮬레이터 검증과 실제 PR diff 실패 관점 30개 검토를 마친 제한 후보. 구현 PR [#1176](https://github.com/ohah/spinon/pull/1176)의 check/merge가 남아 있다. C11.1/C11은 미완료이며 공식 완료 수는 78/214다.

**선행 구현:** [intrinsic explicit tracks](./c11-root-html-intrinsic-grid-tracks.md), [고정 px intrinsic gap · PR #1172](./c11-root-html-grid-intrinsic-gap.md), [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**구현 전 기준:** [Chrome 156 Agent Browser 사전 비교와 독립 기대값](../spec/internal/evidence/c11-root-html-intrinsic-gap-percentage-precomparison-2026-11-05/README.md) · [fixture](../tests/fixtures/css/c11/root-html-grid-intrinsic-gap-percentage-precomparison.html) · [expected](../spec/internal/evidence/c11-root-html-intrinsic-gap-percentage-precomparison-2026-11-05/expected.json) · [Chrome observation](../spec/internal/evidence/c11-root-html-intrinsic-gap-percentage-precomparison-2026-11-05/chrome156-observation.json) · [9-scene comparison](../spec/internal/evidence/c11-root-html-intrinsic-gap-percentage-precomparison-2026-11-05/comparison.json)

**계획 실패 관점 검토:** [20개](./c11-root-html-grid-intrinsic-gap-percentage-plan-review-20.md). **계획 PR 변경 검토:** [30개](./c11-root-html-grid-intrinsic-gap-percentage-plan-pr-diff-review-30.md). 구현/runtime 실패 관점 [20개](../spec/internal/evidence/c11-root-html-intrinsic-gap-percentage-runtime-2026-11-05/implementation-runtime-review-20.md)와 실제 구현 PR diff [30개](../spec/internal/evidence/c11-root-html-intrinsic-gap-percentage-runtime-2026-11-05/pr-diff-review-30.md)를 별도로 마쳤다.

**구현 후보 결과:** 기존 intrinsic Grid 제한 profile에만 direct percentage gap을 연결했다. Layout focused tests 8/8, Root HTML runtime tests 11/11, 변경 crates Clippy, FFI check 및 fixture-module boundary 33/33을 통과했다. Chrome 156 사전 비교의 9 scene과 동일한 runtime fixture를 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 실행해 각각 `layout=ready boxes=48` 및 draw/present를 확인했다. Simulator screenshots는 visual smoke이며 PixelCopy/Metal drawable readback이나 WPT 적합성 근거가 아니다. [실행 evidence](../spec/internal/evidence/c11-root-html-intrinsic-gap-percentage-runtime-2026-11-05/README.md) · [구현/runtime 실패 관점 검토 20개](../spec/internal/evidence/c11-root-html-intrinsic-gap-percentage-runtime-2026-11-05/implementation-runtime-review-20.md). CSS projection에서 표현할 수 없는 큰 computed percentage는 `unsupported_computed_value`로 layout 전에 거부하고, typed percentage의 resolved gap 또는 aggregate gutter가 f32 범위를 넘으면 해당 property로 원자 실패한다.

## 결손과 목표

기본 parsed Root HTML RuntimeRootHtmlTextPaintV1 경로는 PR #1167에서 explicit intrinsic tracks를 연결하고 PR #1172에서 해당 profile의 fixed-px row-gap과 column-gap을 연결했다. 현재 RootHtmlIntrinsicTrackSizingV1 layout admission은 gap 입력으로 finite non-negative fixed px만 허용한다. 같은 profile의 computed percentage gap은 CSS에서 유효하고 기존 LayoutGap과 Taffy Grid adapter가 표현하지만 layout 전에 거부한다.

이 계획은 이미 지원하는 intrinsic explicit-track profile에 finite non-negative direct percentage gap을 연결한다. 열 gap은 definite Grid content-box width, 행 gap은 definite content-box height를 basis로 한다. gap shorthand 및 grid-gap alias도 cascade가 만든 computed row-gap/column-gap 경로를 그대로 사용한다. 새 CSS parser, public API, runtime profile, FFI 경로 또는 fixture-only 성공 경로는 추가하지 않는다.

## 규범과 비교 결과

[CSS Gaps Level 1 §2.3](https://www.w3.org/TR/css-gaps-1/#percentages-in-gap-properties)는 percentage gap을 해당 요소 content-box 축에 비례시킨다. Grid의 cyclic percentage gap은 intrinsic size contribution 계산에서 0으로 처리하고 contents layout 시 content-box 기준으로 해석한다. 이 계획은 Grid의 두 축이 모두 finite positive fixed px인 입력만 받으므로 gap basis는 definite이고 cyclic하지 않다. auto-sized Grid axis, gap이 컨테이너 intrinsic size에 되먹임되는 경우, cyclic contribution 의미를 다루지 않는다.

[CSS Grid Layout 2025-03-26 CRD §10.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#gutters-the-row-gap-column-gap-and-gap-properties)는 각 사용 gutter를 track sizing에서 빈 fixed-size track으로 취급한다. §11.1의 available-space 계산은 gutter를 포함한다. 이 작업의 fixed-size root에서는 10% gutter가 track contribution을 만드는 대신 definite free space에서 빠진다. 0%는 기존 zero-gap과 같고, 120%는 clamp하지 않아 fixed-size item의 visible overflow를 만든다.

Chrome 관측과 기대값은 별도 파일에 둔다. Expected는 고정 item 기여와 축별 definite basis를 독립 산술로 계산했고 [comparison.json](../spec/internal/evidence/c11-root-html-intrinsic-gap-percentage-precomparison-2026-11-05/comparison.json)은 9 scene의 computed gap syntax·resolved track·item frame이 Chrome 156 관측과 모두 맞음을 기록한다. Chrome의 computed style은 authored percentage를 보여 주므로 사용 gap px는 track list와 item position으로 별도 확인한다.

| 장면 | Grid content box | 계산한 사용 gutter | 열 track | 행 track |
|---|---:|---:|---:|---:|
| control | 100×100px | 0/0px | 40, 60px | 20, 80px |
| column 10% | 100×100px | 0/10px | 40, 50px | 20, 80px |
| row 10% | 100×100px | 10/0px | 40, 60px | 20, 70px |
| both 10% | 100×100px | 10/10px | 40, 50px | 20, 70px |
| mixed basis 25% 10% | 120×80px | 20/12px | 40, 68px | 20, 40px |
| column 120% | 100×100px | 0/120px | 40, 0px | 20, 80px |
| row 120% | 100×100px | 120/0px | 40, 60px | 20, 0px |
| zero percent | 100×100px | 0/0px | 40, 60px | 20, 80px |
| grid-gap alias 25% 10% | 120×80px | 20/12px | 40, 68px | 20, 40px |

행 gutter와 열 gutter는 표에서 이 순서로 적었다. 100×100px cases의 first column min-content는 40px이고 first row min-content는 20px이다. 120% case에서 해당 second flexible track은 0px까지 줄지만 고정 크기 item은 gutter 뒤의 위치에 놓여 Grid 밖으로 overflow한다.

## 제한 입력 계약

- 기존 RootHtmlIntrinsicTrackSizingV1 profile, definite positive fixed-px root width/height, zero margin/padding/border, no-area, row auto-flow, supported explicit tracks, fixed placement와 item shape를 유지한다.
- 기존 finite non-negative fixed px gap과 normal/zero 동작을 보존하고 direct LayoutLengthPercentage::Percentage(fraction)을 추가한다. 양수 percentage는 유한하고 0 이상이어야 한다.
- row-gap은 root content height, column-gap은 root content width를 사용한다. 이 profile은 box edge를 0으로 제한하므로 content-box basis와 고정 root dimension이 같다. 값을 반올림하거나 0–100%로 clamp하지 않는다.
- 고정 track 수에서 gutter는 adjacent tracks 사이에만 생긴다. aggregate used gutter는 (explicit track count - 1) × used gap으로 산정한다. f64 중간 연산으로 검증하고 resolved gap, aggregate gutter 또는 Taffy track position이 finite f32 CSS px로 표현되지 않으면 해당 gap property 진단으로 전체 layout을 거부한다.
- Grid gap이 100%를 넘는다는 이유만으로 거부하지 않는다. 유한한 used gap 및 item frame은 성공 결과로 보존하며 실제 overflow도 자르거나 root 크기로 clamp하지 않는다.
- gap shorthand, row-gap, column-gap, grid-gap alias는 같은 typed longhand 값과 축별 resolver를 거친다. 두 축 basis를 서로 바꿔 쓰는 경로를 허용하지 않는다.
- CSS-invalid negative declaration은 Stylo의 cascade fallback으로 다룬다. typed negative, NaN, infinity, resolved overflow와 unsupported computed calc gap은 성공 layout으로 가장하지 않고 전체 layout 전에 거부한다.

## 제외 범위

percentage gap이 있는 indefinite/auto Grid axis, cyclic intrinsic contribution, auto-sized Grid container, intrinsic implicit track sizing, named areas, line names, auto-repeat, dense/column flow, non-zero box edge, arbitrary Grid grammar, calc()/typed math gap, CSS loader, general author CSS admission, WPT conformance, public CSS support 및 C11.1/C11 완료는 이 계획에 포함하지 않는다. PR #1172의 gap extent 검사와 기존 fixed-gap 결과를 유지한다. 이 slice로 공식 완료 count나 내부 contract 0059 숫자 버전 0.1.0을 바꾸지 않는다.

유효하지만 이 profile이 지원하지 않는 computed percentage/calc 입력은 node와 정확한 row-gap 또는 column-gap이 식별되는 layout error로 끝난다. 한 Grid owner가 실패하면 다른 Grid의 성공 frame만 포함한 부분 StyleLayoutOutput이나 paint를 publish하지 않는다.

## 구현·검증 순서

1. 계획 실패 관점 20개와 계획 PR diff 30개를 별도로 마쳤고 계획 PR #1175를 Rebase and merge했다. 이 검토를 구현 검토에 재사용하지 않는다.
2. 기존 profile 경계와 typed percentage projection, Root HTML gap admission, fixed-gap extent preflight, Taffy Grid trace 및 runtime atomicity 경로를 대조했다. 변경은 intrinsic track profile에만 적용했고 Flex·other Grid profile은 바꾸지 않았다.
3. Root HTML gap 지원 검사에 해당 axis의 definite content-box basis를 공급했다. usedGap와 aggregate gutter는 f64 중간 연산으로 검증하며, positive resolved percentage gutter의 non-finite track-line preflight를 연결했다.
4. Rust layout tests는 9-scene fixture의 shorthand/alias, row/column basis, zero 및 120% overflow를 독립 expected와 비교한다. Invalid typed value와 resolved/aggregate f32 overflow 거부도 검사한다. Runtime matrix는 성공 frame과 paint visibility를 확인하고, projection에서 표현 불가능한 큰 percentage 및 기존 invalid/calc 거부를 검사한다.
5. 기본 parsed Root HTML runtime fixture를 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 같은 source hash로 실행했다. 두 플랫폼 모두 layout=ready, 48 boxes 및 draw/present를 기록했다. Screenshot은 visual smoke이며 PixelCopy/Metal drawable readback은 없다.
6. Layout/runtime focused tests, scoped Clippy, FFI check, fixture boundary 33/33, simulator builds와 `cargo fmt --all -- --check`를 통과했다. 전체 `spinon-layout --lib`는 449/450이며 유일한 C15 HostDocument assertion은 fixed-px baseline에서도 실패했다. Fixture-test runtime suite는 후보 276/8/3, 정확한 `5d47b06cc` 기준 275/8/3이고 실패 테스트 목록이 동일하다. 넓은 suite 둘 다 green이라고 주장하지 않는다. [현재·기준 실행 로그](../spec/internal/evidence/c11-root-html-intrinsic-gap-percentage-runtime-2026-11-05/README.md).
7. 구현/runtime 실패 관점 20개 검토를 마치고 Android/iOS simulator 화면을 첨부한 PR #1176을 만들었다. 실제 PR diff 사전 검토에서 simulator runner가 누락된 runtime marker를 통과시킬 수 있고, bundled fixture hash·screenshot·layout/draw 결과를 강제하지 않는 점을 찾았다. runner를 보강하고 양 simulator를 재실행해 source/bundle hash, layout/draw marker 및 non-empty screenshot 검사를 통과시켰다. 후속 실제 PR diff의 서로 다른 실패 관점 30개를 새로 검토해 추가 구현 결함이 없음을 확인했다. 남은 단계는 최신 PR check·merge 확인이다.
8. 병합 뒤 exact merge SHA와 required status/check rollup을 확인하고 internal contract 0059, spec/STATUS.md, C11 계획 및 사용자가 지정한 Tailnet roadmap 미리보기의 제한 범위를 동기화한다. C11.1/C11 체크와 공식 78/214는 그대로 둔다. Tailnet 화면은 Agent Browser로 desktop/mobile을 확인하고 세션·프로세스를 종료한다.

## 완료 판정의 제한

이 계획은 fixed definite axes에서 typed percentage gutter를 기존 intrinsic explicit-track profile에 연결하는 bounded slice다. 전체 Grid intrinsic sizing algorithm, cyclic percentage semantics, general CSS support, 전체 C11.1 또는 C11 완료를 뜻하지 않는다.
