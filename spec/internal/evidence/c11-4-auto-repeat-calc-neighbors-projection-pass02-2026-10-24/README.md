# C11.4 auto-repeat `calc()` neighbor runtime rejection 진단 · 2026-10-24

## 결과

표면 pixel fixture를 활성화한 현재 소스로 Android API 37 ARM64 AVD를 실행했다. 첫 12 scene은 통과했지만, 13번째 `calc-neighbor-col-max-content`에서 capture가 중단됐다. [원본 capture log](./android-capture.log)와 [scene runtime log](./android-avd-c114-calc-neighbor-projection-boundary-android-avd-20261024T035916409Z-535bec8a/runtime-calc-neighbor-col-max-content.log)를 보존했다.

이 scene은 예상대로 `InvalidGridTrackList`로 layout을 거부했다. 그러나 runtime error mapper가 이 typed error를 처리하지 않아 report가 `layout_failed`, `node=None`, 일반 Taffy property text로 내려갔다. fixture는 `invalid_grid_track_list`, node 1, `grid-template-columns`를 기대하므로 exact rejection assertion이 실패했다.

`layout_failure_from_error`에 `InvalidGridTrackList`의 안정된 code·node·property mapping을 추가하고 같은 mapping을 unit test로 고정했다. 회귀 test 1/1과 `cargo fmt --all -- --check`가 통과했다. 이후 수정된 앱을 두 플랫폼에서 다시 빌드하고 전체 14-scene matrix를 통과했다. [pass03 결과](../c11-4-auto-repeat-calc-neighbors-projection-pass03-2026-10-24/README.md).
