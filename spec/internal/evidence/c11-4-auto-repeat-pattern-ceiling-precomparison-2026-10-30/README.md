# C11.4 auto-repeat two-track ceiling · 구현 전 비교 기준

## 목적과 한계

이 기록은 vendored Taffy `0.15.0`의 auto-repeat sizing과 `spinon-layout`의 10,000-track 상한을 두-track pattern에서 비교하기 위한 Rust trace 입력과 독립 계산을 고정한다. CSS Grid §7.2.3.2의 `auto-fill` 반복 횟수와 §8.3의 named-line occurrence가 계산 근거다. §5.4는 implicit-grid 한도만 다루며 explicit auto-repeat 절단의 규범 oracle은 아니다. 10,000이라는 값과 이 clamp 결과를 W3C의 필수 결과라고 해석하지 않는다.

브라우저나 앱 screenshot은 쓰지 않는다. 관찰값은 `TaffyLayoutEngine`이 반환한 `LayoutOutput` track trace와 named-line child frame이다.

## 기준 revision

- Base source: `e17c65f69e9a8639617fad69a6288207b059e3b0` (`origin/main`, 2026-10-30)
- Implementation branch base after plan PR #836: `945c9781be074231bc4163d4847be4aa78b8fc3f` (`origin/main`)
- Internal boundary: `crates/spinon-layout/src/grid.rs`, `GRID_MAX_TRACKS = 10_000`
- Pinned implementation: `vendor/taffy`, version `0.15.0`, `src/compute/grid/explicit_grid.rs`
- Independent rules: [CSS Grid 2025 CRD §5.4](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#limiting-large-grids), [§7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat), and [§8.3](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#line-placement)

## 고정 입력

아래 각 basis를 Columns 반복 축과 Rows 반복 축에서 각각 한 번 실행한다. 직교 축에는 `1px` 고정 track 하나를 둔다. 반복 축 root와 viewport content 크기는 basis와 같고 gap·border·padding은 0이다. explicit track source order는 다음과 같다.

```text
2px repeat(auto-fill,
  [pattern-start] 1px [pattern-middle] 2px [pattern-end]
) 3px
```

`GridAutoRepeat.line_names`에는 `pattern-start`, `pattern-middle`, `pattern-end` 세 이름 그룹을 둔다. Spinon 입력 검증 계약에 맞춰 repeated axis의 outer line-name vector에도 네 개의 빈 group을 둔다 (`4` tracks - `2` repeated tracks + `2`). 고정 `1px` child 하나를 반복 축에서 `pattern-start -1`부터 `pattern-middle -1`까지, 다른 축에서는 line 1부터 2까지 배치한다. `auto-fill`은 빈 반복 track도 유지한다. 이 child는 마지막 named-line 구간을 실행으로 확인하며 고정 track 크기는 바꾸지 않는다.

## 독립 기대값

한 번 반복할 때 `1px + 2px = 3px`를 사용하고 반복 바깥의 고정 track은 `2px + 3px = 5px`를 사용한다.

| basis | 반복 횟수 | cap 전 요청 track | cap 적용 뒤 source order | named-line child 축 frame |
| ---: | ---: | ---: | --- | ---: |
| `15,002px` | `4,999` | `1 + 9,998 + 1 = 10,000` | `2px`, `[1px,2px] × 4,999`, `3px` | offset `14,996px`, size `1px` |
| `15,005px` | `5,000` | `1 + 10,000 + 1 = 10,002` | `2px`, `[1px,2px] × 4,999`, `1px`; suffix 절단 | offset `14,999px`, size `1px` |

두 경우 모두 `used_*.explicit_track_count`와 gutter 제외 `Initialize` stage에 track 10,000개가 있어야 한다. `15,002px`에서는 마지막 완전 반복 쌍이 `2 + 4,998 × 3 = 14,996px`에서 시작한다. `15,005px`에서는 prefix 뒤 반복용 slot이 9,999개 남는다. 온전한 쌍 4,999개와 다음 쌍의 첫 `1px` track이 남고, offset은 `2 + 4,999 × 3 = 14,999px`다.

source-order·capacity·track-size sequence·named-line frame 기대값은 browser serialization과 무관하게 계산했다. 빈 반복 track을 유지하는 `auto-fill`을 사용한다. `auto-fit`은 기존 단일-track ceiling matrix에서만 검증한다.
