# C11.4 제한 track-sizing trace 후보

이 기록은 `feat/c11-4-track-sizing-algorithm-20261019` worktree의 Rust 후보 검증을 정리한다. 코드는 아직 main에 병합되지 않았다.

## 변경 범위

- Spinon의 `GridTrackSize` typed input과 `RuntimeBlockFlexGridTrackSizingV1` profile을 Taffy `0.15.0`에 연결한다.
- `LayoutOutput.grid_track_sizing_traces`는 opt-in profile에서만 값을 가진다. trace에는 grid owner, 열·행 sizing pass, 단계별 track base size/growth limit, Taffy가 요청한 item min-content/max-content/minimum contribution query와 minimum source가 들어간다.
- Taffy는 trace용으로 `vendor/taffy`에 보존한다. patch 목록과 출처/checksum은 `vendor/taffy/PATCHES.md`에 기록한다. 기존 profile은 trace가 기본 비활성이다.
- Track state 100,000개와 contribution 100,000개 상한을 적용한다. trace 공간 예약 실패, 상한 초과, source item ID 대응 실패는 `LayoutError`로 반환하며 부분 `LayoutOutput`을 게시하지 않는다.

## 실행 결과

- `cargo check --workspace`: 통과.
- `cargo fmt --all -- --check`: 통과.
- `cargo test --locked -p spinon-layout grid -- --nocapture`: 41/41 통과.
- `cargo test --locked -p spinon-layout out_of_span_auto_track_does_not_change_text_item_minimum -- --nocapture`: 1/1 통과. Ahem `XXXXXXXX`의 min-content width는 80 CSS px다. span 밖의 `auto`와 `1fr` 추가는 20px 고정 item span을 넓히지 않고, span 안 `auto`는 content-based minimum을 선택한다. 반환 trace의 source metadata도 확인한다.
- `cargo test --locked -p spinon-style-to-layout track_sizing -- --nocapture`: 2/2 통과.
- `cargo clippy --locked -p spinon-layout --all-targets -- -D warnings`: 통과.
- `cargo clippy --locked --workspace --all-targets -- -D warnings`: 기존 `crates/spinon-style/src/stylo_dom/cascade/incremental/compute.rs:44`의 `compute_cascade_with_reuse` 8개 인자 `too_many_arguments`에서 실패한다. 이번 변경 파일은 실패 원인이 아니다.

## 초기 trace 비교

이 trace의 초기 비교에서는 Chrome 155 oracle 11/26개(이미지 8, span 3)를 기록했다. 그 뒤 같은 worktree에서 확장된 Rust frame matrix 26개 case(이미지 8, span 3, text 10, indefinite percentage 1, auto-repeat 4)가 통과했다. 기존 [image oracle matrix](./chrome155-image-matrix-2026-10-19.md)와 [span matrix](./chrome155-spanning-matrix-2026-10-19.md)는 처음 11개를 stage trace와 함께 대조한 내역으로 보존한다. 확장 matrix와 trace의 차이는 아래 후속 결과를 참조한다.


## 2026-10-19 후속: Chrome 고정 frame matrix 및 CSS auto-repeat 후보

이후 같은 후보 worktree에서 Chrome 155 fixed oracle의 26개 case를 Rust frame matrix와 대조했다: image 8, spanning 3, text 10, indefinite percentage 1, auto-repeat 4. `chrome155_*` 테스트 7개가 통과했다. 이는 26개 전체에서 모든 Taffy stage trace를 독립 규범 표와 대조했다는 의미는 아니다.

Stylo cascade와 style-to-layout projection에는 한 축의 단일 auto-repeat component 및 고정 최소 길이 pattern subset을 추가했다. CSS→cascade→projection→layout auto-fill/auto-fit 검사와 Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator의 7-scene frame matrix가 통과했다. auto-repeat의 세 앱 screenshot에서 item-color extent가 100px grid 바깥으로 약 22 CSS px 이어지는 것도 측정했다. border stroke 색 자체는 pixel assertion에 포함되지 않았다. 상세 앱 실행과 색 영역 측정은 [Simulator 근거](../c11-4-grid-track-sizing-simulator-2026-10-19/README.md)와 [auto-repeat 후속](../c11-4-grid-track-sizing-simulator-2026-10-19/auto-repeat/README.md)에 있다.

후속 전체 검증은 `CARGO_INCREMENTAL=0 cargo test --locked --workspace`, `CARGO_INCREMENTAL=0 cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`를 통과했다. CSS contribution 선택, 독립 normative stage oracle, app trace publication, 전체 repeat grammar·named-line/area interaction, WPT는 여전히 미완료다. 이 구현은 C11.4 또는 전체 CSS Grid 완료를 뜻하지 않는다.
