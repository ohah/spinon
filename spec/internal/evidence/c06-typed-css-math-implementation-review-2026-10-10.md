# C06.5 Typed CSS math 구현 실패 경로 검토

- **검토 대상:** `feat/css-c06-unit-conversion`의 C06.5 변경
- **검토 일자:** 2026-10-10
- **상태:** 아래 20개 구현 실패 경로 검토 완료. 구현은 미병합이다.
- **범위 제한:** 코드·고정 fixture·전체 Rust 테스트·정적 CSS reference 검사·Android API 37 emulator·iOS 26.2 Simulator. 실기기, hardware GPU, 성능, 전체 CSS 적합성은 검증하지 않았다.

| # | 공격 입력 / 실패 경로 | 확인 근거 | 결과 |
| --- | --- | --- | --- |
| 1 | Stylo calc 노드가 cascade 종료 뒤 해제되어 dangling pointer가 됨 | `dimensions.rs`가 `ComputedCssMath` owned enum으로 복사하고 `typed_math.rs`가 layout AST로 다시 소유 복사 | Stylo pointer 공유 없음 |
| 2 | CSS 문자열 재파싱이 Stylo와 다른 우선순위·단위를 만듦 | 계산 AST는 `to_typed_value()`에서 추출; `css_math.rs`가 AST만 평가 | 별도 문자열 parser 없음 |
| 3 | winning declaration이 아닌 오래된 `calc()`가 최종 값을 덮음 | `source_math.rs`가 rule-tree winner를 먼저 추리고 computed typed math를 우선함; `typed_css_math_tests.rs`에서 CSS/`var()` 계산 확인 | 현재 fixtures 통과; 독립 cascade 순서 fuzz는 범위 밖 |
| 4 | 알 수 없는 typed 단위나 계산 차원이 layout length로 오인됨 | `computed_math()`의 허용 단위(`number`, `px`, `percent`)와 evaluator dimension 검사 | 미지원 typed AST는 layout 계산으로 성공 전달되지 않음 |
| 5 | 빈 `sum/min/max` 또는 64개 초과 인자에서 panic/과다 계산 | `css_math.rs` arity 검사와 `invalid_math_dimensions_and_resource_limits_still_fail` | 오류 반환 |
| 6 | 32단계 초과 중첩이 재귀 stack을 과도하게 사용 | 같은 테스트의 33단계 `Negate` 입력 | 오류 반환 |
| 7 | 256 AST node cap의 경계 또는 우회 | 같은 테스트의 265-node, 분기당 43-leaf 트리 | 오류 반환; 64-node arity 한도와 별도 확인 |
| 8 | number와 length/percentage를 더해 차원 검사가 무력화 | `dimension_error_discards_the_entire_layout_result`; Rust Chromium 비교 | `InvalidCssMath`, 부분 결과 없음 |
| 9 | length × length 또는 length의 역수가 합법 처리 | `invalid_math_dimensions_and_resource_limits_still_fail`의 product·invert cases | 오류 반환 |
| 10 | `min/max`가 서로 다른 차원이나 NaN을 잘못 선택 | 차원 mismatch 단위 사례, NaN min final censor 사례, Chromium fixture | 차원 오류; NaN은 마지막에 censor |
| 11 | `clamp()` 인자 차원 오류 또는 `min > max` 결과 역전 | dimension mismatch 단위 사례, Chromium `clamp-inverted-bounds` fixture | 차원 오류; inverted bounds는 Chrome의 min 우선 결과 |
| 12 | 0으로 나눌 때 분모 0을 일괄 invalid 처리하거나 부호를 잃음 | positive/negative infinity 단위 사례, Chrome `divide-by-zero`·`zero-divided-by-zero` fixture | ±∞와 NaN을 중간 계산에 보존하고 최종 used value에서 처리 |
| 13 | NaN/∞를 AST 입력 단계에서 거부하거나 Taffy에 무한 frame을 전달 | Chrome `clamp-nan`·divide-by-zero fixture, `non_finite_css_math_intermediate_is_censored_before_a_frame_is_returned` | 최종 width 0 또는 상한으로 유한 frame 생성 |
| 14 | 음수 width와 음수 margin을 동일하게 clamp | Chrome `negative-size`·`negative-margin` fixture; `LayoutCssMathProperty::is_nonnegative()` | 크기 0, margin signed 값 보존 |
| 15 | 계산식 percentage가 해당 property의 단순 percentage basis와 달라짐 | width/height/flex-basis/spacing Chromium fixture 및 C06.1·C06.2 basis validators | 명시된 basis에 따라 계산; 미정 basis를 0으로 대체하지 않음 |
| 16 | cyclic/indefinite percentage gap이나 root gap이 조용히 0으로 계산 | `percentage_basis.rs` 및 기존 percentage spacing tests; C06.5 fixture의 calc gap | 기존 definite-axis 검증 적용; 미지원 root/cyclic 경로 실패 |
| 17 | 중복 `LayoutCalcId`가 다른 표현식으로 덮어써짐 | `duplicate_math_ids_fail_before_taffy_callback` | Taffy 계산 전 전체 입력 거부 |
| 18 | 다른 NodeId/property의 calc ID가 재사용되어 엉뚱한 스타일에 붙음 | `mismatched_math_owner_and_property_fail_before_taffy_callback` | Taffy callback 전 binding 오류 |
| 19 | 임의 opaque pointer 역참조, pointer 재사용 또는 callback placeholder가 성공 frame으로 남음 | `calc_tree.rs`는 owner map 조회만 하고 unknown handle을 dereference하지 않음; unknown-handle 및 전체 결과 오류 테스트; cache는 계산별 tree에 국한 | 안전 placeholder 이후 side-channel 오류로 전체 결과 폐기; 계산 간 cache 공유 없음 |
| 20 | Android/iOS runtime entry가 서로 다르거나 layout 성공 전 화면을 제출 | Node runtime-route 검사 7/7, API 37 emulator·iOS 26.2 Simulator 로그와 캡처 | 양쪽 `status=0`, `layout=ready`, 11 boxes, draw `presented`; Android renderer는 ANGLE/SwiftShader software GL |

## 통합 검증 결과

- `cargo test --locked --workspace`: 통과. C06.5 추가 범위에서 CSS math resource-limit 테스트 포함.
- `cargo clippy --locked --workspace --all-targets -- -D warnings`: 통과.
- `mise exec -- node --test tools/css-reference/c06-typed-css-math.test.mjs`: 7/7 통과.
- `cargo fmt --all -- --check`: 통과.
- 고정 Chrome 154 reference: 25개 지원 node의 각 geometry 축 오차가 `0.5 CSS px` 이내. `round-function`은 지원 결과에서 제외.
- Android API 37 emulator: [원본 로그](c06-typed-css-math/android-api37-emulator.log), [캡처](c06-typed-css-math/android-api37-emulator.png).
- iPhone 17 Pro / iOS 26.2 Simulator: [원본 로그](c06-typed-css-math/ios-26.2-simulator.log), [캡처](c06-typed-css-math/ios-26.2-simulator.png).

이 검토는 C06.5 branch slice에 대한 구현 실패 경로 감사다. 출시 판정이나 100% CSS 호환성 주장이 아니다.

노드 수 한도 회귀 테스트를 설계할 때 단일 연산자 인자 수 한도에 먼저 걸리지 않도록 43개 leaf를 가진 6개 자식 합 트리(총 265 AST node)를 사용했다. 따라서 해당 검사는 operator-arity 오류와 total-node-cap 오류를 구분한다.
