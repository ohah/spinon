# C11.4 · 음수 implicit track과 역방향 목록 순환 계획

**상위 원본:** [C11 상태 대장](../spec/STATUS.md#c11-css-grid) · [C11.4 전체 track sizing 계획](./c11-4-grid-track-sizing.md) · [양의 implicit track 구현 계획과 PR #375](./c11-4-positive-implicit-track-sizing.md) · [CSS Grid Layout §7.1, §7.5–§7.6, §8.3, §8.5](https://www.w3.org/TR/css-grid/#implicit-grids) · [내부 계약 0059](../spec/internal/0059-c11-grid-layout.md)

**상태:** 계획 검토 후 제한 구현 후보를 검증 중이다. W3C 규범에서 독립적으로 계산한 33개 case를 Agent Browser HeadlessChrome 155와 비교했다. SHA-256이 포함된 입력·관찰·기대값·화면은 [사전 비교 자료](../spec/internal/evidence/c11-4-negative-implicit-track-sizing-precomparison-2026-10-20/README.md)에 있다. 이 후보만으로 C11.4 상태나 완료 수를 올리지 않는다.

## 확인된 현재 경계

- `RuntimeBlockFlexGridTrackSizingV1`은 양의 implicit track과 typed `grid-auto-rows`·`grid-auto-columns` 목록을 처리한다. #375의 다음 미완료 경계는 explicit grid 앞의 negative implicit track이다.
- `AxisPlacement.start`는 `Option<usize>`이고 `resolve_axis`는 TrackSizing에서 positive implicit line만 허용한다. explicit grid 밖의 음수 line은 `explicit_line`에서 거부된다.
- dense normalizer의 점유·자동배치 cursor와 track 개수는 explicit grid의 0부터 시작한다. `expand_tracks`는 positive 방향으로만 track을 추가하며, Taffy line으로 바꾸기 전에 negative origin을 이동할 표현이 없다.
- positive pattern은 첫 explicit track 뒤에서 list index 0으로 시작한다. negative track을 같은 index 계산으로 붙이면 규범과 반대로 순환한다.
- C11.2.2와 C11.2.3의 별도 profile도 이 normalizer를 사용한다. 이번 변경은 `TrackSizing` 모드에서만 확장하고 이전 profile의 허용 범위와 오류를 유지해야 한다.
- 106-scene 통합 회귀에서 기존 `auto-fit` explicit-neighbor 경계 gutter도 다시 확인했다. 빈 repeat 뒤 suffix item이 있을 때 Chrome 155는 prefix 쪽 8px gutter를 유지한다. 이 adjacent Taffy regression도 함께 고치며 [독립 Chrome probe](../spec/internal/evidence/c11-4-auto-repeat-explicit-neighbors-precomparison-2026-10-20/README.md)와 Rust 회귀를 근거로 남긴다. 이는 auto-repeat과 negative line placement를 결합해 지원한다는 뜻이 아니다.

## 지원 범위

1. `RuntimeBlockFlexGridTrackSizingV1`에서 columns와 rows 양쪽의 numeric negative grid line placement를 지원한다. explicit track이 `N`개일 때 explicit 첫 line 기준 signed boundary coordinate는 양수 CSS line `n`에서 `n - 1`, 음수 CSS line `n`에서 `N + 1 + n`으로 구한다. 음수 line은 expanded grid가 아니라 explicit grid의 끝을 기준으로 해석하고, 그 결과가 explicit grid 앞이면 필요한 leading implicit track을 생성한다. CSS numeric grid line `0`은 계속 invalid다.
2. 음수 definite line 쌍, `span`과 음수 definite line 조합, explicit 앞·안·뒤를 가로지르는 span, reversed/equal line 정규화, explicit track이 없는 한 줄 grid를 지원한다. `display:none` item은 extent와 점유 계산에서 제외한다.
3. signed track coordinate `q`를 첫 explicit track 기준으로 유지해 배치 범위를 구한다. 각 track은 `[q, q + 1)` boundary를 차지한다. `q = -1`은 explicit grid에 붙은 마지막 leading track, `q = 0`은 첫 explicit track, `q = explicit_track_count`는 첫 trailing track이다. explicit track이 없는 grid는 경계선 하나를 기준으로 `q < 0`과 `q >= 0`을 계산한다.
4. `grid-auto-*`의 비어 있지 않은 목록을 explicit grid 경계에서 양방향으로 순환한다. 목록 `[a, b, c]`는 leading track 좌측부터 `[..., c, b, a]` 순서로 반복하고, trailing track은 `[a, b, c, ...]` 순서로 반복한다. 음수 track coordinate `q < 0`의 목록 index는 `(len - (unsigned_abs(q) % len)) % len`, trailing index는 `(q - explicit_track_count) % len`이다. 코드에서 signed negation overflow를 만들지 않는다. 한 항목 목록, 0px 및 C11.4에서 현재 지원하는 typed track size의 계산을 보존한다. 빈 목록과 음수/overflow size는 converter의 기존 오류로 fail-closed 한다.
5. definite placement로 leading bound를 구한 뒤에도 모든 배치 계산을 signed coordinate에서 유지한다. CSS auto-placement cursor는 implicit grid의 실제 start-most line에서 시작한다. auto-placement가 trailing extent를 늘린 후 최종 min/max가 검증되면 `origin_shift = -min_track_coordinate`를 한 번만 적용해 non-negative Taffy line으로 변환한다. track 배열, occupancy, cursor와 item frame이 동일한 shift를 사용한다. explicit grid 내부의 상대 순서와 positive 영역의 기존 동작을 바꾸지 않는다.
6. `grid-auto-flow: row`, `row dense`, `column`, `column dense`의 자동배치가 leading track을 포함한 implicit grid의 시작선에서 동작한다. 음수 definite placement가 만든 공간을 점유 표에 먼저 반영하고, auto cursor가 음수 영역을 건너뛰거나 explicit 경계에서 재시작하지 않도록 한다.
7. 각 축의 기존 10,000 total-track 상한을 유지한다. signed min/max extent를 먼저 checked arithmetic으로 계산해 leading + explicit + trailing track 합이 상한 안인지 검사한 뒤 allocation한다. 경계값·경계 초과·span/line arithmetic overflow·Spinon 소유 vector reservation 실패에서 부분 placement/LayoutOutput을 성공으로 반환하지 않는다. 기존 Taffy 내부 infallible allocation은 일반 OOM 복구를 보장하지 않는다는 제한을 유지한다.
8. C11.4 TrackSizing profile만 변경한다. C11.2.2/C11.2.3와 기존 layout profile의 fixed `grid-auto-*`, 오류 및 배치 결과에 regression control을 둔다. 새로운 사용자 API를 만들지 않고 내부 계약 0059의 `0.1.0`을 유지한다.

## 명시적 미지원 범위

- `grid-auto-*` typed list와 `grid-template-*` auto-repeat의 결합은 기존처럼 명시 거부한다. auto-repeat의 track expansion과 leading negative origin 교차 지원은 별도 후속이다.
- TrackSizing container에 auto-repeat이 있으면 기존 auto-repeat placement 경로를 사용하며 signed normalizer에는 연결하지 않는다. 이 경로의 negative line과 leading implicit track 상호작용은 이 slice에서 지원이나 거부를 단정하지 않고 별도 검증 대상으로 남긴다.
- named line, named area, `grid-template-areas`, nested/subgrid, anonymous item, out-of-flow item, unsupported computed track value와 일반 CSS math는 현재 C11.4 converter의 오류 경계를 유지한다.
- orthogonal writing mode, 전체 WPT suite 및 전체 CSS Grid 적합성은 이 조각의 완료 조건이 아니다. 이 구현은 C11.4나 C11 전체 완료를 뜻하지 않는다.
- 물리 Android/iOS 기기는 연결하지 않는다. 시뮬레이터 근거는 지정된 AVD와 iOS Simulator에 한정하며 hardware GPU와 iOS OS compositor 이후의 화면 일치를 주장하지 않는다.

## 사전 oracle과 합격 기준

사전 비교 자료의 33개 Chrome 155 case는 양 축의 leading track 1–4개, reverse/forward cycle, fixed·zero·min/max-content·minmax list, explicit 경계 span, reversed line과 span, no-explicit-track grid, hidden control, row/column sparse·dense auto-flow, dense 대 sparse cursor 위치가 달라지는 paired hole case, border 밖 overflow를 분리한다. 별도 `spec-derived-expectations.json`은 W3C §7.1·§7.5–§7.6·§8.3·§8.5 규칙에서 직접 산출했다. `validate-precomparison.mjs`는 33/33 used track list와 자동배치·overflow의 상대 DOMRect를 CSS px 1/64 이내에서 비교한다. Ahem은 고정 fixture font를 실제 로드했는지 검사한다.

- Browser 판정은 Chrome 관찰과 규범 기대의 일치로 제한한다. 구현 결과에서 기대값을 다시 만들거나 단일 browser를 규범으로 삼지 않는다.
- 두 100px root overflow case의 마지막 item은 Chrome 기준 root 오른쪽으로 172 CSS px, 아래쪽으로 108 CSS px 벗어난다. screenshot과 DOMRect에서 root border 기준 좌표를 보존한다.
- 실행 시 fixture/hash/oracle을 먼저 재검증한다. 구현이 기대와 다르면 차이를 좁혀 숨기지 말고 규범 입력, 기존 converter, 재현·환경과 함께 기록한다.

## 구현 및 검증 순서

1. 이 계획을 구현과 별개인 실패 관점 20개로 검토하고, 발견 사항을 계획·oracle·합격 기준에 반영한다.
2. 계획 PR을 현재 `main`에 `Rebase and merge`한 다음 새 implementation worktree를 만들고 merge commit을 확인한다.
3. layout placement input을 signed coordinate 또는 동등한 checked origin-relative 표현으로 변경한다. Taffy로 넘기기 전까지 CSS line resolution, negative/positive extent, item occupancy, row/column flow cursor의 좌표계를 섞지 않는다.
4. 두 축의 leading pattern과 origin shift를 구현한다. C11.4 existing typed converter, fallible reservations, total track cap 및 no-partial-output 정책을 재사용하고 profile gate를 유지한다.
5. 33개 browser case를 Rust projection/layout fixture로 각각 옮기고, additional adversarial controls로 numeric line zero, 9,999/10,000/10,001 total-track 경계, extreme signed line/span, repeated/overlapping placements, sparse/dense cursor rewind, placement-order independence, hidden items, allocation-failure injection, auto-repeat/error route와 unsupported input을 검증한다. 기대값은 Chrome capture와 W3C 규칙에 고정한다.
6. Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 기존 73개 C11.4 scene과 새 음수 방향 33개 scene을 합친 106/106 matrix를 각 플랫폼에서 실행한다. root/item frame, used track list, signed origin/leading count trace, axis stage trace, revision, draw/present를 모두 맞춘다. physical device 연결은 하지 않는다.
7. 두 overflow scene에서 root interior background, root edge background, 경계 바깥 item과 overflow tail surface를 Android PixelCopy와 iOS Simulator Metal drawable 각각에서 고정 좌표로 읽는다. 현재 renderer 경로는 CSS border stroke를 그리지 않으므로 그 색상은 이 slice의 표본으로 주장하지 않으며, border paint가 추가되는 C07.2 후속에서 검증한다. 표면 표본은 앱 screenshot이나 geometry 결과로 대체하지 않는다.
8. `cargo fmt --all -- --check`, focused layout/style tests, `cargo test --locked --workspace`, workspace all-target Clippy `-D warnings`, Android/iOS simulator builds와 fixture artifact boundary를 실행한다. default artifact에는 test-only fixture ABI/payload가 없어야 하고 opt-in control에는 있어야 한다. 기존 기준선 실패는 원래 실패와 변경 유발을 분리한다.
9. 구현 코드·런타임을 계획 표와 독립된 실패 관점 20개로 검토한다. 계획 관점을 재사용하지 않는다. 이후 exact PR diff를 현재 project rules에 따라 서로 다른 실패 관점 30개로 다시 검토하고 수정된 revision의 영향 관점을 재검토한다.
10. Android와 iOS Simulator 화면을 각각 PR 본문에 raw image link로 첨부한다. 현재 required checks와 merge state를 확인하고 green required gate 이후 `Rebase and merge`한다. required check이 설정되지 않았다면 CI green으로 말하지 않는다.
11. merge 결과에 맞춰 `spec/STATUS.md`, 내부 계약 0059, `spec/public/roadmap.html`의 진행·미완료 범위와 완료 수를 동기화한다. 전체 C11.4 gate가 닫힐 때까지 C11.4 완료 체크/완료 수를 올리지 않는다. Tailnet static preview는 최신 main 문서 output을 별도 PR로 sync하고 HTTP/hash/Agent Browser 화면을 확인한다.

## 이 조각의 완료 판정

33/33 Chrome oracle, 독립 expectation validator, Rust/style/layout·cap·allocation·profile 회귀, 기본/opt-in artifact 경계, Android 106/106과 iOS 106/106 Simulator scene, 양 플랫폼 root interior/edge background와 root 바깥 item/tail surface sample, trace/draw/present, 구현 실패 관점 20개 review와 PR diff 30개 review가 모두 있어야 이 signed negative-origin slice를 완료로 기록한다. total-track 10,000 cap의 checked extent arithmetic은 경계 단위 시험으로 검증한다. 전체 Taffy trace는 더 낮은 track 수에서도 100,000 track-state 예산에 도달해 실패할 수 있으므로 10,000-track 전체 layout 성공은 약속하지 않는다. CSS border stroke, WPT full suite, auto-repeat과 negative origin 조합, nested/anonymous/out-of-flow, 실기기, hardware GPU와 iOS compositor-after는 명시된 후속/한계로 남는다.
