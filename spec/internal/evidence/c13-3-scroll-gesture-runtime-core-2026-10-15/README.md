# C13.3 snapshot delta-router 실행 근거

## 변경 후보

- 기준 `main`: `705a5f5409628437b9f20b9ab47324a753c4f61c` (C13.3 계획 PR #177 병합 commit)
- 구현 후보: `4c2c1829` (`feat: C13.3 축별 snapshot delta router 조각 추가`)
- 구현 worktree: `/private/tmp/spinon-c13-3-scroll-gesture-impl-20261015`
- 범위: `spinon-render`의 immutable snapshot 기반 per-axis scroll delta routing primitive와 14개 C13.3 추가 단위 검증
- 계획: [C13.3 gesture plan](../../../../plan/c13-3-scroll-gestures.md) · [내부 계약 0063](../../0063-c13-3-scroll-gestures.md)

이 조각은 policy 입력형 pure Rust API다. 기본 `RuntimeScrollContainer::new`는 user input을 두 축 모두 비활성화한다. CSS cascade에서 `overscroll-behavior`/`touch-action`을 추출하거나, pointer gesture/session을 생성하거나, worker input queue 및 Android/iOS host callback과 연결하지 않았다. 따라서 이 실행은 제품 제스처·시뮬레이터 UI를 검증하지 않는다.

## 검증

| 명령 | 결과 |
| --- | --- |
| `cargo test --locked -p spinon-render` | 32 passed, 0 failed |
| `cargo test --locked --workspace` | 812 passed, 0 failed, 4 ignored |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | 통과 |
| `cargo fmt --all -- --check` | 통과 |
| `git diff --cached --check` | 통과 |

원본 출력은 `focused-test.log`, `workspace-test.log`, `workspace-clippy.log`, `rustfmt.log`, `diff-check.log`에 보존한다. `spinon-render`는 focused 32/32를 통과했고 전체 workspace 결과에도 포함된다.

## 검토

- [구현 실패 경로 20개](implementation-review-20.md)
- [PR 변경 관점 30개](pr-delta-review-30.md)

## 남은 관문

- C13.3 typed style profile과 unsupported `touch-action`/overscroll 값의 fail-closed 경계
- target-to-scroll-owner path의 touch-action 교집합 및 gesture 시작 시 freeze
- begin/pan/move/end/cancel state, pointer identity, geometry key, surface generation과 stale barrier
- per-owner record allocation, input ordering, reserved terminal slot 및 queue pressure
- Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator의 native callback/fixture route, 화면·trace·app hash
- fixture FFI default-artifact negative control과 opt-in positive control
- S04.10 displayed-frame targeting 및 S05 event/adapter 계약은 이 항목과 별도 미완료

이번 pure Rust 조각은 화면을 바꾸지 않아 Android/iOS app build와 Simulator 실행을 하지 않았다. 실기기도 연결하지 않았다.
