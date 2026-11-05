# C11.1 intrinsic Grid fixed-px gap 구현·runtime 적대 검토 20개

**검토 기준:** 구현 source base `b76bf7daa3ecf55c8baa0364100e189fce542872`와 해당 작업 트리의 최종 Rust source, [Chrome 156 비교](../c11-root-html-intrinsic-gap-precomparison-2026-11-05/README.md), [Android/iOS 실행 근거](README.md). 계획 검토와 별도로 20개 실패 경로를 확인했다.

| # | 실패 관점 | 확인과 판정 |
|---:|---|---|
| 1 | gap 허용이 전체 Grid profile로 퍼지는가 | `root_html_intrinsic_gap_is_supported`는 intrinsic track validator에서만 사용한다. **통과**. |
| 2 | one-axis 비intrinsic placement의 기존 zero-gap 계약이 느슨해지는가 | `intrinsic_track_sizing == false`면 기존 fixed-zero predicate만 쓴다. placement 회귀 테스트와 변경부를 확인했다. **통과**. |
| 3 | `row-gap`이 column track 또는 반대로 적용되는가 | 독립 row-only/column-only 입력과 trace를 5개 layout tests에서 확인했다. **통과**. |
| 4 | shorthand가 한 축만 설정하는가 | runtime `both-gap` scene에서 row·column 결과를 Chrome/expected와 비교했다. **통과**. |
| 5 | valid fixed px 외의 단위가 helper에서 허용되는가 | helper는 `LengthPx`만 허용하며 음수·NaN·무한대는 별도 invalid-style 검사로 거부한다. **통과**. |
| 6 | CSS-invalid 음수 선언이 runtime rejection으로 오분류되는가 | `gap:-1px` mutation에서 cascade는 ready, layout은 기존 zero-gap frame으로 완료된다. **통과**. |
| 7 | CSS-valid percentage gap이 0으로 대체되거나 성공 처리되는가 | `column-gap:10%`는 cascade-ready 상태에서 property diagnostic을 내고 completed layout을 publish하지 않는다. **통과**. |
| 8 | CSS-valid math-backed `calc()`가 px로 잘못 평탄화되는가 | `calc(5px + 5px)` runtime mutation이 computed style로 남고 fixed-px-only layout에서 원자 거부된다. **통과**. |
| 9 | 잘못된 typed negative·NaN·infinity가 Taffy까지 도달하는가 | direct layout tests가 세 값을 각각 `InvalidStyle` field로 거부한다. **통과**. |
| 10 | gutter 합 자체 또는 intrinsic base와 결합한 used track extent가 f32 범위를 넘는가 | `f32::MAX` gap과 세 columns는 aggregate precheck가 거부한다. 더 적대적으로 두 min-content base와 finite `0.75 × f32::MAX` gap을 결합하면 gutter 합만은 finite인데 기존 Taffy trace가 `NaNpx`였다. 계산 뒤 detailed track positions를 frame 수집 전에 검사하도록 고쳐 row/column property diagnostic으로 원자 거부한다. 직접 layout 및 parsed runtime regression을 추가했다. **수정 후 통과**. |
| 11 | 하나의 explicit track에서 gap 수를 잘못 `n`개 계산하는가 | helper는 `explicit_track_count.saturating_sub(1)`을 사용한다. 1개 track은 gutter가 없고, 3개 track의 두 gutter overflow negative를 확인했다. **통과**. |
| 12 | gap을 intrinsic minimum contribution에서 빼거나 두 번 반영하는가 | 고정 40px·20px intrinsic bases는 유지되고 gutter는 별도 Grid slot으로 trace된다. four-case expected와 대조했다. **통과**. |
| 13 | definite `fr` 분배가 gutter를 빼지 않은 available space를 쓰는가 | 100px definite axis의 `[40, 50] + 10px` 및 `[20, 70] + 10px` track 결과를 layout trace와 독립 계산에서 확인했다. **통과**. |
| 14 | row/column placement fixture가 현재 intrinsic placement 계약 밖인가 | 두 item은 row 1, tail은 row 2에 고정하고 각 행에서 column auto-placement한다. Chrome과 runtime에서 세 item 위치가 일치한다. **통과**. |
| 15 | absolute frame을 grid-local 값으로 비교하는가 | runtime verifier는 각 item x/y에서 Grid origin을 빼고 root-relative expected와 비교한다. **통과**. |
| 16 | 네 positive scene 중 일부만 layout 완료돼 partial publish되는가 | positive matrix는 4 scene·12 item frame을 모두 요구하고 revision key도 cascade와 일치해야 한다. **통과**. |
| 17 | 실패한 Grid가 같은 document의 성공 scene frame/paint를 남기는가 | percentage, calc, overflow negative에서 cascade-ready 상태와 `completed.is_none()`을 확인했다. **통과**. |
| 18 | gap 추가가 source/style/environment revision 소유권을 약화하는가 | 새 gap 성공 경로가 `completed.key == cascade_completed.key`를 확인한다. 별도 stale-completion guard는 변경하지 않았다. **통과**. |
| 19 | AVD/iOS 성공을 물리 GPU나 pixel parity로 확대 보고하는가 | 두 플랫폼에서 같은 fixture hash, `layout=ready boxes=23`, draw/present를 확인했다. Android는 ANGLE/SwiftShader, screenshots는 visual smoke로만 표기하고 PixelCopy/Metal readback 주장은 제외했다. **통과**. |
| 20 | fixture-test 전용 코드·FFI가 기본 runtime에 추가되는가 | test module은 `c11-1-grid-intrinsic-tracks-fixture-test`로 guard된다. 새 runtime profile·FFI·public API는 diff에 없다. simulator build flags에서 다른 fixture flags는 0이었다. **통과**. |

## 결과

구현/runtime 전용 20개 관점을 검토했다. 10번에서 발견한 gutter aggregate와 track-plus-gutter overflow를 각각 고치고 영향받은 layout 및 runtime negative를 다시 통과시켰다. candidate에 남은 기능 결함은 찾지 못했다. `cargo test --locked -p spinon-layout` 전체는 448개 중 447개 통과이며, C15 HostDocument test 1개 실패는 이 Grid 변경과 무관하고 전체 green으로 주장하지 않는다. C11.1/C11/C11.4, WPT, 실기기·hardware GPU와 compositor 이후 pixel 동등성은 미완료다.
