# C11.3.3 · 존재하지 않는 이름의 암시적 Grid 라인·트랙

**상태:** 제한 구현 후보 [PR #393](https://github.com/ohah/spinon/pull/393)은 2026-10-20T15:47:50Z Rebase and merge됐으며 merge SHA는 `12dc2669814c4d5e7d0e2aa59d5196fdab98bb68`이다. 필수 checks는 보고되지 않았고 PR check rollup은 비어 있어 CI 통과로 기록하지 않는다. 계획 검토와 구현·시뮬레이터 검증은 기록되어 있지만, 이 문서와 fixture는 공개 CSS 지원 선언이나 C11.3.3 완료 근거가 아니다.
**상위:** [C11.3 template areas 계획](./c11-3-template-repeat-named-lines-areas.md) · [C11 상태 대장](../spec/STATUS.md#c11-css-grid) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)
**규범:** [CSS Grid Layout Module Level 1 · 2025-03-26 CRD §8.3](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#line-placement), §7.5, §8.3.1

## 목표

기존 C11.3.2/3 profile은 authored line-name과 template-area alias가 explicit grid 안에 있으면 placement로 변환한다. 이름이 없는 경우에는 `UnsupportedNamedGridLine`으로 끝낸다. CSS Grid §8.3은 이름이 붙은 line을 찾기에 explicit line이 모자라면 배치 검색 방향의 implicit line들을 그 이름으로 간주한다. 이 계획은 그 가운데 **explicit grid 뒤쪽으로만 확장되는 양의 implicit line·track subset**을 구현한다.

고정 WPT 원문은 `css/css-grid/placement/grid-placement-unknown-named-area-001.html`이다. WPT revision은 `c999c58338ee1d223df5ad62e48be1202ced0d37`, 원문 SHA-256은 `7fa795ad262e74399487fa817da1e99a75bbcce2e46b5f4547d55959e2e9ebd4`다. 원문은 두 bare-ident endpoint, 명시된 opposite line과 named span, 양 축의 missing name을 검사하고 기대 geometry를 `data-offset-*`에 둔다. 이 파일은 WPT 원문 전체 실행을 뜻하지 않으며, 별도 Spinon fixture는 원문 규칙을 재현한다.

## 범위와 경계

- C11.3.3 전용 `compute_runtime_block_flex_grid_template_implicit_named_lines_style_layout` opt-in 경로에서 explicit line-name이나 template area/area alias와 일치하지 않는 bare `<custom-ident>`는 해당 축의 첫 trailing implicit named line으로 해석한다. positive line 번호는 explicit track 수보다 2 큰 값을 사용한다. explicit 마지막 경계 line은 `track_count + 1`이므로 implicit track 앞의 line을 생략하지 않는다. 기존 `compute_runtime_block_flex_grid_template_style_layout`은 계속 missing name을 거부해 C11.3.1–3.2 contract를 유지한다.
- opposite definite line 뒤에서 named span을 검색하고 explicit matching line이 모자라면 endward implicit line을 생성해 지정 span을 채운다. WPT의 `grid-row: missing / span 5 absent`와 `grid-column: missing / span 3 absent` 형태를 포함한다.
- `grid-row`/`grid-column` 양 축, 기본 row flow, 양의 implicit tracks만 대상으로 한다. explicit template track과 `grid-auto-rows`/`grid-auto-columns`는 유한한 고정 CSS px subset만 허용한다. implicit track 합은 layout의 기존 `GRID_MAX_TRACKS` 및 Taffy line 범위 안이어야 한다.
- 이름은 case-sensitive다. explicit authored name, area edge, implicit area alias가 있으면 기존 해석이 먼저이며 missing-name fallback이 이를 덮지 않는다. 기존 definite area sibling과 같은 grid에 있을 때도 해당 area placement는 보존한다.
- 구현 경계는 `GridPlacementMode::TemplateGridImplicitTracks`로 분리한다. 새 C11.3.3 opt-in resolver가 missing named line을 실제 positive implicit line으로 만든 경우에만 선택하고, 기존 C11.3.1–3.2 wrapper나 일반 numeric line 입력만으로는 선택하지 않는다. TemplateGrid의 고정 track/item size allowlist를 유지하면서 trailing extent 확장만 허용한다. CSS profile 밖의 `grid-auto-*` track list, `auto`, intrinsic track 또는 negative leading extent는 계속 거부한다.
- 이 변경만으로 일반 numeric positive line, named occurrence 부족, leading/negative implicit grid, `grid-auto-*:auto`, implicit track list, auto-repeat·intrinsic sizing, auto-placement 혼합, writing mode/RTL, subgrid/masonry 또는 전체 WPT 적합성을 지원한다고 주장하지 않는다. 기존 `unknown-area-implicit-tracks` 자동 implicit-track 입력은 고정 `px`로 좁혀 지원하는 신규 case와 분리해 두며, 자동 implicit sizing은 기대 거부로 남긴다.

## 사전 비교 모델

- 원문과 별도로 `tests/fixtures/css/c11/unknown-named-area-implicit-lines-c1133.html` 및 inventory를 둔다. case는 WPT 네 subcase와 template-area `grid-area` bare-ident/mixed explicit-area sibling control이다. 모든 root는 `160×120 CSS px`, `320×240 CSS px` viewport, DPR 1·2, LTR·`horizontal-tb`, zero gap, fixed explicit/auto tracks로 고정한다.
- 규범·WPT의 WPT 네 subcase 기대 item rect는 root-relative `(x,y,width,height)` 순서로 `(0,35,30,5)`, `(85,0,45,10)`, `(0,35,30,25)`, `(85,0,15,10)`이다. 각 subcase에는 explicit `(row 1, column 1)` control을 함께 둔다. 추가 area control은 Chrome 캡처 전에 예상값을 고정하고, WPT 값과 혼합하지 않는다.
- 구현 전 기준은 Chrome for Testing `154.0.8037.92` mac-arm64, CDP revision `@334b65d254ccc35df4fca82706d1753227b01039`다. Chrome 실행 파일 hash, helper/tool hash, fixture/inventory hash, CSSOM resolved track values, 전체 node rect, DPR 1·2 PNG를 [C11.3.3 unknown-area precomparison evidence](../spec/internal/evidence/c11-3-3-unknown-named-area-precomparison-2026-10-20/README.md)에 기록한다. Captured Chrome 결과가 위 네 WPT 좌표와 다르면 implementation 전에 계획 및 기대값을 수정하고 차이를 설명한다.
- 2026-10-20 capture `c1133-unknown-name-chrome-20261020T141317806Z-791f700e`를 완료했다. 고정 Chrome 154 결과는 WPT 네 rect `(0,35,30,5)`, `(85,0,45,10)`, `(0,35,30,25)`, `(85,0,15,10)`와 일치했다. template area scene도 Chrome의 rows `[60,60,5,5]`, columns `[80,80,15,15]`, unknown item rect `(175,125,15,5)`를 보였다. fixture 전체 hash, browser 실행 파일 SHA-256 `ce0c09ea98a7592e7799e54c1867797ccf99d4d6e821a11067c332e454732349`, 10개 PNG와 각 hash는 versioned reference에 보존했다.
- 요청에 따라 WPT 원문 페이지를 Agent Browser/Chrome 155에서 먼저 확인했다. 보이는 네 item geometry는 원문의 기대값과 맞았고 CSSOM은 row case `10px 20px 5px 5px`, row span case `10px 20px 5px 5px 5px 5px 5px 5px`, column span case `30px 40px 15px 15px 15px 15px` 등을 보였다. 이는 탐색 관찰이며 위의 pinned Chrome 154 사전 비교를 대체하지 않는다. Agent Browser session은 확인 후 닫았다.
- 구현 전 사전 비교 pass 기준은 CSSOM explicit+implicit track 순서, 각 item/root rect, WPT 네 geometry, area sibling identity, DPR 독립성이다. Geometry 허용치는 축별 `0.5 CSS px`; 오류 case는 정확한 node/property/error와 layout·frame·draw 결과 부재를 확인한다.
- `unknown-template-area-with-sibling`은 root box `(0,0,160,120)` 밖으로 item box `(175,125,15,5)`가 나간다. Chrome PNG에서 root의 dark background 끝과 item의 red pixels가 분리되어 보이는지 확인했다. Runtime PixelCopy/Metal comparison은 grid 내부 `(150,110)`의 `#101820`, 알려진 area sibling `(10,10)`의 `#30a46c`, root 밖 red item `(182,127)`의 `#e5484d` 세 표본을 비교한다. Chrome의 미커버 viewport 배경 `(170,127)`은 Chrome white와 host-owned native surface backdrop의 차이여서 별도 제외 표본으로 기록하고 이 Grid 비교에서는 판정하지 않는다. Simulator에서 item box와 픽셀이 clipping 없이 보이는 것은 검증 대상이다. 이 입력은 border stroke를 그리지 않으므로 border paint 보증으로 해석하지 않는다.

## 구현 및 검증 계획

1. pinned Chrome 154 capture와 plan review 20개가 구현 전에 통과했다. 이 진입 gate는 충족했으며, 이후 변경은 별도 구현 실패 review에서 검토한다.
2. projection은 area alias와 authored line-name 해석을 먼저 보존하고, 이름이 전혀 없는 bare-ident 및 지원 방향의 named span 부족만 trailing implicit line으로 해석한다. line arithmetic와 span 합은 checked conversion을 사용한다.
3. 기존 template layout entry point와 별도인 C11.3.3 opt-in entry point를 추가한다. `TemplateGridImplicitTracks` mode는 opt-in named fallback으로 생성된 implicit placement에만 선택한다. 기존 C11.3.1–3.2 wrapper에서의 missing-name 및 일반 numeric explicit placement는 기존처럼 거부하고, 새 implicit extent의 row/column 개수·track allocation·overflow 오류가 원자적으로 검증되어야 한다.
4. Rust는 resolver, area/name precedence, row/column symmetry, named-span shortage, Taffy normalization, 0/negative/overflow/cap, numeric line·span이 named mode를 빌려 extent를 늘리는지, 양 축의 fixed `grid-auto-*`, error atomicity, 새 C11.3.3 wrapper success와 기존 C11.3.2 wrapper missing-name rejection을 검사한다.
5. Runtime fixture adapter는 기존 opt-in fixture-test feature 경계를 유지한다. 고정 scene positive/expected rejection, geometry, no partial draw, root 밖 visible item sample을 확인한다. Android API 37 ARM64 AVD PixelCopy와 iPhone 17 Pro/iOS 26.2 Simulator Metal drawable에서 grid interior `(150,110)`, known-area sibling `(10,10)`, overflow item `(182,127)` 세 표본을 비교한다. Chrome page-backdrop `(170,127)` 표본은 별도 기록하며 native surface의 host-owned 배경 차이로 제외한다. 이 결과를 실기기·hardware GPU·compositor 이후 픽셀 근거로 확대하지 않는다.
6. 구현 후 계획 검토와 다른 20개 구현 실패 관점을 기록한다. 이후 독립적인 PR diff 적대 검토 20개를 수행한다(이번 로드맵 작업의 사용자 지정 횟수). 결함은 수정 후 영향받은 관점을 다시 확인한다.
7. `spec/internal/0059-c11-grid-layout.md`, `spec/STATUS.md`, `spec/internal/README.md`, public support table 및 `spec/public/roadmap.html`은 실제 통과 범위와 남은 거부를 반영한다. C11.3.3과 C11 전체는 WPT 적합성을 달성하지 않는 한 candidate/incomplete로 둔다.

## 구현 전 계획 실패 관점 20개

Pinned Chrome capture가 완료되면 각 항목에 근거와 PASS/PARTIAL을 기록한다. 같은 검사 문구를 바꿔 반복하지 않는다.

| # | 독립 실패 관점 | 확인 근거 |
|---:|---|---|
| 1 | explicit 마지막 경계 line을 implicit named line으로 잘못 취급해 첫 implicit track을 건너뛴다 | §8.3 line index 계산, Chrome resolved tracks |
| 2 | bare ident start/end가 같은 line일 때 conflict rule 뒤 기본 span이 달라진다 | WPT `bothNamedGridLineRow`/`Column` |
| 3 | start/end longhand 방향에 따라 positive custom-ident occurrence가 다르게 계산된다 | 동일 unknown name 양 endpoint와 row/column case |
| 4 | explicit authored name이 있는데 implicit fallback이 우선되어 placement가 이동한다 | existing named-line regression과 새 explicit-name control |
| 5 | template-area edge alias가 unknown-line fallback에 가려진다 | area/name precedence fixture |
| 6 | implicit area aliases만 정의된 name이 실제 area가 아닌 plain named line로 처리된다 | existing C11.3.3 explicit alias pair case |
| 7 | area matrix와 explicit track count 불일치 뒤에 implicit line을 더해 invalid declaration을 살린다 | existing mismatch cascade/error case |
| 8 | named span이 opposite line과 반대 방향에서 implicit line을 잘못 생성한다 | WPT row/column span; leading 방향은 fail closed |
| 9 | explicit matching lines가 일부 있을 때 span shortage 계산이 explicit match 수를 중복한다 | added explicit-match-plus-implicit span case |
| 10 | `span 0`, `span auto`, reserved custom-ident를 암시적 이름 생성으로 우회한다 | computed parser contract와 malformed input test |
| 11 | `foo 2`/negative occurrence 등 계획 밖 occurrence를 silently generate한다 | out-of-scope rejection regression |
| 12 | `i32`/`i16`/Taffy line boundary overflow가 wrap 또는 panic으로 통과한다 | checked arithmetic, boundary injection |
| 13 | GRID_MAX_TRACKS 초과 요청이 과대 allocation 전에 거부되지 않는다 | layout limit test와 reserve boundary |
| 14 | auto `grid-auto-*`가 zero 또는 intrinsic track으로 묵시 지원된다 | auto-size expected rejection case |
| 15 | leading/negative implicit tracks가 trailing-only 구현 경계로 잘못 수락된다 | negative line and backward-span rejection |
| 16 | 한 axis만 implicit expansion되어 다른 axis frame/track list가 바뀐다 | independent row/column fixtures and CSSOM tracks |
| 17 | 새 mode가 동일 grid의 named-area sibling auto sizing/minimum size를 바꾼다 | mixed area sibling control |
| 18 | 숨김/중첩 item이 extent 계산에 포함되거나 visible sibling identity/order가 바뀐다 | nested/hidden control and node list |
| 19 | invalid sibling 뒤에 부분 placement/frame/draw 결과가 남는다 | failure-atomicity and no-partial output criteria |
| 20 | DPR, screenshot, reference hash, 또는 Agent Browser lifecycle 증거가 stale session에 묶인다 | pinned capture metadata, both DPRs, close/session/process check |


## 구현 후 후보 결과 · 2026-10-21

전용 opt-in 경로와 `TemplateGridImplicitTracks`를 추가했다. unknown area bare-ident 및 trailing named-span shortage에서만 positive implicit named line을 만들며, unmarked numeric line/span extent는 별도 named mode를 빌릴 수 없다. 양쪽 `grid-auto-rows`와 `grid-auto-columns` 모두 고정 px일 것을 요구한다. leading/negative/general numeric implicit line과 auto/intrinsic tracks는 거부한다.

마지막 코드 변경 뒤 Android API 37 ARM64 AVD 행렬 `c1133-matrix-20261020T150911869Z-7abbeeff`, Android 16/API 36 Samsung SM-S731N 실기기 행렬 `c1133-matrix-20261020T152039508Z-5886c27a`, iPhone 17 Pro/iOS 26.2 Simulator 행렬 `c1133-ios-matrix-20261020T151033155Z-4d6bf988`가 각각 26 scene·65 node, 19 success·5 cascade fallback·2 expected rejection, 61 visible frame, geometry 오차 0 CSS px로 통과했다. 별도 root-overflow case의 3개 frame geometry와 표면 RGBA sample은 세 실행에서 각각 3/3 exact다. SM-S731N renderer는 ANGLE/Xclipse 940/Vulkan 1.3.279였다. 실행 hash, 화면과 플랫폼 경계는 [후속 근거](../spec/internal/evidence/c11-3-3-unknown-area-implicit-lines-followup-2026-10-21/README.md)에 있다.

PR #393의 병합 전 후보 snapshot에서 로컬 Rust workspace는 1,193 passed·4 ignored, focused CSS reference 24/24, C11.3.3 fixture FFI 4/4였고 Clippy, rustfmt와 matrix generator check도 통과했다. Full `bun run test`는 당시 기존 C04, C11.3.2, C13.3 source expectation 네 개에서 중단되어 CSS-reference test 이후 benchmark step은 실행되지 않았다. 이 결과는 병합 전 해당 snapshot에 한정된다. PR #393 병합 뒤에도 제한 범위만 구현됐으므로 공식 C11.3.3/C11 상태는 미완료로 유지한다.

## WPT inventory 재분류 · 2026-10-30

고정 Chrome report의 `grid-placement-unknown-named-area-001.html` 네 `.grid` subtest를 이 문서의 양의 trailing-line/line-span 장면 네 개에 이름 기반으로 연결했다. [재분류 근거](../spec/internal/evidence/c11-3-3-wpt-inventory-refresh-2026-10-30/README.md)는 원본 WPT report를 재사용하고 source/hash를 확인한다. Chrome의 4/4 PASS와 파생 fixture 범위만 기록하며 Spinon WPT 실행·적합성으로 표시하지 않는다. `unknown-area-implicit-tracks`의 template-area rejection 및 C11.3.3/C11 미완료 상태는 바꾸지 않는다.
