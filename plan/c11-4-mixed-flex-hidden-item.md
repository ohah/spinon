# C11.4 · `display:none` 큰 intrinsic child와 mixed flexible/fixed track

**상위 계획:** [C11.4 track sizing](./c11-4-grid-track-sizing.md) · [mixed intrinsic/flexible/fixed runtime](./c11-4-mixed-intrinsic-flex-fixed.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md#c11-css-grid)

**구현 전 비교:** [Chrome 156 관측과 CSS Grid 독립 산술](../spec/internal/evidence/c11-4-mixed-flex-hidden-item-precomparison-2026-10-28/README.md) · [기대값](../spec/internal/evidence/c11-4-mixed-flex-hidden-item-precomparison-2026-10-28/expected.json) · [화면](../spec/internal/evidence/c11-4-mixed-flex-hidden-item-precomparison-2026-10-28/chrome/mixed-flex-hidden-item.png)

**계획 검토:** [서로 다른 실패 관점 20개](./c11-4-mixed-flex-hidden-item-plan-review-20.md) · [계획 PR diff 30관점 검토](./c11-4-mixed-flex-hidden-item-pr-diff-review-30.md)

## 결손과 목표

병합된 [PR #729](https://github.com/ohah/spinon/pull/729)의 여덟 mixed-track 장면과 후속 [PR #731](https://github.com/ohah/spinon/pull/731), [PR #732](https://github.com/ohah/spinon/pull/732)는 visible text/image contribution 및 고정 scene의 입력 거부를 확인한다. PR #732는 `9e9355369a48e5fa6bbed25a76954dbc63a5fcac`에 2026-10-28T02:35:51Z Rebase and merge됐다. 해당 adapter는 direct child 세 개를 요구하므로, 150px text가 `display:none`인 네-child Grid가 첫 flexible track의 freeze 결과에서 빠지는지는 아직 확인되지 않았다.

이 제한 slice는 같은 첫 track에 겹쳐 배치된 정확한 HTML `<span>` 한 개가 `display:none`일 때 그 min-content text를 track sizing에서 제외하는지 Columns와 Rows 양축에서 검사한다. 동일 child를 보이게 한 두 Chrome scene은 제외 효과의 positive control이다. 앱 fixture route는 숨김 scene 두 개만 받으며 visible control이나 임의의 hidden subtree를 지원하지 않는다.

## 고정 oracle과 계산

규범은 CSS Grid 2025 CRD §11.5, §11.7–§11.7.1 및 [CSS Display Level 3 box-generation](https://www.w3.org/TR/css-display-3/#box-generation)이다. 구현 전에 만든 Chrome 156 fixture, 관측 JSON, 화면, 독립 `Fraction` 산술은 [사전 비교 README](../spec/internal/evidence/c11-4-mixed-flex-hidden-item-precomparison-2026-10-28/README.md)에 기록했다. Browser 결과를 규범 기대값으로 역산하지 않는다.

두 hidden scene은 모두 `minmax(min-content, 1fr) 40px minmax(0px, 2fr)`와 target-axis `10px` gap 두 개를 사용한다. definite `300px` 축에서 fixed track과 gutter를 뺀 flexible space는 `240px`, 초기 1fr fraction은 `80px`이다. 보이는 Ahem `XXXXXXXXXX`의 min-content는 `100px`; 숨겨진 15-character sibling은 보이게 했을 때만 `150px`이다.

| Runtime scene | Root | 기대 used tracks | 검증할 핵심 |
| --- | --- | --- | --- |
| `column-hidden` | `300×120px` | `[100, 40, 140]px` | hidden child의 intrinsic input 부재, Columns 1fr freeze와 2fr 재분배 |
| `row-hidden` | `120×300px` | `[100, 40, 140]px` | orthogonal `vertical-rl` text의 Rows 기여 및 동일 hidden exclusion |

Chrome controls `column-visible-control`과 `row-visible-control`은 첫 track `[150px]` 및 전체 `[150, 40, 90]px`를 관찰해 150px text가 실제 기여를 낼 수 있음을 보인다. Runtime expected roots/items는 `expected.json`의 root-relative frame이다. 각 axis에서 track 합과 gutter 합은 `300px`이며 잔여 free space는 `0px`이다.

## 제한 fixture 계약

- 기존 opt-in C11.4 `RuntimeBlockFlexGridTrackSizingV1` fixture route에 두 scene만 추가한다. CSS authoring/admission 범위, public API, 임의 HTML 지원은 늘리지 않는다.
- Grid는 document-root 직속 HTML `div`, exact definite size와 기존 track/gap/style, 한 revision의 source/render-tree/style snapshot을 사용한다. 기존 fixed PNG `24×16px`, visible `a`/`c` text와 placement, Ahem font 설정을 유지한다.
- 네 direct element child의 tag·순서·parent를 정확히 검사한다: `display:none`인 direct HTML `span` + `a` text `div` + 가운데 PNG `img` + `c` text `div`. Hidden span에는 직접 text node 하나만 있고 내용은 정확히 `XXXXXXXXXXXXXXX`다. Hidden span은 첫 track에 놓인다. 다른 tag, descendant, 추가 whitespace/text node, child 누락/추가/재정렬은 거부한다.
- Runtime adapter는 hidden span의 computed `display:none`, 축별 placement와 고정 style을 검증하되 text를 intrinsic measurement 입력에 넣지 않는다. Visible `a`의 100px 기여와 image intrinsic input은 유지한다. Track-sizing trace에서 첫 track base `100px`, used tracks `[100,40,140]`, 정확히 한 번의 target-axis `ExpandFlexibleTracks`와 기존 stage 순서를 확인한다.
- Layout output은 hidden element/subtree의 CSS box가 없음을 반영하는 zero frame만 허용하며, hidden text contribution·track contribution·draw item은 없어야 한다. visible sibling control은 고정 runtime adapter에서 `Unsupported`로 fail closed되어 부분 layout/draw를 만들지 않는다. CSS Display의 box 부재와 내부 zero-frame bookkeeping을 구분해 기록한다.
- Negative controls는 visible `display:block`, `visibility:hidden`, 다른 display value, 변형 text/font/writing mode, span ID/tag/namespace/parent/placement/style 변조, hidden subtree 추가, root children count/order, track/gap/size/axis/profile 및 stale revision 변조를 검사한다. 알려진 scene ID나 일부 일치 입력이 generic fallback 또는 기존 다른 fixture를 선택해서는 안 된다.
- 기존 여덟 mixed intrinsic/flex/fixed scene과 새 두 hidden scene을 Android API 37 ARM64 16 KB AVD 및 iPhone 17 Pro/iOS 26.2 Simulator에서 최종 source로 다시 실행한다. 각 플랫폼에서 10/10 scene의 source/app hash, scene ID, root/item frame, tracks, intrinsic contribution 및 stage trace, draw/present와 화면을 남긴다. hidden node의 제로 frame은 CSS box가 생성됐다는 증거로 해석하지 않는다. Screenshot은 화면/geometry 근거이며 별도 픽셀 표본 검사 없이 pixel equality를 주장하지 않는다.
- 기본 product 및 opt-in fixture-test static FFI archive positive/negative boundary를 검사한다. 이는 APK/IPA 전체에서 fixture code가 빠졌다는 주장이 아니다. Rust focused/runtime regressions, formatter, 관련 Clippy, Android/iOS fixture builds와 `git diff --check`를 실행한다. 엄격 Clippy가 기존 미수정 lint로 실패하면 범위와 정확한 결과를 남긴다.

## 계획·구현·PR 게이트

1. 이 계획을 서로 다른 20가지 실패 관점으로 검토하고, 계획 PR diff를 별도 30관점으로 검토한 뒤 계획 PR을 `Rebase and merge`한다.
2. 계획 병합 뒤 같은 checkout에서 hidden-item runtime adapter와 앱 fixture를 구현한다. 새로운 구현/runtime 실패 관점 20개와 PR diff 30개를 수행하고, 수정한 결함에 영향받은 관점을 다시 확인한다.
3. PR 본문에는 commit-pinned Android/iOS Simulator screenshot link를 넣고 Agent Browser에서 표시되는지 확인한다. 사용한 Agent Browser session과 process는 마친 뒤 종료한다. PR을 `Rebase and merge`한다.
4. 구현 병합 뒤 `spec/STATUS.md`, 상위 계획, 내부 계약 0059 및 Tailnet roadmap을 실제 근거와 동기화한다. C11.4 parent checkbox와 공식 완료 수 `72/213`은 이 제한 fixture로 변경하지 않는다.

## 범위 밖

이 slice는 visible runtime control, arbitrary/ancestor-hidden/nested/anonymous items, `visibility:hidden`의 지원, dynamic style 변경, 다른 fonts/scripts, multiple image, 일반 CSS admission, WPT pass, 실기기, Android hardware GPU, iOS compositor-after pixels, 모든 Grid hidden-item 조합 또는 전체 C11.4 완료를 주장하지 않는다.
