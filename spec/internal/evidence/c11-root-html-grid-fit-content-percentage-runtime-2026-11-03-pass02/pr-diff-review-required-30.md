# C11.1 `fit-content(percentage)` 필수 PR diff 검토 30개

대상 PR [#1106](https://github.com/ohah/spinon/pull/1106), 검토 head `b78af0e1e9f52b098a8fbd78d15dd037124fc494`. 프로젝트 규칙의 병합 전 검토로 실제 코드·문서 diff를 독립 실패 경로 30개에서 살폈다. 계획 검토 30개와 구현/runtime 검토 20개를 재사용하지 않았다.

| # | 파일·실패 경로 | 판정과 근거 |
|---:|---|---|
| 1 | `GridPlacementMode::RootHtmlFitContentV1` 문서가 허용 단위를 잘못 설명한다. | PASS — enum 설명이 px와 percentage를 모두 명시한다. |
| 2 | core `validate_tracks`가 negative/non-finite percentage를 통과시킨다. | PASS — `is_finite()`와 `>= 0.0` 조건이 length/percentage 두 variant 모두에 적용된다. |
| 3 | grid projection의 `Calc` variant가 새 match에 빠져 wildcard로 허용된다. | PASS — `FitContent` 내부의 px/percentage만 허용하고 `Calc(_)`는 false다. |
| 4 | minmax/기타 explicit track admission이 넓어진다. | PASS — 변경된 allowlist arm은 `FitContent`에 한정되고 기존 `minmax` 검사식은 바뀌지 않았다. |
| 5 | column-flow 지원이 percentage track까지 우연히 열린다. | PASS — 기존 column-flow 제한 검사가 별도로 유지되며 percentage support는 일반 explicit track projection에만 추가됐다. |
| 6 | fixed repeat expansion의 repeat-group provenance가 손실된다. | PASS — 기존 group count를 보존하는 cascade test와 projection test가 있다. |
| 7 | percentage 비율이 style snapshot에서 layout 값으로 변환될 때 단위/분모가 손실된다. | PASS — typed `Percentage(f32)`를 그대로 투영하고 0.2/1.2 test로 round trip을 확인한다. |
| 8 | limit이 axis-specific definite size가 아닌 viewport 기준으로 계산된다. | PASS — shared sizing 테스트는 Grid의 definite content axis dimensions를 개별 입력으로 설정한다. |
| 9 | two-axis definite gate를 문서가 과장하거나 숨긴다. | PASS — plan, internal contract, runtime evidence가 두 content dimensions 조건과 반대 축 `auto` 제외를 기록한다. |
| 10 | `validate_root_html_grid_box_inputs`의 border 허용이 non-Grid node에 전파된다. | PASS — `is_grid_container` guard와 `grid_layout` track scan이 함께 필요하다. |
| 11 | border 허용 predicate가 `%` 값의 유한성·부호 검사를 생략한다. | PASS — projection predicate에도 finite non-negative guard가 있다. |
| 12 | Root HTML renderer 변경으로 다른 profile에 border가 그려진다. | PASS — 기본 경로는 `RuntimeRootHtmlTextPaintV1`와 percentage fit-content를 함께 요구하며 `RuntimeBlockFlexGridTrackSizingV1`은 기존 feature-gated 경로를 유지한다. |
| 13 | runtime border helper에서 색상 변환 실패가 무시된다. | PASS — 각 side의 conversion `Result`가 전파되고 invalid `RuntimeBorderSide`도 전파된다. |
| 14 | 기존 C11.4 border fixture 동작을 회귀시킨다. | PASS — 기존 fixture profile branch는 남아 있으며 recorded C11.4 fixture suite 74 passed·1 ignored 및 fixture-mode Clippy가 있다. |
| 15 | renderer의 background/clip 계산 순서가 바뀐다. | PASS — border를 추가한 뒤 기존 Root HTML text-paint clip을 설정하는 순서가 유지된다. |
| 16 | layout regression test의 기존 px 경로를 제거한다. | PASS — 기존 px suite를 유지하며 삭제된 단일 assertion은 percentage module로 분리돼 positive coverage가 확장됐다. |
| 17 | 분리된 `percentage.rs`가 테스트 harness/import 범위 밖에 놓인다. | PASS — parent module의 `#[path]` 선언으로 동일 test module에서 compile되며 전체 20/20 Grid fit-content gate 로그가 남아 있다. |
| 18 | runtime test fixture의 상대경로가 이동 후 잘못 가리킨다. | PASS — `include_str!` 경로가 저장소 `tests/fixtures/css/c11`을 가리키며 Root HTML runtime 3/3이 실행됐다. |
| 19 | runtime의 painted-frame 선택이 장면 색상 중복으로 다른 node를 검사한다. | PASS — fixture가 각 marker에 고유 색을 주고 frame을 색상별로 검사한다. |
| 20 | runtime test가 Root HTML이 아닌 별도 synthetic route를 검증한다. | PASS — `RuntimeSession::new_runtime_gpu_with_root_html`로 parsed HTML을 실행하고 default route의 layout/cascade/render snapshot revision을 비교한다. |
| 21 | out-of-scope runtime test가 오류 property 대신 실패 상태만 확인한다. | PASS — 각 fixture가 기대 property 문자열과 `completed.is_none()`을 모두 assert한다. |
| 22 | Android PixelCopy 플래그가 비-test app build에서도 활성화된다. | PASS — capture 요청은 `BuildConfig.SPINON_C11_GRID_METAL_SURFACE_PIXELS_TEST`로 제한된다. |
| 23 | Android에서 percentage flag가 기존 minmax/px fixture flag와 동시에 켜진다. | PASS — 충돌 조합은 명시적으로 error log를 내고 percentage request는 기존 두 capture와 배타적이다. |
| 24 | Android에서 잘못된 run ID나 작은 표면을 사용해 표본을 잘못 읽는다. | PASS — run ID pattern, positive surface dimensions, CSS viewport bounds를 PixelCopy 전에 검사한다. |
| 25 | Android 기존 px capture의 7개 sample/order가 percentage branch 때문에 바뀐다. | PASS — 기존 sample list와 expected RGBA는 non-percentage branch에 그대로 남아 있다. |
| 26 | Android screenshot output이 이전 run의 동일 파일과 혼합된다. | PASS — percentage capture는 전용 cache directory와 scene ID를 사용하며 PixelCopy summary에 output SHA-256을 남긴다. |
| 27 | Android scene generation 종료/재생성 시 stale callback이 완료로 오인된다. | PASS — 기존 `generation`을 callback·summary에 기록하고 surface capture helper의 generation guard를 재사용한다. |
| 28 | iOS 실행 인자가 px prefix 분기에서 잘못 읽히거나 허용 문자가 넓어진다. | PASS — percentage prefix를 별도 인식하고 8–80자 ASCII alphanumeric/`-`/`_` run ID 검증을 재사용한다. |
| 29 | iOS/Android capture harness가 fixture/test-only 경계를 제품 CSS API로 노출한다. | PASS — 변경은 app demo PixelCopy/Metal capture argument와 test fixture route에 한정되고 일반 cascade admission은 기존 Root HTML profile 내부다. |
| 30 | PR/roadmap 증거가 merge 전 후보를 공식 완료나 CI-green으로 표시한다. | PASS — C11.1/C11 checkbox와 공식 `72/213`은 유지하고, evidence는 simulator 경계·빈 PR check rollup·미지원 기능을 구분한다. |

## 결과와 범위

검토에서 병합을 막을 수정 결함은 발견하지 못했다. 세부적으로 `main` branch protection의 `required_status_checks`는 null이고 PR #1106 check rollup은 비어 있다. 따라서 CI 통과를 주장하지 않는다. Android는 API 37 ARM64 SwiftShader AVD이며 iOS는 iPhone 17 Pro Simulator Metal drawable의 compositor 제출 전 표본이다. 사용자 실기기, hardware GPU, WPT, 일반 CSS admission, 전체 C11은 이 판정에 포함되지 않는다.

## 확인한 자료

- [계획](../../../../plan/c11-root-html-grid-fit-content-percentage.md) · [내부 계약 0059](../../0059-c11-grid-layout.md)
- [실행 및 source/build evidence](README.md) · [구현/runtime 검토 20개](implementation-runtime-review-20.md)
- [변경 코드와 테스트](../../../../crates/spinon-layout/src/grid.rs) · [projection](../../../../crates/spinon-style-to-layout/src/projection/style_values/grid_values.rs) · [render adapter](../../../../crates/spinon-style-to-render/src/runtime.rs)
- [runtime contract tests](../../../../crates/spinon-runtime/src/session/tests/root_html_grid_fit_content_percentage.rs)
