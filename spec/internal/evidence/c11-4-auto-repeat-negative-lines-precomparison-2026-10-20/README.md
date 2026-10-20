# C11.4 auto-repeat과 negative line 사전 비교

이 근거는 auto-repeat으로 확장된 explicit grid와 그 앞의 negative implicit track 배치를 한 CSS 입력에서 비교한다. Chrome 화면은 기준 구현 관찰이며 Spinon 실행 결과가 아니다.

## 독립 규범 기대

기준은 고정 문서인 [CSS Grid Layout Module Level 1 · 2025 CRD](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/)의 §7.2.3.2 auto-repeat 횟수, §7.5 implicit grid, §7.6 `grid-auto-rows`/`grid-auto-columns`, §8.3 numeric line placement, §8.5 auto-placement이다. Agent Browser로 현재 [CSS Grid Level 2](https://www.w3.org/TR/css-grid-2/)도 대조해 같은 규칙 문구를 확인했다.

- definite preferred/max axis size에서는 gap을 포함해 overflow를 만들지 않는 가장 큰 양의 repetition count를 사용한다. 반복 track의 max sizing function이 definite하지 않으면 min sizing function을 기준으로 센다. 따라서 `minmax(40px, 1fr)`은 반복 수 산정에서 40px이고, `minmax(30px, 1fr)`은 30px이다.
- 반복 수는 explicit grid의 track 수를 바꾼다. negative numeric line은 확장된 explicit grid의 끝에서 역방향으로 센다. zero-based boundary는 `expandedExplicitTrackCount + 1 + negativeCssLine`이다. line 0은 invalid다.
- placement가 explicit grid보다 앞이면 필요한 leading implicit track을 만든다. `grid-auto-columns: 31px 47px`의 가장 가까운 leading track은 47px이고 다음 track은 31px이다. `grid-auto-rows: 17px 29px`는 각각 29px, 17px 순서다.
- `auto-fit`은 `auto-fill`과 같은 repetition count를 사용한 뒤 빈 반복 track과 그 양옆 gutter를 접는다. leading implicit track에 놓인 item은 반복 track을 점유하지 않는다.

고정 입력의 독립 반복 수 계산은 columns에서 `20 + 10 + 40n + 8(n+1) = 38 + 48n`, rows에서 `14 + 6 + 30n + 6(n+1) = 26 + 36n`이다. 관찰 case는 columns 2·3·4회, rows 3·4·5회를 포함한다. line index는 각 count에 맞춰 조정한다.

## Chrome 155 관찰

- 입력: [fixture](../../../../tests/fixtures/css/c11/grid-auto-repeat-negative-lines-c114-precomparison.html)
- 환경: macOS 26.5.1 ARM64, Agent Browser 0.31.1, HeadlessChrome 155.0.0.0, viewport 1280×633 CSS px, DPR 1, light color scheme.
- 출력: [computed tracks와 DOMRect](./chrome155-observation.json), [전체 화면](./chrome155-agent-browser-full.png), [별도 규범 기대값](./spec-derived-expectations.json).
- [초기 계획 적대 검토 20개 — 정정으로 무효화](./plan-review-20.md).
- [초기 계획 PR diff 검토 30개 — 정정으로 무효화](./pr-diff-review-30.md).
- [정정 계획 재검토 20개](./plan-correction-review-20.md)와 [정정 PR diff 검토 30개](./pr-correction-diff-review-30.md).
- `mise exec -- node spec/internal/evidence/c11-4-auto-repeat-negative-lines-precomparison-2026-10-20/validate-precomparison.mjs` (Node 24.20.0)는 18개 fixture ID, track count, leading/trailing list cycle, explicit-grid 기준 line 수식, fill mode item 좌표를 대조한다.
- 입력·출력·validator hash: [`sha256.txt`](./sha256.txt).

18/18 case의 computed track 수, CSS numeric line mapping, leading/trailing pattern 순서와 `auto-fit` collapse 목록이 독립 기대값과 일치했다. 모든 `auto-fill` case의 item 시작 좌표도 resolved line 위치와 일치했다. Columns 134/200/230px는 각각 2/3/4회이고 rows 134/180/206px는 3/4/5회다. 200px columns의 세 번 반복에서 `-7 / -6`은 선행 implicit track 하나를 가리킨다. 두 선행 track을 요청한 `-8 / -6`은 auto-column 목록 `[31px, 47px]`을 왼쪽에서 오른쪽으로 `[31px, 47px]` 적용한다. Rows에서도 같은 규칙이 `[17px, 29px]`로 재현됐다.

### auto-fit gutter 차이

`columns-fit-third-repeat-only`에서 Chrome은 computed track을 `20px 0px 0px 154px 10px`으로 반환하고 item x를 prefix 끝에서 8px 뒤에 둔다. W3C 문장은 collapsed track 양옆 gutter도 접도록 한다. 이 관찰은 기존 [auto-fit 축별 계획](../../../../plan/c11-4-auto-fit-min-size-axis.md#2026-10-20-gutter와-fixture-경계-정정)에 기록된 Blink 동작과 일치하는 진단 차이다. validator는 이 auto-fit item frame을 규범 기대값으로 사용하지 않으며, implementation은 규범의 collapse 기준으로 판정한다.

## 구현 전 경계 근거와 정정

최신 코드에서 auto-repeat 입력은 [dense placement dispatcher](../../../../crates/spinon-layout/src/grid_auto_placement/dense_column.rs#L32-L59)의 전용 경로로 들어간다. `normalize_auto_repeat_grid_items`는 typed item placement를 변환하지 않고 보존하지만 `GridAutoRows/Columns::TrackList`를 명시 거부한다. 따라서 일반 [track-sizing normalizer](../../../../crates/spinon-layout/src/grid_auto_placement/track_sizing.rs#L34-L70)의 `container.*.len()` negative-line 계산은 auto-repeat 입력에 적용되지 않는다.

[vendored Taffy](../../../../vendor/taffy/src/compute/grid/mod.rs#L337-L423)는 먼저 axis별 auto-repeat count와 expanded explicit track count를 계산하고, 그 count를 `into_origin_zero`에 전달한 뒤 item placement를 실행한다. 이 자료는 negative line 경로가 잘못됐다고 주장하지 않는다. 계획의 첫 가설과 원 PR 적대 검토의 해당 판정은 이 source dispatch를 놓쳐 부정확했으며, 새 [정정 계획 검토](./plan-correction-review-20.md)와 [정정 PR diff 검토](./pr-correction-diff-review-30.md)로 대체한다.

현재 제한 구현 후보는 auto-repeat branch에서 auto track list가 거부되고 [Taffy Style adapter](../../../../crates/spinon-layout/src/taffy_style.rs#L43-L78)도 `grid_auto_rows/columns`를 전달하지 않는다는 점이다. CSS oracle의 18개 입력은 그대로 유효한 Chrome reference다. 후속 회귀는 negative line이 확장 count를 사용하는지 확인한 뒤, Taffy auto-track list를 통해 leading implicit track cycle과 item frame이 spec 및 oracle에 맞는지 검사한다. 아직 Spinon runtime failure는 재현하지 않았다.

## SHA-256

```text
48cab2c2450a8380184419af45be2d0eace3e05ed582a65a2d51ec2030b7bd8d  tests/fixtures/css/c11/grid-auto-repeat-negative-lines-c114-precomparison.html
82ad06b1afed31116e24a3ff2c995a04ad78957e0d016923ea3dfa8c93a50a72  chrome155-observation.json
a68f58779b102990c7adb3282f7e06936137ff0e8b273219c8fddb5eb11fd907  spec-derived-expectations.json
53308206cd197cccced36d84f9b53f42c596761b1e05e102cbb6f862ecbdde77  validate-precomparison.mjs
309694382a0c57b30077e08f1b2aab0dcea9258e50ba4c96728ac25d73cabfe0  plan-review-20.md
03b3afb7486beaec6d63da1b221541cf77917d1e4ce285668c70308c83a17e54  pr-diff-review-30.md
c665b5efa149ad61750639a2d27adfe0d693f453d513e1894fe3a8ed70e8fed1  plan-correction-review-20.md
40eed0a74766a6f68d4751d7b97acced63b8ef2a1ed691b887461265b8751e0c  pr-correction-diff-review-30.md
5327e42af042a57123ed837ac28e0e6c7e15ad8e4eac793a286a74558927aff4  chrome155-agent-browser-full.png
```
