# PR #738 runtime source 누락 복구

## 결함

`generic_intrinsic_flex_fixed.rs`는 `#[path = "generic_intrinsic_flex_fixed/visibility_hidden.rs"]`로 모듈을 열고 C11.4 fixture 테스트도 해당 adapter를 호출하지만, PR #738의 merge tree에는 이 경로가 없었다. 기존 로컬 체크아웃에는 223줄 파일이 미추적 상태로 남아 있어서 로컬 테스트는 실행됐지만, 깨끗한 clone에서는 fixture feature를 켤 때 모듈 파일을 찾을 수 없다.

PR #738 파일 목록은 runtime 부모 모듈과 테스트 파일을 포함하지만 이 adapter 파일은 포함하지 않는다. 이번 변경은 기존 파일의 바이트를 새 동작으로 고치는 것이 아니라, 이미 참조되는 소스를 Git 추적에 추가한다.

## 검증

| 확인 | 결과 |
| --- | --- |
| 기존 untracked source blob | `bf94b8f953af3a6cc8b13d781db4e637e8c4042d` |
| `cargo fmt --all -- --check` | 통과 |
| `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test visibility_hidden_contributes_to_columns_and_rows_without_a_visible_control_regression` | 1 passed, 290 filtered |
| `cargo clippy --locked -p spinon-runtime --lib --features c11-4-grid-track-sizing-fixture-test` | 통과 |
| 전체 workspace Clippy, `-D warnings` | 차단: `spinon-ffi/src/runtime_layout.rs:139` 테스트 initializer의 `grid_track_sizing_diagnostics` 누락, 기존 runtime fixture의 `clippy::map_identity` 경고 |

전체 workspace Clippy 실패는 이 변경 파일 이외의 기존 코드에서 발생했다. 해당 결과를 새 파일의 실패나 전체 Clippy 통과로 돌려 말하지 않는다.

## 기존 동작 화면

아래 이미지는 이번 변경에서 다시 실행하거나 캡처하지 않았다. PR #738의 merge commit SHA에 고정된 Android API 37 AVD와 iOS Simulator 캡처이며, Agent Browser에서 이미지 본문이 로드되는 것을 확인했다.

| Android API 37 AVD | iPhone 17 Pro / iOS 26.2 Simulator |
| --- | --- |
| ![Android row visibility hidden](https://raw.githubusercontent.com/ohah/spinon/bf4ab21eebc2f3739a7ae61de6511b92733b9464/spec/internal/evidence/c11-4-mixed-intrinsic-flex-fixed-runtime-2026-10-28/runs/c114-mixed-intrinsic-flex-fixed-android-avd-20261028T055342204Z-d0ead38b/android-row-visibility-hidden.png) | ![iOS row visibility hidden](https://raw.githubusercontent.com/ohah/spinon/bf4ab21eebc2f3739a7ae61de6511b92733b9464/spec/internal/evidence/c11-4-mixed-intrinsic-flex-fixed-runtime-2026-10-28/runs/c114-mixed-intrinsic-flex-fixed-ios-simulator-20261028T054646781Z-7759349f/ios-row-visibility-hidden.png) |

이미지 원본 크기는 각각 1080×2400 및 1206×2622다. 브라우저 세션은 닫았고 `agent-browser` 프로세스가 남지 않은 것을 확인했다.

## 적대적 검토

- [구현/runtime 20개 실패 관점](implementation-runtime-review-20.md)
- [PR diff 20개 실패 관점](pr-diff-review-20.md)

이 기록은 한 에이전트가 분리해 수행한 관점별 검토이며, 외부 독립 reviewer 20명의 승인을 의미하지 않는다. 공식 C11.4 완료 수와 상태 대장은 변경하지 않았다.
