# C11.4 · intrinsic contribution이 flexible/fixed track 분배에 미치는 영향

**상위 계획:** [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**사전 비교:** [Chrome 156 관측·독립 산술](../spec/internal/evidence/c11-4-mixed-intrinsic-flex-fixed-precomparison-2026-10-28/README.md) · [예상값 JSON](../spec/internal/evidence/c11-4-mixed-intrinsic-flex-fixed-precomparison-2026-10-28/expected.json) · [전체 비교 화면](../spec/internal/evidence/c11-4-mixed-intrinsic-flex-fixed-precomparison-2026-10-28/chrome/mixed-intrinsic-flex-fixed.png)

**계획 검토:** [서로 다른 실패 관점 20개](./c11-4-mixed-intrinsic-flex-fixed-plan-review-20.md) · [계획 PR diff 30관점 검토](./c11-4-mixed-intrinsic-flex-fixed-pr-diff-review-30.md)

## 목적과 구현 전 결손

계획을 세울 당시 이미 병합된 `ExpandFlexibleTracks` pure Rust oracle과 앱 fixture는 definite empty Grid에서 1fr:2fr 배분, base size가 만드는 freeze/recompute, factor 합이 1보다 작은 경우와 0fr을 확인한다. 직접 text/image fixture와 multi-span runtime은 있었지만, intrinsic item contribution이 실제 flexible track의 freeze와 fixed neighbor를 함께 포함한 한 Grid에서 바뀌는지는 검증하지 않았다. 이 계획은 CSS Grid §11.5.1의 min-content track base가 §11.7.1의 flex 분배 결과를 바꾸는 입력을 `RuntimeBlockFlexGridTrackSizingV1`의 제한 fixture 경로에서 확인했다.

기준 동작은 [사전 비교 자료](../spec/internal/evidence/c11-4-mixed-intrinsic-flex-fixed-precomparison-2026-10-28/README.md)에 구현 전에 고정했다. Chrome은 independent CSS Grid 2025 CRD 계산을 교차 관찰할 뿐 규범 oracle을 대신하지 않는다.

## 고정 입력과 독립 기대값

모든 scene의 definite sizing axis는 300px, cross axis는 120px이다. Target-axis track list는 `minmax(min-content, 1fr) 40px minmax(0px, 2fr)`, 내부 gutter는 각각 10px 두 개다. Grid는 zero border/padding과 definite root size를 사용한다. Columns scene은 `300×120px` root, Rows scene은 `120×300px` root에 배치한다.

각 장면은 직접 child 세 개를 갖는다.

1. `a`: Ahem 10px direct text. freeze input은 `XXXXXXXXXX` (100px min-content), equality input은 `XXXXXXXX` (80px), no-freeze input은 `XXXX` (40px)다. Rows에서만 `writing-mode:vertical-rl`로 text inline size를 Grid row의 block axis에 둔다.
2. `b`: 저장된 `c14-1-png-basic.png`의 natural 24×16px HTML `img`, fixed 40px track 안에 둔다. CSS width/height는 `auto`; `justify-self`/`align-self:start`로 natural frame을 유지한다.
3. `c`: Ahem 10px `XXXX` text를 `minmax(0px, 2fr)` track에 둔다. Rows scene에서는 같은 orthogonal writing mode를 쓴다.

`grid-column`/`grid-row`는 각 item의 위치를 고정한다. 기본 source order `[a,b,c]`와 freeze control의 역전 `[c,b,a]`는 같은 결과를 내야 한다. 모든 scene은 [Chrome fixture](../tests/fixtures/css/c11/grid-mixed-intrinsic-flex-fixed-c114-precomparison.html)의 CSS, font, image와 일치해야 한다.

CSS Grid Layout Module Level 1, 2025 CRD §§11.5.1, 11.7, 11.7.1을 사용한다. 두 10px gutter와 fixed 40px track을 제외하면 flexible tracks에 240px이 남고, 1fr:2fr의 첫 hypothetical flex fraction은 80px이다.

| Scene | Axis | `a` min-content base | Freeze 판정 | 기대 tracks |
| --- | --- | ---: | --- | --- |
| `column-freeze` | Columns | 100px | 1fr hypothetical 80px보다 커서 100px에서 freeze; 남은 140px를 2fr에 재분배 | `[100, 40, 140]px` |
| `column-no-freeze` | Columns | 40px | 80px보다 작아 freeze 없음 | `[80, 40, 160]px` |
| `column-equal-boundary` | Columns | 80px | strict-smaller 조건과 같으므로 freeze 없음 | `[80, 40, 160]px` |
| `column-freeze-reversed` | Columns | 100px | freeze case의 source order 대조 | `[100, 40, 140]px` |
| `row-freeze` | Rows | 100px | row 1fr을 100px에서 freeze한 뒤 2fr 재분배 | `[100, 40, 140]px` |
| `row-no-freeze` | Rows | 40px | row 1fr freeze 없음 | `[80, 40, 160]px` |
| `row-equal-boundary` | Rows | 80px | row-axis strict equality boundary, freeze 없음 | `[80, 40, 160]px` |
| `row-freeze-reversed` | Rows | 100px | row freeze case의 source order 대조 | `[100, 40, 140]px` |

모든 장면에서 `track sum + 2 gaps = 300px`, remaining free space는 0px이다. Exact rational track·item frames는 [`expected.json`](../spec/internal/evidence/c11-4-mixed-intrinsic-flex-fixed-precomparison-2026-10-28/expected.json), 계산은 [`derive_expected.py`](../spec/internal/evidence/c11-4-mixed-intrinsic-flex-fixed-precomparison-2026-10-28/derive_expected.py)에 둔다. 최종 `ExpandFlexibleTracks` trace의 CSS track base와 root-relative item frame은 independent rational 값에서 `1e-4 CSS px` 이내로 비교한다. Chrome final used-track/frame 대조에는 Chrome CSSOM의 1/64px 직렬화 단위만 허용한다.

## 구현 범위와 런타임 계약

- 기존 fixture-only FFI/runtime 경계 안에 정확히 여덟 scene을 추가한다. 고정 launcher ID는 사용하되 root/item `HostNode ID`를 새 값으로 만들어 generic source/style adapter를 통과하게 한다. CSS authoring 범위나 public API를 확장하지 않는다.
- Scene adapter는 root namespace/tag/parent, exact child count/order, `a`/`b`/`c`의 tag와 content, Ahem font/style, PNG bytes/decode/natural dimensions, axis-specific track list, gap, definite size, placement와 document/render-tree/style revision을 검사한다. 알려진 scene ID라도 잘못된 입력은 다른 fixture 또는 generic path로 강등하지 않고 fail closed해야 한다.
- Style cascade와 typed projection은 `minmax(min-content, 1fr)`, fixed 40px, `minmax(0px, 2fr)` 순서 및 지정 축 gap을 보존한다. Unsupported computed function은 node와 `grid-template-columns`/`grid-template-rows` property 맥락을 유지한다.
- Rust runtime test는 각 scene의 intrinsic `a` contribution, image natural-size input, `MaximizeTracks → ExpandFlexibleTracks → StretchAutoTracks` stage 순서, axis pass 수, 세 track/two gutter/boundary sentinel, 최종 CSS track base, root/item frame과 source-order 불변을 검사한다. `ExpandFlexibleTracks`는 대상 축에서 정확히 한 번이어야 한다. `infinitely_growable` 같은 비규범 내부 flag를 CSS 기대값으로 만들지 않는다.
- Negative controls는 unknown scene/root, changed text/style/font, image source/decode/natural-size 변조, missing/extra/reordered child 이외의 순서, 잘못된 track 함수/factor/fixed neighbor/gap/size/placement, wrong axis, stale revision과 non-finite trace를 포함한다. Failure는 partial intrinsic input, successful partial layout, draw/present를 내지 않아야 한다.
- Android API 37 ARM64 16 KB AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 최종 source로 여덟 장면을 각각 실행한다. scene별 source/app hash, run ID, root/item frame, used tracks, contribution/stage trace, draw/present와 screenshot을 저장한다. Expected overflow가 없으므로 border 바깥 paint 표본은 이 slice의 완료 기준이 아니다. Screenshot은 geometry/presentation 근거이며 별도 검증 없이 pixel equality를 주장하지 않는다.
- 기본 product FFI archive에서 새 scene IDs, fixture symbols와 source strings가 빠지고 opt-in fixture-test archive에는 포함되는지 비교한다. 정적 archive 결과를 APK/IPA 전체 fixture 제외로 확대하지 않는다.
- Rust focused tests, 기존 C11.4 text/image/flex/grid regressions, `cargo fmt --all -- --check`, targeted Clippy, Android/iOS fixture builds 및 `git diff --check`를 실행한다. 사용하지 않은 항목은 통과했다고 보고하지 않는다.

## 진행과 PR 게이트

1. 이 계획의 실패 관점 20개와 계획 PR 변경분 30개 검토를 끝낸 뒤 plan PR을 `Rebase and merge`한다.
2. 병합한 plan의 예상 입력에서 벗어나는 source/Stylo AST 변환이 있으면 implementation 전에 계획과 oracle을 고친다.
3. 같은 checkout에서 구현 PR을 만든다. 새로운 구현/runtime 실패 관점 20개와 분리된 PR diff 검토를 수행한다. 저장소 규칙에 따라 PR 변경분은 30개 관점으로 검토하고, 발견 결함을 고친 뒤 관련 관점을 다시 실행한다.
4. PR 본문에는 Android/iOS Simulator screenshot의 commit-pinned image link를 넣고 Agent Browser에서 표시를 확인한다. Agent Browser 세션과 process는 검증 뒤 종료한다. PR은 `Rebase and merge`한다.
5. `spec/STATUS.md`, 이 상위 계획, 내부 계약 0059와 public roadmap은 병합된 구현·근거와 미지원 범위에 맞춰 별도 변경으로 동기화한다. C11.4 checkbox 및 공식 `72/213`은 그대로 미완료로 둔다.

## 범위 밖

이 slice는 arbitrary item/source trees, anonymous/nested/hidden/out-of-flow items, other fonts/scripts, multiple images, dynamic image replacement, unspecified intrinsic track combinations, percentage/calc gap, auto-repeat, full span-group matrix, WPT harness pass, physical devices, Android hardware GPU, iOS compositor-after pixels, general CSS admission 또는 전체 C11.4 완료를 주장하지 않는다.

## 구현 결과 · 2026-10-28

구현 PR [#729](https://github.com/ohah/spinon/pull/729)은 Rebase and merge됐고 최종 merge SHA는 `3e1add7414d534c0b5ca830a9c772ba65a3a3ace`다. 여덟 scene이 Android API 37 ARM64 16 KB AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 8/8 통과했다. 두 simulator에서 미등록 scene과 충돌 C11.2.1 profile도 fail closed됐다. [실행 근거](../spec/internal/evidence/c11-4-mixed-intrinsic-flex-fixed-runtime-2026-10-28/README.md) · [기본·opt-in archive 비교](../spec/internal/evidence/c11-4-mixed-intrinsic-flex-fixed-runtime-2026-10-28/artifact-boundary.md) · [구현/runtime 검토 20개](../spec/internal/evidence/c11-4-mixed-intrinsic-flex-fixed-runtime-2026-10-28/implementation-review-20.md) · [PR diff 검토 30개](../spec/internal/evidence/c11-4-mixed-intrinsic-flex-fixed-runtime-2026-10-28/pr-delta-review-30.md) · [병합 후 문서 검토 30개](../spec/internal/evidence/c11-4-mixed-intrinsic-flex-fixed-runtime-2026-10-28/post-merge-docs-review-30.md).

PR #729 당시 GitHub check rollup은 비어 있었고 strict targeted Clippy는 변경하지 않은 `type_complexity`·`map_identity` 경고로 실패했다. archive 검사는 macOS ARM64 static FFI library에 한정되며 Android APK/iOS IPA 전체 제외를 뜻하지 않는다. 이미지 입력 후속 [PR #731](https://github.com/ohah/spinon/pull/731)은 source URL·decode·natural-size 변조를 두 축의 Rust test로 보완해 `76f7009561c806b24e061f64d9cd9ba3e57fc197`에 병합됐다. [이미지 거부 근거](../spec/internal/evidence/c11-4-mixed-intrinsic-flex-fixed-runtime-2026-10-28/image-negative-controls-followup-review.md).

이번 검증 후보 PR [#732](https://github.com/ohah/spinon/pull/732)은 텍스트, direct-child 수·순서, root/item style, track 함수·factor·fixed neighbor, 축, stale revision과 profile 변조 50개를 dispatcher 단위에서 두 축으로 거부하는 test-only follow-up이다. 전체 `spinon-runtime` fixture-feature suite 284 passed / 3 ignored와 integration 3 + 16 passed를 기록했다. 앱 route나 scene은 바꾸지 않아 Android/iOS 앱 및 Simulator는 다시 실행하지 않았다. [비이미지 입력 거부 근거 20개](../spec/internal/evidence/c11-4-mixed-intrinsic-flex-fixed-runtime-2026-10-28/non-image-negative-controls-implementation-review-20.md).

선택한 고정-fixture 변조 거부와 PR #729의 앱 장면 검증은 전체 CSS Grid, 일반 CSS admission, WPT, hidden/out-of-flow/overflow 조합 또는 전체 C11.4를 증명하지 않는다. C11.4와 공식 `72/213`은 미완료다.
