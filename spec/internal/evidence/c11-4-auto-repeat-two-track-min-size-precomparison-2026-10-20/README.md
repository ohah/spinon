# C11.4 two-track auto-repeat · definite minimum-size 사전 비교

**상태:** 계획 전 Chrome 155 사전 관찰만 완료했다. Spinon, Android/iOS Simulator, WPT 실행 결과가 아니다. C11.4와 공식 완료 수를 바꾸지 않는다.

## 기준과 재현 조건

- 규범 기준: [CSS Grid Layout Module Level 1 §7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#repeat-notation). definite minimum-size만 있으면 이를 충족하는 최소 양의 반복 수를 정한다. 각 반복 track은 definite max sizing function, 없으면 definite min sizing function으로 계산하고, `auto-fit`은 item placement 뒤 빈 반복 track과 해당 gutter를 접는다. `auto-fit` 반복 수 계산은 `auto-fill`과 같다.
- 독립 관찰기: Google Chrome `155.0.8059.85`, Agent Browser `0.31.1`, macOS ARM64, viewport `1600×1500` CSS px, DPR 1.
- fixture: [grid-auto-repeat-two-track-min-size-c114.html](../../../../tests/fixtures/css/c11/grid-auto-repeat-two-track-min-size-c114.html), SHA-256 `1a140a4a2aff61b72240bccd66a63bdd214ae6fdba0877aac2be47e495fffdc9`.
- 원 관찰: [chrome155-observation.json](./chrome155-observation.json), SHA-256 `c20ebeb906564f3980c1be2d443ca80ed40c5e002f28a92cb3d46c94e06ce4fc`. DOM `getComputedStyle()`의 resolved track list와 grid/item `getBoundingClientRect()`를 저장했다.
- Agent Browser 화면: [chrome155-min-only-two-track.png](./chrome155-min-only-two-track.png), SHA-256 `5b069c3eda217d9e94e73d3f7fdffcb19662da97a0b934c4e844316aeea164b3`.

## 독립 기대값

반복 fragment는 `minmax(40px, 1fr) 20px`; 8px gap이며 explicit neighbor는 유한 fixed `px`다. `1fr` max는 definite가 아니므로 반복 수에서 그 track의 definite 40px minimum을 사용한다. 양쪽 neighbor가 있을 때 반복 횟수가 `n`이면 min-size를 채우는 track+gutter 사용량은 `20 + 10 + 60n + 8(2n + 1) = 38 + 76n`이다.

| Min-size | 독립 기대 반복 수 | 사용량 경계 | Chrome auto-fill 관찰 |
| --- | ---: | --- | --- |
| 190px | 2 | 1회 114px < 190px, 2회 190px | 2회, root `190px` |
| 200px | 3 | 2회 190px < 200px, 3회 266px | 3회, root `266px` |
| 342px | 4 | 3회 266px < 342px, 4회 342px | 4회, root `342px` |

앞 neighbor만 둘 때 200px 최소 크기는 `20 + 76n`으로 3회·248px, 뒤 neighbor만 둘 때는 `10 + 76n`으로 3회·238px다. 행 축은 같은 계산을 block axis에 적용한다. Chrome의 fixed `40px 20px` 대조군은 같은 반복 수와 root 크기를 반환했다. 실제 fragment `minmax(40px, 1fr) 20px`도 따로 유지해 indefinite flex maximum의 fallback을 확인한다.

## Chrome 155 관찰

| Case | Axis/mode | Min-size | Resolved repeated pattern | Root |
| --- | --- | ---: | --- | ---: |
| `columns-fill-both-min-190` | columns / auto-fill | 190px | 2회: `40 20` ×2 | 190×16 |
| `columns-fill-both-min-200` | columns / auto-fill | 200px | 3회: `40 20` ×3 | 266×16 |
| `columns-fill-fixed-min-200` | columns / auto-fill, fixed control | 200px | 3회: `40 20` ×3 | 266×16 |
| `columns-fill-both-min-342` | columns / auto-fill | 342px | 4회: `40 20` ×4 | 342×16 |
| `columns-fill-prefix-min-200` | columns / auto-fill | 200px | 3회: `40 20` ×3 | 248×16 |
| `columns-fill-suffix-min-200` | columns / auto-fill | 200px | 3회: `40 20` ×3 | 238×16 |
| `rows-fill-both-min-190` | rows / auto-fill | 190px | 2회: `40 20` ×2 | 16×190 |
| `rows-fill-both-min-200` | rows / auto-fill | 200px | 3회: `40 20` ×3 | 16×266 |
| `rows-fill-fixed-min-200` | rows / auto-fill, fixed control | 200px | 3회: `40 20` ×3 | 16×266 |
| `rows-fill-both-min-342` | rows / auto-fill | 342px | 4회: `40 20` ×4 | 16×342 |

각 auto-fill 관찰의 전체 used tracks, gap, UA, viewport, screenshot은 JSON에 보존했다.

## auto-fit 상호 운용 관찰

§7.2.3.2는 `auto-fit`이 `auto-fill`과 같은 횟수로 반복한 뒤 item placement 이후 빈 반복 track을 접도록 한다. 그런데 이 고정 fixture에서 Chrome 155의 `auto-fit`은 축마다 다른 수를 반환했다.

- columns, 200px min-size, empty: 반복 pattern 2회분 track 네 개가 `0px`; root 200px.
- rows, 같은 min-size와 pattern, empty: 반복 pattern 3회분 track 여섯 개가 `0px`; root 200px.
- `auto-fit` two-track-span case도 columns는 2회분, rows는 3회분을 직렬화했다. Track list와 item rect는 observation JSON을 따른다.

Chromium source의 [NGGridLayoutAlgorithm::ComputeAutomaticRepetitions](https://chromium.googlesource.com/chromium/src/%2B/8c0f71f21625df16367c644d18fa3b4c3b8a4db8/third_party/blink/renderer/core/layout/ng/grid/ng_grid_layout_algorithm.cc#L1116)는 축별 available/min/max size를 넘기고, 같은 반복 수 계산·반올림 코드를 사용한다. 이 fixture에서 입력 크기 상태를 내부 계측하지 않았으므로 축 차이의 원인은 미확정이다. Chrome의 `auto-fit` 결과를 규범 기대값으로 바꾸거나 Spinon 지원 판정에 사용하지 않는다. 이 계획의 실행 acceptance는 규범 계산과 Chrome 관찰이 일치하는 `auto-fill` 조합으로 제한하고, auto-fit 결과는 별도 unresolved issue로 남긴다.

## 판정 범위와 한계

이 자료는 2-track pattern, 8px gap, definite `min-width`/`min-height`, `max-content` preferred size 및 fixed `px` explicit neighbors만 다룬다. 화면은 Chrome DOM oracle이며 Spinon·앱 surface·시뮬레이터·실기기·WPT 근거가 아니다. Named line/area, 비-px neighbor, 두 축 동시 auto-repeat, integer repeat 혼합, 10,000-track 경계, 일반 item placement/span matrix와 C11.4 전체 적합성은 다루지 않는다.

## 해시

이 디렉터리의 fixture observation과 screenshot은 `SHA256SUMS`에 고정한다.
