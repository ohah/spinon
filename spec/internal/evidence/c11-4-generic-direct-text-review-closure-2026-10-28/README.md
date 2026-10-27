# C11.4 generic direct-text review closure

## 범위

PR #645의 후속 diff 검토 addendum에서 남긴 세 가지 PARTIAL을 고정 fixture tests로 닫는다.

- SVG namespace의 `img`와 HTML namespace의 `image`는 HTML `<img>` 거부 조건을 만족하지 않는다.
- 실제 HTML `<img>`가 거부될 때 오류 `node_id`는 그 이미지의 HostDocument `NodeId`와 같아야 한다.
- 한 항목 legacy `TEXT_CASES` scene은 generic collector가 `None`을 반환한 뒤 전체 `c11_4_text::prepare` dispatch를 통해 legacy input으로 준비된다.

변경은 fixture-test 전용 Rust tests와 근거 문서뿐이다. 제품 코드, CSS admission, FFI, Android/iOS adapter, 내부 계약, 공개 API는 바꾸지 않는다. C11.4 checkbox는 미완료로 두고 공식 집계 `72/213`을 유지한다.

## 검증

- `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test generic_direct_text_tests::` — 11 passed, 0 failed.
- `cargo fmt --all -- --check` — 통과.
- `git diff --check` — 통과.
- 기본 targeted Clippy 명령은 변경과 무관한 기존 `clippy::type-complexity` 경고를 `crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_grid_layout_auto_tracks/validation.rs:329`에서 거부했다. 해당 경고만 허용한 `cargo clippy --locked -p spinon-runtime --all-targets --features c11-4-grid-track-sizing-fixture-test -- -D warnings -A clippy::type-complexity`는 통과했다.

테스트 전용 변경이므로 Android/iOS app build, simulator, screenshot, surface readback은 실행하지 않았다. 이 문서는 앱 runtime이나 표면 출력을 검증했다고 주장하지 않는다.

## 적대적 변경 검토

[PR diff의 독립 실패 관점 20개](./pr-diff-review-20.md)를 참조한다. 이 검토는 runtime 구현 변경 또는 전체 C11.4 적합성 검증을 대신하지 않는다.
