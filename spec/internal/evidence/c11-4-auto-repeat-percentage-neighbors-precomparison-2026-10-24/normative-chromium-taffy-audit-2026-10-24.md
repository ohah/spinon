# C11.4 auto-fit gutter 규범·Chromium·Taffy 대조 · 2026-10-24

## 규범 계산

[CSS Grid Layout Module Level 1, 2025 CRD §7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat)은 auto-fit을 auto-fill과 같은 방식으로 반복 횟수 계산한 뒤, 빈 반복 track을 collapse하고 해당 track 양쪽 gutter도 없애도록 규정한다. 이 판정은 item placement 뒤에 적용한다.

`col-fit-first`와 `row-fit-first`의 고정 input은 content axis 240px, percentage prefix 10%=24px, suffix 5%=12px, gap 4px, `minmax(40px, 1fr)` 네 회다. 첫 repeat track만 점유되므로 뒤의 세 repeat track은 collapse한다. Prefix와 첫 repeat 사이의 gutter 하나는 양쪽 track이 남으므로 유지되고, 빈 repeat에 인접한 gutter들은 사라진다. 첫 repeat의 used size는 `240 - 24 - 12 - 4 = 200px`이다.

2px item을 첫 repeat track 끝에 정렬하면 표준 계산 frame은 columns `[236, 10, 2, 2]`, rows `[10, 236, 2, 2]`이다. 첫 track 시작은 각각 x/y=38px이다.

## Chrome 156 관측

Agent Browser의 Chrome 156.0.8078.48에서 저장 oracle의 2×2 item에만 DOM-in-memory `justify-self:end` 또는 `align-self:end`를 적용했다. CSSOM used tracks는 두 축 모두 `[24, 196, 0, 0, 0, 12]`; end-aligned item frame은 columns `[232,10,2,2]`, rows `[10,232,2,2]`였다. Raw result는 [auto-fit end-alignment probe](./auto-fit-end-alignment-probe-2026-10-24.json)에 있다.

기존 suffix-item probe는 Chrome에서 x/y=238px을 관측했다. 그 값은 규범 계산의 200px track + 한 gutter와 Chrome 관측의 196px track + 두 gutter 모두에서 나온다. 따라서 suffix 좌표만으로 Chrome이 gutter를 하나 더 유지했다고 판정할 수 없다. 앞선 상태 설명의 “suffix 경계 gutter 관측” 표현은 판별 근거를 넘어섰으므로 정정한다. 이 자료는 고정 장면의 값 차이만 판정하며 Chrome 내부 원인을 단정하지 않는다.

## 소스 대조와 한계

- Chromium current `main`의 [`grid_track_collection.cc` 317–323행](https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/layout/grid/grid_track_collection.cc;drc=64267ef855260b58bf1bb113650eef3671e1e64a;l=317)은 조건을 만족하는 empty auto-fit range를 collapsed로 표시하고 set count를 0으로 둔다.
- Chromium current `main`의 [`grid_layout_utils.cc` 47–69행](https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/layout/grid/grid_layout_utils.cc;drc=e79f38cbb487b6aa580bf9d3d6566f54b052cc2d;l=47)는 collapsed range가 gap geometry를 만들지 않는다고 설명하고 그 range를 건너뛴다.
- 저장소의 vendored Taffy 0.15.0 [`explicit_grid.rs` 383행 이후](../../../../vendor/taffy/src/compute/grid/explicit_grid.rs)는 CSS Grid §7.2.3.2를 인용해 collapsed auto-fit track 양쪽 gutter를 collapse한다. 기존 focused matrix는 이 경로에서 200px을 냈다.

Chromium source는 current `main`을 읽은 시점의 자료다. Chrome 156.0.8078.48 바이너리와 소스 revision을 매핑하지 않았으므로 이 코드로 해당 바이너리의 내부 원인을 설명하지 않는다. 규범값 200px을 기능 수용값으로 두고 Chrome 196px은 별도 compatibility difference로 보존한다. 이는 다른 engine이나 전체 auto-fit 동작에 관한 결론이 아니다.
