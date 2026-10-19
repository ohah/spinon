# C11.4 Chrome 155 spanning track-sizing matrix

날짜: 2026-10-19
상태: 현재 C11.4 작업 트리의 미병합 Rust 검증

## 기준과 판정

- 고정 기준은 [Chrome 155 observation](../c11-4-grid-track-sizing-precomparison-2026-10-19/chrome/chrome155-observation.json)의 span case 세 개다.
- Grid 크기, track count와 최종 `base_size`, Grid 기준 item frame을 각 field `0.05 CSS px` 이내로 비교한다.
- `spanning-minmax-items`는 폭 70px로 지정된 box다. 이 case는 track span 분배와 specified-size 경로를 확인하지만 intrinsic text/image contribution 증거는 아니다.
- `spanning-min-content-items`는 24×16px natural-size image snapshot을 두 min-content column에 걸쳐 둔다.
- `spanning-auto-min-flex-tracks`는 저장소 Ahem을 등록한 `TextLayoutEngine`으로 20개의 `X`를 측정한다. min/max-content 200px, 두 flexible track 각 48px, item frame 100×10px을 확인한다. Chrome의 `scrollWidth=200px`는 Rust의 원본 min/max-content 입력과 비교한다.
- Chrome은 Taffy 내부 stage를 제공하지 않는다. 따라서 비교 대상은 Chrome의 최종 used tracks와 frame이고, Rust의 마지막 기록 stage 및 완료 marker다.

## 결과

| Case | Grid tracks / gap | Chrome final columns / rows | Grid 크기 | Item frame |
| --- | --- | --- | --- | --- |
| `spanning-minmax-items` | 두 `minmax(0, auto)` column, gap 4px; `auto` row | `33px 33px` / `10px` | `100×30px` | `(0, 0, 70, 10)px` |
| `spanning-min-content-items` | 두 `min-content` column, gap 4px; `auto` row | `10px 10px` / `16px` | `100×30px` | `(0, 0, 24, 16)px` |
| `spanning-auto-min-flex-tracks` | 두 `minmax(0, 1fr)` column, gap 4px; `auto` row | `48px 48px` / `10px` | `100×30px` | `(0, 0, 100, 10)px`; text max-content `200px` |

```sh
cargo test -p spinon-layout chrome155_spanning_track_oracle_cases_match_traces_and_frames -- --nocapture
```

결과: `1 passed; 0 failed; 0 ignored; 166 filtered out`. 단일 table-driven test에서 두 span case가 실행됐다.

```sh
cargo test -p spinon-layout chrome155_spanning_auto_min_flex_tracks_match_text_contribution_and_frames -- --nocapture
```

결과: `1 passed; 0 failed; 0 ignored; 167 filtered out`. 이 test는 Ahem intrinsic input과 최종 track/frame을 확인한다.

전체 layout crate 회귀 실행은 `cargo test -p spinon-layout`에서 `168 passed; 0 failed; 0 ignored`다.

## 남은 범위

3개 이상 track의 spanning order, 경쟁하는 다중 span 및 independent normative stage oracle는 미검증이다. 이 결과는 C11.4 완료 근거가 아니다.
