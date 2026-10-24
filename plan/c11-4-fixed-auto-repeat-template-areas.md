# C11.4 · fixed-repeat + auto-repeat + `grid-template-areas`

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [fixed-repeat named lines](./c11-4-fixed-repeat-line-name-merge.md) · [auto-repeat + areas](./c11-4-auto-repeat-template-areas.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**현재 상태 · 2026-10-25:** 9-scene Chrome 156 precomparison과 CSS Grid 2025 CRD 기반 independent expected values를 고정했고, 계획 실패 관점 20개와 계획 diff 30개 검토를 마쳤다. 제한 구현 후보는 same-axis cascade admission, combined line/area metadata budget, shorthand/axis regressions, Android/iOS fixture adapter와 capture route까지 연결했다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 최종 9/9 scene씩 통과했고, area-larger 표면 표본은 각 플랫폼 8/8 exact다. 구현/runtime 실패 관점 20개와 PR diff 30개 재검토를 마쳤으며, PR 생성·CI·merge는 남았다. C11.4 checkbox와 공식 `72/213`은 바꾸지 않는다.

## 이번 slice에서 해결한 결손과 남은 제한

C11.4의 기존 profile은 fixed-repeat과 auto-repeat의 track/line-name composition 및 auto-repeat + `grid-template-areas`를 별도 지원했다. 이번 후보는 제한 profile에서 같은 axis `grid-template-columns` 또는 `grid-template-rows`의 fixed-repeat + auto-repeat + template areas 조합을 허용하고, combined metadata budget과 두 축 회귀를 연결했다. positive 경로는 [cascade regressions](../crates/spinon-style/src/stylo_dom/cascade/runtime_grid_tests/fixed_repeat_auto_repeat.rs)의 `same_axis_fixed_auto_repeat_areas_preserve_tracks_lines_and_rectangles_on_both_axes` 및 shorthand track-list parity test가 고정한다. 교차축 조합, 두 번째 auto-repeat, unsupported breadth와 공개 CSS admission은 계속 fail-closed한다.

Style-to-layout은 auto-repeat이 있으면 typed template-area rectangles를 `GridContainerInput`으로 투영하고, `spinon-layout`은 그것을 pinned Taffy 0.15.0 `GridContainerStyle`까지 전달한다. 고정된 Taffy의 `NamedLineResolver`는 auto-repeat 수가 계산된 뒤 fixed/auto repeated line names와 area-generated aliases를 함께 읽고, explicit track count를 expanded template과 area dimension 중 큰 값으로 정한다. 이 slice는 해당 경로를 재구현하지 않고 cascade admission 경계를 검증한 뒤 기존 typed metadata를 연결한다.

## 구현 계약

- 적용 profile은 opt-in `RuntimeBlockFlexGridTrackSizingV1`다. 한 track list에 auto-fill 또는 auto-fit 하나와 positive integer fixed-repeat group을 함께 둔다. 각 fixed-repeat pattern은 기존 C11.4 계약의 1–2개 finite, non-negative computed `LengthPx` tracks이며 합계 authored descriptor는 기존 10,000 한도를 넘지 않는다. 이 slice는 기존 복수 fixed-repeat group과 authored line-name metadata를 그대로 보존한다.
- 각 fixture scene에서는 columns 또는 rows 중 한 축만 auto-repeat한다. 반복 축의 앞/뒤 fixed-repeat, template areas, authored/area-generated 동일 이름 line, named positive/negative occurrence를 확인한다. 반대 축에는 repeat을 추가하지 않는다.
- `auto-fill`과 `auto-fit` 각각의 area placement 후 occupancy를 검증한다. 정의된 area만으로는 track이 점유되지 않고, 실제 in-flow item이 span하는 반복 track은 auto-fit collapse에서 제외한다. Empty repeated tracks는 0px이며 양 옆 gutter도 접힌다.
- area dimension이 expanded template보다 큰/작은 경우를 모두 포함한다. 큰 area의 추가 explicit track은 고정된 `grid-auto-columns` 또는 `grid-auto-rows` 값으로 크기를 정하고, root 바깥으로 나오는 item geometry와 표면 표본을 확인한다.
- CSS Grid §7.3.2 및 §8.3대로 같은 물리 line의 authored/implicit duplicate 이름은 occurrence를 중복 증가시키지 않는다. 선행 explicit alias가 있으면 named area shorthand가 선택하는 first matching line을 고정한다. Positive/negative line occurrence의 independent expected result는 [precomparison evidence](../spec/internal/evidence/c11-4-fixed-auto-repeat-areas-precomparison-2026-10-24/README.md)에 있다.
- same-axis fixed-repeat + auto-repeat + template areas만 새로 허용한다. 다른 axis에 fixed-repeat과 auto-repeat을 교차 배치한 경우, 두 번째 auto-repeat, malformed/zero count, unsupported sizing breadth, limit 초과는 계속 fail-closed한다. Existing no-area fixed+auto, auto-repeat-only areas, fixed-repeat line-name, auto-fit placement/collapse regression을 유지한다.
- 공개 CSS admission, arbitrary track grammar, `calc()`·percentage·intrinsic fixed-repeat breadth, arbitrary named-area matrix size beyond existing checked limits, subgrid, additional implicit placement, RTL/writing mode, WPT/전체 C11 적합성은 이 slice의 완료를 뜻하지 않는다. Android 결과는 API 37 ARM64 AVD, iOS 결과는 iPhone 17 Pro/iOS 26.2 Simulator로 한정한다. Android hardware GPU, iOS compositor 제출 이후 pixels는 주장하지 않는다.

## 고정 입력과 독립 oracle

Canonical input은 [9-scene fixture](../tests/fixtures/css/c11/grid-fixed-auto-repeat-template-areas-c114.html)다. Chrome 156 observation과 CSS Grid 2025 CRD에서 계산한 expected values는 [precomparison evidence](../spec/internal/evidence/c11-4-fixed-auto-repeat-areas-precomparison-2026-10-24/README.md)에 있다. 148/168/188px definite size에서 track minimum은 `72px + 24px*n`이며 auto-repeat count는 각각 3/4/4다. Area-template 10/9 또는 8/10 경계, empty/occupied auto-fit, 양·음 occurrence collision을 나눈다. Chrome과 normative expectation이 다른 값이 발견되면 Chrome 수치를 acceptance로 채택하지 않고 새 차이와 규범 계산을 기록한다.

## 구현 및 검증 단계

1. 구현 직전 최신 `main`에서 Stylo specified/computed track list → `ComputedCssGridLayout` → `GridContainerInput` → `CompactTaffyStyle` → Taffy `NamedLineResolver`를 다시 따라가고, fail-closed guard가 대상인 같은 axis만 차단하는지 assertion으로 고정한다.
2. Cascade의 area/fixed+auto rejection을 같은-axis limited profile에서 제거한다. 반대 축 혼합 repeat rejection과 feature profile admission은 유지한다. 기존 projection이 area rectangles와 all line-name groups를 fixed/auto repeat과 함께 전달하는지 독립 style-to-layout test로 확인한다. Alias expansion/counting을 Spinon에서 새로 구현하지 않는다.
3. Cascade/style projection/layout regressions에 9개 Chrome scene의 성공 조건, area-matrix smaller/equal/larger, auto-fit occupancy, generated/authored line collision, positive/negative occurrence, rows/columns transpose, repeated seam duplicate, overflow geometry를 연결한다. Longhand와 `grid-template`/`grid` shorthand의 track-list branch에 area를 별도 longhand로 더했을 때 같은 typed metadata가 나오는지 확인하고, 같은 node의 area→none→area style revision에서 이전 alias나 rectangle이 남지 않는지 검사한다. Area-string shorthand branch 안의 track listing은 CSS Grid §7.4가 `repeat()`를 허용하지 않으므로 이 조합의 지원 범위로 세지 않는다 ([W3C CSS Grid 2025 CRD §7.4](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#explicit-grid-shorthand-the-grid-template-property)). Combined line/area metadata의 exact/over 1 MiB budget, 개별 reservation failure 및 동일 engine recovery도 유지한다. Cross-axis rejection, second auto-repeat, bad count, unsupported breadth, descriptor/metadata boundary와 allocation fault/recovery는 negative control로 유지한다.
4. Opt-in fixture harness가 각 scene의 `grid-template-areas`, repeat mode/count, expanded/used tracks, per-node frame, named placement, sizing trace와 draw/present를 검증한다. Root 안·경계·바깥 표면 sample을 실제 source coordinates에 연결하고 변조 좌표는 readback 전에 거부한다.
5. 최신 implementation source와 installed app SHA를 기록하고 Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 9/9 scene을 확인한다. `columns-area-larger`/`rows-area-larger`는 root 끝 바깥의 blue item sample과 item 끝 바깥의 surface-clear sample을 검사한다. root CSS border stroke가 renderer에 없으면 border pixel 통과를 주장하지 않는다.
6. Fixture boundary/module tests와 default/product vs `c04-runtime-gpu-fixture-test` archive를 비교한다. 기본 artifact에 신규 scene marker, FFI ABI, fixture payload가 없고 opt-in positive control에 포함되는지 확인한다. Rust focused tests, `cargo fmt --all -- --check`, workspace all-target Clippy, Android/iOS simulator app builds를 각각 실행하고 실행 안 한 항목을 pass로 쓰지 않는다.
7. 계획 검토와 겹치지 않는 구현/runtime 실패 관점 20개를 별도 검토한다. PR diff는 30개의 서로 다른 실패 경로로 검토하고 수정된 관점은 재검증한다. PR 본문은 `.github/pull_request_template.md`에 따라 한글로 작성하고 Android/iOS 동작 화면을 가능한 한 직접 표시한다.
8. PR live state, required checks와 merge result를 확인한 뒤 Rebase and merge한다. Tailnet roadmap preview의 기존 파일·서비스를 보존하고 선택 경로만 backup/hash/HTTP 검증으로 동기화한다. Agent Browser에서 roadmap·screenshots를 확인한 뒤 이번 세션의 브라우저 프로세스를 종료한다.

## 계획 단계 acceptance

- Chrome observation은 고정된 9개 scene과 30개 child frame을 포함한다. Independent calculator/validator는 `1/64 CSS px` 허용치 안에서 geometry를 비교한다.
- CSSOM track vector, fixed/auto boundaries, area dimension, generated/authored alias line indices, empty/occupied auto-fit track widths와 148px overflow는 CSS Grid 2025 CRD에서 독립 산출한다.
- [계획 실패 관점 검토 20개](./c11-4-fixed-auto-repeat-template-areas-plan-review-20.md)와 계획 PR diff 30회는 서로 다른 관점의 문서로 남긴다.

이 계획만 병합해도 C11.3.3, C11.4 및 상위 C11은 미완료다. 공개 완료 수 `72/213`과 내부 계약 0059의 `0.1.0`을 유지한다.
