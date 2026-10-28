# C11.4 mixed intrinsic/flex/fixed 입력 거부 검토 20개

## 범위

PR #729의 계획에 있었지만 신규 generic adapter에서 개별 실행되지 않았던 비이미지 입력 변조를 검토한다. 앞선 [이미지 입력 거부 후속](./image-negative-controls-followup-review.md)은 source URL·decode·natural-size를 다뤘다. 이 검토는 Columns와 Rows에서 텍스트, direct-child 구조, root/item computed style, track list, 축, revision/profile 변조가 C11.4 mixed adapter에서 실패하고 다른 fixture로 조용히 넘어가지 않는지 확인한다.

제품 알고리즘, public API, FFI route, app scene/payload는 변경하지 않았다. 고정 fixture-only Rust 경로 검증이므로 Android/iOS 앱·Simulator 실행 근거로 취급하지 않는다.

## 실행 근거

```sh
cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test \
  generic_intrinsic_flex_fixed::tests::negative_controls -- --nocapture
cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test
cargo fmt --all -- --check
cargo clippy --locked -p spinon-runtime --all-targets \
  --features c11-4-grid-track-sizing-fixture-test -- \
  -D warnings -A clippy::type_complexity -A clippy::map_identity
git diff --check
```

새 negative-control test 6개가 통과했고, 두 축의 조합을 포함한 50개 입력 변조가 모두 dispatcher 오류로 끝났다. 전체 `spinon-runtime` fixture-feature suite는 unit 284 passed / 3 ignored, integration 3 passed와 16 passed였다. `fmt`, 위 두 기존 lint만 제외한 Clippy, `git diff --check`가 통과했다. strict Clippy는 변경하지 않은 `c11_4_grid_layout_auto_tracks/validation.rs:329`의 `type_complexity`와 `generic_direct_text_image/fixture.rs:311`의 `map_identity` 두 건으로 실패했다. Android/iOS 앱 matrix는 실행하지 않았다.

## 독립 실패 관점 20개

| # | 공격 가설 | 관찰·판정 |
|---|---|---|
| 1 | A text가 같은 길이의 다른 문자여도 100px min-content 입력으로 오인한다 | `XXXXXXXXXY`를 두 축에서 거부했다. |
| 2 | C text의 40px 대조 입력이 일부 바뀌어도 허용한다 | `XXXY`를 두 축에서 거부했다. |
| 3 | 허용 길이 집합 밖의 A text를 받아들인다 | 6개 `X`를 두 축에서 거부했다. |
| 4 | DOM text를 수정한 뒤 오래된 snapshot/style 쌍으로 검사한다 | mutation 뒤 snapshot과 Stylo view를 다시 만들고 새 cascade를 계산한다. |
| 5 | Grid item 하나가 빠져도 남은 intrinsic 입력을 반환한다 | direct child를 제거한 두 축 장면이 거부됐다. |
| 6 | 추가 direct child를 기존 세 item에 묵시적으로 포함한다 | 네 번째 child가 있는 두 축 장면이 거부됐다. |
| 7 | DOM source order가 `[item 1, item 3, item 2]`여도 순서를 정상화한다 | 동일 배치의 실제 child 재정렬을 두 축에서 거부했다. |
| 8 | 수정된 intrinsic minimum `auto`를 계획된 `min-content`로 처리한다 | target track mutation이 두 축 dispatcher에서 오류로 끝났다. |
| 9 | 2fr을 3fr로 바꾸어도 동일 결과를 만든다 | 변경된 flex factor를 두 축에서 거부했다. |
| 10 | fixed 40px neighbor가 41px이어도 fixture로 분류한다 | 변경된 fixed neighbor를 두 축에서 거부했다. |
| 11 | 비유한 flex factor가 track trace까지 전파된다 | `f32::INFINITY` 입력이 두 축에서 오류가 됐다. |
| 12 | mixed track list가 반대 축으로 이동해도 원래 scene으로 처리한다 | 축을 뒤바꾼 typed track list를 두 축에서 오류로 끝냈다. |
| 13 | definite root 폭이 달라져도 300/120px 기준을 유지한다 | Columns 299px, Rows 119px 변조를 거부했다. |
| 14 | target gutter가 한쪽 축에서만 검증된다 | column/row gap 각각 9px 변조를 해당 축에서 거부했다. |
| 15 | root alignment 속성 문자열이 달라져도 typed Grid 입력만으로 허용한다 | `align-content: normal` 변조를 두 축에서 거부했다. |
| 16 | root min/max constraint가 flexible track 분배를 바꾸어도 놓친다 | non-auto `min-width` 및 `max-height` 변조를 두 축에서 거부했다. |
| 17 | item `display:none`이 intrinsic contribution에 남는다 | display 변조를 두 축에서 거부했다. |
| 18 | item의 position 또는 authored width가 고정 fixture 입력을 우회한다 | relative position과 10px width typed 변조를 두 축에서 거부했다. |
| 19 | text font family·typed size 또는 writing mode 불일치가 숨겨진다 | Arial, 11px, 반대 writing mode를 각각 두 축에서 거부했다. |
| 20 | placement span, computed-style profile 또는 document revision 변조가 다른 route로 빠진다 | 2-track span, 다른 profile, stale revision을 두 축에서 오류로 끝냈다. |

## 잔여 범위

이 결과는 고정 generic mixed fixture의 실패 경계다. 일반 CSS admission, 전체 C11.4/WPT, Android/iOS 앱 실행, 실기기, hardware GPU, compositor 이후 pixel equality를 증명하지 않는다. 공식 C11.4 checkbox와 완료 집계 `72/213`은 유지한다.
