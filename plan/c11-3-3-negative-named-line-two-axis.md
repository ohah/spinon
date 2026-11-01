# C11.3.3 양 축 negative named-line pair의 leading implicit sizing

**상태:** Chrome 156 사전 비교 8 scene과 독립 CSS Grid 기대값을 고정했다. 구현 후보는 양 축 전용 typed mode로 연결했고, 첫 PR 검토에서 발견한 auto-size fallback 우회를 고친 뒤 Android API 37 AVD와 iPhone 17 Pro/iOS 26.2 Simulator를 수정 소스로 재실행했다. PR [#970](https://github.com/ohah/spinon/pull/970)은 2026-11-01T05:59:35Z Rebase and merge로 병합됐다(merge SHA `dd178e769c562d3cbc12cfea0c9e11093b36b8fd`). 구현/runtime 실패 관점 20개와 PR diff 독립 관점 30개 검토를 마쳤다. 병합 후 공식 상태 문서 PR과 Tailnet 반영·화면 확인이 남아 있다. 이 제한 slice는 전체 C11.3.3/C11 완료가 아니다. [병합 확인](../spec/internal/evidence/c11-3-3-negative-named-line-two-axis-runtime-2026-11-01/merge-PR970.md). [병합 후 상태·로드맵 대조 30개](../spec/internal/evidence/c11-3-3-negative-named-line-two-axis-runtime-2026-11-01/post-merge-status-roadmap-review-30.md) · [문서 PR diff 검토 30개](../spec/internal/evidence/c11-3-3-negative-named-line-two-axis-runtime-2026-11-01/post-merge-docs-pr-diff-review-30.md). [사전 비교](../spec/internal/evidence/c11-3-3-negative-named-line-two-axis-precomparison-2026-11-01/README.md) · [실행·artifact 근거](../spec/internal/evidence/c11-3-3-negative-named-line-two-axis-runtime-2026-11-01/README.md) · [구현/runtime 검토 20개](../spec/internal/evidence/c11-3-3-negative-named-line-two-axis-runtime-2026-11-01/implementation-runtime-review-20.md) · [PR diff 검토 30개](../spec/internal/evidence/c11-3-3-negative-named-line-two-axis-runtime-2026-11-01/pr-diff-review-30.md) · [계획 실패 관점 검토 20개](./c11-3-3-negative-named-line-two-axis-plan-review-20.md) · [계획 PR 변경 검토 20개](../spec/internal/evidence/c11-3-3-negative-named-line-two-axis-precomparison-2026-11-01/plan-pr-diff-review-20.md)

**상위:** [C11 CSS Grid](./c11-css-grid.md) · [C11.3 단계 계획](./c11-3-template-repeat-named-lines-areas.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**규범:** [CSS Grid Layout Module Level 1, 2025-03-26 CRD §§7.2, 8.1.3, 8.3.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)

## 해결할 공백

[PR #967](https://github.com/ohah/spinon/pull/967)은 영역 없는 Grid의 단일 fixed-size item에서 같은 custom-ident를 가진 두 negative endpoint가 한 축 앞쪽 implicit line으로 해석되는 경우를 제한 지원한다. 다른 축은 definite placement여야 한다. 양 축 implicit sizing, 두 축 모두 automatic인 placement와 추가 participating child는 그 계획에서 명시적으로 제외했다.

새 입력은 같은 name resolution과 track sizing을 두 축에서 동시에 요구한다. 한 축의 expanded track order, item span 또는 contribution이 다른 축 계산에 섞이면 안 된다. 기존 one-axis profile을 묵시적으로 넓히지 않고 별도의 typed placement mode와 실패 경계를 두 축 조합에만 추가한다.

## 목표와 고정 범위

- `RuntimeBlockFlexGridTemplateV1`의 C11.3.3 opt-in fixture route에서만 허용한다. Public CSS admission이나 일반 runtime profile은 변경하지 않는다.
- Container는 `grid-template-areas:none`, definite `120×100px` content box, `20px` explicit column 1개와 `15px` explicit row 1개다. layout-participating direct grid item은 정확히 하나며 width/height는 각각 fixed `18px`/`12px`다.
- 양 축 모두 같은 custom-ident의 negative integer endpoints를 쓴다. 기본 pair는 `-4 ghost / -1 ghost`이며, 두 endpoints가 actual leading implicit lines로 해석될 때만 새 mode를 선택한다. Authored named occurrence 우선순위는 기존대로 유지한다.
- 양 축 각각의 `grid-auto-*`는 `auto` 또는 비어 있지 않은 finite non-negative fixed-`px` 목록만 허용한다. Pattern은 각 축에서 독립적으로 확장하고 최종 track 배열은 grid start-to-end order로 비교한다.
- `-4 ghost / -1 ghost`는 축마다 네 leading implicit tracks를 만들고 item은 앞 세 track을 span한다. `auto` input에서 18px/12px fixed item contribution은 각각 세 spanned auto tracks에 균등하게 분배된다. 독립 기대값은 Columns `[6,6,6,0,20]px`, Rows `[4,4,4,0,15]px`, item frame `(0,0,18,12)`이다. Fixed pattern `5px 7px` Columns와 `3px 4px` Rows control은 각각 `[5,7,5,7,20]px`와 `[3,4,3,4,15]px`다.
- Equal endpoint와 reversed endpoint는 CSS Grid §8.3.1의 end-line removal 및 endpoint normalization을 거친다. Chrome 156에서 equal endpoints의 Columns `[18,20]px`, Rows `[12,15]px`, reversed pair의 tracks는 regular pair와 동일함을 기록했다. Fixed-pattern scene에서 첫 세 fixed tracks는 Columns 합 `17px`, Rows 합 `10px`이므로 fixed `18×12px` item이 spanned track area를 각각 1px·2px 넘는다. 이 overflow는 track을 늘리지 않고 item frame에 남아야 한다.

고정 입력과 independent expected values는 [사전 비교 자료](../spec/internal/evidence/c11-3-3-negative-named-line-two-axis-precomparison-2026-11-01/README.md)에 있다. 단일-axis auto controls는 existing #967 path의 회귀 기준이며 두 축 profile의 새 성공 scene으로 중복 등록하지 않는다.

## 구현 전 계약과 실패 경계

1. `project_grid_item_placement`와 `expand_tracks`에서 기존 one-axis negative named-line mode가 선택되는 조건, `TemplateGridSignedImplicitTracks`의 signed extent 및 fixed/auto expansion 경계를 확인한다. 양 축 경로에 별도 typed policy를 만들고 기존 mode를 재해석하지 않는다.
2. 각 축의 custom-ident lookup과 negative occurrence resolution을 독립 수행한다. Explicit authored matching lines가 하나라도 선택되어야 하는 경우 implicit mode로 전환하지 않는다. resolved row/column endpoints, signed extent와 item span을 checked arithmetic으로 만든다.
3. Row/column track-list phase, pattern reversal before explicit-grid start, automatic minimum/contribution, item placement와 final track order를 축별로 검증한다. One axis의 intrinsic contribution은 다른 축의 used track size를 바꾸면 안 된다.
4. Scope positive cases는 사전 비교의 여덟 scene이다: columns-only와 rows-only regression control, both-axis auto, both-axis fixed patterns, both-axis equal endpoints, both-axis reversed endpoints, 그리고 한 축씩 reversed인 대조군이다. Rust expected values는 CSS Grid 규범과 fixed item dimensions로부터 독립 계산한다.
5. Fail-closed cases에는 different custom-ident endpoints, mixed positive/negative endpoints, named span, numeric-only endpoint, authored-name precedence conflict, one-axis-only placement dispatched to the wrong mode, `grid-template-areas`, zero/multiple participating items, auto-sized item, indefinite axis, intrinsic/percentage/flex/`minmax()` auto track grammar, stale snapshot/revision, overflow와 10,000-track ceiling 초과를 둔다. 오류가 발생하면 partial frame·track result를 publish하지 않는다.
6. Runtime validators는 source root/item identity, computed placement, same-revision HostDocument/Style snapshot, row/column signed extent, per-axis track vectors, item frame, draw/present identity와 sample-before-readback validation을 검사한다. Scene ID로 style values를 정하지 않는다.
7. `spinon-layout`, style projection, fixture Rust/JS, runtime matrix, FFI allowlist와 Android/iOS strict scene matcher를 모두 연결한다. 기존 C11.3.3 39-scene·126-node regression와 PR #967 one-axis regression을 final source에서 보존하고, Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator 각각에서 새 matrix를 실행한다. AVD software renderer, iOS compositor 제출 전 drawable, 실기기/hardware GPU 미검증을 구분한다.
8. Default/product artifact에는 새 fixture marker·symbol·payload가 없어야 하며 opt-in fixture-test positive control에는 포함되어야 한다. `cargo fmt --all -- --check`, 관련 Clippy, scoped Rust/runtime/FFI tests, Android/iOS app builds와 simulator runtime을 실제 실행해 기록한다.
9. 구현/runtime 변경은 별도로 새로운 20개 실패 관점으로 검토한다. 구현 PR diff는 사용자가 지정한 독립 적대 검증 20회로 확인한다. 발견한 결함은 고친 뒤 영향을 받은 검토 항목을 재검증한다.
10. PR 본문에 실제 simulator 화면이 유효하게 첨부될 수 있으면 첨부하고 Agent Browser에서 로드를 확인한다. Agent Browser session/process를 종료한다. 병합 후 internal contract 0059, `spec/STATUS.md`, C11 계획 인덱스와 public Tailnet roadmap을 실제 결과와 한계에 맞춰 갱신한다.

## 완료 판정과 제외

여덟 scene과 fail-closed controls가 Chrome oracle 및 독립 규범 계산과 일치하고, 양 축의 track expansion·placement·revision·publication·artifact 경계가 모두 확인돼야 이 제한 fixture profile을 완료로 기록할 수 있다. 이는 다른 names, mixed sign, named spans, numeric placement, multi-item sizing, grid areas, 임의 track grammar, public CSS support, WPT conformance, full C11.3.3 또는 C11 전체를 완료로 만들지 않는다. C11.3.3/C11 checkboxes와 공식 `72/213`은 그대로 둔다.

## 구현/runtime 후보 결과 · 2026-11-01

별도 `TemplateGridNegativeNamedLinePairAutoImplicitTracksBothAxes` typed mode를 추가했다. 양 축에서 같은 custom-ident의 negative named-line pair가 leading implicit lines로 풀리는 경우만 선택하며, 기존 한 축 전용 mode는 그대로 유지한다. `grid-auto-columns`와 `grid-auto-rows`는 각각 `auto` 또는 비어 있지 않은 유한 non-negative fixed-px 목록만 허용한다. 한 개의 fixed-size item, no-area template와 definite container shape를 고정하고 invalid placement, template areas, 잘못된 track type, 10,000-track ceiling을 닫는다.

양 축 auto 기준은 Columns `[6,6,6,0,20]px`, Rows `[4,4,4,0,15]px`, item `(0,0,18,12)`이다. fixed pattern, equal/reversed endpoints와 축별 reversal도 Rust·Chrome reference·mobile fixture에서 비교했다. Android API 37 ARM64 AVD/SwiftShader와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 element frame 13개와 표면 표본 6/6 RGBA exact, 최대 채널 오차 0을 확인했다. 잘못된 표본 좌표는 readback 전에 거부됐고 Android 미등록 scene은 runtime draw 전에 거부됐다. iOS 표면은 compositor 제출 전 Metal drawable이다. PR #967의 한 축 negative pair 회귀도 Android/iOS 각각 8/8 exact로 다시 확인했다. 기존 C11.3.3 39-scene·126-node 행렬은 양쪽 모두 geometry 오차 0 CSS px다.

`spinon-layout` 330 tests, focused style/FFI tests, CSS reference 19/19, fixture-boundary 30/30, scoped Clippy와 rustfmt가 통과했다. 전체 `spinon-style-to-layout` suite는 275 passed·1 failed로 끝났으며 실패는 변경과 무관한 기존 C07.3 `aspect_ratio_computed_values_and_frames_match_pinned_chromium_at_both_dprs`의 float serialization 기대값(`1.7777778` 대 `1.77778`)이다. 이를 전체 성공으로 계산하지 않는다. macOS 기본 `spinon-ffi` archive에는 세 candidate marker가 없고 fixture positive control에는 세 marker가 있다. 이 archive 검사는 APK/IPA 전체 bundle 제외를 증명하지 않는다. 자세한 경계와 앱 hash는 실행 근거에 둔다.

현재 결과는 unmerged checkout에서 만든 candidate 증거다. Android는 SwiftShader AVD, iOS는 compositor 이전 drawable이며 실기기/hardware GPU, Spinon WPT, 일반 CSS admission, 전체 Grid 지원을 확인하지 않았다. PR의 hosted checks·diff 검토·merge가 끝나기 전에는 이 계획 slice를 완료 처리하지 않는다.
