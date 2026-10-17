# S04.10 확인 frame registry 구현 근거 · 2026-10-18

## 판정 범위

PR [#291](https://github.com/ohah/spinon/pull/291)은 2026-10-18 KST Rebase and merge 방식으로 main에 병합됐다(merge SHA `e906aed109a7af2b9f7ccfd0cd64ba4b806f5869`). 최신 `origin/main` 기반 별도 worktree에서 `spinon-render`에 순수 Rust `RuntimeConfirmedFrameRegistry` 후보를 추가했다. 확인 전 submit과 확인된 snapshot을 분리하고, registry identity, FrameId, surface generation, pending 상한과 오류 전이를 단위 테스트로 고정했다. 실제 native presentation callback은 연결하지 않았다.

## 검증

| 명령 | 결과 |
| --- | --- |
| `cargo test --locked --workspace --quiet` | 1067 passed, 0 failed, 4 ignored |
| `cargo test --locked -p spinon-render` | 117 passed, 0 failed, 0 ignored; 기존 scroll/input queue 회귀 포함 |
| `cargo test --locked -p spinon-ffi --no-default-features` | 20 passed, 0 failed, 0 ignored |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | 통과, 경고 없음 |
| `cargo fmt --all -- --check` | 통과 |
| `git diff --check` | 통과 |

계획 검토 20개, 구현 검토 20개와 [PR diff 검토 30개](../s04-10-presented-frame-gate-pr-delta-review-30-2026-10-18.md)는 서로 다른 파일에 기록했다. 구현 검토에서 깊은 snapshot equality API와 superseded discard 오류 분류가 드러나 이를 수정하고 영향 관점을 재확인했다.

## 명세 연결

- 계획: [`plan/s04-10-presented-frame-gate.md`](../../../../plan/s04-10-presented-frame-gate.md)
- 구현 계약: [`0079 · S04.10 확인 frame registry`](../../0079-s04-10-confirmed-frame-registry.md)
- 구현 코드: `crates/spinon-render/src/runtime/presented_frame_registry.rs`
- 테스트: `crates/spinon-render/src/runtime/presented_frame_registry_tests.rs`

## 남은 범위

- Android `VK_GOOGLE_display_timing` `presentID` 및 iOS drawable별 callback 연결
- callback과 입력 event의 직렬 순서, clock domain·timestamp 대응 및 late callback 정책 실행 검증
- surface destroy/recreate 중 callback race의 Android/iOS app runtime 검증
- Android/iOS app route, Simulator UI, 실기기, hardware GPU, compositor 이후 표시와 optical scanout
- S04.10 displayed-frame input, S05.2 React callback, DOM event dispatch 및 C13.3 public support

이 실행은 위 항목을 검증하지 않았다. 확인된 frame handle 후보의 순수 Rust 상태 모델만 다룬다.
