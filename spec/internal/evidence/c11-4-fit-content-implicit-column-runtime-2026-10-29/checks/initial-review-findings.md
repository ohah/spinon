# 초기 검증에서 수정한 내용

- 첫 fixture-feature Clippy에서 `GridTrackSize::clone_on_copy` 2건을 발견했다. 두 clone을 제거하고 전체 workspace Clippy를 다시 실행해 통과했다.
- 첫 FFI cargo test filter는 0개를 선택했다. 이 로그는 `ffi-tests-zero-selected-initial.log`에 보존하고, exact test `exposes_only_the_pinned_implicit_fit_content_column_scene`를 다시 지정해 1/1 통과를 확인했다.
- 첫 Rows stage expectation과 축별 `StretchAutoTracks.applied` 해석이 실행 trace와 달랐다. 행 base 배열을 고치고 `applied` 의미를 Taffy 내부 진단으로 문서화했다.
- 구현 리뷰에서 Rows contribution 검사가 빠진 경계를 찾아 Rust validator·FFI report·Android/iOS host matcher에 `AutomaticMinimum=10`, `MinContent=10`, `MaxContent=10` 검사를 추가하고 simulator 실행을 다시 했다.
- 초기 simulator bring-up에서 viewport/title과 gutter expectation을 수정했다. 최종 Android/iOS 실행은 각각 `c114-implicit-col-android-20261029k`와 `c114-implicit-col-ios-20261029e`이며 이전 중간 캡처를 최종 근거로 사용하지 않는다.
