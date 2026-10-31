# C11.4 direct-text preferred aspect-ratio definite-axis transfer

**상태:** 제한 opt-in fixture 구현 완료. 구현 PR [#878](https://github.com/ohah/spinon/pull/878)은 2026-10-30T21:47:44Z Rebase and merge됐고 merge SHA는 `b1b7356e2e2afe397c2d41ebf111863a7f25ccd9`다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 기존 51개와 신규 네 장면을 각 55/55 통과했다. PR check rollup은 비어 있어 CI 통과로 기록하지 않는다. 이 slice는 C11.4 또는 C11 전체 완료를 뜻하지 않으며 공식 `72/213`을 올리지 않는다. [구현·실행 근거](../spec/internal/evidence/c11-4-direct-text-aspect-ratio-definite-axis-runtime-2026-10-31/README.md) · [병합 후 문서 검토 30개](../spec/internal/evidence/c11-4-direct-text-aspect-ratio-definite-axis-post-merge-2026-11-01/post-merge-doc-review-30.md)

**상위:** [C11.4 Grid track sizing](./c11-4-grid-track-sizing.md) · [finite-positive ratio](./c11-4-direct-text-aspect-ratio-finite.md) · [`auto <ratio>` sizing basis](./c11-4-direct-text-aspect-ratio-auto.md) · [C11 Grid 내부 계약 0059](../spec/internal/0059-c11-grid-layout.md) · [공식 상태](../spec/STATUS.md)

## 문제와 목표

기존 direct-text preferred-ratio collector는 두 item의 `width`와 `height`가 모두 computed `auto`일 때만 paragraph input을 만든다. 따라서 definite `width` 또는 definite `height`가 preferred ratio를 통해 반대 축 크기를 결정하는 input은 거부된다. 이 계획은 이 누락만을 별도 C11.4 fixture profile로 검토한다.

목표는 numeric preferred ratio `2 / 1`을 가진 `AAAA` item의 한 definite dimension이 반대 축으로 전달되는 두 방향을, no-ratio controls와 함께 기록하고 검증하는 것이다. 기존 ratio, min/max, multispan 및 일반 direct-text profile은 유지한다. 사용자 CSS admission, 공개 API, 전체 aspect-ratio 지원 범위는 열지 않는다.

## 규범·oracle 및 사전 판정

- 기준은 [CSS Grid Layout Module Level 1, 2025 CRD §6.2 및 §6.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#grid-item-sizing)이다. Grid item sizing과 preferred aspect-ratio에 따른 automatic sizing을 기술한다.
- [CSS Box Sizing Module Level 4, Working Draft 2026-09-04 §4.1–4.3](https://www.w3.org/TR/2026/WD-css-sizing-4-20260904/#aspect-ratio)은 axis transfer를 설명하는 보조 근거다. Working Draft이므로 이를 안정된 CSS Sizing 표준이라고 표현하지 않는다.
- [Agent Browser HeadlessChrome 156 사전 비교](../spec/internal/evidence/c11-4-direct-text-aspect-ratio-axis-transfer-precomparison-2026-10-31/README.md)는 저장소 Ahem font, 1280×633 viewport, DPR 1 및 네 고정 scene의 CSSOM track, item frame, text Range를 보존한다. Fixture, JSON, screenshot은 SHA-256으로 고정되어 있다. 이는 Chrome oracle이지 Spinon 동작, WPT 적합성 또는 플랫폼 전체의 규범 판정이 아니다.
- Independent expectation은 `width / height = 2 / 1`로 계산한다. `width:60px`은 automatic height `30px`을 만들고, `height:30px`은 automatic width `60px`을 만든다. Controls는 ratio transfer가 없을 때 content contribution인 `AAAA` 40px과 Ahem line-height 10px을 유지하는지 분리한다.

| Scene | `AAAA` definite size / ratio | Root columns / row | `AAAA` item frame | Text Range |
|---|---|---|---|---|
| `width-ratio` | `width:60px;height:auto;aspect-ratio:2 / 1` | `[10,60]` / `[30]` | `(10,0) 60×30` | `(10,0) 40×10` |
| `width-control` | `width:60px;height:auto;aspect-ratio:auto` | `[10,60]` / `[10]` | `(10,0) 60×10` | `(10,0) 40×10` |
| `height-ratio` | `width:auto;height:30px;aspect-ratio:2 / 1` | `[10,60]` / `[30]` | `(10,0) 60×30` | `(10,0) 40×10` |
| `height-control` | `width:auto;height:30px;aspect-ratio:auto` | `[10,40]` / `[30]` | `(10,0) 40×30` | `(10,0) 40×10` |

## 제한 입력과 collector 계약

- 현재 `RuntimeBlockFlexGridTrackSizingV1` direct-text fixture route 안에서만 허용한다. 어떤 CSS declaration이든 임의로 layout에 연결하는 일반 admission을 만들지 않는다.
- 새 direct-text profile은 기존 `AspectRatio` 경로와 분리된 하위 모듈로 구현한다. 불일치 root shape에는 `None`; 지원 root shape인데 style/structure가 부분적으로 잘못되면 구체적 error로 fail closed한다. scene ID, fixture label, authored CSS 원문으로 typed value를 추정하지 않는다.
- Root는 HTML namespace, in-flow Grid, computed width `100px`, height `auto`, columns `[max-content,max-content]`, rows `[auto]`, gap 0, `align-items:normal`, 나머지 alignment `normal`, direction LTR, horizontal-tb이며 margin/padding/border가 0이다. Auto-repeat, template areas, named lines, implicit tracks, positioning, root ratio는 거부한다.
- Root에는 정확히 두 in-flow HTML block direct child가 있고 각 item은 meaningful direct Text node 하나만 갖는다. Text는 각각 `A`와 `AAAA`, Ahem 10px/10px, nowrap, LTR, horizontal-tb다. Anonymous text, nested/replaced content, extra item, explicit placement, nonzero order, wrapping, alternate font, nonzero margin/padding/border는 거부한다.
- 최대 한 ratio owner만 허용하며 반드시 `AAAA` item이다. Candidate ratio는 finite positive typed numeric ratio와 computed `aspect-ratio` serialization이 모두 정확히 `2 / 1`이어야 한다. No-ratio controls의 typed ratio는 `None`, computed serialization은 정확히 `auto`여야 한다. `auto <ratio>`, degenerate/mismatched/stale values와 multiple/wrong-owner ratios는 거부한다.
- Ratio owner의 dimensions는 `(60px, auto)` 또는 `(auto, 30px)`만 허용한다. 다른 definite values, both-definite, min/max, percentages, `calc()`, non-default box edges와 sizing constraints는 거부한다. No-ratio control도 대응하는 동일 dimensions만 허용한다.
- Root와 item의 computed property string, typed dimensions, ratio, snapshot revision, paragraph ownership을 함께 검사한다. Partial match, unexpected text, missing style, stale revision, unsupported CSS value, allocation/layout error는 성공처럼 축약하지 않는다.
- `C11_4TextInput`의 existing paragraph layout 경계를 사용하고, 새 public API/FFI, CSS declaration policy, shared product cascade rule은 추가하지 않는다. Collector가 계산 근거가 되는 정확한 ratio owner `NodeId`를 결과에 표시하는 방법은 구현 전에 기존 dispatcher 계약과 대조한다. Runtime은 ratio를 가진 실제 computed owner에만 Taffy sizing을 적용해야 한다.

## 구현·검증 순서

1. Chrome observation JSON과 expected-values를 대조하고 모든 hash·input row track·계산값이 일치하는지 확인한다. 별도의 20 관점 계획 실패 검토를 먼저 마친다.
2. 새 profile module과 dispatcher route를 구현한다. 기존 automatic-width/height ratio, min/max, multispan 및 intrinsic direct-text 경로를 변경하지 않으며, 공유 style-to-layout 경로에 fixture 전용 dimension 예외가 새지 않게 한다.
3. 네 신규 scene의 computed snapshot, ratio owner, columns/rows, root/item frame, overflow, text Range 및 revision을 검증한다. Negative controls는 wrong/multiple ratio owner, ratio mismatch/degenerate/non-finite, both-definite/other dimensions, min/max, percentage/calc, nonzero edges, wrong root track/gap/alignment/font/writing mode, anonymous/nested/replaced/extra text item, stale style revision, missing paragraph owner, allocation/layout failure를 다룬다.
4. 기존 finite ratio·`auto <ratio>`·min/max·multispan matrix와 intrinsic/no-ratio fixtures를 영향 범위에 따라 재실행한다. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 신규 및 회귀 matrix를 확인하고, track/frame trace와 Android PixelCopy 및 iOS compositor 제출 전 Metal sample을 각각 기록한다. Screenshot은 가능한 경우 PR 본문에 올린다. Simulator 근거를 실기기·hardware GPU·compositor 이후 화면이라고 부르지 않는다.
5. fixture-test opt-in route에 대해 default/product archive absence 및 fixture positive-control presence를 확인한다. Focused Rust/layout/runtime checks, 관련 crate Clippy, `cargo fmt --all -- --check`와 두 simulator build를 기록한다. 실행하지 않은 gate는 통과로 쓰지 않는다.
6. 구현 결과에 맞춰 내부 계약 0059에 제한 profile·오류 경계·제외 범위를 기록했다. 공개 API나 내부 인터페이스 의미는 바뀌지 않아 계약 숫자 `0.1.0`을 유지한다. 공식 `spec/STATUS.md` checkbox와 `72/213`은 미완료 상태로 유지한다.
7. 구현/runtime 실패 관점 20개와 구현 PR #878 diff 실패 관점 30개를 별도로 검토했다. 병합 후 상태 동기화는 이 계획과 구별되는 30개 문서 diff 검토를 추가로 기록한다.

## 제외 범위

다른 ratio와 임의 definite values, both-definite sizing, ratio min/max transfer, box-edge transfer, replaced/natural dimensions, multiple ratio owners, span, wrapping/multiline, alternate text/font/script, vertical writing mode, general root HTML/CSS admission, full WPT, physical device, Android hardware GPU, iOS compositor 이후 pixels, C11.4/C11 완료는 포함하지 않는다.
