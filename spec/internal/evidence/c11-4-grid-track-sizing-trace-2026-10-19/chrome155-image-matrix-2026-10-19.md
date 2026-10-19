# C11.4 Chrome 155 image track-sizing matrix

날짜: 2026-10-19
상태: 현재 C11.4 작업 트리의 미병합 Rust 검증

## 기준과 판정

- 고정 oracle은 [Chrome 155 observation](../c11-4-grid-track-sizing-precomparison-2026-10-19/chrome/chrome155-observation.json)이다. 전체 26 case 중 image track sizing 8개를 이번 실행에서 비교했다.
- 각 case는 24×16 CSS px natural-size image 입력, Chrome의 `gridTemplateColumns`·`gridTemplateRows`, Grid 크기 및 item frame을 비교한다. 허용 오차는 각 field `0.05 CSS px`다.
- Rust 쪽 track 값은 각 축의 마지막 기록 stage에서 gutter/collapsed track을 제외한 `base_size`다. completion stage의 존재도 확인한다. Chrome은 Taffy 내부 단계를 노출하지 않으므로 중간 stage별 base/growth-limit을 Chrome과 비교했다는 뜻은 아니다.
- Rust 테스트는 저장된 image natural size를 `LayoutIntrinsicSizeSnapshot`으로 입력한다. PNG decoding, app runtime 이미지 로더, GPU surface 표본은 이 테스트 범위가 아니다.

## 결과

| Case | Grid track 입력 | Chrome columns / rows (CSS px) | Grid 크기 | Item frame (x, y, w, h) |
| --- | --- | --- | --- | --- |
| `auto-image-definite` | `auto / auto` | `24 / 16` | `100×40` | `0, 0, 24, 16` |
| `minmax-auto-image` | `minmax(0, auto) / auto` | `24 / 16` | `100×40` | `0, 0, 24, 16` |
| `minmax-min-max-image` | `minmax(min-content, max-content) / auto` | `24 / 16` | `100×40` | `0, 0, 24, 16` |
| `fit-content-below-minimum` | `fit-content(12px) / auto` | `24 / 16` | `100×40` | `0, 0, 24, 16` |
| `fit-content-above-intrinsic` | `fit-content(40px) / auto` | `24 / 16` | `100×40` | `0, 0, 24, 16` |
| `fit-content-percent` | `fit-content(50%) / auto` | `24 / 16` | `100×40` | `0, 0, 24, 16` |
| `fixed-small-track-overflow` | `minmax(0, 12px) / auto` | `12 / 16` | `12×40` | `0, 0, 24, 16` |
| `min-content-row-image` | `100px / min-content` | `100 / 16` | `100×80` | `0, 0, 24, 16` |

`fixed-small-track-overflow` confirms the item frame extends 12 CSS px beyond the Grid's right border edge. It does not verify that pixels were painted outside the border.

## Rust 실행

```sh
cargo test -p spinon-layout chrome155_image_track_oracle_cases_match_traces_and_frames -- --nocapture
```

결과: `1 passed; 0 failed; 0 ignored; 165 filtered out`. 이 표의 8개 case가 단일 table-driven test 안에서 모두 실행됐다.

전체 layout crate 회귀 실행:

```sh
cargo test -p spinon-layout
```

후속 전체 실행: `cargo test -p spinon-layout` 결과 `168 passed; 0 failed; 0 ignored`. 이 실행 중 발견한 Taffy 0.15 out-of-flow 처리 누락을 layout adapter에서 보완해 기존 absolute-position 7개도 통과시켰다. C10.2 flex oracle 비교기는 nested Chrome frame과 Taffy 결과를 각각 root-relative 좌표로 정규화하도록 고쳤다.

## 남은 범위

전체 Chrome 155 고정 oracle 26개 중 18개는 아직 Rust와 대조하지 않았다. Spanning contribution, auto-repeat, indefinite cyclic percentage, Ahem text contribution, 독립 표준 stage oracle, Android/iOS app trace, surface pixel 및 border 바깥 실제 paint는 이 기록으로 완료되지 않는다. C11.4는 미완료다.
