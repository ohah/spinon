# C11.1 `fit-content(percentage)` 구현·runtime 실패 관점 검토 20개

2026-11-03 후보 소스와 기본 Root HTML runtime을 대상으로 서로 다른 경계 실패를 검토했다. 이 문서는 PR 실제 diff 20/30 검토와 별개다. PASS는 아래에 적은 코드·테스트·실행 evidence 범위에서만 판정한다.

| # | 실패 관점 / 적대 조건 | 판정과 근거 |
|---:|---|---|
| 1 | Cascade가 column의 `%`를 px로 바꾸거나 버린다. | PASS — column/row typed snapshot 테스트에서 `Percentage(0.2)`를 보존한다. Cascade 4/4. |
| 2 | `repeat(2, fit-content(20%))` 확장 중 원래 repeat group metadata가 사라진다. | PASS — 두 확장 track과 group count 1을 cascade에서 확인한다. |
| 3 | `fit-content(calc(10px + 5%))`를 direct percentage로 오인한다. | PASS — cascade는 `grid-template-columns` property 경계에서 거부한다. |
| 4 | Projection에서 음수 percentage가 non-negative 입력처럼 통과한다. | PASS — `-0.5`를 Root HTML explicit-track gate가 거부한다. |
| 5 | `NaN`, `+∞`, `-∞`가 projection/layout 중 Taffy로 전달된다. | PASS — 세 비유한 값이 projection에서 거부되고 layout raw-input 검사도 실패한다. |
| 6 | `0%`를 invalid/zero-track으로 취급해 auto minimum을 잃는다. | PASS — `0%` cap에서 Ahem minimum floor 20px/40px를 유지한다. |
| 7 | definite zero-size basis와 indefinite basis를 혼동한다. | PASS — definite 0 width에서 입력을 허용하고 Ahem minimum 20px을 유지한다. |
| 8 | column cap이 높이, row cap이 너비를 basis로 사용한다. | PASS — column/row의 서로 다른 definite dimension과 runtime marker geometry를 각각 검사한다. |
| 9 | 2px border를 percentage basis에 포함한다. | PASS — 30px content box + 양쪽 2px border의 Chrome 156 및 Ahem layout 표본은 모두 15px track이다. 34px border-box를 잘못 쓰면 17px이다. |
| 10 | grid gap을 percentage basis에서 뺀다. | PASS — content width 200px, column gap 20px, `50%` cap에서 track은 100px이다. |
| 11 | 100% 초과 percentage를 typed/cascade 값에서 100%로 clamp한다. | PASS — `120%` typed argument가 cascade/projection/runtime에서 보존된다. 별도 50px available-space 장면의 50px used track은 cap clamp와 구분한다. |
| 12 | 100% 초과 cap을 definite available grid space보다 크게 사용한다. | PASS — `fit-content(120%)`의 수식상 limit 60px이어도 50px definite grid의 Chrome 156 used track과 Ahem layout 결과는 50px이다. |
| 13 | `auto minimum`이 cap보다 클 때 min-content floor를 자른다. | PASS — 0%·20% row cap에서 40px wrapped minimum floor를 유지한다. |
| 14 | max-content가 cap보다 작을 때 track을 cap까지 불필요하게 늘린다. | PASS — Ahem column matrix에서 max-content 110px과 cap 120px의 결과는 110px이다. |
| 15 | expanded fixed repeat의 두 item이 서로의 contribution으로 track 크기를 바꾼다. | PASS — percentage 및 px 반복 장면에서 `[40px, 40px]`, 두 번째 item x=40px을 확인한다. |
| 16 | named area와 percentage fit-content 조합이 기존 제한을 우회한다. | PASS — projection은 `grid-template-areas` 경계에서 거부하고 default runtime은 partial scene을 발행하지 않는다. |
| 17 | `grid-auto-rows: fit-content()` 또는 implicit track으로 explicit-only 범위를 우회한다. | PASS — cascade에서 implicit `fit-content()`를 거부한다. Core validation도 RootHtml explicit mode의 auto track list를 허용하지 않는다. |
| 18 | unsupported auto-repeat, named line, column-flow 또는 조합이 새 track admission으로 열린다. | PASS — Root HTML projection gate는 line names/auto-repeat을 거부하고 column-flow는 direct px만 받는다. 새 코드는 direct `FitContent(Percentage)` admission만 추가한다. |
| 19 | percentage Grid border 예외가 비-Grid item, padding, margin 또는 기존 px-only profile에 번진다. | PASS — border 예외는 해당 element가 Grid container이고 유효한 percentage fit-content track이 있을 때만 적용한다. padding/margin은 계속 0이어야 하며, px-only overflow fixture의 border는 runtime에서 property-bound error로 거부한다. |
| 20 | 잘못된 입력이 frame/render snapshot 일부를 먼저 발행하거나, 정상 overflow가 root border에서 잘린다. | PASS — named-area/indefinite/px-border rejection은 completed scene 부재를 검사한다. Android PixelCopy와 iOS Metal drawable은 root 내부·노출 border·root 바깥 child·child 바깥 배경 4/4 표본과 expected colors를 확인한다. |

## 판정 범위

- Focused gates: percentage layout 12/12 및 전체 `grid_fit_content` 회귀 20/20, cascade 4/4, style-to-layout `fit_content` 7/7(그중 percentage projection 2개), Root HTML runtime 3/3. Style-render 기본 71 passed·1 ignored, C11.4 fixture mode 74 passed·1 ignored. Scoped Clippy와 rustfmt가 통과했다.
- Runtime 픽셀 근거는 Android API 37 ARM64 software-rendered AVD PixelCopy와 iPhone 17 Pro/iOS 26.2 Simulator의 compositor 제출 전 Metal drawable에 한정된다. 실기기·hardware GPU·iOS compositor 이후 픽셀 결과가 아니다.
- 현재 `RootHtmlFitContentV1` core gate는 두 Grid content dimension 모두 definite하도록 요구한다. percentage가 없는 반대 축의 `auto` size는 이 제한 구현에서 지원하지 않는다.
- Ahem text contribution은 공유 `TrackSizing` sizing algorithm에서 비교했다. 기본 Root HTML platform scene은 fixed-size child를 사용한다. 일반 CSS admission, WPT conformance, public CSS support와 전체 C11은 검증하지 않았다.
- 실제 구현 PR diff 검토와 GitHub checks/merge는 아직 남아 있다.

## 연결된 원본

- [계획](../../../../plan/c11-root-html-grid-fit-content-percentage.md)
- [실행 README, screenshots, Chrome observations와 logs](README.md)
- [Android/iOS pixel expectations](expected.json)
- [source/build identity와 artifact hashes](source-revision.json)
- [layout tests](../../../../crates/spinon-layout/src/c15_text_layout_tests/grid_fit_content.rs) · [percentage tests](../../../../crates/spinon-layout/src/c15_text_layout_tests/grid_fit_content/percentage.rs)
- [cascade tests](../../../../crates/spinon-style/src/stylo_dom/cascade/runtime_layout_tests/c11_1_root_html_fit_content.rs) · [projection tests](../../../../crates/spinon-style-to-layout/src/projection/style_values/grid_values/root_html_grid_track_tests/fit_content.rs) · [runtime tests](../../../../crates/spinon-runtime/src/session/tests/root_html_grid_fit_content_percentage.rs)
