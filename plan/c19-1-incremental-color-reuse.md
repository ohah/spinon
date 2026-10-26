# C19.1 · computed color incremental 재사용

**상태:** 구현 후보 계획 · C19.1과 C19 전체는 미완료 유지<br>
**기준 revision:** `29449e4f794e53255909b58ed8fb2fd4995dd5d2`<br>
**상위 계약:** [0068 C19.1 computed color carrier](../spec/internal/0068-c19-1-computed-color.md)<br>
**사전 비교:** [구현 전 비교 모델](../spec/internal/evidence/c19-1-incremental-color-reuse-2026-10-26/precomparison.md)<br>
**계획 검토:** [실패 관점 20개](../spec/internal/evidence/c19-1-incremental-color-reuse-2026-10-26/plan-review-20.md)

## 목표

`C19ComputedColorV1` snapshot도 기존 runtime subtree incremental cascade 재사용 API에서 지원한다. dirty subtree를 새로 cascade하고 그 밖의 안전한 요소 출력을 재사용하되, `computed_color`와 `computed_background_color` carrier가 같은 문서 revision의 전체 C19.1 cascade 결과와 정확히 같아야 한다. C19.1 snapshot의 직렬화 property 목록도 `color`·`background-color` 두 Longhand로 한정해 full 및 incremental 결과 구조를 일치시킨다.

현재 `compute_runtime_incremental_cascade_with_stylesheets()`는 `runtime_properties_for_profile()`에서 C19.1을 열거하지 않아 해당 profile에 `None`을 돌려준다. `compute_cascade_with_reuse()` 내부에는 C19.1의 author property 허용 목록, `color`/`background-color` 출력 및 합성 HTML 문서 cascade 처리가 이미 있다. 이 작업은 누락된 incremental profile 등록과 회귀 검증으로 한정한다.

## 범위와 제외

- 전용 `c19_computed_color_properties()`가 `background-color`·`color` Longhand만 반환하고 full C19.1 및 incremental 경로가 같은 목록을 사용한다. 불필요한 layout/font property를 color-carrier snapshot에 직렬화하지 않는다. author CSS 허용 목록은 기존 경계를 유지한다.
- 변경된 inline style dirty root와 그 하위 요소를 다시 계산한다. 필요한 상위 조상 cascade는 context로 계산하고 이전 직렬화 출력을 재사용한다.
- 새 incremental snapshot의 모든 요소 및 diagnostics가 같은 현재 문서에 대한 full C19.1 cascade와 정확히 같은지 검사한다. 특히 직렬화 property key가 두 색 property뿐인지, `computed_color`·`computed_background_color`의 source color space, component, missing flags, alpha, `converted_srgb`, `gamut_mapped_srgb` 전체가 같은지, legacy `background_color`·`background_paint` 필드가 계속 `None`인지 확인한다.
- 기존 API guard를 보존한다. profile 불일치, viewport/environment 불일치, 문서 generation/ revision 경계 위반, 범위 밖 `style_revision`, author stylesheet 입력은 재사용을 거부해 호출자가 full cascade로 돌아가게 한다.
- cascade 색 계산, gamut mapping, system color palette, paint output, WCAG 계산, hit-test, DOM/CSS 공개 API, simulator 및 GPU 동작은 바꾸지 않는다.
- `spec/STATUS.md`의 C19 설명에는 구현 근거 링크만 추가한다. C19 checkbox와 공식 완료 수 `72/213`, 공개 roadmap 및 완료 판정은 바꾸지 않는다. 작업은 C19.1 cascade cache 경계의 내부 동작만 다룬다.

## 변경 예정 파일

- `crates/spinon-style/src/stylo_dom/cascade/incremental.rs`: C19.1 profile에 기존 property 목록 연결.
- `crates/spinon-style/src/stylo_dom/cascade/runtime_paint.rs`: full C19.1 cascade도 동일한 2-Longhand 목록을 쓰도록 제한.
- `crates/spinon-style/src/stylo_dom/cascade/incremental_tests.rs`: dirty subtree, context ancestor, sibling 재사용, full cascade 동등성 및 거부 guard 회귀를 추가.
- `spec/internal/0068-c19-1-computed-color.md`: C19.1 incremental reuse의 입력 전제와 full cascade fallback 조건 기록.
- `spec/STATUS.md`: C19 설명에 이 제한 구현의 근거 링크 추가, 체크와 공식 완료 수는 유지.
- `spec/internal/evidence/c19-1-incremental-color-reuse-2026-10-26/`: 비교 기준, 구현 관점 검토와 실행 결과.

## 검증 계획

1. 고정된 4-element fragment tree에서 이전 C19.1 full snapshot을 계산한다. dirty container의 inherited `color`와 자식 `background-color` 선언을 변경하고 style revision을 한 단계 올린다.
2. 현재 문서의 full C19.1 cascade와 incremental 결과를 비교한다. element 순서, `ComputedCssColor` 두 carrier, `color`·`background-color`만 있는 직렬화 properties, legacy paint 필드 `None`, diagnostics, document/render-tree revision이 일치해야 한다.
3. parent context는 다시 cascade되지만 이전 output을 유지하고, dirty container와 자식은 재계산되며, 변경되지 않은 sibling은 재사용된다는 통계를 확인한다.
4. author stylesheet 입력, profile 불일치, viewport 불일치, 잘못된 문서 revision, 한 단계보다 더 먼 style revision은 `None`을 반환해야 한다.
5. `cargo fmt --all -- --check`와 `mise exec -- cargo test -p spinon-style c19_computed_color_incremental_reuse`를 실행한다. 필요하면 focused crate 전체 test로 범위를 넓힌다. Simulator, browser, GPU build는 이 순수 Rust 변화의 판정에 필요하지 않다.
6. 구현 후 계획 검토와 다른 20개 실패 관점을 작성하고, 변경 PR에는 별도 30개 diff 검토를 기록한다.

## 완료 경계

이 계획의 구현은 C19.1 색상 계산 알고리즘이나 전체 C19 기능을 완료하지 않는다. text glyph paint, C19.2 compositor와 C19.3 통합은 각각의 기존 계약과 완료 기준을 따른다.
