# C11.1 · parsed Root HTML 한 축 고정 Grid 배치 계획

**상태:** Chrome 156 Agent Browser 사전 비교 14 scenes·41 item nodes·40 layout frames를 고정했다. 전용 layout mode와 Root HTML projection route 구현 및 focused layout 5/5, style-to-layout 6/6, RuntimeSession 2/2, sparse/dense placement 회귀 77/77, scoped Clippy·rustfmt가 통과했다. Android API 37 AVD SurfaceView PixelCopy와 iPhone 17 Pro/iOS 26.2 Simulator Metal drawable의 direct readback 표본이 각각 6/6 exact match다. Android/iOS default artifact에서 fixture 전용 marker·adapter·payload와 dedicated FFI symbol이 빠지는 경계도 확인했다. 구현/runtime 실패 관점 20개를 마쳤다. 실제 PR diff 검토 20개와 최종 PR/merge 확인은 남아 있다. 구현 전 제품 baseline 재실행은 기록되지 않았다. 이 bounded slice는 PR에 포함·병합되지 않았으며 C11.1, C11 또는 공식 완료 `72/213`을 완료 처리하지 않는다. [실행·표면 표본·artifact 경계 evidence](../spec/internal/evidence/c11-root-html-grid-one-axis-placement-runtime-2026-11-04/README.md) · [구현/runtime 실패 관점 검토 20개](../spec/internal/evidence/c11-root-html-grid-one-axis-placement-runtime-2026-11-04/implementation-runtime-review-20.md)

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.2.2 sparse row 자동 배치](./c11-2-2-sparse-row-placement.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**계획 실패 관점 검토:** [20개](./c11-root-html-grid-one-axis-auto-placement-plan-review-20.md)

**계획 PR diff 실패 관점 검토:** [30개](./c11-root-html-grid-one-axis-auto-placement-pr-diff-review-30.md)

**Chrome 사전 비교:** [관측·기대값·화면](../spec/internal/evidence/c11-root-html-grid-one-axis-placement-precomparison-2026-11-04/README.md)

## 결손과 목표

Parsed Root HTML 기본 route는 모든 direct Grid item이 네 placement endpoint에 numeric line/span을 가질 때 기존 explicit-only mode로 보낸다. 전부 auto인 아이템은 기존 row-flow 경로가 처리하지만, 한 축에만 definite numeric placement가 있고 다른 축은 auto인 아이템은 아직 그 두 route 어느 쪽에도 들어가지 않는다.

Layout core의 sparse row normalizer에는 definite row 고정, definite column 고정, span, occupancy 및 source order 배치 동작이 있다. 이번 작업은 이를 Root HTML에 그대로 일반 노출하지 않고, 같은 축으로 고정된 item만을 허용하는 Root HTML 전용 mode에 연결한다. 구현은 전용 mode validator가 지원 경계와 implicit-track 제한을 확인한 뒤 기존 정규화 로직을 재사용한다.

## 비교 기준

- 사전 비교는 Chrome 156 Agent Browser의 고정 fixture와 독립 기대값을 사용한다. 관측 환경, hash, 한계는 [evidence README](../spec/internal/evidence/c11-root-html-grid-one-axis-placement-precomparison-2026-11-04/README.md)에 기록했다.
- 기대 배치는 [CSS Grid Layout Level 1 §8.5](https://www.w3.org/TR/css-grid-1/#auto-placement-algo)의 sparse row automatic placement를 authored fixed tracks와 source order에 적용해 산출한다.
- Core 10 scenes에서 기존 all-auto control 1개는 기존 경로를 보존하고, 제품 candidate 9개에서 positive 5개와 expected rejection 4개를 확인한다. Supplementary 4 scenes는 hidden-item occupancy, 같은 부모의 sibling owner 두 개, nested Grid rejection을 고정한다. 전체 verifier는 41 item records·40 layout frames와 Chrome computed placement·track geometry를 대조해 최대 오차 0 CSS px를 확인했다. Scene-level 결과는 all-auto 1 control, 8 accepted, 5 expected rejection이다.
- Chrome은 동작 oracle다. 이를 제품 결과·WPT 통과·Android/iOS runtime 결과로 해석하지 않는다.

## 제한 지원 계약

1. Parsed Root HTML `RuntimeRootHtmlTextPaintV1` 기본 route의 no-area, `grid-auto-flow: row` Grid에만 적용한다. Computed typed style과 현재 root/document/revision identity를 사용하고 선언 문자열을 다시 파싱하지 않는다.
2. 전용 `RootHtmlOneAxisAutoPlacementV1` mode를 둔다. 기존 `ExplicitOnly`, C11.2.2 `SparseRow`, C11.2.3 dense/column 및 C11.3 template modes의 계약을 넓히지 않는다.
3. candidate Grid의 모든 visible direct in-flow item은 동일한 한 축에만 definite numeric position을 가진다. 나머지 축의 양 끝은 모두 `auto`다. 행 고정과 열 고정을 한 Grid에서 섞지 않는다. 모든-auto Grid는 기존 route에 남긴다. 양 축 fully definite item 또는 fully definite와 one-axis item의 혼합은 거부한다.
4. definite axis에는 explicit track 범위 안의 양·음수 numeric line 및 양의 numeric span 조합을 허용한다. line·span의 계산 overflow, 0 이하 span, 범위 밖 line, implicit extent는 지정 property와 node가 포함된 기존 layout 오류로 거부한다. Named line, named span 및 area placement는 지원하지 않는다.
5. 행·열 template은 비어 있지 않은 유한한 non-negative fixed-px 목록으로 제한한다. 이 최초 slice의 Chrome fixture는 3 columns `[40, 50, 30]px`, 2 rows `[20, 30]px`, gap 0이다. 제품 profile은 track 개수 기존 상한을 지키고, implicit row 또는 column이 필요한 결과를 publish하지 않는다. `grid-auto-rows`/`grid-auto-columns`에 값을 지정해도 새 implicit track이 필요한 장면은 거부한다.
6. `grid-template-areas`, `repeat()`/auto-repeat, named lines, dense·column flow, nonzero order, positioned item, nested Grid, text/replaced/non-empty item, non-fixed track·gap, non-LTR/vertical writing mode 및 범위 밖 box style은 fail-closed한다. 지원하는 항목은 static, empty, non-replaced direct block item이며 auto/fixed size만 허용하고 기존 track stretch semantics를 사용한다.
7. `display:none` child는 occupancy에서 제외하고 `visibility:hidden` child는 다른 in-flow item과 같이 occupancy에 참여한다. 이 규칙을 candidate discovery와 layout normalizer 양쪽에서 검증한다. DOM source order, paint order, accessibility order는 재배치하지 않는다.
8. 검증 실패 시 일부 item placement, expanded track, frame 또는 draw record를 남기지 않는다. 한 Grid의 reject가 같은 document의 다른 Grid 결과를 바꾸지 않아야 한다.

## 구현과 검증 순서

1. Chrome 사전 비교 단계에서 14-scene fixture의 all-auto·one-axis·span·negative-line·implicit-track 동작과 supplementary hidden/sibling/nested scenes를 재현했다. Independent expected geometry·occupancy와 Chrome observation hash를 확인했다. 구현 전 제품 baseline 재실행은 생략된 상태로 기록하고, 지금은 all-auto 기존 경로 보존과 후보 배치를 변경 후 테스트에서 확인한다. 이전 C11.2.2 sparse matrix와 C11.2.3 dense/column matrix는 변경하지 않는다.
2. Root HTML candidate selector를 추가한다. 각 owner의 visible direct children, same-axis definiteness, endpoint shape, flow, areas, nested/positioned/text conditions를 확인한다. 후보 selection에 실패하면 일반 route를 통해 accidental partial support가 생기지 않는지 assertion으로 고정한다.
3. Layout 전용 mode와 validator를 추가하고 sparse placement normalizer를 재사용한다. partial placements를 기존 explicit-only normalizer가 처리하지 않도록 normalization dispatch를 분리한다. validator는 fixed explicit tracks, no implicit extent, supported child sizing·style, order, occupancy 및 오류 attribution을 확인한다.
4. Style projection/Layout 단위 검사와 실제 parsed Root HTML runtime fixture를 연결한다. Core 9 candidate scenes(성공 5·거부 4), all-auto control 및 supplementary scenes를 함께 수행하고 placement line·item frame·expanded track 부재·rejection atomicity를 비교한다. Sibling Grid의 두 owner 격리와 nested Grid 거부, stale revision도 negative/positive controls로 확인한다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS Simulator에서 최신 source로 같은 fixture를 실행한다. Android PixelCopy와 iOS Metal drawable 결과를 Chrome expected와 geometry tolerance ±0.5 CSS px, 고정 surface sample exact로 각각 비교한다. Android software renderer 및 iOS compositor 전 readback 여부를 별도로 기록한다.
6. 기본 product artifact에서 새 fixture payload·전용 FFI symbol이 제외되는지 확인하고 opt-in fixture-test artifact를 positive control로 확인한다. Rust focused tests, scoped Clippy, `cargo fmt --all -- --check`, CSS reference/fixture boundary 검사를 실행한다. 계획 검토와 구현/runtime 실패 관점 20개, PR 실제 diff 적대 검토 20개는 서로 다른 관점으로 기록한다. 실행하지 않은 CI·simulator gate는 통과로 적지 않는다.
7. 구현 PR에 내부 계약 0059의 bounded candidate 동작과 한계를 같은 변경으로 기록한다. 병합 뒤 계획과 `spec/STATUS.md`에는 실제 merge SHA·검증 결과를 반영하고, C11.1/C11 상위 checkbox 및 공식 완료 `72/213`은 그대로 유지한다.

## 범위 밖과 미검증

전체 CSS Grid, mixed-axis 한 Grid, all-auto와 one-axis 혼합, fully definite 혼합, implicit track 생성, named lines/areas, repeat/auto-repeat, dense/column flow, intrinsic/non-empty/replaced item sizing, nonzero gap, alignment 확장, RTL·vertical writing mode, WPT 적합성, Android 실기기/hardware GPU, iOS compositor 이후 surface와 C11 전체 완료는 이 계획에서 주장하지 않는다.
