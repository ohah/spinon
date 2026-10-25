# C11.4 cyclic percentage × auto-repeat 사전 비교

**상태:** CSS Grid 2025 CRD의 auto-repeat count와 cyclic percentage 최종 sizing을 한 max-content Grid에서 결합하는 고정 입력을 만들었다. 규범 단계별 기대값은 intrinsic max-content pass에서 1회 반복으로 content width 24px을 구한 뒤, definite 24px로 하는 final layout pass에서 2회 반복·`[12px,12px]`이다. Chrome 156 CSSOM 관측은 최종 `[12px,12px]`이며 이 단계별 기대와 일치한다. Spinon layout/runtime은 이 비교에서 실행하지 않았다.

## 입력과 관찰

- 기준: [CSS Grid Layout Module Level 1, 2025 CRD §7.2.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes), [§7.2.3.1–§7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat), [§11.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-grid-sizing), [§11.2·§11.5·§11.6](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-track-sizing); `max-content`가 indefinite size라는 용어는 [CSS Sizing Level 3 §2](https://www.w3.org/TR/css-sizing-3/#definite)로 확인한다.
- 고정 HTML: <a href="/spinon/internal/evidence/c11-4-cyclic-percentage-auto-repeat-precomparison-2026-10-25/fixture.html">`fixture.html`</a>. Target은 `width:max-content`, `grid-template-columns:repeat(auto-fill,minmax(0px,50%))`, `gap:0`, border/padding 0, 한 개의 `24×16px` in-flow item이다. Definite control은 같은 track pattern과 `width:40px`을 사용한다. 화면의 track 경계를 보이기 위한 `.boundary` span은 각 grid의 절대 배치 관측 marker다. 이는 browser screenshot HTML에만 존재하고 out-of-flow라 Grid sizing item이 아니며 runtime scene, node ID 또는 child allowlist에는 포함되지 않는다. Runtime ID `cyclic-max-content-auto-fill`은 browser ID `cyclic`에, `definite-40px-auto-fill-control`은 `definite`에 대응한다.
- Agent Browser `0.31.1`, macOS ARM64, `HeadlessChrome/156.0.0.0`, viewport `1280×633` CSS px, DPR 1에서 실행했다. Chrome은 auto-repeat CSS 문법을 수용했다.
- 원시 DOM/CSSOM 관측은 <a href="/spinon/internal/evidence/c11-4-cyclic-percentage-auto-repeat-precomparison-2026-10-25/chrome156-observation.json">`chrome156-observation.json`</a>, 화면은 <a href="/spinon/internal/evidence/c11-4-cyclic-percentage-auto-repeat-precomparison-2026-10-25/chrome156-agent-browser.png">`chrome156-agent-browser.png`</a>, 독립 계산은 <a href="/spinon/internal/evidence/c11-4-cyclic-percentage-auto-repeat-precomparison-2026-10-25/spec-derived-expectations.json">`spec-derived-expectations.json`</a>이다.

| Browser ID → runtime ID | 독립 CSS Grid 기대값 | Chrome 156 관측 |
|---|---|---|
| `cyclic` → `cyclic-max-content-auto-fill` | intrinsic pass: 반복 1회·max-content width 24px; final pass: 반복 2회, columns `[12px,12px]`, rows `[16px]`, root `24×16px`, item `24×16px` | columns `12px 12px`, rows `16px`, root `24×16px`, item `24×16px` |
| `definite` → `definite-40px-auto-fill-control` | 반복 2회, columns `[20px,20px]`, rows `[16px]`, root `40×16px`, item `24×16px` | columns `20px 20px`, rows `16px`, root `40×16px`, item `24×16px` |

## 독립 규범 계산

`minmax(0px,50%)`은 `minmax(<fixed-breadth>, <track-breadth>)` 형태의 auto-repeat `<fixed-size>`이므로 문법상 허용된다. Intrinsic pass에서 `max-content`는 CSS Sizing의 definite-size 정의에 따라 indefinite이므로 §7.2.3.2상 한 번 반복된다.

Intrinsic sizing에서 cyclic `50%` maximum은 `auto`처럼 취급되어 유일한 repeated track은 `minmax(0px,auto)`로 계산된다. §11.2의 초기화에서 fixed `0px` minimum은 track base를 `0px`에 둔다. 유일한 item은 available inline size와 무관한 fixed preferred width `24px`을 가지므로 max-content contribution도 `24px`이다. §11.5–§11.6에서 이 single-span contribution이 intrinsic auto maximum의 growth limit을 정하고, max-content sizing은 single track의 base를 `0px`에서 `24px`까지 maximize한다. Gap과 이웃 track이 없으므로 grid max-content content width는 `24px`이다. CSS Grid §11.1은 indefinite grid를 min/max-content constraint로 sizing한 뒤 결과 크기로 실제 layout을 하도록 설명한다. 최종 pass에서 24px size는 definite basis이므로 §7.2.3.2의 definite-size 분기를 적용해 두 반복을 만든다. 각 50% maximum은 12px이고, 고정 0px minimum이 이를 올리지 않는다. 최종 columns는 `[12px,12px]`이며 24px item은 첫 track을 12px 넘는다.

Definite control에서는 40px content width가 처음부터 repeat count basis다. 반복당 maximum은 20px이므로 두 track은 정확히 40px을 채우고 세 track은 넘친다. 따라서 반복 수 2와 `[20px,20px]` track은 규범·Chrome 관측이 일치한다.

Chrome 156 최종 CSSOM은 target에서 `12px 12px`을 반환한다. 단계별 규범 기대에서 final count는 2이므로 관측과 일치한다. 브라우저 CSSOM은 final used track만 보여주며 intrinsic pass의 최초 count 1을 증명하지 않는다. 그 pass 간 count 변화는 계획된 Spinon trace acceptance가 직접 검사해야 한다. 단일 Chrome 관측은 다른 브라우저의 동작이나 전체 CSS 지원을 증명하지 않는다.

## 현재 runtime trace의 한계

코드 경로 확인상 `LayoutGridTrackSizingPassTrace`는 `sequence`, `axis`, `stages`, `contributions`를 기록하고, opt-in `c11-4-grid-track-sizing-fixture-test`가 runtime diagnostics 및 FFI JSON report를 제공한다. 현재 report에는 intrinsic max-content/final definite semantic phase, auto-repeat expanded count, percentage basis 필드가 없다. `sequence` 및 track stage 크기만으로 두 root sizing phase를 판별했다고 간주할 수 없다. 계획의 첫 runtime 관문에서 두 sizing phase가 실제 trace에 함께 보존되는지 probe해야 한다. 기록할 수 없으면 fixture-test 전용 계측을 해당 실제 sizing callsite에 추가하고, 계측 없이 phase/count/basis를 보장할 수 없으면 그 관문에서 멈춰야 한다. 이 precomparison에서는 runtime을 실행하지 않았다.

## Negative control과 error owner

실패 owner마다 결과 타입과 실행 경계가 다르다. 아래 기대값은 계획된 negative-control 계약이다.

| Mutation | Error owner | 기대 typed result | Layout/presentation 경계 |
|---|---|---|---|
| `cyclic-max-content-auto-fill-typo` scene ID | FFI `c11_4_grid_track_sizing_runtime_source` | `Err("알 수 없는 C11.4 scene ID: cyclic-max-content-auto-fill-typo")` (`Result<Cow<'static, str>, String>`) | Runtime layout이 시작되지 않는다. |
| `unexpected-item` 1×1px in-flow direct child | Cyclic runtime fixture adapter | `RuntimeLayoutFailure { code: "unsupported_c114_cyclic_percentage_auto_repeat_fixture", node_id: root, property: "<scene>: exactly one in-flow child required" }` | `RuntimeLayoutCompleted`와 frame/trace/render 결과 없음. |
| Browser `.boundary` span을 runtime node list에 추가 | Cyclic runtime fixture adapter | 같은 `RuntimeLayoutFailure` code; `node_id: marker`, property에 `browser-only out-of-flow marker is not a runtime scene node` | `RuntimeLayoutCompleted`와 frame/trace/render 결과 없음. |
| target에 `column-gap:1px` 설정 | Cyclic runtime fixture adapter | 같은 `RuntimeLayoutFailure` code; `node_id: root`, property에 `column-gap` mismatch | `RuntimeLayoutCompleted`와 frame/trace/render 결과 없음. |
| target의 `auto-fill`을 `auto-fit`으로 변경 | Cyclic runtime fixture adapter | 같은 `RuntimeLayoutFailure` code; `node_id: root`, property에 `grid-template-columns` mismatch | `RuntimeLayoutCompleted`와 frame/trace/render 결과 없음. |
| Completed layout에서 diagnostics snapshot 제거 | FFI `grid_track_sizing_trace_report` | `Err("C11.4 layout diagnostic snapshot이 없습니다")` (`Result<String, String>`) | Completed layout이 있어도 성공 report는 없다. |
| diagnostics revision을 completed runtime key와 불일치 | 같은 FFI report owner | `Err("C11.4 trace revision이 완료된 runtime key와 다릅니다")` | stale report를 내지 않는다. |
| semantic phase 없는 `sequence`/`axis` trace만 전달 | C11.4 fixture trace gate | 추가할 test-only `CyclicAutoRepeatTraceGateError::MissingSemanticPhase { node_id }` | Layout completion은 있을 수 있지만 fixture/report/draw/present 성공으로 승격되지 않는다. |

이 오류 경계는 기존 FFI selector/report와 runtime adapter의 실제 return type을 따로 반영한다. 특히 마지막 trace gate 오류는 계획상 추가될 fixture-test error이며 `RuntimeLayoutFailure`나 FFI report 오류가 아니다. 자세한 pass 조건은 <a href="/spinon/plan/c11-4-cyclic-percentage-auto-repeat.html">계획의 error-owner별 negative control 표</a>에 있다.

## Agent Browser 종료

고유 namespace `c114_cyclic`에서 session `c114-cyclic-auto-repeat-20261025`를 열어 확인했다. 작업 후 `agent-browser --namespace c114_cyclic close --all`을 실행했고, session info/list에서 inactive 및 빈 목록, 기존 controller PID `56187`의 종료, namespace 프로세스 부재, profile에 대한 열린 파일 핸들 부재를 확인했다. lifecycle 원자료는 <a href="/spinon/internal/evidence/c11-4-cyclic-percentage-auto-repeat-precomparison-2026-10-25/agent-browser-lifecycle.json">`agent-browser-lifecycle.json`</a>이다. 앞선 기본 namespace session도 해당 session 이름으로 닫았으며 다른 session은 건드리지 않았다.

## 해시

`sha256sum`/`shasum -a 256`으로 아래 입력·관측·기대·화면·브라우저 lifecycle 파일의 SHA-256을 기록한다.

```text
98c9aa84a51085493831aa119461068e9d49210f6e541d96502482e9056d3063  fixture.html
ed51c996834e05395999d015c042c8fee62cfd891d952fc97e3666a12cb9b596  chrome156-observation.json
31ec4e0d6eb2cd231be42b935674b214416e47a3f125072b6101bbec6d26f93f  spec-derived-expectations.json
54f9efab56540c21f3c606c7dd3c4fdf8fe582182a8db114eeac2127318e26f8  chrome156-agent-browser.png
d4d427b2edcae28003bf9686549013ca6f0cce30ef35b679563656f878b2dd18  agent-browser-lifecycle.json
```
