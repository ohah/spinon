# Fixed-repeat + auto-repeat + template areas 사전 비교

## 목적과 범위

이 근거는 CSS Grid fixed-repeat, 같은 track list의 auto-repeat, `grid-template-areas`의 implicit line alias를 함께 쓰는 C11.4 제한 profile의 사전 기준이다. 구현 출력에서 기대값을 만들지 않았다. 고정된 CSS Grid 2025 CRD 규칙으로 반복 수, explicit track 수, named-line occurrence와 item frame을 먼저 계산했고 Chrome 156은 교차 관찰값으로 기록했다.

고정 입력은 [fixture](../../../../tests/fixtures/css/c11/grid-fixed-auto-repeat-template-areas-c114.html)이며 브라우저 관찰 JSON, 독립 기대값, 실행 환경과 PNG 화면을 이 디렉터리에 보존한다.

## 독립 기대값

각 축의 고정 track은 prefix `repeat(2, 10px 6px)`의 4 tracks/32px와 suffix `repeat(2, 10px)`의 2 tracks/20px다. auto-repeat pattern은 fixed minimum `20px`, gutter는 4px다. auto-repeat track 수를 `n`으로 놓으면 minimum 합은 다음과 같다.

```text
fixed breadth             = 32px + 20px = 52px
track count               = 6 + n
gutter count              = 5 + n
minimum grid breadth      = 52px + 20px*n + 4px*(5+n)
                          = 72px + 24px*n
```

CSS Grid §7.2.3.2의 definite-size `auto-fill` 규칙에서 148px은 3회(4회면 168px), 168px은 4회, 188px은 4회(5회면 192px)다. `auto-fit`도 placement 전 반복 수는 같고, placement 후 비어 있는 반복 track과 인접 gutter만 접는다. 따라서 같은 168px에서 repeated area를 비워 둔 장면은 repeated tracks가 0px이고, `hero` item이 네 repeated tracks를 차지하는 장면은 각각 20px이다.

Area matrix는 expanded template보다 크거나 작을 수 있다. Taffy 0.15.0 Grid sizing은 explicit track 수를 expanded template track 수와 area dimension 중 큰 값으로 결정한다. 148px 장면은 template 9 tracks보다 area 10 tracks가 커 `grid-auto-columns/rows:20px`인 track을 하나 추가한다. 마지막 `right` item은 root 끝을 20px 넘어간다. 188px 장면은 template 10 tracks, area 8 tracks로 template의 trailing tracks를 유지한다.

Collision 장면은 prefix fixed-repeat의 authored `hero-start`/`hero-end`, auto-repeat의 같은 이름, `hero` area가 만든 implicit edge names를 동시에 둔다. CSS Grid §7.3.2의 implicit names는 같은 이름의 explicit line names에 더해지는 lines다. §8.3은 named-area edge alias와 같은 이름이 앞에 명시됐다면 그 earlier line을 우선한다. 따라서 bare `grid-area:hero`는 첫 authored `hero-start`/`hero-end`인 lines 1/3을 택하고 첫 fixed pair 크기 20px이 된다. Positive occurrence 3/3은 first auto-repeat track lines 5/6, negative occurrence -1/-1은 last auto-repeat track lines 8/9을 택한다. Same-physical-line tokens count as one line occurrence.

규범 source:

- [CSS Grid Layout 2025 CRD §7.2.3.2: auto-fill/auto-fit](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat)
- [§7.3.2: implicit line names](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#implicit-named-lines)
- [§8.1.1: named areas](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#common-uses-named-areas)
- [§8.3: line-based placement](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#line-placement)

## Chrome 관찰과 검증

Chrome for Testing `156.0.8078.48` / Agent Browser `0.31.1`, macOS ARM64, viewport `1280×633` CSS px, DPR 1, `ltr`, `horizontal-tb`에서 아홉 scene을 관찰했다. 모든 CSSOM used-track list와 30개 child frame이 independent expectation과 `1/64 CSS px` 이내로 일치했다. `validate-precomparison.mjs`는 반복 수 산식, area/template track 수, 사용 track list, expanded named-line occurrence와 area alias 위치를 각각 검사한다.

`chrome156-fixed-auto-repeat-areas.png`는 Chrome 장면 화면이다. columns/rows의 `area-larger` scene에서 파란 item이 root 바깥으로 20px 나오는 경계를 볼 수 있다. 이 PNG는 Chrome oracle 화면이며 Spinon app, simulator, WPT 결과가 아니다. 픽셀 표본이나 border stroke도 주장하지 않는다.

Taffy 대조는 저장소 고정 vendor `0.15.0`의 [`NamedLineResolver::new`](../../../../vendor/taffy/src/compute/grid/types/named.rs)와 `area_edges_are_merged_with_explicitly_named_lines` test다. Grid sizing의 explicit count는 [`max(expanded template, area dimension)`](../../../../vendor/taffy/src/compute/grid/mod.rs)으로 결정한다. 이 코드 참조는 비교 모델 작성에만 쓰이며 변경하지 않았다.

```text
$ node spec/internal/evidence/c11-4-fixed-auto-repeat-areas-precomparison-2026-10-24/validate-precomparison.mjs
PASS: 9 scenes, 30 child boxes; tolerance 0.015625 CSS px
```

## SHA-256

`SHA256SUMS`에는 canonical fixture, Chrome CSSOM/geometry observation, browser environment, independent expected values, validator와 screenshot hash가 있다. 구현에서 이 관찰을 수집하거나 갱신하지 않는다.
