# C13.3 typed style projection 근거

2026-10-15의 `feat/c13-3-scroll-style-20261015` 작업에서 C13.3 제한 computed-style profile을 추가했다. 검증 checkout은 `238041a5fd6c288ed0624c40cdc553dd9770b208` 위에 있다. `rustc 1.96.1`, `cargo 1.96.1`, `aarch64-apple-darwin`에서 실행했다.

## 범위와 판정 기준

- `RuntimeBlockFlexScrollGestureV1`은 기존 C13.1 overflow 결과에 물리 축 `overscroll-behavior-x/y`와 typed `touch-action`을 더한다.
- `overscroll-behavior`의 shorthand 1/2 값, 뒤이은 longhand 우선순위, non-inherited initial을 확인한다. 논리 축 property는 거부한다.
- `touch-action`은 `auto`, `none`, `pan-x`, `pan-y`만 실행 가능한 축 값으로 제공한다. Stylo가 값은 보존하지만 제한 집합 밖인 합법 값과 조합은 `Unsupported`로 남긴다.
- pinned Stylo 0.22가 기본 설정에서 계산값을 조용히 `auto`로 낮추는 `pan-left/right/up/down`은 inline 및 중첩 author rule에서 사전 거부한다. `var()`는 계산 전 입력 손실을 피하려고 거부한다.
- 새 profile은 기존 layout projection까지 전달하지만 renderer의 scroll policy에는 연결하지 않는다. C13.3 완료 판정은 아니다.

## 구현 중 확인한 회귀 경계

Stylo 0.22의 `layout.unimplemented` preference가 꺼져 있으면 `overscroll-behavior` declaration을 파싱하지 않고, 켜면 기존 미지원 속성 `offset-path`도 새로 파싱한다. 이를 프로세스 전역에서 항상 켜 두는 변경은 C12.2의 미지원 진단을 깨뜨렸다.

이번 profile은 `layout.unimplemented`를 C13.3 파싱/cascade 구간에서만 활성화한다. 전역 `RwLock`의 writer guard가 설정을 켜고 drop 시 기본값으로 되돌린다. 일반 `StyloDocumentView` 생성과 공개 stylesheet append는 reader guard를 취하며, C13.3은 고정된 snapshot에서 profile 전용 view를 writer guard 안에서 다시 만든다. cascade 중 stylesheet를 등록하는 내부 경로는 이미 guard를 소유한 상태에서 호출해 재귀 잠금을 피한다. C13.3 증분 cascade 재사용은 아직 허용하지 않는다.

동시 C13.3/C12 위치 cascade 회귀 테스트를 추가했다. 8회 반복 시 C13.3은 `overscroll-behavior:contain`을 typed `Contain`으로 보존하고, C12 위치 profile은 `offset-path` 진단을 계속 보존했다.

## 검증 결과

로그는 이 디렉터리에 보존했다.

- `focused-style-test.log`: `cargo test --locked -p spinon-style runtime_scroll_gesture_tests -- --nocapture` — 8 passed.
- `focused-layout-test.log`: `cargo test --locked -p spinon-style-to-layout c133_scroll_input_style_profile_keeps_existing_layout_projection -- --nocapture` — 1 passed.
- `workspace-test.log`: `cargo test --locked --workspace` — 821 passed, 0 failed, 4 ignored.
- `workspace-clippy.log`: `cargo clippy --locked --workspace --all-targets -- -D warnings` — 통과.
- `rustfmt.log`: `cargo fmt --all -- --check` — 통과.
- `diff-check.log`: `git diff --check` — 통과.

각 focused test는 inline/author stylesheet, shorthand cascade, typed unsupported state, non-inherited default, 물리 축 allowlist와 기존 C13.1 범위를 검사한다. stylesheet의 `@media` 중첩 안 `pan-right`도 preflight 오류를 발생시킨다.

## 남은 범위와 한계

이 style-only 변경에는 C13.3 gesture를 호출할 앱 fixture route나 표시 화면이 없어 Android/iOS Simulator를 실행하지 않았고 앱 screenshot도 없다. 고정 Chrome 155 input oracle은 별도 사전 비교 근거이며 이 profile의 Spinon 화면이나 runtime gesture 실행을 증명하지 않는다. renderer scroll-policy 연결, gesture state/queue, native input adapter, 앱 route 및 default/opt-in fixture 경계가 남아 있어 C13.3은 미완료다. incremental restyle은 `RuntimeBlockFlexScrollGestureV1`에 대해 전체 cascade fallback을 사용한다.
