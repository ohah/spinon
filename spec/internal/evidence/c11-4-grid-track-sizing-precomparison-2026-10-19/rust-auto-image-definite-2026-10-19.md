# C11.4 Rust·Chrome 단일 case 비교

날짜: 2026-10-19

> 이 파일은 최초 단일-case 실행의 과거 기록이다. 후속 8개 image matrix 결과는 [별도 근거](../c11-4-grid-track-sizing-trace-2026-10-19/chrome155-image-matrix-2026-10-19.md)를 참조한다.

## 입력과 기준

- Chromium 기준은 저장된 [Chrome 155 observation](./chrome/chrome155-observation.json)의 `auto-image-definite` case다.
- 같은 저장소 fixture는 100×40 CSS px Grid, `grid-template-columns:auto`, `grid-template-rows:auto`, 24×16 CSS px 자연 크기의 PNG를 사용한다.
- Chrome 결과는 열 24px, 행 16px, Grid 100×40px, item 24×16px다. Browser page 좌표는 Grid와 item 모두 x=158px이고 y=0px이므로 Grid 기준 item 좌표는 (0, 0)이다.
- 판정은 track size, Grid/item extent, Grid 기준 item 위치와 `IntrinsicSizingComplete` stage의 non-gutter track base size를 비교한다. 허용 오차는 0.05 CSS px다.

## Rust 실행

```sh
cargo test -p spinon-layout chrome155_auto_image_definite_matches_track_sizes_and_item_frame -- --nocapture
```

결과: `1 passed; 0 failed`. Rust 결과는 Grid frame 100×40px, item frame (0, 0, 24, 16)px, 열·행의 intrinsic sizing stage base size 24px·16px다. 이 실행은 Chrome 155 고정 oracle 26개 중 한 case만 비교한다.

## 제한

이 case는 natural-size image의 단일 auto track을 비교한다. text contribution, 여러 item/span 경쟁, auto-repeat, border 바깥 paint, app surface sample, WPT는 검증하지 않았다. 따라서 B 전체 또는 C11.4 완료 근거가 아니다.
