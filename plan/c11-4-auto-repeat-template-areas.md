# C11.4 · auto-repeat과 `grid-template-areas` line 결합 계획

**상위 계획:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [C11.3 area 구현](./c11-3-template-repeat-named-lines-areas.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**상태:** Chrome 156의 13개 장면 사전 비교와 계획 실패 관점 20개 검토를 마쳤고 계획 PR #538이 병합됐다. 구현 branch `feat/c11-4-template-areas-auto-repeat-20261023`에서 typed area metadata 전달, focused 회귀, 13개 scene의 Android/iOS simulator 실행과 FFI static archive 경계 대조를 마쳤다. 구현/runtime 검토 20개를 기록했고 구현 PR [#539](https://github.com/ohah/spinon/pull/539)을 열었다. PR diff 검토 30개, Agent Browser screenshot 확인, merge와 Tailnet roadmap 반영은 진행 중이다. C11.4 미완료 checkbox와 공식 `77/213`을 유지한다.

## 결손

C11.3은 `grid-template-areas`의 owned rectangle과 `name-start`/`name-end` aliases를 보존하고, C11.4는 auto-repeat line name과 named placement를 반복 수 결정 뒤 Taffy로 넘긴다. 두 기능을 함께 쓰는 조합은 현재 fail-closed다.

차단점은 한 곳에 한정되지 않는다.

- cascade [`incremental/grid.rs`](../crates/spinon-style/src/stylo_dom/cascade/incremental/grid.rs)는 template areas와 auto-repeat이 함께 있으면 거부한다.
- style-to-layout [`grid_values.rs`](../crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs)는 조합을 거부하고, area 차원을 반복 확장 전 authored track vector 길이와 같아야 한다고 검사한다.
- auto-repeat item projection [`grid_placement_values.rs`](../crates/spinon-style-to-layout/src/projection/style_values/grid_placement_values.rs)는 named placement를 반복 수에 따라 달라지는 symbolic 값으로 Taffy에 넘기지만 area rectangle을 전달하지 않는다.
- layout의 [`CompactTaffyStyle`](../crates/spinon-layout/src/compact_taffy_style.rs)는 `GridContainerStyle::grid_template_areas()`에서 항상 `None`을 반환한다. 따라서 vendored Taffy 0.15.0의 expanded-repeat named resolver가 area에서 만든 alias를 합칠 입력이 없다.

Vendored Taffy 0.15.0 `NamedLineResolver::new`는 auto-repeat 개수를 받은 뒤 반복 line metadata와 `GridTemplateAreas`를 함께 읽는다. Taffy는 explicit track 수를 `max(expanded template count, area dimension)`으로 정한다. Area가 template보다 큰 경우 추가 explicit grid track은 `grid-auto-*` sizing function을 사용한다. 따라서 `DetailedGridInfo.columns/rows.explicit_tracks`는 최종 count를 확인할 수 있지만 area가 template보다 큰 입력을 거부하는 근거로 쓸 수 없다. 계획은 repeat-count/line lookup을 Spinon에서 재구현하지 않고 area metadata만 Taffy style 경계까지 보존한다. Taffy의 area line-name collision, added area-track sizing 및 fixed boundary behavior는 현재 fixture로 통과했다고 간주하지 않으며 새 focused tests에서 pinned source와 Chrome을 대조한다.

## 규범과 고정 비교 기준

독립 계산, Chrome 관측, 차이, screenshot, JSON 및 SHA-256은 [사전 비교 근거](../spec/internal/evidence/c11-4-auto-repeat-template-area-lines-precomparison-2026-10-23/README.md)에 고정했다. 핵심 기준은 CSS Grid 2025 CRD [§7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat), [§7.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#implicit-named-lines), [§8.3–§8.4](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#line-placement)다.

고정 환경은 Chrome for Testing `156.0.8078.48` (macOS ARM64), Agent Browser `0.31.1`, viewport `1280×633 CSS px`, DPR 1, `ltr`, `horizontal-tb`다. Chrome의 `auto-fit` suffix에서 4px extra gutter가 남는 차이는 areas 없는 numeric controls에서도 반복됐다. 이 scene의 normative pass expectation은 Chrome 값 `48px`가 아니라 gutter collapse 후 `44px`이며, 모든 repeated track이 비었을 때도 normative suffix 위치는 `20px`다.

## 계획된 구현 경계

- 기존 opt-in `RuntimeBlockFlexGridTrackSizingV1`의 fixed-minimum repeat subset만 대상으로 한다. auto-repeat은 한 축의 한 track list에 하나만 둔다. 이 slice에서 다른 축은 auto-repeat을 쓰지 않는다.
- `auto-fill`과 `auto-fit`, 한 개의 fixed `20px` repeated track, 앞뒤 fixed `20px` neighbor, `4px` gap, definite `100px`/`116px`/`140px` container를 사용한다. CSS track grammar, fill-count 산식과 repeat expansion은 기존 Taffy 0.15.0 경로를 재사용한다.
- `grid-template-areas` rectangle을 layout-owned typed metadata로 전달한다. 변환은 0-based half-open cell bounds를 유지하고, Taffy `u16` count/coordinate 한계는 checked conversion으로 확인한다. Spinon 소유의 문자열·vector는 모두 fallible reservation을 사용한다.
- 반복 축의 `hero-start`/`hero-end`, bare area name placement와 generated/authored line-name 충돌은 반복 수가 알려지기 전에 숫자로 투영하지 않는다. area rectangle과 named placements를 Taffy `GridContainerStyle` 경계까지 함께 전달해 expanded-grid resolver가 계산한다. 실패 시 partial `LayoutOutput`을 게시하지 않는다.
- no-repeat C11.3 template-area projection, 기존 C11.4 numeric placement, auto-repeat name/span, integer `repeat()`과 line group ordering은 유지한다. 일반 C11.3 area dimension equality를 바꾸지 않는다. C11.4에서 repeat이 있는 축의 area dimension은 authored cell 수를 유지한다. area가 expanded template보다 크면 Taffy가 더 큰 explicit count를 사용하며, 추가 track은 제한된 `grid-auto-columns`/`grid-auto-rows` 입력으로 크기를 정한다.
- auto-fit의 빈 반복 track은 실제 in-flow item occupancy로 판정한다. 정의만 된 area는 track을 점유하지 않는다. collapsed track 양쪽 gutter는 규범에 따라 collapse한다.
- 여러 auto-repeat, columns와 rows의 동시 auto-repeat+areas, subgrid, `grid-auto-*`가 아닌 area-added track sizing, intrinsic/percentage track, RTL/writing mode, arbitrary CSS grammar, public CSS admission, WPT는 이 계획의 acceptance 대상이 아니다. area가 expanded template보다 큰 양성 장면은 fixed `grid-auto-* : 20px`로 포함한다. area dimension이 Taffy 0.15.0 grid coordinate limit을 넘는 입력은 layout publish 전에 fail-closed 처리한다. `DetailedGridInfo.explicit_tracks`는 결과 count 대조에만 사용하며 template count와 area dimension을 분리하거나 area 확장을 거부하는 데 쓰지 않는다. 이 경계를 위해 Spinon에서 auto-repeat count 산식을 복제하지 않는다.

## 고정 scene과 판정

입력의 canonical CSS·DOM은 [fixture](../tests/fixtures/css/c11/grid-auto-repeat-template-areas-c114.html)다. 사전 비교 13개 scene 모두 실제 Agent Browser observation과 독립 기대를 함께 기록한다. Runtime acceptance도 이 scene ID를 유지한다. root 기준 frame은 `[x,y,width,height]` CSS px이며, area/template used track list·repeat count와 모든 visible child frame을 검사한다.

| 장면 묶음 | scene ID | 고정 계약 |
| --- | --- | --- |
| `auto-fill` area aliases | `columns-fill`, `rows-fill` | 반복 3개가 area `hero` 3-track span에 걸린다. 열/행 frame은 규범 계산과 Chrome 관측에 일치한다. |
| `auto-fit` area aliases | `columns-fit`, `rows-fit` | 반복 4개 중 첫 track만 occupied, 나머지 3개는 0px. suffix frame은 Chrome보다 4px 앞선 규범 gutter 위치여야 한다. |
| area matrix가 expanded grid보다 짧음 | `columns-area-shorter`, `rows-area-shorter` | repeat은 4개지만 area는 5 cell로 고정된다. `hero`·`suffix` alias는 area cell index를 따르고 마지막 fixed track은 numeric tail control이 검사한다. |
| area matrix가 expanded template보다 큼 | `columns-area-larger`, `rows-area-larger` | 100px container에서는 auto-fill 2개가 들어가 template은 4 tracks다. 5-cell area는 `grid-auto-* : 20px` 추가 explicit track을 만든다. suffix는 root 기준 96px부터 20px를 차지해 16px overflow한다. |
| authored alias collision | `columns-area-line-collision` | repeat 경계 바깥/안쪽의 동일 `hero-start`와 area-generated line을 함께 보존한다. bare `grid-area: hero`와 `hero-start 2 / hero-end 1`의 서로 다른 expected frame을 확인한다. |
| area만 있고 repeated item 없음 | `columns-fit-empty-area`, `rows-fit-empty-area` | `hero` cell 이름은 남아도 repeated track 4개는 모두 collapse한다. suffix는 첫 fixed neighbor 바로 뒤에 온다. |
| 비-area control | `columns-fit-numeric`, `rows-fit-numeric` | 같은 auto-fit geometry에서 numeric line만 사용한다. Chrome의 4px gutter 차이가 feature가 아니라는 점과 numeric 회귀를 분리한다. |

Chrome float frame과 used track은 `1/64 CSS px` 이하로 비교한다. CSSOM named alias/area name, repeat count, track count, zero-track/gutter collapse와 node identity는 정확히 비교한다. Chrome과 규범 기대가 다른 `auto-fit` suffix는 반드시 독립 normative 값을 사용한다. 사용자 화면에서는 track/box geometry와 draw/present를 확인하고, 실제 sample point가 연결된 내부·edge·바깥 surface만 색 검증으로 보고한다. renderer가 CSS border stroke를 실제로 그리지 않으면 border stroke 통과를 쓰지 않는다.

## 실행 및 검증 절차

1. 계획 PR을 Rebase and merge하고 최신 `origin/main`에서 구현 branch를 만든다. 동일 checkout만 사용하고 기존 미추적 evidence·tmux·Tailnet 서버를 보존한다.
2. 변경 직전 Stylo cascade → computed snapshot → style projection → `GridContainerInput` → Taffy `CompactTaffyStyle` → `NamedLineResolver` data flow를 다시 추적한다. Taffy auto-repeat count 후 areas와 explicit line names를 합치는 순서를 pinned source와 직접 테스트한다. `DetailedGridInfo.columns/rows.explicit_tracks`는 결과 count 확인에 사용하고, area가 template보다 큰 positive track과 `grid-auto-*` sizing을 테스트한다. Taffy grid coordinate limit 초과는 layout publish 전에 fail-closed인지 별도 확인한다.
3. area metadata의 layout-owned type, checked `usize`→`u16` bounds, malformed rectangle, NUL/빈 이름, duplicate names, reservation failure를 정한다. `ComputedCssGridTemplateAreas`를 복사하거나 재구성하는 새 allocation은 failpoint가 확인되기 전 구현을 진행하지 않는다.
4. cascade/projection/layout focused tests에 profile gate, one-axis auto-repeat+areas 성공, two-axis/repeat-count 제외 control, area dimension이 template보다 작음/같음/큼, added `grid-auto-*` track sizing, line alias collision, bare area shorthand, missing/invalid area, auto-fit area-without-item collapse와 existing no-area numeric regression을 고정한다.
5. fixture 13개 scene의 repeat count, area rect/line map, expanded/used Columns·Rows, 각 node frame과 stage trace를 비교한다. area-larger scene은 template 4 tracks + area-added explicit track 1개와 16px overflow를 확인한다. `auto-fit` gutters는 normative expectation 44px/20px와 Chrome diagnostic 48px/24px를 분리해 fixture JSON에 둔다.
6. 현재 source와 설치 artifact SHA를 연결해 Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 13/13 scene을 실행한다. runtime 화면에는 식별 가능한 scene label/색, root 경계 안·밖과 item/gutter sample을 둔다. Android AVD PixelCopy 및 iOS compositor 제출 전 Metal drawable의 실제 읽은 좌표만 판정한다. 실기기/hardware GPU·OS compositor 이후 화면을 주장하지 않는다.
7. 기본/product와 전용 `c04-runtime-gpu-fixture-test` archive의 fixture ABI/payload 부정·긍정 대조, fixture module boundary, focused tests, `cargo fmt --all -- --check`, workspace all-target Clippy, Android/iOS simulator app build를 각각 기록한다. 실제 실행하지 않은 검사는 pass로 쓰지 않는다.
8. 계획 검토와 관점이 겹치지 않는 implementation/runtime 20개를 새로 검토한다. 최종 구현 PR diff는 30개의 독립 실패 관점으로 검토한다. 발견 결함은 수정한 뒤 영향을 받은 관점을 재확인한다.
9. 구현 PR은 `.github/pull_request_template.md`에 맞춰 상태 ID·명세·oracle·실행 화면·artifact hash·한계를 기록한다. 가능한 simulator 화면을 본문에 올리고 Agent Browser에서 GitHub 이미지 링크가 실제로 표시되는지 확인한 뒤 세션/process를 종료한다.
10. Tailnet preview는 문서 preview 경로를 확인한 뒤 hash-manifest 방식으로 선택된 결과만 sync하고 HTTP 응답/hash를 검증한다. Agent Browser로 roadmap·화면 링크를 확인한 후 브라우저를 닫는다. 최신 GitHub required checks와 merge state를 확인하고 PR을 Rebase and merge한다.

## 구현 진행 기록

- [x] 계획 PR #538 병합, cascade → projection → layout → Taffy data flow 추적, one-axis repeat과 fixed-axis dimension gate 확인.
- [x] layout-owned half-open area type, `u16` bounds, duplicate/name/rectangle/metadata 한도 검증과 fallible reservation 연결. malformed input은 layout 결과를 게시하기 전에 거부한다.
- [x] cascade, style projection, layout의 focused 회귀와 fixture module/artifact 경계 검사를 추가했다. Rust focused test는 layout 238, style-to-layout 192, style 221개가 통과했다.
- [x] 최종 source 기준 Android API 37 ARM64 AVD `emulator-5558` 및 iPhone 17 Pro / iOS 26.2 Simulator에서 각각 13/13 scene의 root/child frame, used tracks, auto-fit collapse, trace, draw/present를 확인했다. PixelCopy 및 compositor 제출 전 Metal drawable에서 각각 66/66 고정 표본이 exact였다.
- [x] `cargo fmt`, workspace all-target Clippy, Android/iOS simulator app builds 및 `c04-runtime-gpu` 기본 FFI archive와 `c04-runtime-gpu-fixture-test` positive control을 확인했다. [archive 경계와 실행 근거](../spec/internal/evidence/c11-4-template-area-auto-repeat-runtime-2026-10-23/README.md)에 기록했다. 최종 APK/IPA 전체에서 fixture payload가 빠지는지는 검증하지 않았다.
- [x] 계획과 다른 실패 관점 20개로 implementation/runtime을 새로 검토했다. [검토 결과](../spec/internal/evidence/c11-4-template-area-auto-repeat-runtime-2026-10-23/implementation-runtime-review-20.md).
- [ ] PR diff 실패 관점 30개 검토, GitHub screenshot link 확인, merge를 진행한다.
- [ ] 병합 후 Tailnet roadmap을 선택 파일 hash manifest로 갱신하고 Agent Browser에서 roadmap·runtime 화면 링크를 확인한 뒤 Agent Browser session/process를 닫는다.

Android runtime의 `visibility:hidden` 장면은 현재 runtime CSS parser가 해당 선언을 지원하지 않아 matrix에서 제외했다. authored alias 충돌은 화면상 동일 색의 겹친 box로 두고 DOM node frame과 track trace로 검증한다. 이는 visibility cascade 동작 검증이 아니다. Chrome 156 auto-fit suffix의 4px 관측 차이는 기존 사전 비교와 같이 규범 기대를 판정값으로 쓰고 Chrome 값은 관측으로만 둔다.

## 계획 검토

- [계획 실패 관점 검토 20개](./c11-4-auto-repeat-template-areas-plan-review-20.md)
- [Chrome 156 사전 비교](../spec/internal/evidence/c11-4-auto-repeat-template-area-lines-precomparison-2026-10-23/README.md)
- [계획 PR diff 적대 검토 30개](../spec/internal/evidence/c11-4-auto-repeat-template-area-lines-precomparison-2026-10-23/plan-pr-diff-review-30.md)

## 완료 해석

이 제한 조합이 병합돼도 C11.4와 C11 전체를 완료 처리하지 않는다. 공개 CSS admission, WPT conformance 및 internal contract numeric version `0.1.0`은 이 계획에서 승격하지 않는다. 공식 C11.4 checkbox와 `77/213`은 미완료로 남는다.
