# C11.4 image auto/max 혼합 track fail-closed 검증

기준 checkout은 `e4570a04f1f8d0595836e3f35d7fa6a88b7c8b83`이며, 검증 시점의 수정된 test source SHA-256은 `4e64a61afc2c000d430b62fc0fa97979b15c2a826b2f3d72048984d55572b704`다.

## 확인한 거부 경계

`validate_grid_style` 직접 테스트는 `minmax(auto,max-content)`의 min을 `min-content`로 바꾼 경우, 고정 이웃을 `17px`에서 `18px`로 바꾼 경우, row breadth를 `auto`에서 `1px`로 바꾼 경우, root width/height/border/gap 변경을 거부하는지 확인한다. `align-items`, `align-content`, `justify-content` 변경은 `unsupported_c114_image_style`와 `expected start` detail을 확인한다. `validate_item_style` 직접 테스트는 명시적인 column line 및 span을 `unsupported_c114_image_case` / `replaced item box or placement`로 거부하는지 확인한다.

별도의 `prepare()` 통합 단위 테스트는 allowlisted HTML root/item, fixed PNG source 및 computed root/item style snapshot을 전달한다. Root width를 `99px`로 바꾼 입력은 `unsupported_c114_image_case` / `container box geometry`로 반환되며 `C11_4ImageInput` 성공값을 만들지 않는다. 다른 `prepare()` tests는 해당 새 scene ID에 대한 non-HTML namespace, unknown root ID, wrong item ID 및 wrong source gate를 확인한다. 이들 source/tree 거부는 computed style lookup 전에 발생한다.

직접 validator 테스트는 해당 validator의 error code/detail만 검증한다. `prepare()` 통합 테스트는 그 함수의 error 반환만 확인하며 downstream layout frame·track trace가 생성되지 않는다는 별도 관측은 제공하지 않는다. 앱·simulator 동작과 image paint/pixel 결과를 이 evidence에서 주장하지 않는다.

## 검사

- `cargo fmt --all -- --check`: 성공. [원본 stdout](./cargo-fmt-check.log)
- `cargo test --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test c11_4_image`: **8 passed, 0 failed**. [원본 stdout](./focused-image-tests.log)
- `cargo clippy --locked -p spinon-runtime --features c11-4-grid-track-sizing-fixture-test --all-targets -- -D warnings`: 성공. [원본 stdout](./runtime-clippy.log)
- test source: SHA-256 `4e64a61afc2c000d430b62fc0fa97979b15c2a826b2f3d72048984d55572b704`
- 계획 source: SHA-256 `c735faceabc5d019dc9f5a82736c1605afbf268df6a44439a6973488d10faa25`
