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
- [초기 계획 적대 검토 20개 — 후속 정정으로 무효화](./plan-review-20.md).
- [초기 계획 PR diff 검토 30개 — 후속 정정으로 무효화](./pr-diff-review-30.md).
- [1차 정정 계획 검토 20개 — 양축 경로 누락으로 무효화](./plan-correction-review-20.md)와 [1차 정정 PR diff 검토 30개 — 양축 경로 누락으로 무효화](./pr-correction-diff-review-30.md).
- [양축 negative line draft 재현 기록](./two-axis-negative-line-draft-repro-2026-10-20.md).
- [현재 축 분류 계획 검토 20개](./plan-axis-classification-review-20.md)와 [현재 PR diff 검토 30개](./pr-axis-classification-diff-review-30.md).
- `mise exec -- node spec/internal/evidence/c11-4-auto-repeat-negative-lines-precomparison-2026-10-20/validate-precomparison.mjs` (Node 24.20.0)는 18개 fixture ID, track count, leading/trailing list cycle, explicit-grid 기준 line 수식, fill mode item 좌표를 대조한다.
- 입력·출력·validator hash: [`sha256.txt`](./sha256.txt).
- 근거 디렉터리에서 `shasum -a 256 -c sha256.txt`로 fixture, oracle, validator 및 기존·현재 review 문서의 해시를 확인한다.

18/18 case의 computed track 수, CSS numeric line mapping, leading/trailing pattern 순서와 `auto-fit` collapse 목록이 독립 기대값과 일치했다. 모든 `auto-fill` case의 item 시작 좌표도 resolved line 위치와 일치했다. Columns 134/200/230px는 각각 2/3/4회이고 rows 134/180/206px는 3/4/5회다. 200px columns의 세 번 반복에서 `-7 / -6`은 선행 implicit track 하나를 가리킨다. 두 선행 track을 요청한 `-8 / -6`은 auto-column 목록 `[31px, 47px]`을 왼쪽에서 오른쪽으로 `[31px, 47px]` 적용한다. Rows에서도 같은 규칙이 `[17px, 29px]`로 재현됐다.

### auto-fit gutter 차이

`columns-fit-third-repeat-only`에서 Chrome은 computed track을 `20px 0px 0px 154px 10px`으로 반환하고 item x를 prefix 끝에서 8px 뒤에 둔다. W3C 문장은 collapsed track 양옆 gutter도 접도록 한다. 이 관찰은 기존 [auto-fit 축별 계획](../../../../plan/c11-4-auto-fit-min-size-axis.md#2026-10-20-gutter와-fixture-경계-정정)에 기록된 Blink 동작과 일치하는 진단 차이다. validator는 이 auto-fit item frame을 규범 기대값으로 사용하지 않으며, implementation은 규범의 collapse 기준으로 판정한다.

## 구현 전 경계 근거와 정정

최신 코드에서 auto-repeat 입력은 [dense placement dispatcher](../../../../crates/spinon-layout/src/grid_auto_placement/dense_column.rs#L32-L59)의 전용 경로로 들어간다. [auto-repeat normalizer](../../../../crates/spinon-layout/src/grid_auto_placement/dense_column.rs#L428-L509)는 선택된 repeat axis의 raw placement는 보존하지만, 다른 축을 authored `container.*.len()`으로 `resolve_axis`한다. 양축 repeat에서 secondary axis의 negative line이 leading implicit 영역을 요구하면 `InvalidGridPlacement`를 반환한다. authored 범위를 넘지만 expanded repeat grid 안에 놓이는 positive line도 거부될 수 있다. 한 축 repeat의 repeat-axis negative line은 Taffy까지 전달된다.

[vendored Taffy](../../../../vendor/taffy/src/compute/grid/mod.rs#L337-L423)는 먼저 axis별 auto-repeat count와 expanded explicit track count를 계산하고, 그 count를 `into_origin_zero`에 전달한 뒤 item placement를 실행한다. 한 축 repeat case에서는 이를 확인했다. 양축 case에서는 Spinon이 secondary axis를 먼저 resolve하므로 Taffy 경로에 도달하기 전에 막힌다. 첫 계획 검토와 1차 정정 검토가 이 차이를 놓쳤으며, 현재 [축 분류 계획 검토](./plan-axis-classification-review-20.md)와 [PR diff 검토](./pr-axis-classification-diff-review-30.md)가 이전 검토를 대체한다.

현재 제한 구현 후보는 양축 repeat에서 secondary numeric line을 authored count로 먼저 푸는 분류, auto-repeat branch의 `TrackList` 거부, [Taffy Style 변환](../../../../crates/spinon-layout/src/taffy_style.rs#L83-L290)에서 auto-track 값이 기본값으로 남는 점이다. Chrome oracle의 18개 입력은 그대로 유효하다. 한 축 line 통과 대조와 양축 negative line 실패는 [draft 재현 기록](./two-axis-negative-line-draft-repro-2026-10-20.md)에 구분했다. 후속 회귀는 Taffy auto-track list를 통해 leading implicit track cycle과 item frame을 spec 및 oracle에 대조한다.

## Rust draft 재현

임시 adapter 구현 검증 중 stash commit `0bcb1c6c`의 draft test source에서 `mise exec -- cargo test -p spinon-layout auto_repeat --lib`는 8 passed / 4 failed였다. 한 축 negative line 대조는 통과했다. 양축 negative placement, auto-fit leading placement, scalar/list sizing case는 모두 secondary `grid-row-start`를 authored 3-track basis로 resolve하다가 explicit-range error를 냈다. 영구 회귀 코드와 수정은 계획 PR 병합 뒤 구현 브랜치에 반영한다. 이 실행 결과는 임시 draft 코드의 기록이며 현재 계획 브랜치 상태나 앱 실행 결과를 뜻하지 않는다.

## SHA-256

```text
48cab2c2450a8380184419af45be2d0eace3e05ed582a65a2d51ec2030b7bd8d  ../../../../tests/fixtures/css/c11/grid-auto-repeat-negative-lines-c114-precomparison.html
82ad06b1afed31116e24a3ff2c995a04ad78957e0d016923ea3dfa8c93a50a72  chrome155-observation.json
a68f58779b102990c7adb3282f7e06936137ff0e8b273219c8fddb5eb11fd907  spec-derived-expectations.json
53308206cd197cccced36d84f9b53f42c596761b1e05e102cbb6f862ecbdde77  validate-precomparison.mjs
309694382a0c57b30077e08f1b2aab0dcea9258e50ba4c96728ac25d73cabfe0  plan-review-20.md
03b3afb7486beaec6d63da1b221541cf77917d1e4ce285668c70308c83a17e54  pr-diff-review-30.md
af841c4d0f8d29db4bf8584663cd5f5f03f05955679334c72d907bdf1d568323  plan-correction-review-20.md
de096e1cbb198b04378b66699969466951dd55b72ba702d53ac1e617dd6d6211  pr-correction-diff-review-30.md
a6ee168adef200354bc770d281c5fd6372496f09ea31eb85338e7353db5ccffe  two-axis-negative-line-draft-repro-2026-10-20.md
aea80ecf705716b9768d9807213b6cd9dac4e8c506bb750a38fcdbcc777404e0  plan-axis-classification-review-20.md
9237f7e6d5dcbce3e75903937a81f78dd7313f561dec82ab30cfed9c0291f948  pr-axis-classification-diff-review-30.md
5327e42af042a57123ed837ac28e0e6c7e15ad8e4eac793a286a74558927aff4  chrome155-agent-browser-full.png
```
