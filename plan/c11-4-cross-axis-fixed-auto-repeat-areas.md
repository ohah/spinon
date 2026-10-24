# C11.4 교차축 fixed-repeat + auto-repeat + `grid-template-areas`

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [동일 축 fixed-repeat + auto-repeat + areas](./c11-4-fixed-auto-repeat-template-areas.md) · [양축 auto-repeat + areas](./c11-4-two-axis-template-areas.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

## 목표와 제한

현재 제한 profile은 동일 track list 안의 fixed-repeat + auto-repeat + areas와 서로 다른 축의 auto-repeat + areas를 각각 지원하지만, fixed-repeat이 있는 축과 auto-repeat이 있는 축이 서로 다르면 cascade에서 거부한다. 이번 slice는 opt-in `RuntimeBlockFlexGridTrackSizingV1`에서 columns 또는 rows 중 한 축의 track list가 정확히 하나의 positive-integer fixed-repeat group으로만 구성되고, 다른 축의 track list가 prefix/suffix 없이 computed finite nonnegative `LengthPx` breadth 하나를 가진 auto-fill/auto-fit group 하나만으로 구성되며, rectangular `grid-template-areas`가 있는 경우를 허용한다. fixed group 내부는 1–2개 computed finite nonnegative `LengthPx` track만 허용한다. Stylo가 절대 `calc()`를 `LengthPx`로 정규화한 경우는 computed boundary에서 허용한다.

이 교차축 예외에서는 fixed-repeat group을 정확히 하나로 제한하며 그 track list에 prefix/suffix `TrackSize`를 두지 않는다. auto-repeat 축도 단일 breadth track으로 된 auto-repeat group만 허용하며 prefix/suffix ordinary track size를 두지 않는다. auto-repeat list 내부 fixed-repeat 혼입, 반대 축 두 번째 fixed-repeat group, 한 track list 안의 두 번째 auto-repeat, auto-repeat multi-track pattern, `minmax()`/percentage/flex/intrinsic/non-normalized calc fixed-repeat breadth, 잘못된 area 행렬·크기 한도, 비대상 runtime profile 및 공개 CSS admission은 계속 fail-closed한다. 기존 #545의 양축 auto-repeat, #604의 동일축 fixed+auto, 무-area 혼합 조합에 대한 기존 정책을 회귀 확인한다. 이 제한 slice만으로 C11.4 또는 C11 완료를 표시하지 않는다.

## 표준 근거와 판정 기준

CSS Grid Layout Module Level 1 2025 CRD §7.2.3.1은 각 explicit track list 안에서 auto-repeat 하나와 fixed-repeat을 함께 사용할 수 있도록 규정한다. §7.2.3.2는 definite content 크기와 gap을 기준으로 반복 수를 정하고 auto-fit의 빈 반복 track 및 인접 gutter를 접는다. §7.1은 expanded track list와 named-area 행렬 중 큰 explicit dimension을 선택하고, area로 추가된 track을 `grid-auto-*`로 크기 지정한다. §7.3.2는 named area에서 각 축의 `name-start`/`name-end` line을 만든다. 두 축이 별도 track list라는 점과 per-list 반복 제한으로부터 교차축 조합도 허용된다고 해석한다. 이는 조항을 조합한 규범 해석이며, 구현 변경 전에 Chrome 156 관측과 CRD 산출값을 비교한다.

고정 입력은 [11-scene fixture](../tests/fixtures/css/c11/grid-cross-axis-fixed-auto-repeat-template-areas-c114.html)다. 모든 root는 padding/border 없이 `horizontal-tb`, LTR, definite width/height, 4px gap을 쓴다. fixed-repeat pattern은 `[20px]`과 `[16px 20px]`, auto-repeat pattern은 `[20px]`이며, area 확장 track은 `grid-auto-columns`/`grid-auto-rows: 12px`다. auto-repeat 계산식은 `n * 20px + (n - 1) * 4px <= definite content size`다. 기대값은 auto-repeat track index 범위를 별도로 저장해 area-added track과 분리한다. 독립 CRD 기대값과 Chrome CSSOM/frame 관측은 [사전 비교 증거](../spec/internal/evidence/c11-4-cross-axis-fixed-auto-repeat-areas-precomparison-2026-10-25/README.md)에 기록한다. Chrome 값은 규범 기대값을 대체하지 않는다.

## 구별해야 할 11개 장면

| ID | 조합과 경계 | Columns (px) | Rows (px) | 주요 item frame (px) |
|---|---|---:|---:|---:|
| `fixed-cols-fill-rows-area-short` | columns fixed ×2, rows auto-fill ×3, 2×2 area | `[20,20]` | `[20,20,20]` | `[0,0,20,44]` |
| `fixed-cols-fill-rows-two-track-pattern` | columns fixed pattern `[16,20]` ×2, rows auto-fill ×2 | `[16,20,16,20]` | `[20,20]` | `[0,0,16,44]` |
| `fixed-cols-fit-rows-one-row-occupied` | columns fixed ×2, rows auto-fit ×3, 첫 행만 item 점유 | `[20,20]` | `[20,0,0]` | `[0,0,20,20]` |
| `fixed-cols-fit-rows-empty-area` | area는 있으나 item이 없어 auto-fit 모두 접힘 | `[20,20]` | `[0,0,0]` | 없음 |
| `fixed-cols-fill-rows-area-adds-two-rows` | area가 rows auto-fill ×3보다 2행 큼 | `[20,20]` | `[20,20,20,12,12]` | `[0,88,44,12]` |
| `fixed-cols-fill-rows-area-adds-one-row` | 44px auto-fill ×2 뒤에 area가 1행 추가 | `[20,20]` | `[20,20,12]` | `[0,48,44,12]` |
| `fixed-cols-fit-rows-middle-collapsed-between-occupied` | auto-fit 가운데 반복 행은 비고 양쪽 반복 행은 점유 | `[20,20]` | `[20,0,20]` | `first [0,0,20,20]`, `last [0,24,20,20]` |
| `fixed-rows-fill-cols-shorthand-area-short` | scene 1의 정확한 축 전치 및 `grid-template` track-list shorthand | `[20,20,20]` | `[20,20]` | `[0,0,44,20]` |
| `fixed-rows-fit-cols-two-track-span` | hero가 두 auto-fit 열을 점유하고 세 번째 열은 접힘 | `[20,20,0]` | `[20,20]` | `[0,0,44,44]` |
| `fixed-rows-fit-cols-empty-area` | 전치 축에서 area만 있고 auto-fit 열은 모두 접힘 | `[0,0,0]` | `[20,20]` | 없음 |
| `fixed-rows-fill-cols-area-adds-one-column` | 44px auto-fill ×2 뒤에 area가 1열 추가 | `[20,20,12]` | `[20,20]` | `[48,24,12,20]` |

44px auto 축은 2회, 76px auto 축은 3회 반복한다. 12px area 확장은 반복 수를 늘리지 않는다. 현재 projection의 계약에 맞춰 고정-repeat 축의 area dimension은 expanded track 수와 같게 두고, expanded list보다 큰/작은 area는 auto-repeat 축에만 적용한다. fixed-axis mismatch는 negative control로 유지한다. 각 scene에서 root/content size, CSSOM resolved track vector, area child frame 및 used grid geometry를 비교한다. auto-fit 장면은 빈 named area가 track을 점유하지 않는지와 실제 item의 span만 점유로 세는지 구별한다.

## 고정해야 할 거부 경로

| 입력 변형 | 기대 결과 | 근거/확장 테스트 |
|---|---|---|
| 대상 scene에서 `grid-template-areas` 제거 | 양 축 방향 모두 cascade 거부 | area presence gate와 같은 track input 양성/음성 쌍 추가 |
| fixed-repeat 축의 area가 expanded track dimension보다 작거나 큼 | projection에서 node/property 오류 | columns-fixed와 rows-fixed 양쪽의 mismatch fixture |
| auto-repeat 축에 fixed-repeat group 추가 | 교차축 예외 거부 | fixed+auto+fixed의 exact group count 경계 |
| auto-repeat 축에 prefix/suffix ordinary track size 또는 multi-track auto-repeat pattern 추가 | 교차축 예외 거부 | 단일 auto-repeat group과 area-added track 경계 검사 |
| fixed-repeat 축에 group 하나 추가 | 교차축 예외 거부 | fixed groups 1/2 양 방향 검사 |
| fixed-repeat pattern에 3개 이상 track 또는 `minmax()`/percentage/flex/intrinsic breadth | 거부 | fixed columns/rows 양 방향 sizing whitelist 음성 대조 |
| 한 축은 auto-repeat, 다른 축도 auto-repeat이며 fixed group 포함 | 계속 cascade 거부 | `fixed_repeat_mixture_is_limited_to_one_auto_repeat_axis` 보존 |
| non-target runtime profile | cascade/projection에서 거부 | `RuntimeBlockFlexGridTemplateV1` 등 opt-in 아닌 profile 추가 |
| #545 양축 auto-repeat + areas, #604 동일축 fixed+auto + areas | 기존 positive 동작 유지 | 기존 cascade/layout/runtime regression과 함께 실행 |

현재 한도는 cascade computed tracks 10,000개와 line metadata 1 MiB, style-to-layout combined line/area metadata 1 MiB, layout tracks 10,000개·source metadata 1 MiB·expanded line metadata 16 MiB다. 경계값/초과값과 Spinon-owned fallible reservation failure 뒤 같은 engine의 후속 성공을 유지한다. Taffy 내부 전체 allocation failure recovery를 보장한다고 주장하지 않는다.

## 구현·검증 순서

1. 구현 전 현재 cascade→typed style-to-layout projection→Taffy named-line/area projection 경로를 확인한다. Chrome 156 사전 관측, CSS Grid 2025 CRD 독립 기대값, 입력 환경, 허용 오차와 불일치 처리 기준을 fixture 및 evidence README에 고정한다.
2. `ComputedGridTracks.has_fixed_repeat`만으로는 교차축 예외의 group 수와 fixed pattern을 구분할 수 없으므로 **private cascade 결과**에 group count와 bounded-`LengthPx` eligibility를 보존한다. Positive fixed-axis track list는 바깥 ordinary track 없이 repeat group 하나이고, pattern은 computed finite nonnegative `LengthPx` 1–2개다. Auto-axis list는 prefix/suffix ordinary size 없이 auto-repeat group 하나만 두고, 그 pattern은 computed finite nonnegative `LengthPx` track 하나로 제한한다. Stylo가 `calc(absolute)`을 `LengthPx`로 정규화하면 허용하되 percentage/cyclic `calc`, `minmax`, flex, intrinsic pattern은 거부한다. 교차축 예외는 fixed 축 group 정확히 1개, auto 축 group 0개, 전체 auto-repeat 정확히 1개, areas 존재 조건에서만 열고 공개 layout 타입은 확장하지 않는다. 반대 축 두 그룹, auto 축에 fixed group 또는 ordinary prefix/suffix 혼입, `grid-template-areas` 없는 조합, 양축 auto-repeat + fixed group, 비대상 profile은 거부한다.
3. Cascade positive/negative 회귀와 projection 회귀를 추가한다. fixed 축 area dimension이 expanded fixed track 수와 같을 때만 positive 처리하고, 양 방향의 더 작거나 큰 dimension mismatch는 node/property를 포함해 거부한다. auto 축의 area expansion, line/area metadata 한도, Spinon 소유 reservation 실패 후 cascade 회복을 확인한다. 기존 #545 dual-auto positive와 #604 same-axis mixed positive, 무-area cross-axis negative 및 양축 auto + fixed negative를 함께 회귀한다. Taffy 알고리즘을 중복 구현하지 않는다.
4. opt-in runtime GPU fixture registry `crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/template_area_auto_repeat_scenes.json` 및 dispatch `c11_4.rs`, Android `C0410RuntimeGpuDemo.java`와 iOS `C0410RuntimeGpuDemo.swift`의 scene allowlist/expected adapters를 확장한다. `tools/css-reference/fixture-module-boundary.test.mjs`도 갱신한다. 현재 [simulator capture script](../tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs)와 [fixture scene schema test](../crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_template_area_auto_repeat.rs)는 template-area JSON 29개와 전체 fixture 473개 scene 수 및 일반 scene 2-item/track-array 형태를 고정하므로, 해당 scene 수를 각각 40과 484로 갱신하고 schema dispatch를 확장해 새 0/1/2-item scenes를 허용한다. scene마다 root/frame/track/trace/draw/present를 확인한다. overflow scene의 Android PixelCopy 및 iOS Metal 표본은 source coordinates와 연결하고, renderer가 그리지 않는 경계 픽셀을 통과 처리하지 않는다.
5. Android API 37 ARM64 AVD 및 iPhone 17 Pro / iOS 26.2 Simulator에서 새 `template-area-repeat-cross-axis-` ID 11개만 `--scene-id=<comma-separated IDs>`로 선택해 실행한다. 484개 전체 고정 수는 corpus 통합 검증에 유지하고, simulator 실행 결과는 신규 11개로 분리해 기록한다. 현재 실행 환경과 source/app hash, run별 원본 로그 및 화면을 증거에 남긴다. 결과는 simulator 경로에 한정하며 실기기·hardware GPU·iOS compositor 제출 이후 pixels로 확대 해석하지 않는다.
6. focused Rust tests, `cargo fmt --all -- --check`, workspace all-target Clippy, fixture FFI tests 및 기본/opt-in artifact 경계를 각각 실행한다. 기본 artifact에 fixture symbol, scene marker, payload가 없고 opt-in positive control에는 있는지 증명한다.
7. 계획 검토는 구현/PR 변경 검토와 겹치지 않는 실패 관점 20개로 한다. 구현/runtime 검토는 계획 검토와 겹치지 않는 20개 관점으로 새로 수행한다. 사용자 요청에 따라 각 PR diff도 독립적인 실패 관점 20개로 검토하고, 발견을 수정한 뒤 해당 관점을 재검토한다. 별도 계약·문서 변화와 한계를 PR 양식에 맞춰 기록한다.
8. PR의 live checks, 보호 규칙, merge result를 확인한 뒤 Rebase and merge한다. 병합 후 main을 재확인한다. 화면 첨부가 가능하면 Android/iOS scene screenshot을 PR 본문에 포함한다.

## 제외 범위

일반 CSS admission, fixed-axis area dimension을 expanded list와 다르게 허용하는 동작, 임의 fixed-repeat group 수, 여러 auto-repeat의 동시 반복 수 제한 변경, fixed-repeat pattern의 intrinsic/percentage breadth와 `LengthPx`로 정규화되지 않는 `calc()` breadth, `subgrid`, arbitrary placement, writing mode/RTL, WPT 전체 적합성, 실기기 GPU와 compositor 이후 결과는 이 slice 밖이다. 구현 PR에서는 `spec/STATUS.md`, 내부 계약 0059, `spec/0001-conformance.md`, public roadmap과 runtime evidence를 각각 갱신하되 계약 `0.1.0`, C11.4 checkbox 및 공식 `72/213`을 유지한다.

### 기준 문서

- [CSS Grid Layout Module Level 1 2025 CRD §7.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#explicit-grid)
- [CSS Grid Layout Module Level 1 2025 CRD §7.2.3](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat)
- [CSS Grid Layout Module Level 1 2025 CRD §7.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#implicitly-assigned-line-names)
- [CSS Grid Layout Module Level 1 2025 CRD §7.4](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#explicit-grid-shorthand-the-grid-template-property)
