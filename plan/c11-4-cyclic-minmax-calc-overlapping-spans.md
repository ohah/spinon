# C11.4 · cyclic `minmax(calc())` 겹치는 span 기여 계획

**상위 계획:** [C11.4 CSS Grid track sizing](./c11-4-grid-track-sizing.md) · [직전 cyclic minimum span slice](./c11-4-cyclic-minmax-calc-minimum-span.md) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** 여섯 fixed scene을 style-to-layout, opt-in runtime fixture, Android·iOS simulator harness에 연결했다. 두 simulator에서 각 6/6 scene과 경계 바깥 text 표본을 확인했다. 구현 PR [#500](https://github.com/ohah/spinon/pull/500)이 열려 있으며 C11.4 checkbox와 공식 완료 수 `77/213`은 유지한다.

## 목적과 경계

[직전 PR #497](./c11-4-cyclic-minmax-calc-minimum-span.md)은 cyclic `minmax(calc())` minimum이 있는 두 track을 하나의 span-2 Ahem item이 가로지르는 네 fixed scene을 확인했다. 그 계획은 여러 span group과 복수 item의 source-order interaction을 제외했다. 이 후속 slice는 같은 track function에서 span-2 item 둘의 겹치는 planned increase를 한 그룹으로 처리하고, span-3 item을 다음 그룹에서 처리하는지 고정한다. forward/reverse DOM 순서를 비교해 §11.5.1의 planned-increase 불변식도 확인한다.

범위는 한 axis의 세 explicit column, 세 direct text item, 단일 fixed row, 2px gap, 단일 Ahem font, 정확히 여섯 scene ID다. 일반 CSS admission, arbitrary item count, 다른 track grammar, item span을 통한 implicit track 생성, nested/anonymous/replaced item, auto-repeat, row-axis 후속, WPT 또는 전체 C11.4 지원을 추가하지 않는다.

## 기준 입력과 기대값

고정 HTML, Chrome 156 관측, 독립 계산과 screenshot은 [사전 비교 자료](../spec/internal/evidence/c11-4-cyclic-minmax-calc-overlapping-spans-precomparison-2026-10-22/README.md)에 둔다. track list는 세 번의 `minmax(calc(10% + 2px), max-content)`이고 `column-gap:2px`다. `white-space:nowrap`인 Ahem 10px text는 span-2 left 70px, span-2 right 80px, span-3 140px의 min/max-content contribution을 만든다.

CSS Grid 2025 CRD §7.2.1에서 cyclic percentage는 intrinsic container size를 구할 때 `auto`처럼 다뤄지고 결과 size로 다시 resolve된다. §11.5.1은 span 길이가 짧은 그룹부터 처리하고 track별 planned increase로 같은 span 그룹의 item 순서를 분리한다. 독립 계산은 다음과 같다.

1. span-2 left: `70px − 2px gutter = 68px`; 두 track에 34px씩 계획한다.
2. span-2 right: `80px − 2px gutter = 78px`; 두 track에 39px씩 계획한다.
3. 같은 span 그룹의 per-track 최대 planned increase는 `[34,39,39]`이다.
4. 다음 span-3 item은 `140 − (34+39+39) − 4px gutters = 24px`를 남기므로 각 track에 8px씩 더한다. max-content upper sizes는 `[42,47,47]`, Grid content width는 140px이다.
5. cyclic calc minimum은 `10% × 140 + 2 = 16px`이며 각 max-content upper size보다 작아 final Columns는 `[42,47,47]`이다. definite 140px control도 같은 값을 가져야 한다.
6. definite 100px control의 calc minimum은 12px이고 남은 60px을 세 base size가 균등하게 나눠 Columns `[32,32,32]`가 된다. span-3 box는 100px이지만 nowrap text는 140px이라 content box를 40px 넘는다.

| Scene | Root content width | Final Columns | Root border box | span-2 left / right | span-3 box / text |
|---|---:|---:|---:|---:|---:|
| `cyclic-minmax-calc-overlap-forward` | 140px | `[42,47,47]` | `144×14px` | `91px / 96px` | `140px / 140px` |
| `cyclic-minmax-calc-overlap-reverse` | 140px | `[42,47,47]` | `144×14px` | `91px / 96px` | `140px / 140px` |
| `definite-minmax-calc-overlap-forward` | 140px | `[42,47,47]` | `144×14px` | `91px / 96px` | `140px / 140px` |
| `definite-minmax-calc-overlap-reverse` | 140px | `[42,47,47]` | `144×14px` | `91px / 96px` | `140px / 140px` |
| `definite-minmax-calc-overlap-small-forward` | 100px | `[32,32,32]` | `104×14px` | `66px / 66px` | `100px / 140px` |
| `definite-minmax-calc-overlap-small-reverse` | 100px | `[32,32,32]` | `104×14px` | `66px / 66px` | `100px / 140px` |

위 frame의 정확한 root-relative 좌표와 독립 계산은 [expectation JSON](../spec/internal/evidence/c11-4-cyclic-minmax-calc-overlapping-spans-precomparison-2026-10-22/spec-derived-expectations.json), Chrome 관찰 원본은 [observation JSON](../spec/internal/evidence/c11-4-cyclic-minmax-calc-overlapping-spans-precomparison-2026-10-22/chrome156-observation.json)에 있다. Chrome 156은 여섯 장면에서 track list와 frame을 관찰했다. Browser screenshot의 page white와 앱 native surface clear black은 다른 배경이므로 같은 표면 결과로 취급하지 않는다.

## 구현·판정 관문

1. 여섯 scene ID와 세 direct text child, 세 동일 calc-minimum track, 2px column gap, fixed 10px row, 정의된 numeric column span만 기존 C11.4 opt-in fixture-test 경로에 연결한다. 다른 shape, axis, span, expression과 미등록 ID는 실패로 닫는다.
2. 앱 adapter에서 ID별 DOM 생성 순서와 placement를 그대로 보존한다. `min-width:0`, `white-space:nowrap`, Ahem metrics 및 root sizing 입력이 모두 같은 fixture revision에 들어왔는지 검증한다. CSS cascade allowlist나 공개 API를 넓히지 않는다.
3. style-to-layout/runtime checks는 normalized typed track input, 세 item의 70/80/140px intrinsic source, span별 contribution trace, used Columns/Rows, root 및 item frame을 확인한다. 같은-span planned increases를 item 순서에 따라 누적해 버리거나 span-3 그룹을 span-2 앞에서 처리하는 출력은 실패한다.
4. definite 100px control의 root-relative `(126,7)`에서 root 바깥 Ahem text `#e5484d`, `(150,7)`에서 해당 글자 뒤 host clear를 Android PixelCopy와 iOS 제출 전 Metal drawable로 확인한다. `(4,7)` root 내부 control도 함께 기록한다. Browser page background와 native host clear는 각각 원본 환경으로만 판정하고 CSS border stroke는 주장하지 않는다.
5. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 여섯 scene을 각각 실행한다. Frame, track/input trace, draw/present, PixelCopy·Metal 표본을 기록한다. 실기기는 연결하지 않는다.
6. 기본 release FFI archive에서 새 scene ID, payload와 fixture entry가 빠지고 opt-in fixture positive control에 포함되는지 검사한다. 필요한 focused Rust tests, modified crate Clippy, `cargo fmt`, simulator app builds를 실행한다.
7. 새 code/runtime을 기존 계획 실패 관점과 다른 20개 실패 경로로 검토한다. 정확한 PR diff도 별도 20개 관점으로 검토한다. 수정 후 영향받은 scene과 표면 표본을 재실행한다.
8. PR은 화면·실행 자료와 이 fixed-profile의 한계를 기술한다. C11.4 checkbox와 `77/213`을 올리지 않는다. 병합 전에 현재 required checks와 merge state를 조회하고 프로젝트의 `Rebase and merge` 정책을 따른다.

## 구현 결과

- Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 여섯 scene의 Columns/Rows, intrinsic spans, item/root frames와 draw/present를 각각 6/6 확인했다. Intrinsic/140px cases는 Columns `[42,47,47]`, definite 100px controls는 `[32,32,32]`다.
- 두 100px control에서 root-relative `(4,7)` 내부 text, `(126,7)` root border 밖 red text, `(150,7)` text 뒤 clear 표본이 Android PixelCopy와 제출 전 iOS Metal drawable에서 각각 6/6 exact다. 이는 CSS border stroke나 iOS compositor 이후 pixels 검증이 아니다.
- 기본 release FFI archive에 여섯 scene ID와 fixture entry marker가 없고 opt-in positive control에만 있다. 구현/runtime 검토 20개와 PR diff 검토 20개를 기록했다.
- [실행 화면·simulator·build·artifact 근거](../spec/internal/evidence/c11-4-cyclic-minmax-calc-overlapping-spans-runtime-2026-10-22/README.md)

## 명시적 범위 밖

이 계획은 한 열 축의 세 identical calc-minimum track, 두 overlapping span-2 text item, 하나의 span-3 text item, 두 DOM 순서와 세 width condition만 고정한다. arbitrary item/span group, other track functions, row axis, percentages other than 10%, nested/anonymous/replaced items, border painting, writing modes, general CSS admission, WPT, Android physical/hardware GPU, iOS compositor 이후 pixels 및 전체 C11.4는 미검증으로 둔다.

## 계획 검토

- [서로 다른 계획 실패 관점 20개](./c11-4-cyclic-minmax-calc-overlapping-spans-plan-review-20.md)
- [Chrome 156 사전 비교와 독립 계산](../spec/internal/evidence/c11-4-cyclic-minmax-calc-overlapping-spans-precomparison-2026-10-22/README.md)

계획 PR 병합은 runtime 구현·제품 기능 지원·C11.4 완료를 뜻하지 않는다.
